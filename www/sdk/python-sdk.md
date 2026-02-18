# Python SDK

xtrace can be used from Python through two approaches:

## Option 1: Langfuse Python SDK (Recommended)

Since xtrace is API-compatible with Langfuse, you can use the Langfuse Python SDK directly by pointing it at your xtrace instance.

### Installation

```bash
pip install langfuse
```

### Configuration

```python
import os
os.environ["LANGFUSE_HOST"] = "http://127.0.0.1:8742"
os.environ["LANGFUSE_PUBLIC_KEY"] = "pk-xxx"  # your XTRACE_PUBLIC_KEY
os.environ["LANGFUSE_SECRET_KEY"] = "sk-yyy"  # your XTRACE_SECRET_KEY
```

### Basic Usage

```python
from langfuse import Langfuse

langfuse = Langfuse()

# Create a trace
trace = langfuse.trace(
    name="chat-request",
    user_id="alice",
    session_id="sess-001",
    tags=["prod"],
)

# Record a generation
generation = trace.generation(
    name="llm-call",
    model="gpt-4",
    input=[{"role": "user", "content": "Hello"}],
    output="Hi there!",
    usage={"input": 5, "output": 12, "total": 17},
)

# Flush before exit
langfuse.flush()
```

### Agent Workflow Tracing

For agent frameworks like Zene, use nested spans to trace P3 cycles:

```python
trace = langfuse.trace(
    name="agent-task",
    version="v2.1",
    metadata={"task": "research"},
)

# P3 Cycle 1
cycle = trace.span(name="P3-Cycle-1")

plan = cycle.generation(
    name="Plan",
    model="gpt-4",
    metadata={"agent_role": "Planner"},
    input=[{"role": "user", "content": "Plan the research steps"}],
    output="Step 1: Search for...",
)

execute = cycle.span(name="Execute")
tool_call = execute.span(name="web-search")
# ... tool execution ...

reflect = cycle.generation(
    name="Reflect",
    model="gpt-4",
    metadata={"agent_role": "Planner"},
    input=[{"role": "user", "content": "Evaluate results"}],
    output="Results are satisfactory",
)

langfuse.flush()
```

### OpenAI Integration

```python
from langfuse.openai import openai

# Automatically traces all OpenAI calls
response = openai.chat.completions.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Hello"}],
)
```

## Option 2: Direct HTTP API

For custom Python instrumentation without the Langfuse SDK:

```python
import requests
import uuid
from datetime import datetime, timezone

BASE_URL = "http://127.0.0.1:8742"
TOKEN = "your-api-token"
HEADERS = {
    "Authorization": f"Bearer {TOKEN}",
    "Content-Type": "application/json",
}

# Ingest a trace with observation
trace_id = str(uuid.uuid4())
obs_id = str(uuid.uuid4())
now = datetime.now(timezone.utc).isoformat()

requests.post(f"{BASE_URL}/v1/l/batch", headers=HEADERS, json={
    "trace": {
        "id": trace_id,
        "timestamp": now,
        "name": "chat",
        "userId": "alice",
    },
    "observations": [{
        "id": obs_id,
        "traceId": trace_id,
        "type": "GENERATION",
        "name": "llm-call",
        "model": "gpt-4",
        "startTime": now,
        "promptTokens": 10,
        "completionTokens": 50,
        "totalTokens": 60,
    }],
})

# Push metrics
requests.post(f"{BASE_URL}/v1/metrics/batch", headers=HEADERS, json={
    "metrics": [{
        "name": "token_usage",
        "labels": {"model_name": "gpt-4", "agent_role": "Planner"},
        "value": 60,
        "timestamp": now,
    }],
})

# Query traces
response = requests.get(
    f"{BASE_URL}/api/public/traces",
    headers=HEADERS,
    params={"page": 1, "limit": 10},
)
print(response.json())
```

## Design Principles

The xtrace ingest pipeline is designed for minimal latency impact:

- **Async-first**: Use background threads or async tasks for reporting
- **Batched**: Collect events and flush periodically (e.g., every 500ms)
- **Controlled discard**: Under load, drop events rather than block the main request
- **Idempotent**: Safe to retry with the same trace/observation IDs
