import atexit
import os
import queue
import threading
import time
import uuid
from dataclasses import dataclass
from typing import Any, Dict, List, Optional

import requests


@dataclass
class XTraceConfig:
    base_url: str
    api_key: str
    default_project_id: str
    queue_max_size: int = 10_000
    batch_max_size: int = 100
    flush_interval_s: float = 0.5
    request_timeout_s: float = 2.0
    max_retries: int = 3


class XTraceClient:
    def __init__(
        self,
        base_url: Optional[str] = None,
        api_key: Optional[str] = None,
        default_project_id: Optional[str] = None,
        *,
        queue_max_size: int = 10_000,
        batch_max_size: int = 100,
        flush_interval_s: float = 0.5,
        request_timeout_s: float = 2.0,
        max_retries: int = 3,
    ) -> None:
        base_url = base_url or os.environ.get("XTRACE_BASE_URL") or "http://127.0.0.1:8080"
        api_key = api_key or os.environ.get("XTRACE_API_KEY") or os.environ.get("XTRACE_BEARER_TOKEN")
        if not api_key:
            raise ValueError("missing api_key (env XTRACE_API_KEY or XTRACE_BEARER_TOKEN)")

        default_project_id = default_project_id or os.environ.get("XTRACE_PROJECT_ID") or "default"

        self._cfg = XTraceConfig(
            base_url=base_url.rstrip("/"),
            api_key=api_key,
            default_project_id=default_project_id,
            queue_max_size=queue_max_size,
            batch_max_size=batch_max_size,
            flush_interval_s=flush_interval_s,
            request_timeout_s=request_timeout_s,
            max_retries=max_retries,
        )

        self._q: "queue.Queue[Dict[str, Any]]" = queue.Queue(maxsize=self._cfg.queue_max_size)
        self._stop = threading.Event()
        self._worker = threading.Thread(target=self._run, name="xtrace-ingest", daemon=True)

        self.dropped_events = 0
        self.sent_batches = 0
        self.failed_batches = 0

        self._worker.start()
        atexit.register(self.shutdown, timeout_s=1.0)

    @staticmethod
    def new_id() -> str:
        return str(uuid.uuid4())

    def enqueue_batch(self, payload: Dict[str, Any]) -> None:
        try:
            self._q.put_nowait(payload)
        except queue.Full:
            self.dropped_events += 1

    def flush(self, timeout_s: float = 5.0) -> None:
        deadline = time.time() + timeout_s
        while time.time() < deadline:
            if self._q.empty():
                return
            time.sleep(0.05)

    def shutdown(self, timeout_s: float = 1.0) -> None:
        if self._stop.is_set():
            return
        self._stop.set()
        self.flush(timeout_s=timeout_s)

    def _run(self) -> None:
        session = requests.Session()
        url = f"{self._cfg.base_url}/v1/l/batch"
        headers = {
            "Authorization": f"Bearer {self._cfg.api_key}",
            "Content-Type": "application/json",
        }

        buf: List[Dict[str, Any]] = []
        last_flush = time.time()

        while not self._stop.is_set():
            timeout = max(0.0, self._cfg.flush_interval_s - (time.time() - last_flush))
            try:
                item = self._q.get(timeout=timeout)
                buf.append(item)
            except queue.Empty:
                pass

            if not buf:
                continue

            if len(buf) < self._cfg.batch_max_size and (time.time() - last_flush) < self._cfg.flush_interval_s:
                continue

            payloads = self._split_by_trace(buf)
            buf.clear()
            last_flush = time.time()

            for payload in payloads:
                ok = self._post_with_retry(session, url, headers, payload)
                if ok:
                    self.sent_batches += 1
                else:
                    self.failed_batches += 1

        # best-effort drain
        drain: List[Dict[str, Any]] = []
        while True:
            try:
                drain.append(self._q.get_nowait())
            except queue.Empty:
                break
        if drain:
            for payload in self._split_by_trace(drain):
                ok = self._post_with_retry(session, url, headers, payload)
                if ok:
                    self.sent_batches += 1
                else:
                    self.failed_batches += 1

    def _split_by_trace(self, items: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        grouped: Dict[str, Dict[str, Any]] = {}

        for it in items:
            trace = it.get("trace")
            observations = it.get("observations") or []

            trace_id = None
            if isinstance(trace, dict):
                trace_id = trace.get("id")
            if trace_id is None and observations:
                first = observations[0]
                if isinstance(first, dict):
                    trace_id = first.get("traceId")

            if trace_id is None:
                continue

            if trace_id not in grouped:
                grouped[trace_id] = {"trace": None, "observations": []}

            if grouped[trace_id]["trace"] is None and trace is not None:
                grouped[trace_id]["trace"] = trace

            grouped[trace_id]["observations"].extend(observations)

        return list(grouped.values())

    def _post_with_retry(
        self,
        session: requests.Session,
        url: str,
        headers: Dict[str, str],
        payload: Dict[str, Any],
    ) -> bool:
        delay = 0.2
        for attempt in range(self._cfg.max_retries + 1):
            try:
                resp = session.post(url, headers=headers, json=payload, timeout=self._cfg.request_timeout_s)
                if 200 <= resp.status_code < 300:
                    return True
                if resp.status_code == 429 or 500 <= resp.status_code < 600:
                    time.sleep(delay)
                    delay = min(delay * 2.0, 5.0)
                    continue
                return False
            except requests.RequestException:
                if attempt >= self._cfg.max_retries:
                    return False
                time.sleep(delay)
                delay = min(delay * 2.0, 5.0)
        return False
