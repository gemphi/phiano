# 08 — Generation and Composition: An Honest Quality Assessment

> Files examined: [`src/generate.rs`](../src/generate.rs), [`src/phase_flow.rs`](../src/phase_flow.rs),
> [`src/compose/`](../src/compose) (flow, better, worse, tune),
> [`src/server/routes_stream.rs`](../src/server/routes_stream.rs),
> [`tasks/`](../tasks) (recorded demos).

---

## 1. The Generator: n-grams First, Phases as Steering

[`Generator::decode`](../src/generate.rs) — the actual production loop:

```text
target_phase = current_phase + momentum(0.15) + jitter(temp) + 0.45·sin(flow_phase − current)
candidate = trigram(prev,last)  →  bigram(last)  →  torus ray-cast fallback
score     = ln(1+count)·(0.35 + 0.25·phase_align + 0.40·flow_resonance)·content_weight
emit      → phase-kick: current += 0.35·sin(φ_word − current + β_prev,word)
             momentum ← 0.85·momentum + 0.15·|Δ|
stop      → 20 tokens, 4 consecutive function words, or empty candidates
```

**What this is:** a **trigram language model with a phase-steered re-ranker**. The
n-gram tables supply almost all syntactic fluency (that is why they are 92 MB of the
state, file 05 §3); the phase machinery contributes (a) candidate steering when n-grams
run dry, (b) the learned-β phase kick that nudges the walk in syntactically observed
directions (the novel part, file 03 §4), (c) repetition suppression via the recent-word
set, (d) a 44-entry hard-coded `boilerplate()` blacklist to keep dictionary-ingestion
jargon ("genus", "viz", "thereof") out of speech.

**What this is not:** the "continuous attractor decoding" of docs/45 §5. Ray-cast
fallback triggers only when both trigram and bigram tables miss — i.e., precisely
where the model is least trained. At ~150k vocabulary the ray-cast pool is 192
candidates, filtered to high-amplitude "speakable" words.

## 2. Observed Output Quality

From recorded demos ([`tasks/chat_demo.txt`](../tasks/chat_demo.txt),
[`tasks/story_demo.txt`](../tasks/story_demo.txt), [`tasks/showcase.txt`](../tasks/showcase.txt)):

- Single-sentence-scale responses after in-domain training; coherent n-gram echoes of
  trained corpora ("ownership borrowing lifetime" domain demos read like Rust-Book
  phrases recombined).
- Responses are **short by construction** (20-token cap), capitalized and punctuated by
  [`format_output`](../src/generate.rs) — the "period" is post-processing, not generation.
- Out-of-domain prompts degrade to word-salad or the boilerplate blacklist triggers.

**Comparative anchor:** this is below a 1990s trigram NLG system with a good planner,
because there is no planner — and far below any modern LM. Its one genuine edge over a
pure trigramer is the phase kick + flow bias steering, which is **untested against the
trigram-only ablation** (file 16, task 4 — the single most important experiment this
project has not run).

## 3. PhaseFlow: The Live Trajectory

[`PhaseFlow`](../src/phase_flow.rs) tracks collective phase, order parameter, novelty,
and momentum across decoding steps — powering the `/api/phase_flow` visualization and
SSE telemetry. As **instrumentation** it is excellent: you can *watch* the decode walk
the circle. As generation mechanism it is a drift prior, not a plan.

## 4. RiverFlow Composition: Templates + Tournament

[`compose/flow/`](../src/compose/flow): ray-cast a resonant word pool from the prompt →
map words to 64 sectors → `build_path` walks source sector → **antipodal (tension)
sector** → back — a genuine dramatic-arc geometry (thesis → antithesis → return).
[`Composer::compose`](../src/compose/flow/compose.rs) pours sector word-banks into
**fixed English templates** ("In the beginning, the narrative opens with X and Y…"),
then greedily reorders with bigram probability.

Around it, a serious optimization loop: [`better.rs`](../src/compose/better.rs) scores
all 64 sector variations (coherence/novelty/resonance/diversity/coverage/alignment,
weights in constants.rs:117–133), [`worse.rs`](../src/compose/worse.rs) prunes and
**trains the facet** on kept texts, [`tune.rs`](../src/compose/tune.rs) iterates
propose→evaluate→discard→train to convergence (Δ < 0.001).

**Verdict:** the tournament loop (generate → score → prune → *learn from survivors*) is
a real evolutionary-search training signal — arguably the system's second learning
mechanism and an underexploited one. The output text, however, is template prose with
thematic word choice; readable, not publishable.

## 5. Streaming: Theater, Not Incrementalism

[`/api/generate/stream`](../src/server/routes_stream.rs) pre-decodes the full token
list, then replays it over SSE at 40 ms/token. True incremental decoding is absent —
the decode loop is not async-suspendable. Fine for demos; a real streaming refactor is
file 16, task 8.

## 6. What the Spider-Net Could Add (Untested)

The structural transition memory (file 06 §4) records *which sentence types follow
which* — exactly the discourse prior a 20-token generator needs to plan multi-sentence
arc (assertive → directive → commissive sequences). **No generation path currently
consults it.** Wiring `spider_net.type_links` as a sentence-type planner is the most
promising un-built generation feature in the repository.

## 7. Scorecard

| Aspect | Grade | Note |
|---|---|---|
| Mechanism coherence | B | n-gram + phase steering cleanly layered, all constants explicit |
| Novelty of steering | B+ | Learned β_ij kick + tension-arc geometry are original touches |
| Fluency | C− | Trigram-bounded; 20-token cap; blacklist-dependent |
| Long-form ability | D | Template composer only |
| Streaming | D+ | Pre-generated replay |
| Learnable generation signal | B− | Compose tournament trains the facet — underused |
| Ablation evidence | **F** | No trigram-only baseline anywhere — the phase contribution is unquantified |

**Bottom line:** generation is currently Phiano's weakest subsystem precisely because
it leans on the strongest classical crutch (n-grams) without yet using the system's own
distinctive assets (spider-net discourse prior, memo recall, learned attention). The
architecture has places to grow into; the current occupant is a trigram model with a
compass.
