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
