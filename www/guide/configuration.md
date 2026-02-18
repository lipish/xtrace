# Configuration

xtrace is configured entirely through environment variables.

## Required

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | PostgreSQL connection string, e.g. `postgresql://user:pass@127.0.0.1:5432/xtrace` |
| `API_BEARER_TOKEN` | Bearer token that protects all API endpoints |

## Optional

| Variable | Default | Description |
|----------|---------|-------------|
| `BIND_ADDR` | `127.0.0.1:8742` | Address and port to listen on |
| `DEFAULT_PROJECT_ID` | `default` | Project ID for all ingested data (single-tenant mode) |
| `RATE_LIMIT_QPS` | `20` | Per-token sustained query rate (requests/second) |
| `RATE_LIMIT_BURST` | `40` | Per-token burst allowance |

## Langfuse Compatibility

For drop-in compatibility with Langfuse SDK instrumentation (BasicAuth):

| Variable | Description |
|----------|-------------|
| `XTRACE_PUBLIC_KEY` | Public key for BasicAuth username |
| `XTRACE_SECRET_KEY` | Secret key for BasicAuth password |

Legacy names `LANGFUSE_PUBLIC_KEY` / `LANGFUSE_SECRET_KEY` are also supported.

When both keys are set, the OTLP and project endpoints accept `Basic <base64(public_key:secret_key)>` in addition to Bearer token auth.

## Rate Limiting

Query endpoints (`/api/public/*`) are rate-limited per authentication token using a token-bucket algorithm.

- **Sustained rate**: `RATE_LIMIT_QPS` requests per second (default 20)
- **Burst**: `RATE_LIMIT_BURST` requests (default 40)
- **Scope**: Query routes only; write routes use channel backpressure

When exceeded, the server returns `429 Too Many Requests` with a `Retry-After` header.

::: tip
You can inspect rate limit statistics at `GET /api/internal/rate_limit_stats` (no auth required).
:::

## Database

xtrace runs migrations automatically on startup via `sqlx::migrate!`. No manual migration step is needed.

Tables created:

| Table | Purpose |
|-------|---------|
| `traces` | Trace records with tags, metadata, costs |
| `observations` | Span/observation records with parent hierarchy |
| `metrics` | Time-series metric data points with labels |

## Example Startup

```bash
export DATABASE_URL=postgresql://xinference@localhost:5432/xtrace
export API_BEARER_TOKEN=my-secret-token
export BIND_ADDR=0.0.0.0:8742
export RATE_LIMIT_QPS=50
export RATE_LIMIT_BURST=100

# Optional: Langfuse compatibility
export XTRACE_PUBLIC_KEY=pk-lf-xxx
export XTRACE_SECRET_KEY=sk-lf-yyy

cargo run --release
```
