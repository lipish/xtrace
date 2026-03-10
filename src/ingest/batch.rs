use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value as JsonValue;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration};
use uuid::Uuid;

use crate::{
    http::{common::ApiResponse, error::ApiError},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub(crate) struct BatchIngestRequest {
    #[serde(default)]
    pub trace: Option<TraceIngest>,
    #[serde(default)]
    pub observations: Vec<ObservationIngest>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct TraceIngest {
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
    #[serde(default)]
    pub userId: Option<String>,
    #[serde(default)]
    pub metadata: Option<JsonValue>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub public: Option<bool>,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(default)]
    pub externalId: Option<String>,
    #[serde(default)]
    pub bookmarked: Option<bool>,

    #[serde(default)]
    pub latency: Option<f64>,
    #[serde(default)]
    pub totalCost: Option<f64>,

    #[serde(default)]
    pub projectId: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct ObservationIngest {
    pub id: Uuid,
    pub traceId: Uuid,

    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub startTime: Option<DateTime<Utc>>,
    #[serde(default)]
    pub endTime: Option<DateTime<Utc>>,
    #[serde(default)]
    pub completionStartTime: Option<DateTime<Utc>>,

    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub modelParameters: Option<JsonValue>,

    #[serde(default)]
    pub input: Option<JsonValue>,
    #[serde(default)]
    pub output: Option<JsonValue>,

    #[serde(default)]
    pub usage: Option<JsonValue>,

    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub statusMessage: Option<String>,
    #[serde(default)]
    pub parentObservationId: Option<Uuid>,

    #[serde(default)]
    pub promptId: Option<String>,
    #[serde(default)]
    pub promptName: Option<String>,
    #[serde(default)]
    pub promptVersion: Option<String>,

    #[serde(default)]
    pub modelId: Option<String>,

    #[serde(default)]
    pub inputPrice: Option<f64>,
    #[serde(default)]
    pub outputPrice: Option<f64>,
    #[serde(default)]
    pub totalPrice: Option<f64>,

    #[serde(default)]
    pub calculatedInputCost: Option<f64>,
    #[serde(default)]
    pub calculatedOutputCost: Option<f64>,
    #[serde(default)]
    pub calculatedTotalCost: Option<f64>,

    #[serde(default)]
    pub latency: Option<f64>,
    #[serde(default)]
    pub timeToFirstToken: Option<f64>,

    #[serde(default)]
    pub completionTokens: Option<i64>,
    #[serde(default)]
    pub promptTokens: Option<i64>,
    #[serde(default)]
    pub totalTokens: Option<i64>,
    #[serde(default)]
    pub unit: Option<String>,

    #[serde(default)]
    pub metadata: Option<JsonValue>,

    #[serde(default)]
    pub environment: Option<String>,

    #[serde(default)]
    pub projectId: Option<String>,
}

pub(crate) async fn post_batch(
    State(state): State<AppState>,
    Json(payload): Json<BatchIngestRequest>,
) -> Result<impl IntoResponse, ApiError> {
    match state.ingest_tx.try_send(payload) {
        Ok(()) => Ok((
            StatusCode::OK,
            Json(ApiResponse::<serde_json::Value> {
                message: "Request Successful.".to_string(),
                code: None,
                data: None,
            }),
        )),
        Err(mpsc::error::TrySendError::Full(_)) => Err(ApiError::TooManyRequests),
        Err(mpsc::error::TrySendError::Closed(_)) => Err(ApiError::ServiceUnavailable),
    }
}

pub(crate) async fn ingest_worker(
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
