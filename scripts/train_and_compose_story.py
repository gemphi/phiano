#!/usr/bin/env python3
"""
Phiano Live Dictionary Training & Story Composition Pipeline
Trains on the complete multi-domain definition of 'money' and generates a harmonic story.
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

print("=" * 80)
print("  PHIANO DICTIONARY INGESTION & STORY GENERATION PIPELINE")
print("=" * 80)

# Step 1: Read the complete structured definition of 'money'
with open("data/definitions/money_complete.txt", "r", encoding="utf-8") as f:
    definition_text = f.read()

print(f"\n[Step 1] Ingesting & Multi-Epoch Training ({len(definition_text)} chars)...")
train_res = call("learn_multi", {"text": definition_text, "epochs": 5, "warmup": 2})
print(f"  • Epochs Completed : {train_res.get('epochs')}")
print(f"  • Tokens Learned   : {train_res.get('tokens')}")
print(f"  • Total Vocabulary : {train_res.get('vocabulary')} words")

# Step 2: Test 3D Kuramoto Sphere Synchronization of the Learned Concepts
print("\n[Step 2] Measuring Multi-Sense Harmonic Synchronization on 3D Sphere...")
om_res = call("oscillator/eval", {"text": "money currency gold bank wealth income legal tender juno moneta"})
print(f"  • Coherence Score  : {om_res.get('coherence'):.4f}")
print(f"  • Sync Parameter   : {om_res.get('sync'):.4f}")
print(f"  • Spectral Entropy : {om_res.get('entropy'):.4f} bits")
print(f"  • Dominant Colors  : {om_res.get('dominant_colors')[:4]}")

# Step 3: RiverFlow Harmonic Sector Story Composition
print("\n[Step 3] Composing New Harmonic Story via RiverFlow Beam Search...")
story_prompt = "In the ancient temple of Juno Moneta the minted gold coins became legal tender for wealth and commerce"
comp_res = call("compose", {"text": story_prompt})

print(f"\n  --- COMPOSED STORY OUTPUT ---")
print(f"  Prompt : \"{story_prompt}\"")
print(f"  Story  :\n{comp_res.get('text')}")
print(f"  Sector : {comp_res.get('winning_sector')} ({comp_res.get('winning_color')})")
print(f"  Resonance Coherence: {comp_res.get('coherence'):.4f}")
print(f"  Verdict: {comp_res.get('verdict')}")

# Step 4: Execute John Searle Cognitive Explanation of Money
print("\n[Step 4] Querying Cognitive Speech Act & Intentional Stance...")
instruct_res = call("instruct", {"text": "explain how money functions as legal tender and wealth"})
print(f"  • Output:\n{instruct_res.get('output')}")

print("\n" + "=" * 80)
print("  TRAINING & STORY COMPOSITION COMPLETED SUCCESSFULLY!")
print("=" * 80)
