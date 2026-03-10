import threading
import time
import sys
from pathlib import Path
import unittest
from unittest.mock import patch

SDK_ROOT = Path(__file__).resolve().parents[1]
if str(SDK_ROOT) not in sys.path:
    sys.path.insert(0, str(SDK_ROOT))

from xtrace_sdk.client import XTraceClient
from xtrace_sdk.openai import observe_openai


class _MockResponse:
    def __init__(self, status_code=200):
        self.status_code = status_code


class _CapturedSession:
    instances = []
    instances_lock = threading.Lock()

    def __init__(self):
        self.calls = []
        with self.instances_lock:
            self.__class__.instances.append(self)

    @classmethod
    def reset(cls):
        with cls.instances_lock:
            cls.instances = []

    def post(self, url, headers=None, json=None, timeout=None):
        self.calls.append(
            {
                "url": url,
                "headers": headers,
                "json": json,
                "timeout": timeout,
            }
        )
        return _MockResponse(200)


class _MockUsage:
    prompt_tokens = 10
    completion_tokens = 20
    total_tokens = 30


class _MockMessage:
    content = "Hello from mock!"


class _MockChoice:
    def __init__(self):
        self.message = _MockMessage()


class _MockCompletionResponse:
    def __init__(self):
        self.choices = [_MockChoice()]
        self.usage = _MockUsage()


class _MockCompletions:
    def create(self, *args, **kwargs):
        return _MockCompletionResponse()


class _MockChat:
    def __init__(self):
        self.completions = _MockCompletions()


class _MockOpenAI:
    def __init__(self):
        self.chat = _MockChat()


class SessionIngestIntegrationTest(unittest.TestCase):
    def setUp(self):
        _CapturedSession.reset()

    def tearDown(self):
        for session in list(_CapturedSession.instances):
            session.calls.clear()

    def _build_client(self):
        return XTraceClient(
            base_url="http://127.0.0.1:8742",
            api_key="test-key",
            default_project_id="test-project",
            flush_interval_s=0.01,
            request_timeout_s=0.1,
            max_retries=0,
        )

    def _wait_for_post(self, expected_count=1, timeout_s=2.0):
        deadline = time.time() + timeout_s
        while time.time() < deadline:
            count = sum(len(session.calls) for session in _CapturedSession.instances)
            if count >= expected_count:
                return
            time.sleep(0.01)
        self.fail(f"Timed out waiting for {expected_count} post call(s)")

    @patch("xtrace_sdk.client.requests.Session", side_effect=_CapturedSession)
    def test_chat_turn_payload_contains_session_and_turn_metadata(self, _session_ctor):
        client = self._build_client()
        wrapped = observe_openai(
            _MockOpenAI(),
            xtrace=client,
            session_id="sess_123",
            turn_id="turn_abc",
            project_id="test-project",
        )

        wrapped.chat.completions.create(
            model="gpt-4o",
            messages=[{"role": "user", "content": "Hi"}],
            metadata={"custom_field": "value1"},
        )

        self._wait_for_post()
        client.shutdown(timeout_s=0.2)

        posted = _CapturedSession.instances[0].calls[0]
        payload = posted["json"]
        trace = payload["trace"]
        observation = payload["observations"][0]

        self.assertEqual(posted["url"], "http://127.0.0.1:8742/v1/l/batch")
        self.assertEqual(posted["headers"]["Authorization"], "Bearer test-key")
        self.assertEqual(trace["session_id"], "sess_123")
        self.assertEqual(trace["projectId"], "test-project")
        self.assertEqual(trace["metadata"]["turn_id"], "turn_abc")
        self.assertEqual(trace["metadata"]["custom_field"], "value1")
        self.assertEqual(observation["traceId"], trace["id"])
        self.assertEqual(observation["projectId"], "test-project")
        self.assertEqual(observation["metadata"]["turn_id"], "turn_abc")
        self.assertEqual(observation["metadata"]["custom_field"], "value1")
        self.assertEqual(observation["input"][0]["content"], "Hi")
        self.assertEqual(observation["output"], "Hello from mock!")

    @patch("xtrace_sdk.client.requests.Session", side_effect=_CapturedSession)
    def test_agent_run_payload_contains_run_and_step_metadata(self, _session_ctor):
        client = self._build_client()
        wrapped = observe_openai(
            _MockOpenAI(),
            xtrace=client,
            session_id="sess_456",
            turn_id="turn_xyz",
            run_id="run_789",
            project_id="test-project",
        )

        wrapped.chat.completions.create(
            model="tool-executor",
            messages=[{"role": "system", "content": "Running tool..."}],
            metadata={
                "step_id": "step_001",
                "step_type": "tool_call",
                "tool_name": "search_web",
            },
        )

        self._wait_for_post()
        client.shutdown(timeout_s=0.2)

        posted = _CapturedSession.instances[0].calls[0]
        payload = posted["json"]
        trace = payload["trace"]
        observation = payload["observations"][0]

        self.assertEqual(trace["session_id"], "sess_456")
        self.assertEqual(trace["metadata"]["turn_id"], "turn_xyz")
        self.assertEqual(trace["metadata"]["run_id"], "run_789")
        self.assertEqual(observation["metadata"]["turn_id"], "turn_xyz")
        self.assertEqual(observation["metadata"]["run_id"], "run_789")
        self.assertEqual(observation["metadata"]["step_id"], "step_001")
        self.assertEqual(observation["metadata"]["step_type"], "tool_call")
        self.assertEqual(observation["metadata"]["tool_name"], "search_web")


if __name__ == "__main__":
    unittest.main()
