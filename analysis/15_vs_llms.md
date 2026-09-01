# 15 — Phiano vs. Large Language Models: Claims vs. Reality

> Reference point: the repository's own comparison documents —
> [`docs/45_native_learning_vs_bloated_llms.md`](../docs/45_native_learning_vs_bloated_llms.md)
> (vs. Phi-4 14B and GLM-5.2 744B-MoE), the reference-model assets in [`refs/`](../refs),
> and the inert GGUF files in [`models/`](../models). This file audits that comparison
> line by line, because it is the document most likely to be shown to outsiders.

---

## 1. The Repository's Comparison Table, Audited

docs/45 §1 presents seven dimensions. Audit of each claim:

| docs/45 claim | Audit verdict |
|---|---|
| "155,774 dynamic phasors (~39 MB RAM)" vs Phi-4's 14B / GLM's 744B params | **Misleading comparison of unlike things.** A phasor is 16 bytes of *state*; a transformer parameter is one coefficient in a *function*. The honest comparison is capability per byte: Phiano's 39 MB yields lexical memory + trigram generation; Phi-4's 28 GB yields general generation + reasoning. Both facts are true; juxtaposing the raw counts implies equivalence that does not exist |
| "O(1) multi-turn superposition wave buffer" vs 16K KV-cache | **True and genuinely advantageous** (file 03 §1) — the strongest row in the table |
| "Online Kuramoto coupling Δt ≤ 1 ms" vs offline SGD | **True** (file 11 §1: measured ≈ 2.3 µs/update aggregate) — the second-strongest row |
| "Catastrophic forgetting: Zero" vs "Severe" | **Half-true** (file 10 §3): vocabulary yes; associations drift. "Severe" for LoRA-replay-free fine-tuning is fair, but "Zero" overstates |
| "Sub-millisecond on standard laptop CPU" (inference) | **Conflates update with inference.** Learning a sentence: yes. *Producing a comparable answer*: no comparison exists — and quality, not latency, is what users measure |
| "Destructive wave interference" as semantic distance vs cosine similarity | Both are valid metrics; neither is inherently superior. ComplEx/RotatE literature shows complex metrics earn their keep at **high dimension** — Phiano runs them at d=1 (file 12) |
| "Ingests Phi-4 vocab without inheriting bloat" | **True as stated** — but it inherits *vocabulary*, not the model's knowledge or capability ([src/sources/phi4.rs](../src/sources/phi4.rs) reads `vocab.json`/`merges.txt` text assets; the GGUFs in `models/` are never loaded — file 01 §3) |

## 2. The Correct Comparison Frame

The docs frame Phiano as an *alternative* to LLMs ("native learning vs. bloated
LLMs"). The audit supports a different, stronger frame: **Phiano and LLMs are
complements occupying opposite ends of the learning spectrum.**

| Property | Phiano | LLM (Phi-4 class) |
|---|---|---|
| Learning mode | Online, per-sentence, permanent | Frozen at deployment |
| Knowledge volume | 10⁴–10⁵ associations (P0) | 10⁹+ facts, compressed |
| Reasoning depth | Proximity chaining | Multi-step, tool-augmented |
| Generation | Trigram + phase steering, ≤20 tokens | Fluent, long-form |
| Personalization | **Native, instant, inspectable** | RAG emulation only |
| Resource floor | MB-class, CPU | GB-class, GPU-preferred |
| Failure mode | Word salad (obviously wrong) | Fluent hallucination (wrongly confident) |
| Auditability | Bit-exact replay of any output | Effectively unauditable |

The final two rows are underrated: Phiano *fails visibly*, and its every output is
reproducible. LLMs fail *fluently* and are non-deterministic by default. For
high-trust niches (file 13 §5) that difference is worth more than raw capability.

## 3. The Hybrid Architecture (where the comparison should end)

The strongest system the audit can defend uses each for what it is:

```text
User ──► Phiano manifold (always-on)
            ├─ answers lexical/associative queries locally (~80% of queries in
            │  assistant workloads are lookup-shaped — routing, definitions,
            │  "what did I tell you about X")
            ├─ maintains personal/user state (the LLM's missing hippocampus)
            └─ escalates reasoning queries ──► LLM (Phi-4/GLM, cloud or local GGUF)
                    └─ responses distilled back into the manifold
                       (wiki_bulk / sources/api ingestion — already built)
```

This is not a concession — it is the deployment the codebase is *already shaped for*:
31 REST endpoints, ingestion sources for reference models, an SSE visualizer, and a
2 MB exportable knowledge artifact. As the **hippocampus layer for a stack whose
cortex is an LLM**, Phiano needs zero architectural change to be useful (file 16,
task 10 makes it a first-class product).

## 4. Where the "vs. Transformer" Docs Are Right

Three critiques in docs/45–61 land cleanly and deserve to be kept:

1. **Static-paradigm critique** — frozen post-training parameters + replay-buffer
   patches is genuinely awkward for personalization and streams (files 10, 13).
2. **KV-cache memory linearity** — O(1) vs O(L) context cost is a real structural
   difference (file 03 §1).
3. **Training-economics critique** — per-user/per-domain adaptation by fine-tuning is
   economically absurd; per-sentence online state updates are economically invisible
   (file 11).

## 5. Scorecard for the Comparison Documents Themselves

| Criterion | Grade |
|---|---|
| Technical accuracy of mechanism descriptions (transformer side) | B |
| Technical accuracy of Phiano-side claims | C+ (forgetting, inference-latency, and capability-equivalence overstate) |
| Apples-to-apples discipline | D (state bytes vs function parameters) |
| Strategic framing | C ("replacement" framing undersells the actual complement advantage) |
| Persuasiveness to an informed reader | C- (inflated rows are the first thing an expert checks) |

**Bottom line:** the honest comparison is not "155k phasors beat 14B parameters" — it
is that **Phiano implements the learning half of intelligence that LLMs outsource, and
LLMs implement the competence half that Phiano has not built.** The repository's
materials should lead with the complement story (which the code already supports)
rather than the replacement story (which the code cannot yet support).
