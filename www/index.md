---
layout: home
hero:
  name: xtrace
  text: AI Observability Service
  tagline: Collect, store, and query traces, spans, and metrics across LLM and agent workflows. Built in Rust for speed and reliability.
  actions:
    - theme: brand
      text: Get Started →
      link: /guide/getting-started
    - theme: alt
      text: View on GitHub
      link: https://github.com/lipish/xtrace
  image:
    src: /logo.svg
    alt: xtrace

features:
  - icon: 🔭
    title: Traces & Spans
    details: Full request-chain visibility with nested span trees. Debug P3 cycles (Plan → Execute → Reflect) and multi-step agent workflows.
  - icon: 📊
    title: Time-Series Metrics
    details: Ingest and query GPU utilization, KV cache, token usage, and custom metrics with label-based filtering and downsampling.
  - icon: ⚡
    title: Built in Rust
    details: Axum + Tokio async runtime, batched ingestion via mpsc channels, PostgreSQL storage. Sub-millisecond overhead on the write path.
  - icon: 🔗
    title: OpenTelemetry Compatible
    details: OTLP/HTTP ingestion endpoint with JSON and Protobuf support. Drop-in replacement for Langfuse-compatible instrumentation.
  - icon: 🏷️
    title: Multi-Dimensional Labels
    details: Attach session_id, model_name, agent_role, and arbitrary labels to metrics. Filter and group by any dimension.
  - icon: 📈
    title: Percentile Analytics
    details: Query p50/p90/p99 latency distributions. Compare performance across versions, models, and agent configurations.
---

<div class="changelog-section">

## What's New

<div class="changelog-list">

<div class="changelog-item">
  <span class="changelog-version">v0.0.14</span>
  <span class="changelog-date">Feb 2026</span>
  <div class="changelog-content">
    <strong>tracing Integration</strong> — <code>XtraceLayer</code> for <code>xtrace-client</code> (feature = "tracing"). Automatically push metrics from <code>tracing</code> events and span durations with zero manual instrumentation. Batched background flush, non-blocking hot path.
    <div class="changelog-links">
      <a href="/integrations/tracing">Integration Guide →</a>
      <a href="/sdk/rust-client">SDK Docs →</a>
    </div>
  </div>
</div>

<div class="changelog-item">
  <span class="changelog-version">v0.0.13</span>
  <span class="changelog-date">Feb 2026</span>
  <div class="changelog-content">
    <strong>Percentile Aggregation & group_by</strong> — Metrics query now supports <code>agg=p50|p90|p99</code> via PostgreSQL <code>percentile_cont</code>, and <code>group_by</code> to split time-series by any label key. Frontend dashboard with trace list and detail viewer.
    <div class="changelog-links">
      <a href="/api/metrics-api">Metrics API →</a>
    </div>
  </div>
</div>

<div class="changelog-item">
  <span class="changelog-version">v0.0.11</span>
  <span class="changelog-date">Jan 2026</span>
  <div class="changelog-content">
    <strong>Per-Token Rate Limiting</strong> — Per-bearer-token query rate limiting using the governor crate (token bucket). Configurable via <code>RATE_LIMIT_QPS</code> and <code>RATE_LIMIT_BURST</code>. Returns <code>429 Too Many Requests</code> with a <code>Retry-After</code> header.
  </div>
</div>

<div class="changelog-item">
  <span class="changelog-version">v0.0.8</span>
  <span class="changelog-date">Jan 2026</span>
  <div class="changelog-content">
    <strong>Nebula Signal Contract</strong> — Formalized metrics signal contract for Nebula (GPU cluster) integration. Node reports GPU/KV-cache/queue metrics; Router reads real-time signals for load-aware routing; Scheduler reads for placement decisions.
    <div class="changelog-links">
      <a href="/integrations/nebula">Nebula Docs →</a>
    </div>
  </div>
</div>

</div>
</div>

<style>
.changelog-section {
  max-width: 900px;
  margin: 0 auto;
  padding: 48px 24px 64px;
}

.changelog-section h2 {
  font-size: 1.6rem;
  font-weight: 700;
  margin-bottom: 32px;
  border-bottom: 1px solid var(--vp-c-divider);
  padding-bottom: 12px;
}

.changelog-list {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.changelog-item {
  display: grid;
  grid-template-columns: 90px 80px 1fr;
  gap: 0 20px;
  padding: 20px 0;
  border-bottom: 1px solid var(--vp-c-divider);
  align-items: start;
}

.changelog-item:last-child {
  border-bottom: none;
}

.changelog-version {
  font-family: var(--vp-font-family-mono);
  font-size: 0.82rem;
  font-weight: 600;
  color: var(--vp-c-brand-1);
  background: var(--vp-c-brand-soft);
  padding: 2px 8px;
  border-radius: 4px;
  white-space: nowrap;
  align-self: start;
  margin-top: 2px;
}

.changelog-date {
  font-size: 0.82rem;
  color: var(--vp-c-text-3);
  white-space: nowrap;
  align-self: start;
  margin-top: 4px;
}

.changelog-content {
  font-size: 0.95rem;
  line-height: 1.7;
  color: var(--vp-c-text-1);
}

.changelog-content strong {
  color: var(--vp-c-text-1);
}

.changelog-content code {
  font-size: 0.85em;
  background: var(--vp-c-default-soft);
  padding: 1px 5px;
  border-radius: 3px;
}

.changelog-links {
  margin-top: 8px;
  display: flex;
  gap: 16px;
}

.changelog-links a {
  font-size: 0.85rem;
  color: var(--vp-c-brand-1);
  text-decoration: none;
  font-weight: 500;
}

.changelog-links a:hover {
  text-decoration: underline;
}

@media (max-width: 640px) {
  .changelog-item {
    grid-template-columns: 1fr;
    gap: 6px;
  }
}
</style>
