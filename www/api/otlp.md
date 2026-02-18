# OTLP Ingestion

xtrace supports OpenTelemetry Protocol (OTLP/HTTP) trace ingestion, making it compatible with any instrumentation that exports OTLP traces — including Langfuse's OpenTelemetry exporter.

## Endpoint

```
POST /api/public/otel/v1/traces
```

## Supported Formats

| Content-Type | Description |
|-------------|-------------|
| `application/json` | JSON-encoded OTLP ExportTraceServiceRequest |
| `application/x-protobuf` | Protobuf-encoded OTLP ExportTraceServiceRequest |

Gzip compression is supported via `Content-Encoding: gzip`.

## Authentication

Both Bearer token and Basic auth are supported:

```bash
# Bearer token
curl -X POST http://127.0.0.1:8742/api/public/otel/v1/traces \
  -H "Authorization: Bearer $API_BEARER_TOKEN" \
  -H "Content-Type: application/json" \
  -d '...'

# Basic auth (Langfuse compatibility)
curl -X POST http://127.0.0.1:8742/api/public/otel/v1/traces \
  -u "$XTRACE_PUBLIC_KEY:$XTRACE_SECRET_KEY" \
  -H "Content-Type: application/json" \
  -d '...'
```

## Attribute Mapping

xtrace extracts the following OTLP span attributes:

| OTLP Attribute | Maps To |
|---------------|---------|
| `traceId` (16 bytes hex) | `trace.id` (UUID) |
| `spanId` (8 bytes hex) | `observation.id` (UUID, zero-padded) |
| `parentSpanId` | `observation.parentObservationId` |
| `name` | `observation.name` |
| `startTimeUnixNano` | `observation.startTime` |
| `endTimeUnixNano` | `observation.endTime` |
| `langfuse.observation.type` | `observation.type` |
| `langfuse.generation.model` | `observation.model` |
| `gen_ai.request.model` | `observation.model` (fallback) |
| `langfuse.observation.input` | `observation.input` |
| `langfuse.observation.output` | `observation.output` |
| `langfuse.observation.usage_details` | Token counts |
| `langfuse.trace.name` | `trace.name` |
| `user.id` | `trace.userId` |
| `session.id` | `trace.sessionId` |
| `langfuse.trace.tags` | `trace.tags` |
| `langfuse.trace.metadata.*` | `trace.metadata` |

## Span Hierarchy

OTLP parent-child span relationships are preserved:

- Spans with `parentSpanId` create nested observations
- Root spans (no parent) become top-level observations
- The span tree is reconstructed client-side using `parentObservationId`

This naturally supports agent workflow visualization:

```
Trace
  └─ Span: "P3 Cycle"
       ├─ Span: "Plan"    (parentSpanId = cycle)
       ├─ Span: "Execute"  (parentSpanId = cycle)
       │    └─ Span: "Tool Call"  (parentSpanId = execute)
       └─ Span: "Reflect"  (parentSpanId = cycle)
```

## Usage with Langfuse Python SDK

```python
import os
os.environ["LANGFUSE_HOST"] = "http://127.0.0.1:8742"
os.environ["LANGFUSE_PUBLIC_KEY"] = "pk-xxx"
os.environ["LANGFUSE_SECRET_KEY"] = "sk-yyy"

from langfuse import Langfuse
langfuse = Langfuse()

trace = langfuse.trace(name="my-trace")
span = trace.span(name="agent-step")
generation = span.generation(
    name="llm-call",
    model="gpt-4",
    input=[{"role": "user", "content": "Hello"}],
    output="Hi there!",
)
langfuse.flush()
```
