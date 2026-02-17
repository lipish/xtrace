# xtrace Signal Contract Template (for Nebula Router/Scheduler)

This template aligns the signal contract between xtrace and Nebula so that Router and Scheduler behave predictably under normal, jitter, and degradation scenarios.

## 1) Owner and Scope

- xtrace owner: (TBD)
- Nebula owner: (TBD)
- Effective environments:
  - dev: ✅
  - staging: ✅
  - prod: ✅
- Effective date: (TBD after mutual confirmation)
- Compatibility window (legacy contract retention): 30 days recommended

## 2) Standard Connection Conventions

- Standard URL (single source of truth): `http://<BIND_ADDR>`, default `http://127.0.0.1:8742`, controlled by env var `BIND_ADDR`
- Auth mode (service/internal): service (all endpoints require auth except `/healthz`)
- Token source and rotation: Static config via `API_BEARER_TOKEN`; rotation requires service restart. **⚠️ Auto-rotation not supported; consider multi-token or JWT later**
- Port and path baseline:
  - metrics query path: `GET /api/public/metrics/query`
  - metrics names path: `GET /api/public/metrics/names`
  - metrics ingest path: `POST /v1/metrics/batch`
  - traces path: `GET /api/public/traces`
- Connection timeout: 5s recommended
- Query timeout: 10s recommended (metrics/query may be slow with large data; normal case < 1s)

## 3) Metric Contract (P0 Required)

### 3.1 pending_requests

- Metric name: `pending_requests`
- Type: number (f64)
- Unit: count
- Valid range: >= 0
- labels (required):
  - `model_uid`: Model unique identifier
  - `replica_id`: Replica ID
- labels (optional):
  - `node_id`: Node ID (included when Nebula Node reports)
- Missing labels behavior: Writes succeed; query uses `labels @>` JSONB containment; missing labels do not cause errors, only exclude series from matches
- Empty result semantics: Returns `{"data": []}`; **does not return 0 or null**

### 3.2 kv_cache_usage

- Metric name: `kv_cache_usage`
- Type: number (f64)
- Unit: ratio
- Valid range: [0, 1] (xtrace does not validate; writer must ensure)
- labels (required):
  - `model_uid`: Model unique identifier
  - `replica_id`: Replica ID
- labels (optional):
  - `node_id`: Node ID
- Calculation (used/total): `EndpointStats.kv_cache_used_bytes / kv_cache_total_bytes`, computed by Nebula Node before reporting
- Missing labels behavior: Same as 3.1
- Empty result semantics: Same as 3.1, returns `{"data": []}`

### 3.3 prefix_cache_hit_rate

- Metric name: `prefix_cache_hit_rate`
- Type: number (f64)
- Unit: ratio
- Valid range: [0, 1] (xtrace does not validate; writer must ensure)
- labels (required):
  - `model_uid`: Model unique identifier
  - `replica_id`: Replica ID
- labels (optional):
  - `node_id`: Node ID
- Stats window: Determined by Nebula Node heartbeat (~5s); xtrace does not re-aggregate
- Missing labels behavior: Same as 3.1
- Empty result semantics: Same as 3.1, returns `{"data": []}`

## 4) Query Semantics and Return Stability (P0/P1)

- Supported aggregations:
  - `last`: ✅
  - `avg`: ✅ (default)
  - `max`: ✅
  - `min`: ✅
  - `sum`: ✅
- `last` definition: Within each time bucket, take the value of the row with the largest timestamp. SQL: `(ARRAY_AGG(value ORDER BY timestamp DESC))[1]`
- `step` semantics: Bucket width; uses `to_timestamp(floor(extract(epoch from timestamp) / step) * step)` to align to bucket start. Supports `1m` (60s), `5m` (300s), `1h` (3600s), `1d` (86400s); default `1m`
- `from`/`to` boundary semantics:
  - Type: ISO8601 timestamp
  - Default: `to` = now, `from` = `to - 1h`
  - Boundary: Closed interval `[from, to]` (SQL: `timestamp >= from AND timestamp <= to`)
  - Constraint: `from > to` returns 400
- When no data:
  - Empty series: ✅ returns `{"data": []}`
  - Zero fill: ❌ not applied
  - null: ❌ not returned
- Partial success (some series have data): Only series with data are returned; missing series are omitted
- Timestamp and timezone: UTC, RFC3339 format (e.g. `2026-02-12T10:00:00+00:00`)

**Query limits:**
- Max 50 series per query (`MAX_SERIES = 50`)
- Max 1000 points per series (`MAX_POINTS_PER_SERIES = 1000`)
- Silent truncation when exceeded; no error

## 5) Freshness and Availability SLO (P1)

- Recommended freshness threshold (Router): 15s. Compare `meta.latest_ts` with now; treat as stale if diff > 15s
- Recommended freshness threshold (Scheduler): 30s. Same as above; diff > 30s = stale
- Target query success rate (5-min window): 99.9% (xtrace target)
- Target p95 latency: < 100ms (single metric, single series)
- Target p99 latency: < 500ms
- Jitter tolerance: Router/Scheduler should cache the last successful query; use cache on failure/timeout; cache TTL 30s recommended

metrics/query response includes `meta.latest_ts` (UTC timestamp of the latest point across all series). Nebula can use it for freshness. Field is omitted when there is no data.

## 6) Error Model (P0 Required)

HTTP status mapping:

| Scenario | HTTP | code | Retryable | Description |
| --- | --- | --- | --- | --- |
| Auth failure | 401 | `UNAUTHORIZED` | ❌ | Missing `Authorization` header or token mismatch |
| Bad params | 400 | `BAD_REQUEST` | ❌ | Missing `name`, `from > to`, invalid `step`, `labels` JSON parse error, etc. |
| Rate limit | 429 | `TOO_MANY_REQUESTS` | ✅ | Write channel full (ingest capacity=1000, metrics capacity=5000) |
| Upstream timeout | 500 | `INTERNAL_ERROR` | ✅ | DB query timeout or pool exhausted; sqlx::Error → 500 |
| Internal error | 500 | `INTERNAL_ERROR` | ✅ | Other DB errors |
| Service unavailable | 503 | `SERVICE_UNAVAILABLE` | ✅ | Write channel closed (service shutting down) |
| Not found | 404 | `NOT_FOUND` | ❌ | Trace detail only; metrics/query returns empty array, not 404 |

Additional:
- Error body schema:
```json
{
  "message": "<human-readable error message>",
  "code": "<machine-readable error code enum>",
  "data": null
}
```
- `code` enum: `UNAUTHORIZED` | `BAD_REQUEST` | `TOO_MANY_REQUESTS` | `INTERNAL_ERROR` | `SERVICE_UNAVAILABLE` | `NOT_FOUND`
- Success (200) responses do not include `code`
- Request id: **⚠️ Not supported.** Add `X-Request-Id` header later (G2)

## 7) Rate Limit and Capacity (P1)

- Server-side query rate limit: ✅ Per-token token bucket (governor crate)
- Default limits: 20 QPS sustained, burst 40 (configurable via `RATE_LIMIT_QPS` / `RATE_LIMIT_BURST`)
- Scope: Query routes only (`/api/public/metrics/*`, `/api/public/traces*`); write routes use channel backpressure
- Rate limit key: Per auth token (Bearer or Basic auth username)
- Recommended QPS per client (Router): ≤ 10 QPS
- Recommended QPS per client (Scheduler): ≤ 5 QPS
- Over-limit response:
  - HTTP 429
  - `Retry-After` response header (seconds, integer)
  - Body: `{"message": "Too Many Requests", "code": "TOO_MANY_REQUESTS", "data": null, "meta": {"rate_limit": {"remaining": 0, "reset_at": "<RFC3339>"}}}`
- Backoff: Use `Retry-After` as minimum wait; add ±20% jitter; exponential backoff cap 5s

## 8) Backward Compatibility and Versioning (P2)

- Contract version: v1.0.0 (initial)
- Breaking change process: Proposal → mutual review → staging validation → advance notice → production rollout
- Advance notice: At least 2 weeks
- Dual-stack period: Legacy endpoints retained for at least 30 days

## 9) Integration Test Samples (P0 Required)

Reproducible samples (request + expected response):

### 1. Normal with data

**Request:**
```bash
curl -H "Authorization: Bearer <TOKEN>" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=pending_requests&step=1m&agg=last&labels=%7B%22model_uid%22%3A%22qwen-72b%22%7D"
```

**Expected response:** `200 OK`
```json
{
  "data": [
    {
      "labels": {"model_uid": "qwen-72b", "replica_id": "0", "node_id": "node-1"},
      "values": [
        {"timestamp": "2026-02-14T10:00:00+00:00", "value": 5.0},
        {"timestamp": "2026-02-14T10:01:00+00:00", "value": 3.0}
      ]
    }
  ],
  "meta": {
    "latest_ts": "2026-02-14T10:01:00+00:00",
    "series_count": 1,
    "truncated": false
  }
}
```

### 2. Normal empty data

**Request:**
```bash
curl -H "Authorization: Bearer <TOKEN>" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=pending_requests&step=1m&agg=last&labels=%7B%22model_uid%22%3A%22nonexistent%22%7D"
```

**Expected response:** `200 OK`
```json
{
  "data": [],
  "meta": {
    "series_count": 0,
    "truncated": false
  }
}
```
Note: `latest_ts` is omitted when data is empty (skip_serializing_if).

### 3. Partial data missing

Querying multiple replicas; only some have data.

**Request:**
```bash
curl -H "Authorization: Bearer <TOKEN>" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=kv_cache_usage&step=1m&agg=last&labels=%7B%22model_uid%22%3A%22qwen-72b%22%7D"
```

**Expected response:** `200 OK` (only series with data returned)
```json
{
  "data": [
    {
      "labels": {"model_uid": "qwen-72b", "replica_id": "0", "node_id": "node-1"},
      "values": [
        {"timestamp": "2026-02-14T10:00:00+00:00", "value": 0.45}
      ]
    }
  ],
  "meta": {
    "latest_ts": "2026-02-14T10:00:00+00:00",
    "series_count": 1,
    "truncated": false
  }
}
```
Note: `replica_id=1` does not appear if it has no data.

### 4. Auth failure

**Request:**
```bash
curl -H "Authorization: Bearer wrong_token" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=pending_requests"
```

**Expected response:** `401 Unauthorized`
```json
{
  "message": "Unauthorized",
  "code": "UNAUTHORIZED",
  "data": null
}
```

### 5. Rate limit (query side — per-token token bucket)

**Trigger:** Same token exceeds query QPS limit (default 20 qps, burst 40)

**Request:**
```bash
# Rapidly send more requests than burst limit
for i in $(seq 1 50); do
  curl -s -o /dev/null -w "%{http_code}\n" \
    -H "Authorization: Bearer <TOKEN>" \
    "http://127.0.0.1:8742/api/public/metrics/query?name=pending_requests" &
done
wait
```

**Expected response:** Over-limit requests return `429 Too Many Requests`

Response headers:
```
Retry-After: 1
```

Response body:
```json
{
  "message": "Too Many Requests",
  "code": "TOO_MANY_REQUESTS",
  "data": null,
  "meta": {
    "rate_limit": {
      "remaining": 0,
      "reset_at": "2026-02-14T10:00:01+00:00"
    }
  }
}
```

Note: Write side (`POST /v1/metrics/batch`) uses channel backpressure, not token bucket; over-limit also returns 429 but without `Retry-After` header.

### 6. Internal server error

**Trigger:** Database unreachable

**Request:**
```bash
curl -H "Authorization: Bearer <TOKEN>" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=pending_requests"
```

**Expected response:** `500 Internal Server Error`
```json
{
  "message": "Internal Error",
  "code": "INTERNAL_ERROR",
  "data": null
}
```

## 10) Acceptance Criteria (Mutual Sign-off)

### 10.1 Router acceptance

- No parse failures in 24h (labels/type/range)
  - labels are JSONB, type is f64
  - pending_requests >= 0, kv_cache_usage ∈ [0,1], prefix_cache_hit_rate ∈ [0,1]
- Stale freshness data correctly degraded
  - **⚠️ Router must implement:** Compare latest timestamp in returned series with current time
- Fallback when no signal is observable
  - Empty data array → Router falls back to default policy

### 10.2 Scheduler acceptance

- Autoscaling signal availability met
  - Based on metrics/query 200 success rate
- No false scale-up/down when xtrace jitters
  - Scheduler should cache last valid result; use cache on timeout/failure

### 10.3 Joint acceptance

- prefix_cache_hit_rate visible in policy
  - Use `agg=last&step=1m` for latest value
- Error-code-driven degradation path reproducible
  - 401 → Fix token
  - 429 → Backoff retry (write side only)
  - 500 → Backoff retry
  - 200 + empty data → Fallback

## 11) Changelog

| Date | Author | Change |
| --- | --- | --- |
| 2026-02-14 | xtrace | Initial version, based on xtrace v0.0.7 actual behavior |

---

## Appendix: Current Gaps and Suggested Improvements

| # | Gap | Priority | Status | Notes |
|---|------|--------|------|------|
| G1 | Error responses lacked machine-readable `code` | P0 | ✅ Done | All error responses now include `"code"` enum: `UNAUTHORIZED`, `BAD_REQUEST`, `TOO_MANY_REQUESTS`, `INTERNAL_ERROR`, `SERVICE_UNAVAILABLE`, `NOT_FOUND`. Success responses omit code (skip_serializing_if) |
| G2 | No `X-Request-Id` response header | P1 | Pending | Middleware generates UUID, writes to response header and logs |
| G3 | metrics/query lacked freshness metadata | P1 | ✅ Done | Response adds `meta` with `latest_ts`, `series_count`, `truncated` |
| G4 | Query side had no rate limiter | P1 | ✅ Done | Per-token token bucket (governor crate), query routes 429 + Retry-After |
| G5 | No value-range validation (kv_cache_usage allows > 1) | P2 | Pending | Add optional validation on write side |
| G6 | Token auto-rotation not supported | P2 | Pending | Support multi-token or JWT |
| G7 | Data retention policy not implemented | P2 | Pending | Implement `METRICS_RETENTION_DAYS` background cleanup |
| G8 | No truncation hint on query | P2 | ✅ Done | `meta.truncated = true` when exceeding MAX_SERIES(50) or MAX_POINTS_PER_SERIES(1000) |
