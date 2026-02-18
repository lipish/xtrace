# Nebula Integration

[Nebula](https://github.com/lipish/nebula) is a distributed GPU inference cluster management system. xtrace serves as its lightweight observability backend for historical metrics and traces.

## Architecture

| Nebula Component | Reports To xtrace |
|-----------------|-------------------|
| **Node** | GPU metrics (utilization, temperature, memory, KV cache) via `POST /v1/metrics/batch` |
| **Gateway** | Request traces via `POST /v1/l/batch` (trace + observation per request) |
| **Router** | Reads real-time metrics via `GET /api/public/metrics/query` for routing decisions |

## Metrics Reported by Node

Each Node reports metrics on every heartbeat cycle (~5 seconds):

| Metric | Labels | Value | Description |
|--------|--------|-------|-------------|
| `gpu_utilization` | `node_id`, `gpu_index` | 0–100 | GPU utilization percentage |
| `gpu_temperature` | `node_id`, `gpu_index` | Celsius | GPU temperature |
| `gpu_memory_used_mb` | `node_id`, `gpu_index` | MB | GPU memory used |
| `gpu_memory_total_mb` | `node_id`, `gpu_index` | MB | GPU memory total |
| `kv_cache_usage` | `model_uid`, `replica_id`, `node_id` | 0.0–1.0 | KV cache utilization ratio |
| `pending_requests` | `model_uid`, `replica_id`, `node_id` | count | Pending request count |
| `prefix_cache_hit_rate` | `model_uid`, `replica_id`, `node_id` | 0.0–1.0 | Prefix cache hit ratio |

### Example: Node Reporting

```bash
NOW=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
curl -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"metrics\":[
    {\"name\":\"gpu_utilization\",\"labels\":{\"node_id\":\"node-1\",\"gpu_index\":\"0\"},\"value\":85.0,\"timestamp\":\"$NOW\"},
    {\"name\":\"kv_cache_usage\",\"labels\":{\"model_uid\":\"qwen-72b\",\"replica_id\":\"0\",\"node_id\":\"node-1\"},\"value\":0.45,\"timestamp\":\"$NOW\"}
  ]}" \
  "http://127.0.0.1:8742/v1/metrics/batch"
```

## Gateway Traces

Each Gateway request produces a trace with an observation:

- **Trace**: `name` = request path, `userId` = requester, `tags` = `["gateway"]`
- **Observation** (type=GENERATION): `model`, `input/output`, `promptTokens/completionTokens`, `latency`

## Router Queries

The Router queries xtrace for real-time routing signals:

```bash
# Latest pending requests for a model
curl -H "Authorization: Bearer $TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/query?\
name=pending_requests&\
step=1m&agg=last&\
labels=%7B%22model_uid%22%3A%22qwen-72b%22%7D"
```

### Freshness Check

Use `meta.latest_ts` to detect stale data:

```
if (now - meta.latest_ts) > 15s → treat as stale, use fallback
```

| Consumer | Freshness Threshold | Cache TTL |
|----------|-------------------|-----------|
| Router | 15 seconds | 30 seconds |
| Scheduler | 30 seconds | 30 seconds |

## Signal Contract

Key specifications include:

- Metric value ranges and semantics
- Query limits (50 series, 1000 points)
- Error model and retry behavior
- Acceptance criteria

## Recommended Client Settings

| Setting | Router | Scheduler |
|---------|--------|-----------|
| Max QPS | ≤ 10 | ≤ 5 |
| Connection timeout | 5s | 5s |
| Query timeout | 10s | 10s |
| Backoff on 429 | Use `Retry-After` + jitter | Use `Retry-After` + jitter |
| Fallback on failure | Cache last result (30s TTL) | Cache last result (30s TTL) |
