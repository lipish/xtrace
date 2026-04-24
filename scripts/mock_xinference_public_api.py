#!/usr/bin/env python3
"""
Simulate Xinference xinference/v2/api/utils.py HTTP calls to the Langfuse-compatible host.

Matches:
  - GET {host}/api/public/metrics/daily  (+ fromTimestamp / toTimestamp like get_langfuse_daily)
  - GET {host}/api/public/traces          (+ query params like get_langfuse_traces)
  - GET {host}/api/public/traces/{id}
Auth: HTTP Basic = public_key : secret_key (same as Xinference get_langfuse_token).

Env:
  LANGFUSE_HOST or XTRACE_BASE_URL   default http://127.0.0.1:8742
  XTRACE_PUBLIC_KEY / LANGFUSE_PUBLIC_KEY
  XTRACE_SECRET_KEY / LANGFUSE_SECRET_KEY
Optional:
  VERIFY_SSL  default 1 (set 0 to mimic Xinference verify=False)
"""

from __future__ import annotations

import base64
import json
import os
import sys
import urllib.error
import urllib.parse
import urllib.request
from datetime import datetime, timedelta, timezone


def basic_header(user: str, password: str) -> str:
    raw = f"{user}:{password}".encode("utf-8")
    return "Basic " + base64.b64encode(raw).decode("ascii")


def get_json(url: str, auth: str, verify: bool) -> tuple[int, object]:
    ctx = None
    if not verify:
        import ssl

        ctx = ssl._create_unverified_context()
    req = urllib.request.Request(url, headers={"Authorization": auth})
    try:
        with urllib.request.urlopen(req, timeout=60, context=ctx) as resp:
            body = resp.read().decode("utf-8")
            return resp.status, json.loads(body) if body else None
    except urllib.error.HTTPError as e:
        err_body = e.read().decode("utf-8", errors="replace")
        try:
            parsed = json.loads(err_body) if err_body else None
        except json.JSONDecodeError:
            parsed = err_body
        return e.code, parsed


def beijing_day_range_utc_iso() -> tuple[str, str]:
    """Same window shape as get_langfuse_daily(type='days') in Xinference utils (Beijing midnight bounds)."""
    beijing_tz = timezone(timedelta(hours=8))
    now = datetime.now(beijing_tz)
    start_of_day = datetime(now.year, now.month, now.day, tzinfo=beijing_tz)
    end_of_day = start_of_day + timedelta(days=1)
    return (
        start_of_day.astimezone(timezone.utc).isoformat(),
        end_of_day.astimezone(timezone.utc).isoformat(),
    )


def main() -> int:
    host = (
        os.environ.get("LANGFUSE_HOST")
        or os.environ.get("XTRACE_BASE_URL")
        or "http://127.0.0.1:8742"
    ).rstrip("/")
    pk = os.environ.get("XTRACE_PUBLIC_KEY") or os.environ.get("LANGFUSE_PUBLIC_KEY")
    sk = os.environ.get("XTRACE_SECRET_KEY") or os.environ.get("LANGFUSE_SECRET_KEY")
    verify = os.environ.get("VERIFY_SSL", "1") not in ("0", "false", "no")

    if not pk or not sk:
        print(
            "Set XTRACE_PUBLIC_KEY and XTRACE_SECRET_KEY (or LANGFUSE_*).",
            file=sys.stderr,
        )
        return 2

    auth_h = basic_header(pk, sk)

    print(f"Host: {host}\n")

    # 1) projects — SDK auth_check
    code, data = get_json(f"{host}/api/public/projects", auth_h, verify)
    print(f"[1] GET /api/public/projects -> {code}")
    print(json.dumps(data, indent=2)[:2000])
    if code != 200:
        print("\n(Stop: fix auth or server.)", file=sys.stderr)
        return 1

    # 2) traces list — same path as Xinference get_langfuse_traces
    q = urllib.parse.urlencode({"page": "1", "limit": "5"})
    code, data = get_json(f"{host}/api/public/traces?{q}", auth_h, verify)
    print(f"\n[2] GET /api/public/traces?page=1&limit=5 -> {code}")
    print(json.dumps(data, indent=2)[:4000])

    trace_id = None
    if code == 200 and isinstance(data, dict):
        inner = data.get("data")
        if isinstance(inner, list) and inner:
            trace_id = inner[0].get("id")

    # 3) metrics daily — same query shape as get_langfuse_daily (single day window)
    fts, tts = beijing_day_range_utc_iso()
    qm = urllib.parse.urlencode(
        {"fromTimestamp": fts, "toTimestamp": tts, "page": "1", "limit": "50"}
    )
    code, data = get_json(f"{host}/api/public/metrics/daily?{qm}", auth_h, verify)
    print(f"\n[3] GET /api/public/metrics/daily (today Asia/Shanghai window) -> {code}")
    print(json.dumps(data, indent=2)[:4000])

    # 4) trace detail — get_langfuse_traces_by_trace_id
    if trace_id:
        code, data = get_json(f"{host}/api/public/traces/{trace_id}", auth_h, verify)
        print(f"\n[4] GET /api/public/traces/{trace_id} -> {code}")
        print(json.dumps(data, indent=2)[:6000])
    else:
        print("\n[4] Skip trace detail (no traces in list).")

    print("\nDone.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except urllib.error.URLError as e:
        print(f"Network error (is xtrace listening?): {e}", file=sys.stderr)
        raise SystemExit(1)
