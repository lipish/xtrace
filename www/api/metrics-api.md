# Metrics API

xtrace provides a time-series metrics subsystem for collecting and querying infrastructure and application metrics.

## Write Metrics

### POST /v1/metrics/batch

Batch write metric data points with arbitrary labels.

**Request Body:**

```json
{
  "metrics": [
    {
      "name": "gpu_utilization",
      "labels": {"node_id": "node-1", "gpu_index": "0"},
      "value": 85.0,
      "timestamp": "2026-02-14T10:00:00Z"
    },
    {
      "name": "token_usage",
      "labels": {
        "session_id": "sess-abc",
        "model_name": "gpt-4",
        "agent_role": "Planner"
      },
      "value": 1523,
      "timestamp": "2026-02-14T10:00:00Z"
    }
  ]
}
```

**Response:** `200 OK`

Labels are stored as JSONB with a GIN index for efficient filtering.

## Query Metrics

### GET /api/public/metrics/query

Query metric time-series with downsampling and aggregation.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Metric name |
| `from` | ISO 8601 | No | Start time (default: 1 hour ago) |
| `to` | ISO 8601 | No | End time (default: now) |
| `labels` | JSON string | No | Label filter via JSONB containment |
| `step` | string | No | Bucket width: `1m`, `5m`, `1h`, `1d` (default `1m`) |
| `agg` | string | No | Aggregation function (default `avg`) |
| `group_by` | string | No | Group by a specific label key |

**Aggregation Functions:**

| Value | Description |
|-------|-------------|
| `avg` | Average (default) |
| `max` | Maximum |
| `min` | Minimum |
| `sum` | Sum |
| `last` | Latest value in each bucket |
| `p50` | 50th percentile (median) |
| `p90` | 90th percentile |
| `p99` | 99th percentile |

**Example — Basic query:**

```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=gpu_utilization&step=5m&agg=avg"
```

**Example — Label filtering:**

```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/query?\
name=token_usage&\
labels=%7B%22agent_role%22%3A%22Planner%22%7D&\
step=1h&agg=sum"
```

**Example — Percentile analysis:**

```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=latency&step=1h&agg=p90"
```

**Example — Group by label key:**

```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/query?\
name=token_usage&\
group_by=agent_role&\
step=1h&agg=sum"
```

Returns separate series for each unique value of `agent_role`:

```json
{
  "data": [
    {
      "labels": {"agent_role": "Planner"},
      "values": [
        {"timestamp": "2026-02-14T10:00:00+00:00", "value": 4500}
      ]
    },
    {
      "labels": {"agent_role": "Executor"},
      "values": [
        {"timestamp": "2026-02-14T10:00:00+00:00", "value": 2100}
      ]
    }
  ],
  "meta": {
    "latest_ts": "2026-02-14T10:00:00+00:00",
    "series_count": 2,
    "truncated": false
  }
}
```

### GET /api/public/metrics/names

List all available metric names.

```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://127.0.0.1:8742/api/public/metrics/names"
```

```json
{
  "data": ["gpu_utilization", "kv_cache_usage", "pending_requests", "token_usage"]
}
```

## Query Limits

| Limit | Value |
|-------|-------|
| Max series per query | 50 |
| Max points per series | 1000 |

Results are silently truncated when limits are exceeded. Check `meta.truncated` in the response.

## Response Metadata

Every metrics query response includes a `meta` object:

```json
{
  "meta": {
    "latest_ts": "2026-02-14T10:05:00+00:00",
    "series_count": 3,
    "truncated": false
  }
}
```

- **`latest_ts`**: Timestamp of the most recent data point (omitted when no data)
- **`series_count`**: Number of distinct series returned
- **`truncated`**: `true` when results hit the max series or points limit
