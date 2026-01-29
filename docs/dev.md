# XTrace Rust 后端开发文档

本文档用于指导在本仓库内实现并开发一个“类似 Langfuse 的可观测性后台”，对外接口以 `docs/api.md` 为准。

## 目标与范围（MVP）

第一阶段仅实现：

- Trace 列表查询：`GET /api/public/traces`
- Trace 详情查询：`GET /api/public/traces/{traceId}`
- 日聚合指标：`GET /api/public/metrics/daily`

说明：为了让系统可用，后端还需要一组“内部写入接口”（Ingest）用于把 trace/observation 写入数据库。该部分不对外暴露给最终用户，但属于必须能力。

## 技术栈建议

- Web 框架：Axum（基于 Tokio）
- 序列化：Serde
- 数据库：PostgreSQL
- ORM/查询：SQLx（建议启用编译期校验）
- 日志与链路：tracing + tower-http（请求日志、trace id）
- 配置：环境变量（可选 dotenv）

## 目录结构（建议）

后端代码建议放在 `crates/` 或仓库根目录的 Rust workspace 中（按你们习惯决定）。推荐结构如下：

- `crates/xtrace-api/`：HTTP API（路由、DTO、鉴权、中间件）
- `crates/xtrace-service/`：业务服务层（查询、写入、聚合）
- `crates/xtrace-storage/`：SQLx 访问层（repo、迁移、模型映射）
- `migrations/`：数据库迁移

如果先做单体也可以：一个 crate 内分 `router/ service/ storage/` 三层。

## 本地开发依赖

- Rust stable（建议 >= 1.75）
- PostgreSQL 14+
- sqlx-cli（可选，但强烈建议用于迁移）

安装 sqlx-cli（需要你本机有 postgres 开发库/openssl 依赖，macOS 一般没问题）：

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

## 配置（环境变量）

服务启动至少需要：

- `DATABASE_URL`：例如 `postgres://user:pass@127.0.0.1:5432/xtrace`
- `API_BEARER_TOKEN`：对外 API 的 Bearer Token（MVP 先用单一 token）
- `BIND_ADDR`：监听地址，默认 `127.0.0.1:8080`

兼容 Langfuse public API 的 BasicAuth（可选）：

- `LANGFUSE_PUBLIC_KEY`
- `LANGFUSE_SECRET_KEY`

建议约定：

- 对外接口（`/api/public/*`）必须校验 `Authorization: Bearer <token>`
- 内部写入接口（如 `/v1/l/*`）可配置单独 token，或同 token（先简单）

## 数据库迁移

初始化数据库与迁移（示例）：

```bash
sqlx database create
sqlx migrate run
```

迁移内容（MVP 建议至少包含）：

- `traces` 表
- `observations` 表
- （可选）`daily_metrics` 表（预聚合）

## 运行与调试

本地运行（示例）：

```bash
export DATABASE_URL=postgres://user:pass@127.0.0.1:5432/xtrace
export API_BEARER_TOKEN=dev-token
export BIND_ADDR=127.0.0.1:8080
cargo run -p xtrace-api
```

健康检查建议提供：

- `GET /healthz`：不走鉴权，用于探活

## API 落地约定（对齐 docs/api.md）

### 统一响应结构

对外 public 接口响应结构以 Langfuse OpenAPI 为准：

`GET /api/public/traces` 返回分页对象：`{ data: [...], meta: { page, limit, totalItems, totalPages } }`

`GET /api/public/traces/{traceId}` 直接返回 trace 对象（不带外层 `data/meta/message` 包装）。

### 分页约定

`GET /api/public/traces` 返回 `data.meta`：

- `page`
- `limit`
- `totalItems`
- `totalPages`

### 时间过滤

`from_timestamp` / `to_timestamp` 使用 ISO8601；后端解析失败应返回 400。

### tags 过滤

`tags` 为数组参数；语义为“包含所有 tags（all-of）”。

### 排序

`order_by` 建议白名单：

- `timestamp.asc | timestamp.desc`
- `latency.asc | latency.desc`
- `totalCost.asc | totalCost.desc`

无效值返回 400。

## 最小验证（curl）

下面示例假设：

`API_BEARER_TOKEN=dev-token`，服务监听 `127.0.0.1:8080`。

```bash
export API_BEARER_TOKEN=dev-token
export BASE_URL=http://127.0.0.1:8080

# 如需用 BasicAuth（兼容 xinference/langfuse 调用方式）
# export LANGFUSE_PUBLIC_KEY=pk-xxx
# export LANGFUSE_SECRET_KEY=sk-yyy

# 1) traces 列表（默认返回 core + io + scores + observations + metrics）
curl -sS "$BASE_URL/api/public/traces?page=1&limit=2" \
  -H "Authorization: Bearer $API_BEARER_TOKEN"

# 2) traces 列表（fields 选择：不包含 io/scores/observations/metrics 时，
#    input/output/metadata 字段会被省略；latency/totalCost 会返回 -1；scores/observations 为空数组）
curl -sS "$BASE_URL/api/public/traces?page=1&limit=2&fields=core" \
  -H "Authorization: Bearer $API_BEARER_TOKEN"

# 3) traces 列表（tags all-of + environment 多值过滤）
curl -sS "$BASE_URL/api/public/traces?page=1&limit=2&tags=foo&tags=bar&environment=default" \
  -H "Authorization: Bearer $API_BEARER_TOKEN"

# 4) trace 详情（把 <TRACE_ID> 替换为真实 UUID）
curl -sS "$BASE_URL/api/public/traces/<TRACE_ID>" \
  -H "Authorization: Bearer $API_BEARER_TOKEN"

# 5) metrics/daily（默认最近 30 天，支持 fromTimestamp/toTimestamp、tags、traceName、userId）
curl -sS "$BASE_URL/api/public/metrics/daily?page=1&limit=50" \
  -H "Authorization: Bearer $API_BEARER_TOKEN"

# 6) traces 列表（BasicAuth）
curl -sS -u "$LANGFUSE_PUBLIC_KEY:$LANGFUSE_SECRET_KEY" \
  "$BASE_URL/api/public/traces?page=1&limit=2"
```

## 写入（Ingest）建议（内部接口）

为了实现完整闭环，建议增加内部写入接口（不属于 `docs/api.md` 的 public 部分）：

- `POST /v1/l/traces`：创建或 upsert trace
- `POST /v1/l/observations`：创建或 upsert observation

关键点：

- 支持幂等（以 `id` 或 `externalId` 作为 upsert key）
- 允许 observation 先到或 trace 先到（至少其中一种要能处理）

## 测试建议

- 路由层：用 `tower::ServiceExt` 做请求级测试
- 存储层：用 testcontainers 或本地 postgres 跑集成测试

## 里程碑

- M1：跑起来 + 鉴权 + DB 迁移 + ingest 写入
- M2：实现 `GET /api/public/traces` 与 `GET /api/public/traces/{trace_id}`
- M3：实现 `GET /api/public/metrics/daily`（先实时聚合，后续再做预聚合）
