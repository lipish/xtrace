# xtrace

xtrace is a lightweight, self-hosted observability backend for AI/LLM applications. It collects traces, observations, and time-series metrics to help you diagnose latency, cost, quality, and failure patterns in production.

## Running

Dependencies: PostgreSQL.

Environment variables:

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | ✓ | — | PostgreSQL connection string |
| `API_BEARER_TOKEN` | ✓ | — | Protects all API endpoints |
| `BIND_ADDR` | | `127.0.0.1:8742` | Listen address |
| `DEFAULT_PROJECT_ID` | | `default` | Project id for ingested data |
| `XTRACE_PUBLIC_KEY` | | — | Langfuse BasicAuth compatibility |
| `XTRACE_SECRET_KEY` | | — | Langfuse BasicAuth compatibility |
| `RATE_LIMIT_QPS` | | `20` | Per-token query rate limit |
| `RATE_LIMIT_BURST` | | `40` | Per-token burst cap |

Also accepts legacy names `LANGFUSE_PUBLIC_KEY` / `LANGFUSE_SECRET_KEY`.

```bash
DATABASE_URL=postgresql://user@localhost:5432/xtrace \
API_BEARER_TOKEN=secret \
cargo run --release
```

Health check:

```bash
curl http://127.0.0.1:8742/healthz
```

## HTTP API

All endpoints except `/healthz` require:
`Authorization: Bearer $API_BEARER_TOKEN`

### Traces

`POST /v1/l/batch` — Batch ingest traces and observations.

```json
{
  "trace": {
    "id": "00000000-0000-0000-0000-000000000000",
    "timestamp": "2026-01-01T00:00:00Z",
    "name": "chat",
    "userId": "alice",
    "tags": ["prod"]
  },
  "observations": [
    {
      "id": "00000000-0000-0000-0000-000000000001",
      "traceId": "00000000-0000-0000-0000-000000000000",
      "type": "GENERATION",
      "name": "llm",
      "startTime": "2026-01-01T00:00:00Z",
      "endTime": "2026-01-01T00:00:01Z",
      "model": "gpt-4o-mini",
      "input": {"role": "user", "content": "hi"},
      "output": {"role": "assistant", "content": "hello"}
    }
  ]
}
```

`GET /api/public/traces` — Paginated trace list.
`GET /api/public/traces/:traceId` — Single trace detail.
`GET /api/public/metrics/daily` — Daily aggregated metrics.

### Metrics (Time-Series)

`POST /v1/metrics/batch` — Write time-series metrics.

```bash
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"metrics":[{"name":"gpu_utilization","labels":{"node_id":"node-1","gpu_index":"0"},"value":85.0,"timestamp":"2026-02-14T12:00:00Z"}]}' \
  http://127.0.0.1:8742/v1/metrics/batch
```

`GET /api/public/metrics/names` — List all metric names.

`GET /api/public/metrics/query` — Query time-series with downsampling.

| Parameter | Values | Default |
|-----------|--------|---------|
| `name` | metric name (required) | — |
| `from` / `to` | ISO8601 timestamps | last 1 hour |
| `labels` | JSON label filter | — |
| `step` | `1m` `5m` `1h` `1d` | `1m` |
| `agg` | `avg` `max` `min` `sum` `last` **`p50` `p90` `p99`** | `avg` |
| `group_by` | label key to split series by | — |

Example — p99 latency grouped by model:

```bash
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=span_duration&step=5m&agg=p99&group_by=model"
```

## Rust SDK (xtrace-client)

```toml
[dependencies]
xtrace-client = "0.0.12"
```

```rust
use xtrace_client::{Client, MetricPoint, MetricsQueryParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("http://127.0.0.1:8742/", "YOUR_TOKEN")?;
    client.healthz().await?;

    // Push metrics
    client.push_metrics(&[MetricPoint {
        name: "gpu_utilization".to_string(),
        labels: std::collections::HashMap::from([
            ("node_id".to_string(), "node-1".to_string()),
        ]),
        value: 85.0,
        timestamp: chrono::Utc::now(),
    }]).await?;

    // Query with percentile aggregation
    let result = client.query_metrics(&MetricsQueryParams {
        name: "gpu_utilization".to_string(),
        step: Some("5m".to_string()),
        agg: Some("p99".to_string()),
        group_by: Some("node_id".to_string()),
        ..Default::default()
    }).await?;

    Ok(())
}
```

### tracing Integration

Enable the `tracing` feature to automatically push metrics from `tracing` events and span durations — no manual `push_metrics` calls needed:

```toml
xtrace-client = { version = "0.0.12", features = ["tracing"] }
```

```rust
use xtrace_client::{Client, XtraceLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

let client = Client::new("http://127.0.0.1:8742/", "YOUR_TOKEN")?;

tracing_subscriber::registry()
    .with(XtraceLayer::new(client))
    .with(tracing_subscriber::fmt::layer())
    .init();

// Any event with metric= and value= is auto-pushed:
tracing::info!(metric = "zene_tokens", value = 512, model = "gpt-4o");

// Span durations are auto-reported as span_duration with a span_name label:
let _span = tracing::info_span!("execute_tool").entered();
```

## Frontend Dashboard

A React dashboard (Vite + shadcn/ui) is included in the `frontend/` directory.

```bash
cd frontend
VITE_XTRACE_BASE_URL=http://127.0.0.1:8742 \
VITE_XTRACE_API_TOKEN=your_token \
npm install && npm run dev
```

Features: trace list, trace detail viewer with observation tree, and a metrics dashboard.
