# xtrace

xtrace is a server-side component for AI/LLM observability that collects, stores, and queries traces/observations/metrics to help you diagnose latency, cost, quality, and failure patterns in production.

This crate is currently published as a **binary (executable service)** (only `src/main.rs`), so it **does not provide a Rust SDK API that can be imported as a dependency**. You can deploy and call it as an HTTP service.

## Running

Dependencies: PostgreSQL.

Environment variables:
`DATABASE_URL` (required)
`API_BEARER_TOKEN` (required, protects the API)
`BIND_ADDR` (optional, default `127.0.0.1:8742`)
`DEFAULT_PROJECT_ID` (optional, default `default`)
`XTRACE_PUBLIC_KEY` (optional, for Langfuse public API BasicAuth compatibility)
`XTRACE_SECRET_KEY` (optional, for Langfuse public API BasicAuth compatibility)
`RATE_LIMIT_QPS` (optional, default `20`, per-token query rate limit QPS)
`RATE_LIMIT_BURST` (optional, default `40`, per-token query rate limit burst cap)

Compatibility:
Also supports legacy names `LANGFUSE_PUBLIC_KEY` / `LANGFUSE_SECRET_KEY`.

Start:
```bash
DATABASE_URL=postgresql://xinference@localhost:5432/xtrace \
API_BEARER_TOKEN=... \
cargo run --release
```

Health check:
```bash
curl http://127.0.0.1:8742/healthz
```

## HTTP API (Core Routes)

Except `/healthz`, all other endpoints require a Bearer token:
`Authorization: Bearer $API_BEARER_TOKEN`

`POST /v1/l/batch`
Batch ingest events.

Request body structure (simplified):
`trace` (optional) + `observations` (array)

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

`GET /api/public/traces`
Paginated trace query.

`GET /api/public/traces/:traceId`
Fetch a single trace's details.

`GET /api/public/metrics/daily`
Query daily aggregated metrics.

## Nebula Integration (Metrics)

To support Nebula reporting GPU/node metrics (time-series metrics), xtrace provides additional metrics write and query endpoints.

Note: The current implementation works in single-tenant/single-project mode; all written metrics go to `DEFAULT_PROJECT_ID` and `environment` is fixed to `default`.

### Write

`POST /v1/metrics/batch`

Request body:

```json
{
  "metrics": [
    {
      "name": "gpu_utilization",
      "labels": {"node_id": "node-1", "gpu_index": "0"},
      "value": 85.0,
      "timestamp": "2026-02-12T12:18:00Z"
    }
  ]
}
```

Example:

```bash
NOW=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"metrics\":[{\"name\":\"gpu_utilization\",\"labels\":{\"node_id\":\"node-1\",\"gpu_index\":\"0\"},\"value\":85.0,\"timestamp\":\"$NOW\"}]}" \
  "http://127.0.0.1:8742/v1/metrics/batch"
```

### Query

`GET /api/public/metrics/names`

Returns all metric names under the current project:

```bash
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/names"
```

`GET /api/public/metrics/query`

Parameters:

- name (required)
- from/to (optional, ISO8601; default last 1 hour)
- labels (optional, JSON string; backend filters with `labels @> ...`)
- step (optional: 1m/5m/1h/1d; default 1m)
- agg (optional: avg/max/min/sum/last; default avg)

Example:

```bash
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=gpu_utilization&step=1m&agg=last&labels=%7B%22node_id%22%3A%22node-1%22%2C%22gpu_index%22%3A%220%22%7D"
```

Example:
```bash
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "http://127.0.0.1:8742/api/public/traces?page=1&limit=50"
```

## Rust SDK (xtrace-client)

The repository includes an `xtrace-client` crate (HTTP SDK, based on `reqwest`).

```toml
[dependencies]
xtrace-client = "0.0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Example:
```rust
use xtrace_client::{Client, TraceListQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("http://127.0.0.1:8742/", "YOUR_TOKEN")?;
    client.healthz().await?;

    let traces = client.list_traces(&TraceListQuery::default()).await?;
    println!("{}", traces.data.len());

    let now = chrono::Utc::now();
    client
        .push_metrics(&[xtrace_client::MetricPoint {
            name: "gpu_utilization".to_string(),
            labels: std::collections::HashMap::from([
                ("node_id".to_string(), "node-1".to_string()),
                ("gpu_index".to_string(), "0".to_string()),
            ]),
            value: 85.0,
            timestamp: now,
        }])
        .await?;
    Ok(())
}
```

The `xtrace` crate is positioned as a server-side component; splitting the SDK separately avoids pulling server-side dependencies (axum/sqlx) into client applications.
