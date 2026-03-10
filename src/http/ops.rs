use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::atomic::Ordering;

use crate::state::AppState;

pub(crate) async fn get_rate_limit_stats(State(state): State<AppState>) -> impl IntoResponse {
    let total_allowed = state.rate_limit_stats.total_allowed.load(Ordering::Relaxed);
    let total_rejected = state
        .rate_limit_stats
        .total_rejected
        .load(Ordering::Relaxed);

    let mut per_token: Vec<(String, u64)> = state
        .rate_limit_stats
        .per_token_rejected
        .iter()
        .map(|entry| (entry.key().clone(), *entry.value()))
        .collect();
    per_token.sort_by(|a, b| b.1.cmp(&a.1));

    let top = per_token
        .into_iter()
        .take(20)
        .map(|(token, count)| serde_json::json!({ "token": token, "count": count }))
        .collect::<Vec<_>>();

    let body = serde_json::json!({
        "rate_limit_qps": state.rate_limit_qps,
        "rate_limit_burst": state.rate_limit_burst,
        "total_allowed": total_allowed,
        "total_rejected": total_rejected,
        "rejection_rate": if total_allowed + total_rejected > 0 {
            total_rejected as f64 / (total_allowed + total_rejected) as f64
        } else {
            0.0
        },
        "top_rejected_tokens": top,
    });

    (StatusCode::OK, Json(body))
}
