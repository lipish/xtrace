use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use bytes::Bytes;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use flate2::read::GzDecoder;
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest as PbExportTraceServiceRequest;
use prost::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::{postgres::PgPoolOptions, PgPool, QueryBuilder};
use std::{collections::HashSet, net::SocketAddr, sync::Arc};
use thiserror::Error;
use std::io::Read;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    api_bearer_token: Arc<str>,
    langfuse_public_key: Option<Arc<str>>,
    langfuse_secret_key: Option<Arc<str>>,
    default_project_id: Arc<str>,
    ingest_tx: mpsc::Sender<BatchIngestRequest>,
}

#[derive(Debug, Serialize)]
struct ProjectsResponse {
    data: Vec<Project>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Project {
    id: String,
    name: String,
    created_at: String,
    updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<JsonValue>,
}

async fn get_projects(State(state): State<AppState>) -> impl IntoResponse {
    // Langfuse Python SDK v3.x 的 auth_check() 会调用 GET /api/public/projects
    // 并检查返回的 projects.data 是否非空。
    let now = Utc::now().to_rfc3339();
    let project_id = state.default_project_id.as_ref().to_string();
    (
        StatusCode::OK,
        Json(ProjectsResponse {
            data: vec![Project {
                id: project_id.clone(),
                name: project_id,
                created_at: now.clone(),
                updated_at: now,
                metadata: Some(JsonValue::Object(serde_json::Map::new())),
            }],
        }),
    )
}

// --- Langfuse OTLP/HTTP traces ingestion (JSON protobuf) ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtelExportTraceServiceRequest {
    resource_spans: Vec<OtelResourceSpan>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtelResourceSpan {
    #[serde(default)]
    resource: Option<OtelResource>,
    #[serde(default)]
    scope_spans: Vec<OtelScopeSpan>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtelResource {
    #[serde(default)]
    attributes: Vec<OtelKeyValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtelScopeSpan {
    #[serde(default)]
    spans: Vec<OtelSpan>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtelSpan {
    trace_id: String,
    span_id: String,
    #[serde(default)]
    parent_span_id: Option<String>,
    name: String,
    #[serde(default)]
    start_time_unix_nano: Option<String>,
    #[serde(default)]
    end_time_unix_nano: Option<String>,
    #[serde(default)]
    attributes: Vec<OtelKeyValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtelKeyValue {
    key: String,
    #[serde(default)]
    value: Option<OtelAnyValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtelAnyValue {
    #[serde(default)]
    string_value: Option<String>,
    #[serde(default)]
    int_value: Option<String>,
    #[serde(default)]
    double_value: Option<f64>,
    #[serde(default)]
    bool_value: Option<bool>,
    #[serde(default)]
    array_value: Option<OtelArrayValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtelArrayValue {
    #[serde(default)]
    values: Vec<OtelAnyValue>,
}

fn decode_hex(s: &str) -> Option<Vec<u8>> {
    let s = s.trim();
    if s.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    let hex_val = |c: u8| -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    };
    for i in (0..bytes.len()).step_by(2) {
        let hi = hex_val(bytes[i])?;
        let lo = hex_val(bytes[i + 1])?;
        out.push((hi << 4) | lo);
    }
    Some(out)
}

fn otel_trace_id_to_uuid(trace_id: &str) -> Option<Uuid> {
    // OTLP traceId 是 16 bytes (32 hex)
    let raw = decode_hex(trace_id)?;
    if raw.len() != 16 {
        return None;
    }
    Uuid::from_slice(&raw).ok()
}

fn otel_span_id_to_uuid(span_id: &str) -> Option<Uuid> {
    // OTLP spanId 是 8 bytes (16 hex)，这里左侧补 0 成 16 bytes 映射到 UUID
    let raw = decode_hex(span_id)?;
    if raw.len() != 8 {
        return None;
    }
    let mut padded = [0u8; 16];
    padded[8..].copy_from_slice(&raw);
    Uuid::from_slice(&padded).ok()
}

fn unix_nano_to_datetime(v: &Option<String>) -> Option<DateTime<Utc>> {
    let s = v.as_deref()?.trim();
    let nanos: i128 = s.parse().ok()?;
    if nanos <= 0 {
        return None;
    }
    let secs = (nanos / 1_000_000_000) as i64;
    let sub_nanos = (nanos % 1_000_000_000) as u32;
    Utc.timestamp_opt(secs, sub_nanos).single()
}

fn otel_any_value_to_json(v: &OtelAnyValue) -> JsonValue {
    if let Some(s) = &v.string_value {
        return JsonValue::String(s.clone());
    }
    if let Some(s) = &v.int_value {
        if let Ok(i) = s.parse::<i64>() {
            return JsonValue::Number(i.into());
        }
        return JsonValue::String(s.clone());
    }
    if let Some(f) = v.double_value {
        if let Some(n) = serde_json::Number::from_f64(f) {
            return JsonValue::Number(n);
        }
    }
    if let Some(b) = v.bool_value {
        return JsonValue::Bool(b);
    }
    if let Some(arr) = &v.array_value {
        return JsonValue::Array(arr.values.iter().map(otel_any_value_to_json).collect());
    }
    JsonValue::Null
}

fn otel_value_to_json(v: &Option<OtelAnyValue>) -> JsonValue {
    v.as_ref().map(otel_any_value_to_json).unwrap_or(JsonValue::Null)
}

fn attributes_to_map(attrs: &[OtelKeyValue]) -> serde_json::Map<String, JsonValue> {
    let mut m = serde_json::Map::new();
    for kv in attrs {
        m.insert(kv.key.clone(), otel_value_to_json(&kv.value));
    }
    m
}

fn extract_string_attr(attrs: &[OtelKeyValue], key: &str) -> Option<String> {
    attrs
        .iter()
        .find(|kv| kv.key == key)
        .and_then(|kv| kv.value.as_ref())
        .and_then(|v| v.string_value.clone())
}

fn extract_array_string_attr(attrs: &[OtelKeyValue], key: &str) -> Option<Vec<String>> {
    let v = attrs
        .iter()
        .find(|kv| kv.key == key)
        .and_then(|kv| kv.value.as_ref())?
        .array_value
        .as_ref()?;

    let mut out = Vec::with_capacity(v.values.len());
    for item in &v.values {
        if let Some(s) = &item.string_value {
            out.push(s.clone());
        }
    }
    Some(out)
}

fn extract_prefixed_map(attrs: &[OtelKeyValue], prefix: &str) -> serde_json::Map<String, JsonValue> {
    let mut out = serde_json::Map::new();
    for kv in attrs {
        if let Some(rest) = kv.key.strip_prefix(prefix) {
            if !rest.is_empty() {
                if let Some(v) = &kv.value {
                    out.insert(rest.to_string(), otel_any_value_to_json(v));
                }
            }
        }
    }
    out
}

fn parse_usage_details(attrs: &[OtelKeyValue]) -> (Option<i64>, Option<i64>, Option<i64>, Option<JsonValue>) {
    let raw = match extract_string_attr(attrs, "langfuse.observation.usage_details") {
        Some(s) => s,
        None => return (None, None, None, None),
    };
    let v: JsonValue = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(_) => return (None, None, None, None),
    };
    let prompt = v.get("promptTokens").and_then(|x| x.as_i64());
    let completion = v.get("completionTokens").and_then(|x| x.as_i64());
    let total = v.get("totalTokens").and_then(|x| x.as_i64());
    let usage = Some(serde_json::json!({
        "input": prompt.unwrap_or(0),
        "output": completion.unwrap_or(0),
        "total": total.unwrap_or(0)
    }));
    (completion, prompt, total, usage)
}

fn is_gzip(headers: &HeaderMap) -> bool {
    headers
        .get(header::CONTENT_ENCODING)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|s| s.to_ascii_lowercase().contains("gzip"))
}

fn content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(';').next().unwrap_or("").trim().to_ascii_lowercase())
}

fn ungzip_if_needed(headers: &HeaderMap, body: Bytes) -> Result<Vec<u8>, ApiError> {
    if !is_gzip(headers) {
        return Ok(body.to_vec());
    }
    let mut decoder = GzDecoder::new(body.as_ref());
    let mut out = Vec::new();
    decoder
        .read_to_end(&mut out)
        .map_err(|e| ApiError::BadRequest(format!("gzip decode failed: {e}")))?;
    Ok(out)
}

fn map_otel_to_batches(
    state: &AppState,
    payload: OtelExportTraceServiceRequest,
) -> Result<Vec<BatchIngestRequest>, ApiError> {
    let default_project_id = state.default_project_id.as_ref().to_string();
    let mut per_trace: std::collections::BTreeMap<Uuid, Vec<ObservationIngest>> =
        std::collections::BTreeMap::new();
    let mut trace_first_ts: std::collections::BTreeMap<Uuid, DateTime<Utc>> =
        std::collections::BTreeMap::new();
    let mut trace_acc: std::collections::BTreeMap<Uuid, TraceIngest> =
        std::collections::BTreeMap::new();

    for rs in payload.resource_spans {
        let resource_attrs = rs.resource.as_ref().map(|r| &r.attributes);
        for ss in rs.scope_spans {
            for span in ss.spans {
                let trace_id = match otel_trace_id_to_uuid(&span.trace_id) {
                    Some(v) => v,
                    None => continue,
                };
                let span_uuid = match otel_span_id_to_uuid(&span.span_id) {
                    Some(v) => v,
                    None => continue,
                };
                let parent_uuid = match span.parent_span_id.as_deref() {
                    Some(p) if !p.is_empty() && p != "0000000000000000" => {
                        otel_span_id_to_uuid(p)
                    }
                    _ => None,
                };

                let start_time = unix_nano_to_datetime(&span.start_time_unix_nano);
                let end_time = unix_nano_to_datetime(&span.end_time_unix_nano);

                if let Some(st) = start_time {
                    trace_first_ts
                        .entry(trace_id)
                        .and_modify(|cur| {
                            if st < *cur {
                                *cur = st
                            }
                        })
                        .or_insert(st);
                }

                let obs_type = extract_string_attr(&span.attributes, "langfuse.observation.type")
                    .map(|s| s.to_uppercase());

                let model = extract_string_attr(&span.attributes, "langfuse.generation.model")
                    .or_else(|| extract_string_attr(&span.attributes, "gen_ai.request.model"));

                let input = extract_string_attr(&span.attributes, "langfuse.observation.input")
                    .and_then(|s| {
                        serde_json::from_str::<JsonValue>(&s)
                            .ok()
                            .or(Some(JsonValue::String(s)))
                    });

                let output = extract_string_attr(&span.attributes, "langfuse.observation.output")
                    .and_then(|s| {
                        serde_json::from_str::<JsonValue>(&s)
                            .ok()
                            .or(Some(JsonValue::String(s)))
                    });

                // trace-level promotion
                let trace_name = extract_string_attr(&span.attributes, "langfuse.trace.name");
                let user_id = extract_string_attr(&span.attributes, "user.id");
                let session_id = extract_string_attr(&span.attributes, "session.id");
                let tags = extract_array_string_attr(&span.attributes, "langfuse.trace.tags");
                let trace_meta = extract_prefixed_map(&span.attributes, "langfuse.trace.metadata.");

                trace_acc
                    .entry(trace_id)
                    .and_modify(|t| {
                        if t.name.is_none() {
                            t.name = trace_name.clone();
                        }
                        if t.userId.is_none() {
                            t.userId = user_id.clone();
                        }
                        if t.session_id.is_none() {
                            t.session_id = session_id.clone();
                        }
                        if t.tags.is_empty() {
                            if let Some(tags) = &tags {
                                t.tags = tags.clone();
                            }
                        }
                        if let Some(m) = &mut t.metadata {
                            if let JsonValue::Object(obj) = m {
                                for (k, v) in trace_meta.clone() {
                                    obj.insert(k, v);
                                }
                            }
                        } else if !trace_meta.is_empty() {
                            t.metadata = Some(JsonValue::Object(trace_meta.clone()));
                        }
                    })
                    .or_insert_with(|| TraceIngest {
                        id: trace_id,
                        timestamp: None,
                        name: trace_name.clone(),
                        input: None,
                        output: None,
                        session_id: session_id.clone(),
                        release: None,
                        version: None,
                        userId: user_id.clone(),
                        metadata: if trace_meta.is_empty() {
                            None
                        } else {
                            Some(JsonValue::Object(trace_meta.clone()))
                        },
                        tags: tags.unwrap_or_default(),
                        public: None,
                        environment: Some("default".to_string()),
                        externalId: None,
                        bookmarked: None,
                        latency: None,
                        totalCost: None,
                        projectId: Some(default_project_id.clone()),
                    });

                let mut meta = attributes_to_map(&span.attributes);
                if let Some(rattrs) = resource_attrs {
                    meta.insert(
                        "otel.resource".to_string(),
                        JsonValue::Object(attributes_to_map(rattrs)),
                    );
                }

                let (completion_tokens, prompt_tokens, total_tokens, usage_json) =
                    parse_usage_details(&span.attributes);

                let obs = ObservationIngest {
                    id: span_uuid,
                    traceId: trace_id,
                    r#type: obs_type,
                    name: Some(span.name),
                    startTime: start_time,
                    endTime: end_time,
                    completionStartTime: None,
                    model,
                    modelParameters: None,
                    input,
                    output,
                    usage: usage_json,
                    level: None,
                    statusMessage: None,
                    parentObservationId: parent_uuid,
                    promptId: None,
                    promptName: None,
                    promptVersion: None,
                    modelId: None,
                    inputPrice: None,
                    outputPrice: None,
                    totalPrice: None,
                    calculatedInputCost: None,
                    calculatedOutputCost: None,
                    calculatedTotalCost: None,
                    latency: None,
                    timeToFirstToken: None,

                    completionTokens: completion_tokens,
                    promptTokens: prompt_tokens,
                    totalTokens: total_tokens,
                    unit: None,
                    metadata: Some(JsonValue::Object(meta)),
                    environment: None,
                    projectId: Some(default_project_id.clone()),
                };

                per_trace.entry(trace_id).or_default().push(obs);
            }
        }
    }

    let mut out = Vec::with_capacity(per_trace.len());
    for (trace_id, observations) in per_trace {
        let timestamp = trace_first_ts.get(&trace_id).cloned();
        let mut trace = trace_acc.remove(&trace_id).unwrap_or(TraceIngest {
            id: trace_id,
            timestamp: None,
            name: None,
            input: None,
            output: None,
            session_id: None,
            release: None,
            version: None,
            userId: None,
            metadata: None,
            tags: vec![],
            public: None,
            environment: Some("default".to_string()),
            externalId: None,
            bookmarked: None,
            latency: None,
            totalCost: None,
            projectId: Some(default_project_id.clone()),
        });
        trace.timestamp = timestamp;
        out.push(BatchIngestRequest {
            trace: Some(trace),
            observations,
        });
    }
    Ok(out)
}

fn pb_to_otel_json(payload: PbExportTraceServiceRequest) -> OtelExportTraceServiceRequest {
    // 为了避免引入完整 OTLP 类型转换，这里走“protobuf -> serde_json -> 我们的 JSON 结构”路径。
    // prost Message 通常不直接支持 serde；因此我们只在 protobuf 情况下做最小字段提取。
    // 注意：这里会丢弃大量字段，但对 xinference 主要需求（traceId/spanId/name/attrs/time）已足够。
    let resource_spans = payload
        .resource_spans
        .into_iter()
        .map(|rs| {
            let resource = rs.resource.map(|r| OtelResource {
                attributes: r
                    .attributes
                    .into_iter()
                    .map(|kv| OtelKeyValue {
                        key: kv.key,
                        value: kv.value.map(|av| OtelAnyValue {
                            string_value: av.value.as_ref().and_then(|v| match v {
                                opentelemetry_proto::tonic::common::v1::any_value::Value::StringValue(s) => Some(s.clone()),
                                _ => None,
                            }),
                            int_value: av.value.as_ref().and_then(|v| match v {
                                opentelemetry_proto::tonic::common::v1::any_value::Value::IntValue(i) => Some(i.to_string()),
                                _ => None,
                            }),
                            double_value: av.value.as_ref().and_then(|v| match v {
                                opentelemetry_proto::tonic::common::v1::any_value::Value::DoubleValue(f) => Some(*f),
                                _ => None,
                            }),
                            bool_value: av.value.as_ref().and_then(|v| match v {
                                opentelemetry_proto::tonic::common::v1::any_value::Value::BoolValue(b) => Some(*b),
                                _ => None,
                            }),
                            array_value: av.value.as_ref().and_then(|v| match v {
                                opentelemetry_proto::tonic::common::v1::any_value::Value::ArrayValue(arr) => Some(OtelArrayValue {
                                    values: arr
                                        .values
                                        .iter()
                                        .filter_map(|x| x.value.as_ref())
                                        .map(|vv| OtelAnyValue {
                                            string_value: match vv {
                                                opentelemetry_proto::tonic::common::v1::any_value::Value::StringValue(s) => Some(s.clone()),
                                                _ => None,
                                            },
                                            int_value: match vv {
                                                opentelemetry_proto::tonic::common::v1::any_value::Value::IntValue(i) => Some(i.to_string()),
                                                _ => None,
                                            },
                                            double_value: match vv {
                                                opentelemetry_proto::tonic::common::v1::any_value::Value::DoubleValue(f) => Some(*f),
                                                _ => None,
                                            },
                                            bool_value: match vv {
                                                opentelemetry_proto::tonic::common::v1::any_value::Value::BoolValue(b) => Some(*b),
                                                _ => None,
                                            },
                                            array_value: None,
                                        })
                                        .collect(),
                                }),
                                _ => None,
                            }),
                        }),
                    })
                    .collect(),
            });

            let scope_spans = rs
                .scope_spans
                .into_iter()
                .map(|ss| OtelScopeSpan {
                    spans: ss
                        .spans
                        .into_iter()
                        .map(|s| OtelSpan {
                            trace_id: hex::encode(s.trace_id),
                            span_id: hex::encode(s.span_id),
                            parent_span_id: if s.parent_span_id.is_empty() {
                                None
                            } else {
                                Some(hex::encode(s.parent_span_id))
                            },
                            name: s.name,
                            start_time_unix_nano: if s.start_time_unix_nano == 0 {
                                None
                            } else {
                                Some(s.start_time_unix_nano.to_string())
                            },
                            end_time_unix_nano: if s.end_time_unix_nano == 0 {
                                None
                            } else {
                                Some(s.end_time_unix_nano.to_string())
                            },
                            attributes: s
                                .attributes
                                .into_iter()
                                .map(|kv| OtelKeyValue {
                                    key: kv.key,
                                    value: kv.value.map(|av| OtelAnyValue {
                                        string_value: av.value.as_ref().and_then(|v| match v {
                                            opentelemetry_proto::tonic::common::v1::any_value::Value::StringValue(s) => Some(s.clone()),
                                            _ => None,
                                        }),
                                        int_value: av.value.as_ref().and_then(|v| match v {
                                            opentelemetry_proto::tonic::common::v1::any_value::Value::IntValue(i) => Some(i.to_string()),
                                            _ => None,
                                        }),
                                        double_value: av.value.as_ref().and_then(|v| match v {
                                            opentelemetry_proto::tonic::common::v1::any_value::Value::DoubleValue(f) => Some(*f),
                                            _ => None,
                                        }),
                                        bool_value: av.value.as_ref().and_then(|v| match v {
                                            opentelemetry_proto::tonic::common::v1::any_value::Value::BoolValue(b) => Some(*b),
                                            _ => None,
                                        }),
                                        array_value: av.value.as_ref().and_then(|v| match v {
                                            opentelemetry_proto::tonic::common::v1::any_value::Value::ArrayValue(arr) => Some(OtelArrayValue {
                                                values: arr
                                                    .values
                                                    .iter()
                                                    .filter_map(|x| x.value.as_ref())
                                                    .map(|vv| OtelAnyValue {
                                                        string_value: match vv {
                                                            opentelemetry_proto::tonic::common::v1::any_value::Value::StringValue(s) => Some(s.clone()),
                                                            _ => None,
                                                        },
                                                        int_value: match vv {
                                                            opentelemetry_proto::tonic::common::v1::any_value::Value::IntValue(i) => Some(i.to_string()),
                                                            _ => None,
                                                        },
                                                        double_value: match vv {
                                                            opentelemetry_proto::tonic::common::v1::any_value::Value::DoubleValue(f) => Some(*f),
                                                            _ => None,
                                                        },
                                                        bool_value: match vv {
                                                            opentelemetry_proto::tonic::common::v1::any_value::Value::BoolValue(b) => Some(*b),
                                                            _ => None,
                                                        },
                                                        array_value: None,
                                                    })
                                                    .collect(),
                                            }),
                                            _ => None,
                                        }),
                                    }),
                                })
                                .collect(),
                        })
                        .collect(),
                })
                .collect();

            OtelResourceSpan {
                resource,
                scope_spans,
            }
        })
        .collect();

    OtelExportTraceServiceRequest { resource_spans }
}

async fn post_otel_traces(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    let raw = ungzip_if_needed(&headers, body)?;
    let ct = content_type(&headers).unwrap_or_else(|| "application/json".to_string());

    let otel: OtelExportTraceServiceRequest = if ct == "application/json" {
        serde_json::from_slice(&raw).map_err(|e| ApiError::BadRequest(format!("invalid json: {e}")))?
    } else if ct == "application/x-protobuf" {
        let pb =
            PbExportTraceServiceRequest::decode(raw.as_slice()).map_err(|e| ApiError::BadRequest(format!("invalid protobuf: {e}")))?;
        pb_to_otel_json(pb)
    } else {
        return Err(ApiError::BadRequest(format!("unsupported content-type: {ct}")));
    };

    let batches = map_otel_to_batches(&state, otel)?;
    for batch in batches {
        state
            .ingest_tx
            .try_send(batch)
            .map_err(|e| match e {
                mpsc::error::TrySendError::Full(_) => ApiError::TooManyRequests,
                mpsc::error::TrySendError::Closed(_) => ApiError::ServiceUnavailable,
            })?;
    }

    Ok((StatusCode::OK, Json(serde_json::json!({}))))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info,sqlx=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| anyhow::anyhow!("missing env DATABASE_URL"))?;
    let api_bearer_token = std::env::var("API_BEARER_TOKEN")
        .map_err(|_| anyhow::anyhow!("missing env API_BEARER_TOKEN"))?;

    let langfuse_public_key = std::env::var("XTRACE_PUBLIC_KEY")
        .ok()
        .or_else(|| std::env::var("LANGFUSE_PUBLIC_KEY").ok());
    let langfuse_secret_key = std::env::var("XTRACE_SECRET_KEY")
        .ok()
        .or_else(|| std::env::var("LANGFUSE_SECRET_KEY").ok());
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8742".to_string());
    let default_project_id =
        std::env::var("DEFAULT_PROJECT_ID").unwrap_or_else(|_| "default".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let (ingest_tx, ingest_rx) = mpsc::channel::<BatchIngestRequest>(1000);

    let state = AppState {
        pool,
        api_bearer_token: Arc::from(api_bearer_token),
        langfuse_public_key: langfuse_public_key.map(Arc::from),
        langfuse_secret_key: langfuse_secret_key.map(Arc::from),
        default_project_id: Arc::from(default_project_id),
        ingest_tx,
    };

    tokio::spawn(ingest_worker(
        state.pool.clone(),
        state.default_project_id.clone(),
        ingest_rx,
    ));

    let protected_routes = Router::new()
        .route("/v1/l/batch", post(post_batch))
        .route("/api/public/projects", get(get_projects))
        .route("/api/public/otel/v1/traces", post(post_otel_traces))
        .route("/api/public/metrics/daily", get(get_metrics_daily))
        .route("/api/public/traces", get(get_traces))
        .route("/api/public/traces/:traceId", get(get_trace))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth));

    let app = Router::new()
        .route("/healthz", get(healthz))
        .merge(protected_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = bind_addr.parse()?;
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn healthz() -> impl IntoResponse {
    StatusCode::OK
}

async fn auth(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> impl IntoResponse {
    let path = request.uri().path();
    let is_langfuse_compat = matches!(
        path,
        "/api/public/projects" | "/api/public/otel/v1/traces"
    );
    let langfuse_auth_not_configured =
        state.langfuse_public_key.is_none() && state.langfuse_secret_key.is_none();

    match extract_auth(&headers) {
        Ok(AuthHeader::Bearer(token)) if token == state.api_bearer_token.as_ref() => {
            next.run(request).await
        }
        Ok(AuthHeader::Basic { username, password })
            if state
                .langfuse_public_key
                .as_deref()
                .is_some_and(|k| k == username)
                && state
                    .langfuse_secret_key
                    .as_deref()
                    .is_some_and(|k| k == password) =>
        {
            next.run(request).await
        }
        Err(()) if is_langfuse_compat && langfuse_auth_not_configured => next.run(request).await,
        Ok(AuthHeader::Basic { .. }) if is_langfuse_compat && langfuse_auth_not_configured => {
            next.run(request).await
        }
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<serde_json::Value> {
                message: "Unauthorized".to_string(),
                data: None,
            }),
        )
            .into_response(),
    }
}

enum AuthHeader {
    Bearer(String),
    Basic { username: String, password: String },
}

fn extract_auth(headers: &HeaderMap) -> Result<AuthHeader, ()> {
    let value = headers
        .get(header::AUTHORIZATION)
        .ok_or(())
        .and_then(|v| v.to_str().map_err(|_| ()))?
        .trim();

    if let Some(rest) = value.strip_prefix("Bearer ") {
        return Ok(AuthHeader::Bearer(rest.trim().to_string()));
    }

    if let Some(rest) = value.strip_prefix("Basic ") {
        let decoded = BASE64_STANDARD
            .decode(rest.trim().as_bytes())
            .map_err(|_| ())?;
        let decoded = std::str::from_utf8(&decoded).map_err(|_| ())?;
        let (username, password) = decoded.split_once(':').ok_or(())?;
        return Ok(AuthHeader::Basic {
            username: username.to_string(),
            password: password.to_string(),
        });
    }

    Err(())
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
struct PageMeta {
    page: i64,
    limit: i64,
    totalItems: i64,
    totalPages: i64,
}

#[derive(Debug, Serialize)]
struct PagedData<T> {
    data: Vec<T>,
    meta: PageMeta,
}

#[derive(Debug, Deserialize)]
struct BatchIngestRequest {
    #[serde(default)]
    trace: Option<TraceIngest>,
    #[serde(default)]
    observations: Vec<ObservationIngest>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct TraceIngest {
    id: Uuid,
    #[serde(default)]
    timestamp: Option<DateTime<Utc>>,

    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    input: Option<JsonValue>,
    #[serde(default)]
    output: Option<JsonValue>,
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default)]
    release: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    userId: Option<String>,
    #[serde(default)]
    metadata: Option<JsonValue>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    public: Option<bool>,
    #[serde(default)]
    environment: Option<String>,
    #[serde(default)]
    externalId: Option<String>,
    #[serde(default)]
    bookmarked: Option<bool>,

    #[serde(default)]
    latency: Option<f64>,
    #[serde(default)]
    totalCost: Option<f64>,

    #[serde(default)]
    projectId: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ObservationIngest {
    id: Uuid,
    traceId: Uuid,

    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    name: Option<String>,

    #[serde(default)]
    startTime: Option<DateTime<Utc>>,
    #[serde(default)]
    endTime: Option<DateTime<Utc>>,
    #[serde(default)]
    completionStartTime: Option<DateTime<Utc>>,

    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    modelParameters: Option<JsonValue>,

    #[serde(default)]
    input: Option<JsonValue>,
    #[serde(default)]
    output: Option<JsonValue>,

    #[serde(default)]
    usage: Option<JsonValue>,

    #[serde(default)]
    level: Option<String>,
    #[serde(default)]
    statusMessage: Option<String>,
    #[serde(default)]
    parentObservationId: Option<Uuid>,

    #[serde(default)]
    promptId: Option<String>,
    #[serde(default)]
    promptName: Option<String>,
    #[serde(default)]
    promptVersion: Option<String>,

    #[serde(default)]
    modelId: Option<String>,

    #[serde(default)]
    inputPrice: Option<f64>,
    #[serde(default)]
    outputPrice: Option<f64>,
    #[serde(default)]
    totalPrice: Option<f64>,

    #[serde(default)]
    calculatedInputCost: Option<f64>,
    #[serde(default)]
    calculatedOutputCost: Option<f64>,
    #[serde(default)]
    calculatedTotalCost: Option<f64>,

    #[serde(default)]
    latency: Option<f64>,
    #[serde(default)]
    timeToFirstToken: Option<f64>,

    #[serde(default)]
    completionTokens: Option<i64>,
    #[serde(default)]
    promptTokens: Option<i64>,
    #[serde(default)]
    totalTokens: Option<i64>,
    #[serde(default)]
    unit: Option<String>,

    #[serde(default)]
    metadata: Option<JsonValue>,

    #[serde(default)]
    environment: Option<String>,

    #[serde(default)]
    projectId: Option<String>,
}

async fn post_batch(
    State(state): State<AppState>,
    Json(payload): Json<BatchIngestRequest>,
) -> Result<impl IntoResponse, ApiError> {
    match state.ingest_tx.try_send(payload) {
        Ok(()) => Ok((
            StatusCode::OK,
            Json(ApiResponse::<serde_json::Value> {
                message: "Request Successful.".to_string(),
                data: None,
            }),
        )),
        Err(mpsc::error::TrySendError::Full(_)) => Err(ApiError::TooManyRequests),
        Err(mpsc::error::TrySendError::Closed(_)) => Err(ApiError::ServiceUnavailable),
    }
}

async fn ingest_worker(
    pool: PgPool,
    default_project_id: Arc<str>,
    mut rx: mpsc::Receiver<BatchIngestRequest>,
) {
    const MAX_BATCHES: usize = 200;
    let window = Duration::from_millis(50);

    while let Some(first) = rx.recv().await {
        let mut batches = Vec::with_capacity(MAX_BATCHES);
        batches.push(first);

        let start = tokio::time::Instant::now();
        while batches.len() < MAX_BATCHES {
            let elapsed = start.elapsed();
            let remaining = match window.checked_sub(elapsed) {
                Some(r) if !r.is_zero() => r,
                _ => break,
            };

            match tokio::time::timeout(remaining, rx.recv()).await {
                Ok(Some(p)) => batches.push(p),
                Ok(None) => break,
                Err(_) => break,
            }
        }

        if let Err(err) = write_batches(&pool, default_project_id.as_ref(), batches).await {
            tracing::error!(error = ?err, "failed to write batch");
        }
    }
}

async fn write_batches(
    pool: &PgPool,
    default_project_id: &str,
    payloads: Vec<BatchIngestRequest>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    for payload in payloads {
        write_one(&mut tx, default_project_id, payload).await?;
    }
    tx.commit().await?;
    Ok(())
}

async fn write_one(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    default_project_id: &str,
    payload: BatchIngestRequest,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();

    if let Some(trace) = payload.trace {
        let project_id = trace.projectId.as_deref().unwrap_or(default_project_id);
        let timestamp = trace.timestamp.unwrap_or(now);
        let environment = trace.environment.unwrap_or_else(|| "default".to_string());

        sqlx::query(
            r#"
INSERT INTO traces (
  id, project_id, environment, timestamp, name, input, output, session_id, release, version, user_id,
  metadata, tags, public, external_id, bookmarked, latency, total_cost, created_at, updated_at
) VALUES (
  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
  $11, $12, $13, $14, $15, $16, $17, $18, NOW(), NOW()
)
ON CONFLICT (id) DO UPDATE SET
  project_id = EXCLUDED.project_id,
  environment = EXCLUDED.environment,
  timestamp = EXCLUDED.timestamp,
  name = EXCLUDED.name,
  input = EXCLUDED.input,
  output = EXCLUDED.output,
  session_id = EXCLUDED.session_id,
  release = EXCLUDED.release,
  version = EXCLUDED.version,
  user_id = EXCLUDED.user_id,
  metadata = EXCLUDED.metadata,
  tags = EXCLUDED.tags,
  public = EXCLUDED.public,
  external_id = EXCLUDED.external_id,
  bookmarked = EXCLUDED.bookmarked,
  latency = EXCLUDED.latency,
  total_cost = EXCLUDED.total_cost,
  updated_at = NOW()
            "#,
        )
        .bind(trace.id)
        .bind(project_id.to_string())
        .bind(environment.clone())
        .bind(timestamp)
        .bind(trace.name.clone())
        .bind(trace.input.clone())
        .bind(trace.output.clone())
        .bind(trace.session_id.clone())
        .bind(trace.release.clone())
        .bind(trace.version.clone())
        .bind(trace.userId.clone())
        .bind(trace.metadata.clone())
        .bind(trace.tags.clone())
        .bind(trace.public.unwrap_or(false))
        .bind(trace.externalId.clone())
        .bind(trace.bookmarked.unwrap_or(false))
        .bind(trace.latency)
        .bind(trace.totalCost)
        .execute(&mut **tx)
        .await?;
    }

    for obs in payload.observations {
        let project_id = obs.projectId.as_deref().unwrap_or(default_project_id);
        let environment = obs.environment.unwrap_or_else(|| "default".to_string());

        sqlx::query(
            r#"
INSERT INTO traces (id, project_id, environment, timestamp, created_at, updated_at)
VALUES ($1, $2, $3, NOW(), NOW(), NOW())
ON CONFLICT (id) DO NOTHING
            "#,
        )
        .bind(obs.traceId)
        .bind(project_id.to_string())
        .bind(environment.clone())
        .execute(&mut **tx)
        .await?;

        sqlx::query(
            r#"
INSERT INTO observations (
  id, trace_id, type, name, start_time, end_time, completion_start_time,
  model, model_parameters, input, output, usage, level, status_message,
  parent_observation_id, prompt_id, prompt_name, prompt_version, model_id,
  input_price, output_price, total_price,
  calculated_input_cost, calculated_output_cost, calculated_total_cost,
  latency, time_to_first_token,
  completion_tokens, prompt_tokens, total_tokens, unit,
  metadata, environment, project_id, created_at, updated_at
) VALUES (
  $1, $2, $3, $4, $5, $6, $7,
  $8, $9, $10, $11, $12, $13, $14,
  $15, $16, $17, $18, $19,
  $20, $21, $22,
  $23, $24, $25,
  $26, $27,
  $28, $29, $30, $31,
  $32, $33, $34, NOW(), NOW()
)
ON CONFLICT (id) DO UPDATE SET
  trace_id = EXCLUDED.trace_id,
  type = EXCLUDED.type,
  name = EXCLUDED.name,
  start_time = EXCLUDED.start_time,
  end_time = EXCLUDED.end_time,
  completion_start_time = EXCLUDED.completion_start_time,
  model = EXCLUDED.model,
  model_parameters = EXCLUDED.model_parameters,
  input = EXCLUDED.input,
  output = EXCLUDED.output,
  usage = EXCLUDED.usage,
  level = EXCLUDED.level,
  status_message = EXCLUDED.status_message,
  parent_observation_id = EXCLUDED.parent_observation_id,
  prompt_id = EXCLUDED.prompt_id,
  prompt_name = EXCLUDED.prompt_name,
  prompt_version = EXCLUDED.prompt_version,
  model_id = EXCLUDED.model_id,
  input_price = EXCLUDED.input_price,
  output_price = EXCLUDED.output_price,
  total_price = EXCLUDED.total_price,
  calculated_input_cost = EXCLUDED.calculated_input_cost,
  calculated_output_cost = EXCLUDED.calculated_output_cost,
  calculated_total_cost = EXCLUDED.calculated_total_cost,
  latency = EXCLUDED.latency,
  time_to_first_token = EXCLUDED.time_to_first_token,
  completion_tokens = EXCLUDED.completion_tokens,
  prompt_tokens = EXCLUDED.prompt_tokens,
  total_tokens = EXCLUDED.total_tokens,
  unit = EXCLUDED.unit,
  metadata = EXCLUDED.metadata,
  environment = EXCLUDED.environment,
  project_id = EXCLUDED.project_id,
  updated_at = NOW()
            "#,
        )
        .bind(obs.id)
        .bind(obs.traceId)
        .bind(obs.r#type.unwrap_or_else(|| "GENERATION".to_string()))
        .bind(obs.name.clone())
        .bind(obs.startTime)
        .bind(obs.endTime)
        .bind(obs.completionStartTime)
        .bind(obs.model.clone())
        .bind(obs.modelParameters.clone())
        .bind(obs.input.clone())
        .bind(obs.output.clone())
        .bind(obs.usage.clone())
        .bind(obs.level.clone())
        .bind(obs.statusMessage.clone())
        .bind(obs.parentObservationId)
        .bind(obs.promptId.clone())
        .bind(obs.promptName.clone())
        .bind(obs.promptVersion.clone())
        .bind(obs.modelId.clone())
        .bind(obs.inputPrice)
        .bind(obs.outputPrice)
        .bind(obs.totalPrice)
        .bind(obs.calculatedInputCost)
        .bind(obs.calculatedOutputCost)
        .bind(obs.calculatedTotalCost)
        .bind(obs.latency)
        .bind(obs.timeToFirstToken)
        .bind(obs.completionTokens)
        .bind(obs.promptTokens)
        .bind(obs.totalTokens)
        .bind(obs.unit.clone())
        .bind(obs.metadata.clone())
        .bind(environment)
        .bind(project_id.to_string())
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct MetricsDailyQuery {
    #[serde(default)]
    page: Option<i64>,
    #[serde(default)]
    limit: Option<i64>,

    #[serde(default, rename = "traceName")]
    trace_name: Option<String>,
    #[serde(default, rename = "userId")]
    user_id: Option<String>,
    #[serde(default)]
    tags: Vec<String>,

    #[serde(default, rename = "fromTimestamp")]
    from_timestamp: Option<DateTime<Utc>>,
    #[serde(default, rename = "toTimestamp")]
    to_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct MetricsDailyRow {
    day: NaiveDate,
    count_traces: i64,
    count_observations: i64,
    total_cost: f64,
    usage: JsonValue,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MetricsDailyItem {
    date: String,
    count_traces: i64,
    count_observations: i64,
    total_cost: f64,
    usage: JsonValue,
}

async fn get_metrics_daily(
    State(state): State<AppState>,
    Query(q): Query<MetricsDailyQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let page = q.page.unwrap_or(1).max(1);
    let limit = q.limit.unwrap_or(50).clamp(1, 200);
    let offset = (page - 1) * limit;

    let now = Utc::now();
    let to_ts = q.to_timestamp.unwrap_or(now);
    let from_ts = q
        .from_timestamp
        .unwrap_or_else(|| to_ts - chrono::Duration::days(30));

    let project_id = state.default_project_id.as_ref();

    let mut count_builder: QueryBuilder<'_, sqlx::Postgres> = QueryBuilder::new(
        "SELECT COUNT(*)::BIGINT FROM (SELECT date_trunc('day', t.\"timestamp\")::date AS day FROM traces t WHERE 1=1",
    );
    count_builder.push(" AND t.project_id = ");
    count_builder.push_bind(project_id.to_string());
    count_builder.push(" AND t.\"timestamp\" >= ");
    count_builder.push_bind(from_ts);
    count_builder.push(" AND t.\"timestamp\" <= ");
    count_builder.push_bind(to_ts);

    if let Some(trace_name) = &q.trace_name {
        count_builder.push(" AND t.name = ");
        count_builder.push_bind(trace_name.clone());
    }
    if let Some(user_id) = &q.user_id {
        count_builder.push(" AND t.user_id = ");
        count_builder.push_bind(user_id.clone());
    }
    if !q.tags.is_empty() {
        count_builder.push(" AND t.tags @> ");
        count_builder.push_bind(q.tags.clone());
    }
    count_builder.push(" GROUP BY 1) x");

    let total_items: i64 = count_builder
        .build_query_scalar()
        .fetch_one(&state.pool)
        .await?;

    let total_pages = if total_items == 0 {
        0
    } else {
        (total_items + limit - 1) / limit
    };

    let mut builder: QueryBuilder<'_, sqlx::Postgres> =
        QueryBuilder::new("WITH filtered_traces AS (SELECT t.* FROM traces t WHERE 1=1");
    builder.push(" AND t.project_id = ");
    builder.push_bind(project_id.to_string());
    builder.push(" AND t.\"timestamp\" >= ");
    builder.push_bind(from_ts);
    builder.push(" AND t.\"timestamp\" <= ");
    builder.push_bind(to_ts);

    if let Some(trace_name) = &q.trace_name {
        builder.push(" AND t.name = ");
        builder.push_bind(trace_name.clone());
    }
    if let Some(user_id) = &q.user_id {
        builder.push(" AND t.user_id = ");
        builder.push_bind(user_id.clone());
    }
    if !q.tags.is_empty() {
        builder.push(" AND t.tags @> ");
        builder.push_bind(q.tags.clone());
    }

    builder.push(
        ")\n, daily AS (\n  SELECT\n    date_trunc('day', ft.\"timestamp\")::date AS day,\n    COUNT(*)::BIGINT AS count_traces,\n    COALESCE(SUM(ft.total_cost), 0)::DOUBLE PRECISION AS total_cost\n  FROM filtered_traces ft\n  GROUP BY 1\n)\n, daily_obs AS (\n  SELECT\n    date_trunc('day', ft.\"timestamp\")::date AS day,\n    COUNT(o.id)::BIGINT AS count_observations\n  FROM filtered_traces ft\n  JOIN observations o ON o.trace_id = ft.id\n  GROUP BY 1\n)\n, model_usage AS (\n  SELECT\n    date_trunc('day', ft.\"timestamp\")::date AS day,\n    COALESCE(o.model, 'unknown') AS model,\n    COALESCE(SUM(o.prompt_tokens), 0)::BIGINT AS input_usage,\n    COALESCE(SUM(o.completion_tokens), 0)::BIGINT AS output_usage,\n    COALESCE(SUM(o.total_tokens), 0)::BIGINT AS total_usage,\n    COUNT(DISTINCT ft.id)::BIGINT AS count_traces,\n    COUNT(o.id)::BIGINT AS count_observations,\n    COALESCE(SUM(o.calculated_total_cost), 0)::DOUBLE PRECISION AS total_cost\n  FROM filtered_traces ft\n  JOIN observations o ON o.trace_id = ft.id\n  WHERE o.type = 'GENERATION'\n  GROUP BY 1, 2\n)\n, daily_usage AS (\n  SELECT\n    mu.day,\n    COALESCE(jsonb_agg(\n      jsonb_build_object(\n        'model', mu.model,\n        'inputUsage', mu.input_usage,\n        'outputUsage', mu.output_usage,\n        'totalUsage', mu.total_usage,\n        'countTraces', mu.count_traces,\n        'countObservations', mu.count_observations,\n        'totalCost', mu.total_cost\n      ) ORDER BY mu.total_cost DESC\n    ), '[]'::jsonb) AS usage\n  FROM model_usage mu\n  GROUP BY 1\n)\nSELECT\n  d.day AS day,\n  d.count_traces AS count_traces,\n  COALESCE(dob.count_observations, 0) AS count_observations,\n  d.total_cost AS total_cost,\n  COALESCE(du.usage, '[]'::jsonb) AS usage\nFROM daily d\nLEFT JOIN daily_obs dob ON dob.day = d.day\nLEFT JOIN daily_usage du ON du.day = d.day\nORDER BY d.day DESC\nLIMIT ",
    );
    builder.push_bind(limit);
    builder.push(" OFFSET ");
    builder.push_bind(offset);

    let rows: Vec<MetricsDailyRow> = builder.build_query_as().fetch_all(&state.pool).await?;

    let items = rows
        .into_iter()
        .map(|r| MetricsDailyItem {
            date: r.day.to_string(),
            count_traces: r.count_traces,
            count_observations: r.count_observations,
            total_cost: r.total_cost,
            usage: r.usage,
        })
        .collect::<Vec<_>>();

    Ok((
        StatusCode::OK,
        Json(PagedData {
            data: items,
            meta: PageMeta {
                page,
                limit,
                totalItems: total_items,
                totalPages: total_pages,
            },
        }),
    ))
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct TraceListQuery {
    #[serde(default)]
    page: Option<i64>,
    #[serde(default)]
    limit: Option<i64>,

    #[serde(default, rename = "userId")]
    user_id: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default, rename = "sessionId")]
    session_id: Option<String>,

    #[serde(default, rename = "fromTimestamp")]
    from_timestamp: Option<DateTime<Utc>>,
    #[serde(default, rename = "toTimestamp")]
    to_timestamp: Option<DateTime<Utc>>,

    #[serde(default, rename = "orderBy")]
    order_by: Option<String>,

    #[serde(default)]
    tags: Vec<String>,

    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    release: Option<String>,
    #[serde(default)]
    environment: Vec<String>,

    #[serde(default)]
    fields: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct TraceListRow {
    id: Uuid,
    project_id: String,
    timestamp: DateTime<Utc>,
    name: Option<String>,
    input: Option<JsonValue>,
    output: Option<JsonValue>,
    session_id: Option<String>,
    release: Option<String>,
    version: Option<String>,
    user_id: Option<String>,
    metadata: Option<JsonValue>,
    tags: Vec<String>,
    public: bool,
    environment: String,
    latency: Option<f64>,
    total_cost: Option<f64>,
    observations: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TraceListItem {
    id: Uuid,
    timestamp: DateTime<Utc>,
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<JsonValue>,
    session_id: Option<String>,
    release: Option<String>,
    version: Option<String>,
    user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<JsonValue>,
    tags: Vec<String>,
    public: bool,
    environment: String,
    html_path: String,
    latency: Option<f64>,
    total_cost: Option<f64>,
    observations: Vec<String>,
    scores: Vec<String>,
}

#[derive(Clone, Copy)]
struct TraceFieldsMask {
    io: bool,
    scores: bool,
    observations: bool,
    metrics: bool,
}

fn parse_trace_fields(fields: Option<&str>) -> TraceFieldsMask {
    let Some(fields) = fields else {
        return TraceFieldsMask {
            io: true,
            scores: true,
            observations: true,
            metrics: true,
        };
    };

    let set: HashSet<&str> = fields
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    TraceFieldsMask {
        io: set.contains("io"),
        scores: set.contains("scores"),
        observations: set.contains("observations"),
        metrics: set.contains("metrics"),
    }
}

async fn get_traces(
    State(state): State<AppState>,
    Query(q): Query<TraceListQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let page = q.page.unwrap_or(1).max(1);
    let limit = q.limit.unwrap_or(50).clamp(1, 200);
    let offset = (page - 1) * limit;

    let fields = parse_trace_fields(q.fields.as_deref());

    let (order_column, order_desc) = parse_order_by(q.order_by.as_deref())?;

    let mut count_builder: QueryBuilder<'_, sqlx::Postgres> =
        QueryBuilder::new("SELECT COUNT(*)::BIGINT AS cnt FROM traces t WHERE 1=1");
    count_builder.push(" AND t.project_id = ");
    count_builder.push_bind(state.default_project_id.to_string());
    apply_trace_filters(&mut count_builder, &q);

    let total_items: i64 = count_builder
        .build_query_scalar()
        .fetch_one(&state.pool)
        .await?;

    let total_pages = if total_items == 0 {
        0
    } else {
        (total_items + limit - 1) / limit
    };

    let mut builder: QueryBuilder<'_, sqlx::Postgres> = QueryBuilder::new(
        r#"
SELECT
  t.id,
  t.project_id,
  t.timestamp,
  t.name,
  t.input,
  t.output,
  t.session_id,
  t.release,
  t.version,
  t.user_id,
  t.metadata,
  t.tags,
  t.public,
  t.environment,
  t.latency,
  t.total_cost,
  COALESCE(array_agg(o.id) FILTER (WHERE o.id IS NOT NULL), '{}') AS observations
FROM traces t
LEFT JOIN observations o ON o.trace_id = t.id
WHERE 1=1
        "#,
    );

    builder.push(" AND t.project_id = ");
    builder.push_bind(state.default_project_id.to_string());

    apply_trace_filters(&mut builder, &q);
    builder.push(" GROUP BY t.id");

    builder.push(" ORDER BY ");
    builder.push(order_column);
    builder.push(if order_desc { " DESC" } else { " ASC" });
    builder.push(" LIMIT ");
    builder.push_bind(limit);
    builder.push(" OFFSET ");
    builder.push_bind(offset);

    let rows: Vec<TraceListRow> = builder.build_query_as().fetch_all(&state.pool).await?;

    let items = rows
        .into_iter()
        .map(|r| {
            let observations = if fields.observations {
                r.observations
                    .into_iter()
                    .map(|id| id.to_string())
                    .collect()
            } else {
                vec![]
            };
            let scores = if fields.scores {
                Vec::new()
            } else {
                Vec::with_capacity(0)
            };

            let latency = if fields.metrics {
                r.latency
            } else {
                Some(-1.0)
            };
            let total_cost = if fields.metrics {
                r.total_cost
            } else {
                Some(-1.0)
            };

            TraceListItem {
                html_path: format!("/project/{}/traces/{}", r.project_id, r.id),
                id: r.id,
                timestamp: r.timestamp,
                name: r.name,
                input: if fields.io {
                    Some(r.input.unwrap_or(JsonValue::Null))
                } else {
                    None
                },
                output: if fields.io {
                    Some(r.output.unwrap_or(JsonValue::Null))
                } else {
                    None
                },
                session_id: r.session_id,
                release: r.release,
                version: r.version,
                user_id: r.user_id,
                metadata: if fields.io {
                    Some(r.metadata.unwrap_or(JsonValue::Null))
                } else {
                    None
                },
                tags: r.tags,
                public: r.public,
                environment: r.environment,
                latency,
                total_cost,
                observations,
                scores,
            }
        })
        .collect::<Vec<_>>();

    Ok((
        StatusCode::OK,
        Json(PagedData {
            data: items,
            meta: PageMeta {
                page,
                limit,
                totalItems: total_items,
                totalPages: total_pages,
            },
        }),
    ))
}

fn apply_trace_filters(builder: &mut QueryBuilder<'_, sqlx::Postgres>, q: &TraceListQuery) {
    if let Some(user_id) = &q.user_id {
        builder.push(" AND t.user_id = ");
        builder.push_bind(user_id.clone());
    }
    if let Some(name) = &q.name {
        builder.push(" AND t.name = ");
        builder.push_bind(name.clone());
    }
    if let Some(session_id) = &q.session_id {
        builder.push(" AND t.session_id = ");
        builder.push_bind(session_id.clone());
    }
    if let Some(from_ts) = &q.from_timestamp {
        builder.push(" AND t.timestamp >= ");
        builder.push_bind(*from_ts);
    }
    if let Some(to_ts) = &q.to_timestamp {
        builder.push(" AND t.timestamp <= ");
        builder.push_bind(*to_ts);
    }
    if !q.tags.is_empty() {
        builder.push(" AND t.tags @> ");
        builder.push_bind(q.tags.clone());
    }

    if let Some(version) = &q.version {
        builder.push(" AND t.version = ");
        builder.push_bind(version.clone());
    }
    if let Some(release) = &q.release {
        builder.push(" AND t.release = ");
        builder.push_bind(release.clone());
    }
    if !q.environment.is_empty() {
        builder.push(" AND t.environment = ANY(");
        builder.push_bind(q.environment.clone());
        builder.push(")");
    }
}

fn parse_order_by(order_by: Option<&str>) -> Result<(&'static str, bool), ApiError> {
    let s = order_by.unwrap_or("timestamp.desc").trim();
    let (col, dir) = s.split_once('.').unwrap_or((s, "desc"));
    let (col, default_desc) = match col {
        "id" => ("t.id", true),
        "timestamp" => ("t.timestamp", true),
        "name" => ("t.name", false),
        "userId" | "user_id" => ("t.user_id", false),
        "release" => ("t.release", false),
        "version" => ("t.version", false),
        "public" => ("t.public", true),
        "bookmarked" => ("t.bookmarked", true),
        "sessionId" | "session_id" => ("t.session_id", false),
        "latency" => ("t.latency", true),
        "totalCost" | "total_cost" => ("t.total_cost", true),
        _ => return Err(ApiError::BadRequest("invalid order_by".into())),
    };
    let desc = match dir {
        "desc" => true,
        "asc" => false,
        _ => default_desc,
    };
    Ok((col, desc))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct ObservationRow {
    id: Uuid,
    trace_id: Uuid,
    r#type: String,
    name: Option<String>,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    completion_start_time: Option<DateTime<Utc>>,
    model: Option<String>,
    model_parameters: Option<JsonValue>,
    input: Option<JsonValue>,
    output: Option<JsonValue>,
    usage: Option<JsonValue>,
    level: Option<String>,
    status_message: Option<String>,
    parent_observation_id: Option<Uuid>,
    prompt_id: Option<String>,
    prompt_name: Option<String>,
    prompt_version: Option<String>,
    model_id: Option<String>,
    input_price: Option<f64>,
    output_price: Option<f64>,
    total_price: Option<f64>,
    calculated_input_cost: Option<f64>,
    calculated_output_cost: Option<f64>,
    calculated_total_cost: Option<f64>,
    latency: Option<f64>,
    time_to_first_token: Option<f64>,
    completion_tokens: Option<i64>,
    prompt_tokens: Option<i64>,
    total_tokens: Option<i64>,
    unit: Option<String>,
    metadata: Option<JsonValue>,
    environment: String,
    project_id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PublicUsage {
    input: i64,
    output: i64,
    total: i64,
    unit: Option<String>,
    input_cost: Option<f64>,
    output_cost: Option<f64>,
    total_cost: Option<f64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ObservationsViewDto {
    id: Uuid,
    trace_id: Option<Uuid>,
    r#type: String,
    name: Option<String>,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    completion_start_time: Option<DateTime<Utc>>,
    model: Option<String>,
    model_parameters: JsonValue,
    input: JsonValue,
    version: Option<String>,
    metadata: JsonValue,
    output: JsonValue,
    usage: PublicUsage,
    level: String,
    status_message: Option<String>,
    parent_observation_id: Option<Uuid>,
    prompt_id: Option<String>,
    prompt_name: Option<String>,
    prompt_version: Option<i64>,
    model_id: Option<String>,
    input_price: Option<f64>,
    output_price: Option<f64>,
    total_price: Option<f64>,
    calculated_input_cost: Option<f64>,
    calculated_output_cost: Option<f64>,
    calculated_total_cost: Option<f64>,
    latency: Option<f64>,
    time_to_first_token: Option<f64>,
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
    usage_details: JsonValue,
    cost_details: JsonValue,
    environment: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct TraceRow {
    id: Uuid,
    timestamp: DateTime<Utc>,
    name: Option<String>,
    input: Option<JsonValue>,
    output: Option<JsonValue>,
    session_id: Option<String>,
    release: Option<String>,
    version: Option<String>,
    user_id: Option<String>,
    metadata: Option<JsonValue>,
    tags: Vec<String>,
    public: bool,
    environment: String,
    latency: Option<f64>,
    total_cost: Option<f64>,
    external_id: Option<String>,
    bookmarked: bool,
    project_id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TraceDetailDto {
    id: Uuid,
    timestamp: DateTime<Utc>,
    name: Option<String>,
    input: JsonValue,
    output: JsonValue,
    session_id: Option<String>,
    release: Option<String>,
    version: Option<String>,
    user_id: Option<String>,
    metadata: JsonValue,
    tags: Vec<String>,
    public: bool,
    environment: String,
    html_path: String,
    latency: Option<f64>,
    total_cost: Option<f64>,
    observations: Vec<ObservationsViewDto>,
    scores: Vec<ScoreV1Dto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ScoreV1Dto {
    #[serde(rename = "dataType")]
    data_type: String,

    id: String,
    trace_id: String,
    name: String,
    source: String,
    observation_id: Option<String>,
    timestamp: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    author_user_id: Option<String>,
    comment: Option<String>,
    metadata: JsonValue,
    config_id: Option<String>,
    queue_id: Option<String>,
    environment: String,

    value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<String>,
}

async fn get_trace(
    State(state): State<AppState>,
    Path(trace_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let trace: Option<TraceRow> = sqlx::query_as(
        r#"
SELECT
  id,
  timestamp,
  name,
  input,
  output,
  session_id,
  release,
  version,
  user_id,
  metadata,
  tags,
  public,
  environment,
  latency,
  total_cost,
  external_id,
  bookmarked,
  project_id,
  created_at,
  updated_at
FROM traces
WHERE id = $1
         "#,
    )
    .bind(trace_id)
    .fetch_optional(&state.pool)
    .await?;

    let Some(trace) = trace else {
        return Err(ApiError::NotFound);
    };

    let observations: Vec<ObservationRow> = sqlx::query_as(
        r#"
SELECT
  id,
  trace_id,
  type as "type",
  name,
  start_time,
  end_time,
  completion_start_time,
  model,
  model_parameters,
  input,
  output,
  usage,
  level,
  status_message,
  parent_observation_id,
  prompt_id,
  prompt_name,
  prompt_version,
  model_id,
  input_price,
  output_price,
  total_price,
  calculated_input_cost,
  calculated_output_cost,
  calculated_total_cost,
  latency,
  time_to_first_token,
  completion_tokens,
  prompt_tokens,
  total_tokens,
  unit,
  metadata,
  environment,
  project_id,
  created_at,
  updated_at
FROM observations
WHERE trace_id = $1
ORDER BY start_time NULLS LAST, created_at
         "#,
    )
    .bind(trace_id)
    .fetch_all(&state.pool)
    .await?;

    let obs_dtos = observations
        .into_iter()
        .map(|o| {
            let prompt_tokens = o.prompt_tokens.unwrap_or(0);
            let completion_tokens = o.completion_tokens.unwrap_or(0);
            let total_tokens = o.total_tokens.unwrap_or(0);
            let calculated_input_cost = o.calculated_input_cost.unwrap_or(0.0);
            let calculated_output_cost = o.calculated_output_cost.unwrap_or(0.0);
            let calculated_total_cost = o.calculated_total_cost.unwrap_or(0.0);

            ObservationsViewDto {
                version: None,
                id: o.id,
                trace_id: Some(o.trace_id),
                r#type: o.r#type,
                name: o.name,
                start_time: o.start_time.unwrap_or(o.created_at),
                end_time: o.end_time,
                completion_start_time: o.completion_start_time,
                model: o.model,
                model_parameters: o.model_parameters.unwrap_or_else(|| serde_json::json!({})),
                input: o.input.unwrap_or(JsonValue::Null),
                metadata: o.metadata.unwrap_or(JsonValue::Null),
                output: o.output.unwrap_or(JsonValue::Null),
                usage: PublicUsage {
                    input: prompt_tokens,
                    output: completion_tokens,
                    total: total_tokens,
                    unit: o.unit.clone(),
                    input_cost: o.calculated_input_cost,
                    output_cost: o.calculated_output_cost,
                    total_cost: o.calculated_total_cost,
                },
                usage_details: serde_json::json!({
                    "input": prompt_tokens,
                    "output": completion_tokens,
                    "total": total_tokens
                }),
                cost_details: serde_json::json!({
                    "input": calculated_input_cost,
                    "output": calculated_output_cost,
                    "total": calculated_total_cost
                }),
                level: o.level.unwrap_or_else(|| "DEFAULT".to_string()),
                status_message: o.status_message,
                parent_observation_id: o.parent_observation_id,
                prompt_id: o.prompt_id,
                prompt_name: o.prompt_name,
                prompt_version: o
                    .prompt_version
                    .as_deref()
                    .and_then(|s| s.parse::<i64>().ok()),
                model_id: o.model_id,
                input_price: o.input_price,
                output_price: o.output_price,
                total_price: o.total_price,
                calculated_input_cost: o.calculated_input_cost,
                calculated_output_cost: o.calculated_output_cost,
                calculated_total_cost: o.calculated_total_cost,
                latency: o.latency,
                time_to_first_token: o.time_to_first_token,
                prompt_tokens,
                completion_tokens,
                total_tokens,
                environment: o.environment,
            }
        })
        .collect::<Vec<_>>();

    let dto = TraceDetailDto {
        html_path: format!("/project/{}/traces/{}", trace.project_id, trace.id),
        scores: vec![],
        id: trace.id,
        timestamp: trace.timestamp,
        name: trace.name,
        input: trace.input.unwrap_or(JsonValue::Null),
        output: trace.output.unwrap_or(JsonValue::Null),
        session_id: trace.session_id,
        release: trace.release,
        version: trace.version,
        user_id: trace.user_id,
        metadata: trace.metadata.unwrap_or(JsonValue::Null),
        tags: trace.tags,
        public: trace.public,
        environment: trace.environment,
        latency: trace.latency,
        total_cost: trace.total_cost,
        observations: obs_dtos,
    };

    Ok((StatusCode::OK, Json(dto)))
}

#[derive(Debug, Error)]
enum ApiError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("not found")]
    NotFound,
    #[error("too many requests")]
    TooManyRequests,
    #[error("service unavailable")]
    ServiceUnavailable,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        if let ApiError::Sqlx(err) = &self {
            tracing::error!(error = %err, "sqlx error");
        }

        let (status, msg) = match self {
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            ApiError::TooManyRequests => (
                StatusCode::TOO_MANY_REQUESTS,
                "Too Many Requests".to_string(),
            ),
            ApiError::ServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service Unavailable".to_string(),
            ),
            ApiError::Sqlx(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Error".to_string(),
            ),
        };

        let body = Json(ApiResponse::<serde_json::Value> {
            message: msg,
            data: None,
        });

        (status, body).into_response()
    }
}
