use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct PageMeta {
    pub page: i64,
    pub limit: i64,
    pub totalItems: i64,
    pub totalPages: i64,
}

#[derive(Debug, Serialize)]
pub struct PagedData<T> {
    pub data: Vec<T>,
    pub meta: PageMeta,
}

pub async fn healthz() -> impl IntoResponse {
    StatusCode::OK
}
