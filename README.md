# xtrace

xtrace 是一个用于 AI/LLM 可观测性的服务端组件，用于采集、存储与查询 traces/observations/metrics，帮助你在生产环境定位延迟、成本、质量与失败模式。

这个 crate 当前以 **binary（可执行服务）** 的形式发布（只有 `src/main.rs`），因此 **不提供可作为依赖导入的 Rust SDK API**。你可以把它当作一个 HTTP 服务来部署与调用。

## 运行

依赖：PostgreSQL。

环境变量：
`DATABASE_URL`（必填）
`API_BEARER_TOKEN`（必填，用于保护接口）
`BIND_ADDR`（可选，默认 `127.0.0.1:8080`）
`DEFAULT_PROJECT_ID`（可选，默认 `default`）
`LANGFUSE_PUBLIC_KEY`（可选）
`LANGFUSE_SECRET_KEY`（可选）

启动：
```bash
DATABASE_URL=postgres://... \
API_BEARER_TOKEN=... \
cargo run --release
```

健康检查：
```bash
curl http://127.0.0.1:8080/healthz
```

## HTTP API（核心路由）

除 `/healthz` 外，其它接口需要携带 Bearer token：
`Authorization: Bearer $API_BEARER_TOKEN`

`POST /v1/l/batch`
用于批量写入（ingest）事件。

请求体结构（简化）：
`trace`（可选）+ `observations`（数组）

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
用于分页查询 traces。

`GET /api/public/traces/:traceId`
用于查询某条 trace 的详情。

`GET /api/public/metrics/daily`
用于查询按天聚合的指标。

示例：
```bash
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "http://127.0.0.1:8080/api/public/traces?page=1&limit=50"
```

## Rust SDK（xtrace-client）

仓库内已提供 `xtrace-client` crate（HTTP SDK，基于 `reqwest`）。

```toml
[dependencies]
xtrace-client = { path = "crates/xtrace-client" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

示例：
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

当前 `xtrace` crate 的定位是服务端；SDK 单独拆分可以避免把服务端依赖（axum/sqlx）带进业务侧。
