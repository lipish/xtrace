# xtrace

xtrace 是一个用于 AI/LLM 可观测性的服务端组件，用于采集、存储与查询 traces/observations/metrics，帮助你在生产环境定位延迟、成本、质量与失败模式。

这个 crate 当前以 **binary（可执行服务）** 的形式发布（只有 `src/main.rs`），因此 **不提供可作为依赖导入的 Rust SDK API**。你可以把它当作一个 HTTP 服务来部署与调用。

## 运行

依赖：PostgreSQL。

环境变量：
`DATABASE_URL`（必填）
`API_BEARER_TOKEN`（必填，用于保护接口）
`BIND_ADDR`（可选，默认 `127.0.0.1:8742`）
`DEFAULT_PROJECT_ID`（可选，默认 `default`）
`XTRACE_PUBLIC_KEY`（可选，用于兼容 Langfuse public API BasicAuth）
`XTRACE_SECRET_KEY`（可选，用于兼容 Langfuse public API BasicAuth）
`RATE_LIMIT_QPS`（可选，默认 `20`，per-token 查询限流 QPS）
`RATE_LIMIT_BURST`（可选，默认 `40`，per-token 查询限流 burst 上限）

兼容：
也支持旧命名 `LANGFUSE_PUBLIC_KEY` / `LANGFUSE_SECRET_KEY`。

启动：
```bash
DATABASE_URL=postgresql://xinference@localhost:5432/xtrace \
API_BEARER_TOKEN=... \
cargo run --release
```

健康检查：
```bash
curl http://127.0.0.1:8742/healthz
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

## Nebula 集成（Metrics）

为支持 Nebula 上报 GPU/节点等指标（time-series metrics），xtrace 额外提供一组 metrics 写入与查询接口。

说明：当前实现按“单租户/单 project”方式工作，所有写入的 metrics 会落到 `DEFAULT_PROJECT_ID`，且 `environment` 固定为 `default`。

### 写入

`POST /v1/metrics/batch`

请求体：

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

示例：

```bash
NOW=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"metrics\":[{\"name\":\"gpu_utilization\",\"labels\":{\"node_id\":\"node-1\",\"gpu_index\":\"0\"},\"value\":85.0,\"timestamp\":\"$NOW\"}]}" \
  "http://127.0.0.1:8742/v1/metrics/batch"
```

### 查询

`GET /api/public/metrics/names`

返回当前 project 下所有指标名：

```bash
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/names"
```

`GET /api/public/metrics/query`

参数：

- name（必填）
- from/to（可选，ISO8601；默认最近 1 小时）
- labels（可选，JSON 字符串；后端用 `labels @> ...` 过滤）
- step（可选：1m/5m/1h/1d；默认 1m）
- agg（可选：avg/max/min/sum/last；默认 avg）

示例：

```bash
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=gpu_utilization&step=1m&agg=last&labels=%7B%22node_id%22%3A%22node-1%22%2C%22gpu_index%22%3A%220%22%7D"
```

示例：
```bash
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "http://127.0.0.1:8742/api/public/traces?page=1&limit=50"
```

## Rust SDK（xtrace-client）

仓库内已提供 `xtrace-client` crate（HTTP SDK，基于 `reqwest`）。

```toml
[dependencies]
xtrace-client = "0.0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

示例：
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

当前 `xtrace` crate 的定位是服务端；SDK 单独拆分可以避免把服务端依赖（axum/sqlx）带进业务侧。
