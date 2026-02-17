# xtrace-client

Rust SDK for the xtrace server (HTTP API).

## Installation

Published on crates.io:

```toml
[dependencies]
xtrace-client = "0.0.1"
```

## Usage

```rust
use xtrace_client::{Client, TraceListQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("http://127.0.0.1:8742/", "YOUR_TOKEN")?;

    client.healthz().await?;

    let traces = client.list_traces(&TraceListQuery::default()).await?;
    println!("{}", traces.data.len());

    Ok(())
}
```

Available methods:
- healthz
- ingest_batch (POST /v1/l/batch)
- list_traces (GET /api/public/traces)
- get_trace (GET /api/public/traces/:traceId)
- metrics_daily (GET /api/public/metrics/daily)
- push_metrics (POST /v1/metrics/batch)
- query_metrics (GET /api/public/metrics/query)
- list_metric_names (GET /api/public/metrics/names)
