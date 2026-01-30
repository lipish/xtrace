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

# Xinference/模型监控

## GET traces接口

GET /api/public/traces

### 请求参数

| 名称           | 位置  | 类型          | 必选 | 说明             |
| -------------- | ----- | ------------- | ---- | ---------------- |
| page           | query | integer       | 否   | 页数             |
| limit          | query | integer       | 否   | 返回条数限制     |
| userId         | query | string        | 否   | 记录的用户id     |
| name           | query | string        | 否   | 记录的名称       |
| sessionId      | query | string        | 否   | 记录的session_id |
| fromTimestamp  | query | string        | 否   | iso 8601格式     |
| toTimestamp    | query | string        | 否   | iso 8601格式     |
| orderBy        | query | string        | 否   | 排序             |
| tags           | query | array[string] | 否   | 标签             |

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

| 状态码 | 状态码含义                                              | 说明 | 数据模型 |
| ------ | ------------------------------------------------------- | ---- | -------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none | Inline   |

### 返回数据结构

状态码 **200**

| 名称             | 类型          | 必选  | 约束 | 中文名 | 说明 |
| ---------------- | ------------- | ----- | ---- | ------ | ---- |
| » message        | string        | true  | none |        | none |
| » data           | object        | false | none |        | none |
| »» data          | [object]      | false | none |        | none |
| »»» id           | string        | false | none |        | none |
| »»» timestamp    | string¦null   | false | none |        | none |
| »»» name         | string¦null   | false | none |        | none |
| »»» input        | string¦null   | false | none |        | none |
| »»» output       | string¦null   | false | none |        | none |
| »»» sessionId    | string¦null   | false | none |        | none |
| »»» release      | string¦null   | false | none |        | none |
| »»» version      | string¦null   | false | none |        | none |
| »»» userId       | string¦null   | false | none |        | none |
| »»» metadata     | object¦null   | false | none |        | none |
| »»» tags         | [string]¦null | false | none |        | none |
| »»» public       | boolean       | false | none |        | none |
| »»» htmlPath     | string¦null   | false | none |        | none |
| »»» latency      | string        | false | none |        | none |
| »»» totalCost    | string        | false | none |        | none |
| »»» observations | [string]      | false | none |        | none |
| »»» scores       | [string]¦null | false | none |        | none |
| »»» externalId   | string¦null   | false | none |        | none |
| »»» bookmarked   | boolean       | false | none |        | none |
| »»» projectId    | string        | false | none |        | none |
| »»» createdAt    | string        | false | none |        | none |
| »»» updatedAt    | string        | false | none |        | none |
| »» meta          | object        | false | none |        | none |
| »»» page         | integer       | false | none |        | none |
| »»» limit        | integer       | false | none |        | none |
| »»» totalItems   | integer       | false | none |        | none |
| »»» totalPages   | integer       | false | none |        | none |

## GET trace接口

GET /api/public/traces/{trace_id}

### 请求参数

| 名称     | 位置 | 类型   | 必选 | 说明                       |
| -------- | ---- | ------ | ---- | -------------------------- |
| trace_id | path | string | 是   | 通过traces接口获取的id的值 |

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

| 状态码 | 状态码含义                                              | 说明 | 数据模型 |
| ------ | ------------------------------------------------------- | ---- | -------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none | Inline   |

### 返回数据结构

状态码 **200**

| 名称                     | 类型          | 必选  | 约束 | 中文名 | 说明    |
| ------------------------ | ------------- | ----- | ---- | ------ | ------- |
| » message                | string        | true  | none |        | none    |
| » data                   | object        | false | none |        | none    |
| »» id                    | string        | false | none |        | ID 编号 |
| »» timestamp             | string        | false | none |        | none    |
| »» name                  | string¦null   | false | none |        | none    |
| »» input                 | string¦null   | false | none |        | none    |
| »» output                | string¦null   | false | none |        | none    |
| »» sessionId             | string¦null   | false | none |        | none    |
| »» release               | string¦null   | false | none |        | none    |
| »» version               | string¦null   | false | none |        | none    |
| »» userId                | string¦null   | false | none |        | none    |
| »» metadata              | object¦null   | false | none |        | none    |
| »» tags                  | [string]¦null | false | none |        | none    |
| »» public                | boolean       | false | none |        | none    |
| »» htmlPath              | string¦null   | false | none |        | none    |
| »» latency               | number        | false | none |        | none    |
| »» totalCost             | number        | false | none |        | none    |
| »» observations          | [object]      | false | none |        | none    |
| »»» id                   | string        | false | none |        | none    |
| »»» traceId              | string        | false | none |        | none    |
| »»» type                 | string        | false | none |        | none    |
| »»» name                 | string        | false | none |        | none    |
| »»» startTime            | string        | false | none |        | none    |
| »»» endTime              | string        | false | none |        | none    |
| »»» completionStartTime  | string        | false | none |        | none    |
| »»» model                | string        | false | none |        | none    |
| »»» modelParameters      | object¦null   | false | none |        | none    |
| »»» input                | [object]      | false | none |        | none    |
| »»»» role                | string        | false | none |        | none    |
| »»»» content             | string        | false | none |        | none    |
| »»» version              | string¦null   | false | none |        | none    |
| »»» metadata             | object        | false | none |        | none    |
| »»»» stream              | boolean       | false | none |        | none    |
| »»»» stream_options      | object        | false | none |        | none    |
| »»»»» include_usage      | boolean       | false | none |        | none    |
| »»» output               | string        | false | none |        | none    |
| »»» usage                | object        | false | none |        | none    |
| »»»» input               | integer       | false | none |        | none    |
| »»»» output              | integer       | false | none |        | none    |
| »»»» total               | integer       | false | none |        | none    |
| »»»» unit                | string        | false | none |        | none    |
| »»» level                | string        | false | none |        | none    |
| »»» statusMessage        | string¦null   | false | none |        | none    |
| »»» parentObservationId  | string¦null   | false | none |        | none    |
| »»» promptId             | string¦null   | false | none |        | none    |
| »»» promptName           | string¦null   | false | none |        | none    |
| »»» promptVersion        | string¦null   | false | none |        | none    |
| »»» modelId              | string¦null   | false | none |        | none    |
| »»» inputPrice           | string¦null   | false | none |        | none    |
| »»» outputPrice          | string¦null   | false | none |        | none    |
| »»» totalPrice           | string¦null   | false | none |        | none    |
| »»» calculatedInputCost  | string¦null   | false | none |        | none    |
| »»» calculatedOutputCost | string¦null   | false | none |        | none    |
| »»» calculatedTotalCost  | string¦null   | false | none |        | none    |
| »»» latency              | number        | false | none |        | none    |
| »»» timeToFirstToken     | number        | false | none |        | none    |
| »»» completionTokens     | integer       | false | none |        | none    |
| »»» unit                 | string        | false | none |        | none    |
| »»» totalTokens          | integer       | false | none |        | none    |
| »»» projectId            | string        | false | none |        | none    |
| »»» createdAt            | string        | false | none |        | none    |
| »»» promptTokens         | integer       | false | none |        | none    |
| »»» updatedAt            | string        | false | none |        | none    |
| »» scores                | [string]      | false | none |        | none    |
| »» externalId            | string¦null   | false | none |        | none    |
| »» bookmarked            | boolean       | false | none |        | none    |
| »» projectId             | string        | false | none |        | none    |
| »» createdAt             | string        | false | none |        | none    |
| »» updatedAt             | string        | false | none |        | none    |

# 数据模型





以下接口均通过 **HTTP REST API** 方式调用，  
不依赖 Langfuse SDK，适用于服务端直接请求、网关转发或统一监控场景。

### 接口列表

- `GET /api/public/metrics/daily`  
  按天维度返回模型调用的使用量与成本等聚合统计数据，  
  主要用于每日调用量、Token 使用量及成本统计。

- `GET /api/public/traces`  
  查询 Trace 列表接口，用于按条件筛选和分页获取 Trace 元信息，  
  通常作为 Trace 查询入口及 Trace ID 获取接口。

- `GET /api/public/traces/{trace_id}`  
  查询单条 Trace 的详细信息，返回完整的 Trace 与 Observation 数据，  
  主要用于单次模型调用的调试、审计及性能分析。

### 接口关系说明

| 接口                            | 调用方式 | 数据粒度      | 主要用途          |
| ------------------------------- | -------- | ------------- | ----------------- |
| `/api/public/metrics/daily`     | HTTP     | 日级聚合      | 使用量 / 成本统计 |
| `/api/public/traces`            | HTTP     | Trace 列表    | Trace 查询与筛选  |
| `/api/public/traces/{trace_id}` | HTTP     | 单 Trace 明细 | Trace 调试与分析  |

https://api.reference.langfuse.com/#tag/trace/GET/api/public/traces/{traceId}