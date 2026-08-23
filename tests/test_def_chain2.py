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

def get(endpoint):
    with urllib.request.urlopen(f"{API}/{endpoint}", timeout=60) as resp:
        return json.loads(resp.read())

# First, check vocab before
r = get("stats")
vocab_before = r['vocabulary']
print(f"Vocab before: {vocab_before}")

# Test with a word that exists in dictionary chunks but might not be in lexicon
# The lexicon has 155K words from the dictionary, so let's try learning
# a new word directly via the learn endpoint, then test def chain
# Actually, let's test with a word we know is in the chunks but check
# if the def chain fires by using a very rare word

# Let's use the learn endpoint to learn a nonsense word, then chat with it
print("\n=== LEARN: xyzzyabc ===")
r = post("learn", {"text": "xyzzyabc"})
print(f"  tokens: {r['tokens']}, vocab: {r['vocabulary']}")

# Now chat with the nonsense word - it's in the lexicon now but has no definition
print("\n=== CHAT: what is xyzzyabc ===")
r = post("chat", {"text": "what is xyzzyabc"})
print(f"Response: {r['response']}")
print(f"Words known: {r['words_learned']}")
print(f"Definitions learned: {r['definitions_learned']}")
print(f"Wiki learned: {r['wiki_learned']}")
print(f"Vocab: {r['vocabulary']}")

# Test with a real rare word that might not be in lexicon
# The model loads from chunks - let's try a very obscure word
print("\n=== CHAT: what is floccinaucinihilipilification ===")
r = post("chat", {"text": "what is floccinaucinihilipilification"})
print(f"Response: {r['response']}")
print(f"Words known: {r['words_learned']}")
print(f"Definitions learned: {r['definitions_learned']}")
print(f"Wiki learned: {r['wiki_learned']}")
print(f"Vocab: {r['vocabulary']}")
print(f"Coherence: {r['coherence']:.3f}")

# Check vocab after
r = get("stats")
vocab_after = r['vocabulary']
print(f"\nVocab after: {vocab_after}")
print(f"Vocab delta: {vocab_after - vocab_before}")
