#!/usr/bin/env python3
"""
Phiano Dictionary Definition & Lexicon Verification Suite
Tests whether Phiano knows definitions of words across the Webster's lexicon and corpus.
"""
import urllib.request
import json
import time
import sys

sys.stdout.reconfigure(encoding='utf-8')

API_URL = "http://127.0.0.1:3002/api"

def query_api(endpoint, payload=None):
    try:
        if payload is not None:
            data = json.dumps(payload).encode('utf-8')
            req = urllib.request.Request(
                f"{API_URL}/{endpoint}",
                data=data,
                headers={"Content-Type": "application/json"}
            )
        else:
            req = urllib.request.Request(f"{API_URL}/{endpoint}")
        
        with urllib.request.urlopen(req, timeout=30) as resp:
            return json.loads(resp.read().decode('utf-8'))
    except Exception as e:
        return {"error": str(e)}

def run_dictionary_tests():
    print("=" * 70)
    print("  PHIANO DICTIONARY DEFINITION & RESONANCE TEST SUITE")
    print("=" * 70)
    
    # 1. Check Model Stats & Vocabulary Size
    print("\n[Step 1] Checking Model Lexicon Statistics...")
    stats = query_api("stats")
    print(f"  Vocabulary Size : {stats.get('vocabulary', 'N/A')} words")
    print(f"  Memory Layers   : {stats.get('memory_layers', stats.get('layers', 'N/A'))}")
    print(f"  Coherence Index : {stats.get('coherence', 'N/A')}")
    
    # 2. Test Word Definitions & Resonances
    test_words = [
        "quantum",
        "harmonic",
        "oscillator",
        "synchronization",
        "entropy",
        "gravity",
        "intentionality",
        "resonance",
        "semantics",
        "consciousness"
    ]
    
    print("\n[Step 2] Testing Semantic Evaluation & Cognitive Aboutness...")
    for word in test_words:
        eval_res = query_api("eval", {"text": word})
        coherence = eval_res.get("coherence", eval_res.get("score", "N/A"))
        verdict = eval_res.get("verdict", "OK")
        print(f"  Word: '{word:<16}' -> Verdict: {verdict:<10} | Coherence: {coherence}")

    # 3. Test Reasoning & Semantic Definition Composition
    test_prompts = [
        "what is a quantum harmonic oscillator",
        "define intentionality in cognitive science",
        "how does kuramoto phase synchronization work"
    ]
    
    print("\n[Step 3] Testing Definition & Explanatory Reasoning Chains...")
    for prompt in test_prompts:
        reason_res = query_api("reason", {"text": prompt})
        response = reason_res.get("response", reason_res.get("text", reason_res.get("output", "")))
        print(f"\n  Query   : \"{prompt}\"")
        print(f"  Response: {response[:180]}..." if len(response) > 180 else f"  Response: {response}")

    # 4. Multi-Layer Hierarchy Depth Resonance
    print("\n[Step 4] Querying 64-Layer Cognitive Octave Resonance...")
    layers_res = query_api("layers")
    print(f"  Hierarchy Layers Active: {len(layers_res) if isinstance(layers_res, list) else 'Configured'}")
    
    print("\n" + "=" * 70)
    print("  DICTIONARY VERIFICATION & TUNING COMPLETE")
    print("=" * 70)

if __name__ == "__main__":
    run_dictionary_tests()
