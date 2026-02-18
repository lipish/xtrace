# Zene Agent Framework — xtrace Integration Suggestions

Collected from Zene team feedback. Prioritized by implementation value and feasibility.

## P0: tracing::Layer for xtrace-client

Wrap `xtrace-client` as a `tracing::Layer` so the entire Rust ecosystem can report metrics without manual API calls.

- Intercept `on_event` to extract events with `metric` fields
- Intercept `on_close` to auto-compute span latency
- Internal batch queue with async flush
- Ship as a feature gate: `xtrace-client = { features = ["tracing"] }`
- Solves the "GlobalReporter" problem — with a Layer, any `tracing::info!()` auto-reports

Ideal API:

```rust
#[tracing::instrument(skip(self), fields(session_id = %session.id))]
async fn execute_tool(&self, ...) {
    let _metrics = xtrace::track_scope("tool_execution");
    // ... execution logic ...
}
```

## P0: model/provider Label Convention (Zene-side)

Standardize that all token metrics carry `model` and `provider` labels.

```json
{
  "name": "zene_total_tokens",
  "labels": {"model": "gpt-4o", "provider": "openai"},
  "value": 1523
}
```

No xtrace changes needed — existing metrics API supports arbitrary labels.

## P1: Tool-level Span Instrumentation (Zene-side)

Create child observations per tool call in ToolHandler/Executor.

```json
{
  "name": "zene_tool_latency",
  "labels": {"tool_name": "grep", "status": "success"},
  "value": 0.342
}
```

xtrace already supports nested spans via `parentObservationId`. Zene just needs to emit child spans.

## P1: Error Attribution Metrics (Zene-side)

Report error frequency using the ZeneError type system.

```json
{
  "name": "zene_errors_total",
  "labels": {"error_type": "McpError", "code": "CONNECTION_REFUSED"},
  "value": 1
}
```

Use existing `POST /v1/metrics/batch`. Alerting should be handled externally (not inside xtrace).

## P2: Custom Metrics Channel (Zene-side)

Allow users to inject custom business metrics via `AgentEvent::CustomMetric` or RunRequest context fields. This is a Zene framework design concern — xtrace's metrics batch API is already generic enough.

## Summary

| Priority | Item | Owner | xtrace Change |
|----------|------|-------|--------------|
| P0 | tracing::Layer | xtrace-client | New feature gate |
| P0 | model/provider labels | Zene | None |
| P1 | Tool-level spans | Zene | None |
| P1 | Error metrics | Zene | None |
| P2 | Custom metrics channel | Zene | None |
