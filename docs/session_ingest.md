# Session-Aware Ingest and Verification

This document explains how xtrace models session-oriented interactions (chat turns and agent executions), how metadata is propagated through ingest, and how to validate the full pipeline.

## Why this exists

In multi-turn AI applications, a single user interaction can span:

- a session (conversation lifecycle)
- multiple turns (user/assistant exchanges)
- one or more execution runs (agent plans)
- multiple steps (tool calls, model calls, planners)

If these identifiers are not persisted consistently, trace search and debugging become fragmented.

## Metadata model

xtrace stores these identifiers in trace and/or observation metadata:

- `session_id`: conversation-level grouping key
- `turn_id`: single chat turn grouping key
- `run_id`: agent execution grouping key
- `step_id`: execution step key
- `step_type`: step kind (e.g. `tool_call`)
- optional step context such as `tool_name`

Recommended conventions:

- `session_id` should remain stable for one conversation
- `turn_id` should be unique per user turn
- `run_id` should be unique per agent execution branch
- `step_id` should be unique within one run

## SDK-side instrumentation flow

The Python SDK wrapper (`observe_openai`) attaches identifiers and custom metadata when invoking OpenAI-like clients.

A practical example is provided in:

- `scripts/verify_session_ingest.py`

The script simulates two patterns:

1. Chat turn instrumentation (`session_id + turn_id`)
2. Agent run instrumentation (`session_id + turn_id + run_id + step metadata`)

Then it flushes buffered events and verifies persisted data via trace APIs.

## End-to-end verification procedure

Prerequisites:

- xtrace server running locally
- API token configured
- Python SDK available in the current environment

Run:

```bash
python scripts/verify_session_ingest.py
```

What it verifies:

1. Writes one normal chat turn with `turn_id`
2. Writes one agent run with `run_id`, and one step with `step_id` / `step_type`
3. Calls `GET /api/public/traces?sessionId=...` to locate traces in the same session
4. Calls `GET /api/public/traces/:traceId` to check observation-level step metadata

Expected outcome:

- Turn trace can be found by `turn_id`
- Agent trace can be found by `run_id`
- At least one observation contains matching `step_id` and `step_type`

## Query and debugging tips

If session-linked data is missing:

1. Ensure identifiers are passed to `observe_openai(...)`
2. Ensure `client.flush()` is called before process exit
3. Check ingest auth token and endpoint correctness
4. Re-query by `sessionId` and inspect trace detail metadata/observations

## API touchpoints used in verification

- `GET /api/public/traces` (with `sessionId` filter)
- `GET /api/public/traces/:traceId`

These endpoints are sufficient to verify session-turn-run-step continuity in the current model.
