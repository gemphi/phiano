# Persona Training Tasks

This folder contains training examples for the persona matching system.

## Structure

```
task/
├── README.md           - This file
├── examples/           - Text samples for each persona
│   ├── poet.txt        - Poetry-style writing samples
│   ├── engineer.txt    - Technical/engineering writing samples
│   ├── philosopher.txt - Philosophical writing samples
│   └── storyteller.txt - Narrative/fiction writing samples
├── match.txt           - Unknown texts to match against personas
└── run.txt             - REPL command sequence to train and match
```

## How to use

1. Ensure `data/manifold.chroma` exists (run `train 30` first)
2. Run the pipeline:
   ```
   cat task/run.txt | cargo run
   ```
3. This will:
   - Create personas from the example files
   - Compare all personas against each other
   - Attempt to match unknown texts to their authors
   - Impersonate each persona on a test prompt

## Adding new personas

1. Create `task/examples/<name>.txt` with writing samples (one per line)
2. Add persona creation commands to `run.txt`
3. Add match samples to `match.txt`

The system is generic - feed it anyone's text and it learns their
phase-space fingerprint. No hardcoded names or styles.
