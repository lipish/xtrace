//! `tracing::Layer` for automatic metric collection.
//!
//! When the `tracing` feature is enabled, add `XtraceLayer` to your subscriber
//! to automatically push metrics from tracing events and span durations to xtrace.
//!
//! # Example
//!
//! ```ignore
//! use tracing_subscriber::layer::SubscriberExt;
//! use tracing_subscriber::util::SubscriberInitExt;
//! use xtrace_client::{Client, XtraceLayer};
//!
//! let client = Client::new("http://127.0.0.1:8742/", "token")?;
//! let layer = XtraceLayer::new(client);
//!
//! tracing_subscriber::registry()
//!     .with(layer)
//!     .with(tracing_subscriber::fmt::layer())
//!     .init();
//!
//! // Events with metric= and value= are auto-pushed:
//! tracing::info!(metric = "zene_tokens", value = 100, model = "gpt-4");
//!
//! // Span durations are auto-reported as span_duration metric:
//! #[tracing::instrument(fields(session_id = %session.id))]
//! async fn execute_tool() {
//!     // ...
//! }
//! ```

use chrono::Utc;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use tracing::field::Field;
use tracing::field::Visit;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

use crate::Client;
use crate::MetricPoint;

/// Default metric name for span duration.
pub const SPAN_DURATION_METRIC: &str = "span_duration";

/// Default batch size before flush.
const BATCH_SIZE: usize = 50;

/// Default flush interval in milliseconds.
const FLUSH_INTERVAL_MS: u64 = 500;

/// Fields to promote as labels when present (session_id, task_id, model, etc.).
const LABEL_FIELDS: &[&str] = &[
    "session_id",
    "task_id",
    "model",
    "model_name",
    "provider",
    "agent_role",
    "tool_name",
    "status",
];

/// A tracing Layer that pushes metrics to xtrace.
///
/// - **Events** with `metric` and `value` fields are pushed as metrics.
///   Other string/numeric fields (session_id, task_id, model, etc.) become labels.
/// - **Span durations** are reported as `span_duration` metric with `span_name` label.
#[derive(Clone)]
pub struct XtraceLayer {
    inner: Arc<XtraceLayerInner>,
}

struct XtraceLayerInner {
    tx: mpsc::SyncSender<MetricPoint>,
    /// Tracks span creation time by span id (u64) for duration measurement on close.
    span_records: Mutex<Vec<SpanRecord>>,
}

struct SpanRecord {
    span_id: u64,
    created_at: Instant,
    name: String,
}

impl XtraceLayer {
    /// Create a new XtraceLayer and spawn the background flusher.
    pub fn new(client: Client) -> Self {
        let (tx, rx) = mpsc::sync_channel(1000);
        let inner = Arc::new(XtraceLayerInner {
            tx,
            span_records: Mutex::new(Vec::new()),
        });

        let client = client.clone();
        std::thread::spawn(move || {
            let rt = match tokio::runtime::Runtime::new() {
                Ok(r) => r,
                Err(_) => return,
            };
            let mut batch = Vec::with_capacity(BATCH_SIZE);
            let mut last_flush = Instant::now();
            let flush_interval = std::time::Duration::from_millis(FLUSH_INTERVAL_MS);

            loop {
                match rx.recv_timeout(flush_interval) {
                    Ok(point) => {
                        batch.push(point);
                        if batch.len() >= BATCH_SIZE {
                            flush_batch(&rt, &client, &mut batch);
                            last_flush = Instant::now();
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        if !batch.is_empty() && last_flush.elapsed() >= flush_interval {
                            flush_batch(&rt, &client, &mut batch);
                            last_flush = Instant::now();
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
            if !batch.is_empty() {
                flush_batch(&rt, &client, &mut batch);
            }
        });

        Self { inner }
    }

    fn try_send(&self, point: MetricPoint) {
        let _ = self.inner.tx.try_send(point);
    }
}

fn flush_batch(
    rt: &tokio::runtime::Runtime,
    client: &Client,
    batch: &mut Vec<MetricPoint>,
) {
    if batch.is_empty() {
        return;
    }
    let points = std::mem::take(batch);
    let client = client.clone();
    rt.block_on(async move {
        let _ = client.push_metrics(&points).await;
    });
}

impl<S> Layer<S> for XtraceLayer
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        let mut visitor = MetricEventVisitor::default();
        event.record(&mut visitor);
        if let Some(point) = visitor.into_metric_point() {
            self.try_send(point);
        }
    }

    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::Id,
        _ctx: Context<'_, S>,
    ) {
        let name = attrs.metadata().name().to_string();
        let key = id.clone().into_u64();
        let mut guard = self.inner.span_records.lock().unwrap();
        guard.push(SpanRecord {
            span_id: key,
            created_at: Instant::now(),
            name,
        });
    }

    fn on_close(&self, id: tracing::Id, _ctx: Context<'_, S>) {
        let key = id.into_u64();
        let (duration_secs, span_name) = {
            let mut guard = self.inner.span_records.lock().unwrap();
            let pos = guard.iter().position(|r| r.span_id == key);
            if let Some(pos) = pos {
                let rec = guard.remove(pos);
                (rec.created_at.elapsed().as_secs_f64(), rec.name)
            } else {
                return;
            }
        };
        let mut labels = HashMap::new();
        labels.insert("span_name".to_string(), span_name);
        let point = MetricPoint {
            name: SPAN_DURATION_METRIC.to_string(),
            labels,
            value: duration_secs,
            timestamp: Utc::now(),
        };
        self.try_send(point);
    }
}

#[derive(Default)]
struct MetricEventVisitor {
    metric: Option<String>,
    value: Option<f64>,
    labels: HashMap<String, String>,
}

impl MetricEventVisitor {
    fn into_metric_point(self) -> Option<MetricPoint> {
        let metric = self.metric?;
        let value = self.value.unwrap_or(0.0);
        Some(MetricPoint {
            name: metric,
            labels: self.labels,
            value,
            timestamp: Utc::now(),
        })
    }
}

impl Visit for MetricEventVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if LABEL_FIELDS.contains(&field.name()) {
            self.labels
                .insert(field.name().to_string(), format!("{:?}", value));
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "metric" {
            self.metric = Some(value.to_string());
        } else if LABEL_FIELDS.contains(&field.name()) {
            self.labels.insert(field.name().to_string(), value.to_string());
        }
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        if field.name() == "value" {
            self.value = Some(value);
        } else if LABEL_FIELDS.contains(&field.name()) {
            self.labels
                .insert(field.name().to_string(), value.to_string());
        }
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        if field.name() == "value" {
            self.value = Some(value as f64);
        } else if LABEL_FIELDS.contains(&field.name()) {
            self.labels
                .insert(field.name().to_string(), value.to_string());
        }
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        if field.name() == "value" {
            self.value = Some(value as f64);
        } else if LABEL_FIELDS.contains(&field.name()) {
            self.labels
                .insert(field.name().to_string(), value.to_string());
        }
    }
}

