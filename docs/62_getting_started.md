# Getting Started with Phiano (5 minutes)

Phiano is a phase-coupled language engine. Words live on a C³² torus; generation is Kuramoto coupling, not attention.

## 1. Install

Requires Rust 1.75+ and (for the PUI) Node 18+.

```bash
git clone https://github.com/gemphi/phiano.git
cd phiano
cargo build --release
```

## 2. Bootstrap a facet (or reuse the existing one)

A **facet** is the trained lexicon (`.chroma` file). `data/manifold.chroma` is loaded automatically if present.

```bash
# Train a default facet from curriculum + rust-book corpus (first 4000 sentences)
cargo run --release --bin bootstrap_facet

# Custom: cargo run --release --bin bootstrap_facet -- data/rust_book_corpus.txt 8000 data/manifold.chroma
```

## 3. Talk to it

**REPL**

```bash
cargo run --release
```

```
phiano> learn "the mushroom is growing in the forest"
phiano> eval "the mushroom is growing"
phiano> exit
```

**Web PUI** (API + dashboard)

```bash
# Terminal 1 — API on :3000, serves web/dist
cargo run --release -- --web

# Terminal 2 — Vite dev server (hot reload)
cd web && npm install && npm run dev
```

Open `http://localhost:5173`. Use **Cognitive Chat**, **Phase Topology**, and **Math Symbols**.

## 4. Generate and stream

```bash
# One-shot
curl -s -X POST http://127.0.0.1:3000/api/generate \
  -H "Content-Type: application/json" \
  -d "{\"text\":\"the child learns language\",\"max_tokens\":16}"

# Token stream (SSE) — each event is {token, step, collective_phase, resonance, done}
curl -N -X POST http://127.0.0.1:3000/api/generate/stream \
  -H "Content-Type: application/json" \
  -d "{\"text\":\"the child learns language\",\"max_tokens\":16}"
```

## 5. Train from Wikipedia, Phi-4, and dialogue

```bash
# CLI: curriculum + rust-book + dialogue + Phi-4 refs + 24 wiki topics
cargo run --release --bin ingest -- --wiki 24

# Skip network / skip Phi-4
cargo run --release --bin ingest -- --no-wiki --no-phi4

# Live API (server must be running)
curl -s -X POST http://127.0.0.1:3000/api/ingest \
  -H "Content-Type: application/json" \
  -d "{\"wiki_topics\":12,\"phi4\":true,\"dialogue\":true,\"curriculum\":true}"

# One Wikipedia article
curl -s -X POST http://127.0.0.1:3000/api/wiki/learn \
  -H "Content-Type: application/json" \
  -d "{\"topic\":\"Kuramoto model\"}"

# Phi-4 vocab + merges + docs only
curl -s -X POST http://127.0.0.1:3000/api/phi4/learn
```

Then `POST /api/save` (or `ingest` writes `data/manifold.chroma` itself).

## 6. Benchmark

```bash
cargo run --release --bin bench
# or: cargo run --release --bin bench -- data/manifold.chroma
```

Prints coherence / novelty / resonance on 8 standard prompts plus a generated continuation.

## What just happened

| Step | What Phiano did | Transformer analog |
|------|-----------------|--------------------|
| Bootstrap | Kuramoto-trained a facet, saved `.chroma` | Pre-trained checkpoint |
| Learn | Hebbian phase shift on one sentence | Fine-tune (minutes–hours) |
| Generate | PhaseFlow + torus ray-cast | Autoregressive sampling |
| Stream | SSE of tokens + φ / R | Token streaming |
| Bench | Coherence / novelty / resonance | Perplexity / BLEU |

Next: [Configuration Guide](63_configuration.md) · [API Reference](API_REFERENCE.md) · [Phiano vs Transformers](46_vs_transformer_problem.md)
