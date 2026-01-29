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
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::{postgres::PgPoolOptions, PgPool, QueryBuilder};
use std::{collections::HashSet, net::SocketAddr, sync::Arc};
use thiserror::Error;
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
    let langfuse_public_key = std::env::var("LANGFUSE_PUBLIC_KEY").ok();
    let langfuse_secret_key = std::env::var("LANGFUSE_SECRET_KEY").ok();
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let default_project_id =
        std::env::var("DEFAULT_PROJECT_ID").unwrap_or_else(|_| "default".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

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
        _ => StatusCode::UNAUTHORIZED.into_response(),
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
            let scores = if fields.scores { vec![] } else { vec![] };

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
        builder.push_bind(from_ts.clone());
    }
    if let Some(to_ts) = &q.to_timestamp {
        builder.push(" AND t.timestamp <= ");
        builder.push_bind(to_ts.clone());
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
