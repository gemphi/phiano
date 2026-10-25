# 16 — Roadmap to Power: The Concrete Program

> This file converts the entire audit into a sequenced task list. Ordering principle:
> **credibility and measurement first, capacity second, competence third** — because
> every later stage depends on the earlier stages' evidence being believed.
> Effort labels: S ≈ days, M ≈ weeks, L ≈ months (single experienced Rust engineer).

---

## Stage 0 — Truth Infrastructure (make the evidence believable)

**Task 1 — Real evaluation harness.** [S–M]
Replace self-referential headline metrics with external ones:
- held-out perplexity/bpw against a trigram-only ablation and a hash-embedding baseline
  (the harness in [`src/metrics/`](../src/metrics) already has splits/eval_split);
- real ARC-format grids (public 400-task set) with exact-output match, executed by the
  program-synthesis path ([`src/synthesis/`](../src/synthesis)) — retire the
  coherence+substring criterion ([`src/metrics/arc.rs`](../src/metrics/arc.rs));
- a **fact-retention benchmark**: teach N facts, train on M unrelated sentences,
  measure recall vs M — the missing forgetting curve (file 10 §3).
*Exit criterion: every number on the README maps to a reproducible command.*

**Task 2 — State interning.** [M]
String interning + `u32` word ids across [`Facet`](../src/facet/mod.rs) tables
(92 MB → 6–10 MB per docs/how/13). Makes the advertised footprints true, cuts load
times, and unblocks edge deployments (file 13 §2). Add format versioning while there.

**Task 3 — Familiarity-gated plasticity + consolidation.** [S]
One line in [`train_sentence`](../src/trainer/mod.rs): `lr_i ∝ 1/A_i` — entrenched
words stiffen; recent words stay plastic. Pair with a periodic rehearsal pass over
high-amplitude clusters (the band-ratchet already gestures at consolidation). This is
the retention fix's mechanism half; Task 1's benchmark proves it.

**Task 4 — The ablation the project has never run.** [S]
A `--no-phase` generation flag: trigram/bigram only, phase steering disabled. Measure
quality delta on held-out text. This single experiment either quantifies the phase
machinery's contribution (the novel claim of the whole project — file 03 §4, file 08 §7)
or saves months by revealing the steering is currently decorative. Either result is
progress.

## Stage 1 — Wire the Assets That Already Exist (capability without new theory)

**Task 5 — Memo recall into generation.** [S–M]
Before decoding, retrieve top-k [`Memo`](../src/memory/mod.rs) entries by
context-wave resonance and phase-kick the decoder toward them. Converts the write-only
diary (file 05 §4) into episodic memory — the single largest behavioral gain per line
of code in the repository.

**Task 6 — Spider-net as discourse planner.** [M]
Feed [`spider_net.type_links`](../src/phinum/topology/spider_net.rs) transitions into
[`Generator::decode`](../src/generate.rs) as a sentence-type plan (assertive →
directive → …). Removes the 20-token single-sentence ceiling's root cause: nothing
plans beyond one sentence (file 08 §6). Also raise the cap; length is currently a
constant, not a property.

**Task 7 — Implement or re-scope the advertised stubs.** [S]
`Map`/`Reduce`/`Compose` ops in [`synthesis/program.rs`](../src/synthesis/program.rs);
wire [`heuristic.rs`](../src/synthesis/heuristic.rs) into the search;
un-dead-code [`attention_pick`](../src/generate.rs) or delete it. Then grow
[`ComponentLibrary`](../src/synthesis/library.rs) + relational analogy
([`program_analogy.rs`](../src/reasoning/program_analogy.rs)) toward real
program induction — the file-09 diamond, and the L2 seed.

**Task 8 — Operational hardening.** [M]
API auth + rate limits ([`src/server/`](../src/server)); correction-pulse guardrails
(chat-driven manifold vandalism — file 14 §4); snapshot/rollback of `manifold.chroma`;
async true streaming (retire the 40 ms replay, file 08 §5).

**Task 9 — Naming honesty pass.** [S]
Rename per file 06 §6 / file 14 §5: "variation buckets" for Phinum engines,
"hexagram labels" for I Ching, "fixed phase-spotlight pooling" for attention,
template synthesis as templates. Update README/PLAN footprints to post-Task-2 truth.
Zero code risk; maximal credibility yield with exactly the expert audience that
matters.

## Stage 2 — Capacity: The Rotor-Vector Upgrade (P1 → P2 of file 12 §6)

**Task 10 — d-dimensional phasors.** [L]
`SpectralPhasor` → `RotorVector { phases: [f64; D] }` with D = 32–256, each dimension
seeded by golden-ratio angles, trained by the *same* per-dimension Kuramoto pull.
This is the RotatE/ComplEx regime (file 12 §4) with online updates — the difference
between 10⁴ and 10⁶-word capacity. Sequential rollout: keep d=1 fast path for edge,
d=D for server. All similarity/ray-cast/eval code generalizes mechanically (they are
already norm/cosine based — [wave.rs](../src/wave.rs)).

**Task 11 — Binding via phase rotation.** [L]
Generalize β_ij scalar lags to per-relation phase rotations: represent
`relation(w_a, w_b)` as a learned rotation vector; compose relations by rotation
addition (VSA binding, file 12 §4). Sentence encoding becomes *order-aware*
(fixing file 12 §3.1): "dog bites man" ≠ "man bites dog" because the rotations differ.
Train rotations by the existing EMA rule — no new optimizer needed.

**Task 12 — Contrastive negative sampling.** [M]
Alongside positive pulls, sample k random words per sentence and apply *anti-phase*
repulsion (the [`correct_mistake`](../src/trainer/mod.rs) mechanism, repurposed).
This is the known fix that makes complex embeddings sharp at scale (RotatE's
self-adversarial sampling), it reuses machinery the project already has, and it
directly attacks phase-neighborhood collisions (file 12 §2).

## Stage 3 — Competence: Close the Loop (P3 → P4)

**Task 13 — Trained readout.** [L]
A small learned layer (logistic regression → tiny MLP) over phase features for
concrete tasks: next-word re-ranking, speech-act classification replacing keyword
rules, OOD detection. Fixed dynamics + trained readout is the reservoir-computing
pattern with universal-approximation pedigree (file 12 §4); it can run on-CPU in
microseconds and does not betray the architecture.

**Task 14 — Execution feedback loop.** [L]
Extend the compose tournament (the system's only working selection loop, file 08 §4)
into a general propose→execute→verify→reinforce cycle: programs run on held-out
inputs; generated claims checked against retrieved facts; user accept/correct signals
(from chat) drive both pull and anti-phase. This installs credit assignment — the
L3 gate (file 12 §3.2) — without backpropagation through the manifold.

**Task 15 — Meta-plasticity.** [L]
Promote the remaining fixed constants (the 0.7/0.3 mix, kick strength, momentum rates)
to *learned* state via the same EMA that learns β_ij, tuned by Task 1's external
metrics. When the coupling itself learns, the system crosses from adaptive state to
adaptive dynamics — the formal threshold of "a system that learns how to learn"
(file 12 §5).

**Task 16 — The hybrid product.** [M]
Ship the file-15 §3 architecture: Phiano as the always-on hippocampus (personal
memory, routing, 80% lexical queries) fronting Phi-4-or-better as the reasoning
cortex, with automatic distillation of LLM answers back into the manifold via the
existing [`sources/`](../src/sources) ingestion. This is the deployment the current
codebase is already shaped for, and the one no incumbent ships.

---

## Dependency Graph and Sequencing

```text
Stage 0 (1,2,3,4) ──► Stage 1 (5,6,7,8,9) ──► Stage 2 (10,11,12) ──► Stage 3 (13,14,15)
                                │
                                └──(16: hybrid product can ship as early as end of Stage 1)
```

- Tasks 1+4 unblock everything: without honest measurement, Stage 2's gains are
  unprovable.
- Tasks 5+6 deliver the largest user-visible jump per effort.
- Task 10 is the long pole; start its design doc during Stage 1.
- Task 16 is deliberately stage-independent: the hybrid is valuable at P0 and
  becomes more valuable at every stage.

## What Success Looks Like at Each Gate

| Gate | Evidence required |
|---|---|
| End of Stage 0 | README numbers reproduce; ablation quantifies phase contribution; forgetting curve published |
| End of Stage 1 | Multi-sentence planned generation; episodic recall demo ("remember what I told you"); hardened API; honest docs |
| End of Stage 2 | 10⁵–10⁶-word manifold with clean nearest-neighbor geometry; order-aware sentence encoding; collision-resistant similarity |
| End of Stage 3 | Non-trivial scores on real ARC tasks; calibrated correctness on a QA set; self-tuned coupling constants |

## Final Word

The audit's verdict, compressed: **Phiano has built the hard, original half — a
working online-learning substrate with a genuine novel mechanism (learned phase lags)
and verified speed/memory numbers — and wrapped it in superstructure that currently
runs ahead of its evidence.** The path to power is not more metaphor; it is
measurement (Stage 0), wiring what already exists (Stage 1), the dimensionality
increase its own cousins proved viable (Stage 2), and closing the feedback loop
(Stage 3). Every task above has a file:line landing zone in this repository. That —
more than any claim in the docs — is the strongest evidence that this architecture
can go where its author wants it to go.
