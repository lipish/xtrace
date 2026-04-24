import asyncio
import base64
import json
import os
from pathlib import Path
import sys
import time
import traceback
import uuid

import requests
import xoscar as xo
from xoscar import create_actor_pool

REPO_ROOT = Path(__file__).resolve().parents[1]
XINFERENCE_BACKEND_ROOT = REPO_ROOT / "xinference" / "xinference-backend"
if str(XINFERENCE_BACKEND_ROOT) not in sys.path:
    sys.path.insert(0, str(XINFERENCE_BACKEND_ROOT))

os.environ["XINFERENCE_RUNNING_ENV"] = "TEST"
os.environ["LANGFUSE_HOST"] = "http://127.0.0.1:8742"
os.environ["LANGFUSE_PUBLIC_KEY"] = "pk-xtrace-test"
os.environ["LANGFUSE_SECRET_KEY"] = "sk-xtrace-test"

from xinference.v2.api.restful_api import RESTfulAPI
from xinference.v2.core.model import ModelActor


class DummyRequest:
    def __init__(self, payload, headers=None):
        self._payload = payload
        self.headers = headers or {}

    async def json(self):
        return self._payload


class MockModelFamily:
    def to_description(self):
        return {
            "model_type": "LLM",
            "model_format": "mock",
            "quantization": "none",
        }


class MockChatModel:
    def __init__(self, output_text):
        self.model_family = MockModelFamily()
        self.model_spec = "mock-chat-spec"
        self.model_uid = "mock-chat-model"
        self._output_text = output_text

    async def chat(self, messages, generate_config=None, **kwargs):
        _ = messages, generate_config, kwargs
        return {
            "id": "chatcmpl-" + uuid.uuid4().hex[:12],
            "object": "chat.completion",
            "created": int(time.time()),
            "model": self.model_uid,
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": self._output_text,
                    },
                    "finish_reason": "stop",
                }
            ],
            "usage": {
                "prompt_tokens": 11,
                "completion_tokens": 7,
                "total_tokens": 18,
            },
        }


class MockChatModelActor(ModelActor):
    async def __pre_destroy__(self):
        pass

    async def record_metrics(self, name, op, kwargs):
        _ = name, op, kwargs
        return None


class MockSupervisor:
    def __init__(self, model_ref):
        self._model_ref = model_ref

    async def get_model(self, model_uid, replica_id=None):
        _ = model_uid, replica_id
        return self._model_ref

    async def describe_model(self, model_uid, replica_id=None):
        _ = model_uid, replica_id
        return {"model_family": "mock-chat-family"}


async def find_trace(output_marker):
    auth = base64.b64encode(b"pk-xtrace-test:sk-xtrace-test").decode("ascii")
    headers = {"Authorization": "Basic " + auth}
    deadline = time.time() + 8
    while time.time() < deadline:
        rows_resp = requests.get(
            "http://127.0.0.1:8742/api/public/traces?page=1&limit=20",
            headers=headers,
            timeout=5,
        )
        rows_resp.raise_for_status()
        for row in rows_resp.json().get("data", []):
            detail_resp = requests.get(
                f"http://127.0.0.1:8742/api/public/traces/{row['id']}",
                headers=headers,
                timeout=5,
            )
            detail_resp.raise_for_status()
            detail = detail_resp.json()
            for observation in detail.get("observations", []):
                if observation.get("output") == output_marker:
                    return detail
        await asyncio.sleep(0.5)
    raise RuntimeError(f"missing trace for output {output_marker}")


async def main():
    output_marker = "E2E_TRACE_OUTPUT_" + uuid.uuid4().hex[:12]
    prompt_marker = "E2E_TRACE_INPUT_" + uuid.uuid4().hex[:12]
    print("MARKERS", output_marker, prompt_marker)
    pool = await create_actor_pool(
        f"test://127.0.0.1:{xo.utils.get_next_port()}", n_process=0
    )
    async with pool:
        model_ref = await xo.create_actor(
            MockChatModelActor,
            address=pool.external_address,
            uid=MockChatModelActor.default_uid(),
            supervisor_address="test-supervisor:1234",
            worker_address="test-worker:5678",
            model=MockChatModel(output_marker),
            replica_model_uid="mock-chat-model",
        )
        api = RESTfulAPI("test-supervisor-address", "127.0.0.1", 9997)

        async def fake_supervisor_ref():
            return MockSupervisor(model_ref)

        async def fake_report_error_event(*args, **kwargs):
            _ = args, kwargs
            return None

        async def fake_get_model_last_error(uid, err):
            _ = uid
            return err

        api._get_supervisor_ref = fake_supervisor_ref
        api._report_error_event = fake_report_error_event
        api._get_model_last_error = fake_get_model_last_error
        payload = {
            "model": "mock-chat-model",
            "stream": False,
            "messages": [{"role": "user", "content": prompt_marker}],
            "temperature": 0,
        }
        response = await api.create_chat_completion(DummyRequest(payload, headers={}))
        print("RESPONSE_STATUS", response.status_code)
        print("RESPONSE_BODY", response.body.decode())
        trace = await find_trace(output_marker)
        observations = trace.get("observations", [])
        span = next(o for o in observations if o.get("name") == "create_chat_completion")
        generation = next(o for o in observations if o.get("name") == "chat")
        assert generation.get("parentObservationId") == span.get("id")
        assert generation.get("traceId") == trace.get("id")
        assert generation.get("output") == output_marker
        assert generation.get("input")[0]["content"] == prompt_marker
        assert generation.get("usage", {}).get("input") == 11
        print(
            json.dumps(
                {
                    "trace_id": trace["id"],
                    "span_id": span["id"],
                    "generation_id": generation["id"],
                    "parent_observation_id": generation["parentObservationId"],
                    "output": generation["output"],
                    "usage": generation.get("usage"),
                    "langfuse_module": type(api._langfuse).__module__ if api._langfuse else None,
                },
                ensure_ascii=False,
            )
        )


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except Exception:
        traceback.print_exc()
        raise
