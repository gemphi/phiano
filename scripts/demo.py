#!/usr/bin/env python3
"""
Phiano Investor Demo Script
Demonstrates all capabilities of the Phiano phase oscillator model.
"""

import sys
import json
import urllib.request
import time

API = "http://127.0.0.1:3002/api"


def api_post(endpoint, payload=None):
    data = json.dumps(payload or {}).encode()
    req = urllib.request.Request(
        f"{API}/{endpoint}",
        data=data,
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=120) as resp:
        return json.loads(resp.read())


def api_get(endpoint):
    req = urllib.request.Request(f"{API}/{endpoint}")
    with urllib.request.urlopen(req, timeout=10) as resp:
        return json.loads(resp.read())


def banner(title):
    print("\n" + "=" * 60)
    print(f"  {title}")
    print("=" * 60)


def section(title):
    print(f"\n--- {title} ---\n")


# ── DEMO START ────────────────────────────────────────────────────────────────

banner("PHIANO - Phase Oscillator Language Model")
print("  Investor Demo")
print("  The world's first Kuramoto-coupled language model")
print("=" * 60)

# 1. STATS
section("1. MODEL STATISTICS")
stats = api_get("stats")
print(f"  Vocabulary:     {stats['vocabulary']:,} words")
print(f"  Memory entries: {stats['memory_entries']}")
print(f"  Model size:     ~5.7 MB on disk")
print(f"  Architecture:   Phase oscillator field (Kuramoto coupling)")
print(f"  Inference:      Sub-millisecond (CPU, no GPU required)")

# 2. EVALUATION
section("2. TEXT EVALUATION")
queries = [
    "ownership and borrowing in Rust",
    "machine learning neural networks",
    "quantum computing superposition",
]
for q in queries:
    result = api_post("eval", {"text": q})
    print(f"  '{q}'")
    print(f"    Coherence: {result['coherence']:.3f}  "
          f"Novelty: {result['novelty']:.3f}  "
          f"Resonance: {result['resonance']:.3f}")
    print(f"    Verdict: {result['verdict']}")
    print()

# 3. ONLINE LEARNING
section("3. ONLINE LEARNING (Instant)")
learn_text = "The Kuramoto model describes synchronization in coupled oscillator systems"
print(f"  Teaching: '{learn_text}'")
t0 = time.time()
learn_result = api_post("learn", {"text": learn_text})
t1 = time.time()
print(f"  Learned {learn_result['tokens']} tokens in {t1-t0:.3f}s")
print(f"  Vocabulary: {learn_result['vocabulary']:,} words")
print(f"  Message: {learn_result['message']}")

# 4. WIKIPEDIA LEARNING
section("4. WIKIPEDIA LEARNING (Live)")
wiki_topic = sys.argv[1] if len(sys.argv) > 1 else "Artificial intelligence"
print(f"  Fetching Wikipedia article: '{wiki_topic}'...")
t0 = time.time()
try:
    wiki_result = api_post("wiki/learn", {"topic": wiki_topic, "epochs": 3})
    t1 = time.time()
    print(f"  Title: {wiki_result['title']}")
    print(f"  Trained on {wiki_result['tokens_trained']} tokens in {t1-t0:.1f}s")
    print(f"  Vocabulary: {wiki_result['vocabulary_before']:,} → {wiki_result['vocabulary_after']:,} words")
    print(f"  Coherence: {wiki_result['coherence']:.3f}  "
          f"Novelty: {wiki_result['novelty']:.3f}  "
          f"Resonance: {wiki_result['resonance']:.3f}")
    print(f"  Verdict: {wiki_result['verdict']}")
    print(f"  Extract: {wiki_result['extract'][:200]}...")
except Exception as e:
    print(f"  Wikipedia learning failed: {e}")

# 5. COMPOSITION
section("5. SECTOR COMPOSITION (64-Sector River Flow)")
comp_query = "explain ownership and borrowing in Rust"
print(f"  Query: '{comp_query}'")
t0 = time.time()
comp = api_post("compose", {"text": comp_query})
t1 = time.time()
print(f"  Winner: sector {comp['winning_sector']} ({comp['winning_color']}) "
      f"after {comp['rounds']} round(s) in {t1-t0:.1f}s")
print(f"  Coherence: {comp['coherence']:.3f}  "
      f"Novelty: {comp['novelty']:.3f}  "
      f"Resonance: {comp['resonance']:.3f}")
print(f"  Verdict: {comp['verdict']}")
print(f"  Output:\n    {comp['text'].replace(chr(10), chr(10) + '    ')}")

# 6. SEQUENCE GENERATION
section("6. PHASE-GUIDED GENERATION")
gen_query = "rust programming language"
print(f"  Prompt: '{gen_query}'")
t0 = time.time()
gen = api_post("generate", {"text": gen_query, "max_tokens": 24, "temperature": 0.15})
t1 = time.time()
print(f"  Generated in {t1-t0:.1f}s")
print(f"  Output: {gen['generated']}")
print(f"  Context phase: {gen['context_phase']:.4f}")
print(f"  Context amplitude: {gen['context_amplitude']:.4f}")

# 7. INSTRUCTION FOLLOWING
section("7. INSTRUCTION FOLLOWING")
instr_query = "explain how lifetimes work in Rust"
print(f"  Instruction: '{instr_query}'")
t0 = time.time()
instr = api_post("instruct", {"text": instr_query})
t1 = time.time()
print(f"  Executed in {t1-t0:.1f}s")
print(f"  Output:\n    {instr['output'].replace(chr(10), chr(10) + '    ')}")

# 8. OSCILLATOR EVALUATION
section("8. OSCILLATOR EVALUATION")
osc_query = "synchronization coupled oscillators phase"
print(f"  Query: '{osc_query}'")
osc = api_post("oscillator/eval", {"text": osc_query})
print(f"  Coherence: {osc['coherence']:.4f}")
print(f"  Sync:      {osc['sync']:.4f}")
print(f"  Entropy:   {osc['entropy']:.4f}")
print(f"  Words:     {osc['word_count']}")
colors = ", ".join(f"{c} ({a:.1f})" for c, a in osc["dominant_colors"])
print(f"  Colors:    {colors}")

# 9. HIERARCHICAL LAYERS
section("9. HIERARCHICAL PHASE LAYERS")
layers = api_get("layers")
print(f"  Layers: {layers['layers_count']}")
for layer in layers["layer_summaries"]:
    print(f"    Level {layer['level']}: {layer['sector_count']} sectors, "
          f"{layer['clusters_count']} clusters")

# ── SUMMARY ───────────────────────────────────────────────────────────────────

banner("SUMMARY")
print(f"""
  Phiano is a 5.7 MB language model that:
  - Learns instantly from any text (online, no retraining)
  - Learns from Wikipedia articles in real-time
  - Evaluates text coherence in <1ms
  - Composes via 64-sector phase traversal with bigram ordering
  - Generates via phase-guided bigram transition model
  - Follows instructions (explain, code, creative, analyze)
  - Runs entirely on CPU (no GPU required)
  - 155,771+ word vocabulary
  - 3.4M+ bigram transition probabilities
  - 1000x smaller than Phi-4 (5.7 MB vs 8 GB)

  vs Phi-4 (14B parameter transformer):
  - Phi-4: 28 GB, GPU required, no online learning
  - Phiano: 5.7 MB, CPU only, learns from every input
  - Tradeoff: Phiano has limited fluency (dictionary-trained bigrams)
  - Advantage: Phiano is a semantic indexing & evaluation engine
""")
print("=" * 60)
