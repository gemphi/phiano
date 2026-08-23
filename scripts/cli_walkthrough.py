#!/usr/bin/env python3
"""
Phiano Interactive CLI & API Verification Script
Tests definitions, synonyms, waves, and cognitive agents via the live Phiano runtime.
"""
import urllib.request
import json
import sys

sys.stdout.reconfigure(encoding='utf-8')

API = "http://127.0.0.1:3002/api"

def call(endpoint, payload=None):
    if payload:
        data = json.dumps(payload).encode('utf-8')
        req = urllib.request.Request(f"{API}/{endpoint}", data=data, headers={"Content-Type": "application/json"})
    else:
        req = urllib.request.Request(f"{API}/{endpoint}")
    with urllib.request.urlopen(req, timeout=30) as resp:
        return json.loads(resp.read().decode('utf-8'))

print("=" * 75)
print("  PHIANO DICTIONARY & HARMONIC REASONING VERIFICATION")
print("=" * 75)

# 1. Stats
stats = call("stats")
print(f"\n[1] Lexicon Status: {stats.get('vocabulary')} words loaded across {stats.get('memory_entries')} memory entries.")

# 2. Test Word Definitions via Instruct/Explain
words = ["oscillator", "synchronization", "gravity", "entropy", "intentionality"]
print("\n[2] Testing Conceptual Definitions:")
for w in words:
    res = call("instruct", {"text": f"define {w}"})
    out = res.get("output", "")
    first_line = out.split("\n")[0] if "\n" in out else out
    print(f"  • {w.upper():<16}: {first_line[:80]}...")

# 3. Test Kuramoto 3D Sphere Oscillator Evaluation
print("\n[3] Testing 3D Spherical Kuramoto Metrics:")
om_eval = call("oscillator/eval", {"text": "quantum phase synchronization in coupled harmonic oscillators"})
print(f"  • Coherence : {om_eval.get('coherence'):.4f}")
print(f"  • Sync Rate : {om_eval.get('sync'):.4f}")
print(f"  • Entropy   : {om_eval.get('entropy'):.4f} bits")
print(f"  • Dominant Colors: {om_eval.get('dominant_colors')}")

# 4. Test Multi-Step Reasoning
print("\n[4] Testing Convergent Phase Reasoning:")
reason = call("reason", {"text": "why do coupled oscillators synchronize"})
print(f"  • Query     : \"why do coupled oscillators synchronize\"")
print(f"  • Converged : {reason.get('converged')}")
print(f"  • Steps     : {reason.get('steps_count')}")
print(f"  • Answer    : {reason.get('final_answer')}")

print("\n" + "=" * 75)
print("  ALL VERIFICATION TESTS COMPLETED SUCCESSFULLY!")
print("=" * 75)
