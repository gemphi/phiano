import urllib.request, json, sys
sys.stdout.reconfigure(encoding='utf-8')

BASE = "http://127.0.0.1:3002/api"

def post(endpoint, data):
    req = urllib.request.Request(
        f"{BASE}/{endpoint}",
        data=json.dumps(data).encode(),
        headers={"Content-Type": "application/json"},
    )
    resp = urllib.request.urlopen(req, timeout=120)
    return json.loads(resp.read())

print("=== COGNITIVE: DIRECT SPEECH ACT ===")
r = post("cognitive", {"text": "explain how ownership works in rust"})
print(f"Synthesized: {r['synthesized_output']}")
print(f"Speech act: {r['speech_act']}")
print(f"Direction of fit: {r['direction_of_fit']}")
print(f"Propositional content: {r['propositional_content']}")
print(f"Perlocutionary effect: {r['perlocutionary_effect']}")
print(f"Literal meaning: {r['literal_meaning'][:60]}")
print(f"Speaker meaning: {r['speaker_meaning'][:60]}")
print(f"Satisfaction: {r['satisfaction']:.0%}")
print(f"\nFelicity conditions:")
fc = r['felicity_conditions']
print(f"  Propositional content rule: {fc['propositional_content_rule']}")
print(f"  Preparatory condition: {fc['preparatory_condition']}")
print(f"  Sincerity condition: {fc['sincerity_condition']}")
print(f"  Essential condition: {fc['essential_condition']}")
print(f"  Satisfied: {fc['satisfied']}")
print(f"\nIntentional states ({len(r['intentional_states'])}):")
for s in r['intentional_states']:
    mode = s.get('mode', '?')
    if isinstance(mode, dict):
        mode = mode.get('type', str(mode))
    print(f"  [{mode}] {s['content'][:60]}")
    dof = s.get('direction_of_fit', '?')
    if isinstance(dof, dict):
        dof = dof.get('type', str(dof))
    print(f"    DoF: {dof} | Satisfaction: {s['satisfaction_condition'][:50]}")

print("\n=== COGNITIVE: INDIRECT SPEECH ACT ===")
r = post("cognitive", {"text": "can you explain what is knowledge"})
print(f"Speech act: {r['speech_act']}")
print(f"Literal: {r['literal_meaning'][:60]}")
print(f"Speaker: {r['speaker_meaning'][:80]}")
print(f"Propositional content: {r['propositional_content']}")
print(f"Perlocutionary: {r['perlocutionary_effect']}")

print("\n=== COGNITIVE: EXPRESSIVE ===")
r = post("cognitive", {"text": "i think knowledge is beautiful"})
print(f"Speech act: {r['speech_act']}")
print(f"Direction of fit: {r['direction_of_fit']}")
print(f"Intentional states: {len(r['intentional_states'])}")
for s in r['intentional_states']:
    mode = s.get('mode', '?')
    if isinstance(mode, dict):
        mode = mode.get('type', str(mode))
    print(f"  [{mode}] {s['content'][:60]}")

print("\n=== COGNITIVE: COMMISSIVE ===")
r = post("cognitive", {"text": "i will explain the concept of justice"})
print(f"Speech act: {r['speech_act']}")
print(f"Direction of fit: {r['direction_of_fit']}")
for s in r['intentional_states']:
    mode = s.get('mode', '?')
    if isinstance(mode, dict):
        mode = mode.get('type', str(mode))
    print(f"  [{mode}] {s['content'][:60]}")

print("\n=== COGNITIVE: SOCIAL ONTOLOGY ===")
r = post("cognitive", {"text": "explain how money and property work"})
print(f"Synthesized: {r['synthesized_output']}")
for a in r['agent_outputs']:
    if a['agent_name'] == 'SocialOntology':
        print(f"SocialOntology: {a['output']}")

print("\n=== ALL 16 AGENTS ===")
r = post("cognitive", {"text": "what is knowledge"})
print(f"Agent count: {len(r['agent_outputs'])}")
for a in r['agent_outputs']:
    print(f"  [{a['agent_name']}] ({a['confidence']:.0%}) {a['output'][:80]}")
