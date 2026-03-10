# Session-Aware Ingest

This page describes how xtrace models session-oriented workflows (chat turns and agent executions), and how to verify end-to-end metadata continuity.

## Problem

In multi-turn AI systems, one user journey can span multiple layers:

- a session (conversation lifecycle)
- multiple turns
- one or more agent runs
- multiple execution steps

Without stable identifiers, trace discovery and debugging become fragmented.

## Metadata model

xtrace persists the following fields in trace and/or observation metadata:

- `session_id`: conversation-level key
- `turn_id`: per-turn key
- `run_id`: per-agent-run key
- `step_id`: per-step key inside a run
- `step_type`: step category (for example `tool_call`)
- optional context such as `tool_name`

Recommended conventions:

- Keep `session_id` stable for one conversation
- Use unique `turn_id` for each turn
- Use unique `run_id` per execution branch
- Use unique `step_id` within each run

## Python SDK instrumentation

The Python wrapper `observe_openai(...)` can attach these IDs when calling OpenAI-like clients.

Reference script:

- `scripts/verify_session_ingest.py`

The script simulates:

1. Chat turn instrumentation (`session_id + turn_id`)
2. Agent run instrumentation (`session_id + turn_id + run_id + step metadata`)

Then it flushes events and verifies data through query APIs.

## End-to-end verification

Prerequisites:

- xtrace server is running
- API token is configured
- Python SDK is available

Run:

```bash
python scripts/verify_session_ingest.py
```

Verification checks:

1. A trace can be found by `turn_id`
2. A trace can be found by `run_id`
3. At least one observation contains matching `step_id` and `step_type`

## APIs used in verification

- `GET /api/public/traces?sessionId=...`
- `GET /api/public/traces/:traceId`

If session-linked data is missing, first verify ID propagation, then verify `client.flush()`, auth token, endpoint, and query filters.
