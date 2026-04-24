# xtrace 对 Langfuse Python 兼容层的需求说明

本文档定义 **xtrace 为支持 Xinference 而需要提供的 Langfuse Python 兼容能力**。

目标不是让 xtrace 服务端完整复刻 Langfuse 产品，也不是继续依赖官方 `langfuse` Python 包，而是提供一组**面向 Xinference 实际调用面**的最小兼容能力，使 Xinference 可以在不重写核心 tracing 逻辑的前提下，将 trace / generation / 指标写入 xtrace，并继续通过 `/api/public/*` 读取。

本文是 [docs/xinference_integration.md](docs/xinference_integration.md) 的补充文档。前者描述当前 Xinference 与 xtrace 的 HTTP 集成边界；本文聚焦 **Langfuse Python SDK 这一层** 的兼容要求。

---

## 1. 结论与建议

### 1.1 结论

如果目标是“从 Xinference 中剔除 Langfuse 包，但继续保留它当前的 tracing 编程模型”，则**仅改 xtrace 服务端 HTTP API 不够**。

原因是 Xinference 当前依赖的不只是：

- `LANGFUSE_HOST`
- `GET /api/public/*`
- `POST /v1/l/batch`

还直接依赖了 **Langfuse Python SDK 的运行时语义**，包括：

- `Langfuse(...)`
- `auth_check()`
- `@observe(...)`
- `start_generation(...)`
- `update_current_generation(...)`
- `TraceContext(...)`
- `resolve_media_references(...)`

其中 `@observe(...)`、上下文传播、当前 generation 的增量更新，都是**客户端能力**，不是服务端能力。

### 1.2 建议

建议采用“两层方案”：

1. xtrace 服务端继续提供并稳定维护 Langfuse 兼容读写 HTTP 契约。
2. xtrace 新增一个 **Python 兼容层**，实现 Langfuse Python SDK 的最小子集，供 Xinference 使用。

换句话说，推荐形态是：

- `xtrace server`
- `xtrace python compat layer`

而不是单独要求 `xtrace server` 吞掉所有 Python SDK 语义。

---

## 2. Xinference 当前真实依赖面

以下结论基于当前 `xinference-backend` 源码，而不是 Langfuse 官方完整 SDK 能力。

### 2.1 客户端对象

Xinference 需要构造一个类似下面形状的对象：

```python
Langfuse(
    secret_key=...,
    public_key=...,
    host=...,
    tracing_enabled=True | False,
)
```

当前使用位置包括：

- `xinference/v2/core/model.py`
- `xinference/v2/api/restful_api.py`
- `xinference/model/llm/mindie/core.py`

### 2.2 装饰器语义

Xinference 广泛使用：

```python
@observe(as_type="generation", capture_output=False)
```

以及：

```python
@observe()
```

该装饰器不是静态标记，而是需要：

1. 进入函数时建立一条 observation / generation。
2. 将当前 observation 放入上下文。
3. 在函数内部允许 `update_current_generation(...)` 获取并更新当前 observation。
4. 在函数退出或抛错时做收尾 flush。

### 2.3 generation 增量更新

Xinference 当前会先创建 generation，再多次补写字段，例如：

- `model`
- `input`
- `output`
- `metadata`
- `completion_start_time`
- `usage_details`

这要求兼容层支持：

```python
generation = langfuse.start_generation(...)
generation.update(...)
langfuse.update_current_generation(...)
```

### 2.4 父子关系传播

Xinference 会使用：

```python
TraceContext(trace_id=..., parent_span_id=...)
```

所以兼容层必须能把：

- `trace_id`
- `parent_span_id`

翻译为 xtrace 可存储的 trace / observation 关系，尤其是 `parentObservationId`。

### 2.5 媒体解析辅助

在单条 trace 详情读取路径中，Xinference 还会调用：

```python
langfuse.resolve_media_references(obj=data, resolve_with="base64_data_uri")
```

对于 xtrace 来说，第一阶段可以将其定义为 **no-op**：原样返回对象，不做额外解析。

---

## 3. 兼容层的推荐边界

### 3.1 第一阶段：Xinference-Compatible Subset

第一阶段不要追求“Langfuse Python SDK 全兼容”，而只实现 Xinference 当前真实使用到的子集。

推荐对外导出以下接口：

```python
class Langfuse:
    def __init__(self, *, secret_key: str, public_key: str, host: str, tracing_enabled: bool = True): ...
    def auth_check(self): ...
    def start_generation(self, *, name: str, trace_context: TraceContext | None = None): ...
    def update_current_generation(self, **kwargs): ...
    def resolve_media_references(self, obj, resolve_with: str = "base64_data_uri"): ...

class TraceContext:
    trace_id: str | None
    parent_span_id: str | None

def observe(as_type: str | None = None, capture_output: bool = True): ...
```

同时 generation handle 至少要支持：

```python
generation.update(...)
```

### 3.2 第二阶段：可独立发布的兼容包

如果后续不止 Xinference 要使用这层能力，建议演进为独立包，例如：

- `xtrace-langfuse-compat`
- 或 `xtrace.langfuse_compat`

这样其它依赖 Langfuse Python SDK 子集的应用也能直接接入 xtrace。

---

## 4. 对 xtrace 服务端的要求

Python 兼容层要能工作，xtrace 服务端至少必须稳定满足以下要求。

### 4.1 兼容读接口

必须支持并稳定维护：

- `GET /api/public/projects`
- `GET /api/public/traces`
- `GET /api/public/traces/{trace_id}`
- `GET /api/public/metrics/daily`

这些接口目前已在 Xinference 中用于：

- `auth_check()`
- trace 列表
- trace 详情
- 日聚合指标

### 4.2 兼容写接口

兼容层第一阶段推荐写入：

- `POST /v1/l/batch`

原因：

1. 该接口已经与当前 xtrace 兼容实现对齐。
2. 能直接表达 trace + observations。
3. 最适合做 `start_generation / update / flush` 这种客户端聚合后写入。

### 4.3 批量写入字段要求

xtrace 在 batch ingest 中必须继续稳定接受以下 observation 字段：

- `id`
- `traceId`
- `type`
- `name`
- `startTime`
- `endTime`
- `completionStartTime`
- `model`
- `input`
- `output`
- `metadata`
- `statusMessage`
- `parentObservationId`
- `promptTokens`
- `completionTokens`
- `totalTokens`
- `usage`

以及 trace 级字段：

- `id`
- `timestamp`
- `name`
- `input`
- `output`
- `userId`
- `sessionId`
- `metadata`
- `tags`

### 4.4 幂等要求

同一个 observation id 重复写入时，xtrace 应执行 upsert，而不是插入重复记录。

这是因为兼容层可能在：

1. 初次创建 generation 时写入一次。
2. 函数结束或流式完成时再次补写 output / usage。

同一个 trace id 重复写入时，xtrace 也应执行 upsert，而不是要求“先创建 trace，再单独查找并更新”。

这是 Xinference supervisor / worker 分离链路成立的前提：

1. supervisor 先创建当前 span，并生成 `trace_id` 与 `observation_id`。
2. worker 拿到该 `trace_id` 与父 `observation_id` 后，再写入 generation / usage / output。
3. trace 级字段允许由 supervisor 与 worker 分批补写，只要都落在同一个 `trace.id` 上即可。

也就是说，xtrace 需要支持的是“按同一个 `trace.id` 持续 upsert”，而不是额外提供一个独立的“先查 trace，再 update trace”的专用协议。

### 4.5 读接口返回契约要求

`GET /api/public/traces/{trace_id}` 返回的 observation 详情中，必须稳定包含：

- `parentObservationId`
- `completionStartTime`
- `promptTokens`
- `completionTokens`
- `totalTokens`
- `usage`

否则 Xinference UI 无法正确展示 generation 详情与层级关系。

---

## 5. 对 Python 兼容层的行为要求

### 5.1 `auth_check()`

行为定义：

1. 使用 `public_key:secret_key` 做 HTTP Basic Auth。
2. 请求 `GET {host}/api/public/projects`。
3. `200` 即判定成功。
4. 非 `200` 或请求异常即抛错。

### 5.2 `observe(...)`

行为定义：

1. 装饰函数进入时，创建一条 generation 或默认 observation。
2. 自动建立当前上下文。
3. 若存在父上下文，则继承 `trace_id`，并将父 observation id 作为 `parentObservationId`。
4. 正常返回时，如 `capture_output=True`，则补写 output。
5. 抛异常时，写入 `statusMessage` 或 error level。
6. 最终做 flush。

### 5.3 `start_generation(...)`

行为定义：

1. 返回一个 generation handle。
2. generation handle 内部应持有：
   - `trace_id`
   - `observation_id`
   - `parent_observation_id`
   - `start_time`
3. 支持 `update(...)` 多次补写。

若调用方显式传入 `TraceContext(trace_id=..., parent_span_id=...)`，则 generation 必须续写到该 `trace_id`，并将 `parent_span_id` 映射为 `parentObservationId`。这正是 Xinference 中 supervisor 先建 span、worker 再记录 generation 的对接方式。

### 5.4 `update_current_generation(...)`

行为定义：

1. 从上下文变量中取当前 generation。
2. 将传入字段 merge 到当前 observation 状态。
3. 在合适时机 flush 到 xtrace。

### 5.5 `TraceContext`

行为定义：

只要求保存：

- `trace_id`
- `parent_span_id`

其中 `parent_span_id` 在 xtrace 侧对应 `parentObservationId`。

### 5.6 `resolve_media_references(...)`

第一阶段要求：

- 允许调用
- 原样返回输入对象
- 不因未实现媒体解析而报错

---

## 6. 推荐的字段映射

### 6.1 usage 映射

兼容层建议将 Langfuse 风格 usage_details 统一归一到 xtrace：

| 兼容层输入 | xtrace 字段 |
| --- | --- |
| `input` / `promptTokens` / `prompt_tokens` | `promptTokens` |
| `output` / `completionTokens` / `completion_tokens` | `completionTokens` |
| `total` / `totalTokens` / `total_tokens` | `totalTokens` |

同时建议写出规范化 `usage`：

```json
{
  "input": 13,
  "output": 353,
  "total": 366,
  "unit": "TOKENS"
}
```

### 6.2 `observe(as_type="generation")` 映射

建议映射为 observation：

- `type = "GENERATION"`
- `name = <function name 或调用方指定 name>`

### 6.3 父子关系映射

| Langfuse Python 概念 | xtrace 字段 |
| --- | --- |
| `trace_id` | `trace.id` |
| `parent_span_id` | `parentObservationId` |
| 当前 generation id | `observation.id` |

---

## 7. 配置要求

为避免继续使用 `LANGFUSE_*` 作为唯一配置入口，建议 xtrace 与 Xinference 双方都接受以下别名：

- `XTRACE_HOST`
- `XTRACE_PUBLIC_KEY`
- `XTRACE_SECRET_KEY`

兼容层或 Xinference 内部适配层可以将其映射为：

- `LANGFUSE_HOST`
- `LANGFUSE_PUBLIC_KEY`
- `LANGFUSE_SECRET_KEY`

这样既能与现有 UI / 配置结构兼容，又能在语义上明确“当前后端是 xtrace，而不是 Langfuse”。

---

## 8. 非目标

以下能力不属于第一阶段目标：

1. 完整实现 Langfuse Python SDK 全部 API。
2. 实现 Prompt 管理、Dataset、Score、Experiment 等 Langfuse 产品能力。
3. 在 xtrace 服务端解析并执行 Python 装饰器语义。
4. 完整复制 Langfuse 的媒体引用处理行为。

---

## 9. 验收标准

### 9.1 最小可用验收

当以下条件全部成立时，可视为第一阶段完成：

1. Xinference 不再依赖官方 `langfuse` Python 包才能启动。
2. `xinference-local --help` 与 `xinference-local --host 0.0.0.0 --port 9997` 可以正常启动。
3. `auth_check()` 能通过 `GET /api/public/projects` 成功探测 xtrace。
4. 带 `@observe(as_type="generation")` 的 chat 路径能成功写入 xtrace。
5. 流式 chat 路径能补写：
   - output
   - completionStartTime
   - prompt/completion/total tokens
6. Xinference UI 中 trace 列表、trace 详情、daily metrics 可正常读取。

### 9.2 建议联调用例

1. 启动 xtrace，配置 Basic 与 Bearer 鉴权。
2. 启动 Xinference，将 `XTRACE_HOST` / `XTRACE_PUBLIC_KEY` / `XTRACE_SECRET_KEY` 指向 xtrace。
3. 拉起一个小模型，例如 `qwen2.5-instruct 0.5B`。
4. 发起一次非流式 chat。
5. 发起一次流式 chat。
6. 验证 xtrace 中出现：
   - trace
   - generation observation
   - usage
   - time to first token / completion start time
7. 验证 Xinference UI 的 `/v1/l/*` 代理读取结果正常。

---

## 10. 实施建议

### 10.1 推荐实施顺序

1. 先稳定 xtrace 现有 HTTP 兼容面。
2. 再实现 Python 兼容层的最小子集。
3. 先让 Xinference 单机链路跑通。
4. 最后再考虑将该兼容层抽成独立包。

### 10.2 不推荐的实施顺序

不建议：

1. 继续围绕官方 `langfuse` 包版本做适配。
2. 试图让 xtrace 服务端独立吸收客户端 SDK 语义。
3. 一开始就追求“Langfuse 全量 SDK 兼容”。

---

## 11. 与现有文档的关系

- [docs/xinference_integration.md](docs/xinference_integration.md)：描述当前 HTTP 集成路径与联调方法。
- 本文：定义 xtrace 为支持 **Langfuse Python SDK 这一层** 需要补齐的最小兼容面。

后续若 xtrace 团队决定实现独立 Python 兼容层，建议将本文拆分为：

1. 服务端契约要求
2. Python 兼容层接口契约
3. Xinference 联调用例

以便分别由后端、SDK、集成方维护。