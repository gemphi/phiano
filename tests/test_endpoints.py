#!/usr/bin/env python3
"""Quick endpoint test for all Phiano API endpoints."""
import urllib.request
import json
import time

API = "http://127.0.0.1:3002/api"

def post(endpoint, payload):
    data = json.dumps(payload).encode()
    req = urllib.request.Request(
        f"{API}/{endpoint}", data=data,
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=120) as resp:
        return json.loads(resp.read())

def get(endpoint):
    req = urllib.request.Request(f"{API}/{endpoint}")
    with urllib.request.urlopen(req, timeout=10) as resp:
        return json.loads(resp.read())

def test(name, fn):
    print(f"\n--- {name} ---")
    t = time.time()
    try:
        result = fn()
        dt = time.time() - t
        print(f"  OK ({dt:.1f}s)")
        for k, v in result.items():
            if isinstance(v, str) and len(v) > 100:
                print(f"  {k}: {v[:100]}...")
            elif isinstance(v, list):
                print(f"  {k}: [{len(v)} items]")
            else:
                print(f"  {k}: {v}")
        return True
    except Exception as e:
        dt = time.time() - t
        print(f"  FAIL ({dt:.1f}s): {e}")
        return False

results = []
results.append(test("stats", lambda: get("stats")))
results.append(test("eval", lambda: post("eval", {"text": "ownership borrowing rust"})))
results.append(test("learn", lambda: post("learn", {"text": "Kuramoto synchronization coupled oscillators"})))
results.append(test("generate", lambda: post("generate", {"text": "rust programming", "max_tokens": 24, "temperature": 0.15})))
results.append(test("compose", lambda: post("compose", {"text": "explain ownership in rust"})))
results.append(test("instruct", lambda: post("instruct", {"text": "explain how lifetimes work"})))
results.append(test("reason", lambda: post("reason", {"text": "what is ownership in rust"})))
results.append(test("layers", lambda: get("layers")))
results.append(test("oscillator/eval", lambda: post("oscillator/eval", {"text": "synchronization oscillators phase"})))
results.append(test("infinity/visualize", lambda: post("infinity/visualize", {"text": "rust ownership"})))
results.append(test("infinity/train", lambda: post("infinity/train", {"text": "phase synchronization coupling"})))
results.append(test("wiki/search", lambda: post("wiki/search", {"topic": "artificial intelligence"})))
results.append(test("wiki/learn", lambda: post("wiki/learn", {"topic": "Rust programming language", "epochs": 2})))

print(f"\n{'='*40}")
passed = sum(1 for r in results if r)
print(f"Results: {passed}/{len(results)} passed")
