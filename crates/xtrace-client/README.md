# xtrace-client

这是 xtrace 服务端（HTTP API）的 Rust SDK。

## 安装

在仓库内开发时推荐用 path 依赖：

```toml
[dependencies]
xtrace-client = { path = "../crates/xtrace-client" }
```

如果后续发布到 crates.io，可以改为版本依赖。

## 使用

```rust
use xtrace_client::{Client, TraceListQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("http://127.0.0.1:8080/", "YOUR_TOKEN")?;

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
