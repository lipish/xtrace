use axum::{http::StatusCode, response::IntoResponse, Json};
use thiserror::Error;

use crate::http::common::ApiResponse;

#[derive(Debug, Error)]
pub enum ApiError {
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

        let (status, code, msg) = match self {
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", m),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND", "Not Found".to_string()),
            ApiError::TooManyRequests => (
                StatusCode::TOO_MANY_REQUESTS,
                "TOO_MANY_REQUESTS",
                "Too Many Requests".to_string(),
            ),
            ApiError::ServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                "Service Unavailable".to_string(),
            ),
            ApiError::Sqlx(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Internal Error".to_string(),
            ),
        };

        let body = Json(ApiResponse::<serde_json::Value> {
            message: msg,
            code: Some(code),
            data: None,
        });

        (status, body).into_response()
    }
}
