# Langfuse Compatibility

xtrace provides drop-in compatibility with Langfuse SDK instrumentation, allowing you to use existing Langfuse client libraries without modification.

## How It Works

xtrace implements the following Langfuse-compatible endpoints:

| Endpoint | Purpose |
|----------|---------|
| `GET /api/public/projects` | SDK auth check (returns default project) |
| `POST /api/public/otel/v1/traces` | OTLP trace ingestion |
| `GET /api/public/traces` | Trace listing |
| `GET /api/public/traces/:traceId` | Trace detail |
| `GET /api/public/metrics/daily` | Daily metrics |

## Setup

### 1. Configure xtrace Keys

Set the compatibility keys in your xtrace environment:

```bash
export XTRACE_PUBLIC_KEY=pk-lf-your-public-key
export XTRACE_SECRET_KEY=sk-lf-your-secret-key
```

### 2. Point Langfuse SDK to xtrace

```python
import os
os.environ["LANGFUSE_HOST"] = "http://your-xtrace-host:8742"
os.environ["LANGFUSE_PUBLIC_KEY"] = "pk-lf-your-public-key"
os.environ["LANGFUSE_SECRET_KEY"] = "sk-lf-your-secret-key"
```

### 3. Use Langfuse as Normal

```python
from langfuse import Langfuse

langfuse = Langfuse()

trace = langfuse.trace(name="my-trace")
generation = trace.generation(
    name="llm-call",
    model="gpt-4",
    input=[{"role": "user", "content": "Hello"}],
    output="Hi!",
)
langfuse.flush()
```

All traces and observations will be stored in xtrace.

## Authentication

xtrace supports both authentication methods:

| Method | Header | Use Case |
|--------|--------|----------|
| Bearer token | `Authorization: Bearer <API_BEARER_TOKEN>` | Direct API access, xtrace SDKs |
| Basic auth | `Authorization: Basic <base64(public:secret)>` | Langfuse SDK compatibility |

Both methods work on all endpoints. The Bearer token is always accepted; Basic auth works when `XTRACE_PUBLIC_KEY` and `XTRACE_SECRET_KEY` are configured.

## Differences from Langfuse

| Feature | Langfuse | xtrace |
|---------|----------|--------|
| Multi-tenant | Yes | Single-tenant (single project) |
| Scores | Yes | Stub (empty arrays) |
| Prompt management | Yes | Not supported |
| Datasets | Yes | Not supported |
| Time-series metrics | No | Yes (`/v1/metrics/batch`, `/api/public/metrics/query`) |
| Percentile queries | No | Yes (p50/p90/p99) |
| Self-hosted | Docker/K8s | Single binary |

## Migration

If you're migrating from Langfuse to xtrace:

1. Change `LANGFUSE_HOST` to your xtrace URL
2. Set matching `PUBLIC_KEY` and `SECRET_KEY`
3. No code changes needed in instrumentation
4. Historical data does not migrate (start fresh)
