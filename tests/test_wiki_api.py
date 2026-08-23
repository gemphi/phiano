import urllib.request, json, time

API = "http://127.0.0.1:3001/api"

def post(endpoint, payload):
    data = json.dumps(payload).encode()
    req = urllib.request.Request(
        f"{API}/{endpoint}", data=data,
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        return json.loads(resp.read())

print("=== wiki/search ===")
t = time.time()
try:
    r = post("wiki/search", {"topic": "artificial intelligence"})
    print(f"OK ({time.time()-t:.1f}s): {json.dumps(r, indent=2)[:500]}")
except Exception as e:
    print(f"FAIL ({time.time()-t:.1f}s): {e}")

print("\n=== wiki/learn ===")
t = time.time()
try:
    r = post("wiki/learn", {"topic": "Rust programming language", "epochs": 2})
    print(f"OK ({time.time()-t:.1f}s): {json.dumps(r, indent=2)[:800]}")
except Exception as e:
    print(f"FAIL ({time.time()-t:.1f}s): {e}")

