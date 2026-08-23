"""
Interactive training session: teach Phiano new concepts and watch it learn.
"""
import urllib.request, json, sys, time

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

def section(title):
    print(f"\n{'='*60}")
    print(f"  {title}")
    print(f"{'='*60}")

# ── Baseline ──────────────────────────────────────────────────────────
section("BASELINE: Before training")
r = get("stats")
print(f"  Vocabulary: {r['vocabulary']}")
print(f"  Memory entries: {r['memory_entries']}")

# Test current reasoning on several topics
baselines = [
    "explain how neural networks learn",
    "what is consciousness",
    "describe the water cycle",
    "explain rust ownership model",
    "what is quantum decoherence",
]

for prompt in baselines:
    r = post("chat", {"text": prompt})
    print(f"\n  Q: {prompt}")
    print(f"  A: {r['response'][:120]}...")
    print(f"     speech_act={r['speech_act']} coh={r['coherence']:.2f} "
          f"defs={r['definitions_learned']} wiki={r['wiki_learned']}")

# ── Interactive Training Phase 1: Wikipedia learning ──────────────────
section("TRAINING PHASE 1: Wikipedia articles")

wiki_topics = [
    "Neural network",
    "Consciousness",
    "Water cycle",
    "Ownership",
    "Quantum decoherence",
    "Machine learning",
    "Deep learning",
    "Transformer (machine learning model)",
    "Attention mechanism",
    "Backpropagation",
]

for topic in wiki_topics:
    try:
        r = post("wiki/learn", {"topic": topic, "epochs": 2})
        status = "OK" if 'message' in r else "FAIL"
        vocab = r.get('vocabulary_after', '?')
        tokens = r.get('tokens_trained', 0)
        print(f"  [{status}] {topic}: {tokens} tokens, vocab={vocab}")
    except Exception as e:
        print(f"  [ERR] {topic}: {e}")

# ── Interactive Training Phase 2: Direct text training ────────────────
section("TRAINING PHASE 2: Direct concept training")

concepts = [
    "neural networks learn by adjusting weights through backpropagation",
    "consciousness emerges from complex neural activity in the brain",
    "the water cycle involves evaporation condensation precipitation and collection",
    "ownership in rust ensures memory safety through borrow checking",
    "quantum decoherence occurs when quantum states interact with environment",
    "transformers use self attention to process sequential data efficiently",
    "deep learning models learn hierarchical representations of data",
    "attention mechanisms allow models to focus on relevant parts of input",
    "backpropagation computes gradients by applying the chain rule",
    "machine learning enables systems to learn patterns from data",
]

for text in concepts:
    try:
        r = post("learn", {"text": text})
        print(f"  [{r['message']}] vocab={r['vocabulary']}")
    except Exception as e:
        print(f"  [ERR] {e}")

# ── Interactive Training Phase 3: Chat-based learning ─────────────────
section("TRAINING PHASE 3: Chat-based real-time learning")

chat_prompts = [
    "explain how attention mechanisms work in transformers",
    "what is backpropagation in neural networks",
    "describe consciousness from a neuroscience perspective",
    "explain the rust ownership and borrowing system",
    "what is quantum decoherence and why does it matter",
]

for prompt in chat_prompts:
    try:
        r = post("chat", {"text": prompt})
        print(f"\n  Q: {prompt}")
        print(f"  A: {r['response'][:150]}")
        print(f"     defs={r['definitions_learned']} wiki={r['wiki_learned']} "
              f"coh={r['coherence']:.3f} vocab={r['vocabulary']}")
    except Exception as e:
        print(f"\n  Q: {prompt}\n  [ERR] {e}")

# ── Post-training evaluation ──────────────────────────────────────────
section("POST-TRAINING: Re-test same questions")

for prompt in baselines:
    try:
        r = post("chat", {"text": prompt})
        print(f"\n  Q: {prompt}")
        print(f"  A: {r['response'][:150]}")
        print(f"     speech_act={r['speech_act']} coh={r['coherence']:.2f} "
              f"defs={r['definitions_learned']} wiki={r['wiki_learned']}")
    except Exception as e:
        print(f"\n  Q: {prompt}\n  [ERR] {e}")

# ── Reasoning chain after training ────────────────────────────────────
section("REASONING CHAIN: what is consciousness (post-training)")

try:
    r = post("reason_chain", {"text": "what is consciousness", "max_steps": 4})
    print(f"  Converged: {r['converged']}, Steps: {len(r['steps'])}")
    for s in r['steps']:
        print(f"\n  Step {s['step']} [{s['speech_act']}] coh={s['coherence']:.2f}")
        print(f"    Out: {s['output'][:120]}")
except Exception as e:
    print(f"  [ERR] {e}")

# ── Final stats ───────────────────────────────────────────────────────
section("FINAL STATS")
r = get("stats")
print(f"  Vocabulary: {r['vocabulary']}")
print(f"  Memory entries: {r['memory_entries']}")

# Eval
try:
    r = post("eval", {"text": "neural network consciousness quantum"})
    print(f"  Eval: coh={r['coherence']:.3f} novelty={r['novelty']:.3f} "
          f"resonance={r['resonance']:.3f} overall={r['overall']:.3f}")
    print(f"  Verdict: {r['verdict']}")
except Exception as e:
    print(f"  [ERR] {e}")
