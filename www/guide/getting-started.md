# Getting Started

xtrace is an AI observability service that collects, stores, and queries traces, observations, and metrics for LLM and agent workflows.

## Prerequisites

- **Rust** stable (>= 1.75)
- **PostgreSQL** 14+

## Install from crates.io

```bash
cargo install xtrace
```

## Run with Docker (coming soon)

```bash
docker run -d \
  -e DATABASE_URL=postgresql://user:pass@host:5432/xtrace \
  -e API_BEARER_TOKEN=your-secret-token \
  -p 8742:8742 \
  ghcr.io/lipish/xtrace:latest
```

## Run from Source

```bash
git clone https://github.com/lipish/xtrace.git
cd xtrace

DATABASE_URL=postgresql://user:pass@localhost:5432/xtrace \
API_BEARER_TOKEN=your-secret-token \
cargo run --release
```

## Verify

```bash
curl http://127.0.0.1:8742/healthz
# 200 OK
```

## Send Your First Trace

```bash
export API_BEARER_TOKEN=your-secret-token

curl -X POST http://127.0.0.1:8742/v1/l/batch \
  -H "Authorization: Bearer $API_BEARER_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "trace": {
      "id": "00000000-0000-0000-0000-000000000001",
      "timestamp": "2026-01-01T00:00:00Z",
      "name": "chat",
      "userId": "alice",
      "tags": ["prod"]
    },
    "observations": [
      {
        "id": "00000000-0000-0000-0000-000000000002",
        "traceId": "00000000-0000-0000-0000-000000000001",
        "type": "GENERATION",
        "name": "llm-call",
        "startTime": "2026-01-01T00:00:00Z",
        "endTime": "2026-01-01T00:00:01Z",
        "model": "gpt-4o-mini",
        "input": {"role": "user", "content": "Hello"},
        "output": {"role": "assistant", "content": "Hi there!"},
        "promptTokens": 5,
        "completionTokens": 12,
        "totalTokens": 17
      }
    ]
  }'
```

## Query Traces

```bash
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "http://127.0.0.1:8742/api/public/traces?page=1&limit=10"
```

## What's Next

- [Configuration](/guide/configuration) — Environment variables and tuning
- [REST API Reference](/api/rest-api) — Full endpoint documentation
- [Rust Client SDK](/sdk/rust-client) — Use the `xtrace-client` crate
