#!/usr/bin/env python3
"""
End-to-end test: ingest user/model input-output via Bearer, then verify with Basic
(the same auth pattern Xinference uses for public reads).

Steps:
  1) POST /v1/l/batch — trace + GENERATION observation with known chat-style input/output
  2) GET /api/public/traces?name=... (Basic) — locate trace
  3) GET /api/public/traces/{id} (Basic) — assert trace + observation input/output round-trip
  4) GET /api/public/projects, /api/public/metrics/daily (Basic) — same surface as mock_xinference

Env:
  XTRACE_BASE_URL
  API_BEARER_TOKEN
  XTRACE_PUBLIC_KEY, XTRACE_SECRET_KEY
"""

from __future__ import annotations

import base64
import json
import os
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
import uuid
from datetime import datetime, timedelta, timezone


def basic_header(user: str, password: str) -> str:
    raw = f"{user}:{password}".encode("utf-8")
    return "Basic " + base64.b64encode(raw).decode("ascii")


def http_req(
    method: str,
    url: str,
    *,
    headers: dict[str, str] | None = None,
    data: bytes | None = None,
) -> tuple[int, object | None]:
    h = dict(headers or {})
    req = urllib.request.Request(url, data=data, method=method, headers=h)
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            body = resp.read().decode("utf-8")
            return resp.status, json.loads(body) if body else None
    except urllib.error.HTTPError as e:
        raw = e.read().decode("utf-8", errors="replace")
        try:
            parsed = json.loads(raw) if raw else None
        except json.JSONDecodeError:
            parsed = raw
        return e.code, parsed


def main() -> int:
    base = os.environ.get("XTRACE_BASE_URL", "http://127.0.0.1:8742").rstrip("/")
    token = os.environ.get("API_BEARER_TOKEN", "")
    pk = os.environ.get("XTRACE_PUBLIC_KEY") or os.environ.get("LANGFUSE_PUBLIC_KEY", "")
    sk = os.environ.get("XTRACE_SECRET_KEY") or os.environ.get("LANGFUSE_SECRET_KEY", "")
    if not token or not pk or not sk:
        print("Need API_BEARER_TOKEN, XTRACE_PUBLIC_KEY, XTRACE_SECRET_KEY", file=sys.stderr)
        return 2

    auth_b = basic_header(pk, sk)
    trace_name = "full-io-test"
    user_content = "E2E_USER_CONTENT_42"
    assistant_content = "E2E_ASSISTANT_REPLY_42"
    tid = str(uuid.uuid4())
    oid = str(uuid.uuid4())

    chat_input = [{"role": "user", "content": user_content}]

    batch = {
        "trace": {
            "id": tid,
            "name": trace_name,
            "userId": "e2e-user",
            "tags": ["e2e", "full-integration"],
            "input": chat_input,
            "output": assistant_content,
        },
        "observations": [
            {
                "id": oid,
                "traceId": tid,
                "type": "GENERATION",
                "name": "chat",
                "model": "e2e-model",
                "input": chat_input,
                "output": assistant_content,
                "promptTokens": 12,
                "completionTokens": 7,
                "totalTokens": 19,
            }
        ],
    }

    print("=== [ingest] POST /v1/l/batch (Bearer) ===")
    code, body = http_req(
        "POST",
        f"{base}/v1/l/batch",
        headers={
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
        },
        data=json.dumps(batch).encode("utf-8"),
    )
    if code != 200:
        print(f"FAIL batch: {code} {body}", file=sys.stderr)
        return 1
    print("    ok, waiting for worker …")
    time.sleep(2.5)

    print("\n=== [mock Xinference reads] Basic auth ===")

    code, data = http_req("GET", f"{base}/api/public/projects", headers={"Authorization": auth_b})
    print(f"GET /api/public/projects -> {code}")
    if code != 200:
        print(f"FAIL {data}", file=sys.stderr)
        return 1

    q = urllib.parse.urlencode({"name": trace_name, "page": "1", "limit": "10"})
    code, data = http_req(
        "GET",
        f"{base}/api/public/traces?{q}",
        headers={"Authorization": auth_b},
    )
    print(f"GET /api/public/traces?name={trace_name} -> {code}")
    if code != 200:
        print(f"FAIL {data}", file=sys.stderr)
        return 1
    rows = data.get("data") if isinstance(data, dict) else None
    if not rows:
        print("FAIL: no trace row for name filter (ingest not visible?)", file=sys.stderr)
        return 1
    found = None
    for r in rows:
        if r.get("id") == tid:
            found = r
            break
    if not found:
        print(f"FAIL: trace id {tid} not in list", file=sys.stderr)
        return 1
    print(f"    found trace {tid} in list")

    code, detail = http_req(
        "GET",
        f"{base}/api/public/traces/{tid}",
        headers={"Authorization": auth_b},
    )
    print(f"GET /api/public/traces/{tid} -> {code}")
    if code != 200:
        print(f"FAIL {detail}", file=sys.stderr)
        return 1

    # --- assertions: round-trip input/output ---
    def norm(v: object) -> str:
        return json.dumps(v, sort_keys=True, ensure_ascii=False)

    tin, tout = detail.get("input"), detail.get("output")
    if tin is None or (isinstance(tin, list) and len(tin) == 0):
        print("FAIL: trace.input missing", file=sys.stderr)
        return 1
    if norm(tin) != norm(chat_input):
        print(f"FAIL: trace.input mismatch:\n  got {tin}\n  exp {chat_input}", file=sys.stderr)
        return 1
    if isinstance(tout, str):
        if tout != assistant_content:
            print(f"FAIL: trace.output string mismatch: {tout!r}", file=sys.stderr)
            return 1
    elif tout is not None:
        if norm(tout) != norm(assistant_content):
            print(f"FAIL: trace.output mismatch:\n  got {tout}\n  exp {assistant_content!r}", file=sys.stderr)
            return 1

    obs = detail.get("observations") or []
    if not obs:
        print("FAIL: no observations", file=sys.stderr)
        return 1
    o0 = obs[0]
    oin, oout = o0.get("input"), o0.get("output")
    if norm(oin) != norm(chat_input):
        print(f"FAIL: observation.input mismatch:\n  got {oin}", file=sys.stderr)
        return 1
    # API may return output as string or structured
    if isinstance(oout, str):
        if oout != assistant_content:
            print(f"FAIL: observation.output mismatch: {oout!r}", file=sys.stderr)
            return 1
    else:
        if norm(oout) != norm(assistant_content):
            print(f"FAIL: observation.output mismatch:\n  got {oout}", file=sys.stderr)
            return 1

    print("    ASSERT trace.input / trace.output / observation input/output match ingest payload")

    now = datetime.now(timezone.utc)
    fts = (now - timedelta(days=1)).isoformat().replace("+00:00", "Z")
    tts = (now + timedelta(hours=1)).isoformat().replace("+00:00", "Z")
    qm = urllib.parse.urlencode(
        {"fromTimestamp": fts, "toTimestamp": tts, "page": "1", "limit": "50"}
    )
    code, data = http_req(
        "GET",
        f"{base}/api/public/metrics/daily?{qm}",
        headers={"Authorization": auth_b},
    )
    print(f"GET /api/public/metrics/daily -> {code}")
    if code != 200:
        print(f"WARN metrics/daily {data}", file=sys.stderr)

    print("\n=== Full integration test PASSED (input/output verified). ===")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except urllib.error.URLError as e:
        print(f"Network error: {e}", file=sys.stderr)
        raise SystemExit(1)
