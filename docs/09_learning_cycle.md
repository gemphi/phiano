# 09 - Recursive Learning Cycle

```
  ┌──────────────────────────────────────────────────────────────┐
  │                                                              │
  │     ┌──────────┐                                             │
  │     │ ENVISION │ ◄─────────────────────────────┐            │
  │     └────┬─────┘                                │            │
  │          │                                      │            │
  │          │ detect unknown words                 │            │
  │          │ suggest related known words          │            │
  │          ▼                                      │            │
  │     ┌──────────┐                                │            │
  │     │  APPLY   │                                │            │
  │     └────┬─────┘                                │            │
  │          │                                      │            │
  │          │ train on input                       │            │
  │          │ (Kuramoto phase relaxation)          │            │
  │          ▼                                      │            │
  │     ┌──────────┐                                │            │
  │     │   EVAL   │                                │            │
  │     └────┬─────┘                                │            │
  │          │                                      │            │
  │          │ score: coherence, novelty, resonance │            │
  │          │ produce verdict                      │            │
  │          ▼                                      │            │
  │     ┌──────────┐                                │            │
  │     │ ITERATE  │ ──────────────────────────────►│            │
  │     └────┬─────┘       (next user input)        │            │
  │          │                                      │            │
  │          │ on exit:                             │            │
  │          ▼                                      │            │
  │     ┌──────────┐                                │            │
  │     │  SCALE   │                                │            │
  │     └──────────┘                                │            │
  │     persist facet + memory to disk              │            │
  │                                                              │
  └──────────────────────────────────────────────────────────────┘
```

## Per-Input Flow in Code

```
  User types: "the cat sat on the mat"
         │
         ▼
  Model::iterate()
         │
         ├─► Dispatcher::dispatch()
         │         │
         │         ├─► Command::Unknown → Learn::default()
         │         │         │
         │         │         ├─► Trainer::train_online()  ← APPLY
         │         │         ├─► Evaluator::eval()        ← EVAL
         │         │         └─► Envision::detect_gaps()  ← ENVISION
         │         │
         │         └─► (or specific command handler)
         │
         └─► Model::envision()  ← second envision pass
                    │
                    ▼
               detect gaps → suggest definitions
```
