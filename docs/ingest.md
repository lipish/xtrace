# XTrace 写入（Ingest）与 Python SDK 设计（性能优先）

本文档定义 XTrace 的写入链路（internal ingest HTTP API）与 Python SDK 的行为约定，目标是在不明显影响在线请求延迟的前提下，记录 OpenAI 兼容接口（`messages -> completion`）的调用过程。

> 说明
> - `docs/api.md` 中的 `/api/public/*` 仅用于查询与聚合。
> - 写入接口为 internal，供 SDK/网关/服务端埋点调用。

## 设计目标

- 最小侵入：业务代码尽量少改（类似 Langfuse 的 drop-in wrapper 思路）。
- 异步优先：写入上报默认异步，不阻塞主请求。
- 可控丢弃：在极端情况下允许丢弃写入事件，以保护业务延迟与稳定性。
- 幂等：上报重试不产生重复数据。
- 批量：网络与数据库写入尽量批量化。

## 术语与数据模型

- Trace：一次业务级调用/请求的容器。
- Observation：Trace 内的事件/跨度。
  - Generation：一次模型调用（对应 chat completion）。

## Internal Ingest HTTP API（建议）

### 认证

统一使用 Bearer Token：

- `Authorization: Bearer <token>`

token 到 `projectId` 的映射方式（MVP）：

- 服务端配置静态 `projectId`（所有写入落同一 project）
- 或者 token 映射到 project（后续扩展）

### 1) 批量写入（推荐）

`POST /v1/l/batch`

请求体（概念结构）：

- `trace`：Trace 对象（可选）
- `observations`：Observation 数组（可选）

服务端语义：

- 支持只上报 observations（trace 不存在则延迟关联或创建占位 trace）
- 支持只上报 trace
- 对每条记录执行 upsert

返回建议：

- 200：全部成功
- 207：部分成功（返回失败项 id 与原因，SDK 可选择性重试）
- 400：请求体校验失败
- 401/403：鉴权失败
- 429：服务端背压（SDK 应退避）

### 2) 单条写入（可选，便于调试）

- `POST /v1/l/traces`
- `POST /v1/l/observations`

生产默认由 SDK 合并为 batch 上报。

### 幂等与 upsert key

- trace：以 `id` 作为主键 upsert
- observation：以 `id` 作为主键 upsert

SDK 生成规则建议：

- `trace_id`：每次 chat 请求生成 UUID
- `observation_id`：每次模型调用生成 UUID

### 字段约定（对齐 `docs/api.md` 返回结构）

Trace（MVP）：

- `id` (uuid)
- `timestamp` (ISO8601)
- `name` (string|null)
- `userId` (string|null)
- `sessionId` (string|null)
- `tags` (string[])
- `metadata` (object|null)
- `input` / `output`（可选，允许为空；更常见是放在 generation observation）

Generation Observation（MVP）：

- `id` (uuid)
- `traceId` (uuid)
- `type`: 固定 `GENERATION`
- `name`: 如 `chat`
- `startTime` / `endTime` / `completionStartTime`
- `model`
- `input`: messages 数组（role/content）
- `output`: completion 文本（或结构化）
- `usage`: { input, output, total, unit }
- `latency`（秒或毫秒需统一，建议毫秒；若沿用 `docs/api.md` 示例可继续用秒）
- `timeToFirstToken`
- `metadata`（可选）

## Python SDK（建议能力）

### 初始化

- 支持环境变量：
  - `XTRACE_BASE_URL`
  - `XTRACE_API_KEY`（Bearer token）
  - `XTRACE_PROJECT_ID`（可选，若服务端无法从 token 推导）

### 异步上报机制（核心）

SDK 内部维护：

- 内存队列（bounded queue）
- 后台 worker 线程/协程
- 批量聚合与定时 flush

建议默认参数（可配置）：

- `queue_max_size`: 10_000
- `batch_max_size`: 100
- `flush_interval_ms`: 500
- `request_timeout_ms`: 2_000
- `max_retries`: 3（指数退避）

队列满时策略（默认）：

- 丢弃新事件，并在本地计数（暴露 metrics），避免阻塞业务
- 可选策略：阻塞等待（仅用于离线/批处理场景）

退出与 flush：

- 提供 `flush()` 显式等待队列清空
- 提供 `shutdown()`（注册 `atexit` 尝试 flush，设置最大等待时间）

### OpenAI drop-in wrapper（建议）

目标：业务侧尽量只替换 import 或 client 初始化。

记录内容：

- 每次 `chat.completions.create(...)` 生成一个 trace + 一个 generation observation
- stream 场景：
  - 收到首 token 记录 `completionStartTime` / `timeToFirstToken`
  - 结束时写最终 `output` 与 `usage`

### 失败处理与背压

- 429/5xx：指数退避重试，超过重试次数后丢弃并计数
- 4xx（非 429）：认为是不可重试错误，丢弃并记录错误原因

## 服务端性能策略（Rust）

### 写入路径

- HTTP handler 只做轻量校验与鉴权
- 将 batch payload 进入内部 channel
- 后台写入任务：
  - 合并多个请求形成更大批量（例如每 50ms 或凑够 N 条）
  - 使用单次事务 + 批量 upsert

### 数据库

- 对 `traces(id)`、`observations(id)` 建主键
- 对 `observations(trace_id, start_time)` 建索引（详情页查询）
- 对 traces 的查询条件字段建索引：`project_id + timestamp`、`user_id`、`session_id`、`tags(GIN)`

### 背压

- 服务端内部队列满：直接返回 429
- 限流可先按进程级别做（后续再按 token/project 维度）

## 与 `docs/api.md` 的关系

- public 查询接口保持不变：`/api/public/*`
- ingest 接口只用于把 trace/observation 写入库，以支撑 public 查询

