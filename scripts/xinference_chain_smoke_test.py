#!/usr/bin/env python3
"""
Pre-flight smoke test before wiring Xinference -> xtrace.

Covers paths Xinference uses (Basic on public API) plus Bearer ingest and guards.

Env (required unless noted):
  XTRACE_BASE_URL     default http://127.0.0.1:8742
  API_BEARER_TOKEN      for POST /v1/l/batch
  XTRACE_PUBLIC_KEY / XTRACE_SECRET_KEY   same pair as xtrace + Xinference

Exit code 0 = all checks passed; non-zero = failure (see stderr).

Usage:
  export API_BEARER_TOKEN=xtrace123
  export XTRACE_PUBLIC_KEY=pk-...
  export XTRACE_SECRET_KEY=sk-...
  python3 scripts/xinference_chain_smoke_test.py
"""

from __future__ import annotations

import base64
import json
import os
import sys
import urllib.error
import urllib.parse
import urllib.request
import uuid


def basic_auth(pk: str, sk: str) -> str:
    raw = f"{pk}:{sk}".encode("utf-8")
    return "Basic " + base64.b64encode(raw).decode("ascii")


def request(
    method: str,
    url: str,
    *,
    headers: dict[str, str] | None = None,
    data: bytes | None = None,
) -> tuple[int, object | None]:
    h = dict(headers or {})
    req = urllib.request.Request(url, data=data, method=method, headers=h)
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            body = resp.read().decode("utf-8")
            return resp.status, json.loads(body) if body else None
    except urllib.error.HTTPError as e:
        raw = e.read().decode("utf-8", errors="replace")
        try:
            parsed = json.loads(raw) if raw else None
        except json.JSONDecodeError:
            parsed = raw
        return e.code, parsed


def fail(msg: str) -> None:
    print(f"FAIL: {msg}", file=sys.stderr)


def main() -> int:
    base = os.environ.get("XTRACE_BASE_URL", "http://127.0.0.1:8742").rstrip("/")
    token = os.environ.get("API_BEARER_TOKEN", "")
    pk = os.environ.get("XTRACE_PUBLIC_KEY") or os.environ.get("LANGFUSE_PUBLIC_KEY", "")
    sk = os.environ.get("XTRACE_SECRET_KEY") or os.environ.get("LANGFUSE_SECRET_KEY", "")
    if not token or not pk or not sk:
        fail("Set API_BEARER_TOKEN, XTRACE_PUBLIC_KEY, XTRACE_SECRET_KEY")
        return 2

    bad_basic = basic_auth("wrong", "wrong")
    good_basic = basic_auth(pk, sk)
    n = 0

    def step(name: str) -> None:
        nonlocal n
        n += 1
        print(f"[{n}] {name}")

    # --- unauthenticated probes ---
    step("GET /healthz (no auth)")
    code, _ = request("GET", f"{base}/healthz")
    assert code == 200, f"healthz {code}"
    print("    ok")

    step("GET /readyz (no auth)")
    code, body = request("GET", f"{base}/readyz")
    assert code == 200, f"readyz {code}"
    assert isinstance(body, dict) and body.get("status") == "ready", body
    print("    ok")

    step("GET /api/public/traces without Authorization -> 401")
    code, _ = request("GET", f"{base}/api/public/traces")
    assert code == 401, f"expected 401, got {code}"
    print("    ok")

    step("GET /api/public/traces with wrong Basic -> 401")
    code, _ = request(
        "GET",
        f"{base}/api/public/traces?page=1&limit=1",
        headers={"Authorization": bad_basic},
    )
    assert code == 401, f"expected 401, got {code}"
    print("    ok")

    # --- Xinference-style Basic reads ---
    step("GET /api/public/projects (Basic)")
    code, body = request("GET", f"{base}/api/public/projects", headers={"Authorization": good_basic})
    assert code == 200, f"projects {code}: {body}"
    assert isinstance(body, dict) and body.get("data"), body
    print("    ok")

    step("GET /api/public/traces?page=1&limit=3 (Basic)")
    code, body = request(
        "GET",
        f"{base}/api/public/traces?page=1&limit=3",
        headers={"Authorization": good_basic},
    )
    assert code == 200, f"traces {code}: {body}"
    assert isinstance(body, dict) and "data" in body and "meta" in body, body
    rows = body["data"]
    trace_id = rows[0]["id"] if rows else None
    session_for_filter = None
    for r in rows:
        sid = r.get("sessionId")
        if sid:
            session_for_filter = sid
            break
    print(f"    ok ({len(rows)} rows)")

    if trace_id:
        step(f"GET /api/public/traces/{trace_id} (Basic)")
        code, body = request(
            "GET",
            f"{base}/api/public/traces/{trace_id}",
            headers={"Authorization": good_basic},
        )
        assert code == 200, f"trace detail {code}: {body}"
        assert isinstance(body, dict) and body.get("id") == trace_id, body
        print("    ok")

    step("GET /api/public/traces unknown uuid -> 404")
    fake = "00000000-0000-4000-8000-000000000099"
    code, _ = request(
        "GET",
        f"{base}/api/public/traces/{fake}",
        headers={"Authorization": good_basic},
    )
    assert code == 404, f"expected 404, got {code}"
    print("    ok")

    if session_for_filter:
        q = urllib.parse.urlencode({"sessionId": session_for_filter, "limit": "20"})
        step(f"GET /api/public/traces?sessionId=... (Basic)")
        code, body = request(
            "GET",
            f"{base}/api/public/traces?{q}",
            headers={"Authorization": good_basic},
        )
        assert code == 200, f"session filter {code}: {body}"
        for row in body.get("data", []):
            assert row.get("sessionId") == session_for_filter, row
        print(f"    ok ({len(body.get('data', []))} rows)")

    step("GET /api/public/metrics/daily (Basic)")
    code, body = request(
        "GET",
        f"{base}/api/public/metrics/daily?page=1&limit=10",
        headers={"Authorization": good_basic},
    )
    assert code == 200, f"metrics daily {code}: {body}"
    assert isinstance(body, dict) and "data" in body, body
    print("    ok")

    # --- Bearer ingest (SDK / worker path) ---
    tid = str(uuid.uuid4())
    oid = str(uuid.uuid4())
    batch = {
        "trace": {
            "id": tid,
            "name": "smoke-test",
            "userId": "smoke",
            "tags": ["smoke", "xinference-chain"],
            "sessionId": "smoke-session-1",
        },
        "observations": [
            {
                "id": oid,
                "traceId": tid,
                "type": "GENERATION",
                "name": "llm",
                "model": "smoke-model",
                "input": {"x": 1},
                "output": "ok",
                "promptTokens": 1,
                "completionTokens": 2,
                "totalTokens": 3,
                "promptId": "smoke-prompt-id",
                "promptName": "smoke-prompt-name",
                "promptVersion": "2",
            }
        ],
    }
    step("POST /v1/l/batch (Bearer)")
    code, body = request(
        "POST",
        f"{base}/v1/l/batch",
        headers={
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
        },
        data=json.dumps(batch).encode("utf-8"),
    )
    assert code == 200, f"batch {code}: {body}"
    print("    ok (async flush ~2s)")
    import time

    time.sleep(2.5)

    step("GET /api/public/traces/{tid} after ingest (Basic)")
    code, body = request(
        "GET",
        f"{base}/api/public/traces/{tid}",
        headers={"Authorization": good_basic},
    )
    assert code == 200, f"detail after ingest {code}: {body}"
    assert body.get("sessionId") == "smoke-session-1", body
    obs = body.get("observations") or []
    assert obs, "expected observations"
    o0 = obs[0]
    assert o0.get("promptId") == "smoke-prompt-id", o0
    assert o0.get("promptName") == "smoke-prompt-name", o0
    assert o0.get("promptVersion") == 2, o0  # API returns numeric prompt version when parseable
    print("    ok (promptId / promptName / promptVersion)")

    step("POST /v1/l/batch with wrong Bearer -> 401")
    code, _ = request(
        "POST",
        f"{base}/v1/l/batch",
        headers={
            "Authorization": "Bearer definitely-wrong",
            "Content-Type": "application/json",
        },
        data=b"{}",
    )
    assert code == 401, f"expected 401, got {code}"
    print("    ok")

    print(f"\nAll {n} steps passed. Safe to proceed with Xinference chain.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except urllib.error.URLError as e:
        fail(f"cannot reach server: {e}")
        raise SystemExit(1)
    except AssertionError as e:
        fail(str(e))
        raise SystemExit(1)
