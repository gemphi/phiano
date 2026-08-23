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

# Test with obscure words that are likely NOT in the 155K lexicon
# but ARE in the dictionary chunks
tests = [
    "what is sesquipedalian",
    "explain perspicacious obfuscation",
    "what is pulchritude",
]

for prompt in tests:
    print(f"\n=== CHAT: {prompt} ===")
    r = post("chat", {"text": prompt})
    print(f"Response: {r['response']}")
    print(f"Speech act: {r['speech_act']}")
    print(f"Words known: {r['words_learned']}")
    print(f"Definitions learned: {r['definitions_learned']}")
    print(f"Wiki learned: {r['wiki_learned']}")
    print(f"Vocab: {r['vocabulary']}")
    print(f"Coherence: {r['coherence']:.3f}")
