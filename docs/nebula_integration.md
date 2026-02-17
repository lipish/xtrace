# xtrace Requirements: Nebula Observability Integration

## Background

Nebula is a distributed GPU inference cluster management system with the following components:

| Component | Responsibility |
|-----------|----------------|
| **Node** | Manages single-node GPU, container lifecycle, engine health checks, metrics collection |
| **Router** | Makes routing decisions based on endpoint stats (least-pending, KV cache aware) |
| **Gateway** | External HTTP API gateway, proxies requests to Router, authentication, rate limiting |

Current Nebula runtime data (endpoint stats, node status) is stored in etcd for real-time routing decisions, but lacks **historical observability**—unable to retrospect latency trends, GPU utilization changes, request volume statistics, etc.

Prometheus + Grafana is too heavy. xtrace can be directly integrated as a lightweight self-developed observability service.

## Goals

On top of xtrace's existing trace/observation capabilities, add **metrics time-series data** write and query capabilities so it can cover:

1. **Trace data** (existing): Full request chain per Gateway request
2. **Metrics data** (new): Periodic metric snapshots (GPU, KV cache, QPS, etc.)

---

## 1. New Metrics Time-Series Data Model

### 1.1 Database Table

```sql
CREATE TABLE IF NOT EXISTS metrics (
  id BIGSERIAL PRIMARY KEY,
  project_id TEXT NOT NULL,
  environment TEXT NOT NULL DEFAULT 'default',

  -- Metric identifier
  name TEXT NOT NULL,           -- e.g. "gpu_utilization", "kv_cache_usage", "request_count"
  
  -- Labels (for filtering and grouping)
  labels JSONB NOT NULL DEFAULT '{}',  -- e.g. {"node_id":"node-1","gpu_index":"0","model_uid":"qwen-72b"}

  -- Value
  value DOUBLE PRECISION NOT NULL,

  -- Timestamp
  timestamp TIMESTAMPTZ NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_metrics_name_ts ON metrics (project_id, name, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_metrics_labels_gin ON metrics USING GIN (labels);
CREATE INDEX IF NOT EXISTS idx_metrics_ts ON metrics (timestamp DESC);
```

### 1.2 Write API

**`POST /v1/metrics/batch`**

Batch write metric data points.

```json
{
  "metrics": [
    {
      "name": "gpu_utilization",
      "labels": {"node_id": "node-1", "gpu_index": "0"},
      "value": 85.0,
      "timestamp": "2026-02-12T10:00:00Z"
    },
    {
      "name": "gpu_memory_used_mb",
      "labels": {"node_id": "node-1", "gpu_index": "0"},
      "value": 40960.0,
      "timestamp": "2026-02-12T10:00:00Z"
    },
    {
      "name": "kv_cache_usage",
      "labels": {"model_uid": "qwen-72b", "replica_id": "0"},
      "value": 0.45,
      "timestamp": "2026-02-12T10:00:00Z"
    }
  ]
}
```

Response: `200 OK`

Implementation requirements:
- Reuse existing `ingest_worker` mpsc channel + micro-batch write mode
- Use `COPY` or multi-row `INSERT` for batch writes

### 1.3 Query API

**`GET /api/public/metrics/query`**

Query metric time-series data with support for time range, label filtering, and downsampling.

Parameters:

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | string | Required, metric name |
| `from` | ISO8601 | Start time, default 1 hour ago |
| `to` | ISO8601 | End time, default now |
| `labels` | string | Label filter, JSON format, e.g. `{"node_id":"node-1"}` |
| `step` | string | Downsample step: `1m`, `5m`, `1h`, `1d`, default `1m` |
| `agg` | string | Aggregation function: `avg`, `max`, `min`, `sum`, `last`, default `avg` |

Response:

```json
{
  "data": [
    {
      "labels": {"node_id": "node-1", "gpu_index": "0"},
      "values": [
        {"timestamp": "2026-02-12T10:00:00Z", "value": 85.0},
        {"timestamp": "2026-02-12T10:01:00Z", "value": 87.0}
      ]
    }
  ]
}
```

Implementation requirements:
- Use `date_trunc` or `date_bin` for time-bucket downsampling
- Group by unique label combinations and return multiple time series
- Consider adding `LIMIT` for large datasets (e.g. max 1000 data points × 50 time series)

**`GET /api/public/metrics/names`**

List all available metric names.

```json
{
  "data": ["gpu_utilization", "gpu_memory_used_mb", "kv_cache_usage", "pending_requests", "request_count"]
}
```

---

## 2. Metrics Reported by Nebula

### 2.1 Node Reports (per heartbeat cycle, ~5s)

| Metric Name | labels | Value | Source |
|-------------|--------|-------|--------|
| `gpu_utilization` | `node_id`, `gpu_index` | 0-100 | `GpuStatus.utilization_gpu` |
| `gpu_temperature` | `node_id`, `gpu_index` | Celsius | `GpuStatus.temperature_c` |
| `gpu_memory_used_mb` | `node_id`, `gpu_index` | MB | `GpuStatus.memory_used_mb` |
| `gpu_memory_total_mb` | `node_id`, `gpu_index` | MB | `GpuStatus.memory_total_mb` |
| `kv_cache_usage` | `model_uid`, `replica_id`, `node_id` | 0.0-1.0 | `EndpointStats.kv_cache_used_bytes / total` |
| `pending_requests` | `model_uid`, `replica_id`, `node_id` | count | `EndpointStats.pending_requests` |
| `prefix_cache_hit_rate` | `model_uid`, `replica_id`, `node_id` | 0.0-1.0 | `EndpointStats.prefix_cache_hit_rate` |

### 2.2 Gateway Reports (per request completion, as trace)

Uses existing trace/observation model, no changes needed:

- **Trace**: One user request
  - `name`: Request path (e.g. `POST /v1/chat/completions`)
  - `userId`: Requester identifier
  - `tags`: `["gateway"]`
  - `latency`: End-to-end latency
- **Observation** (type=GENERATION):
  - `model`: Actual model used
  - `input/output`: Request/response body
  - `promptTokens/completionTokens`: Token usage

### 2.3 Router Reports (optional, as observation attached to Gateway trace)

- Routing decision latency
- Selected endpoint

---

## 3. xtrace-client SDK Extension

Add metrics write methods in the `xtrace-client` crate:

```rust
impl Client {
    /// Batch write metric data points
    pub async fn push_metrics(&self, metrics: &[MetricPoint]) -> Result<(), Error>;
    
    /// Query metric time-series
    pub async fn query_metrics(&self, q: &MetricsQuery) -> Result<MetricsQueryResult, Error>;
    
    /// List all metric names
    pub async fn list_metric_names(&self) -> Result<Vec<String>, Error>;
}

pub struct MetricPoint {
    pub name: String,
    pub labels: HashMap<String, String>,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
}
```

---

## 4. Data Retention Policy

New environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `METRICS_RETENTION_DAYS` | `30` | Metrics table data retention in days |
| `TRACES_RETENTION_DAYS` | `90` | Traces/observations table data retention in days |

Create a background task at startup that runs hourly cleanup:

```sql
DELETE FROM metrics WHERE timestamp < NOW() - INTERVAL '30 days';
DELETE FROM observations WHERE created_at < NOW() - INTERVAL '90 days';
DELETE FROM traces WHERE created_at < NOW() - INTERVAL '90 days';
```

---

## 5. Implementation Priority

| Priority | Content | Effort |
|----------|---------|--------|
| **P0** | metrics table + `POST /v1/metrics/batch` write | Low |
| **P0** | `xtrace-client` add `push_metrics` | Low |
| **P1** | `GET /api/public/metrics/query` downsampling query | Medium |
| **P1** | `GET /api/public/metrics/names` | Low |
| **P2** | Data retention policy background cleanup | Low |
| **P2** | `xtrace-client` add query methods | Low |

After P0 is complete, Nebula can start integration and reporting; after P1 is complete, the frontend can display the Dashboard.
