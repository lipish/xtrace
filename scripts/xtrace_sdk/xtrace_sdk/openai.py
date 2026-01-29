import time
from typing import Any, Dict, Iterable, Optional

from .client import XTraceClient


def observe_openai(
    client: Any,
    *,
    xtrace: XTraceClient,
    name: Optional[str] = None,
    user_id: Optional[str] = None,
    session_id: Optional[str] = None,
    tags: Optional[list[str]] = None,
    metadata: Optional[dict] = None,
    project_id: Optional[str] = None,
) -> Any:
    return _OpenAIClientWrapper(
        client,
        xtrace=xtrace,
        name=name,
        user_id=user_id,
        session_id=session_id,
        tags=tags or [],
        metadata=metadata or {},
        project_id=project_id,
    )


class _OpenAIClientWrapper:
    def __init__(
        self,
        client: Any,
        *,
        xtrace: XTraceClient,
        name: Optional[str],
        user_id: Optional[str],
        session_id: Optional[str],
        tags: list[str],
        metadata: dict,
        project_id: Optional[str],
    ) -> None:
        self._client = client
        self._xtrace = xtrace
        self._name = name
        self._user_id = user_id
        self._session_id = session_id
        self._tags = tags
        self._metadata = metadata
        self._project_id = project_id

    @property
    def chat(self) -> Any:
        return _ChatWrapper(self)

    def __getattr__(self, item: str) -> Any:
        return getattr(self._client, item)


class _ChatWrapper:
    def __init__(self, root: _OpenAIClientWrapper) -> None:
        self._root = root

    @property
    def completions(self) -> Any:
        return _ChatCompletionsWrapper(self._root)


class _ChatCompletionsWrapper:
    def __init__(self, root: _OpenAIClientWrapper) -> None:
        self._root = root

    def create(self, *args: Any, **kwargs: Any) -> Any:
        trace_id = self._root._xtrace.new_id()
        obs_id = self._root._xtrace.new_id()

        start = time.time()
        stream = bool(kwargs.get("stream", False))

        result = self._root._client.chat.completions.create(*args, **kwargs)

        if stream:
            return _StreamWrapper(
                result,
                root=self._root,
                trace_id=trace_id,
                obs_id=obs_id,
                start=start,
                kwargs=kwargs,
            )

        latency = time.time() - start
        _record_non_stream(
            root=self._root,
            trace_id=trace_id,
            obs_id=obs_id,
            start=start,
            latency=latency,
            response=result,
            kwargs=kwargs,
        )
        return result


class _StreamWrapper:
    def __init__(
        self,
        inner: Iterable[Any],
        *,
        root: _OpenAIClientWrapper,
        trace_id: str,
        obs_id: str,
        start: float,
        kwargs: Dict[str, Any],
    ) -> None:
        self._inner = iter(inner)
        self._root = root
        self._trace_id = trace_id
        self._obs_id = obs_id
        self._start = start
        self._kwargs = kwargs
        self._ttfb: Optional[float] = None
        self._output_parts: list[str] = []
        self._usage: Optional[Dict[str, Any]] = None

    def __iter__(self):
        return self

    def __next__(self):
        try:
            chunk = next(self._inner)
        except StopIteration:
            self._finalize()
            raise
        else:
            delta = _extract_stream_delta_text(chunk)
            if delta:
                if self._ttfb is None:
                    self._ttfb = time.time() - self._start
                self._output_parts.append(delta)

            usage = _extract_stream_usage(chunk)
            if usage is not None:
                self._usage = usage
            return chunk

    def close(self) -> None:
        self._finalize()

    def _finalize(self) -> None:
        latency = time.time() - self._start
        output_text = "".join(self._output_parts) if self._output_parts else None

        payload = _build_payload(
            root=self._root,
            trace_id=self._trace_id,
            obs_id=self._obs_id,
            start=self._start,
            latency=latency,
            ttfb=self._ttfb,
            output_text=output_text,
            usage=self._usage,
            kwargs=self._kwargs,
        )
        self._root._xtrace.enqueue_batch(payload)


def _record_non_stream(
    *,
    root: _OpenAIClientWrapper,
    trace_id: str,
    obs_id: str,
    start: float,
    latency: float,
    response: Any,
    kwargs: Dict[str, Any],
) -> None:
    usage = _extract_usage(response)
    output_text = _extract_output_text(response)

    payload = _build_payload(
        root=root,
        trace_id=trace_id,
        obs_id=obs_id,
        start=start,
        latency=latency,
        ttfb=None,
        output_text=output_text,
        usage=usage,
        kwargs=kwargs,
    )
    root._xtrace.enqueue_batch(payload)


def _build_payload(
    *,
    root: _OpenAIClientWrapper,
    trace_id: str,
    obs_id: str,
    start: float,
    latency: float,
    ttfb: Optional[float],
    output_text: Optional[str],
    usage: Optional[Dict[str, Any]],
    kwargs: Dict[str, Any],
) -> Dict[str, Any]:
    ts_iso = _to_iso(start)
    end_iso = _to_iso(start + latency)
    completion_iso = _to_iso(start + ttfb) if ttfb is not None else None

    trace = {
        "id": trace_id,
        "timestamp": ts_iso,
        "name": kwargs.get("name") or root._name,
        "userId": root._user_id,
        "sessionId": root._session_id,
        "tags": root._tags,
        "metadata": root._metadata or None,
        "projectId": root._project_id or root._xtrace._cfg.default_project_id,
        "latency": latency,
        "totalCost": None,
    }

    messages = kwargs.get("messages")
    model = kwargs.get("model")

    obs = {
        "id": obs_id,
        "traceId": trace_id,
        "type": "GENERATION",
        "name": "chat",
        "startTime": ts_iso,
        "endTime": end_iso,
        "completionStartTime": completion_iso,
        "model": model,
        "modelParameters": None,
        "input": messages,
        "output": output_text,
        "usage": usage,
        "level": "DEFAULT",
        "statusMessage": None,
        "parentObservationId": None,
        "promptId": None,
        "promptName": None,
        "promptVersion": None,
        "modelId": None,
        "inputPrice": None,
        "outputPrice": None,
        "totalPrice": None,
        "calculatedInputCost": None,
        "calculatedOutputCost": None,
        "calculatedTotalCost": None,
        "latency": latency,
        "timeToFirstToken": ttfb,
        "completionTokens": _safe_get_int(usage, ["output"]),
        "promptTokens": _safe_get_int(usage, ["input"]),
        "totalTokens": _safe_get_int(usage, ["total"]),
        "unit": _safe_get_str(usage, ["unit"]),
        "metadata": kwargs.get("metadata"),
        "projectId": root._project_id or root._xtrace._cfg.default_project_id,
    }

    return {"trace": trace, "observations": [obs]}


def _to_iso(ts: float) -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%S", time.gmtime(ts)) + f".{int((ts % 1) * 1_000_000):06d}Z"


def _extract_output_text(resp: Any) -> Optional[str]:
    try:
        choice0 = resp.choices[0]
        msg = getattr(choice0, "message", None)
        if msg is not None:
            return getattr(msg, "content", None)
    except Exception:
        return None


def _extract_stream_usage(chunk: Any) -> Optional[Dict[str, Any]]:
    # Best-effort: depends on provider and whether stream_options.include_usage is enabled.
    try:
        u = getattr(chunk, "usage", None)
        if u is not None:
            prompt = getattr(u, "prompt_tokens", None)
            completion = getattr(u, "completion_tokens", None)
            total = getattr(u, "total_tokens", None)
            if prompt is None and completion is None and total is None:
                return None
            return {
                "input": int(prompt or 0),
                "output": int(completion or 0),
                "total": int(total or ((prompt or 0) + (completion or 0))),
                "unit": "TOKENS",
            }

        # allow custom injected dict
        u2 = getattr(chunk, "xtrace_usage", None)
        if isinstance(u2, dict):
            return u2
        return None
    except Exception:
        return None
    return None


def _extract_usage(resp: Any) -> Optional[Dict[str, Any]]:
    try:
        u = getattr(resp, "usage", None)
        if u is None:
            return None
        prompt = getattr(u, "prompt_tokens", None)
        completion = getattr(u, "completion_tokens", None)
        total = getattr(u, "total_tokens", None)
        if prompt is None and completion is None and total is None:
            return None
        return {
            "input": int(prompt or 0),
            "output": int(completion or 0),
            "total": int(total or ((prompt or 0) + (completion or 0))),
            "unit": "TOKENS",
        }
    except Exception:
        return None


def _extract_stream_delta_text(chunk: Any) -> Optional[str]:
    try:
        choices = getattr(chunk, "choices", None)
        if not choices:
            return None
        delta = getattr(choices[0], "delta", None)
        if delta is None:
            return None
        return getattr(delta, "content", None)
    except Exception:
        return None


def _safe_get_int(d: Optional[Dict[str, Any]], path: list[str]) -> Optional[int]:
    if not d:
        return None
    cur: Any = d
    for p in path:
        if not isinstance(cur, dict) or p not in cur:
            return None
        cur = cur[p]
    try:
        return int(cur)
    except Exception:
        return None


def _safe_get_str(d: Optional[Dict[str, Any]], path: list[str]) -> Optional[str]:
    if not d:
        return None
    cur: Any = d
    for p in path:
        if not isinstance(cur, dict) or p not in cur:
            return None
        cur = cur[p]
    if cur is None:
        return None
    return str(cur)
