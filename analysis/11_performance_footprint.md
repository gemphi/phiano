# 11 — Performance and Resource Footprint: The Real Numbers

> Sources: [`docs/rust_mastery/06_phiano_rust_benchmark_results.md`](../docs/rust_mastery/06_phiano_rust_benchmark_results.md)
> (the repository's only measured benchmark run), [`docs/how/13_persistence_and_cost.md`](../docs/how/13_persistence_and_cost.md)
> (design/target numbers), on-disk artifacts in [`data/`](../data), and complexity
> analysis of the code.

This file separates **measured**, **derivable**, and **target** numbers — the
repository's documentation blends them, which is the main honesty tax on its
performance story.

---

## 1. Measured (Rust Book benchmark, docs/rust_mastery/06)

| Metric | Value |
|---|---|
| Corpus | 104 Rust Book chapters → **13,958 clean sentences** |
| Ingestion time | **~0.4 s** |
| Training | 3 epochs, **1.11 s total** (< 370 ms/epoch), 488,052 phasor updates (162,684/epoch) |
| Vocabulary learned | **6,993 unique terms** |
| Model in RAM | **~27 MB** (phasor table) |
| Hardware | AMD Ryzen 9 5900X (per docs/papers/07) |
| Post-training coherence on 4 in-domain prompts | 1.0000 (see file 14 §3 for the caveat) |
| Reasoning convergence | 3 steps, phase shifts 0.0446 → 0.0159 → 0.0081 rad |

**Derived throughput:** ~488k phasor updates in 1.11 s ≈ **440k updates/sec** on a
single desktop CPU, release build, rayon-parallel ray-casts ([wave.rs](../src/wave.rs)
`par_iter`). Per-sentence training is **O(L)** — genuinely sub-millisecond at L ≤ 30.

## 2. Measured (On-Disk Reality Today)

| Artifact | Size | Content |
|---|---|---|
| `data/manifold.chroma` | **92,157,679 B (~92 MB)** | Full facet incl. n-gram tables |
| `models/phi4-mm-Q4_K_M.gguf` | ~9 GB class | **Inert** — no code loads it (file 01 §3) |

## 3. Derivable (Complexity Table from Code)

| Operation | Complexity | Where |
|---|---|---|
| Train one sentence | O(L) | [`train_sentence`](../src/trainer/mod.rs) |
| Add one word | O(1) | `initialize_tokens` |
| Ray-cast (top-k over lexicon V) | O(V) parallel, O(V + k log k) | [`Wave::ray_cast`](../src/wave.rs) |
| Full-lexicon nearest neighbors (Aboutness agent) | O(|prompt|·V) | [`AboutnessAgent`](../src/cognitive/intentionality.rs) |
| Generate one token | O(candidates + ray-cast pool 192) | [`attractor_select`](../src/generate.rs) |
| Oscillator train epoch | O(V²) pairwise | [`train_epoch`](../src/oscillator/train.rs) |
| Memo record | O(1) amortized | [`Memo::record`](../src/memory/mod.rs) |
| Eval one text | O(L + 1) | [`Evaluator::eval`](../src/eval.rs) |

Scaling hazards worth naming: the **O(V²) oscillator trainer** at V = 150k is 2.25·10¹⁰
pairwise terms per epoch (the field view is for slices, not the whole lexicon); and
**AboutnessAgent** is O(|prompt|·V) per chat turn — fine at 7k words, quadratically
uncomfortable at 150k. The core paths (train/ray-cast/generate) are all linear or
better. ✔

## 4. Targets (docs/how/13 — *not yet realized*)

| Claimed target | Status |
|---|---|
| Learn one sentence ~1 µs | Plausible mechanism-wise; not benchmarked at that granularity (measured aggregate ≈ 2.3 µs/update incl. bookkeeping) |
| Ray-cast ~100 µs at V = 10⁵ | Plausible with rayon; only 7k-V measured |
| Generate one token 100 µs–1 ms | Plausible; not isolated-benchmarked |
| 92 MB → 6–10 MB after interning | **Design, not code** — the single most valuable storage task |
| "~2/5/12 MB" Phinum footprints (README) | **Not realized** — presumes the interning refactor *and* dropping n-gram tables that generation currently depends on |

## 5. The Honest Comparison Table (vs. actually-running alternatives)

| Dimension | Phiano (measured) | Phi-4 14B Q4 (GGUF, local) | 7B int4 (llama.cpp class) |
|---|---|---|---|
| RAM | 27 MB (7k vocab) / ~40 MB @150k phasors | ~9–10 GB | ~4 GB |
| Disk state | 92 MB today → 6–10 MB planned | ~9 GB | ~3.5–4 GB |
| Learn from user, live | **Native, µs** | No | No |
| Token generation | µs-class per candidate, 20-token cap, trigram quality | ~10–30 tok/s CPU, transformer quality | ~5–20 tok/s CPU |
| Deterministic/reproducible | **Yes, bit-exact** | Yes given seed | Yes given seed |
| GPU required | **No** | No (CPU), better with | No (CPU) |

The two right-hand columns are included because the models/ directory shows the author
considered local GGUF serving; the comparison makes Phiano's *actual* niche precise:
**two-to-three orders of magnitude less memory, two-to-three orders faster learning,
one-to-two orders weaker generation.**

## 6. Scorecard

| Aspect | Grade | Note |
|---|---|---|
| Training throughput | **A** | 440k updates/s CPU, verified |
| Memory footprint (model state) | **A−** | 16 B/word is excellent; 92 MB total is n-gram freight |
| Footprint claims honesty | **C** | README/PLAN numbers are post-refactor targets presented as present facts |
| Inference latency | **B** | Sub-ms steering; but generation quality, not latency, is the binding constraint |
| Scaling analysis | **B−** | Linear cores; two quadratic paths unbounded |
| Benchmark discipline | **C+** | One good measured run; no repetition/variance reporting; coherence metric self-referential |

**Bottom line:** on *its own* terms — learning speed and state size per learned word —
Phiano is extraordinary and the numbers are real. The performance story is taxed by
(1) presenting planned footprints as current ones and (2) never benchmarking the
generation path that users actually feel. Fixing (2) is file 16, task 4.
