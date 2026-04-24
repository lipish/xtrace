# Xinference 与 xtrace 集成开发说明

本文档说明如何用 **xtrace** 替代 **Langfuse** 作为 Xinference 的可观测后端：基于 Xinference 源码中的真实调用关系编写，并与本仓库 HTTP 实现对照。OpenAPI 合集（如 `docs/xinference.md`）可能包含完整 Langfuse API 参考，**不代表** Xinference 运行时依赖；以本文与源码为准。

若要进一步从 Xinference 中剔除官方 `langfuse` Python 包、转为由 xtrace 提供 Python 兼容层，见 [docs/xtrace_langfuse_python_compat.md](docs/xtrace_langfuse_python_compat.md)。

---

## 1. 目标与边界

**目标**：在不大改 Xinference 的前提下，将 `LANGFUSE_HOST` 指向 xtrace，使推理链路上的 Langfuse Python SDK 与控制台对 trace / 日聚合指标的读写走 xtrace。

**边界**：xtrace 定位为与 Langfuse **公共读接口 + SDK 摄入路径** 兼容的轻量后端，**不**等价于 Langfuse 全站能力（多租户、数据集、Prompt 管理、评分业务等）。若 Xinference 某功能依赖 Langfuse 专有行为，需单独验证（见第 6 节）。

---

## 2. 架构与数据流

Xinference 进程内使用官方 **Langfuse Python 包**，以 `LANGFUSE_HOST`、`LANGFUSE_PUBLIC_KEY`、`LANGFUSE_SECRET_KEY`（或等价环境变量）连接远端；控制台经 Xinference 自身路由 `**/v1/l/*`** 聚合展示，**服务端再向 `LANGFUSE_HOST` 发起 HTTP**，路径为 Langfuse **Public API** 形态，而非 `/v1/l`。

数据流可概括为两条：

1. **摄入（SDK）**：Worker / API 内 `Langfuse(...)`、`@observe`、`start_generation`、`update_current_generation` 等 → Langfuse 客户端 → xtrace 提供的 **batch / OTLP** 等兼容端点。
2. **查询（HTTP 直连）**：`xinference/v2/api/utils.py` 中 `requests` 调用 `**{LANGFUSE_HOST}/api/public/...`**，Basic 认证，供列表、详情与日指标聚合使用。

---

## 3. Xinference 源码中的依赖（权威清单）

以下路径以 **xinference-backend** 仓库为参照（若你本地目录名不同，以同名文件为准）。

### 3.1 服务端直连后端的只读接口

实现文件：`xinference/v2/api/utils.py`


| 用途       | 请求 URL（相对 `LANGFUSE_HOST`）          | 认证                                  |
| -------- | ----------------------------------- | ----------------------------------- |
| 日聚合指标    | `GET /api/public/metrics/daily`     | HTTP Basic（public key : secret key） |
| Trace 列表 | `GET /api/public/traces`            | 同上                                  |
| 单条 Trace | `GET /api/public/traces/{trace_id}` | 同上                                  |


未启用 Langfuse（如 `XINFERENCE_DISABLE_LANGFUSE`）时上述函数提前返回，不发起请求。

### 3.2 Xinference 对外暴露的代理式路由

实现文件：`xinference/v2/api/restful_api.py`（路由注册处可查 `/v1/l/...`）

与控制台对应关系：前端调 Xinference 的 `**/v1/l/metric/daily`、`/v1/l/traces`、`/v1/l/traces/{trace_id}`**，由 Handler 组装参数后调用上一节的 `get_langfuse_*`，**最终仍访问 `{LANGFUSE_HOST}/api/public/...`**。因此 **xtrace 只需实现标准 `/api/public/*`**，无需在 xtrace 侧实现 `/v1/l`。

### 3.3 Langfuse SDK 在进程内的用法（摄入与辅助）

出现位置包括但不限于：`xinference/v2/api/restful_api.py`（如 `create_chat_completion` 中 span、`auth_check`）、`xinference/v2/core/model.py`、`xinference/model/llm/mindie/core.py` 等。

典型能力：

- `**auth_check()**`：依赖与 Langfuse 兼容的鉴权与项目探测（xtrace 提供 `GET /api/public/projects`）。
- **Trace / Generation 上报**：走 SDK 内置摄入协议（需 xtrace 对 `**POST /v1/l/batch`** 或 `**POST /api/public/otel/v1/traces**` 等兼容实现一致）。
- **单条 Trace 详情**：在 `get_langfuse_trace` 中，先 `GET /api/public/traces/{id}` 取 JSON，再调用 `**langfuse.resolve_media_references(obj=data, resolve_with="base64_data_uri")`**。若 trace 内含 Langfuse 媒体引用字段，SDK 可能做额外解析；与纯 Langfuse 相比，**在 xtrace 上需做一次联调**，确认 UI 与多模态展示是否满足预期。

---

## 4. xtrace 侧必须可用的端点

与本仓库 `src/app.rs` 一致，与 Langfuse 兼容路径相关的包括：


| 端点                                | 用途                                           |
| --------------------------------- | -------------------------------------------- |
| `GET /api/public/projects`        | Langfuse SDK `auth_check`                    |
| `GET /api/public/traces`          | Xinference `get_langfuse_traces`             |
| `GET /api/public/traces/:traceId` | Xinference `get_langfuse_traces_by_trace_id` |
| `GET /api/public/metrics/daily`   | Xinference `get_langfuse_daily*`             |
| `POST /v1/l/batch`                | 批量摄入（SDK 常用路径之一）                             |
| `POST /api/public/otel/v1/traces` | OTLP 摄入（若使用 OTLP 链路）                         |


所有受保护路由均需携带 `**Authorization`**（Bearer `API_BEARER_TOKEN` 或 Basic，与 `README.md` 一致）。更细的契约见 `docs/api.md` 与 `www/integrations/langfuse.md`。

---

## 5. 配置说明

### 5.1 xtrace 环境变量

至少：`DATABASE_URL`、`API_BEARER_TOKEN`。要与 Langfuse SDK / Xinference 的 Basic 模式对齐时，还需配置 `**XTRACE_PUBLIC_KEY` / `XTRACE_SECRET_KEY**`（或兼容名 `LANGFUSE_PUBLIC_KEY` / `LANGFUSE_SECRET_KEY`），与下文 Xinference 侧密钥**一致**。

本地开发示例（PostgreSQL 角色与系统用户一致时，常见于本机）：`DATABASE_URL=postgresql://xinference@localhost:5432/xtrace`。

监听地址默认 `127.0.0.1:8742`，生产请通过反向代理暴露 HTTPS。

### 5.2 Xinference 侧

将 `**LANGFUSE_HOST`** 设为 xtrace 的根 URL（含 scheme 与端口，无尾部斜杠问题以 Xinference 实现为准）。**公钥、私钥**与 xtrace 配置的 public/secret **同一对**。

可通过环境变量（见 Xinference `constants.py` 中 `XINFERENCE_ENV_LANGFUSE_*`）或 `**PUT /v1/setting/langfuse`** 写入，具体以你所部署的 Xinference 版本为准。

### 5.3 为何文档里的 `/v1/l` 与 xtrace 路径不一致

`docs/xinference.md` 中的 `/v1/l/*` 是 **Xinference API 网关前缀**；xtrace 作为独立服务只实现 `**/api/public/*`**。对接时 **把 Xinference 的「Langfuse 后端地址」指到 xtrace**，由 Xinference 负责把控制台请求转成对 `/api/public/*` 的调用，**不在 xtrace 重复实现 `/v1/l`**。

---

## 6. 与 Langfuse 的差异与风险


| 领域                         | 说明                                                   |
| -------------------------- | ---------------------------------------------------- |
| 多租户 / 多项目                  | xtrace 默认单项目；与 Xinference 单机部署通常可接受。                 |
| Score / Dataset / Prompt   | xtrace 未对齐 Langfuse 全量产品能力；若控制台仅展示 trace 与聚合指标，一般无感。 |
| `resolve_media_references` | 见 3.3，建议在真实负载下验证多模态与附件展示。                            |
| 历史数据                       | 切换后端后旧数据留在原 Langfuse；需迁移时另做数据层方案，不在本文范围。             |


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

### 7.1 本次联调结论（2026-04-24）

本轮已在**远程测试环境**完成一轮从 compat 层到 Xinference HTTP 代理路由的实际验证，结论如下。

- 已确认 `supervisor -> worker` 的 trace continuation 模型可用：supervisor 先创建 span 并生成 `trace_id` / `observation_id`，worker 再以相同 `trace_id` 和父 `observation_id` 续写 generation；xtrace 侧不需要新增“先查 trace 再 update”的专门 API。
- 已确认 xtrace 的 batch ingest 语义满足该模型：trace 按 `trace.id` upsert，observation 按 `observation.id` upsert。
- 已将 Xinference 测试链路切到 `xinference.trace_compat`，`xinference.xtrace_compat` 保留为薄别名层。
- 已修复一个真实联调中暴露的 compat 层问题：未设置的 trace 字段会被序列化为 JSON `null`，其中 `trace.tags: null` 会导致 xtrace 的 `POST /v1/l/batch` 返回 `422`。修复后 compat 层改为**省略未设置字段**，而不是发送 `null`。
- 已在远程环境验证单独的 SPAN 可以通过 compat 层成功写入 xtrace。
- 已在远程环境验证真实主链路 `create_chat_completion -> worker.chat -> xtrace`：supervisor span 与 worker generation 均能落库，且 generation 的 `parentObservationId` 正确指向 span。
- 已在远程环境验证 xtrace 兼容读写面：
	- `GET /api/public/projects`
	- `GET /api/public/traces`
	- `GET /api/public/traces/{trace_id}`
	- `GET /api/public/metrics/daily`
	- `POST /v1/l/batch`
- 已在远程环境验证 Xinference 自身暴露的 HTTP 代理路由：
	- `GET /v1/l/traces`
	- `GET /v1/l/traces/{trace_id}`
	- `GET /v1/l/metric/daily`
- 已验证 trace / observation 的 `input` 与 `output` 可以经由 xtrace public API 正确读回，满足 Xinference trace 列表与详情页展示的核心需求。

本轮验证使用了仓库内脚本与临时探针的组合，其中长期保留在仓库中的脚本包括：

- `scripts/remote_xinference_e2e.py`：验证 `create_chat_completion -> worker.chat -> xtrace` 主链路。
- `scripts/xinference_chain_smoke_test.py`：验证 xtrace 兼容读写接口、鉴权与基础筛选行为。
- `scripts/full_integration_test.py`：验证 `input` / `output` round-trip。

### 7.2 当前明确未覆盖的范围

以下范围**不在本轮通过项内**，如需要上线前进一步收紧风险，应单独补测。

- `POST /api/public/otel/v1/traces`（OTLP 摄入链路）
- MindIE 的真实后端运行面
- Langfuse 更宽的产品能力，如 Prompt 管理、Dataset、Score 等

关于 MindIE：当前源码中 `xinference/model/llm/mindie/core.py` 确实接入了 tracing，但更像是**复用通用 compat 装饰器**的接入点，而不是像 `v2/core/model.py` 那样维护一条单独的 trace continuation 分支；因此本轮没有把 MindIE 作为单独验收面。

---

## 8. 故障排查要点

- **401 / 403**：核对 Xinference 与 xtrace 的 public/secret 是否一致；Bearer 与 Basic 是否混用正确。
- **连接超时**：检查 `LANGFUSE_HOST` 是否可从 Xinference 进程访问（网络、防火墙、K8s Service 名）。
- **列表为空**：确认摄入成功（SDK 是否 flush、xtrace 日志与库表是否有数据）。
- `**auth_check` 失败**：确认 `GET /api/public/projects` 在相同 Basic 下可用。

---

## 9. 参考文档


| 文档                             | 内容                       |
| ------------------------------ | ------------------------ |
| `README.md`                    | xtrace 运行与环境变量           |
| `docs/api.md`                  | HTTP 契约                  |
| `www/integrations/langfuse.md` | Langfuse SDK 与 xtrace 对照 |
| `docs/session_ingest.md`       | 会话维度元数据（若需多轮对话建模）        |


---

## 10. 文档维护

当 Xinference 或 xtrace 任一修改 Langfuse 兼容路由或认证方式时，应同步更新本文；重大行为变更建议同时在 `docs/project_status.md` 的快照中提一句，便于审计。

---

## 11. 生产部署要点（面向 Xinference）

**探针**：编排系统应对 xtrace 使用 `**GET /readyz`** 作为 readiness（依赖数据库），`**GET /healthz**` 作为 liveness（仅进程存活）。二者均无需鉴权。

**安全**：生产环境必须配置 `**XTRACE_PUBLIC_KEY` / `XTRACE_SECRET_KEY`** 并与 Xinference 一致；**不要**设置 `XTRACE_ALLOW_UNAUTHENTICATED_COMPAT`（默认已关闭）。若未配置 Langfuse 兼容密钥，旧版行为曾允许在未配置密钥时对部分路由匿名访问；当前默认已禁止，仅当显式开启该变量且未配置密钥时才恢复（仅供本地开发）。

**资源**：按负载调大 `XTRACE_MAX_REQUEST_BODY_BYTES`（批量摄入）；查询限流由 `RATE_LIMIT_QPS` / `RATE_LIMIT_BURST` 控制。

**Trace 列表字段**：`GET /api/public/traces` 返回的每条 trace 包含与 Langfuse 控制台常见字段对齐的 `**projectId`、`createdAt`、`updatedAt`、`externalId`、`bookmarked`** 等，便于 Xinference 前端与其它集成方解析。

**更多演示数据**：xtrace 已启动且已配置 `API_BEARER_TOKEN` 时，可执行 `python3 scripts/seed_demo_data.py` 写入多条 trace（多用户、标签、同会话多轮、跨日、书签与 externalId）及少量时序指标；脚本结束后可再执行 `python3 scripts/mock_xinference_public_api.py`，应能看到非空列表与第 4 步单条 trace 详情。

**接 Xinference 前自检**：在同一环境变量下执行 `python3 scripts/xinference_chain_smoke_test.py`，会依次检查探针、未授权/错误 Basic、Xinference 用的 Basic 读接口、`sessionId` 过滤、Bearer 摄入与错误令牌；全部通过后再配 Xinference 更稳。

**含 input/output 的端到端**：`python3 scripts/full_integration_test.py` 先用 Bearer 写入带聊天式 `input`/`output` 的 batch，再用 Basic 拉列表与详情并 **断言与写入内容一致**，覆盖「用户内容 ↔ 模型回复」这条 Langfuse 核心链。

**关于 prompt**：xtrace **没有** Langfuse 的 **Prompt 管理（模板库 / Prompts API）**；若 SDK 在 **observation** 上带上 `promptId`、`promptName`、`promptVersion`，会随 trace 详情返回。`xinference_chain_smoke_test.py` 与 `seed_demo_data.py` 已包含带上述字段的示例并做断言/灌库。