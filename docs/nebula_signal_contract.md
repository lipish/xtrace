# xtrace Signal Contract Template (for Nebula Router/Scheduler)

本模板用于对齐 xtrace 与 Nebula 的信号契约，确保 Router 与 Scheduler 在正常、抖动、降级场景下都可预测。

## 1) Owner 与范围

- xtrace owner: （待填写）
- Nebula owner: （待填写）
- 生效环境:
  - dev: ✅
  - staging: ✅
  - prod: ✅
- 生效日期: （待双方确认后填写）
- 兼容窗口（旧契约保留时长）: 建议 30 天

## 2) 标准连接约定

- 标准 URL（唯一来源）: `http://<BIND_ADDR>`，默认 `http://127.0.0.1:8742`，由环境变量 `BIND_ADDR` 控制
- 鉴权模式（service/internal）: service（所有接口统一鉴权，`/healthz` 除外）
- token 来源与轮换策略: 环境变量 `API_BEARER_TOKEN` 静态配置，轮换需重启服务。**⚠️ 当前不支持自动轮换，建议后续改为支持多 token 或 JWT**
- 端口与路径基线:
  - metrics query path: `GET /api/public/metrics/query`
  - metrics names path: `GET /api/public/metrics/names`
  - metrics ingest path: `POST /v1/metrics/batch`
  - traces path: `GET /api/public/traces`
- 连接超时建议: 5s
- 查询超时建议: 10s（metrics/query 在大量数据时可能较慢，建议设 10s；正常场景 < 1s）

## 3) 指标契约（P0 必填）

### 3.1 pending_requests

- 指标名: `pending_requests`
- 类型: number (f64)
- 单位: count
- 合法范围: >= 0
- labels（必填）:
  - `model_uid`: 模型唯一标识
  - `replica_id`: 副本 ID
- labels（可选）:
  - `node_id`: 节点 ID（Nebula Node 上报时会携带）
- 缺失 labels 行为: 正常写入，查询时 `labels @>` 做 JSONB 包含匹配，缺失的 label 不影响查询（不会报错，只会导致该 series 不被匹配到）
- 空结果语义（无数据时）: 返回 `{"data": []}`（空数组），**不返回 0 值，也不返回 null**

### 3.2 kv_cache_usage

- 指标名: `kv_cache_usage`
- 类型: number (f64)
- 单位: ratio
- 合法范围: [0, 1]（xtrace 不做值域校验，写入方需保证）
- labels（必填）:
  - `model_uid`: 模型唯一标识
  - `replica_id`: 副本 ID
- labels（可选）:
  - `node_id`: 节点 ID
- 计算口径（used/total）: `EndpointStats.kv_cache_used_bytes / kv_cache_total_bytes`，由 Nebula Node 计算后上报
- 缺失 labels 行为: 同 3.1
- 空结果语义（无数据时）: 同 3.1，返回 `{"data": []}`

### 3.3 prefix_cache_hit_rate

- 指标名: `prefix_cache_hit_rate`
- 类型: number (f64)
- 单位: ratio
- 合法范围: [0, 1]（xtrace 不做值域校验，写入方需保证）
- labels（必填）:
  - `model_uid`: 模型唯一标识
  - `replica_id`: 副本 ID
- labels（可选）:
  - `node_id`: 节点 ID
- 统计窗口: 由 Nebula Node 心跳周期决定（~5s），xtrace 侧不做二次聚合
- 缺失 labels 行为: 同 3.1
- 空结果语义（无数据时）: 同 3.1，返回 `{"data": []}`

## 4) 查询语义与返回稳定性（P0/P1）

- 支持聚合:
  - `last`: ✅
  - `avg`: ✅（默认）
  - `max`: ✅
  - `min`: ✅
  - `sum`: ✅
- `last` 的精确定义: 每个时间桶内取 timestamp 最大的那条记录的 value。SQL 实现为 `(ARRAY_AGG(value ORDER BY timestamp DESC))[1]`
- `step` 参数语义: 时间桶宽度，使用 `to_timestamp(floor(extract(epoch from timestamp) / step) * step)` 对齐到桶起始时间。支持 `1m` (60s)、`5m` (300s)、`1h` (3600s)、`1d` (86400s)，默认 `1m`
- `from`/`to` 边界语义:
  - 类型: ISO8601 时间戳
  - 默认: `to` = 当前时间，`from` = `to - 1h`
  - 边界行为: 闭区间 `[from, to]`（SQL: `timestamp >= from AND timestamp <= to`）
  - 约束: `from > to` 返回 400
- 无数据时返回:
  - 空 series: ✅ 返回 `{"data": []}`
  - 0 值: ❌ 不补零
  - null: ❌ 不返回 null
- 部分成功语义（部分 series 有数据）: 只返回有数据的 series，缺失的 series 不会出现在结果中
- 时间戳字段与时区约定: UTC，RFC3339 格式（如 `2026-02-12T10:00:00+00:00`）

**查询限制:**
- 单次查询最多返回 50 条 series（`MAX_SERIES = 50`）
- 每条 series 最多返回 1000 个数据点（`MAX_POINTS_PER_SERIES = 1000`）
- 超出限制时静默截断，不报错

## 5) Freshness 与可用性 SLO（P1）

- 推荐 freshness 阈值（Router）: 15s。可直接使用 `meta.latest_ts` 与当前时间比较，差值 > 15s 视为 stale
- 推荐 freshness 阈值（Scheduler）: 30s。同上，差值 > 30s 视为 stale
- 目标查询成功率（5 分钟窗口）: 99.9%（xtrace 侧目标）
- 目标 p95 延迟: < 100ms（单指标、单 series 查询）
- 目标 p99 延迟: < 500ms
- 短时抖动容忍策略建议: Router/Scheduler 应缓存最近一次有效查询结果，查询失败或超时时使用缓存值降级，建议缓存 TTL 30s

metrics/query 响应已包含 `meta.latest_ts` 字段（所有 series 中最新数据点的 UTC 时间戳），Nebula 可直接用于 freshness 判断。无数据时该字段不出现。

## 6) 错误模型（P0 必填）

HTTP 状态映射：

| 场景 | HTTP | code | 可重试 | 说明 |
| --- | --- | --- | --- | --- |
| 鉴权失败 | 401 | `UNAUTHORIZED` | ❌ | `Authorization` 头缺失或 token 不匹配 |
| 参数错误 | 400 | `BAD_REQUEST` | ❌ | `name` 缺失、`from > to`、`step` 非法、`labels` JSON 解析失败等 |
| 限流 | 429 | `TOO_MANY_REQUESTS` | ✅ | 写入通道已满（ingest channel capacity=1000, metrics channel capacity=5000） |
| 上游超时 | 500 | `INTERNAL_ERROR` | ✅ | 数据库查询超时或连接池耗尽，映射为 sqlx::Error → 500 |
| 内部错误 | 500 | `INTERNAL_ERROR` | ✅ | 其他数据库错误 |
| 服务不可用 | 503 | `SERVICE_UNAVAILABLE` | ✅ | 写入通道已关闭（服务正在关闭） |
| 资源不存在 | 404 | `NOT_FOUND` | ❌ | 仅用于 trace 详情查询，metrics/query 无数据返回空数组而非 404 |

补充：
- 错误体 schema:
```json
{
  "message": "<人类可读的错误信息>",
  "code": "<机器可读的错误码枚举>",
  "data": null
}
```
- `code` 枚举值: `UNAUTHORIZED` | `BAD_REQUEST` | `TOO_MANY_REQUESTS` | `INTERNAL_ERROR` | `SERVICE_UNAVAILABLE` | `NOT_FOUND`
- 正常响应（200）不包含 `code` 字段
- request id 字段: **⚠️ 当前不支持。** 建议后续在响应头中添加 `X-Request-Id`（G2）

## 7) 限流与容量建议（P1）

- 服务端查询限流: ✅ 已实现 per-token 令牌桶（governor crate）
- 默认限额: 20 QPS sustained，burst 40（可通过 `RATE_LIMIT_QPS` / `RATE_LIMIT_BURST` 环境变量调整）
- 限流范围: 仅查询路由（`/api/public/metrics/*`、`/api/public/traces*`），写入路由不受限流影响（使用通道反压）
- 限流 key: 按认证 token 隔离（Bearer token 或 Basic auth username 各自独立计量）
- 单客户端推荐 QPS（Router）: ≤ 10 QPS
- 单客户端推荐 QPS（Scheduler）: ≤ 5 QPS
- 超限返回语义:
  - HTTP 429
  - `Retry-After` 响应头（秒，整数）
  - Body: `{"message": "Too Many Requests", "code": "TOO_MANY_REQUESTS", "data": null, "meta": {"rate_limit": {"remaining": 0, "reset_at": "<RFC3339>"}}}`
- 建议 backoff 策略: 读取 `Retry-After` 头作为最小等待时间，叠加随机抖动（±20%），指数退避上限 5s

## 8) 向后兼容与版本策略（P2）

- 契约版本号: v1.0.0（初始版本）
- 破坏性变更流程: 先发 proposal → 双方 review → staging 验证 → 提前通知 → 生产切换
- 提前通知周期: 至少 2 周
- 双栈兼容期: 旧接口保留至少 30 天

## 9) 对接测试样例（P0 必填）

请提供可复现样例（请求 + 期望响应）:

### 1. 正常有数据

**请求:**
```bash
curl -H "Authorization: Bearer <TOKEN>" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=pending_requests&step=1m&agg=last&labels=%7B%22model_uid%22%3A%22qwen-72b%22%7D"
```

**期望响应:** `200 OK`
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

### 2. 正常空数据

**请求:**
```bash
curl -H "Authorization: Bearer <TOKEN>" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=pending_requests&step=1m&agg=last&labels=%7B%22model_uid%22%3A%22nonexistent%22%7D"
```

**期望响应:** `200 OK`
```json
{
  "data": [],
  "meta": {
    "series_count": 0,
    "truncated": false
  }
}
```
注：空数据时 `latest_ts` 字段不出现（skip_serializing_if）。

### 3. 部分数据缺失

查询多个 replica，只有部分有数据。

**请求:**
```bash
curl -H "Authorization: Bearer <TOKEN>" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=kv_cache_usage&step=1m&agg=last&labels=%7B%22model_uid%22%3A%22qwen-72b%22%7D"
```

**期望响应:** `200 OK`（只返回有数据的 series）
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
注：`replica_id=1` 如果无数据，则不出现在结果中。

### 4. 鉴权失败

**请求:**
```bash
curl -H "Authorization: Bearer wrong_token" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=pending_requests"
```

**期望响应:** `401 Unauthorized`
```json
{
  "message": "Unauthorized",
  "code": "UNAUTHORIZED",
  "data": null
}
```

### 5. 限流（查询侧 — per-token 令牌桶）

**触发条件:** 同一 token 查询 QPS 超过限额（默认 20 qps, burst 40）

**请求:**
```bash
# 快速连续发送超过 burst 数量的请求
for i in $(seq 1 50); do
  curl -s -o /dev/null -w "%{http_code}\n" \
    -H "Authorization: Bearer <TOKEN>" \
    "http://127.0.0.1:8742/api/public/metrics/query?name=pending_requests" &
done
wait
```

**期望响应:** 超限请求返回 `429 Too Many Requests`

响应头:
```
Retry-After: 1
```

响应体:
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

注：写入侧（`POST /v1/metrics/batch`）使用通道反压，不经过令牌桶限流，超限同样返回 429 但无 `Retry-After` 头。

### 6. 服务内部错误

**触发条件:** 数据库不可达

**请求:**
```bash
curl -H "Authorization: Bearer <TOKEN>" \
  "http://127.0.0.1:8742/api/public/metrics/query?name=pending_requests"
```

**期望响应:** `500 Internal Server Error`
```json
{
  "message": "Internal Error",
  "code": "INTERNAL_ERROR",
  "data": null
}
```

## 10) 验收标准（双方签字）

### 10.1 Router 侧验收

- 24 小时内无解析失败（labels/类型/范围）
  - labels 为 JSONB，类型固定为 f64
  - pending_requests >= 0, kv_cache_usage ∈ [0,1], prefix_cache_hit_rate ∈ [0,1]
- freshness 过期数据被正确降级
  - **⚠️ 需 Router 自行实现**：对比返回 series 中最新 timestamp 与当前时间
- 无信号时策略可回退且可观测
  - 空 data 数组 → Router 回退到默认策略

### 10.2 Scheduler 侧验收

- autoscaling 信号可用率达标
  - 基于 metrics/query 的 200 成功率判断
- xtrace 抖动时不出现误扩缩容
  - Scheduler 应缓存最近有效结果，超时/失败时使用缓存

### 10.3 联合验收

- prefix_cache_hit_rate 在策略中可见生效
  - 可通过 `agg=last&step=1m` 获取最近值
- 错误码驱动的降级路径可复现
  - 401 → 修复 token
  - 429 → 退避重试（仅写入侧）
  - 500 → 退避重试
  - 200 + 空 data → 降级回退

## 11) 变更记录

| 日期 | 变更人 | 变更内容 |
| --- | --- | --- |
| 2026-02-14 | xtrace | 初始版本，基于 xtrace v0.0.7 代码实际行为填写 |

---

## 附录：当前差距与建议改进项

| # | 差距 | 优先级 | 状态 | 说明 |
|---|------|--------|------|------|
| G1 | 错误响应无 machine-readable `code` 字段 | P0 | ✅ 已实现 | 所有错误响应已增加 `"code"` 字段，枚举值：`UNAUTHORIZED`、`BAD_REQUEST`、`TOO_MANY_REQUESTS`、`INTERNAL_ERROR`、`SERVICE_UNAVAILABLE`、`NOT_FOUND`。正常响应不含 code 字段（skip_serializing_if） |
| G2 | 无 `X-Request-Id` 响应头 | P1 | 待实现 | 中间件层生成 UUID，写入响应头和日志 |
| G3 | metrics/query 响应无 freshness 元数据 | P1 | ✅ 已实现 | 响应新增 `meta` 对象，包含 `latest_ts`（最新数据点 UTC 时间戳）、`series_count`、`truncated` |
| G4 | 查询侧无 rate limiter | P1 | ✅ 已实现 | per-token 令牌桶（governor crate），查询路由 429 + Retry-After |
| G5 | 无值域校验（kv_cache_usage 允许 > 1） | P2 | 待实现 | 写入侧增加可选校验 |
| G6 | token 不支持自动轮换 | P2 | 待实现 | 支持多 token 或 JWT |
| G7 | 数据保留策略未实现 | P2 | 待实现 | 实现 `METRICS_RETENTION_DAYS` 后台清理 |
| G8 | 查询截断时无提示 | P2 | ✅ 已实现 | 超出 MAX_SERIES(50) 或 MAX_POINTS_PER_SERIES(1000) 时 `meta.truncated = true` |
