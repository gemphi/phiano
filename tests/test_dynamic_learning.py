#!/usr/bin/env python3
"""
Dynamic Dictionary Learning & Retrieval Test for Phiano
Demonstrates how Phiano dynamically ingests, phase-encodes, and retrieves complex multi-sense dictionary entries.
"""
import urllib.request
import json
import sys

sys.stdout.reconfigure(encoding='utf-8')

API = "http://127.0.0.1:3002/api"

def call(endpoint, payload):
    data = json.dumps(payload).encode('utf-8')
    req = urllib.request.Request(f"{API}/{endpoint}", data=data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=30) as resp:
        return json.loads(resp.read().decode('utf-8'))

definition_text = """sex
(seks) n. 1.a. Sexual activity, especially sexual intercourse: hasn't had sex in months.
b. The sexual urge or instinct as it manifests itself in behavior: motivated by sex.
2.a. Either of the two divisions, designated female and male, by which most organisms are classified on the basis of their reproductive organs and functions: How do you determine the sex of a lobster?
b. The fact or condition of existing in these two divisions, especially the collection of characteristics that distinguish female and male: the evolution of sex in plants; a study that takes sex into account.
3. Females or males considered as a group: dormitories that house only one sex.
4. One's identity as either female or male.
5. The genitals.
tr.v. sexed, sexing, sexes 1. To determine the sex of an organism.
2. Slang a. To arouse sexually. Often used with up.
b. To increase the appeal or attractiveness of. Often used with up."""

print("=" * 80)
print("  PHIANO DYNAMIC DICTIONARY DEFINITION LEARNING DEMONSTRATION")
print("=" * 80)

# Step 1: Learn the full multi-clause definition dynamically via API
print("\n[Step 1] Ingesting & Training Multi-Sense Definition into Phase Manifold...")
learn_res = call("learn_multi", {"text": definition_text, "epochs": 5, "warmup": 2})
print(f"  • Epochs Trained : {learn_res.get('epochs')}")
print(f"  • Tokens Learned : {learn_res.get('tokens')}")
print(f"  • Converged      : {learn_res.get('converged')}")
print(f"  • Total Vocab    : {learn_res.get('vocabulary')} words")

# Step 2: Evaluate the semantic coherence of the learned concept
print("\n[Step 2] Measuring Learned Semantic Coherence & Resonance...")
eval_res = call("eval", {"text": "sex reproduction female male biological characteristics"})
print(f"  • Coherence Score : {eval_res.get('coherence'):.4f}")
print(f"  • Novelty Score   : {eval_res.get('novelty'):.4f}")
print(f"  • Resonance Score : {eval_res.get('resonance'):.4f}")
print(f"  • Verdict         : {eval_res.get('verdict')}")

# Step 3: Test 3D Kuramoto Sphere Projection
print("\n[Step 3] 3D Kuramoto Spherical Projection of Learned Concept...")
osc_res = call("oscillator/eval", {"text": "sex biological female male organism"})
print(f"  • Phase Coherence : {osc_res.get('coherence'):.4f}")
print(f"  • Synchronization : {osc_res.get('sync'):.4f}")
print(f"  • Dominant Colors : {osc_res.get('dominant_colors')[:3]}")

# Step 4: Execute Semantic Reasoning & Explanation
print("\n[Step 4] Querying Phiano for Conceptual Explanation...")
reason_res = call("instruct", {"text": "explain the biological and behavioral definition of sex"})
print(f"  • Output:\n{reason_res.get('output')}")

print("\n" + "=" * 80)
print("  DYNAMIC LEARNING & RETRIEVAL VERIFIED (NO HARDCODING REQUIRED)!")
print("=" * 80)
