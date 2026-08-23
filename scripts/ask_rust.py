import sys
import json
import urllib.request

API = "http://127.0.0.1:3000/api"

def api_post(endpoint, text, epochs=None):
    payload = {"text": text}
    if epochs:
        payload["epochs"] = epochs
    data = json.dumps(payload).encode()
    req = urllib.request.Request(
        f"{API}/{endpoint}",
        data=data,
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=120) as resp:
        return json.loads(resp.read())

query = " ".join(sys.argv[1:]) if len(sys.argv) > 1 else "explain ownership and borrowing in Rust"

print("============================================================")
print("  PHIANO - RUST QUESTION SOLVER (RUST PHASE ENGINE)         ")
print("============================================================\n")

print(f'--> Query: "{query}"\n')

# 1. Evaluate the query against the trained facet
eval_result = api_post("eval", query)
print(f"--> Coherence: {eval_result['coherence']:.3f}  "
      f"Novelty: {eval_result['novelty']:.3f}  "
      f"Resonance: {eval_result['resonance']:.3f}")
print(f"--> Verdict: {eval_result['verdict']}")
print(f"--> Vocabulary: {eval_result['vocabulary']:,} words\n")

# 2. Compose using the existing Rust sector composition engine
#    This walks the phase circle, generates 64 sector variations,
#    evaluates them, keeps the best, trains on it, and recurses.
print("--- COMPOSITION (Rust sector traversal engine) ---\n")
comp = api_post("compose", query)
print(f"  Winner: sector {comp['winning_sector']} ({comp['winning_color']}) after {comp['rounds']} rounds")
print(f"  Coherence: {comp['coherence']:.3f}  Novelty: {comp['novelty']:.3f}  Resonance: {comp['resonance']:.3f}")
print(f"  Verdict: {comp['verdict']}\n")
print(f"  {comp['text']}")

# 3. Oscillator evaluation — shows sync, entropy, colors
print("\n--- OSCILLATOR EVALUATION ---\n")
osc_result = api_post("oscillator/eval", query)
print(f"  Coherence: {osc_result['coherence']:.4f}")
print(f"  Sync:      {osc_result['sync']:.4f}")
print(f"  Entropy:   {osc_result['entropy']:.4f}")
print(f"  Words:     {osc_result['word_count']}")
colors = ", ".join(f"{c} ({a:.1f})" for c, a in osc_result["dominant_colors"])
print(f"  Colors:    {colors}")

print("\n============================================================")
