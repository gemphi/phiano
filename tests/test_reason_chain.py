import urllib.request, json, sys
sys.stdout.reconfigure(encoding='utf-8')

API = "http://127.0.0.1:3002/api"

def post(endpoint, payload):
    data = json.dumps(payload).encode()
    req = urllib.request.Request(
        f"{API}/{endpoint}", data=data,
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=60) as resp:
        return json.loads(resp.read())

print("=== REASON CHAIN: what is knowledge ===")
r = post("reason_chain", {"text": "what is knowledge", "max_steps": 4})
print(f"Converged: {r['converged']}")
print(f"Steps: {len(r['steps'])}")
print(f"Final: {r['final_answer']}")
print()
for s in r['steps']:
    print(f"  Step {s['step']}: [{s['speech_act']}] coh={s['coherence']:.2f}")
    print(f"    Prompt: {s['prompt']}")
    print(f"    Output: {s['output']}")

print()
print("=== CHAT: explain quantum entanglement ===")
r = post("chat", {"text": "explain quantum entanglement"})
print(f"Response: {r['response']}")
print(f"Speech act: {r['speech_act']}")
print(f"DoF: {r['direction_of_fit']}")
print(f"Words known: {r['words_learned']}")
print(f"Definitions learned: {r['definitions_learned']}")
print(f"Wiki learned: {r['wiki_learned']}")
print(f"Vocab: {r['vocabulary']}")
print(f"Coherence: {r['coherence']:.3f}")

print()
print("=== CHAT: i think consciousness is beautiful ===")
r = post("chat", {"text": "i think consciousness is beautiful"})
print(f"Response: {r['response']}")
print(f"Speech act: {r['speech_act']}")
print(f"DoF: {r['direction_of_fit']}")
print(f"Definitions learned: {r['definitions_learned']}")
print(f"Wiki learned: {r['wiki_learned']}")
print(f"Vocab: {r['vocabulary']}")

print()
print("=== CHAT: what is epistemology ===")
r = post("chat", {"text": "what is epistemology"})
print(f"Response: {r['response']}")
print(f"Speech act: {r['speech_act']}")
print(f"DoF: {r['direction_of_fit']}")
print(f"Words known: {r['words_learned']}")
print(f"Definitions learned: {r['definitions_learned']}")
print(f"Wiki learned: {r['wiki_learned']}")
print(f"Vocab: {r['vocabulary']}")
print(f"Coherence: {r['coherence']:.3f}")
