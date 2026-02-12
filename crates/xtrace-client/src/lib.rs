use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid base url: {0}")]
    InvalidBaseUrl(#[from] url::ParseError),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}

#[derive(Clone)]
pub struct Client {
    base_url: Url,
    http: reqwest::Client,
}

impl Client {
    pub fn new(base_url: &str, bearer_token: &str) -> Result<Self, Error> {
        let base_url = Url::parse(base_url)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", bearer_token)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self { base_url, http })
    }

    pub async fn healthz(&self) -> Result<(), Error> {
        let url = self.base_url.join("healthz")?;
        self.http.get(url).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn ingest_batch(&self, req: &BatchIngestRequest) -> Result<ApiResponse<JsonValue>, Error> {
        let url = self.base_url.join("v1/l/batch")?;
        let res = self
            .http
            .post(url)
            .json(req)
            .send()
            .await?
            .error_for_status()?;
        Ok(res.json::<ApiResponse<JsonValue>>().await?)
    }

    pub async fn list_traces(&self, q: &TraceListQuery) -> Result<PagedData<TraceListItem>, Error> {
        let mut url = self.base_url.join("api/public/traces")?;
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(v) = q.page {
                pairs.append_pair("page", &v.to_string());
            }
            if let Some(v) = q.limit {
                pairs.append_pair("limit", &v.to_string());
            }
            if let Some(v) = q.user_id.as_deref() {
                pairs.append_pair("userId", v);
            }
            if let Some(v) = q.name.as_deref() {
                pairs.append_pair("name", v);
            }
            if let Some(v) = q.session_id.as_deref() {
                pairs.append_pair("sessionId", v);
            }
            if let Some(v) = q.from_timestamp.as_ref() {
                pairs.append_pair("fromTimestamp", &v.to_rfc3339());
            }
            if let Some(v) = q.to_timestamp.as_ref() {
                pairs.append_pair("toTimestamp", &v.to_rfc3339());
            }
            if let Some(v) = q.order_by.as_deref() {
                pairs.append_pair("orderBy", v);
            }
            for tag in &q.tags {
                pairs.append_pair("tags", tag);
            }
            if let Some(v) = q.version.as_deref() {
                pairs.append_pair("version", v);
            }
            if let Some(v) = q.release.as_deref() {
                pairs.append_pair("release", v);
            }
            for env in &q.environment {
                pairs.append_pair("environment", env);
            }
            if let Some(v) = q.fields.as_deref() {
                pairs.append_pair("fields", v);
            }
        }

        let res = self.http.get(url).send().await?.error_for_status()?;
        Ok(res.json::<PagedData<TraceListItem>>().await?)
    }

    pub async fn get_trace(&self, trace_id: Uuid) -> Result<TraceDetailDto, Error> {
        let url = self
            .base_url
            .join(&format!("api/public/traces/{}", trace_id))?;
        let res = self.http.get(url).send().await?.error_for_status()?;
        Ok(res.json::<TraceDetailDto>().await?)
    }

    pub async fn metrics_daily(&self, q: &MetricsDailyQuery) -> Result<PagedData<MetricsDailyItem>, Error> {
        let mut url = self.base_url.join("api/public/metrics/daily")?;
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(v) = q.page {
                pairs.append_pair("page", &v.to_string());
            }
            if let Some(v) = q.limit {
                pairs.append_pair("limit", &v.to_string());
            }
            if let Some(v) = q.trace_name.as_deref() {
                pairs.append_pair("traceName", v);
            }
            if let Some(v) = q.user_id.as_deref() {
                pairs.append_pair("userId", v);
            }
            for tag in &q.tags {
                pairs.append_pair("tags", tag);
            }
            if let Some(v) = q.from_timestamp.as_ref() {
                pairs.append_pair("fromTimestamp", &v.to_rfc3339());
            }
            if let Some(v) = q.to_timestamp.as_ref() {
                pairs.append_pair("toTimestamp", &v.to_rfc3339());
            }
        }

        let res = self.http.get(url).send().await?.error_for_status()?;
        Ok(res.json::<PagedData<MetricsDailyItem>>().await?)
    }

    pub async fn push_metrics(&self, metrics: &[MetricPoint]) -> Result<ApiResponse<JsonValue>, Error> {
        let url = self.base_url.join("v1/metrics/batch")?;
        let req = MetricsBatchRequest {
            metrics: metrics.to_vec(),
        };
        let res = self
            .http
            .post(url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?;
        Ok(res.json::<ApiResponse<JsonValue>>().await?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricPoint {
    pub name: String,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct MetricsBatchRequest {
    metrics: Vec<MetricPoint>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub message: String,
    #[serde(default)]
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMeta {
    pub page: i64,
    pub limit: i64,
    pub total_items: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize)]
pub struct PagedData<T> {
    pub data: Vec<T>,
    pub meta: PageMeta,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchIngestRequest {
    #[serde(default)]
    pub trace: Option<TraceIngest>,
    #[serde(default)]
    pub observations: Vec<ObservationIngest>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceIngest {
    pub id: Uuid,
    #[serde(default)]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub input: Option<JsonValue>,
    #[serde(default)]
    pub output: Option<JsonValue>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub release: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default, rename = "userId")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub metadata: Option<JsonValue>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub public: Option<bool>,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(default)]
    pub external_id: Option<String>,
    #[serde(default)]
    pub bookmarked: Option<bool>,

    #[serde(default)]
    pub latency: Option<f64>,
    #[serde(default, rename = "totalCost")]
    pub total_cost: Option<f64>,

    #[serde(default, rename = "projectId")]
    pub project_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationIngest {
    pub id: Uuid,
    #[serde(rename = "traceId")]
    pub trace_id: Uuid,

    #[serde(default, rename = "type")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub completion_start_time: Option<DateTime<Utc>>,

    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub model_parameters: Option<JsonValue>,

    #[serde(default)]
    pub input: Option<JsonValue>,
    #[serde(default)]
    pub output: Option<JsonValue>,

    #[serde(default)]
    pub usage: Option<JsonValue>,

    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub parent_observation_id: Option<Uuid>,

    #[serde(default)]
    pub prompt_id: Option<String>,
    #[serde(default)]
    pub prompt_name: Option<String>,
    #[serde(default)]
    pub prompt_version: Option<String>,

    #[serde(default)]
    pub model_id: Option<String>,

    #[serde(default)]
    pub input_price: Option<f64>,
    #[serde(default)]
    pub output_price: Option<f64>,
    #[serde(default)]
    pub total_price: Option<f64>,

    #[serde(default)]
    pub calculated_input_cost: Option<f64>,
    #[serde(default)]
    pub calculated_output_cost: Option<f64>,
    #[serde(default)]
    pub calculated_total_cost: Option<f64>,

    #[serde(default)]
    pub latency: Option<f64>,
    #[serde(default)]
    pub time_to_first_token: Option<f64>,

    #[serde(default)]
    pub completion_tokens: Option<i64>,
    #[serde(default)]
    pub prompt_tokens: Option<i64>,
    #[serde(default)]
    pub total_tokens: Option<i64>,
    #[serde(default)]
    pub unit: Option<String>,

    #[serde(default)]
    pub metadata: Option<JsonValue>,

    #[serde(default)]
    pub environment: Option<String>,

    #[serde(default, rename = "projectId")]
    pub project_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceListQuery {
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,

    #[serde(default, rename = "userId")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "sessionId")]
    pub session_id: Option<String>,

    #[serde(default, rename = "fromTimestamp")]
    pub from_timestamp: Option<DateTime<Utc>>,
    #[serde(default, rename = "toTimestamp")]
    pub to_timestamp: Option<DateTime<Utc>>,

    #[serde(default, rename = "orderBy")]
    pub order_by: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub release: Option<String>,
    #[serde(default)]
    pub environment: Vec<String>,

    #[serde(default)]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceListItem {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub name: Option<String>,
    #[serde(default)]
    pub input: Option<JsonValue>,
    #[serde(default)]
    pub output: Option<JsonValue>,
    pub session_id: Option<String>,
    pub release: Option<String>,
    pub version: Option<String>,
    pub user_id: Option<String>,
    #[serde(default)]
    pub metadata: Option<JsonValue>,
    pub tags: Vec<String>,
    pub public: bool,
    pub environment: String,
    pub html_path: String,
    pub latency: Option<f64>,
    pub total_cost: Option<f64>,
    pub observations: Vec<String>,
    pub scores: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsDailyQuery {
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,

    #[serde(default, rename = "traceName")]
    pub trace_name: Option<String>,
    #[serde(default, rename = "userId")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default, rename = "fromTimestamp")]
    pub from_timestamp: Option<DateTime<Utc>>,
    #[serde(default, rename = "toTimestamp")]
    pub to_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsDailyItem {
    pub date: String,
    pub count_traces: i64,
    pub count_observations: i64,
    pub total_cost: f64,
    pub usage: JsonValue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceDetailDto {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub name: Option<String>,
    pub input: JsonValue,
    pub output: JsonValue,
    pub session_id: Option<String>,
    pub release: Option<String>,
    pub version: Option<String>,
    pub user_id: Option<String>,
    pub metadata: JsonValue,
    pub tags: Vec<String>,
    pub public: bool,
    pub environment: String,
    pub html_path: String,
    pub latency: Option<f64>,
    pub total_cost: Option<f64>,
    pub observations: Vec<JsonValue>,
    pub scores: Vec<JsonValue>,
}
