import os
import sys
from datetime import datetime, timezone

from langfuse import Langfuse
from langfuse.types import TraceContext


def _require_env(name: str) -> str:
    v = os.environ.get(name)
    if not v:
        raise RuntimeError(f"missing env {name}")
    return v


def main() -> int:
    host = _require_env("LANGFUSE_HOST").rstrip("/")
    public_key = _require_env("LANGFUSE_PUBLIC_KEY")
    secret_key = _require_env("LANGFUSE_SECRET_KEY")

    langfuse = Langfuse(
        host=host,
        public_key=public_key,
        secret_key=secret_key,
        debug=True,
        tracing_enabled=True,
    )

    print(f"LANGFUSE_HOST={host}")

    ok = langfuse.auth_check()
    print(f"auth_check={ok}")

    with langfuse.start_as_current_span(
        name="xinference.chat",
        input={"messages": [{"role": "user", "content": "hello"}]},
        metadata={"mock": True, "component": "restful_api"},
    ) as span:
        trace_id = langfuse.get_current_trace_id()
        obs_id = langfuse.get_current_observation_id()
        print(f"trace_id={trace_id}")
        print(f"observation_id={obs_id}")

        span.update_trace(
            name="xinference.chat.trace",
            user_id="mock-user",
            session_id="mock-session",
            tags=["mock", "xinference"],
            metadata={"trace_meta": 1},
        )

        span.update(
            metadata={"span_meta": 1},
            output={"partial": "ok"},
        )

        with langfuse.start_as_current_generation(
            name="chat",
            trace_context=TraceContext(trace_id=trace_id, parent_span_id=obs_id),
        ) as generation:
            generation.update(
                model="mock-model",
                input=[{"role": "user", "content": "hello"}],
                metadata={"raw_params": {"temperature": 0.7}},
            )

            generation.update(
                completion_start_time=datetime.now(timezone.utc),
                output={"role": "assistant", "content": "world"},
                usage_details={"promptTokens": 2, "completionTokens": 2, "totalTokens": 4},
            )

    if hasattr(langfuse, "flush"):
        langfuse.flush()

    if hasattr(langfuse, "shutdown"):
        langfuse.shutdown()

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        raise
