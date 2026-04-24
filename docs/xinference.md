---
title: 默认模块
language_tabs:
  - shell: Shell
  - http: HTTP
  - javascript: JavaScript
  - ruby: Ruby
  - python: Python
  - php: PHP
  - java: Java
  - go: Go
toc_footers: []
includes: []
search: true
code_clipboard: true
highlight_theme: darkula
headingLevel: 2
generator: "@tarslib/widdershins v4.0.30"

---

# 默认模块

Base URLs:

# Authentication

- HTTP Authentication, scheme: bearer

- HTTP Authentication, scheme: basic

# Xinference/全局配置

## GET 全局配置

GET /v1/setting/global

全局配置接口: (之前的monitor接口字段迁移至此接口中)
1.监控地址: monitor_address，
2.langfuse: url, token
3.xx菜单的权限
4.用户菜单权限：["菜单1", "菜单2"]
5.xxxxGuide: 新功能引导

> 返回示例

```json
{
  "monitor_address": "http://192.168.1.16:33545",
  "langfuse_url": "http://192.168.1.16:3001",
  "langfuse_token": "xxx",
  "disable_langfuse": true,
  "showSettingGuide": true
}
```

```json
{
  "message": "Request Successful.",
  "data": {
    "monitor_address": "http://192.168.1.16:28902",
    "langfuse_url": "http://192.168.1.16:3001",
    "langfuse_token": "Basic cGstbGYtOGE3OWQ4NGUtNTUzNy00N2I3LTg2YmItYmEzNjI1NzY4NGYyOnNrLWxmLWQxZDQxYTBiLTY3MmUtNDljOS1hZWNlLWY5YTJiNDA4MjhkZQ==",
    "enable_langfuse": false,
    "show_setting_guide": false
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» monitor_address|string|true|none||none|
|»» langfuse_url|string|true|none||none|
|»» langfuse_token|string|true|none||none|
|»» enable_langfuse|boolean|true|none||none|
|»» show_setting_guide|boolean|true|none||none|

## PUT 修改全局配置

PUT /v1/setting/global

> Body 请求参数

```json
{
  "show_setting_guide": true
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» show_setting_guide|body|boolean| 是 |none|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "show_setting_guide": true
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» show_setting_guide|boolean|true|none||none|

# Xinference/平台监控

## GET 获取集群设备信息

GET /v1/device/info

获取集群中的设备信息，包含所使用的GPU数量，类型等信息。
对应`平台监控-在运行的模型实例数量`模块。
对应`平台监控-集群管理`模块。

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|curPageNum|query|integer| 是 |当前页数|
|numPerPage|query|integer| 是 |每页个数|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 1,
  "gpu_count": 2,
  "results": [
    {
      "uuid": "04:42:1a:ca:f5:ab",
      "name": "04:42:1a:ca:f5:ab",
      "status": "online",
      "gpu_count": 2,
      "worker_address": "0.0.0.0:35559",
      "gpus": {
        "gpu-0": {
          "name": "NVIDIA A800 80GB PCIe",
          "mem_total": 85899345920,
          "mem_free": 83109412864,
          "mem_used": 2789933056
        },
        "gpu-1": {
          "name": "NVIDIA A800 80GB PCIe",
          "mem_total": 85899345920,
          "mem_free": 72274804736,
          "mem_used": 13624541184
        }
      }
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» count|integer|true|none||none|
|» gpu_count|integer|true|none||none|
|» results|[object]|true|none||none|
|»» uuid|string|false|none||none|
|»» name|string|false|none||none|
|»» status|string|false|none||none|
|»» gpu_count|integer|false|none||none|
|»» worker_address|string|false|none||none|
|»» gpus|object|false|none||none|
|»»» gpu-0|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» mem_total|integer|true|none||none|
|»»»» mem_free|integer|true|none||none|
|»»»» mem_used|integer|true|none||none|
|»»» gpu-1|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» mem_total|integer|true|none||none|
|»»»» mem_free|integer|true|none||none|
|»»»» mem_used|integer|true|none||none|

#### 枚举值

|属性|值|
|---|---|
|status|online|
|status|offline|

## GET 获取集群节点信息

GET /v1/cluster/info

获取集群中在运行的节点信息。
对应`平台监控-节点`模块。
对应`平台监控-在运行的模型实例数量`模块。
对应`平台监控-集群管理`模块。
对应`平台监控-GPU数量包括类型`模块。

节点信息-》下线-同步需求改动：
新增id字段(workid)
node_name(worker名字，之前没有返回该字段)
新增worker状态： 用于判断什么条件下展示下线/同步按钮

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|curPageNum|query|integer| 是 |none|
|numPerPage|query|integer| 是 |none|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 2,
  "gpu_count": 2,
  "results": [
    {
      "node_name": "",
      "node_type": "Supervisor",
      "ip_address": "127.0.0.1",
      "gpu_count": 0,
      "gpu_type": "",
      "gpu_vram_total": 0
    },
    {
      "node_name": "机器1",
      "node_type": "Worker",
      "ip_address": "127.0.0.1",
      "gpu_count": 2,
      "gpu_type": "Nvidia",
      "gpu_vram_total": "49999MiB"
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» count|integer|true|none|节点总数|none|
|» gpu_count|integer|true|none|GPU总数|none|
|» results|[object]|true|none||none|
|»» node_name|string|true|none|名称|none|
|»» node_type|string|true|none|类型|none|
|»» ip_address|string|true|none|IP|none|
|»» gpu_count|integer|true|none|GPU数量|none|
|»» gpu_type|string|true|none|GPU类型|none|
|»» gpu_vram_total|any|true|none|vRAM(Total)|none|

*oneOf*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|»»» *anonymous*|string|false|none||none|

*xor*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|»»» *anonymous*|integer|false|none||none|

*continued*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|»» id|string|true|none|worker_id|下线，同步等用此字段|
|»» worker_status|string|true|none|worker状态|none|

## GET 获取集群GPU数量

GET /v1/cluster/devices

获取集群中GPU数量信息。
对应`平台监控-GPU数量`模块。

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 2
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» count|integer|true|none||none|

## GET 获取实时Tokens信息

GET /v1/prometheus/tokens

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "total_tokens": 126560,
  "ratio": 0.0647,
  "basis": 0.0647
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» total_tokens|integer|true|none||none|
|» ratio|number|true|none||none|
|» basis|number|true|none||none|

## GET 实例监控

GET /v1/monitors/instances/{model_uid}

使用此IP：18.116.197.226:3030 测试

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_uid|path|string| 是 |模型实例uid|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "throughput_monitor": "http://xx",
  "delay_monitor": "http://xx",
  "requests_monitor": "http://xx",
  "tokens_monitor": "http://xx"
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» throughput_monitor|string|true|none|吞吐图|none|
|» delay_monitor|string|true|none|延迟图|none|
|» requests_monitor|string|true|none|API请求图|none|
|» tokens_monitor|string|true|none|token图|none|

## GET 平台监控

GET /v1/monitors/cluster

使用此IP：18.116.197.226:3030 测试

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "cpu_monitor": "http://xx",
  "memory_monitor": "http://xx",
  "gpu_monitor": "http://xx",
  "resource_usage_monitor": "http://xx",
  "event_monitor": "http://xx"
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» cpu_monitor|string|true|none|CPU图|none|
|» memory_monitor|string|true|none|RAM图|none|
|» gpu_monitor|string|true|none|GPU图|none|

## GET worker_gpu_status

GET /api/v1/query

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|query|query|array[string]| 否 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET 获取监控地址

GET /v1/monitor

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

```json
{
  "monitor_address": "http://192.168.1.16:59628",
  "langfuse_url": "http://192.168.1.16:3001",
  "token": "Basic cGstbGYtOGE3OWQ4NGUtNTUzNy00N2I3LTg2YmItYmEzNjI1NzY4NGYyOnNrLWxmLWQxZDQxYTBiLTY3MmUtNDljOS1hZWNlLWY5YTJiNDA4MjhkZQ=="
}
```

```json
{
  "monitor_address": "http://192.168.1.16:59628",
  "langfuse_url": "http://192.168.1.16:3001",
  "token": "Basic cGstbGYtOGE3OWQ4NGUtNTUzNy00N2I3LTg2YmItYmEzNjI1NzY4NGYyOnNrLWxmLWQxZDQxYTBiLTY3MmUtNDljOS1hZWNlLWY5YTJiNDA4MjhkZQ=="
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» monitor_address|string|true|none||none|
|» langfuse_url|string|true|none||none|
|» token|string|true|none||none|

## GET 概览页

GET /v1/overview

> 返回示例

```json
{
  "message": "Request Successful.",
  "data": {
    "count_traces_total": 1,
    "data_source": [
      {
        "model": null,
        "count_traces": 1
      }
    ],
    "instance_count": 0,
    "batch_list": [
      {
        "batch_id": 1,
        "process": 1
      }
    ],
    "task_list": [
      {
        "task_name": "qwen2",
        "status": "pending"
      }
    ],
    "cpu_used_rate": 0.08807446473554022,
    "gpu_used_rate": 0,
    "tokensUsage": [
      {
        "date": "2024-11-26",
        "model": "qianwen1.5",
        "totalUsage": 100
      },
      {
        "date": "2024-11-26",
        "model": "qianwen2.5",
        "totalUsage": 200
      },
      {
        "date": "2024-11-27",
        "model": "qianwen1.5",
        "totalUsage": 300
      },
      {
        "date": "2024-11-27",
        "model": "qianwen2.5",
        "totalUsage": 400
      }
    ],
    "deviceInfo": {
      "uuid": "08:c0:eb:b3:6d:3c",
      "name": "08:c0:eb:b3:6d:3c",
      "status": "online",
      "gpu_count": 0,
      "worker_address": "192.168.1.16:29513",
      "gpus": {
        "gpu-0": {
          "name": "NVIDIA A800 80GB PCIe",
          "mem_total": 85899345920,
          "mem_free": 83109412864,
          "mem_used": 2789933056
        },
        "gpu-1": {
          "name": "NVIDIA A800 80GB PCIe",
          "mem_total": 85899345920,
          "mem_free": 83109412864,
          "mem_used": 2789933056
        }
      },
      "cpu": {
        "usage": 0.027000000000000003,
        "total": 96,
        "memory_used": 35686350848,
        "memory_available": 1039733338112,
        "memory_total": 1081827491840
      }
    }
  }
}
```

```json
{
  "message": "Request Successful.",
  "data": {
    "count_traces_total": 1,
    "data_source": [
      {
        "model": "qwen2.5",
        "count_traces": 1
      }
    ],
    "tokens_usage": [
      {
        "model": "qwen2.5",
        "date": "2024-12-02",
        "total_usage": 74
      },
      {
        "model": "qwen1.5",
        "date": "2024-11-29",
        "total_usage": 182
      },
      {
        "model": "qwen2.5",
        "date": "2024-11-29",
        "total_usage": 14949
      }
    ],
    "instance_count": 0,
    "batch_list": [
      {
        "batch_id": 2,
        "process": 1
      }
    ],
    "task_list": [
      {
        "task_name": "Xinf",
        "status": "finished"
      },
      {
        "task_name": "qwen2",
        "status": "pending"
      }
    ],
    "cpu_used_rate": 0.032241152478641746,
    "gpu_used_rate": 0.2628675348070347,
    "device_info": [
      {
        "uuid": "08:c0:eb:b3:6d:3c",
        "name": "08:c0:eb:b3:6d:3c",
        "status": "online",
        "gpu_count": 4,
        "worker_address": "192.168.1.16:25214",
        "gpus": {
          "gpu-0": {
            "name": "NVIDIA GeForce RTX 4090 D",
            "mem_total": 25757220864,
            "mem_free": 3422617600,
            "mem_used": 22334603264
          },
          "gpu-1": {
            "name": "NVIDIA GeForce RTX 4090 D",
            "mem_total": 25757220864,
            "mem_free": 25156780032,
            "mem_used": 600440832
          },
          "gpu-2": {
            "name": "NVIDIA GeForce RTX 4090 D",
            "mem_total": 25757220864,
            "mem_free": 25156780032,
            "mem_used": 600440832
          },
          "gpu-3": {
            "name": "NVIDIA GeForce RTX 4090 D",
            "mem_total": 25757220864,
            "mem_free": 22209757184,
            "mem_used": 3547463680
          }
        },
        "cpu": {
          "usage": 0.025,
          "total": 96,
          "memory_used": 34879365120,
          "memory_available": 1036305907712,
          "memory_total": 1081827491840
        }
      }
    ]
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» count_traces_total|integer|true|none||none|
|»» data_source|[object]|true|none||none|
|»»» model|string|false|none||none|
|»»» count_traces|integer|false|none||none|
|»» tokens_usage|[object]|true|none||none|
|»»» model|string|true|none||none|
|»»» date|string|true|none||none|
|»»» total_usage|integer|true|none||none|
|»» instance_count|integer|true|none||none|
|»» batch_list|[object]|true|none||none|
|»»» batch_id|integer|false|none||none|
|»»» process|integer|false|none||none|
|»» task_list|[object]|true|none||none|
|»»» task_name|string|true|none||none|
|»»» status|string|true|none||none|
|»» cpu_used_rate|number|true|none||none|
|»» gpu_used_rate|number|true|none||none|
|»» device_info|[object]|true|none||none|
|»»» uuid|string|false|none||none|
|»»» name|string|false|none||none|
|»»» status|string|false|none||none|
|»»» gpu_count|integer|false|none||none|
|»»» worker_address|string|false|none||none|
|»»» gpus|object|false|none||none|
|»»»» gpu-0|object|true|none||none|
|»»»»» name|string|true|none||none|
|»»»»» mem_total|integer|true|none||none|
|»»»»» mem_free|integer|true|none||none|
|»»»»» mem_used|integer|true|none||none|
|»»»» gpu-1|object|true|none||none|
|»»»»» name|string|true|none||none|
|»»»»» mem_total|integer|true|none||none|
|»»»»» mem_free|integer|true|none||none|
|»»»»» mem_used|integer|true|none||none|
|»»»» gpu-2|object|true|none||none|
|»»»»» name|string|true|none||none|
|»»»»» mem_total|integer|true|none||none|
|»»»»» mem_free|integer|true|none||none|
|»»»»» mem_used|integer|true|none||none|
|»»»» gpu-3|object|true|none||none|
|»»»»» name|string|true|none||none|
|»»»»» mem_total|integer|true|none||none|
|»»»»» mem_free|integer|true|none||none|
|»»»»» mem_used|integer|true|none||none|
|»»» cpu|object|false|none||none|
|»»»» usage|number|true|none||none|
|»»»» total|integer|true|none||none|
|»»»» memory_used|integer|true|none||none|
|»»»» memory_available|integer|true|none||none|
|»»»» memory_total|integer|true|none||none|

## POST 下线worker

POST /v1/worker/offline

> Body 请求参数

```json
{
  "worker_id": "string"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» worker_id|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## POST 同步worker

POST /v1/worker/sync

> Body 请求参数

```json
{
  "worker_id": "string"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» worker_id|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## PUT 同步worker节点

PUT /v1/worker/sync

> Body 请求参数

```json
{
  "worker_address": "0.0.0.0:44414",
  "sync_worker_address": "0.0.0.0:57418"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» worker_address|body|string| 是 |none|
|» sync_worker_address|body|string| 是 |none|

> 返回示例

> 200 Response

```json
null
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|null|

## GET 获取需要同步的worker列表

GET /v1/worker_source/sync

> 返回示例

> 200 Response

```json
[
  {
    "worker_name": "string",
    "worker_id": "string",
    "instances_source": [
      {
        "model_name": "string",
        "model_type": "string",
        "model_uid": "string",
        "model_engine": "string",
        "model_version": "string",
        "model_ability": [
          "string"
        ],
        "replica": 0,
        "status": "string",
        "instance_created_ts": 0,
        "n_gpu": "string",
        "gpu_idx": [
          "string"
        ],
        "peft_model_config": {
          "lora_list": [
            {}
          ],
          "image_lora_load_kwargs": null,
          "image_lora_fuse_kwargs": null
        },
        "is_builtin": true,
        "error_info": null
      }
    ]
  }
]
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» worker_name|string|false|none||none|
|» worker_id|string|false|none||none|
|» instances_source|[object]|false|none||none|
|»» model_name|string|true|none||none|
|»» model_type|string|true|none||none|
|»» model_uid|string|true|none||none|
|»» model_engine|string|true|none||none|
|»» model_version|string|true|none||none|
|»» model_ability|[string]|true|none||none|
|»» replica|integer|true|none||none|
|»» status|string|true|none||none|
|»» instance_created_ts|integer|true|none||none|
|»» n_gpu|string|true|none||none|
|»» gpu_idx|[string]|true|none||none|
|»» peft_model_config|object|true|none||none|
|»»» lora_list|[object]|true|none||none|
|»»»» lora_name|string|false|none||none|
|»»»» local_path|string|false|none||none|
|»»» image_lora_load_kwargs|null|true|none||none|
|»»» image_lora_fuse_kwargs|null|true|none||none|
|»» is_builtin|boolean|true|none||none|
|»» error_info|null|true|none||none|

## DELETE 下线worker节点

DELETE /v1/workers

> Body 请求参数

```json
{
  "worker_address": "0.0.0.0:57418"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» worker_address|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful."
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|

## GET 获取Worker列表

GET /v1/workers

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|detail|query|string| 是 |none|

> 返回示例

> 200 Response

```json
[
  {
    "work-ip": "0.0.0.0:15035",
    "models": {}
  },
  {
    "work-ip": "0.0.0.0:43083",
    "models": {}
  },
  {}
]
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» work-ip|string|true|none||none|
|» models|object|true|none||none|

# Xinference/模型监控

## GET 模型监控

GET /v1/monitors/models/{model_type}

使用此IP：18.116.197.226:3030 测试

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_type|path|string| 是 |模型类型|
|curPageNum|query|integer| 是 |none|
|numPerPage|query|integer| 是 |none|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 50,
  "results": [
    {
      "model_uid": "qwen-chat-aabb",
      "monitor_url": "http://xx"
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» count|integer|true|none||none|
|» results|[object]|true|none||none|
|»» model_uid|string|true|none||none|
|»» monitor_url|string|true|none||none|

## GET query接口

GET /v1/p/query

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|query|query|string| 是 |记录的指标值|
|time|query|string| 否 |时间戳|
|timeout|query|string| 否 |超时时间|
|limit|query|string| 否 |返回序列的长度，0表示禁用|

> 返回示例

```json
{
  "message": "Request Successful.",
  "data": {
    "status": "success",
    "data": {
      "resultType": "vector",
      "result": [
        {
          "metric": {
            "__name__": "xinference:input_tokens_total",
            "format": "pytorch",
            "instance": "10.1.0.44:33619",
            "job": "worker-4738bae2",
            "model": "qwen3",
            "node": "10.1.0.44:59656",
            "quantization": "none",
            "type": "LLM",
            "user_id": "administrator"
          },
          "value": [
            1750908056.899,
            "13"
          ]
        }
      ]
    }
  }
}
```

```json
{
  "message": "Request Successful.",
  "data": {
    "status": "success",
    "data": {
      "resultType": "vector",
      "result": []
    }
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|false|none||none|
|»» status|string|true|none||状态|
|»» data|object|true|none||none|
|»»» resultType|string|true|none||none|
|»»» result|[object]|true|none||none|
|»»»» metric|object|false|none||none|
|»»»»» __name__|string|false|none||none|
|»»»»» format|string|false|none||none|
|»»»»» instance|string|false|none||none|
|»»»»» job|string|false|none||none|
|»»»»» model|string|false|none||none|
|»»»»» node|string|false|none||none|
|»»»»» quantization|string|false|none||none|
|»»»»» type|string|false|none||none|
|»»»»» user_id|string|false|none||none|
|»»»» value|[any]|false|none||none|

## GET query_range接口

GET /v1/p/query_range

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|query|query|string| 是 |记录的指标值|
|start|query|string| 是 |时间戳|
|end|query|string| 是 |时间戳|
|step|query|string| 是 |步长|
|timeout|query|string| 否 |超时时间|
|limit|query|string| 否 |返回序列的长度，0表示禁用|

> 返回示例

```json
{
  "message": "Request Successful.",
  "data": {
    "status": "success",
    "data": {
      "resultType": "matrix",
      "result": [
        {
          "metric": {
            "__name__": "xinference:input_tokens_total_gauge",
            "format": "pytorch",
            "instance": "10.1.0.44:33619",
            "job": "worker-4738bae2",
            "model": "qwen3",
            "node": "10.1.0.44:59656",
            "quantization": "none",
            "type": "LLM",
            "user_id": "administrator"
          },
          "values": [
            [
              1750908066,
              "13"
            ],
            [
              1750908246,
              "13"
            ],
            [
              1750908426,
              "13"
            ],
            [
              1750908606,
              "13"
            ],
            [
              1750908786,
              "13"
            ],
            [
              1750908966,
              "13"
            ],
            [
              1750909146,
              "13"
            ],
            [
              1750909326,
              "13"
            ],
            [
              1750909506,
              "107"
            ],
            [
              1750909686,
              "107"
            ]
          ]
        }
      ]
    }
  }
}
```

```json
{
  "message": "Request Successful.",
  "data": {
    "status": "success",
    "data": {
      "resultType": "matrix",
      "result": []
    }
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|false|none||none|
|»» status|string|true|none||状态|
|»» data|object|true|none||none|
|»»» resultType|string|true|none||none|
|»»» result|[object]|true|none||none|
|»»»» metric|object|false|none||none|
|»»»»» __name__|string|false|none||none|
|»»»»» format|string|false|none||none|
|»»»»» instance|string|false|none||none|
|»»»»» job|string|false|none||none|
|»»»»» model|string|false|none||none|
|»»»»» node|string|false|none||none|
|»»»»» quantization|string|false|none||none|
|»»»»» type|string|false|none||none|
|»»»»» user_id|string|false|none||none|
|»»»» value|[any]|false|none||none|

## GET daily接口

GET /v1/l/metric/daily

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|traceName|query|string| 否 |名称|
|userId|query|string| 否 |记录的用户id|
|tags|query|array[string]| 否 |标签|
|fromTimestamp|query|string| 否 |iso 8601格式|
|toTimestamp|query|string| 否 |iso 8601格式|

> 返回示例

```json
{
  "message": "Request Successful.",
  "data": {
    "data_source": {
      "data": [
        {
          "date": "2025-06-26",
          "countTraces": 1,
          "countObservations": 1,
          "totalCost": 0,
          "usage": [
            {
              "model": "qwen3",
              "inputUsage": 13,
              "outputUsage": 353,
              "totalUsage": 366,
              "totalCost": 0,
              "countObservations": 1,
              "countTraces": 1
            }
          ]
        }
      ],
      "meta": {
        "page": 1,
        "limit": 50,
        "totalItems": 1,
        "totalPages": 1
      }
    }
  }
}
```

```json
{
  "message": "Request Successful.",
  "data": {
    "data_source": null
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|false|none||none|
|»» data_source|object|false|none||none|
|»»» data|[object]|false|none||none|
|»»»» date|string|false|none||none|
|»»»» countTraces|integer|false|none||none|
|»»»» countObservations|integer|false|none||none|
|»»»» totalCost|integer|false|none||none|
|»»»» usage|[object]|false|none||none|
|»»»»» model|string|false|none||none|
|»»»»» inputUsage|integer|false|none||none|
|»»»»» outputUsage|integer|false|none||none|
|»»»»» totalUsage|integer|false|none||none|
|»»»»» totalCost|integer|false|none||none|
|»»»»» countObservations|integer|false|none||none|
|»»»»» countTraces|integer|false|none||none|
|»»» meta|object|false|none||none|
|»»»» page|integer|false|none||none|
|»»»» limit|integer|false|none||none|
|»»»» totalItems|integer|false|none||none|
|»»»» totalPages|integer|false|none||none|

## GET traces接口

GET /v1/l/traces

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|page|query|integer| 否 |页数|
|limit|query|integer| 否 |返回条数限制|
|user_id|query|string| 否 |记录的用户id|
|name|query|string| 否 |记录的名称|
|session_id|query|string| 否 |记录的session_id|
|from_timestamp|query|string| 否 |iso 8601格式|
|to_timestamp|query|string| 否 |iso 8601格式|
|order_by|query|string| 否 |排序|
|tags|query|array[string]| 否 |标签|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "data": [
      {
        "id": "2b19f7aa-3c9e-4102-b31f-fdc461a9991d",
        "timestamp": "2025-06-26T06:16:19.504000Z",
        "name": null,
        "input": null,
        "output": null,
        "sessionId": null,
        "release": null,
        "version": null,
        "userId": "administrator",
        "metadata": null,
        "tags": [],
        "public": false,
        "htmlPath": "/project/20250101/traces/2b19f7aa-3c9e-4102-b31f-fdc461a9991d",
        "latency": 12.771000146865845,
        "totalCost": 0,
        "observations": [
          "96e16fda-f796-414d-8cfb-61f9a4343be0"
        ],
        "scores": [],
        "externalId": null,
        "bookmarked": false,
        "projectId": "20250101",
        "createdAt": "2025-06-26T06:16:27.893Z",
        "updatedAt": "2025-06-26T06:16:27.893Z"
      }
    ],
    "meta": {
      "page": 1,
      "limit": 50,
      "totalItems": 1,
      "totalPages": 1
    }
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|false|none||none|
|»» data|[object]|false|none||none|
|»»» id|string|false|none||none|
|»»» timestamp|string¦null|false|none||none|
|»»» name|string¦null|false|none||none|
|»»» input|string¦null|false|none||none|
|»»» output|string¦null|false|none||none|
|»»» sessionId|string¦null|false|none||none|
|»»» release|string¦null|false|none||none|
|»»» version|string¦null|false|none||none|
|»»» userId|string¦null|false|none||none|
|»»» metadata|object¦null|false|none||none|
|»»» tags|[string]¦null|false|none||none|
|»»» public|boolean|false|none||none|
|»»» htmlPath|string¦null|false|none||none|
|»»» latency|string|false|none||none|
|»»» totalCost|string|false|none||none|
|»»» observations|[string]|false|none||none|
|»»» scores|[string]¦null|false|none||none|
|»»» externalId|string¦null|false|none||none|
|»»» bookmarked|boolean|false|none||none|
|»»» projectId|string|false|none||none|
|»»» createdAt|string|false|none||none|
|»»» updatedAt|string|false|none||none|
|»» meta|object|false|none||none|
|»»» page|integer|false|none||none|
|»»» limit|integer|false|none||none|
|»»» totalItems|integer|false|none||none|
|»»» totalPages|integer|false|none||none|

## GET trace接口

GET /v1/l/traces/{trace_id}

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|trace_id|path|string| 是 |通过traces接口获取的id的值|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "id": "2b19f7aa-3c9e-4102-b31f-fdc461a9991d",
    "timestamp": "2025-06-26T06:16:19.504000Z",
    "name": null,
    "input": null,
    "output": null,
    "sessionId": null,
    "release": null,
    "version": null,
    "userId": "administrator",
    "metadata": null,
    "tags": [],
    "public": false,
    "htmlPath": "/project/20250101/traces/2b19f7aa-3c9e-4102-b31f-fdc461a9991d",
    "latency": 12.77100014686584,
    "totalCost": 0,
    "observations": [
      {
        "id": "96e16fda-f796-414d-8cfb-61f9a4343be0",
        "traceId": "2b19f7aa-3c9e-4102-b31f-fdc461a9991d",
        "type": "GENERATION",
        "name": "chat",
        "startTime": "2025-06-26T06:16:19.504000Z",
        "endTime": "2025-06-26T06:16:32.275000Z",
        "completionStartTime": "2025-06-26T06:16:20.716000Z",
        "model": "qwen3",
        "modelParameters": null,
        "input": [
          {
            "role": "user",
            "content": "test"
          }
        ],
        "version": null,
        "metadata": {
          "stream": true,
          "stream_options": {
            "include_usage": true
          }
        },
        "output": "test",
        "usage": {
          "input": 13,
          "output": 353,
          "total": 366,
          "unit": "TOKENS"
        },
        "level": "DEFAULT",
        "statusMessage": null,
        "parentObservationId": null,
        "promptId": null,
        "promptName": null,
        "promptVersion": null,
        "modelId": null,
        "inputPrice": null,
        "outputPrice": null,
        "totalPrice": null,
        "calculatedInputCost": null,
        "calculatedOutputCost": null,
        "calculatedTotalCost": null,
        "latency": 12.771,
        "timeToFirstToken": 1.212,
        "completionTokens": 353,
        "unit": "TOKENS",
        "totalTokens": 366,
        "projectId": "20250101",
        "createdAt": "2025-06-26T06:16:28.040Z",
        "promptTokens": 13,
        "updatedAt": "2025-06-26T06:16:32.306Z"
      }
    ],
    "scores": [],
    "externalId": null,
    "bookmarked": false,
    "projectId": "20250101",
    "createdAt": "2025-06-26T06:16:27.893Z",
    "updatedAt": "2025-06-26T06:16:27.893Z"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|false|none||none|
|»» id|string|false|none||ID 编号|
|»» timestamp|string|false|none||none|
|»» name|string¦null|false|none||none|
|»» input|string¦null|false|none||none|
|»» output|string¦null|false|none||none|
|»» sessionId|string¦null|false|none||none|
|»» release|string¦null|false|none||none|
|»» version|string¦null|false|none||none|
|»» userId|string¦null|false|none||none|
|»» metadata|object¦null|false|none||none|
|»» tags|[string]¦null|false|none||none|
|»» public|boolean|false|none||none|
|»» htmlPath|string¦null|false|none||none|
|»» latency|number|false|none||none|
|»» totalCost|number|false|none||none|
|»» observations|[object]|false|none||none|
|»»» id|string|false|none||none|
|»»» traceId|string|false|none||none|
|»»» type|string|false|none||none|
|»»» name|string|false|none||none|
|»»» startTime|string|false|none||none|
|»»» endTime|string|false|none||none|
|»»» completionStartTime|string|false|none||none|
|»»» model|string|false|none||none|
|»»» modelParameters|object¦null|false|none||none|
|»»» input|[object]|false|none||none|
|»»»» role|string|false|none||none|
|»»»» content|string|false|none||none|
|»»» version|string¦null|false|none||none|
|»»» metadata|object|false|none||none|
|»»»» stream|boolean|false|none||none|
|»»»» stream_options|object|false|none||none|
|»»»»» include_usage|boolean|false|none||none|
|»»» output|string|false|none||none|
|»»» usage|object|false|none||none|
|»»»» input|integer|false|none||none|
|»»»» output|integer|false|none||none|
|»»»» total|integer|false|none||none|
|»»»» unit|string|false|none||none|
|»»» level|string|false|none||none|
|»»» statusMessage|string¦null|false|none||none|
|»»» parentObservationId|string¦null|false|none||none|
|»»» promptId|string¦null|false|none||none|
|»»» promptName|string¦null|false|none||none|
|»»» promptVersion|string¦null|false|none||none|
|»»» modelId|string¦null|false|none||none|
|»»» inputPrice|string¦null|false|none||none|
|»»» outputPrice|string¦null|false|none||none|
|»»» totalPrice|string¦null|false|none||none|
|»»» calculatedInputCost|string¦null|false|none||none|
|»»» calculatedOutputCost|string¦null|false|none||none|
|»»» calculatedTotalCost|string¦null|false|none||none|
|»»» latency|number|false|none||none|
|»»» timeToFirstToken|number|false|none||none|
|»»» completionTokens|integer|false|none||none|
|»»» unit|string|false|none||none|
|»»» totalTokens|integer|false|none||none|
|»»» projectId|string|false|none||none|
|»»» createdAt|string|false|none||none|
|»»» promptTokens|integer|false|none||none|
|»»» updatedAt|string|false|none||none|
|»» scores|[string]|false|none||none|
|»» externalId|string¦null|false|none||none|
|»» bookmarked|boolean|false|none||none|
|»» projectId|string|false|none||none|
|»» createdAt|string|false|none||none|
|»» updatedAt|string|false|none||none|

# Xinference/模型仓库

## GET 获取模型列表

GET /v1/model_registrations/{model_type}

`response`中的`is_builtin`用于判断模型是否为注册模型。
`true`为内置模型，`false`为注册模型。

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_type|path|string| 是 |模型类型，LLM/embedding/rerank/image/multimodal|
|model_ability|query|string| 否 |模型选项-类型|
|context_length|query|integer| 否 |模型选项-上下文长度|
|model_lang|query|string| 否 |模型选项-语言|
|model_name|query|string| 否 |模型名称|
|detailed|query|boolean| 是 |none|
|curPageNum|query|integer| 是 |当前页数|
|numPerPage|query|integer| 是 |每页个数|
|is_builtin|query|boolean| 否 |是否是内置模型|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

#### 枚举值

|属性|值|
|---|---|
|model_ability|chat|
|model_ability|generate|
|model_ability|tools|
|model_ability|vision|

> 返回示例

> 200 Response

```json
{
  "count": 1,
  "results": [
    {
      "version": 1,
      "context_length": 4096,
      "model_name": "baichuan",
      "model_version_count": 5,
      "model_instance_count": 3,
      "model_lang": [
        "en",
        "zh"
      ],
      "model_ability": [
        "generate"
      ],
      "model_description": "Baichuan is an open-source Transformer based LLM that is trained on both Chinese and English data.",
      "model_specs": [
        {
          "model_format": "ggmlv3",
          "model_size_in_billions": 7,
          "quantizations": [
            "q2_K",
            "q3_K_L",
            "q3_K_M",
            "q3_K_S",
            "q4_0",
            "q4_1",
            "q4_K_M",
            "q4_K_S",
            "q5_0",
            "q5_1",
            "q5_K_M",
            "q5_K_S",
            "q6_K",
            "q8_0"
          ],
          "model_id": "TheBloke/baichuan-llama-7B-GGML",
          "model_file_name_template": "baichuan-llama-7b.ggmlv3.{quantization}.bin",
          "model_hub": "huggingface",
          "model_uri": null,
          "model_revision": null,
          "model_file_dir": "",
          "model_file_path": "/path/to/model-file",
          "cache_status": [
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false
          ]
        },
        {
          "model_format": "pytorch",
          "model_size_in_billions": 7,
          "quantizations": [
            "4-bit",
            "8-bit",
            "none"
          ],
          "model_id": "baichuan-inc/Baichuan-7B",
          "model_hub": "huggingface",
          "model_uri": null,
          "model_revision": "c1a5c7d5b7f50ecc51bb0e08150a9f12e5656756",
          "cache_status": false,
          "model_file_dir": "/path/to/model-dir",
          "model_file_path": ""
        },
        {
          "model_format": "pytorch",
          "model_size_in_billions": 13,
          "quantizations": [
            "4-bit",
            "8-bit",
            "none"
          ],
          "model_id": "baichuan-inc/Baichuan-13B-Base",
          "model_hub": "huggingface",
          "model_uri": null,
          "model_revision": "0ef0739c7bdd34df954003ef76d80f3dabca2ff9",
          "cache_status": false,
          "model_file_dir": "/path/to/model-dir",
          "model_file_path": ""
        }
      ],
      "prompt_style": null,
      "is_builtin": true
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» version|integer|true|none||none|
|» context_length|integer|true|none||模型支持的上下文最大长度|
|» model_name|string|true|none||模型名|
|» model_version_count|integer|true|none||模型版本总数|
|» model_instance_count|integer|true|none||运行实例总数|
|» model_lang|[string]|true|none||模型支持的语言|
|» model_ability|[string]|true|none||模型能力，可以是chat / generate，或两者都有|
|» model_description|string|true|none||模型说明|
|» model_specs|[object]|true|none||模型的各种规格|
|»» model_format|string|true|none||模型格式，可以是pytorch / ggmlv3 / ggufv2 / gptq|
|»» model_size_in_billions|integer|true|none||模型参数量大小，以十亿为单位|
|»» quantizations|[string]|true|none||模型的量化方案|
|»» model_id|string|false|none||模型在下载源中的id|
|»» model_file_name_template|string|false|none||模型文件名的f-string格式，仅当模型格式为ggmlv3 / ggufv2时有值|
|»» model_hub|string|false|none||模型下载源，可以是huggingface / modelscope|
|»» model_uri|string|false|none||模型文件uri，仅当这是自定义模型时有值|
|»» model_revision|string|false|none||模型在下载源中的版本号（git tag或是git commit id）|
|»» model_file_dir|string|true|none||存有模型文件的目录，当模型格式不为ggmlv3 / ggufv2时有值|
|»» model_file_path|string|true|none||模型文件路径，仅当模型格式为ggmlv3 / ggufv2时有值|
|»» cache_status|any|true|none||模型的缓存状态。|

*oneOf*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|»»» *anonymous*|boolean|false|none||当模型格式不为ggmlv3 / ggufv2时，cache_status为布尔值|

*xor*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|»»» *anonymous*|[boolean]|false|none||当模型格式为ggmlv3 / ggufv2时，cache_status为array[boolean]类型，array的长度与quantizations字段长度一致且一一对应，表示具体某个quantization对应的模型文件是否缓存|

*continued*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» prompt_style|object¦null|false|none||当模型能力含有chat时，模型的提示词样式，可以为null|
|» is_builtin|boolean|true|none||模型是否为内置模型|

## GET 获取模型部署版本

GET /v1/models/{model_type}/{model_name}/versions

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_type|path|string| 是 |模型类型 LLM / embedding / rerank / image|
|model_name|path|string| 是 |获取某个模型的所有部署版本|
|curPageNum|query|integer| 是 |当前页数|
|numPerPage|query|integer| 是 |每页个数|
|isCacheStatusSort|query|boolean| 否 |缓存标识，默认true, 筛选已经缓存的字段（cache_status）排序在前面|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 1,
  "results": [
    {
      "model_version": "qwen-chat-7B-ggufv2-Q4_K_M",
      "quantization": "Q4_K_M",
      "model_file_location": "/root/.cache/qwen-chat-7b.gguf",
      "model_format": "ggufv2",
      "model_size_in_billions": 7,
      "cache_status": true
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» model_version|string|true|none|版本|none|
|» quantization|string|true|none|量化选项|none|
|» model_file_location|string¦null|true|none|位置|当cache_status为true时有值，否则为null|
|» model_format|string|true|none|模型格式|none|
|» model_size_in_billions|number|true|none|参数量（十亿）|none|
|» cache_status|boolean|true|none|缓存状态|true: 已缓存，false: 未缓存|
|» model_engine|object|false|none|模型引擎|只有在llm下才会有|

## POST 创建运行实例

POST /v1/models/instance

> Body 请求参数

```json
{
  "model_uid": null,
  "model_type": "LLM",
  "model_version": "qwen1.5-chat--7B--gptq--Int4",
  "replica": 1,
  "n_gpu": "auto"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|
|body|body|object| 否 |none|
|» model_uid|body|string¦null| 是 |如果用户填了，以用户填入为准，否则可以填null让服务端生成（如果客户端生成，规则为<model_name>-<8位随机字符>）|
|» model_type|body|string| 是 |模型类型，LLM / embedding / image / rerank|
|» model_version|body|string| 是 |模型部署版本|
|» model_engine|body|string| 是 |引擎类型|
|» replica|body|integer| 否 |副本数|
|» n_gpu|body|any| 否 |模型要跑在几个GPU上|
|»» *anonymous*|body|string| 否 |可传入的string只能是auto|
|»» *anonymous*|body|integer| 否 |none|
|» gpu_idx|body|any| 是 |指定模型要跑在哪张卡上|
|»» *anonymous*|body|integer| 否 |none|
|»» *anonymous*|body|[integer]| 否 |none|
|» peft_model_config|body|object| 是 |none|
|»» lora_list|body|[object]| 是 |none|
|»»» lora_name|body|string| 是 |none|
|»»» local_path|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "model_uid": "<model_uid>"
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» model_uid|string|true|none||生成的model_uid|

## GET 模型引擎获取

GET /v1/engines/{model_name}

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_name|path|string| 是 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET 获取模型详情

GET /v1/model_registrations/{model_name}

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_name|path|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "data": {
    "version": 0,
    "context_length": 0,
    "model_name": "string",
    "model_lang": [
      "string"
    ],
    "model_ability": [
      "string"
    ],
    "model_description": "string",
    "model_family": null,
    "model_specs": [
      {
        "model_format": "string",
        "model_size_in_billions": 0,
        "quantizations": [
          "string"
        ],
        "model_id": "string",
        "model_hub": "string",
        "model_uri": null,
        "model_revision": "string",
        "cache_status": [
          true
        ],
        "model_file_name_template": "string",
        "model_file_name_split_template": "string",
        "quantization_parts": {
          "UD-IQ1_S": [
            null
          ],
          "UD-IQ1_M": [
            null
          ],
          "UD-IQ2_XXS": [
            null
          ],
          "UD-Q2_K_XL": [
            null
          ],
          "Q2_K": [
            null
          ],
          "Q2_K_L": [
            null
          ],
          "Q2_K_XS": [
            null
          ],
          "Q3_K_M": [
            null
          ],
          "Q4_K_M": [
            null
          ],
          "Q5_K_M": [
            null
          ],
          "Q6_K": [
            null
          ],
          "Q8_0": [
            null
          ],
          "BF16": [
            null
          ]
        }
      }
    ],
    "chat_template": "string",
    "stop_token_ids": [
      0
    ],
    "stop": [
      "string"
    ],
    "reasoning_start_tag": null,
    "reasoning_end_tag": null,
    "is_builtin": true,
    "model_version_count": 0,
    "model_instance_count": 0
  },
  "message": "string"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» data|object|true|none||none|
|»» version|integer|true|none||none|
|»» context_length|integer|true|none||none|
|»» model_name|string|true|none||none|
|»» model_lang|[string]|true|none||none|
|»» model_ability|[string]|true|none||none|
|»» model_description|string|true|none||none|
|»» model_family|null|true|none||none|
|»» model_specs|[object]|true|none||none|
|»»» model_format|string|true|none||none|
|»»» model_size_in_billions|integer|true|none||none|
|»»» quantizations|[string]|true|none||none|
|»»» model_id|string|true|none||none|
|»»» model_hub|string|true|none||none|
|»»» model_uri|null|true|none||none|
|»»» model_revision|string¦null|true|none||none|
|»»» cache_status|[boolean]|true|none||none|
|»»» model_file_name_template|string|false|none||none|
|»»» model_file_name_split_template|string|false|none||none|
|»»» quantization_parts|object|false|none||none|
|»»»» UD-IQ1_S|[string]|true|none||none|
|»»»» UD-IQ1_M|[string]|true|none||none|
|»»»» UD-IQ2_XXS|[string]|true|none||none|
|»»»» UD-Q2_K_XL|[string]|true|none||none|
|»»»» Q2_K|[string]|true|none||none|
|»»»» Q2_K_L|[string]|true|none||none|
|»»»» Q2_K_XS|[string]|true|none||none|
|»»»» Q3_K_M|[string]|true|none||none|
|»»»» Q4_K_M|[string]|true|none||none|
|»»»» Q5_K_M|[string]|true|none||none|
|»»»» Q6_K|[string]|true|none||none|
|»»»» Q8_0|[string]|true|none||none|
|»»»» BF16|[string]|true|none||none|
|»» chat_template|string|true|none||none|
|»» stop_token_ids|[integer]|true|none||none|
|»» stop|[string]|true|none||none|
|»» reasoning_start_tag|null|true|none||none|
|»» reasoning_end_tag|null|true|none||none|
|»» is_builtin|boolean|true|none||none|
|»» model_version_count|integer|true|none||none|
|»» model_instance_count|integer|true|none||none|
|» message|string|true|none||none|

## GET 获取模型列表 Copy

GET /v1/model_registrations/{model_type}/{model_name}

`response`中的`is_builtin`用于判断模型是否为注册模型。
`true`为内置模型，`false`为注册模型。

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_type|path|string| 是 |模型类型，LLM/embedding/rerank/image/multimodal|
|model_name|path|string| 是 |none|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 1,
  "results": [
    {
      "version": 1,
      "context_length": 4096,
      "model_name": "baichuan",
      "model_version_count": 5,
      "model_instance_count": 3,
      "model_lang": [
        "en",
        "zh"
      ],
      "model_ability": [
        "generate"
      ],
      "model_description": "Baichuan is an open-source Transformer based LLM that is trained on both Chinese and English data.",
      "model_specs": [
        {
          "model_format": "ggmlv3",
          "model_size_in_billions": 7,
          "quantizations": [
            "q2_K",
            "q3_K_L",
            "q3_K_M",
            "q3_K_S",
            "q4_0",
            "q4_1",
            "q4_K_M",
            "q4_K_S",
            "q5_0",
            "q5_1",
            "q5_K_M",
            "q5_K_S",
            "q6_K",
            "q8_0"
          ],
          "model_id": "TheBloke/baichuan-llama-7B-GGML",
          "model_file_name_template": "baichuan-llama-7b.ggmlv3.{quantization}.bin",
          "model_hub": "huggingface",
          "model_uri": null,
          "model_revision": null,
          "model_file_dir": "",
          "model_file_path": "/path/to/model-file",
          "cache_status": [
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false
          ]
        },
        {
          "model_format": "pytorch",
          "model_size_in_billions": 7,
          "quantizations": [
            "4-bit",
            "8-bit",
            "none"
          ],
          "model_id": "baichuan-inc/Baichuan-7B",
          "model_hub": "huggingface",
          "model_uri": null,
          "model_revision": "c1a5c7d5b7f50ecc51bb0e08150a9f12e5656756",
          "cache_status": false,
          "model_file_dir": "/path/to/model-dir",
          "model_file_path": ""
        },
        {
          "model_format": "pytorch",
          "model_size_in_billions": 13,
          "quantizations": [
            "4-bit",
            "8-bit",
            "none"
          ],
          "model_id": "baichuan-inc/Baichuan-13B-Base",
          "model_hub": "huggingface",
          "model_uri": null,
          "model_revision": "0ef0739c7bdd34df954003ef76d80f3dabca2ff9",
          "cache_status": false,
          "model_file_dir": "/path/to/model-dir",
          "model_file_path": ""
        }
      ],
      "prompt_style": null,
      "is_builtin": true
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» version|integer|true|none||none|
|» context_length|integer|true|none||模型支持的上下文最大长度|
|» model_name|string|true|none||模型名|
|» model_version_count|integer|true|none||模型版本总数|
|» model_instance_count|integer|true|none||运行实例总数|
|» model_lang|[string]|true|none||模型支持的语言|
|» model_ability|[string]|true|none||模型能力，可以是chat / generate，或两者都有|
|» model_description|string|true|none||模型说明|
|» model_specs|[object]|true|none||模型的各种规格|
|»» model_format|string|true|none||模型格式，可以是pytorch / ggmlv3 / ggufv2 / gptq|
|»» model_size_in_billions|integer|true|none||模型参数量大小，以十亿为单位|
|»» quantizations|[string]|true|none||模型的量化方案|
|»» model_id|string|false|none||模型在下载源中的id|
|»» model_file_name_template|string|false|none||模型文件名的f-string格式，仅当模型格式为ggmlv3 / ggufv2时有值|
|»» model_hub|string|false|none||模型下载源，可以是huggingface / modelscope|
|»» model_uri|string|false|none||模型文件uri，仅当这是自定义模型时有值|
|»» model_revision|string|false|none||模型在下载源中的版本号（git tag或是git commit id）|
|»» model_file_dir|string|true|none||存有模型文件的目录，当模型格式不为ggmlv3 / ggufv2时有值|
|»» model_file_path|string|true|none||模型文件路径，仅当模型格式为ggmlv3 / ggufv2时有值|
|»» cache_status|any|true|none||模型的缓存状态。|

*oneOf*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|»»» *anonymous*|boolean|false|none||当模型格式不为ggmlv3 / ggufv2时，cache_status为布尔值|

*xor*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|»»» *anonymous*|[boolean]|false|none||当模型格式为ggmlv3 / ggufv2时，cache_status为array[boolean]类型，array的长度与quantizations字段长度一致且一一对应，表示具体某个quantization对应的模型文件是否缓存|

*continued*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» prompt_style|object¦null|false|none||当模型能力含有chat时，模型的提示词样式，可以为null|
|» is_builtin|boolean|true|none||模型是否为内置模型|

# Xinference/模型注册

## POST 注册模型 - Image

POST /v1/model_registrations/image

> Body 请求参数

```json
{
  "model": "{\"model_family\": \"stable_diffusion\",\"model_uid\": \"my_sd\",\"model_name\": \"my_sd\",\"model_uri\": \"/Users/xprobe/.xinference/cache/stable-diffusion-v1.5\",\"controlnet\": [{\"model_family\": \"controlnet\",\"model_uid\": \"my_controlnet\",\"model_name\": \"my_controlnet\",\"model_uri\": \"/Users/xprobe/.xinference/cache/mlsd\",}]}",
  "persist": false
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» model|body|object| 是 |none|
|»» model_id|body|string| 否 |none|
|»» model_name|body|string| 是 |none|
|»» model_family|body|string| 是 |none|
|»» model_uri|body|string| 是 |none|
|»» controlnet|body|[object]| 否 |none|
|»»» model_id|body|string| 否 |none|
|»»» model_name|body|string| 是 |none|
|»»» model_family|body|string| 是 |none|
|»»» model_uri|body|string| 是 |none|
|» persist|body|boolean| 是 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## POST 注册模型 - Audio

POST /v1/model_registrations/audio

> Body 请求参数

```json
{
  "model": "{\"model_family\": \"whisper\",\"model_uid\": \"my_whisper\",\"model_name\": \"my_whisper\",\"model_uri\": \"/Users/lishulei/.xinference/cache/orca-ggmlv3-3b\",\"multilingual\": True}",
  "persist": true
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» model|body|object| 是 |none|
|»» model_name|body|string| 是 |none|
|»» model_family|body|string| 是 |none|
|»» model_id|body|string| 否 |none|
|»» multilingual|body|boolean| 是 |none|
|»» model_uri|body|string| 是 |none|
|» persist|body|boolean| 是 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## POST 注册模型 - Rerank

POST /v1/model_registrations/rerank

> Body 请求参数

```json
{
  "model": "{\"model_name\":\"custom-rerank\",\"language\":[\"en\"],\"model_uri\":\"/path/to/rerank-model\"}",
  "persist": true
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» model|body|object| 是 |none|
|»» model_name|body|string| 是 |none|
|»» language|body|[string]| 是 |none|
|»» model_id|body|string| 否 |none|
|»» model_uri|body|string| 是 |none|
|» persist|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## POST 注册模型 - Embedding

POST /v1/model_registrations/embedding

> Body 请求参数

```json
{
  "model": "{\"model_name\":\"custom-embedding\",\"dimensions\":768,\"max_tokens\":512,\"language\":[\"en\"],\"model_uri\":\"/path/to/embedding-model\"}",
  "persist": true
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» model|body|object| 是 |none|
|»» model_name|body|string| 是 |none|
|»» language|body|[string]| 是 |none|
|»» model_id|body|string| 否 |none|
|»» model_uri|body|string| 是 |none|
|» persist|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## POST 注册模型 - LLM

POST /v1/model_registrations/LLM

> Body 请求参数

```json
{
  "model": "{\"version\":1,\"context_length\":2048,\"model_name\":\"custom-llama-2\",\"model_lang\":[\"en\"],\"model_ability\":[\"generate\"],\"model_description\":\"This is a custom model description.\",\"model_family\":\"Yi-200k\",\"model_specs\":[{\"model_format\":\"pytorch\",\"model_size_in_billions\":7,\"quantizations\":[\"4-bit\",\"8-bit\",\"none\"],\"model_id\":\"\",\"model_uri\":\"/path/to/llama-2\"}]}",
  "persist": true
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|
|» model|body|object| 是 |none|
|»» version|body|integer| 是 |none|
|»» context_length|body|integer| 是 |none|
|»» model_name|body|string| 是 |none|
|»» model_lang|body|[string]| 是 |none|
|»» model_ability|body|[string]| 是 |none|
|»» model_description|body|string| 是 |none|
|»» model_family|body|string| 是 |none|
|»» model_specs|body|object| 是 |none|
|»»» model_format|body|string| 是 |none|
|»»» model_size_in_billions|body|string| 是 |none|
|»»» quantizations|body|[string]| 是 |none|
|»»» model_id|body|string| 是 |none|
|»»» model_uri|body|string| 是 |none|
|» persist|body|boolean| 是 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Xinference/运行实例

## GET 获取运行实例

GET /v1/models/instances

返回运行中的模型列表信息
返回的body的key是model_uid

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_name|query|string| 否 |根据内置模型名称（某一类）过滤运行实例，否则显示所有|
|model_uid|query|string| 否 |用户自定的模型名称|
|model_type|query|string| 否 |none|
|model_status|query|string| 否 |none|
|curPageNum|query|integer| 是 |当前页数|
|numPerPage|query|integer| 是 |每页个数|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 1,
  "results": [
    {
      "model_name": "qwen1.5-chat",
      "model_type": "LLM",
      "model_uid": "qwen",
      "model_engine": "Transformers",
      "model_version": "qwen1.5-chat--0_5B--pytorch--none",
      "model_ability": [
        "chat",
        "tools"
      ],
      "replica": 2,
      "status": "READY",
      "instance_created_ts": 1733386198,
      "n_gpu": "auto",
      "gpu_idx": [
        "0",
        "1"
      ],
      "peft_model_config": null,
      "is_builtin": false,
      "error_info": null,
      "replica_data_source": [
        {
          "replica_mode_uid": "qwen-0",
          "gpu_idx": [
            0
          ],
          "worker_address": "0.0.0.0:21557"
        },
        {
          "replica_mode_uid": "qwen-1",
          "gpu_idx": [
            1
          ],
          "worker_address": "0.0.0.0:21557"
        }
      ]
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» count|integer|true|none||none|
|» results|[object]|true|none||none|
|»» model_name|string|false|none||none|
|»» model_type|string|false|none||none|
|»» model_uid|string|false|none||none|
|»» model_engine|string|false|none||none|
|»» model_version|string|false|none||none|
|»» model_ability|[string]|false|none||none|
|»» replica|integer|false|none||none|
|»» status|string|false|none||none|
|»» instance_created_ts|integer|false|none||none|
|»» n_gpu|string|false|none||none|
|»» gpu_idx|[string]|false|none||none|
|»» peft_model_config|null|false|none||none|
|»» is_builtin|boolean|false|none||none|
|»» error_info|null|false|none||none|
|»» replica_data_source|[object]|false|none|副本信息|none|
|»»» replica_mode_uid|string|true|none||none|
|»»» gpu_idx|[integer]|true|none||none|
|»»» worker_address|string|true|none||none|

## PUT 编辑与修改运行实例

PUT /v1/models/instance

> Body 请求参数

```json
{
  "model_uid": "qwen-chat-xxxxxxxxx",
  "model_version": "qwen-chat-7B-pytorch-Int4",
  "replica": 1,
  "n_gpu": "auto"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|
|body|body|object| 否 |none|
|» model_uid|body|string| 是 |这是编辑与修改运行实例，一定要填model_uid|
|» model_version|body|string| 是 |模型部署版本|
|» replica|body|integer| 否 |副本数|
|» n_gpu|body|any| 否 |模型要跑在几个GPU上|
|»» *anonymous*|body|string| 否 |可传入的string只能是auto|
|»» *anonymous*|body|integer| 否 |none|
|» gpu_idx|body|any| 否 |指定模型跑在哪些GPU上|
|»» *anonymous*|body|integer| 否 |none|
|»» *anonymous*|body|[integer]| 否 |none|

> 返回示例

> 200 Response

```json
{
  "model_uid": "<model_uid>"
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» model_uid|string|true|none||生成的model_uid|

## DELETE 删除运行实例

DELETE /v1/models/{model_uid}

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_uid|path|string| 是 |none|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

## GET 获取运行实例事件

GET /v1/models/{model_uid}/events

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_uid|path|string| 是 |none|
|curPageNum|query|integer| 是 |当前页数|
|numPerPage|query|integer| 是 |每页个数|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 1,
  "results": [
    {
      "event_type": "Error",
      "event_ts": 1704176375,
      "event_content": "CUDA out of memory error: xxxx"
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» event_type|string|true|none|类型|none|
|» event_ts|integer|true|none|时间|none|
|» event_content|string|true|none|消息|none|

## POST 测试运行实例 - Generate

POST /v1/completions

Request body中，除了展示的temperature、max_tokens、top_k参数外，若还有其他参数，与这些参数并列填入json即可

> Body 请求参数

```json
{
  "model": "qwen-chat-aaaabbbb",
  "prompt": "Once upon a time...",
  "max_tokens": 1024,
  "temperature": 1,
  "top_k": 1
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|
|body|body|object| 否 |none|
|» model|body|string| 是 |实例名|
|» prompt|body|string| 是 |提示词|
|» max_tokens|body|integer| 否 |none|
|» temperature|body|integer| 否 |none|
|» top_k|body|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "id": "generate-7371ba6b-f424-4aa7-bd9c-29dc1eda3326",
  "model": "qwen-chat-aaaabbbb",
  "object": "text_completion",
  "created": 1703500579,
  "choices": [
    {
      "index": 0,
      "text": "! How can I help you today?",
      "finish_reason": null,
      "logprobs": null
    }
  ],
  "usage": {
    "prompt_tokens": -1,
    "completion_tokens": -1,
    "total_tokens": -1
  }
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» id|string|true|none||none|
|» model|string|true|none||none|
|» object|string|true|none||none|
|» created|integer|true|none||none|
|» choices|[object]|true|none||none|
|»» index|integer|true|none||none|
|»» text|string|true|none||模型返回的具体内容|
|»» finish_reason|string¦null|true|none||none|
|»» logprobs|string¦null|true|none||none|
|» usage|object|true|none||none|
|»» prompt_tokens|integer|true|none||none|
|»» completion_tokens|integer|true|none||none|
|»» total_tokens|integer|true|none||none|

## POST 测试运行实例 - Embedding

POST /v1/embeddings

> Body 请求参数

```json
{
  "model": "bge-small-zh",
  "input": "你好"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|
|body|body|object| 否 |none|
|» model|body|string| 是 |none|
|» input|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "object": "list",
  "model": "bge-small-zh",
  "data": [
    {
      "index": 0,
      "object": "embedding",
      "embedding": [
        -0.0643513947725296,
        -0.06281541287899017,
        0.05967331305146217,
        -0.06245768070220947,
        -0.013114606030285358,
        0.01824272610247135
      ]
    }
  ],
  "usage": {
    "prompt_tokens": 37,
    "total_tokens": 37
  }
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» object|string|true|none||none|
|» model|string|true|none||model_uid，实例名|
|» data|[object]|true|none||none|
|»» index|integer|false|none||none|
|»» object|string|false|none||none|
|»» embedding|[number]|true|none||embedding模型返回的embedding数组|
|» usage|object|true|none||none|
|»» prompt_tokens|integer|true|none||none|
|»» total_tokens|integer|true|none||none|

## POST 测试运行实例 - Rerank

POST /v1/rerank

> Body 请求参数

```json
{
  "model": "bge-rerank-base",
  "documents": [
    "Hello",
    "World"
  ],
  "query": "Hello"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|
|body|body|object| 否 |none|
|» model|body|string| 是 |实例名|
|» documents|body|[string]| 是 |corpus|
|» query|body|string| 是 |query|

> 返回示例

> 200 Response

```json
{
  "id": "529b93b0-a312-11ee-a8c0-047c1643e4f5",
  "results": [
    {
      "index": 0,
      "relevance_score": 0.626825749874115,
      "document": null
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» id|string|true|none||none|
|» results|[object]|true|none||表示每个document对于query的得分|
|»» index|integer|true|none||none|
|»» relevance_score|number|true|none||none|
|»» document|null|false|none||none|

## POST 测试运行实例 - Image

POST /v1/images/generations

Response body中的b64_json是字符串，将该字符串用base64 decode之后的bytes就是输出图片的bytes

> Body 请求参数

```json
{
  "model": "sd-aaaabbbb",
  "prompt": "Generate a picture that shows a basketball.",
  "n": 1,
  "size": "1024*1024",
  "response_format": "b64_json"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|
|body|body|object| 否 |none|
|» model|body|string| 是 |实例名|
|» prompt|body|string| 是 |none|
|» n|body|integer| 是 |none|
|» size|body|string| 是 |none|
|» response_format|body|string| 是 |none|

> 返回示例

> 200 Response

```json
{
  "created": 1704176375,
  "data": [
    {
      "b64_json": "xxxxxxxxxxxxxxxxxx"
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» created|integer|true|none||none|
|» data|[object]|true|none||none|
|»» b64_json|string|true|none||用base64 decode之后就是图片的bytes|

## POST 测试运行实例 - Chat

POST /v1/chat/completions

> Body 请求参数

```json
{
  "model": "qwen1.5-chat",
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant."
    },
    {
      "role": "user",
      "content": "Hello"
    }
  ],
  "max_tokens": 1024,
  "temperature": 1,
  "top_k": 1,
  "stream": true,
  "stream_option": {
    "include_usage": true
  }
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|
|body|body|object| 否 |none|
|» model|body|string| 是 |实例名|
|» messages|body|[object]| 否 |如果用户填写了system_prompt和chat history，请将信息组织成这种数据结构|
|»» role|body|string| 是 |system / user / assistant, 其中system只能有一个，content对应system_prompt输入框填入的值|
|»» content|body|string| 是 |none|
|» max_tokens|body|integer| 否 |none|
|» temperature|body|integer| 否 |none|
|» top_k|body|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "id": "chatdd5e2c58-a955-11ee-afe9-047c1643e4f5",
  "object": "chat.completion",
  "created": 1704189674,
  "model": "qwen-chat",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Hello! How can I help you today? Is there something on your mind that you would like to talk about or ask about? I'm here to listen and offer any assistance that I can."
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 20,
    "completion_tokens": 40,
    "total_tokens": 60
  }
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» id|string|true|none||none|
|» object|string|true|none||none|
|» created|integer|true|none||none|
|» model|string|true|none||none|
|» choices|[object]|true|none||none|
|»» index|integer|true|none||none|
|»» message|object|false|none||模型对话结果|
|»»» role|string|true|none||none|
|»»» content|string|true|none||none|
|»» finish_reason|string|true|none||none|
|» usage|object|true|none||none|
|»» prompt_tokens|integer|true|none||none|
|»» completion_tokens|integer|true|none||none|
|»» total_tokens|integer|true|none||none|

## GET 获取上次运行实例参数

GET /v1/instance/latest

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_name|query|string| 是 |none|
|model_type|query|string| 是 |none|
|model_uid|query|string| 否 |none|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "model_uid": "qwen1.5-chat-111",
    "model_type": "LLM",
    "model_version": "qwen1.5-chat--7B--gptq--Int4",
    "replica": 1,
    "n_gpu": "auto",
    "model_engine": "vLLM",
    "model_name": "qwen1.5-chat"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## POST 副本缩容

POST /v1/models/{model_uid}/replicas/scale_in

> Body 请求参数

```json
{
  "cnt": 1
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_uid|path|string| 是 |none|
|body|body|object| 否 |none|
|» cnt|body|integer| 是 |none|

> 返回示例

> 500 Response

```json
{
  "detail": "[address=192.168.1.16:50930, pid=4071197] The count of scale-in: 1 exceeds the current count of replicas: 1, call `terminate_model` directly."
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **500**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» detail|string|true|none||none|

## GET 获取实例详情

GET /v1/models/instances/{model_uid}

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_uid|path|string| 是 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## POST 副本扩容

POST /v1/models/{model_uid}/replicas/scale_out

> Body 请求参数

```json
{
  "cnt": 1
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|model_uid|path|string| 是 |none|
|body|body|object| 否 |none|
|» cnt|body|integer| 是 |none|
|» replica_config|body|[object]| 是 |none|
|»» replica_uid|body|string| 否 |none|
|»» devices|body|[object]| 否 |none|
|»»» worker_ip|body|string| 否 |none|
|»»» n_gpu|body|string| 否 |none|
|»»» gpu_idx|body|[string]| 否 |none|
|»»» model_path|body|string| 否 |none|

> 返回示例

```json
{
  "detail": "[address=192.168.1.16:50930, pid=4071197] The count of scale-in: 1 exceeds the current count of replicas: 1, call `terminate_model` directly."
}
```

```json
{
  "message": "Instances scaled out successfully",
  "model_uid": "qwen3",
  "cnt": 1
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» model_uid|string|true|none||none|
|» cnt|integer|true|none||none|

## POST cor

POST /v1/images/ocr

Response body中的b64_json是字符串，将该字符串用base64 decode之后的bytes就是输出图片的bytes

> Body 请求参数

```yaml
model: ""
image: ""

```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 |客户端IP|
|body|body|object| 否 |none|
|» model|body|string| 否 |none|
|» image|body|string(binary)| 否 |none|

> 返回示例

> 200 Response

```json
{
  "created": 1704176375,
  "data": [
    {
      "b64_json": "xxxxxxxxxxxxxxxxxx"
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» created|integer|true|none||none|
|» data|[object]|true|none||none|
|»» b64_json|string|true|none||用base64 decode之后就是图片的bytes|

# Xinference/微调任务

## POST 创建微调任务

POST /v1/tasks/tuning/{model_type}

 

> Body 请求参数

```json
{
  "task_name": "test_tuning",
  "tuning_args": {
    "training_stage": "sft",
    "finetuning_type": "lora",
    "use_llama_pro": false
  },
  "model_args": {
    "model_name": "qwen1.5-chat",
    "model_version": "qwen1.5-chat--0_5B--pytorch--none",
    "quantization_bit": "none",
    "rope_scaling": "none",
    "booster": "none",
    "visual_inputs": false,
    "resize_vocab": false,
    "upcast_layernorm": false,
    "shift_attn": false
  },
  "data_args": {
    "template": "qwen",
    "dataset": [
      "xprobe"
    ],
    "dataset_dir": "/home/xprobe/projects/LLaMA-Factory/data",
    "cutoff_len": 1024,
    "packing": false,
    "max_samples": 100000,
    "val_size": 0
  },
  "lora_args": {
    "lora_rank": 8,
    "lora_alpha": 16,
    "lora_dropout": 0,
    "lora_target": "all",
    "use_dora": false,
    "use_rslora": false,
    "create_new_adapter": false
  },
  "train_args": {
    "learning_rate": 0.00005,
    "num_train_epochs": 3,
    "per_device_train_batch_size": 2,
    "max_grad_norm": 1,
    "gradient_accumulation_steps": 8,
    "lr_scheduler_type": "cosine",
    "batch_size": 2,
    "compute_type": "fp16"
  },
  "output_args": {
    "logging_steps": 5,
    "save_steps": 100,
    "warmup_steps": 0,
    "optim": "adamw_torch"
  }
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|model_type|path|string| 是 ||none|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|object| 否 ||none|
|» task_name|body|string| 是 | 任务名称|none|
|» worker_ip|body|string| 否 | Worker IP|执行任务worker的ip地址|
|» tuning_args|body|object| 是 | 微调配置|none|
|»» finetuning_type|body|string| 是 | 微调方法|--目前仅支持lora|
|»» training_stage|body|string| 是 | 训练阶段|目前采用的训练方式。--目前仅支持sft|
|»» use_llama_pro|body|boolean| 是 | 使用 LLaMA Pro|仅训练块扩展后的参数。|
|» model_args|body|object| 是 | 微调模型配置|none|
|»» model_name|body|string| 是 | 模型名称|none|
|»» model_version|body|string| 否 | 模型版本|none|
|»» model_path|body|string| 否 | 模型路径|none|
|»» quantization_bit|body|string| 是 | 量化等级|启用 4/8 比特模型量化（QLoRA）|
|»» rope_scaling|body|string| 是 | RoPE 插值方法|none|
|»» booster|body|string| 是 | 加速方式|none|
|»» visual_inputs|body|boolean| 是 | 图像输入|none|
|»» resize_vocab|body|boolean| 是 | 更改词表大小|是否更改分词器词表和嵌入层的大小|
|»» upcast_layernorm|body|boolean| 是 | 缩放归一化层|是否将归一化层权重缩放至 32 位精度。|
|»» shift_attn|body|boolean| 是 | 使用 S^2 Attention|是否使用 LongLoRA 提出的 shift short attention。|
|» data_args|body|object| 是 | 数据集配置|none|
|»» template|body|string| 是 | 提示模板|构建提示词时使用的模板|
|»» dataset|body|[string]| 是 | 数据集|none|
|»» dataset_dir|body|string| 是 | 数据路径|数据文件夹的路径。|
|»» cutoff_len|body|integer| 是 | 截断长度|输入序列分词后的最大长度。|
|»» max_samples|body|integer| 是 | 最大样本数|每个数据集的最大样本数。|
|»» packing|body|boolean| 是 | 序列打包|是否将序列打包为等长样本|
|»» val_size|body|number| 是 | 验证集比例|验证集占全部样本的百分比。|
|» lora_args|body|object| 否 | LoRA配置|none|
|»» lora_alpha|body|integer| 是 | LoRA 缩放系数|LoRA 缩放系数大小|
|»» lora_dropout|body|number| 是 | LoRA 随机丢弃|LoRA 权重随机丢弃的概率。|
|»» lora_rank|body|integer| 是 | LoRA 秩|LoRA 矩阵的秩大小。|
|»» lora_target|body|string| 否 | LoRA 作用模块|应用 LoRA 的模块名称。使用英文逗号分隔多个名称。|
|»» loraplus_lr_ratio|body|integer¦null| 否 | LoRA+ 学习率比例|LoRA+ 中 B 矩阵的学习率倍数。|
|»» create_new_adapter|body|boolean| 是 | 新建适配器|在现有的适配器上创建一个随机初始化后的新适配器。|
|»» use_rslora|body|boolean| 是 | 使用 rslora|对 LoRA 层使用秩稳定缩放方法。|
|»» use_dora|body|boolean| 是 | 使用 DoRA|使用权重分解的 LoRA。|
|»» additional_target|body|string| 否 | 附加模块|除 LoRA 层以外的可训练模块名称。使用英文逗号分隔多个名称。|
|» galore_args|body|object| 否 | Galore配置|none|
|»» use_galore|body|boolean| 是 | 使用 GaLore|使用梯度低秩投影。|
|»» galore_rank|body|integer| 是 | GaLore 秩|GaLore 梯度的秩大小。|
|»» galore_update_interval|body|integer| 是 | 更新间隔|相邻两次投影更新的步数。|
|»» galore_target|body|string| 是 | GaLore 作用模块|应用 GaLore 的模块名称。使用英文逗号分隔多个名称。|
|»» galore_scale|body|number| 是 | GaLore 缩放系数|GaLore 缩放系数大小。|
|» badam_args|body|object| 否 | BAdam配置|none|
|»» use_badam|body|boolean| 是 | 使用 BAdam|使用 BAdam 优化器。|
|»» badam_mode|body|string| 是 | BAdam 模式|使用 layer-wise 或 ratio-wise BAdam 优化器。|
|»» badam_switch_mode|body|string| 是 | 切换策略|Layer-wise BAdam 优化器的块切换策略。|
|»» badam_switch_interval|body|integer| 是 | 切换频率|Layer-wise BAdam 优化器的块切换频率。|
|»» badam_update_ratio|body|number| 是 | Block 更新比例|Ratio-wise BAdam 优化器的更新比例。|
|» train_args|body|object| 否 | 训练配置|none|
|»» learning_rate|body|number| 是 | 学习率|AdamW 优化器的初始学习率。|
|»» num_train_epochs|body|number| 是 | 训练轮数|需要执行的训练总轮数。|
|»» batch_size|body|integer| 是 | 批处理大小|每个 GPU 处理的样本数量。|
|»» gradient_accumulation_steps|body|integer| 是 | 梯度累积|梯度累积的步数。|
|»» max_grad_norm|body|number| 是 | 最大梯度范数|用于梯度裁剪的范数。|
|»» lr_scheduler_type|body|string| 是 | 学习率调节器|学习率调度器的名称。|
|»» compute_type|body|string| 是 | 计算类型|是否使用混合精度训练|
|» output_args|body|object| 是 | 输出配置|none|
|»» logging_steps|body|integer| 是 | 日志间隔|每两次日志输出间的更新步数。|
|»» save_steps|body|integer| 是 | 保存间隔|每两次断点保存间的更新步数|
|»» warmup_steps|body|integer| 是 | 预热步数|学习率预热采用的步数。|
|»» neftune_alpha|body|number| 是 | NEFTune 噪声参数|嵌入向量所添加的噪声大小。|
|»» optim|body|string| 是 | 优化器|使用的优化器：adamw_torch、adamw_8bit 或 adafactor。|
|»» report_to|body|boolean| 是 | 启用外部记录面板|使用 TensorBoard 或 wandb 记录实验。|
|»» output_dir|body|string| 否 | 输出目录|保存结果的路径。|

#### 枚举值

|属性|值|
|---|---|
|model_type|LLM|
|model_type|embedding|
|model_type|rerank|
|model_type|image|
|model_type|audio|
|»» finetuning_type|lora|
|»» finetuning_type|freeze|
|»» finetuning_type|full|
|»» training_stage|pt|
|»» training_stage|sft|
|»» training_stage|rm|
|»» training_stage|ppo|
|»» training_stage|dpo|
|»» training_stage|kto|
|»» quantization_bit|none|
|»» quantization_bit|4|
|»» quantization_bit|8|
|»» rope_scaling|linear|
|»» rope_scaling|dynamic|
|»» rope_scaling|none|
|»» booster|flashattn2|
|»» booster|unsloth|
|»» booster|none|
|»» badam_mode|layer|
|»» badam_mode|ratio|
|»» badam_switch_mode|ascending|
|»» badam_switch_mode|descending|
|»» badam_switch_mode|random|
|»» badam_switch_mode|fixed|
|»» compute_type|fp16|
|»» compute_type|bf16|
|»» compute_type|fp32|
|»» compute_type|pure_bf16|
|»» optim|adamw_torch|
|»» optim|adamw_8bit|
|»» optim|adafactor|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful."
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|

## POST 运行微调任务

POST /v1/tasks/{task_id}/start

 

> Body 请求参数

```json
"string"
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|task_id|path|integer| 是 ||none|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful."
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|

## DELETE 删除微调任务

DELETE /v1/tasks/{task_id}

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|task_id|path|integer| 是 ||none|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful."
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|

## GET 获取任务详情

GET /v1/tasks/{task_id}

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|task_id|path|integer| 是 ||none|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|

> 返回示例

> 200 Response

```json
{
  "task_name": "qwen1.5-tuning",
  "model_type": "LLM",
  "status": "success",
  "create_time": 1717493169,
  "start_time": 1717493169,
  "finish_time": 1717493169,
  "execute_time": {
    "create": 20,
    "prepare": 50,
    "running": 100,
    "total": 170
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» task_id|integer|true|none||none|
|» task_name|string|true|none||none|
|» model_type|string|true|none||none|
|» status|string|true|none||none|
|» create_time|integer|true|none||none|
|» start_time|integer|true|none||none|
|» finish_time|integer|true|none||none|
|» 节点镜像|string|true|none||none|
|» 节点数量|string|true|none||none|
|» 节点配置|string|true|none||none|
|» execute_time|object|true|none||none|
|»» create|integer|true|none||none|
|»» prepare|integer|true|none||none|
|»» running|integer|true|none||none|
|»» total|integer|true|none||none|

## GET 获取数据集列表

GET /v1/tasks/tuning/dataset

 

> Body 请求参数

```json
"string"
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|training_stage|query|string| 是 ||目前采用的训练方式。--目前仅支持sft|
|dataset_dir|query|string| 是 ||数据文件夹的路径。|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|string| 否 ||none|

#### 枚举值

|属性|值|
|---|---|
|training_stage|pt|
|training_stage|sft|
|training_stage|rm|
|training_stage|ppo|
|training_stage|dpo|
|training_stage|kto|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": [
    "identity",
    "alpaca_en_demo",
    "alpaca_zh_demo",
    "glaive_toolcall_en_demo",
    "glaive_toolcall_zh_demo",
    "mllm_demo",
    "alpaca_en",
    "alpaca_zh",
    "alpaca_gpt4_en",
    "alpaca_gpt4_zh",
    "glaive_toolcall_en",
    "glaive_toolcall_zh",
    "lima",
    "guanaco",
    "belle_2m",
    "belle_1m",
    "belle_0.5m",
    "belle_dialog",
    "belle_math",
    "belle_multiturn",
    "ultra_chat",
    "open_platypus",
    "codealpaca",
    "alpaca_cot",
    "openorca",
    "slimorca",
    "mathinstruct",
    "firefly",
    "wikiqa",
    "webqa",
    "webnovel",
    "nectar_sft",
    "deepctrl",
    "adgen",
    "sharegpt_hyper",
    "sharegpt4",
    "ultrachat_200k",
    "agent_instruct",
    "lmsys_chat",
    "evol_instruct",
    "glaive_toolcall_100k",
    "cosmopedia",
    "stem_zh",
    "ruozhiba_gpt4",
    "neo_sft",
    "llava_1k_en",
    "llava_1k_zh",
    "llava_150k_en",
    "llava_150k_zh",
    "mllm_pt_demo",
    "oasst_de",
    "dolly_15k_de",
    "alpaca-gpt4_de",
    "openschnabeltier_de",
    "evol_instruct_de",
    "dolphin_de",
    "booksum_de",
    "airoboros_de",
    "ultrachat_de",
    "kto_en_demo",
    "kto_mix_en",
    "ultrafeedback_kto",
    "wiki_demo",
    "c4_demo",
    "refinedweb",
    "redpajama_v2",
    "wikipedia_en",
    "wikipedia_zh",
    "pile",
    "skypile",
    "fileweb",
    "fileweb_edu",
    "the_stack",
    "starcoder_python",
    "xprobe"
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|[string]|true|none||none|

## PUT 编辑与修改微调任务

PUT /v1/tasks/{task_id}/modify

 

> Body 请求参数

```json
{
  "task_name": "test_tuning",
  "tuning_args": {
    "training_stage": "sft",
    "finetuning_type": "lora",
    "use_llama_pro": false
  },
  "model_args": {
    "model_name": "qwen1.5-chat",
    "model_type": "LLM",
    "model_version": "qwen1.5-chat--0_5B--pytorch--none",
    "quantization_bit": "none",
    "rope_scaling": "none",
    "booster": "none",
    "visual_inputs": false,
    "resize_vocab": false,
    "upcast_layernorm": false,
    "shift_attn": false
  },
  "data_args": {
    "template": "qwen",
    "dataset": [
      "xprobe"
    ],
    "dataset_dir": "/home/xprobe/projects/LLaMA-Factory/data",
    "cutoff_len": 1024,
    "packing": false,
    "max_samples": 100000,
    "val_size": 0
  },
  "lora_args": {
    "lora_rank": 8,
    "lora_alpha": 16,
    "lora_dropout": 0,
    "lora_target": "all",
    "use_dora": false,
    "use_rslora": false,
    "create_new_adapter": false
  },
  "train_args": {
    "learning_rate": 0.00005,
    "num_train_epochs": 3,
    "per_device_train_batch_size": 2,
    "max_grad_norm": 1,
    "gradient_accumulation_steps": 8,
    "lr_scheduler_type": "cosine",
    "batch_size": 2,
    "compute_type": "fp16"
  },
  "output_args": {
    "logging_steps": 5,
    "save_steps": 100,
    "warmup_steps": 0,
    "optim": "adamw_torch"
  }
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|task_id|path|integer| 是 ||none|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|object| 否 ||none|
|» task_name|body|string| 是 | 任务名称|none|
|» worker_ip|body|string| 否 | Worker IP|执行任务worker的ip地址|
|» tuning_args|body|object| 是 | 微调配置|none|
|»» finetuning_type|body|string| 是 | 微调方法|--目前仅支持lora|
|»» training_stage|body|string| 是 | 训练阶段|目前采用的训练方式。--目前仅支持sft|
|»» use_llama_pro|body|boolean| 是 | 使用 LLaMA Pro|仅训练块扩展后的参数。|
|» model_args|body|object| 是 | 微调模型配置|none|
|»» model_name|body|string| 是 | 模型名称|none|
|»» model_type|body|string| 是 ||none|
|»» model_version|body|string| 否 | 模型版本|none|
|»» model_path|body|string| 否 | 模型路径|none|
|»» quantization_bit|body|string| 是 | 量化等级|启用 4/8 比特模型量化（QLoRA）|
|»» rope_scaling|body|string| 是 | RoPE 插值方法|none|
|»» booster|body|string| 是 | 加速方式|none|
|»» visual_inputs|body|boolean| 是 | 图像输入|none|
|»» resize_vocab|body|boolean| 是 | 更改词表大小|是否更改分词器词表和嵌入层的大小|
|»» upcast_layernorm|body|boolean| 是 | 缩放归一化层|是否将归一化层权重缩放至 32 位精度。|
|»» shift_attn|body|boolean| 是 | 使用 S^2 Attention|是否使用 LongLoRA 提出的 shift short attention。|
|» data_args|body|object| 是 | 数据集配置|none|
|»» template|body|string| 是 | 提示模板|构建提示词时使用的模板|
|»» dataset|body|[string]| 是 | 数据集|none|
|»» dataset_dir|body|string| 是 | 数据路径|数据文件夹的路径。|
|»» cutoff_len|body|integer| 是 | 截断长度|输入序列分词后的最大长度。|
|»» max_samples|body|integer| 是 | 最大样本数|每个数据集的最大样本数。|
|»» packing|body|boolean| 是 | 序列打包|是否将序列打包为等长样本|
|»» val_size|body|number| 是 | 验证集比例|验证集占全部样本的百分比。|
|» lora_args|body|object| 否 | LoRA配置|none|
|»» lora_alpha|body|integer| 是 | LoRA 缩放系数|LoRA 缩放系数大小|
|»» lora_dropout|body|number| 是 | LoRA 随机丢弃|LoRA 权重随机丢弃的概率。|
|»» lora_rank|body|integer| 是 | LoRA 秩|LoRA 矩阵的秩大小。|
|»» lora_target|body|string| 否 | LoRA 作用模块|应用 LoRA 的模块名称。使用英文逗号分隔多个名称。|
|»» loraplus_lr_ratio|body|integer¦null| 否 | LoRA+ 学习率比例|LoRA+ 中 B 矩阵的学习率倍数。|
|»» create_new_adapter|body|boolean| 是 | 新建适配器|在现有的适配器上创建一个随机初始化后的新适配器。|
|»» use_rslora|body|boolean| 是 | 使用 rslora|对 LoRA 层使用秩稳定缩放方法。|
|»» use_dora|body|boolean| 是 | 使用 DoRA|使用权重分解的 LoRA。|
|»» additional_target|body|string| 否 | 附加模块|除 LoRA 层以外的可训练模块名称。使用英文逗号分隔多个名称。|
|» galore_args|body|object| 否 | Galore配置|none|
|»» use_galore|body|boolean| 是 | 使用 GaLore|使用梯度低秩投影。|
|»» galore_rank|body|integer| 是 | GaLore 秩|GaLore 梯度的秩大小。|
|»» galore_update_interval|body|integer| 是 | 更新间隔|相邻两次投影更新的步数。|
|»» galore_target|body|string| 是 | GaLore 作用模块|应用 GaLore 的模块名称。使用英文逗号分隔多个名称。|
|»» galore_scale|body|number| 是 | GaLore 缩放系数|GaLore 缩放系数大小。|
|» badam_args|body|object| 否 | BAdam配置|none|
|»» use_badam|body|boolean| 是 | 使用 BAdam|使用 BAdam 优化器。|
|»» badam_mode|body|string| 是 | BAdam 模式|使用 layer-wise 或 ratio-wise BAdam 优化器。|
|»» badam_switch_mode|body|string| 是 | 切换策略|Layer-wise BAdam 优化器的块切换策略。|
|»» badam_switch_interval|body|integer| 是 | 切换频率|Layer-wise BAdam 优化器的块切换频率。|
|»» badam_update_ratio|body|number| 是 | Block 更新比例|Ratio-wise BAdam 优化器的更新比例。|
|» train_args|body|object| 否 | 训练配置|none|
|»» learning_rate|body|number| 是 | 学习率|AdamW 优化器的初始学习率。|
|»» num_train_epochs|body|number| 是 | 训练轮数|需要执行的训练总轮数。|
|»» batch_size|body|integer| 是 | 批处理大小|每个 GPU 处理的样本数量。|
|»» gradient_accumulation_steps|body|integer| 是 | 梯度累积|梯度累积的步数。|
|»» max_grad_norm|body|number| 是 | 最大梯度范数|用于梯度裁剪的范数。|
|»» lr_scheduler_type|body|string| 是 | 学习率调节器|学习率调度器的名称。|
|»» compute_type|body|string| 是 | 计算类型|是否使用混合精度训练|
|» output_args|body|object| 是 | 输出配置|none|
|»» logging_steps|body|integer| 是 | 日志间隔|每两次日志输出间的更新步数。|
|»» save_steps|body|integer| 是 | 保存间隔|每两次断点保存间的更新步数|
|»» warmup_steps|body|integer| 是 | 预热步数|学习率预热采用的步数。|
|»» neftune_alpha|body|number| 是 | NEFTune 噪声参数|嵌入向量所添加的噪声大小。|
|»» optim|body|string| 是 | 优化器|使用的优化器：adamw_torch、adamw_8bit 或 adafactor。|
|»» report_to|body|boolean| 是 | 启用外部记录面板|使用 TensorBoard 或 wandb 记录实验。|
|»» output_dir|body|string| 否 | 输出目录|保存结果的路径。|

#### 枚举值

|属性|值|
|---|---|
|»» finetuning_type|lora|
|»» finetuning_type|freeze|
|»» finetuning_type|full|
|»» training_stage|pt|
|»» training_stage|sft|
|»» training_stage|rm|
|»» training_stage|ppo|
|»» training_stage|dpo|
|»» training_stage|kto|
|»» model_type|LLM|
|»» model_type|audio|
|»» model_type|image|
|»» model_type|rerank|
|»» model_type|embedding|
|»» quantization_bit|none|
|»» quantization_bit|4|
|»» quantization_bit|8|
|»» rope_scaling|linear|
|»» rope_scaling|dynamic|
|»» rope_scaling|none|
|»» booster|flashattn2|
|»» booster|unsloth|
|»» booster|none|
|»» badam_mode|layer|
|»» badam_mode|ratio|
|»» badam_switch_mode|ascending|
|»» badam_switch_mode|descending|
|»» badam_switch_mode|random|
|»» badam_switch_mode|fixed|
|»» compute_type|fp16|
|»» compute_type|bf16|
|»» compute_type|fp32|
|»» compute_type|pure_bf16|
|»» optim|adamw_torch|
|»» optim|adamw_8bit|
|»» optim|adafactor|

> 返回示例

> 200 Response

```json
{}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

## GET 获取微调任务列表

GET /v1/tasks

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|task_name|query|string| 否 ||任务名称|
|curPageNum|query|integer| 否 ||当前页数|
|numPerPage|query|integer| 否 ||每页个数|
|model_type|query|string| 否 ||模型类型，|
|task_status|query|string| 否 ||状态：|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 2,
  "results": [
    {
      "task_name": "qwen1.5-tuning",
      "model_type": "LLM",
      "status": "success",
      "create_time": 1717493169,
      "start_time": 1717493169,
      "finish_time": 1717493169,
      "execute_time": 1717493169
    },
    {
      "task_name": "qwen1.5-tuning-2",
      "model_type": "LLM",
      "status": "running",
      "create_time": 1717493169,
      "start_time": 1717493169,
      "finish_time": 1717493169,
      "execute_time": 1717493169
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» count|integer|true|none||总数|
|» numPerPage|string|true|none||每页个数|
|» curPageNum|string|true|none||当前页数|
|» results|[object]|true|none||none|
|»» task_id|string|true|none||任务ID|
|»» task_name|string|true|none||任务名称|
|»» model_type|string|true|none||模型类型|
|»» model_version|string|true|none||模型版本|
|»» model_name|string|true|none||模型名称|
|»» status|string|true|none||状态<br />[<br />  {<br />    label: '待处理',<br />    value: 'pending',<br />    desc: '任务已创建，等待调度开始执行',<br />  },<br />  {<br />    label: '已调度',<br />    value: 'scheduled',<br />    desc: '任务已被调度，分配了资源，准备开始执行',<br />  },<br />  {<br />    label: '运行中',<br />    value: 'running',<br />    desc: '任务正在执行',<br />  },<br />  {<br />    label: '暂停',<br />    value: 'paused',<br />    desc: '任务被暂停，暂时停止执行，可以稍后恢复',<br />  },<br />  {<br />    label: '恢复中',<br />    value: 'resuming',<br />    desc: '任务从暂停状态恢复到运行状态',<br />  },<br />  {<br />    label: '已完成',<br />    value: 'completed',<br />    desc: '任务成功执行完毕',<br />  },<br />  {<br />    label: '失败',<br />    value: 'failed',<br />    desc: '任务执行失败，遇到错误或异常',<br />  },<br />  {<br />    label: '已取消',<br />    value: 'canceled',<br />    desc: '任务被取消，停止执行',<br />  },<br />  {<br />    label: '重试中',<br />    value: 'retrying',<br />    desc: '任务在失败后自动重试',<br />  },<br />  {<br />    label: '超时',<br />    value: 'timeout',<br />    desc: '任务执行超时，自动停止',<br />  },<br />]|
|»» create_time|integer|true|none||创建时间|
|»» start_time|integer|true|none||开始时间|
|»» finish_time|integer|true|none||启动时间|
|»» execute_time|integer|true|none||执行时长|
|»» is_builtin|boolean|true|none||是内置模型|

## GET 获取任务日志

GET /v1/tasks/{task_id}/log

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|task_id|path|integer| 是 ||none|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|

> 返回示例

> 200 Response

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Xinference/用户管理

## POST 登录

POST /v1/user/signin

> Body 请求参数

```json
{
  "username": "administrator",
  "password": "administrator",
  "token_expire_in_minutes": 1800
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|object| 否 ||none|
|» username|body|string| 是 ||用户名|
|» password|body|string| 是 ||密码|
|» token_expire_in_minutes|body|integer| 是 ||勾选一周内免登录，则值为10080，否则为null|

> 返回示例

> 200 Response

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJsaWNoZW5namllIiwic2NvcGVzIjpbImFkbWluIl0sImV4cCI6MTcwNDE3MTU5M30.ZVJS8CsOnYP8IC0SGDboKKkOkkeKIo_Kky6bJ6m3lj4",
  "token_type": "bearer",
  "expire_in_minutes": 30
}
```

> 401 Response

```json
{
  "detail": "Incorrect username or password"
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» access_token|string|true|none||token字符串|
|» token_type|string|true|none||目前只支持bearer|
|» expire_in_minutes|string|true|none||token过期时间，以分钟为单位。为null表示不过期|

## POST 修改密码

POST /v1/user/password

> Body 请求参数

```json
{
  "old_password": "password1",
  "new_password": "password2"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|object| 否 ||none|
|» old_password|body|string| 是 ||原密码|
|» new_password|body|string| 是 ||新密码|

> 返回示例

> 200 Response

```json
{}
```

> 401 Response

```json
{
  "detail": "Original password verification failed"
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

## POST 退出

POST /v1/user/logout

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|

> 返回示例

> 200 Response

```json
{}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

## GET 列举用户

GET /v1/users

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|curPageNum|query|string| 是 ||当前页数|
|numPerPage|query|string| 是 ||每页个数|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 50,
  "results": [
    {
      "account": "test@xprobe.io",
      "username": "iron man",
      "role": "开发者",
      "email": "test@xprobe.io",
      "status": "enabled",
      "last_login_ts": 1704176375
    },
    {
      "account": "test2@xprobe.io",
      "username": "hulk",
      "role": "管理员",
      "email": "test2@xprobe.io",
      "status": "disabled",
      "last_login_ts": 1704176375
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» account|string|true|none|账号|none|
|» username|string|true|none|名称|none|
|» role|string|true|none|角色|none|
|» email|string|true|none|邮箱|none|
|» status|string|true|none|账号状态|none|
|» last_login_ts|integer|true|none|最近登录时间|none|

#### 枚举值

|属性|值|
|---|---|
|status|enabled|
|status|disabled|

## POST 创建用户

POST /v1/users

> Body 请求参数

```json
{
  "username": "captain america",
  "email": "test3@xprobe.io",
  "password": "abc.123",
  "role": "管理员"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|object| 否 ||none|
|» username|body|string| 是 | 名称|none|
|» email|body|string| 是 | 邮箱|none|
|» password|body|string| 是 | 密码|none|
|» role|body|string| 是 | 角色|none|

> 返回示例

> 200 Response

```json
{}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

## PUT 修改用户

PUT /v1/users

> Body 请求参数

```json
{
  "account": "test@xprobe.io",
  "username": "iron man",
  "status": "disabled",
  "role": "管理员"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|object| 否 ||none|
|» account|body|string| 是 | 账号|none|
|» username|body|string| 是 | 名称|none|
|» status|body|string| 是 | 状态|none|
|» role|body|string| 是 | 角色|none|

> 返回示例

> 200 Response

```json
{}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

# Xinference/角色管理

## GET 列举角色

GET /v1/roles

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|curPageNum|query|string| 是 ||当前页数|
|numPerPage|query|string| 是 ||每页个数|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|

> 返回示例

```json
{
  "count": 10,
  "results": [
    {
      "role": "开发者",
      "update_ts": 1704176375,
      "permissions": {
        "models": [
          "list",
          "read",
          "register",
          "unregister"
        ],
        "instances": [
          "list",
          "read",
          "start",
          "stop"
        ],
        "users": [
          "add",
          "modify",
          "list",
          "delete"
        ],
        "roles": [
          "list",
          "modify",
          "delete"
        ],
        "secrets": [
          "add",
          "list",
          "delete"
        ]
      }
    }
  ]
}
```

```json
{
  "code": 0,
  "message": "Request successful.",
  "data": {
    "count": 2,
    "results": [
      {
        "role": "user",
        "permissions": {
          "page": {
            "dashboard": [],
            "models": [
              "repository",
              "register",
              "instances"
            ],
            "tasks": [
              "finetune",
              "batch"
            ],
            "monitor": [
              "platform",
              "model_usage",
              "device_info",
              "traces",
              "logs"
            ],
            "admin": [
              "users",
              "roles",
              "secret_key"
            ]
          },
          "action": {
            "models": [
              "list",
              "read",
              "register",
              "unregister",
              "add"
            ],
            "instances": [
              "list",
              "read",
              "start",
              "stop"
            ],
            "users": [
              "add",
              "modify",
              "list",
              "delete"
            ],
            "roles": [
              "add",
              "modify",
              "list",
              "delete"
            ],
            "secrets": [
              "add",
              "list",
              "delete"
            ],
            "tasks": [
              "add",
              "read",
              "list",
              "start",
              "modify",
              "delete",
              "cancel"
            ],
            "caches": [
              "list",
              "delete"
            ]
          }
        },
        "update_ts": 1768204127
      },
      {
        "role": "测试角色1",
        "permissions": {
          "page": {
            "dashboard": [],
            "models": [
              "repository",
              "register",
              "instances"
            ],
            "tasks": [
              "finetune",
              "batch"
            ],
            "monitor": [
              "platform",
              "model_usage",
              "device_info",
              "traces",
              "logs"
            ],
            "admin": [
              "users",
              "roles",
              "secret_key"
            ]
          },
          "action": {
            "models": [
              "list",
              "read",
              "register",
              "unregister",
              "add"
            ],
            "instances": [
              "list",
              "read",
              "start",
              "stop"
            ],
            "users": [
              "add",
              "modify",
              "list",
              "delete"
            ],
            "roles": [
              "add",
              "modify",
              "list",
              "delete"
            ],
            "secrets": [
              "add",
              "list",
              "delete"
            ],
            "tasks": [
              "add",
              "read",
              "list",
              "start",
              "modify",
              "delete",
              "cancel"
            ],
            "caches": [
              "list",
              "delete"
            ]
          }
        },
        "update_ts": 1768204832
      }
    ]
  }
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» count|integer|true|none||none|
|»» results|[object]|true|none||none|
|»»» role|string|true|none||none|
|»»» permissions|object|true|none||none|
|»»»» page|object|true|none||none|
|»»»»» dashboard|[string]|true|none||none|
|»»»»» models|[string]|true|none||none|
|»»»»» tasks|[string]|true|none||none|
|»»»»» monitor|[string]|true|none||none|
|»»»»» admin|[string]|true|none||none|
|»»»» action|object|true|none||none|
|»»»»» models|[string]|true|none||none|
|»»»»» instances|[string]|true|none||none|
|»»»»» users|[string]|true|none||none|
|»»»»» roles|[string]|true|none||none|
|»»»»» secrets|[string]|true|none||none|
|»»»»» tasks|[string]|true|none||none|
|»»»»» caches|[string]|true|none||none|
|»»» update_ts|integer|true|none||none|

## POST 创建角色

POST /v1/roles

> Body 请求参数

```json
{
  "role": "测试角色12",
  "permissions": {
    "page": {
      "dashboard": [],
      "models": [
        "repository",
        "register",
        "instances"
      ],
      "tasks": [
        "finetune",
        "batch"
      ],
      "monitor": [
        "platform",
        "model_usage",
        "device_info",
        "traces",
        "logs"
      ],
      "admin": [
        "users",
        "roles",
        "secret_key"
      ]
    },
    "action": {
      "models": [
        "list",
        "read",
        "register",
        "unregister",
        "add"
      ],
      "instances": [
        "list",
        "read",
        "start",
        "stop"
      ],
      "users": [
        "add",
        "modify",
        "list",
        "delete"
      ],
      "roles": [
        "add",
        "modify",
        "list",
        "delete"
      ],
      "secrets": [
        "add",
        "list",
        "delete"
      ],
      "tasks": [
        "add",
        "read",
        "list",
        "start",
        "modify",
        "delete",
        "cancel"
      ],
      "caches": [
        "list",
        "delete"
      ]
    }
  }
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|object| 否 ||none|
|» role|body|string| 是 | 角色名|none|
|» permissions|body|object| 是 | 权限|none|
|»» page|body|object| 是 ||none|
|»»» dashboard|body|[string]| 是 ||none|
|»»» models|body|[string]| 是 ||none|
|»»» tasks|body|[string]| 是 ||none|
|»»» monitor|body|[string]| 是 ||none|
|»»» admin|body|[string]| 是 ||none|
|»» action|body|object| 是 ||none|
|»»» models|body|[string]| 是 ||none|
|»»» instances|body|[string]| 是 ||none|
|»»» users|body|[string]| 是 ||none|
|»»» roles|body|[string]| 是 ||none|
|»»» secrets|body|[string]| 是 ||none|
|»»» tasks|body|[string]| 是 ||none|
|»»» caches|body|[string]| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "Request successful.",
  "data": {
    "role": "测试角色12",
    "permissions": {
      "page": {
        "dashboard": [],
        "models": [
          "repository",
          "register",
          "instances"
        ],
        "tasks": [
          "finetune",
          "batch"
        ],
        "monitor": [
          "platform",
          "model_usage",
          "device_info",
          "traces",
          "logs"
        ],
        "admin": [
          "users",
          "roles",
          "secret_key"
        ]
      },
      "action": {
        "models": [
          "list",
          "read",
          "register",
          "unregister",
          "add"
        ],
        "instances": [
          "list",
          "read",
          "start",
          "stop"
        ],
        "users": [
          "add",
          "modify",
          "list",
          "delete"
        ],
        "roles": [
          "add",
          "modify",
          "list",
          "delete"
        ],
        "secrets": [
          "add",
          "list",
          "delete"
        ],
        "tasks": [
          "add",
          "read",
          "list",
          "start",
          "modify",
          "delete",
          "cancel"
        ],
        "caches": [
          "list",
          "delete"
        ]
      }
    },
    "update_ts": 1768205810
  }
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» role|string|true|none||none|
|»» permissions|object|true|none||none|
|»»» page|object|true|none||none|
|»»»» dashboard|[string]|true|none||none|
|»»»» models|[string]|true|none||none|
|»»»» tasks|[string]|true|none||none|
|»»»» monitor|[string]|true|none||none|
|»»»» admin|[string]|true|none||none|
|»»» action|object|true|none||none|
|»»»» models|[string]|true|none||none|
|»»»» instances|[string]|true|none||none|
|»»»» users|[string]|true|none||none|
|»»»» roles|[string]|true|none||none|
|»»»» secrets|[string]|true|none||none|
|»»»» tasks|[string]|true|none||none|
|»»»» caches|[string]|true|none||none|
|»» update_ts|integer|true|none||none|

## PUT 修改角色

PUT /v1/roles

> Body 请求参数

```json
{
  "role": "user",
  "permissions": {
    "page": {
      "dashboard": [],
      "models": [
        "repository",
        "register",
        "instances"
      ],
      "tasks": [
        "finetune",
        "batch"
      ],
      "monitor": [
        "platform",
        "model_usage",
        "device_info",
        "traces",
        "logs"
      ],
      "admin": [
        "users",
        "roles",
        "secret_key"
      ]
    },
    "action": {
      "models": [
        "list",
        "read",
        "register",
        "unregister",
        "add"
      ],
      "instances": [
        "list",
        "read",
        "start",
        "stop"
      ],
      "users": [
        "add",
        "modify",
        "list",
        "delete"
      ],
      "roles": [
        "add",
        "modify",
        "list",
        "delete"
      ],
      "secrets": [
        "add",
        "list",
        "delete"
      ],
      "tasks": [
        "add",
        "read",
        "list",
        "start",
        "modify",
        "delete",
        "cancel"
      ],
      "caches": [
        "list",
        "delete"
      ]
    }
  }
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|object| 否 ||none|
|» role|body|string| 是 | 角色名|none|
|» permissions|body|object| 是 | 权限|none|
|»» page|body|object| 是 ||none|
|»»» dashboard|body|[string]| 是 ||none|
|»»» models|body|[string]| 是 ||none|
|»»» tasks|body|[string]| 是 ||none|
|»»» monitor|body|[string]| 是 ||none|
|»»» admin|body|[string]| 是 ||none|
|»» action|body|object| 是 ||none|
|»»» models|body|[string]| 是 ||none|
|»»» instances|body|[string]| 是 ||none|
|»»» users|body|[string]| 是 ||none|
|»»» roles|body|[string]| 是 ||none|
|»»» secrets|body|[string]| 是 ||none|
|»»» tasks|body|[string]| 是 ||none|
|»»» caches|body|[string]| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "Request successful.",
  "data": {}
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|

## GET 列举所有权限

GET /v1/roles/permissions

用于前端知晓所有模块对应的权限有哪些

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|

> 返回示例

```json
{
  "models": [
    "list",
    "read",
    "register",
    "unregister"
  ],
  "instances": [
    "list",
    "read",
    "start",
    "stop"
  ],
  "users": [
    "add",
    "modify",
    "list",
    "delete"
  ],
  "roles": [
    "add",
    "list",
    "modify",
    "delete"
  ],
  "secrets": [
    "add",
    "list",
    "delete"
  ]
}
```

```json
{
  "code": 0,
  "message": "Request successful.",
  "data": {
    "page": {
      "dashboard": [],
      "models": [
        "repository",
        "register",
        "instances"
      ],
      "tasks": [
        "finetune",
        "batch"
      ],
      "monitor": [
        "platform",
        "model_usage",
        "device_info",
        "traces",
        "logs"
      ],
      "admin": [
        "users",
        "roles",
        "secret_key"
      ]
    },
    "action": {
      "models": [
        "list",
        "read",
        "register",
        "unregister",
        "add"
      ],
      "instances": [
        "list",
        "read",
        "start",
        "stop"
      ],
      "users": [
        "add",
        "modify",
        "list",
        "delete"
      ],
      "roles": [
        "add",
        "modify",
        "list",
        "delete"
      ],
      "secrets": [
        "add",
        "list",
        "delete"
      ],
      "tasks": [
        "add",
        "read",
        "list",
        "start",
        "modify",
        "delete",
        "cancel"
      ],
      "caches": [
        "list",
        "delete"
      ]
    }
  }
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» page|object|true|none||none|
|»»» dashboard|[string]|true|none||none|
|»»» models|[string]|true|none||none|
|»»» tasks|[string]|true|none||none|
|»»» monitor|[string]|true|none||none|
|»»» admin|[string]|true|none||none|
|»» action|object|true|none||none|
|»»» models|[string]|true|none||none|
|»»» instances|[string]|true|none||none|
|»»» users|[string]|true|none||none|
|»»» roles|[string]|true|none||none|
|»»» secrets|[string]|true|none||none|
|»»» tasks|[string]|true|none||none|
|»»» caches|[string]|true|none||none|

# Xinference/密钥管理

## GET 列举密钥

GET /v1/secrets

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|curPageNum|query|string| 是 ||当前页数|
|numPerPage|query|string| 是 ||每页个数|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 20,
  "results": [
    {
      "name": "test",
      "secrets": "app-123abcxxx",
      "created_ts": 1704176375
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» name|string|true|none|密钥名称|none|
|» secrets|string|true|none|密钥|none|
|» created_ts|integer|true|none|创建时间|none|

## DELETE 删除密钥

DELETE /v1/secrets

> Body 请求参数

```json
{
  "name": "test",
  "secrets": "app-xxxxxxxxx"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|
|body|body|object| 否 ||none|
|» name|body|string| 是 | 密钥名称|none|
|» secrets|body|string| 是 | 密钥|none|

> 返回示例

> 200 Response

```json
{}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

## POST 创建密钥

POST /v1/secrets

服务端生成

> Body 请求参数

```json
{
  "name": "test"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|XINFERENCE-CLIENT-IP|query|string| 是 ||客户端IP|
|body|body|object| 否 ||none|
|» name|body|string| 是 | 密钥名称|none|

> 返回示例

> 200 Response

```json
{
  "name": "test",
  "secrets": "app-xxxxxxxxxxxx",
  "created_ts": 1704176375
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» name|string|true|none|密钥名称|none|
|» secrets|string|true|none|密钥|none|
|» created_ts|integer|true|none|创建时间|none|

# Xinference/日志管理

## GET 操作日志

GET /v1/user/operations

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|curPageNum|query|string| 是 ||当前页数|
|numPerPage|query|string| 是 ||每页个数|
|XINFERENCE-CLIENT-IP|header|string| 是 ||客户端IP|

> 返回示例

> 200 Response

```json
{
  "count": 2,
  "results": [
    {
      "resourceId": "test@xprobe.io",
      "module": "users",
      "opType": "list",
      "operator": "admin@xprobe.io",
      "opTime": 1704176375,
      "ipAddress": "101.228.187.162"
    },
    {
      "resourceId": "qwen-chat",
      "module": "models",
      "opType": "register",
      "operator": "admin@xprobe.io",
      "opTime": 1704176375,
      "ipAddress": "101.228.187.162"
    }
  ]
}
```

> 500 Response

```json
{
  "detail": "<any other messages>"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» count|integer|true|none||总数|
|» results|[object]|true|none||none|
|»» resourceId|string|true|none||资源ID，被操作的资源名称。根据功能模块和操作类型，值是不同的。例如：- 功能模块是 用户管理 且 操作类型是 修改，此字段是用户账号（邮箱）- 功能模块是 用户管理 且 操作类型是 列表，此字段是null|
|»» module|string|true|none||功能模块。中英文对应如下：<br />- users：用户管理<br />- models：模型管理<br />- instances：实例管理<br />- secrets：密钥管理<br />- roles：角色管理|
|»» opType|string|true|none||操作类型。中英文对应如下：<br />- read：查看<br />- list：列表<br />- start：启动<br />- stop：停止<br />- register：注册<br />- unregister：取消注册<br />- add：创建<br />- modify：修改<br />- delete：删除<br />- signin: 登入<br />- logout: 登出|
|»» operator|string|true|none||操作人|
|»» opTime|integer|true|none||操作时间，时间戳，精确到秒|
|»» ipAddress|string|true|none||IP地址，操作执行时的客户端IP|

## GET API日志信息

GET /api/logs

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Xinference/证书注册

## GET 获取证书信息

GET /v1/license

> 返回示例

```json
{
  "code": 0,
  "message": "Request successful.",
  "data": {
    "mac_address": "BC:24:11:FE:6C:28",
    "license_count": 2,
    "licenses": [
      {
        "license_key": "TcLwIqXugOyFPaGL5TAByN0rp8HgIpWj_OKgNruYPYElnUBlxzC4JTLhMNCbl6r14jRkPIje4YNrB31LX7Cr-Q==",
        "expire_time": "2025-09-22 07:22:27",
        "auth_type": null,
        "auth_count": null
      },
      {
        "license_key": "vrBOf6WOEiRxejCn1p7IXiOOww0GKgmr2wtZPzKueHQ9FjfxnzSpTt2E1uppM-rV0z1lqjVZPGNyenBnKXV0X8ex6oz5KNag3AuXrcGIuQc=",
        "expire_time": "Expired",
        "auth_type": "worker",
        "auth_count": 10
      }
    ]
  }
}
```

```json
{
  "code": 0,
  "message": "Request successful.",
  "data": {
    "mac_address": "BC:24:11:FE:6C:28",
    "license_count": 4,
    "licenses": [
      {
        "license_key": "Y0UaXRS4M_ffSOZRO6unS6PUfYh5hi8sdt2-4L9NDfbtTB1xaO8h54OT255HbO39slHI6-FvgzE7uCLLHsv5gG2BZI0fV8VIznSmvoc1QpE=",
        "expire_time": "2970-01-21 07:44:14",
        "auth_type": "gpu",
        "auth_count": 1,
        "is_expired": false
      },
      {
        "license_key": "LbZ5wwHrzc6lmipGlfu7913EZIaScohcwmyzK49xIcPP67MitzUY3x3sQ3i6zVxkRkn-AuXiTOtBlwkQDr7gAl9SuClxipBrtq7M7ig6wMA=",
        "expire_time": "2970-01-21 07:44:14",
        "auth_type": "worker",
        "auth_count": 10,
        "is_expired": false
      },
      {
        "license_key": "ze8iVihs09vw5MfGUNjMGBWNWa0HYDMXyctwJvrT61zhOJOiJCFS2mTnv2xvdKTqLEGAHljXR4Bd50zFtWTArBi2Bisygu5x-zJ0bu64g4s=",
        "expire_time": "2070-01-02 07:58:24",
        "auth_type": "gpu",
        "auth_count": 1,
        "is_expired": false
      },
      {
        "license_key": "IICwrkHldjXvNNNfhP4SA_P0MO5zTLCu858JgNLLolYEr1uyPJlqZnpXJkPzjnKvr2gSZg9oWh6K6z6MxJF8PNa6J0FlbZnq8ZXWUNjZ9Ew=",
        "expire_time": "2025-08-28 18:13:00",
        "auth_type": "gpu",
        "auth_count": 1,
        "is_expired": true
      }
    ]
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» mac_address|string|true|none||none|
|»» license_count|integer|true|none||none|
|»» licenses|[object]|true|none||none|
|»»» license_key|string|true|none||none|
|»»» expire_time|string|true|none||none|
|»»» auth_type|string|true|none||none|
|»»» auth_count|integer|true|none||none|
|»»» is_expired|boolean|true|none||none|

## POST 设置证书

POST /v1/license

> Body 请求参数

```json
{
  "license_key": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|any| 否 ||none|

> 返回示例

```json
{
  "code": 0,
  "message": "Request successful.",
  "data": {
    "valid_licenses": [
      "LbZ5wwHrzc6lmipGlfu7913EZIaScohcwmyzK49xIcPP67MitzUY3x3sQ3i6zVxkRkn-AuXiTOtBlwkQDr7gAl9SuClxipBrtq7M7ig6wMA="
    ],
    "invalid_licenses": []
  }
}
```

```json
{
  "code": 0,
  "message": "Request successful.",
  "data": {
    "valid_licenses": [],
    "invalid_licenses": [
      {
        "license_key": "IICwrkHldjXvNNNfhP4SA_P0MO5zTLCu858JgNLLolYEr1uyPJlqZnpXJkPzjnKvr2gSZg9oWh6K6z6MxJF8PNa6J0FlbZnq8ZXWUNjZ9Ew=",
        "prompt": "License is expired."
      }
    ]
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» valid_licenses|[string]|true|none||none|
|»» invalid_licenses|[object]|true|none||none|
|»»» license_key|string|false|none||none|
|»»» prompt|string|false|none||none|

# Xinference/语言设置

## GET 获取系统全局配置

GET /v1/setting

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "locale": "zh-CN"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» locale|string|true|none||none|

## PUT 设置系统全局配置

PUT /v1/setting

> Body 请求参数

```json
{
  "locale": "zh-CN"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» locale|body|string| 是 ||none|

#### 枚举值

|属性|值|
|---|---|
|» locale|zh-CN|
|» locale|en-US|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "locale": "zh-CN"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» locale|string|true|none||none|

## GET 获取用户配置

GET /v1/user/me

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "Request successful.",
  "data": {
    "username": "administrator",
    "locale": "zh-CN",
    "permissions": {
      "page": {
        "dashboard": [],
        "models": [
          "repository",
          "register",
          "instances"
        ],
        "tasks": [
          "finetune",
          "batch"
        ],
        "monitor": [
          "platform",
          "model_usage",
          "device_info",
          "traces",
          "logs"
        ],
        "admin": [
          "users",
          "roles",
          "secret_key"
        ]
      },
      "action": {
        "models": [
          "list",
          "read",
          "register",
          "unregister",
          "add"
        ],
        "instances": [
          "list",
          "read",
          "start",
          "stop"
        ],
        "users": [
          "add",
          "modify",
          "list",
          "delete"
        ],
        "roles": [
          "add",
          "modify",
          "list",
          "delete"
        ],
        "secrets": [
          "add",
          "list",
          "delete"
        ],
        "tasks": [
          "add",
          "read",
          "list",
          "start",
          "modify",
          "delete",
          "cancel"
        ],
        "caches": [
          "list",
          "delete"
        ]
      }
    }
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» username|string|true|none||none|
|»» locale|string|true|none||none|
|»» permissions|object|true|none||none|
|»»» page|object|true|none||none|
|»»»» dashboard|[string]|true|none||none|
|»»»» models|[string]|true|none||none|
|»»»» tasks|[string]|true|none||none|
|»»»» monitor|[string]|true|none||none|
|»»»» admin|[string]|true|none||none|
|»»» action|object|true|none||none|
|»»»» models|[string]|true|none||none|
|»»»» instances|[string]|true|none||none|
|»»»» users|[string]|true|none||none|
|»»»» roles|[string]|true|none||none|
|»»»» secrets|[string]|true|none||none|
|»»»» tasks|[string]|true|none||none|
|»»»» caches|[string]|true|none||none|

## PUT 设置用户配置

PUT /v1/user/me

> Body 请求参数

```json
{
  "locale": "zh-CN"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» locale|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "locale": "string"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» locale|string|true|none||none|

# Xinference/批处理

## POST 创建批次

POST /v1/batches

> Body 请求参数

```yaml
file: file:///Users/arthur/Downloads/xinference.log
endpoint: ""
completion_window: ""

```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» file|body|string(binary)| 否 ||none|
|» endpoint|body|string| 否 ||none|
|» completion_window|body|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
  "id": "batch_abc123",
  "object": "batch",
  "endpoint": "/v1/chat/completions",
  "errors": null,
  "input_file_id": "file-abc123",
  "completion_window": "24h",
  "status": "validating",
  "output_file_id": null,
  "error_file_id": null,
  "created_at": 1711471533,
  "in_progress_at": null,
  "expires_at": null,
  "finalizing_at": null,
  "completed_at": null,
  "failed_at": null,
  "expired_at": null,
  "cancelling_at": null,
  "cancelled_at": null,
  "request_counts": {
    "total": 0,
    "completed": 0,
    "failed": 0
  },
  "metadata": {
    "customer_id": "user_123456789",
    "batch_description": "Nightly eval job"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» id|string|true|none||none|
|» object|string|true|none||none|
|» endpoint|string|true|none||none|
|» errors|null|true|none||none|
|» input_file_id|string|true|none||none|
|» completion_window|string|true|none||none|
|» status|string|true|none||none|
|» output_file_id|null|true|none||none|
|» error_file_id|null|true|none||none|
|» created_at|integer|true|none||none|
|» in_progress_at|null|true|none||none|
|» expires_at|null|true|none||none|
|» finalizing_at|null|true|none||none|
|» completed_at|null|true|none||none|
|» failed_at|null|true|none||none|
|» expired_at|null|true|none||none|
|» cancelling_at|null|true|none||none|
|» cancelled_at|null|true|none||none|
|» request_counts|object|true|none||none|
|»» total|integer|true|none||none|
|»» completed|integer|true|none||none|
|»» failed|integer|true|none||none|
|» metadata|object|true|none||none|
|»» customer_id|string|true|none||none|
|»» batch_description|string|true|none||none|

## GET 列出批次

GET /v1/batches

> Body 请求参数

```yaml
{}

```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|cur_page_num|query|integer| 否 ||none|
|num_per_page|query|integer| 否 ||none|
|status|query|string| 否 ||none|
|body|body|object| 否 ||none|

> 返回示例

> 200 Response

```json
{
  "object": "list",
  "data": [
    {
      "id": "batch_abc123",
      "object": "batch",
      "endpoint": "/v1/chat/completions",
      "errors": null,
      "input_file_id": "file-abc123",
      "completion_window": "24h",
      "status": "completed",
      "output_file_id": "file-cvaTdG",
      "error_file_id": "file-HOWS94",
      "created_at": 1711471533,
      "in_progress_at": 1711471538,
      "expires_at": 1711557933,
      "finalizing_at": 1711493133,
      "completed_at": 1711493163,
      "failed_at": null,
      "expired_at": null,
      "cancelling_at": null,
      "cancelled_at": null,
      "request_counts": {
        "total": 100,
        "completed": 95,
        "failed": 5
      },
      "metadata": {
        "customer_id": "user_123456789",
        "batch_description": "Nightly job"
      }
    }
  ],
  "first_id": "batch_abc123",
  "last_id": "batch_abc456",
  "has_more": true
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» object|string|true|none||none|
|» data|[object]|true|none||none|
|»» id|string|false|none||none|
|»» object|string|false|none||none|
|»» endpoint|string|false|none||none|
|»» errors|null|false|none||none|
|»» input_file_id|string|false|none||none|
|»» completion_window|string|false|none||none|
|»» status|string|false|none||none|
|»» output_file_id|string|false|none||none|
|»» error_file_id|string|false|none||none|
|»» created_at|integer|false|none||none|
|»» in_progress_at|integer|false|none||none|
|»» expires_at|integer|false|none||none|
|»» finalizing_at|integer|false|none||none|
|»» completed_at|integer|false|none||none|
|»» failed_at|null|false|none||none|
|»» expired_at|null|false|none||none|
|»» cancelling_at|null|false|none||none|
|»» cancelled_at|null|false|none||none|
|»» request_counts|object|false|none||none|
|»»» total|integer|true|none||none|
|»»» completed|integer|true|none||none|
|»»» failed|integer|true|none||none|
|»» metadata|object|false|none||none|
|»»» customer_id|string|true|none||none|
|»»» batch_description|string|true|none||none|
|» first_id|string|true|none||none|
|» last_id|string|true|none||none|
|» has_more|boolean|true|none||none|

## GET 检索批次

GET /v1/batches/{batch_id}

> Body 请求参数

```yaml
{}

```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|batch_id|path|string| 是 ||none|
|body|body|object| 否 ||none|

> 返回示例

> 200 Response

```json
{
  "id": "batch_abc123",
  "object": "batch",
  "endpoint": "/v1/completions",
  "errors": null,
  "input_file_id": "file-abc123",
  "completion_window": "24h",
  "status": "completed",
  "output_file_id": "file-cvaTdG",
  "error_file_id": "file-HOWS94",
  "created_at": 1711471533,
  "in_progress_at": 1711471538,
  "expires_at": 1711557933,
  "finalizing_at": 1711493133,
  "completed_at": 1711493163,
  "failed_at": null,
  "expired_at": null,
  "cancelling_at": null,
  "cancelled_at": null,
  "request_counts": {
    "total": 100,
    "completed": 95,
    "failed": 5
  },
  "metadata": {
    "customer_id": "user_123456789",
    "batch_description": "Nightly eval job"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» id|string|true|none||none|
|» object|string|true|none||none|
|» endpoint|string|true|none||none|
|» errors|null|true|none||none|
|» input_file_id|string|true|none||none|
|» completion_window|string|true|none||none|
|» status|string|true|none||none|
|» output_file_id|string|true|none||none|
|» error_file_id|string|true|none||none|
|» created_at|integer|true|none||none|
|» in_progress_at|integer|true|none||none|
|» expires_at|integer|true|none||none|
|» finalizing_at|integer|true|none||none|
|» completed_at|integer|true|none||none|
|» failed_at|null|true|none||none|
|» expired_at|null|true|none||none|
|» cancelling_at|null|true|none||none|
|» cancelled_at|null|true|none||none|
|» request_counts|object|true|none||none|
|»» total|integer|true|none||none|
|»» completed|integer|true|none||none|
|»» failed|integer|true|none||none|
|» metadata|object|true|none||none|
|»» customer_id|string|true|none||none|
|»» batch_description|string|true|none||none|

## DELETE 删除批次

DELETE /v1/batches/{batch_id}

> Body 请求参数

```yaml
{}

```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|batch_id|path|string| 是 ||none|
|body|body|object| 否 ||none|

> 返回示例

> 200 Response

```json
{
  "id": "batch_abc123",
  "object": "batch",
  "endpoint": "/v1/chat/completions",
  "errors": null,
  "input_file_id": "file-abc123",
  "completion_window": "24h",
  "status": "cancelling",
  "output_file_id": null,
  "error_file_id": null,
  "created_at": 1711471533,
  "in_progress_at": 1711471538,
  "expires_at": 1711557933,
  "finalizing_at": null,
  "completed_at": null,
  "failed_at": null,
  "expired_at": null,
  "cancelling_at": 1711475133,
  "cancelled_at": null,
  "request_counts": {
    "total": 100,
    "completed": 23,
    "failed": 1
  },
  "metadata": {
    "customer_id": "user_123456789",
    "batch_description": "Nightly eval job"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» id|string|true|none||none|
|» object|string|true|none||none|
|» endpoint|string|true|none||none|
|» errors|null|true|none||none|
|» input_file_id|string|true|none||none|
|» completion_window|string|true|none||none|
|» status|string|true|none||none|
|» output_file_id|null|true|none||none|
|» error_file_id|null|true|none||none|
|» created_at|integer|true|none||none|
|» in_progress_at|integer|true|none||none|
|» expires_at|integer|true|none||none|
|» finalizing_at|null|true|none||none|
|» completed_at|null|true|none||none|
|» failed_at|null|true|none||none|
|» expired_at|null|true|none||none|
|» cancelling_at|integer|true|none||none|
|» cancelled_at|null|true|none||none|
|» request_counts|object|true|none||none|
|»» total|integer|true|none||none|
|»» completed|integer|true|none||none|
|»» failed|integer|true|none||none|
|» metadata|object|true|none||none|
|»» customer_id|string|true|none||none|
|»» batch_description|string|true|none||none|

## POST 取消批次

POST /v1/batches/{batch_id}

> Body 请求参数

```yaml
{}

```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|batch_id|path|string| 是 ||none|
|body|body|object| 否 ||none|

> 返回示例

> 200 Response

```json
{
  "id": "batch_abc123",
  "object": "batch",
  "endpoint": "/v1/chat/completions",
  "errors": null,
  "input_file_id": "file-abc123",
  "completion_window": "24h",
  "status": "cancelling",
  "output_file_id": null,
  "error_file_id": null,
  "created_at": 1711471533,
  "in_progress_at": 1711471538,
  "expires_at": 1711557933,
  "finalizing_at": null,
  "completed_at": null,
  "failed_at": null,
  "expired_at": null,
  "cancelling_at": 1711475133,
  "cancelled_at": null,
  "request_counts": {
    "total": 100,
    "completed": 23,
    "failed": 1
  },
  "metadata": {
    "customer_id": "user_123456789",
    "batch_description": "Nightly eval job"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» id|string|true|none||none|
|» object|string|true|none||none|
|» endpoint|string|true|none||none|
|» errors|null|true|none||none|
|» input_file_id|string|true|none||none|
|» completion_window|string|true|none||none|
|» status|string|true|none||none|
|» output_file_id|null|true|none||none|
|» error_file_id|null|true|none||none|
|» created_at|integer|true|none||none|
|» in_progress_at|integer|true|none||none|
|» expires_at|integer|true|none||none|
|» finalizing_at|null|true|none||none|
|» completed_at|null|true|none||none|
|» failed_at|null|true|none||none|
|» expired_at|null|true|none||none|
|» cancelling_at|integer|true|none||none|
|» cancelled_at|null|true|none||none|
|» request_counts|object|true|none||none|
|»» total|integer|true|none||none|
|»» completed|integer|true|none||none|
|»» failed|integer|true|none||none|
|» metadata|object|true|none||none|
|»» customer_id|string|true|none||none|
|»» batch_description|string|true|none||none|

# Xinference/配置设置

## GET 获取database详情

GET /v1/setting/database

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "database_name": "",
    "database_dialect": "sqlite",
    "database_url": "",
    "database_port": 3306,
    "database_username": "",
    "database_password": "",
    "database_driver": ""
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» database_name|string|true|none||none|
|»» database_dialect|string|true|none||none|
|»» database_url|string|true|none||none|
|»» database_port|integer|true|none||none|
|»» database_username|string|true|none||none|
|»» database_password|string|true|none||none|
|»» database_driver|string|true|none||none|

## PUT 修改database

PUT /v1/setting/database

> Body 请求参数

```json
{
  "database_name": "",
  "database_dialect": "sqlite",
  "database_url": "",
  "database_port": 3306,
  "database_username": "",
  "database_password": "",
  "database_driver": ""
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» database_name|body|string| 是 ||none|
|» database_dialect|body|string| 是 ||none|
|» database_url|body|string| 是 ||none|
|» database_username|body|string| 是 ||none|
|» database_password|body|string| 是 ||none|
|» database_driver|body|string| 是 ||none|
|» database_port|body|integer| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "database_name": "",
    "database_dialect": "sqlite",
    "database_url": "",
    "database_port": 3306,
    "database_username": "",
    "database_password": "",
    "database_driver": ""
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» database_name|string|true|none||none|
|»» database_dialect|string|true|none||none|
|»» database_url|string|true|none||none|
|»» database_port|integer|true|none||none|
|»» database_username|string|true|none||none|
|»» database_password|string|true|none||none|
|»» database_driver|string|true|none||none|

## GET 获取auth详情

GET /v1/setting/auth

> 返回示例

```json
{
  "auth_algorithm": "xx",
  "auth_tokenDefaultExpire": 30,
  "auth_secretKey": "xxx",
  "auth_licenseKey": "xxx"
}
```

```json
{
  "message": "Request Successful.",
  "data": {
    "auth_algorithm": "HS256",
    "auth_token_default_expire": 30,
    "auth_secret_key": "09d25e094faa6ca2556c818166b7a9563b93f7099f6f0f4caa6cf63b88e8d3e7",
    "auth_license_key": "license"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» auth_algorithm|string|true|none||none|
|»» auth_token_default_expire|integer|true|none||none|
|»» auth_secret_key|string|true|none||none|
|»» auth_license_key|string|true|none||none|

## PUT 修改auth

PUT /v1/setting/auth

> Body 请求参数

```json
{
  "auth_algorithm": "HS256",
  "auth_token_default_expire": 30,
  "auth_secret_key": "09d25e094faa6ca2556c818166b7a9563b93f7099f6f0f4caa6cf63b88e8d3e7",
  "auth_license_key": "license"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» auth_algorithm|body|string| 是 | 认证算法|none|
|» auth_token_default_expire|body|integer| 是 ||none|
|» auth_secret_key|body|string| 是 ||none|
|» auth_license_key|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "auth_algorithm": "HS256",
    "auth_token_default_expire": 30,
    "auth_secret_key": "09d25e094faa6ca2556c818166b7a9563b93f7099f6f0f4caa6cf63b88e8d3e7",
    "auth_license_key": "license"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» auth_algorithm|string|true|none||none|
|»» auth_token_default_expire|integer|true|none||none|
|»» auth_secret_key|string|true|none||none|
|»» auth_license_key|string|true|none||none|

## GET 获取langfuse详情

GET /v1/setting/langfuse

> 返回示例

```json
{
  "langfuse_host": "xx",
  "langfuse_secretKey": "xx",
  "langfuse_publicKey": "xx"
}
```

```json
{
  "message": "Request Successful.",
  "data": {
    "langfuse_host": "",
    "langfuse_secret_key": "",
    "langfuse_public_key": ""
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» langfuse_host|string|true|none||none|
|»» langfuse_secret_key|string|true|none||none|
|»» langfuse_public_key|string|true|none||none|

## PUT 修改langfuse

PUT /v1/setting/langfuse

> Body 请求参数

```json
{
  "langfuse_host": "",
  "langfuse_secret_key": "",
  "langfuse_public_key": ""
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» langfuse_host|body|string| 是 | Langfuse主机地址|none|
|» langfuse_secret_key|body|string| 是 ||none|
|» langfuse_public_key|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "langfuse_host": "",
    "langfuse_secret_key": "",
    "langfuse_public_key": ""
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» langfuse_host|string|true|none||none|
|»» langfuse_secret_key|string|true|none||none|
|»» langfuse_public_key|string|true|none||none|

# Xinference/单点登录

## POST 未命名接口

POST /v1/sso/providers

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET 未命名接口

GET /api/oidc/authorize

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET 未命名接口

GET /user/login

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Comments

## POST Create

POST /api/public/comments

Create a comment. Comments may be attached to different object types (trace, observation, session, prompt).

> Body 请求参数

```json
{
  "projectId": "example",
  "objectType": "example",
  "objectId": "example",
  "content": "example",
  "authorUserId": "example"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» projectId|body|string| 是 ||none|
|» objectType|body|string| 是 ||none|
|» objectId|body|string| 是 ||none|
|» content|body|string| 是 ||none|
|» authorUserId|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get

GET /api/public/comments

Get all comments

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 是 ||Page number, starts at 1.|
|limit|query|string| 是 ||Limit of items per page. If you encounter api issues due to too large page sizes, try to reduce the limit|
|objectType|query|string| 是 ||Filter comments by object type (trace, observation, session, prompt).|
|objectId|query|string| 是 ||Filter comments by object id. If objectType is not provided, an error will be thrown.|
|authorUserId|query|string| 是 ||Filter comments by author user id.|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get By Id

GET /api/public/comments/{commentId}

Get a comment by id

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|commentId|path|string| 是 ||The unique langfuse identifier of a comment|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Dataset Items

## POST Create

POST /api/public/dataset-items

Create a dataset item

> Body 请求参数

```json
{
  "datasetName": "example",
  "input": "UNKNOWN",
  "expectedOutput": "UNKNOWN",
  "metadata": "UNKNOWN",
  "sourceTraceId": "example",
  "sourceObservationId": "example",
  "id": "example",
  "status": "ACTIVE"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» datasetName|body|string| 是 ||none|
|» input|body|string| 是 ||none|
|» expectedOutput|body|string| 是 ||none|
|» metadata|body|string| 是 ||none|
|» sourceTraceId|body|string| 是 ||none|
|» sourceObservationId|body|string| 是 ||none|
|» id|body|string| 是 ||none|
|» status|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET List

GET /api/public/dataset-items

Get dataset items

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|datasetName|query|string| 是 ||none|
|sourceTraceId|query|string| 是 ||none|
|sourceObservationId|query|string| 是 ||none|
|page|query|string| 是 ||page number, starts at 1|
|limit|query|string| 是 ||limit of items per page|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get

GET /api/public/dataset-items/{id}

Get a dataset item

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|path|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Dataset Run Items

## POST Create

POST /api/public/dataset-run-items

Create a dataset run item

> Body 请求参数

```json
{
  "runName": "example",
  "runDescription": "example",
  "metadata": "UNKNOWN",
  "datasetItemId": "example",
  "observationId": "example",
  "traceId": "example"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» runName|body|string| 是 ||none|
|» runDescription|body|string| 是 ||none|
|» metadata|body|string| 是 ||none|
|» datasetItemId|body|string| 是 ||none|
|» observationId|body|string| 是 ||none|
|» traceId|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Datasets

## GET List

GET /api/public/v2/datasets

Get all datasets

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 是 ||page number, starts at 1|
|limit|query|string| 是 ||limit of items per page|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## POST Create

POST /api/public/v2/datasets

Create a dataset

> Body 请求参数

```json
{
  "name": "example",
  "description": "example",
  "metadata": "UNKNOWN"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» name|body|string| 是 ||none|
|» description|body|string| 是 ||none|
|» metadata|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get

GET /api/public/v2/datasets/{datasetName}

Get a dataset

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|datasetName|path|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get Run

GET /api/public/datasets/{datasetName}/runs/{runName}

Get a dataset run and its items

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|datasetName|path|string| 是 ||none|
|runName|path|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get Runs

GET /api/public/datasets/{datasetName}/runs

Get dataset runs

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|datasetName|path|string| 是 ||none|
|page|query|string| 是 ||page number, starts at 1|
|limit|query|string| 是 ||limit of items per page|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Health

## GET Health

GET /api/public/health

Check health of API and database

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Ingestion

## POST Batch

POST /api/public/ingestion

Batched ingestion for Langfuse Tracing. If you want to use tracing via the API, such as to build your own Langfuse client implementation, this is the only API route you need to implement.

Notes:

- Batch sizes are limited to 3.5 MB in total. You need to adjust the number of events per batch accordingly.
- The API does not return a 4xx status code for input errors. Instead, it responds with a 207 status code, which includes a list of the encountered errors.

> Body 请求参数

```json
{
  "batch": [
    {
      "type": "trace-create",
      "body": {
        "id": "example",
        "timestamp": "1994-11-05T13:15:30Z",
        "name": "example",
        "userId": "example",
        "input": "UNKNOWN",
        "output": "UNKNOWN",
        "sessionId": "example",
        "release": "example",
        "version": "example",
        "metadata": "UNKNOWN",
        "tags": [
          "example"
        ],
        "public": true
      },
      "id": "example",
      "timestamp": "example",
      "metadata": "UNKNOWN"
    }
  ],
  "metadata": "UNKNOWN"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» batch|body|[object]| 是 ||none|
|»» type|body|string| 否 ||none|
|»» body|body|object| 否 ||none|
|»»» id|body|string| 是 ||none|
|»»» timestamp|body|string| 是 ||none|
|»»» name|body|string| 是 ||none|
|»»» userId|body|string| 是 ||none|
|»»» input|body|string| 是 ||none|
|»»» output|body|string| 是 ||none|
|»»» sessionId|body|string| 是 ||none|
|»»» release|body|string| 是 ||none|
|»»» version|body|string| 是 ||none|
|»»» metadata|body|string| 是 ||none|
|»»» tags|body|[string]| 是 ||none|
|»»» public|body|boolean| 是 ||none|
|»» id|body|string| 否 ||none|
|»» timestamp|body|string| 否 ||none|
|»» metadata|body|string| 否 ||none|
|» metadata|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Metrics

## GET Daily

GET /api/public/metrics/daily

Get daily metrics of the Langfuse project

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 是 ||page number, starts at 1|
|limit|query|string| 是 ||limit of items per page|
|traceName|query|string| 是 ||Optional filter by the name of the trace|
|userId|query|string| 是 ||Optional filter by the userId associated with the trace|
|tags|query|string| 是 ||Optional filter for metrics where traces include all of these tags|
|fromTimestamp|query|string| 是 ||Optional filter to only include traces and observations on or after a certain datetime (ISO 8601)|
|toTimestamp|query|string| 是 ||Optional filter to only include traces and observations before a certain datetime (ISO 8601)|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Models

## POST Create

POST /api/public/models

Create a model

> Body 请求参数

```json
{
  "modelName": "example",
  "matchPattern": "example",
  "startDate": "1994-11-05T13:15:30Z",
  "unit": "CHARACTERS",
  "inputPrice": 0,
  "outputPrice": 0,
  "totalPrice": 0,
  "tokenizerId": "example",
  "tokenizerConfig": "UNKNOWN"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» modelName|body|string| 是 ||none|
|» matchPattern|body|string| 是 ||none|
|» startDate|body|string| 是 ||none|
|» unit|body|string| 是 ||none|
|» inputPrice|body|integer| 是 ||none|
|» outputPrice|body|integer| 是 ||none|
|» totalPrice|body|integer| 是 ||none|
|» tokenizerId|body|string| 是 ||none|
|» tokenizerConfig|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET List

GET /api/public/models

Get all models

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 是 ||page number, starts at 1|
|limit|query|string| 是 ||limit of items per page|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get

GET /api/public/models/{id}

Get a model

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|path|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## DELETE Delete

DELETE /api/public/models/{id}

Delete a model. Cannot delete models managed by Langfuse. You can create your own definition with the same modelName to override the definition though.

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|path|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Observations

## GET Get

GET /api/public/observations/{observationId}

Get a observation

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|observationId|path|string| 是 ||The unique langfuse identifier of an observation, can be an event, span or generation|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get Many

GET /api/public/observations

Get a list of observations

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 是 ||Page number, starts at 1.|
|limit|query|string| 是 ||Limit of items per page. If you encounter api issues due to too large page sizes, try to reduce the limit.|
|name|query|string| 是 ||none|
|userId|query|string| 是 ||none|
|type|query|string| 是 ||none|
|traceId|query|string| 是 ||none|
|parentObservationId|query|string| 是 ||none|
|fromStartTime|query|string| 是 ||Retrieve only observations with a start_time or or after this datetime (ISO 8601).|
|toStartTime|query|string| 是 ||Retrieve only observations with a start_time before this datetime (ISO 8601).|
|version|query|string| 是 ||Optional filter to only include observations with a certain version.|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Projects

## GET Get

GET /api/public/projects

Get Project associated with API key

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Prompts

## GET Get

GET /api/public/v2/prompts/{promptName}

Get a prompt

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|promptName|path|string| 是 ||The name of the prompt|
|version|query|string| 是 ||Version of the prompt to be retrieved.|
|label|query|string| 是 ||Label of the prompt to be retrieved. Defaults to "production" if no label or version is set.|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET List

GET /api/public/v2/prompts

Get a list of prompt names with versions and labels

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|name|query|string| 是 ||none|
|label|query|string| 是 ||none|
|tag|query|string| 是 ||none|
|page|query|string| 是 ||page number, starts at 1|
|limit|query|string| 是 ||limit of items per page|
|fromUpdatedAt|query|string| 是 ||Optional filter to only include prompt versions created/updated on or after a certain datetime (ISO 8601)|
|toUpdatedAt|query|string| 是 ||Optional filter to only include prompt versions created/updated before a certain datetime (ISO 8601)|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## POST Create

POST /api/public/v2/prompts

Create a new version for the prompt with the given `name`

> Body 请求参数

```json
{
  "type": "chat",
  "name": "example",
  "prompt": [
    {
      "role": "example",
      "content": "example"
    }
  ],
  "config": "UNKNOWN",
  "labels": [
    "example"
  ],
  "tags": [
    "example"
  ]
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» type|body|string| 是 ||none|
|» name|body|string| 是 ||none|
|» prompt|body|[object]| 是 ||none|
|»» role|body|string| 否 ||none|
|»» content|body|string| 否 ||none|
|» config|body|string| 是 ||none|
|» labels|body|[string]| 是 ||none|
|» tags|body|[string]| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Score Configs

## POST Create

POST /api/public/score-configs

Create a score configuration (config). Score configs are used to define the structure of scores

> Body 请求参数

```json
{
  "name": "example",
  "dataType": "NUMERIC",
  "categories": [
    {
      "value": 0,
      "label": "example"
    }
  ],
  "minValue": 0,
  "maxValue": 0,
  "description": "example"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» name|body|string| 是 ||none|
|» dataType|body|string| 是 ||none|
|» categories|body|[object]| 是 ||none|
|»» value|body|integer| 否 ||none|
|»» label|body|string| 否 ||none|
|» minValue|body|integer| 是 ||none|
|» maxValue|body|integer| 是 ||none|
|» description|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get

GET /api/public/score-configs

Get all score configs

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 是 ||Page number, starts at 1.|
|limit|query|string| 是 ||Limit of items per page. If you encounter api issues due to too large page sizes, try to reduce the limit|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get By Id

GET /api/public/score-configs/{configId}

Get a score config

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|configId|path|string| 是 ||The unique langfuse identifier of a score config|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Score

## POST Create

POST /api/public/scores

Create a score

> Body 请求参数

```json
{
  "name": "novelty",
  "value": 0.9,
  "traceId": "cdef-1234-5678-90ab"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» name|body|string| 是 ||none|
|» value|body|number| 是 ||none|
|» traceId|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get

GET /api/public/scores

Get a list of scores

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 是 ||Page number, starts at 1.|
|limit|query|string| 是 ||Limit of items per page. If you encounter api issues due to too large page sizes, try to reduce the limit.|
|userId|query|string| 是 ||Retrieve only scores with this userId associated to the trace.|
|name|query|string| 是 ||Retrieve only scores with this name.|
|fromTimestamp|query|string| 是 ||Optional filter to only include scores created on or after a certain datetime (ISO 8601)|
|toTimestamp|query|string| 是 ||Optional filter to only include scores created before a certain datetime (ISO 8601)|
|source|query|string| 是 ||Retrieve only scores from a specific source.|
|operator|query|string| 是 ||Retrieve only scores with <operator> value.|
|value|query|string| 是 ||Retrieve only scores with <operator> value.|
|scoreIds|query|string| 是 ||Comma-separated list of score IDs to limit the results to.|
|configId|query|string| 是 ||Retrieve only scores with a specific configId.|
|queueId|query|string| 是 ||Retrieve only scores with a specific annotation queueId.|
|dataType|query|string| 是 ||Retrieve only scores with a specific dataType.|
|traceTags|query|string| 是 ||Only scores linked to traces that include all of these tags will be returned.|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get By Id

GET /api/public/scores/{scoreId}

Get a score

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|scoreId|path|string| 是 ||The unique langfuse identifier of a score|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## DELETE Delete

DELETE /api/public/scores/{scoreId}

Delete a score

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|scoreId|path|string| 是 ||The unique langfuse identifier of a score|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Sessions

## GET List

GET /api/public/sessions

Get sessions

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 是 ||Page number, starts at 1|
|limit|query|string| 是 ||Limit of items per page. If you encounter api issues due to too large page sizes, try to reduce the limit.|
|fromTimestamp|query|string| 是 ||Optional filter to only include sessions created on or after a certain datetime (ISO 8601)|
|toTimestamp|query|string| 是 ||Optional filter to only include sessions created before a certain datetime (ISO 8601)|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET Get

GET /api/public/sessions/{sessionId}

Get a session. Please note that `traces` on this endpoint are not paginated, if you plan to fetch large sessions, consider `GET /api/public/traces?sessionId=<sessionId>`

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|sessionId|path|string| 是 ||The unique id of a session|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Langfuse/Trace

## GET Get

GET /api/public/traces/{traceId}

Get a specific trace

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|traceId|path|string| 是 ||The unique langfuse identifier of a trace|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET List

GET /api/public/traces

Get list of traces

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 是 ||Page number, starts at 1|
|limit|query|string| 是 ||Limit of items per page. If you encounter api issues due to too large page sizes, try to reduce the limit.|
|userId|query|string| 是 ||none|
|name|query|string| 是 ||none|
|sessionId|query|string| 是 ||none|
|fromTimestamp|query|string| 是 ||Optional filter to only include traces with a trace.timestamp on or after a certain datetime (ISO 8601)|
|toTimestamp|query|string| 是 ||Optional filter to only include traces with a trace.timestamp before a certain datetime (ISO 8601)|
|orderBy|query|string| 是 ||Format of the string [field].[asc/desc]. Fields: id, timestamp, name, userId, release, version, public, bookmarked, sessionId. Example: timestamp.asc|
|tags|query|string| 是 ||Only traces that include all of these tags will be returned.|
|version|query|string| 是 ||Optional filter to only include traces with a certain version.|
|release|query|string| 是 ||Optional filter to only include traces with a certain release.|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# Prometheus/query

## GET metr

GET /

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## GET 原接口

GET /原接口

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# 数据模型

<h2 id="tocS_Tag">Tag</h2>

<a id="schematag"></a>
<a id="schema_Tag"></a>
<a id="tocStag"></a>
<a id="tocstag"></a>

```json
{
  "id": 1,
  "name": "string"
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|id|integer(int64)|false|none||标签ID编号|
|name|string|false|none||标签名称|

<h2 id="tocS_Category">Category</h2>

<a id="schemacategory"></a>
<a id="schema_Category"></a>
<a id="tocScategory"></a>
<a id="tocscategory"></a>

```json
{
  "id": 1,
  "name": "string"
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|id|integer(int64)|false|none||分组ID编号|
|name|string|false|none||分组名称|

<h2 id="tocS_Pet">Pet</h2>

<a id="schemapet"></a>
<a id="schema_Pet"></a>
<a id="tocSpet"></a>
<a id="tocspet"></a>

```json
{
  "id": 1,
  "category": {
    "id": 1,
    "name": "string"
  },
  "name": "doggie",
  "photoUrls": [
    "string"
  ],
  "tags": [
    {
      "id": 1,
      "name": "string"
    }
  ],
  "status": "available"
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|id|integer(int64)|true|none||宠物ID编号|
|category|[Category](#schemacategory)|true|none||分组|
|name|string|true|none||名称|
|photoUrls|[string]|true|none||照片URL|
|tags|[[Tag](#schematag)]|true|none||标签|
|status|string|true|none||宠物销售状态|

#### 枚举值

|属性|值|
|---|---|
|status|available|
|status|pending|
|status|sold|
