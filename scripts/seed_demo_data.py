#!/usr/bin/env python3
"""
Push richer demo data into xtrace for local / integration testing.

Scenarios covered:
  - Multiple traces with GENERATION observations (different users, tags, names)
  - Shared session_id (multi-turn style)
  - externalId + bookmarked
  - Traces on different calendar days (for GET /api/public/metrics/daily buckets)
  - Optional time-series points via POST /v1/metrics/batch

Requires Bearer auth (same as normal API). Run AFTER xtrace is up.

Env:
  XTRACE_BASE_URL   default http://127.0.0.1:8742
  API_BEARER_TOKEN    required (must match xtrace process)
"""

from __future__ import annotations

import json
import os
import sys
import time
import urllib.error
import urllib.request
import uuid
from datetime import datetime, timedelta, timezone


def post_json(path: str, body: object, token: str, base: str) -> tuple[int, object | None]:
    url = base.rstrip("/") + path
    data = json.dumps(body).encode("utf-8")
    req = urllib.request.Request(
        url,
        data=data,
        method="POST",
        headers={
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
        },
    )
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            raw = resp.read().decode("utf-8")
            return resp.status, json.loads(raw) if raw else None
    except urllib.error.HTTPError as e:
        err = e.read().decode("utf-8", errors="replace")
        try:
            parsed = json.loads(err)
        except json.JSONDecodeError:
            parsed = err
        return e.code, parsed


def uid() -> str:
    return str(uuid.uuid4())


def iso(dt: datetime) -> str:
    return dt.replace(tzinfo=timezone.utc).isoformat().replace("+00:00", "Z")


def main() -> int:
    base = os.environ.get("XTRACE_BASE_URL", "http://127.0.0.1:8742")
    token = os.environ.get("API_BEARER_TOKEN", "")
    if not token:
        print("Set API_BEARER_TOKEN (must match xtrace).", file=sys.stderr)
        return 2

    now = datetime.now(timezone.utc)
    yesterday = now - timedelta(days=1)
    session_a = "demo-session-" + uid()[:8]

    batches: list[dict] = []

    # 1) Standard chat + generation
    t1 = uid()
    batches.append(
        {
            "trace": {
                "id": t1,
                "timestamp": iso(now),
                "name": "chat-demo",
                "userId": "administrator",
                "tags": ["demo", "xinference"],
                "sessionId": session_a,
                "latency": 0.42,
                "totalCost": 0.001,
            },
            "observations": [
                {
                    "id": uid(),
                    "traceId": t1,
                    "type": "GENERATION",
                    "name": "llm",
                    "startTime": iso(now),
                    "endTime": iso(now + timedelta(milliseconds=420)),
                    "model": "qwen-demo",
                    "input": [{"role": "user", "content": "hello xtrace"}],
                    "output": "Hello from seeded data.",
                    "promptTokens": 10,
                    "completionTokens": 20,
                    "totalTokens": 30,
                    # Langfuse-style prompt linkage (stored on observation, not Prompt Management API)
                    "promptId": "seed-prompt-001",
                    "promptName": "demo-chat-template",
                    "promptVersion": "3",
                }
            ],
        }
    )

    # 2) Same session, second trace (multi-turn)
    t2 = uid()
    batches.append(
        {
            "trace": {
                "id": t2,
                "timestamp": iso(now + timedelta(seconds=2)),
                "name": "chat-demo",
                "userId": "administrator",
                "tags": ["demo", "xinference"],
                "sessionId": session_a,
            },
            "observations": [
                {
                    "id": uid(),
                    "traceId": t2,
                    "type": "GENERATION",
                    "name": "llm",
                    "startTime": iso(now + timedelta(seconds=2)),
                    "endTime": iso(now + timedelta(seconds=3)),
                    "model": "qwen-demo",
                    "input": [{"role": "user", "content": "follow-up"}],
                    "output": "Second turn.",
                    "promptTokens": 5,
                    "completionTokens": 8,
                    "totalTokens": 13,
                }
            ],
        }
    )

    # 3) Different user + tags (filtering tests)
    t3 = uid()
    batches.append(
        {
            "trace": {
                "id": t3,
                "timestamp": iso(now),
                "name": "embedding-job",
                "userId": "alice",
                "tags": ["batch", "embed"],
            },
            "observations": [
                {
                    "id": uid(),
                    "traceId": t3,
                    "type": "GENERATION",
                    "name": "embed",
                    "startTime": iso(now),
                    "endTime": iso(now + timedelta(milliseconds=50)),
                    "model": "bge-m3",
                    "input": "chunk text",
                    "output": "[0.1, 0.2, ...]",
                    "promptTokens": 100,
                    "completionTokens": 0,
                    "totalTokens": 100,
                }
            ],
        }
    )

    # 4) Bookmark + external id
    t4 = uid()
    batches.append(
        {
            "trace": {
                "id": t4,
                "timestamp": iso(now),
                "name": "bookmarked-trace",
                "userId": "bob",
                "tags": ["demo"],
                "externalId": "ext-seed-001",
                "bookmarked": True,
            },
            "observations": [
                {
                    "id": uid(),
                    "traceId": t4,
                    "type": "GENERATION",
                    "name": "llm",
                    "startTime": iso(now),
                    "endTime": iso(now + timedelta(milliseconds=200)),
                    "model": "gpt-demo",
                    "input": {"prompt": "ping"},
                    "output": "pong",
                    "promptTokens": 2,
                    "completionTokens": 1,
                    "totalTokens": 3,
                }
            ],
        }
    )

    # 5) Yesterday (second day bucket for metrics/daily)
    t5 = uid()
    batches.append(
        {
            "trace": {
                "id": t5,
                "timestamp": iso(yesterday),
                "name": "yesterday-job",
                "userId": "administrator",
                "tags": ["demo", "history"],
            },
            "observations": [
                {
                    "id": uid(),
                    "traceId": t5,
                    "type": "GENERATION",
                    "name": "llm",
                    "startTime": iso(yesterday),
                    "endTime": iso(yesterday + timedelta(seconds=1)),
                    "model": "qwen-demo",
                    "input": "old",
                    "output": "old reply",
                    "promptTokens": 3,
                    "completionTokens": 4,
                    "totalTokens": 7,
                }
            ],
        }
    )

    ok = 0
    for i, body in enumerate(batches, 1):
        code, resp = post_json("/v1/l/batch", body, token, base)
        if code == 200:
            ok += 1
            print(f"[batch {i}/{len(batches)}] OK")
        else:
            print(f"[batch {i}/{len(batches)}] FAILED {code}: {resp}", file=sys.stderr)

    # Time-series metrics (optional dashboard / metrics API)
    metrics_body = {
        "metrics": [
            {
                "name": "xinference:input_tokens_total",
                "labels": {"model": "qwen-demo", "user_id": "administrator"},
                "value": 42.0,
                "timestamp": iso(now),
            },
            {
                "name": "gpu_utilization",
                "labels": {"node_id": "node-1", "gpu_index": "0"},
                "value": 77.5,
                "timestamp": iso(now),
            },
        ]
    }
    code, resp = post_json("/v1/metrics/batch", metrics_body, token, base)
    if code == 200:
        print("[metrics] OK")
    else:
        print(f"[metrics] FAILED {code}: {resp}", file=sys.stderr)

    print(f"\nIngested {ok}/{len(batches)} trace batches. Waiting for worker flush...")
    time.sleep(2)
    print("Done. Re-run: python3 scripts/mock_xinference_public_api.py")
    return 0 if ok == len(batches) else 1


if __name__ == "__main__":
    raise SystemExit(main())
