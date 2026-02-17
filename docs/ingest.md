# XTrace Ingest and Python SDK Design (Performance-First)

This document defines XTrace's ingest pipeline (internal ingest HTTP API) and Python SDK behavior, with the goal of recording OpenAI-compatible (`messages -> completion`) calls without materially affecting online request latency.

> Note
> - `/api/public/*` in `docs/api.md` is for query and aggregation only.
> - Ingest endpoints are internal, for SDK/gateway/server instrumentation.

## Design Goals

- Minimal intrusion: Few changes to business code (Langfuse-style drop-in wrapper).
- Async-first: Ingest is async by default; does not block the main request.
- Controlled discard: Under extreme load, allow dropping ingest events to protect latency and stability.
- Idempotent: Retries do not produce duplicate data.
- Batched: Network and DB writes are batched where possible.

## Terminology and Data Model

- Trace: Container for one business-level call/request.
- Observation: Event/span within a trace.
  - Generation: One model call (chat completion).

## Internal Ingest HTTP API (Suggested)

### Authentication

Use Bearer token:

- `Authorization: Bearer <token>`

Token-to-`projectId` mapping (MVP):

- Server configures static `projectId` (all writes go to one project)
- Or token maps to project (future extension)

### 1) Batch Write (Recommended)

`POST /v1/l/batch`

Request body (conceptual):

- `trace`: Trace object (optional)
- `observations`: Observation array (optional)

Server semantics:

- Supports observations-only (create placeholder trace if missing)
- Supports trace-only
- Upsert each record

Suggested responses:

- 200: All success
- 207: Partial success (return failed ids and reasons; SDK may retry selectively)
- 400: Request body validation failed
- 401/403: Auth failed
- 429: Server backpressure (SDK should back off)

### 2) Single-Record Write (Optional, for debugging)

- `POST /v1/l/traces`
- `POST /v1/l/observations`

Production SDK should merge into batch.

### Idempotency and Upsert Key

- trace: Upsert by `id`
- observation: Upsert by `id`

SDK generation rules:

- `trace_id`: UUID per chat request
- `observation_id`: UUID per model call

### Field Conventions (aligned with `docs/api.md` response structure)

Trace (MVP):

- `id` (uuid)
- `timestamp` (ISO8601)
- `name` (string|null)
- `userId` (string|null)
- `sessionId` (string|null)
- `tags` (string[])
- `metadata` (object|null)
- `input` / `output` (optional, may be empty; often in generation observation)

Generation Observation (MVP):

- `id` (uuid)
- `traceId` (uuid)
- `type`: fixed `GENERATION`
- `name`: e.g. `chat`
- `startTime` / `endTime` / `completionStartTime`
- `model`
- `input`: messages array (role/content)
- `output`: completion text (or structured)
- `usage`: { input, output, total, unit }
- `latency` (seconds or milliseconds; recommend ms; `docs/api.md` examples use seconds)
- `timeToFirstToken`
- `metadata` (optional)

## Python SDK (Suggested Capabilities)

### Initialization

Environment variables:

- `XTRACE_BASE_URL`
- `XTRACE_API_KEY` (Bearer token)
- `XTRACE_PROJECT_ID` (optional if server cannot derive from token)

### Async Reporting (Core)

SDK maintains:

- In-memory bounded queue
- Background worker thread/coroutine
- Batch aggregation and periodic flush

Suggested defaults (configurable):

- `queue_max_size`: 10_000
- `batch_max_size`: 100
- `flush_interval_ms`: 500
- `request_timeout_ms`: 2_000
- `max_retries`: 3 (exponential backoff)

Queue-full policy (default):

- Drop new events and count locally (expose metrics); avoid blocking business
- Optional: Block and wait (offline/batch only)

Shutdown and flush:

- `flush()`: Explicit wait for queue to drain
- `shutdown()`: Register `atexit` to attempt flush with max wait time

### OpenAI Drop-in Wrapper (Suggested)

Goal: Business code only changes import or client initialization.

Recorded content:

- Each `chat.completions.create(...)` produces one trace + one generation observation
- Stream: Record `completionStartTime` / `timeToFirstToken` on first token; write final `output` and `usage` on completion

### Failure Handling and Backpressure

- 429/5xx: Exponential backoff retry; drop and count after max retries
- 4xx (except 429): Non-retryable; drop and log reason

## Server Performance Strategy (Rust)

### Write Path

- HTTP handler: Light validation and auth only
- Push batch payload into internal channel
- Background writer:
  - Merge requests into larger batches (e.g. every 50ms or N records)
  - Single transaction + batch upsert

### Database

- Primary keys on `traces(id)`, `observations(id)`
- Index on `observations(trace_id, start_time)` (detail queries)
- Indexes on trace query fields: `project_id + timestamp`, `user_id`, `session_id`, `tags(GIN)`

### Backpressure

- Internal queue full: Return 429
- Rate limit can start at process level (later per token/project)

## Relationship to `docs/api.md`

- Public query endpoints unchanged: `/api/public/*`
- Ingest endpoints only write trace/observation to DB to support public queries
