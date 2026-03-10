use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::state::AppState;

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

pub(crate) async fn get_projects(State(state): State<AppState>) -> impl IntoResponse {
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
