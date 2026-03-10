import time
import uuid
import requests
from xtrace_sdk.client import XTraceClient
from xtrace_sdk.openai import observe_openai

# Configuration
BASE_URL = "http://127.0.0.1:8742"
API_KEY = "test-key"
PROJECT_ID = "test-project"

def run_verification():
    print(f"Connecting to {BASE_URL}...")
    client = XTraceClient(base_url=BASE_URL, api_key=API_KEY, default_project_id=PROJECT_ID)
    
    # Mock OpenAI client
    class MockChat:
        def create(self, *args, **kwargs):
            return MockResponse()
            
    class MockResponse:
        def __init__(self):
            self.choices = [MockChoice()]
            self.usage = MockUsage()
            
    class MockChoice:
        def __init__(self):
            self.message = MockMessage()
            
    class MockMessage:
        content = "Hello from mock!"
        
    class MockUsage:
        prompt_tokens = 10
        completion_tokens = 20
        total_tokens = 30
        
    class MockOpenAI:
        def __init__(self):
            self.chat = MockChatWrapper()
            
    class MockChatWrapper:
        completions = MockChat()

    # 1. Simulate a Chat Turn
    session_id = f"sess_{uuid.uuid4().hex[:8]}"
    turn_id_1 = f"turn_{uuid.uuid4().hex[:8]}"
    
    print(f"Sending Chat Turn (Session: {session_id}, Turn: {turn_id_1})...")
    mock_openai = MockOpenAI()
    wrapped = observe_openai(
        mock_openai, 
        xtrace=client, 
        session_id=session_id,
        turn_id=turn_id_1,
        project_id=PROJECT_ID
    )
    
    wrapped.chat.completions.create(
        model="gpt-4o",
        messages=[{"role": "user", "content": "Hi"}],
        metadata={"custom_field": "value1"}
    )
    
    # 2. Simulate an Agent Run (Execution Paradigm)
    turn_id_2 = f"turn_{uuid.uuid4().hex[:8]}"
    run_id = f"run_{uuid.uuid4().hex[:8]}"
    step_id = f"step_{uuid.uuid4().hex[:8]}"
    
    print(f"Sending Agent Run (Turn: {turn_id_2}, Run: {run_id}, Step: {step_id})...")
    wrapped_agent = observe_openai(
        mock_openai,
        xtrace=client,
        session_id=session_id,
        turn_id=turn_id_2,
        run_id=run_id,
        project_id=PROJECT_ID
    )
    
    # Simulate a tool step via metadata override
    wrapped_agent.chat.completions.create(
        model="tool-executor",
        messages=[{"role": "system", "content": "Running tool..."}],
        metadata={
            "step_id": step_id,
            "step_type": "tool_call",
            "tool_name": "search_web"
        }
    )
    
    client.flush()
    print("Data flushed. Waiting for ingestion...")
    time.sleep(2)
    
    # 3. Verify via API
    print("Verifying data via API...")
    
    # Verify Turn 1
    resp = requests.get(f"{BASE_URL}/api/public/traces", params={"sessionId": session_id}, headers={"Authorization": f"Bearer {API_KEY}"})
    if resp.status_code != 200:
        print(f"Failed to list traces: {resp.text}")
        return

    traces = resp.json().get("data", [])
    print(f"Found {len(traces)} traces for session {session_id}")
    
    found_turn_1 = False
    found_run = False
    
    for t in traces:
        meta = t.get("metadata", {})
        print(f"Trace {t['id']} metadata: {meta}")
        
        if meta.get("turn_id") == turn_id_1:
            found_turn_1 = True
            
        if meta.get("run_id") == run_id:
            found_run = True
            # Verify step in observation
            trace_detail = requests.get(f"{BASE_URL}/api/public/traces/{t['id']}", headers={"Authorization": f"Bearer {API_KEY}"}).json()
            obs_list = trace_detail.get("observations", [])
            for obs in obs_list:
                obs_meta = obs.get("metadata", {})
                print(f"  Observation metadata: {obs_meta}")
                if obs_meta.get("step_id") == step_id and obs_meta.get("step_type") == "tool_call":
                    print("  ✅ Found Agent Step with correct metadata!")
    
    if found_turn_1:
        print("✅ Found Turn 1 with correct turn_id")
    else:
        print("❌ Turn 1 NOT found")
        
    if found_run:
        print("✅ Found Agent Run with correct run_id")
    else:
        print("❌ Agent Run NOT found")

if __name__ == "__main__":
    run_verification()
