# xtrace 改造需求：Nebula 可观测性集成

## 背景

Nebula 是一个分布式 GPU 推理集群管理系统，包含以下组件：

| 组件 | 职责 |
|------|------|
| **Node** | 管理单机 GPU、容器生命周期、引擎健康检查、metrics 采集 |
| **Router** | 基于 endpoint stats 做路由决策（least-pending、KV cache 感知） |
| **Gateway** | 对外 HTTP API 入口，代理请求到 Router，认证、限流 |

当前 Nebula 的运行时数据（endpoint stats、node status）存在 etcd 中用于实时路由决策，但缺少**历史可观测性**——无法回溯延迟趋势、GPU 利用率变化、请求量统计等。

Prometheus + Grafana 方案过重。xtrace 作为轻量级自研可观测服务，可以直接集成。

## 改造目标

在 xtrace 现有 trace/observation 能力基础上，增加 **metrics 时序数据** 写入和查询能力，使其能同时覆盖：

1. **Trace 类数据**（已有）：Gateway 每次请求的完整链路
2. **Metrics 类数据**（新增）：周期性指标快照（GPU、KV cache、QPS 等）

---

## 一、新增 Metrics 时序数据模型

### 1.1 数据库表

```sql
CREATE TABLE IF NOT EXISTS metrics (
  id BIGSERIAL PRIMARY KEY,
  project_id TEXT NOT NULL,
  environment TEXT NOT NULL DEFAULT 'default',

  -- 指标标识
  name TEXT NOT NULL,           -- e.g. "gpu_utilization", "kv_cache_usage", "request_count"
  
  -- 标签（用于过滤和分组）
  labels JSONB NOT NULL DEFAULT '{}',  -- e.g. {"node_id":"node-1","gpu_index":"0","model_uid":"qwen-72b"}

  -- 值
  value DOUBLE PRECISION NOT NULL,

  -- 时间戳
  timestamp TIMESTAMPTZ NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_metrics_name_ts ON metrics (project_id, name, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_metrics_labels_gin ON metrics USING GIN (labels);
CREATE INDEX IF NOT EXISTS idx_metrics_ts ON metrics (timestamp DESC);
```

### 1.2 写入 API

**`POST /v1/metrics/batch`**

批量写入指标数据点。

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

响应：`200 OK`

实现要求：
- 复用现有 `ingest_worker` 的 mpsc channel + 微批写入模式
- 使用 `COPY` 或多行 `INSERT` 批量写入

### 1.3 查询 API

**`GET /api/public/metrics/query`**

查询指标时序数据，支持时间范围、标签过滤、降采样。

参数：

| 参数 | 类型 | 说明 |
|------|------|------|
| `name` | string | 必填，指标名 |
| `from` | ISO8601 | 起始时间，默认 1 小时前 |
| `to` | ISO8601 | 结束时间，默认当前 |
| `labels` | string | 标签过滤，JSON 格式，e.g. `{"node_id":"node-1"}` |
| `step` | string | 降采样步长：`1m`, `5m`, `1h`, `1d`，默认 `1m` |
| `agg` | string | 聚合函数：`avg`, `max`, `min`, `sum`, `last`，默认 `avg` |

响应：

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

实现要求：
- 使用 `date_trunc` 或 `date_bin` 做时间桶降采样
- 按 labels 中的唯一组合分组返回多条时序
- 数据量大时考虑加 `LIMIT`（如最多返回 1000 个数据点 × 50 条时序）

**`GET /api/public/metrics/names`**

列出所有可用的指标名称。

```json
{
  "data": ["gpu_utilization", "gpu_memory_used_mb", "kv_cache_usage", "pending_requests", "request_count"]
}
```

---

## 二、Nebula 会上报的指标

### 2.1 Node 上报（每个心跳周期，~5s）

| 指标名 | labels | 值 | 来源 |
|--------|--------|----|------|
| `gpu_utilization` | `node_id`, `gpu_index` | 0-100 | `GpuStatus.utilization_gpu` |
| `gpu_temperature` | `node_id`, `gpu_index` | 摄氏度 | `GpuStatus.temperature_c` |
| `gpu_memory_used_mb` | `node_id`, `gpu_index` | MB | `GpuStatus.memory_used_mb` |
| `gpu_memory_total_mb` | `node_id`, `gpu_index` | MB | `GpuStatus.memory_total_mb` |
| `kv_cache_usage` | `model_uid`, `replica_id`, `node_id` | 0.0-1.0 | `EndpointStats.kv_cache_used_bytes / total` |
| `pending_requests` | `model_uid`, `replica_id`, `node_id` | count | `EndpointStats.pending_requests` |
| `prefix_cache_hit_rate` | `model_uid`, `replica_id`, `node_id` | 0.0-1.0 | `EndpointStats.prefix_cache_hit_rate` |

### 2.2 Gateway 上报（每次请求完成时，作为 trace）

使用现有 trace/observation 模型，无需改造：

- **Trace**：一次用户请求
  - `name`: 请求路径（如 `POST /v1/chat/completions`）
  - `userId`: 请求方标识
  - `tags`: `["gateway"]`
  - `latency`: 端到端延迟
- **Observation**（type=GENERATION）：
  - `model`: 实际使用的模型
  - `input/output`: 请求/响应体
  - `promptTokens/completionTokens`: token 用量

### 2.3 Router 上报（可选，作为 observation 挂在 Gateway trace 下）

- 路由决策耗时
- 选中的 endpoint

---

## 三、xtrace-client SDK 扩展

在 `xtrace-client` crate 中新增 metrics 写入方法：

```rust
impl Client {
    /// 批量写入指标数据点
    pub async fn push_metrics(&self, metrics: &[MetricPoint]) -> Result<(), Error>;
    
    /// 查询指标时序
    pub async fn query_metrics(&self, q: &MetricsQuery) -> Result<MetricsQueryResult, Error>;
    
    /// 列出所有指标名
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

## 四、数据保留策略

新增环境变量：

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `METRICS_RETENTION_DAYS` | `30` | metrics 表数据保留天数 |
| `TRACES_RETENTION_DAYS` | `90` | traces/observations 表数据保留天数 |

启动时创建一个后台任务，每小时执行一次清理：

```sql
DELETE FROM metrics WHERE timestamp < NOW() - INTERVAL '30 days';
DELETE FROM observations WHERE created_at < NOW() - INTERVAL '90 days';
DELETE FROM traces WHERE created_at < NOW() - INTERVAL '90 days';
```

---

## 五、实现优先级

| 优先级 | 内容 | 工作量 |
|--------|------|--------|
| **P0** | metrics 表 + `POST /v1/metrics/batch` 写入 | 小 |
| **P0** | `xtrace-client` 增加 `push_metrics` | 小 |
| **P1** | `GET /api/public/metrics/query` 降采样查询 | 中 |
| **P1** | `GET /api/public/metrics/names` | 小 |
| **P2** | 数据保留策略后台清理 | 小 |
| **P2** | `xtrace-client` 增加查询方法 | 小 |

P0 完成后 Nebula 即可开始接入上报；P1 完成后前端可展示 Dashboard。
