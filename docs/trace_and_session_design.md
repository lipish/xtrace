# Trace And Session Design

This document defines a forward-looking design for trace, session, multi-turn conversation, and agent execution in XGateway.

## Background

The current request log and trace pipeline is request-oriented:

- `request_logs` stores one row per upstream request.
- `xtrace` stores one trace per request and one observation per generation.
- OpenAI-compatible chat requests are stateless from the gateway perspective.

This is enough for single-request observability, but it does not reliably answer these questions:

- Which requests belong to the same multi-turn conversation?
- Which model answered each turn when models change between turns?
- Which internal steps belong to the same agent execution?
- How should an interrupted agent run be resumed?

## Goals

- Reliably identify that multiple turns belong to the same conversation.
- Make each user turn correspond to the actual model and provider that produced the answer.
- Support both plain chat and future agent workflows with one unified model.
- Separate business state from observability state.
- Allow progressive rollout without breaking the current API.

## Non-Goals

- Replacing `request_logs` entirely.
- Encoding all conversation state only inside trace records.
- Inferring conversation identity from message similarity or time windows.

## Design Principles

1. Use explicit identifiers instead of heuristics.
2. Separate conversation state from execution logs.
3. Model one user input as one turn.
4. Model agent internals as steps within a run.
5. Treat trace as an observability layer, not as the primary source of business truth.

## Conceptual Model

The recommended model has four layers.

### 1. Session

A session represents one continuous conversation context.

Examples:

- A user chatting with the gateway in a web UI.
- A client application continuing the same support conversation for 30 turns.
- An agent working on a task over several rounds.

Responsibilities:

- Multi-turn grouping.
- Session lifecycle.
- High-level state and metadata.
- Recovery anchor for future agent memory.

### 2. Turn

A turn represents one user input and the system work triggered by that input.

Examples:

- User asks a question and receives one final answer.
- User asks an agent to summarize a CSV and the system performs multiple internal steps.

Responsibilities:

- User-visible unit in the UI.
- Final answer for the turn.
- Final provider and model attribution.
- Turn-level latency and token aggregation.

### 3. Run

A run represents one execution instance for a turn.

This is most relevant for agent mode, retries, resumptions, or alternate execution strategies.

Examples:

- The first attempt of an agent plan.
- A resumed run after a tool timeout.
- A re-run requested by the user.

Responsibilities:

- Execution state machine.
- Retry and resume boundaries.
- Checkpoints and progress tracking.

### 4. Step

A step represents one internal action within a run.

Examples:

- LLM call.
- Tool call.
- Retrieval call.
- Planner step.
- Memory write.

Responsibilities:

- Fine-grained debugging.
- Detailed trace correlation.
- Accurate attribution of provider, model, tokens, and errors.

## Why Session Is Necessary

Without an explicit session identifier, the gateway can only see separate stateless requests. Even if each request includes historical `messages`, that history is only content, not identity.

That means the gateway cannot reliably distinguish between:

- The next turn of an existing conversation.
- A new conversation that happens to include similar context.
- A replayed request from another client.

For this reason, session grouping must be based on an explicit identifier such as `session_id`.

## Recommended Identifiers

### `session_id`

Stable across a multi-turn conversation.

Use cases:

- Group all turns in one conversation.
- Link conversation state and long-term context.
- Populate trace `session_id`.

### `turn_id`

Unique per user input.

Use cases:

- Show one request/answer pair in the UI.
- Aggregate all work done for that turn.
- Associate final model/provider with the answer.

### `run_id`

Unique per execution instance for a turn.

Use cases:

- Retries.
- Resume after interruption.
- Agent progress tracking.

### `step_id`

Unique per internal operation.

Use cases:

- Detailed observability.
- Tool and model call correlation.
- Step-by-step timeline rendering.

## Proposed Data Model

### Sessions

Suggested fields:

- `id`
- `session_key`
- `project_id`
- `org_id`
- `api_key_id`
- `end_user_id`
- `kind` (`chat` or `agent`)
- `title`
- `status` (`active`, `archived`, `closed`)
- `metadata` (JSON)
- `session_summary`
- `working_memory` (JSON)
- `created_at`
- `updated_at`
- `last_turn_at`

Notes:

- `session_key` is the externally supplied stable identifier.
- `working_memory` should store compact structured state, not raw full history.

### Turns

Suggested fields:

- `id`
- `session_id`
- `turn_index`
- `user_input`
- `input_payload` (JSON)
- `final_output`
- `final_provider_id`
- `final_provider_name`
- `final_model`
- `status` (`running`, `success`, `error`)
- `trace_id`
- `started_at`
- `ended_at`
- `latency_ms`
- `input_tokens`
- `output_tokens`
- `total_tokens`
- `metadata` (JSON)

Notes:

- A turn is the correct main object for the right-side details panel in the admin UI.
- The final answer shown to the user should map to `final_model` and `final_provider_name`.

### Runs

Suggested fields:

- `id`
- `turn_id`
- `run_index`
- `status` (`queued`, `running`, `waiting_tool`, `paused`, `success`, `error`, `cancelled`)
- `planner_state` (JSON)
- `checkpoint` (JSON)
- `started_at`
- `ended_at`
- `error_message`
- `metadata` (JSON)

Notes:

- For plain chat, each turn can have exactly one run.
- For agents, runs make resume and retry semantics explicit.

### Steps

Suggested fields:

- `id`
- `run_id`
- `parent_step_id`
- `step_index`
- `step_type` (`llm_call`, `tool_call`, `retrieval`, `planner`, `memory_write`, `handoff`)
- `provider_id`
- `provider_name`
- `model`
- `input_payload` (JSON)
- `output_payload` (JSON)
- `status`
- `error_message`
- `input_tokens`
- `output_tokens`
- `total_tokens`
- `trace_id`
- `observation_id`
- `started_at`
- `ended_at`
- `latency_ms`
- `metadata` (JSON)

Notes:

- A normal chat completion often maps to one `llm_call` step.
- Fallback across providers or models should create separate step records.

## Relationship To Existing Tables

### `request_logs`

Keep `request_logs`, but redefine its role as a low-level execution log.

Recommended future additions:

- `session_id`
- `turn_id`
- `run_id`
- `step_id`
- `trace_id`

This preserves backward compatibility while allowing the log table to participate in richer correlation.

### Existing `conversations` and `messages`

The repository already contains a `conversations/messages` model, but it is not currently connected to the OpenAI-compatible request path.

Recommended direction:

- Either evolve `conversations` into the canonical `sessions` layer.
- Or introduce a new `sessions` table and keep the existing conversation tables for a separate product surface.

For long-term clarity, one canonical session layer is preferable.

## API Design

## Input Contract

The gateway should support an explicit session identifier from clients.

Recommended options:

### Option A: Request Header

- `X-Session-Id`
- `X-Turn-Id` (optional)

Pros:

- Does not modify the standard OpenAI request body.
- Easier to introduce progressively.

Cons:

- Some clients are less convenient to customize at header level.

### Option B: Request Metadata

Add a structured extension field in the body, for example:

```json
{
  "model": "gpt-4o",
  "messages": [...],
  "metadata": {
    "session_id": "sess_123",
    "turn_id": "turn_456"
  }
}
```

Pros:

- More visible in debugging payloads.
- Easier to carry multiple correlation fields.

Cons:

- Less pure from an OpenAI compatibility perspective.

### Recommendation

Support both, with this precedence:

1. Request body metadata.
2. Request header.
3. Auto-generate on the gateway only when the client opts into gateway-managed sessions.

## Output Contract

The gateway should return correlation identifiers so the client can continue the same conversation.

Recommended response headers:

- `X-Session-Id`
- `X-Turn-Id`
- `X-Run-Id`

## Trace Integration

Trace should mirror the business identifiers, not replace them.

Recommended mapping:

- Trace `session_id` = gateway `session_id`
- Trace metadata includes `turn_id`, `run_id`, and `step_id`
- Trace observation per model/tool step

Recommended trace tags:

- `xgateway`
- session kind (`chat` or `agent`)
- provider name
- step type

Benefits:

- Session-wide observability in trace backends.
- Turn-level and run-level debugging without duplicating business logic into trace.

## Agent State Design

Future agent support requires durable state beyond raw message history.

Recommended durable state buckets:

- Session summary.
- Working memory.
- Planner state.
- Pending tool actions.
- Resume checkpoint.
- External artifact references.

Recommended rule:

- Persist compact structured state, not full prompt transcripts as the only recovery mechanism.

This allows:

- Resume after timeout or restart.
- Branching and retries.
- Better token efficiency.
- Clear UI for agent progress.

## UI Design Implications

### Current Limitation

The current request details panel is centered on one `request_log` row, so it cannot express a full multi-turn conversation with per-turn model changes.

### Recommended Future UI Shape

#### Session View

Shows:

- Session title.
- Session status.
- Turn list ordered by time.
- Latest summary and metadata.

#### Turn Details View

Shows:

- User input.
- Final output.
- Final provider and model.
- Total latency and tokens.
- Timeline of internal steps.

#### Step Timeline

Shows:

- Step type.
- Provider and model.
- Success or failure.
- Tool outputs.
- Retry and fallback path.

This UI can support both plain chat and future agent execution with the same conceptual model.

## Migration Strategy

### Phase 1: Introduce Correlation Fields

- Accept optional `session_id`.
- Return correlation identifiers.
- Populate trace `session_id`.
- Add nullable correlation columns to `request_logs`.

### Phase 2: Introduce Session And Turn Tables

- Create canonical `sessions` and `turns` tables.
- Persist each request as a turn.
- Backfill new logs with `turn_id`.

### Phase 3: Introduce Run And Step Tables

- Add `runs` and `steps` for agent support.
- Map fallback and retries to step records.
- Connect trace observations to step identifiers.

### Phase 4: Update Admin UI

- Shift from request-log-centric details to turn-centric details.
- Add session view and execution timeline.
- Preserve request log pages for low-level debugging.

## Tradeoffs

### Why Not Infer Sessions From Messages?

Because inference is not reliable. Identical or similar payloads can represent different conversations, and different payloads can represent the same conversation.

### Why Not Store Everything Only In Trace?

Because trace is best for observability, not for product state, durable session lifecycle, or query patterns needed by the admin UI.

### Why Keep `request_logs`?

Because the table is still useful for:

- Auditing.
- Low-level debugging.
- Compatibility with current admin pages.
- Operational analysis independent of session semantics.

## Open Questions

- Should the current `conversations` table evolve into `sessions`, or should a new canonical `sessions` table be introduced?
- Should session identifiers be opaque UUIDs generated by the gateway, or user-supplied stable IDs from clients?
- Should agent memory be stored inline on sessions, or delegated to a dedicated memory subsystem with references from sessions?
- What retention policy should apply to raw request content versus compact session state?

## Recommended Next Step

The most practical next step is to define the minimum viable version:

- Introduce `session_id`.
- Map each request to one turn.
- Populate trace `session_id`.
- Keep `request_logs` as the execution log layer.

That gives reliable multi-turn grouping immediately, while leaving room for agent runs and step timelines later.
