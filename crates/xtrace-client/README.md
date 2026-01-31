# xtrace-client

这是 xtrace 服务端（HTTP API）的 Rust SDK。

## 安装

已发布到 crates.io：

```toml
[dependencies]
xtrace-client = "0.0.1"
```

## 使用

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

目前封装的接口：
- healthz
- ingest_batch（POST /v1/l/batch）
- list_traces（GET /api/public/traces）
- get_trace（GET /api/public/traces/:traceId）
- metrics_daily（GET /api/public/metrics/daily）
