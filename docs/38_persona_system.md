# 38 — Persona: Fingerprint, World, Impersonation

```
┌──────────────────────────────────────────────────────────────────┐
│                    PERSONA ARCHITECTURE                          │
│                                                                  │
│   ┌──────────┐    ┌──────────────┐    ┌───────────────┐         │
│   │ Examples │───→│  Fingerprint │───→│ Impersonator  │         │
│   │ (text)   │    │  (sector     │    │ (biased flow) │         │
│   └──────────┘    │   histogram) │    └───────┬───────┘         │
│                   └──────┬───────┘            │                 │
│                          │                    │                 │
│                          ▼                    ▼                 │
│                   ┌──────────────┐    ┌───────────────┐         │
│                   │    World     │    │  Composition  │         │
│                   │ (persona     │    │  in persona's │         │
│                   │  collection) │    │  style        │         │
│                   └──────────────┘    └───────────────┘         │
│                          │                                      │
│                          ▼                                      │
│                   ┌──────────────┐                              │
│                   │  Comparison  │                              │
│                   │ (similarity, │                              │
│                   │  differences)│                              │
│                   └──────────────┘                              │
└──────────────────────────────────────────────────────────────────┘
```

## How personas work

A persona is NOT a hardcoded character. It is a phase-space fingerprint
extracted from text examples. The fingerprint captures:

1. **Sector histogram** — which sectors the persona's words cluster in
2. **Amplitude profile** — how familiar the persona is with each sector
3. **Dominant sectors** — where their words cluster most
4. **Diversity (entropy)** — how spread out their style is

## Creating a persona

```
persona add poet "the moon whispers silver" "stars fall like tears"
```

1. Trainer learns the examples (Kuramoto re-tunes the facet)
2. Fingerprint is extracted from the resulting phase distribution
3. Each example → wave → sector; histogram accumulates
4. Word-level sectors also contribute (at lower weight)

## Comparing personas

```
persona compare poet engineer
```

- **Cosine similarity** on sector histograms (0.0 = different, 1.0 = identical)
- **Difference vector** shows which sectors each persona is stronger in

Example from test:
```
poet vs engineer
similarity: 0.171 (very different)
poet stronger:     lime, violet, emerald (organic, soft)
engineer stronger: scarlet, yellow, green (sharp, technical)
```

## Impersonation

```
persona impersonate poet "the night sky"
```

1. Generate variations biased toward persona's dominant sectors
2. Score each by quality AND persona fit (how close to fingerprint)
3. Keep the best, train on them
4. Recurse until convergence

The `bias_strength` parameter (0.0-1.0) controls how strongly the
composition is pulled toward the persona's characteristic sectors.

## Module structure

```
src/persona/
├── mod.rs          — Persona struct, from_examples()
├── fingerprint.rs  — Fingerprint: extract, similarity, difference_vector
├── impersonate.rs  — Impersonator: biased composition + persona fit scoring
└── world.rs        — PersonaWorld: collection, compare, PersonaComparison
```

## REPL commands

```
persona add <name> "ex1" "ex2" ...       — Create persona from examples
persona list                             — List all personas
persona show <name>                      — Show fingerprint
persona compare <a> <b>                  — Compare two personas
persona impersonate <name> "prompt"      — Compose as persona
```

## File references

- `src/persona/mod.rs` — Persona struct
- `src/persona/fingerprint.rs` — Fingerprint extraction and comparison
- `src/persona/impersonate.rs` — Impersonator, ImpersonationResult
- `src/persona/world.rs` — PersonaWorld, PersonaComparison
- `src/command/persona.rs` — REPL command handler
