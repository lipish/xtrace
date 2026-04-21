# Xinference 与 xtrace 集成开发说明

本文档说明如何用 **xtrace** 替代 **Langfuse** 作为 Xinference 的可观测后端：基于 Xinference 源码中的真实调用关系编写，并与本仓库 HTTP 实现对照。OpenAPI 合集（如 `docs/xinference.md`）可能包含完整 Langfuse API 参考，**不代表** Xinference 运行时依赖；以本文与源码为准。

---

## 1. 目标与边界

**目标**：在不大改 Xinference 的前提下，将 `LANGFUSE_HOST` 指向 xtrace，使推理链路上的 Langfuse Python SDK 与控制台对 trace / 日聚合指标的读写走 xtrace。

**边界**：xtrace 定位为与 Langfuse **公共读接口 + SDK 摄入路径** 兼容的轻量后端，**不**等价于 Langfuse 全站能力（多租户、数据集、Prompt 管理、评分业务等）。若 Xinference 某功能依赖 Langfuse 专有行为，需单独验证（见第 6 节）。

---

## 2. 架构与数据流

Xinference 进程内使用官方 **Langfuse Python 包**，以 `LANGFUSE_HOST`、`LANGFUSE_PUBLIC_KEY`、`LANGFUSE_SECRET_KEY`（或等价环境变量）连接远端；控制台经 Xinference 自身路由 **`/v1/l/*`** 聚合展示，**服务端再向 `LANGFUSE_HOST` 发起 HTTP**，路径为 Langfuse **Public API** 形态，而非 `/v1/l`。

数据流可概括为两条：

1. **摄入（SDK）**：Worker / API 内 `Langfuse(...)`、`@observe`、`start_generation`、`update_current_generation` 等 → Langfuse 客户端 → xtrace 提供的 **batch / OTLP** 等兼容端点。
2. **查询（HTTP 直连）**：`xinference/v2/api/utils.py` 中 `requests` 调用 **`{LANGFUSE_HOST}/api/public/...`**，Basic 认证，供列表、详情与日指标聚合使用。

---

## 3. Xinference 源码中的依赖（权威清单）

以下路径以 **xinference-backend** 仓库为参照（若你本地目录名不同，以同名文件为准）。

### 3.1 服务端直连后端的只读接口

实现文件：`xinference/v2/api/utils.py`

| 用途 | 请求 URL（相对 `LANGFUSE_HOST`） | 认证 |
|------|----------------------------------|------|
| 日聚合指标 | `GET /api/public/metrics/daily` | HTTP Basic（public key : secret key） |
| Trace 列表 | `GET /api/public/traces` | 同上 |
| 单条 Trace | `GET /api/public/traces/{trace_id}` | 同上 |

未启用 Langfuse（如 `XINFERENCE_DISABLE_LANGFUSE`）时上述函数提前返回，不发起请求。

### 3.2 Xinference 对外暴露的代理式路由

实现文件：`xinference/v2/api/restful_api.py`（路由注册处可查 `/v1/l/...`）

与控制台对应关系：前端调 Xinference 的 **`/v1/l/metric/daily`、`/v1/l/traces`、`/v1/l/traces/{trace_id}`**，由 Handler 组装参数后调用上一节的 `get_langfuse_*`，**最终仍访问 `{LANGFUSE_HOST}/api/public/...`**。因此 **xtrace 只需实现标准 `/api/public/*`**，无需在 xtrace 侧实现 `/v1/l`。

### 3.3 Langfuse SDK 在进程内的用法（摄入与辅助）

出现位置包括但不限于：`xinference/v2/api/restful_api.py`（如 `create_chat_completion` 中 span、`auth_check`）、`xinference/v2/core/model.py`、`xinference/model/llm/mindie/core.py` 等。

典型能力：

- **`auth_check()`**：依赖与 Langfuse 兼容的鉴权与项目探测（xtrace 提供 `GET /api/public/projects`）。
- **Trace / Generation 上报**：走 SDK 内置摄入协议（需 xtrace 对 **`POST /v1/l/batch`** 或 **`POST /api/public/otel/v1/traces`** 等兼容实现一致）。
- **单条 Trace 详情**：在 `get_langfuse_trace` 中，先 `GET /api/public/traces/{id}` 取 JSON，再调用 **`langfuse.resolve_media_references(obj=data, resolve_with="base64_data_uri")`**。若 trace 内含 Langfuse 媒体引用字段，SDK 可能做额外解析；与纯 Langfuse 相比，**在 xtrace 上需做一次联调**，确认 UI 与多模态展示是否满足预期。

---

## 4. xtrace 侧必须可用的端点

与本仓库 `src/app.rs` 一致，与 Langfuse 兼容路径相关的包括：

| 端点 | 用途 |
|------|------|
| `GET /api/public/projects` | Langfuse SDK `auth_check` |
| `GET /api/public/traces` | Xinference `get_langfuse_traces` |
| `GET /api/public/traces/:traceId` | Xinference `get_langfuse_traces_by_trace_id` |
| `GET /api/public/metrics/daily` | Xinference `get_langfuse_daily*` |
| `POST /v1/l/batch` | 批量摄入（SDK 常用路径之一） |
| `POST /api/public/otel/v1/traces` | OTLP 摄入（若使用 OTLP 链路） |

所有受保护路由均需携带 **`Authorization`**（Bearer `API_BEARER_TOKEN` 或 Basic，与 `README.md` 一致）。更细的契约见 `docs/api.md` 与 `www/integrations/langfuse.md`。

---

## 5. 配置说明

### 5.1 xtrace 环境变量

至少：`DATABASE_URL`、`API_BEARER_TOKEN`。要与 Langfuse SDK / Xinference 的 Basic 模式对齐时，还需配置 **`XTRACE_PUBLIC_KEY` / `XTRACE_SECRET_KEY`**（或兼容名 `LANGFUSE_PUBLIC_KEY` / `LANGFUSE_SECRET_KEY`），与下文 Xinference 侧密钥**一致**。

监听地址默认 `127.0.0.1:8742`，生产请通过反向代理暴露 HTTPS。

### 5.2 Xinference 侧

将 **`LANGFUSE_HOST`** 设为 xtrace 的根 URL（含 scheme 与端口，无尾部斜杠问题以 Xinference 实现为准）。**公钥、私钥**与 xtrace 配置的 public/secret **同一对**。

可通过环境变量（见 Xinference `constants.py` 中 `XINFERENCE_ENV_LANGFUSE_*`）或 **`PUT /v1/setting/langfuse`** 写入，具体以你所部署的 Xinference 版本为准。

### 5.3 为何文档里的 `/v1/l` 与 xtrace 路径不一致

`docs/xinference.md` 中的 `/v1/l/*` 是 **Xinference API 网关前缀**；xtrace 作为独立服务只实现 **`/api/public/*`**。对接时 **把 Xinference 的「Langfuse 后端地址」指到 xtrace**，由 Xinference 负责把控制台请求转成对 `/api/public/*` 的调用，**不在 xtrace 重复实现 `/v1/l`**。

---

## 6. 与 Langfuse 的差异与风险

| 领域 | 说明 |
|------|------|
| 多租户 / 多项目 | xtrace 默认单项目；与 Xinference 单机部署通常可接受。 |
| Score / Dataset / Prompt | xtrace 未对齐 Langfuse 全量产品能力；若控制台仅展示 trace 与聚合指标，一般无感。 |
| `resolve_media_references` | 见 3.3，建议在真实负载下验证多模态与附件展示。 |
| 历史数据 | 切换后端后旧数据留在原 Langfuse；需迁移时另做数据层方案，不在本文范围。 |

---

## 7. 联调与验收建议

**健康检查**

```bash
curl -sS http://127.0.0.1:8742/healthz
curl -sS http://127.0.0.1:8742/readyz
```

**Basic 读接口（将密钥换为实际值）**

```bash
export H=http://127.0.0.1:8742
export AUTH='Authorization: Basic <base64(pk:sk)>'
curl -sS -H "$AUTH" "$H/api/public/projects"
curl -sS -H "$AUTH" "$H/api/public/traces?page=1&limit=10"
curl -sS -H "$AUTH" "$H/api/public/metrics/daily"
```

**Xinference 侧**：配置完成后发起一次推理请求，确认 Xinference 日志无 Langfuse 连接错误；在控制台打开 trace 列表与单条详情、概览中的日/周统计是否与预期一致。

---

## 8. 故障排查要点

- **401 / 403**：核对 Xinference 与 xtrace 的 public/secret 是否一致；Bearer 与 Basic 是否混用正确。
- **连接超时**：检查 `LANGFUSE_HOST` 是否可从 Xinference 进程访问（网络、防火墙、K8s Service 名）。
- **列表为空**：确认摄入成功（SDK 是否 flush、xtrace 日志与库表是否有数据）。
- **`auth_check` 失败**：确认 `GET /api/public/projects` 在相同 Basic 下可用。

---

## 9. 参考文档

| 文档 | 内容 |
|------|------|
| `README.md` | xtrace 运行与环境变量 |
| `docs/api.md` | HTTP 契约 |
| `www/integrations/langfuse.md` | Langfuse SDK 与 xtrace 对照 |
| `docs/session_ingest.md` | 会话维度元数据（若需多轮对话建模） |

---

## 10. 文档维护

当 Xinference 或 xtrace 任一修改 Langfuse 兼容路由或认证方式时，应同步更新本文；重大行为变更建议同时在 `docs/project_status.md` 的快照中提一句，便于审计。

---

## 11. 生产部署要点（面向 Xinference）

**探针**：编排系统应对 xtrace 使用 **`GET /readyz`** 作为 readiness（依赖数据库），**`GET /healthz`** 作为 liveness（仅进程存活）。二者均无需鉴权。

**安全**：生产环境必须配置 **`XTRACE_PUBLIC_KEY` / `XTRACE_SECRET_KEY`** 并与 Xinference 一致；**不要**设置 `XTRACE_ALLOW_UNAUTHENTICATED_COMPAT`（默认已关闭）。若未配置 Langfuse 兼容密钥，旧版行为曾允许在未配置密钥时对部分路由匿名访问；当前默认已禁止，仅当显式开启该变量且未配置密钥时才恢复（仅供本地开发）。

**资源**：按负载调大 `XTRACE_MAX_REQUEST_BODY_BYTES`（批量摄入）；查询限流由 `RATE_LIMIT_QPS` / `RATE_LIMIT_BURST` 控制。

**Trace 列表字段**：`GET /api/public/traces` 返回的每条 trace 包含与 Langfuse 控制台常见字段对齐的 **`projectId`、`createdAt`、`updatedAt`、`externalId`、`bookmarked`** 等，便于 Xinference 前端与其它集成方解析。
