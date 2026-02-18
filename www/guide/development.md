# Development Guide

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Web framework | Axum (on Tokio) |
| Serialization | Serde |
| Database | PostgreSQL + SQLx |
| Logging | tracing + tower-http |
| Configuration | Environment variables |

## Local Setup

### Prerequisites

- Rust stable (>= 1.75)
- PostgreSQL 14+
- sqlx-cli (optional but recommended)

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

### Create Database

```bash
createdb xtrace
# or
psql -c "CREATE DATABASE xtrace;"
```

### Run Migrations

Migrations run automatically on startup. To run them manually:

```bash
export DATABASE_URL=postgres://user:pass@127.0.0.1:5432/xtrace
sqlx migrate run
```

### Start the Server

```bash
export DATABASE_URL=postgres://user:pass@127.0.0.1:5432/xtrace
export API_BEARER_TOKEN=dev-token
cargo run
```

## Architecture

### Write Path

The write path is designed for minimal latency impact:

1. HTTP handler performs light validation and auth
2. Payload is pushed into an internal `mpsc` channel
3. Background worker batches requests (up to 200 per 50ms window)
4. Single transaction with batch upsert to PostgreSQL

```
Client → HTTP Handler → mpsc channel → Background Worker → PostgreSQL
                                              ↑
                                    (micro-batch: 200/50ms)
```

### Database Schema

**Traces** store the top-level request context:
- `id`, `project_id`, `timestamp`, `name`, `session_id`, `version`, `release`
- `tags` (TEXT[] with GIN index), `metadata` (JSONB)
- `latency`, `total_cost`

**Observations** store spans within a trace:
- `trace_id` → foreign key to traces
- `parent_observation_id` → self-referential for span trees
- `model`, `prompt_tokens`, `completion_tokens`, `total_tokens`
- `metadata` (JSONB) for arbitrary structured data

**Metrics** store time-series data points:
- `name`, `labels` (JSONB with GIN index), `value`, `timestamp`

### Indexing Strategy

| Table | Index | Purpose |
|-------|-------|---------|
| traces | `(project_id, timestamp DESC)` | List queries |
| traces | `(session_id)` | Session filtering |
| traces | GIN on `tags` | Tag containment |
| observations | `(trace_id, start_time)` | Detail queries |
| metrics | `(project_id, name, timestamp DESC)` | Time-series queries |
| metrics | GIN on `labels` | Label filtering |

## Testing

### Curl Verification

```bash
export API_BEARER_TOKEN=dev-token
export BASE_URL=http://127.0.0.1:8742

# Health check
curl $BASE_URL/healthz

# List traces
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "$BASE_URL/api/public/traces?page=1&limit=2"

# Daily metrics
curl -H "Authorization: Bearer $API_BEARER_TOKEN" \
  "$BASE_URL/api/public/metrics/daily?page=1&limit=50"
```

## Workspace Structure

```
xtrace/
├── src/
│   ├── main.rs         # Entry point
│   └── server.rs       # All server logic
├── crates/
│   └── xtrace-client/  # Rust HTTP SDK
├── migrations/         # SQLx migrations
├── frontend/           # Dashboard UI (Vite + React)
├── www/                # Documentation site (VitePress)
└── docs/               # Design documents
```
