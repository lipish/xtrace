# XTrace Requirements

This document defines platform-level requirements for XTrace as a general observability system for AI applications.

It focuses on reusable tracing paradigms and generic correlation mechanisms, not on any single application's business identity.

It distinguishes between Phase 1 (MVP Compatibility) and Phase 2 (First-Class Support) so that applications can adopt richer observability incrementally.

## Scope

This document covers:

- The general trace and observation paradigms supported by XTrace.
- Required identifier and metadata capabilities.
- Semantic requirements for model attribution, retry, fallback, streaming, and execution tracing.
- Privacy, reliability, and rollout constraints.
- The boundary between native platform support and application-level conventions.

This document does not define how any one application should map its domain model into XTrace. Application-specific mappings should be defined separately by the integrating system.

## Current Gaps

Today, many AI applications can emit request and generation traces into XTrace, but there are common structural gaps in how richer workflows are represented:

- `session_id` is not populated.
- Trace records are request-oriented rather than conversation-oriented.
- There is no stable `turn_id`, `run_id`, or `step_id` correlation.
- Multiple model attempts within one logical turn are not fully represented as distinct execution steps.
- Streaming traces are recorded primarily at completion time, without a complete lifecycle contract.

## Goals

- Make every trace correlate cleanly when an application chooses to use session or execution semantics.
- Make user-visible outcomes traceable to the model and provider that actually produced the result.
- Preserve visibility into fallback, retries, streaming, and failures.
- Support complex execution workflows without redesigning the trace contract.
- Keep XTrace useful as an observability system without making it the source of business truth.

## Compatibility Assessment

XTrace already has enough structural flexibility to support the first rollout of this design, but that should not be confused with full first-class support.

### What XTrace Already Supports Well

- Native `session_id` support on traces.
- Flexible `metadata` payloads on traces and observations.
- A trace-plus-observation model that can represent a turn and its internal steps.
- Additive rollout from application code without requiring an immediate schema rewrite.

This means any integrating application can begin sending `session_id`, `turn_id`, `run_id`, and `step_id` today, with `session_id` in the native trace field and the other identifiers in metadata.

### What Is Only Application-Level Convention Today

- `turn_id` is not a native top-level field.
- `run_id` is not a native top-level field.
- `step_id` is not a native top-level field.
- `parent_step_id`, `step_type`, `step_index`, and `attempt_index` are not enforced by XTrace itself.
- Querying by turn, run, or step depends on consistent metadata conventions and whatever filtering or indexing support exists above or below the trace store.

This means the current model is compatible, but the richer semantics still rely on disciplined producer-side contracts.

### Practical Interpretation

For an integrating application, the correct short-term conclusion is:

- XTrace is not a blocker.
- The correlation model can be implemented incrementally.
- Metadata schema must be treated as part of the contract, not as an implementation detail.

The correct long-term conclusion is:

- If session, turn, run, and step become core product concepts across many applications, some of these identifiers may eventually deserve stronger query, index, or UI-level support.

## Universal Tracing Paradigms

XTrace should support multiple reusable paradigms. Applications may choose one or combine several of them.

### 1. Stateless Trace Paradigm

Use this paradigm when each request or task is independent.

Characteristics:

- Each trace stands alone.
- `session_id` is optional.
- Observations describe spans, generations, tools, or sub-operations within a single request.

Typical use cases:

- One-shot chat requests.
- Single inference calls.
- Stateless API tasks.

### 2. Session Trace Paradigm

Use this paradigm when multiple traces belong to the same longer-lived interaction context.

Characteristics:

- Multiple traces share one `session_id`.
- Ordering across traces matters.
- A trace may represent one turn, request, or interaction unit.

Typical use cases:

- Multi-turn conversations.
- Support chats.
- Stateful copilots or assistants.

### 3. Execution Trace Paradigm

Use this paradigm when one logical task includes retries, tool usage, hierarchical steps, or state transitions.

Characteristics:

- `run_id` identifies one execution instance.
- `step_id` identifies atomic steps.
- `parent_step_id` can express hierarchy.
- Attempts, retries, and fallback need explicit semantics.

Typical use cases:

- Agent execution.
- Workflow orchestration.
- Multi-step automation with tools or retrieval.

## Phase Model

### Phase 1: MVP Compatibility

Phase 1 is intentionally additive. Applications should be able to send richer correlation data without waiting for a schema redesign in XTrace.

Phase 1 expectations:

- `session_id` is populated as a native trace field.
- `turn_id`, `run_id`, `step_id`, and related execution semantics are stored in metadata.
- Trace ingestion remains compatible with existing request-oriented dashboards.
- Session, turn, and execution reconstruction is possible using native fields plus metadata.

### Phase 2: First-Class Support

Phase 2 is about making common session and agent concepts easier to query, index, validate, and present in product surfaces.

Phase 2 expectations:

- High-value correlation identifiers may be promoted to stronger schema or indexing support.
- Query and UI experiences should no longer depend solely on raw metadata inspection.
- Step hierarchy and run lifecycle should become easier to reconstruct without ad hoc producer-specific logic.

## Event Model Requirements

Applications integrating with XTrace should standardize not only which fields are emitted, but also which events are emitted.

### Required Event Types

#### 1. Request Trace

Purpose:

- Represent an outer request or task lifecycle.

Requirements:

- One request trace per inbound request or task wrapper.
- Trace name identifies the route, task type, or request wrapper event.
- Includes method, status, streaming flag, and correlation identifiers when available.
- Input and output payload capture follows the configured privacy mode.

#### 2. Interaction Trace

Purpose:

- Represent one logical interaction unit.

Requirements:

- One interaction trace per logical unit chosen by the application.
- Includes `session_id`, `turn_id`, or equivalent interaction-level metadata when the application uses session semantics.
- Includes requested model and final model attribution when they differ.

#### 3. Step Observation

Purpose:

- Represent one operationally meaningful internal step.

Requirements:

- One observation per model attempt, tool call, retrieval action, planner action, memory operation, handoff, or equivalent execution step.
- Includes `step_type`, `step_id`, status, latency, provider, and model information when applicable.
- Fallbacks and retries must be represented as separate observations.

## Identifier Requirements

### Requirement 1: Trace Must Be Session-Aware

Every trace created under session semantics must carry a stable session identifier.

Required behavior:

- `TraceIngest.session_id` must be populated whenever a request belongs to a session.
- The session identifier must remain stable across all related traces within that session.

### Requirement 2: Trace Must Be Turn-Aware

Every application using turn-style interaction semantics must have a stable turn identifier.

Required behavior:

- A `turn_id` must be included in trace metadata when turn semantics are used.
- All observations produced while handling one interaction unit must carry the same `turn_id`.
- The final user-visible outcome should be attributable to exactly one `turn_id` when turn semantics are used.

### Requirement 3: Trace Must Support Execution Runs

Every application using execution semantics must have a run identifier for each execution instance.

Required behavior:

- A `run_id` must be included in trace metadata.
- Retries or resumptions for the same logical execution must produce distinct `run_id` values.
- All steps within one execution attempt must share the same `run_id`.

### Requirement 4: Trace Must Support Step-Level Correlation

Every internal operation that matters operationally under execution semantics must have a step identifier.

Required behavior:

- A `step_id` must be included in observation metadata.
- A step may optionally reference `parent_step_id` for hierarchy.
- Each fallback model attempt should be represented as a distinct step.
- Each tool call, retrieval call, planner step, or memory operation in agent mode should be represented as a distinct step.

## Metadata Contract

Applications and XTrace need a stable metadata contract even before first-class schema support exists.

### Canonical Naming Rule

XTrace should distinguish between platform-level correlation fields and application-specific extension fields.

Recommended platform-level fields without application prefixes:

- `turn_id`
- `run_id`
- `step_id`
- `parent_step_id`
- `step_type`
- `step_index`
- `attempt_index`

Recommended application extension namespaces:

- `client.*`
- `gateway.*`
- `app.*`

`session_id` remains a native trace field and should not be duplicated unless a backend-specific mirror is required.

### Required Turn Metadata

The following metadata keys should be treated as required for interaction traces unless not applicable:

- `turn_id` when turn semantics are used
- `run_id` when execution semantics are used
- `request_type` when relevant
- `path` when relevant
- `method` when relevant
- `is_stream` when relevant
- `status`
- `requested_model` when relevant
- `final_model` when known
- `final_provider_id` when known
- `final_provider_name` when known
- `request_id` if available

### Required Step Metadata

The following metadata keys should be treated as required for step observations unless not applicable:

- `turn_id` when turn semantics are used
- `run_id` when execution semantics are used
- `step_id`
- `parent_step_id` when applicable
- `step_type`
- `step_index`
- `attempt_index`
- `status`
- `is_fallback`
- `is_retry`
- `provider_id` when applicable
- `provider_name` when applicable
- `error_type` when applicable
- `error_message` when applicable

### Optional But Recommended Metadata

- `end_user_id`
- `client.name`
- `client.version`
- `app.name`
- `app.mode`
- `tool_name`
- `tool_args_schema`
- `retrieval_source`
- `memory_scope`
- `run_status`
- `planner_phase`

## Semantic Requirements

### Requirement 5: Requested Model And Actual Step Model Must Be Distinguishable

XTrace must preserve both what the client requested and what each step actually executed.

Required behavior:

- `requested_model` reflects what the client asked for.
- Observation `model` reflects the actual model used for that specific step.
- `final_model` identifies the model that produced the final user-visible or task-visible result.

This avoids introducing a redundant `actual_model` field that overlaps with observation-level `model` semantics.

### Requirement 6: Multiple Attempts Must Be Visible

If one turn involves multiple provider or model attempts, each attempt must be visible as a separate step observation.

Required behavior:

- Failed attempt to provider A is recorded as one step observation.
- Retry or fallback to provider B is recorded as another step observation.
- The final response-producing attempt is marked as the successful step that produced the answer.

### Requirement 7: Agent Step Types Must Be Standardized

Step semantics must be consistently represented so that timelines and analytics are comparable.

Required step types:

- `llm_call`
- `tool_call`
- `retrieval`
- `planner`
- `memory_write`
- `handoff`

### Requirement 8: Run Lifecycle Must Be Representable

The system must support recording the lifecycle states of a run.

Required lifecycle states:

- `queued`
- `running`
- `waiting_tool`
- `paused`
- `success`
- `error`
- `cancelled`

These states may initially live in metadata during Phase 1.

## Streaming Requirements

### Requirement 9: Streaming Must Preserve Final Attribution

For streaming responses, XTrace must still clearly attribute the final answer.

Required behavior:

- The final generation step includes the actual final model and provider.
- The trace includes total latency and time to first token when available.
- Final output content may be emitted only when the stream completes or when a configured partial-capture policy allows incremental snapshots.

### Requirement 10: Streaming Lifecycle Must Be Visible

Required behavior:

- Metadata distinguishes stream started, first token observed, stream completed, and stream aborted.
- Interrupted streams should capture partial token counts when available.
- Stream errors must be reported against the same logical step.

## Ordering And Idempotency Requirements

### Requirement 11: Ordering Must Be Good Enough For Timeline Reconstruction

Required behavior:

- Timestamps must allow reconstructing the order of steps within a turn.
- If the backend cannot guarantee ordering from timestamps alone, metadata must include `step_index` or `attempt_index`.

### Requirement 12: Trace Records Must Be Idempotent Enough For Safe Retries

Required behavior:

- Re-ingestion after network error should not create ambiguous duplicate semantics.
- Stable identifiers should be used whenever retrying the same logical event.
- Producers should avoid generating fresh semantic IDs for the same logical retry of an ingest operation.

## Privacy And Governance Requirements

### Requirement 13: Payload Capture Must Be Policy-Driven

Payload capture must be configurable.

Required behavior:

- Support full payload capture mode.
- Support sanitized payload capture mode.
- Support metadata-only mode for sensitive environments.
- Optionally support error-only capture when operators want payloads only for failures.

Recommended environment controls:

- `XTRACE_CAPTURE_MODE=full|sanitized|metadata_only|error_only`
- `XTRACE_REDACT_HEADERS`
- `XTRACE_REDACT_FIELDS`

### Requirement 14: Sensitive Fields Must Be Redactable

Required behavior:

- API keys, authorization headers, secret provider config values, and configured sensitive payload fields must never be emitted in clear text when redaction is enabled.
- Redaction must happen before enqueueing the trace payload.

## Reliability Requirements

### Requirement 15: Trace Failures Must Not Break User Traffic

Required behavior:

- XTrace ingestion must remain best-effort and non-blocking for the serving path.
- Queue overflow, backend timeout, or trace backend failure must not fail the user request.
- Failures should be logged internally with enough detail for operators.

### Requirement 16: High-Volume Step Emission Must Be Bufferable

Required behavior:

- High-volume execution loops should be supported through asynchronous buffering and batching.
- The tracing design must not require every step to synchronously block the main request path.

## Compatibility And Rollout Requirements

### Requirement 17: Existing Request-Oriented Trace Behavior Must Continue To Work During Rollout

Required behavior:

- New fields are additive.
- Existing dashboards should continue to receive trace and observation records.
- Rollout of session and execution correlation must not require breaking the current request-oriented trace contract.

### Requirement 18: Plain Chat Must Not Pay The Full Complexity Cost Of Agent Mode

Required behavior:

- Simple traces can map one interaction to one execution and one model step in the common case.
- The tracing design must support richer execution semantics without forcing unnecessary complexity into every basic trace path.

## Query And UI Requirements

### Phase 1: Basic Retrieval

- Users must be able to filter traces by `session_id`.
- Operators must be able to inspect raw metadata for turn, run, and step correlation.
- Trace and observation payloads must remain retrievable with correlation metadata intact.

### Phase 2: Product-Grade Experience

- Session views should aggregate related traces within one session chronologically.
- Execution timelines should reconstruct step hierarchy and ordering without raw JSON inspection as the only workflow.
- Query performance for core correlation identifiers should be good enough for product-grade retrieval and debugging.

This document deliberately does not require a specific complexity target such as O(1), because actual performance depends on schema, indexing, storage engine behavior, and workload shape.

## Suggested Canonical Metadata Shape

```json
{
  "turn_id": "turn_008",
  "run_id": "run_001",
  "step_id": "step_002",
  "parent_step_id": "step_001",
  "request_type": "chat",
  "step_type": "llm_call",
  "path": "/v1/chat/completions",
  "method": "POST",
  "is_stream": true,
  "requested_model": "gpt-4o",
  "final_model": "kimi-k2.5",
  "provider_id": 5,
  "provider_name": "Provider A",
  "step_index": 2,
  "attempt_index": 2,
  "status": "success",
  "is_fallback": true,
  "is_retry": false,
  "error_type": null,
  "error_message": null,
  "client.name": "demo-client",
  "app.mode": "conversation"
}
```

Notes:

- `session_id` should be carried in the native trace field.
- Observation-level `model` should hold the actual model used by that step.
- `final_model` should identify the model that produced the final visible result for the application.

## Recommended Naming Conventions

- Trace name for interaction traces: `chat.turn`
- Trace name for request-level wrapper events: `request.trace`
- Observation name for model generation: `chat.generation`
- Observation name for tool call: `agent.tool`
- Observation name for planner action: `agent.plan`
- Observation name for retrieval: `agent.retrieval`
- Observation name for memory write: `agent.memory`

## Acceptance Criteria

The XTrace integration should be considered complete for the first rollout when all of the following are true:

1. All traces using session semantics include native `session_id` plus stable correlation metadata.
2. All model attempts for one logical execution are visible as distinct observations.
3. Requested model and observation-level actual model are both preserved.
4. Streaming traces include final attribution and time to first token when available.
5. Request-level trace failures do not affect user traffic.
6. A session or execution timeline can be reconstructed using trace data plus application records.

## Minimum Viable Rollout

The minimum viable version of the XTrace upgrade should include:

1. Populate `session_id` on traces.
2. Add `turn_id` and `requested_model` to trace metadata when the application uses those paradigms.
3. Emit one observation per actual model attempt.
4. Distinguish final model from requested model.
5. Carry streaming TTFT and completion status consistently.

This is enough to make session correlation and execution attribution substantially better before fuller first-class support is implemented.