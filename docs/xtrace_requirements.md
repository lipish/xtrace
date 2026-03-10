# XTrace Requirements for Session & Agent Observability

This document defines the requirements for XTrace to support the Session-Turn-Run-Step model defined in [Trace And Session Design](trace_and_session_design.md).

It distinguishes between **Phase 1 (MVP Compatibility)** and **Phase 2 (First-Class Support)** to guide the implementation roadmap.

## 1. Data Model Requirements

### Phase 1: MVP Compatibility (Storage Only)

XTrace must allow clients to persist the hierarchical structure using existing schemas.

- **Session ID**: MUST be supported as a top-level field on Traces.
- **Correlation IDs**: `turn_id`, `run_id`, and `step_id` MUST be accepted within the `metadata` JSON blob of Traces and Observations.
- **Metadata Flexibility**: The `metadata` field MUST accept arbitrary JSON structures without schema validation errors.

### Phase 2: First-Class Support (Native Schema)

To support efficient querying and semantic understanding, XTrace SHOULD evolve its schema:

- **Native Indexing**: `turn_id` and `run_id` SHOULD be indexed fields (or promoted to top-level columns) to enable O(1) lookup and efficient aggregation.
- **Step Hierarchy**: `step_id` and `parent_step_id` SHOULD be natively understood to reconstruct execution trees without parsing full trace blobs.

## 2. Semantic Paradigms (Optional & Composable)

XTrace defines composable observability paradigms. Applications should choose the identifiers that match their domain.

### Paradigm A: Execution (Universal)
Applies to any AI task, whether it's a chat bot, a background job, or a function call.

- **Run ID**: Represents a single execution instance (including retries).
- **Step ID**: Represents an atomic operation within a Run.
- **Step Types**: Observations MUST support a strict enumeration of step types in metadata or a dedicated field:
  - `llm_call`: Standard model generation.
  - `tool_call`: Execution of an external tool/function.
  - `retrieval`: RAG or memory lookup operations.
  - `planner`: Reasoning or planning steps.
  - `memory_write`: State persistence operations.
  - `handoff`: Delegation to another agent or human.

- **Run Lifecycle**: The system MUST support recording the state transitions of a Run:
  - `queued` -> `running` -> (`waiting_tool` <-> `running`) -> `success` / `error` / `cancelled`.
  - `paused`: Explicit state for interrupted/suspended agents.

- **Retry & Fallback**:
  - **Retries**: Multiple attempts for the same logical step MUST be linked (e.g., sharing a `logical_step_id` while having unique `attempt_id`s).
  - **Fallbacks**: When a provider fails and another is tried, both MUST be recorded, with the failed attempt clearly marked as non-final.

### Paradigm B: Session Context (Stateful)
Applies to applications that maintain state over time.

- **Session ID**: A stable identifier linking multiple interactions.

### Paradigm C: Conversational (Chat-Specific)
Applies specifically to multi-turn user dialogues.

- **Turn ID**: A unique identifier for a single user interaction turn.
  *Note: Non-chat applications (e.g., batch processing) may skip this level.*

### Model Attribution

- **Requested vs. Actual**: The trace MUST distinguish between:
  - `requested_model`: The model ID requested by the user/client.
  - `actual_model`: The specific model ID served (e.g., after routing or fallback).
  - `final_model`: The model attributed for billing and analytics.

## 3. Query & UI Requirements

### Phase 1: Basic Retrieval

- **Session Filter**: Users MUST be able to filter traces by `session_id`.
- **Raw Inspection**: Users MUST be able to view the raw `metadata` JSON to inspect Turn/Run/Step IDs.

### Phase 2: Product-Grade Experience

- **Session View**: A dedicated UI view aggregating all Traces (Turns) belonging to a Session, ordered chronologically.
- **Agent Timeline**: A Gantt-chart or tree view visualizing the `step_id` / `parent_step_id` relationships within a Run.
- **State Reconstruction**: The API SHOULD support fetching the latest state of a Session (e.g., "Show me the last successful Step of the current Run").

## 4. Non-Functional Requirements

### Privacy & Governance

- **Payload Capture Modes**:
  - `full`: Capture all inputs and outputs.
  - `metadata_only`: Capture only token counts, latency, and model IDs; drop payloads.
  - `error_only`: Capture payloads only on error status.
  - `pii_masked`: Apply PII redaction rules before storage.
  
- **Desensitization**: Sensitive headers (e.g., Authorization) and configured metadata keys MUST be redacted before persistence.

### Reliability

- **Non-Blocking**: Trace ingestion failures MUST NOT impact the critical path of the gateway's request processing.
- **Async Ingestion**: High-volume agent steps (e.g., loops) SHOULD be buffered and ingested asynchronously.
- **Streaming Lifecycle**:
  - For streaming responses, XTrace MUST capture the `ttfb` (Time to First Token) and total latency.
  - Interrupted streams MUST be recorded with partial token counts and a specific status (e.g., `cancelled_by_user`).

## 5. Integration Contract (Naming Conventions)

To ensure **generality**, we distinguish between **Platform Reserved Fields** (which XTrace may use for native features) and **Application Context Fields**.

### Platform Reserved Fields (NO Prefix)

These keys map to XTrace's core data model paradigms.

- `session_id` (Native)
- `run_id`
- `step_id`
- `parent_step_id`
- `step_type`
- `attempt_index`
- `turn_id` (Optional, for chat apps)

### Application Context Fields (Prefix Recommended)

Keys describing the runtime environment or specific application logic.

- `gateway.*` (e.g., `gateway.route_id`)
- `client.*` (e.g., `client.app_version`)
- `app.*` (e.g., `app.user_tier`)

### Recommended Trace/Observation Names

- Trace Name: `chat.turn`
- Observation Names:
  - `chat.generation`
  - `agent.tool`
  - `agent.plan`
  - `agent.retrieval`
  - `agent.memory`

### Client Responsibility

- **ID Generation**: The client is responsible for generating globally unique `session_id`, `turn_id`, `run_id`, and `step_id`.
- **State Management**: The client manages the business logic of "what is a session"; XTrace purely records the provided IDs.

### Server Responsibility

- **Storage Fidelity**: XTrace guarantees that all provided correlation IDs and metadata are stored and retrievable.
- **Query Performance**: XTrace ensures that queries by `session_id` remain performant as dataset size grows.
