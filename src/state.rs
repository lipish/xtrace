use governor::{clock::DefaultClock, Quota};
use sqlx::PgPool;
use std::num::NonZeroU32;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::{
    http::metrics::MetricsBatchRequest,
    ingest::batch::BatchIngestRequest,
};

pub type KeyedRateLimiter = governor::RateLimiter<
    String,
    governor::state::keyed::DashMapStateStore<String>,
    DefaultClock,
>;

pub struct RateLimitStats {
    pub total_allowed: AtomicU64,
    pub total_rejected: AtomicU64,
    pub per_token_rejected: dashmap::DashMap<String, u64>,
}

impl RateLimitStats {
    pub fn new() -> Self {
        Self {
            total_allowed: AtomicU64::new(0),
            total_rejected: AtomicU64::new(0),
            per_token_rejected: dashmap::DashMap::new(),
        }
    }

    pub fn record_allowed(&self) {
        self.total_allowed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_rejected(&self, masked_key: &str) {
        self.total_rejected.fetch_add(1, Ordering::Relaxed);
        self.per_token_rejected
            .entry(masked_key.to_string())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
}

pub fn mask_client_key(key: &str) -> String {
    if let Some(rest) = key.strip_prefix("bearer:") {
        if rest.len() > 8 {
            format!("bearer:{}***", &rest[..8])
        } else {
            format!("bearer:{rest}***")
        }
    } else {
        key.to_string()
    }
}

pub struct ServerConfig {
    pub database_url: String,
    pub api_bearer_token: String,
    pub bind_addr: String,
    pub default_project_id: String,
    pub langfuse_public_key: Option<String>,
    pub langfuse_secret_key: Option<String>,
    pub rate_limit_qps: u32,
    pub rate_limit_burst: u32,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub api_bearer_token: Arc<str>,
    pub langfuse_public_key: Option<Arc<str>>,
    pub langfuse_secret_key: Option<Arc<str>>,
    pub default_project_id: Arc<str>,
    pub ingest_tx: mpsc::Sender<BatchIngestRequest>,
    pub metrics_tx: mpsc::Sender<MetricsBatchRequest>,
    pub query_limiter: Arc<KeyedRateLimiter>,
    pub rate_limit_stats: Arc<RateLimitStats>,
    pub rate_limit_qps: u32,
    pub rate_limit_burst: u32,
}

impl AppState {
    pub fn build_limiter(qps: u32, burst: u32) -> Arc<KeyedRateLimiter> {
        let quota = Quota::per_second(NonZeroU32::new(qps).expect("rate_limit_qps must be > 0"))
            .allow_burst(NonZeroU32::new(burst).expect("rate_limit_burst must be > 0"));
        Arc::new(KeyedRateLimiter::keyed(quota))
    }
}
