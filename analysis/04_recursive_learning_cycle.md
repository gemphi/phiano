# 04 — The Recursive Learning Cycle: Envision → Apply → Eval → Iterate → Scale

> Files examined: [`src/model.rs`](../src/model.rs), [`src/command/`](../src/command),
> [`src/envision.rs`](../src/envision.rs), [`src/eval.rs`](../src/eval.rs),
> [`docs/09_learning_cycle.md`](../docs/09_learning_cycle.md), [`docs/17_envision.md`](../docs/17_envision.md).

---

## 1. The Cycle as Implemented

Every user input passes through [`Model::iterate`](../src/model.rs) — one full loop of the
advertised cycle:

```text
User line
   │
   ├─ 1. DISPATCH   Dispatcher::dispatch(line, ctx)         (src/command/mod.rs)
   │      └─ unknown command ⇒ Learn::default()
   │             ├─ APPLY    Trainer::train_online(facet, line)
   │             ├─ EVAL     Evaluator::eval(facet, line)
   │             └─ ENVISION Envision::detect_gaps(facet, line)
   │
   ├─ 2. RECORD     Wave::text(line) → Memo::record(wave, line)   (16-layer log)
   │
   └─ 3. ENVISION   second gap-detection pass (Model::envision)
          │
          ▼
   on exit: SCALE    Storage::save(facet) + Memo::save (Model::scale)
```

The cycle is genuinely recursive in the operational sense: the *output* of one turn
(new vocabulary, new phases) changes the *behavior* of the next, and gaps detected in
turn N become learnable material in turn N+1.

## 2. Phase-by-Phase Audit

### 2.1 ENVISION — gap detection ([`src/envision.rs`](../src/envision.rs))
Scans input for words missing from the lexicon and suggests known words nearest in
phase as scaffolding; chat-mode additionally fires [`learn_definition_chain`](../src/trainer/mod.rs)
so unknown words are *self-taught from the dictionary chunk store* before answering
([routes_chat.rs:49](../src/server/routes_chat.rs)). Optional Wikipedia fetch grounds
proper nouns with live text (5 s timeout, [routes_chat.rs:123–160](../src/server/routes_chat.rs)).

**Assessment: working and genuinely useful.** "I don't know this word → I will go read
its definition → now I know it" is more self-directed learning than most deployed
chatbots perform.

### 2.2 APPLY — training ([file 03](03_learning_engine.md))
Single-sentence online training, sub-millisecond. Also reachable explicitly via
`learn`, `learn_multi` API endpoints and the `train_rust_book` binaries.

### 2.3 EVAL — self-scoring ([`src/eval.rs`](../src/eval.rs))
Computes three numbers per input:
- **resonance** = fraction of tokens known,
- **coherence** = Kuramoto order parameter `r = ‖ΣZ‖ / N_known` (phase alignment),
- **novelty** = angular distance of the input wave from the global facet centroid,
  squashed through `1 − exp(−x)`,

then maps them to a qualitative [`Verdict`](../src/eval.rs) (CoherentNovel,
DissonantNovel, Noise, …). This is the system's *epistemic self-monitor* — cheap,
continuous, and explainable.

**Assessment: honest design, self-referential metric.** Resonance is objective.
Coherence measures *internal consistency*, and since training *is* phase alignment,
training data scores high almost by construction (file 14 §3 quantifies). Novelty is
legitimate but coarse (one scalar for the whole input).

### 2.4 ITERATE — the turn loop
Memory recording + context-wave update; multi-turn state carried by the O(1)
[`ContextWaveBuffer`](../src/generate.rs).

### 2.5 SCALE — persistence ([`Model::scale`](../src/model.rs))
bincode serialization of the facet to `data/manifold.chroma` and the memo to
`data/memory.chroma`. Current on-disk size: **92 MB** (mostly n-gram tables — file 05 §3).

## 3. What the Cycle Achieves That Static Models Cannot

1. **Closing the loop with the user.** `correct_mistake` (file 03 §2) means a correction
   in turn 5 measurably changes responses in turn 50 — persistent behavioral change
   from a single natural-language interaction, with no fine-tuning run. This is the
   capability LLM vendors approximate with RAG patches; here it is native.
2. **Curriculum self-assembly.** [`src/curriculum.rs`](../src/curriculum.rs) +
   the chunk store let the system sequence its own reading material
   (dictionary → Rust Book → reference-model docs).
3. **Explainable self-state.** Every turn ends with numbers (coherence/novelty/
   resonance) the user can inspect — a primitive but real introspection channel
   (the `eval`, `stats`, `layers` commands and `/api/eval`, `/api/stats` endpoints).

## 4. Where the Cycle Overpromises

| Claimed phase | Reality in code |
|---|---|
| "Iterate on gaps" (docs/09) | Gap *detection* exists; there is no planner that schedules gap-remediation, no prioritization among gaps, no verification that a gap closed |
| "Envision what it doesn't know" | Detects unknown *tokens*; it cannot detect unknown *concepts* expressible in known words (e.g., it "knows" every word in an unfamiliar formula) |
| "5-phase recursive learning cycle" per input | True for the chat/learn path; generation-only paths (compose, reason) skip EVAL/ENVISION |
| 16-layer memo feeding back into behavior | **The memo is write-only today** — no code path recalls entries into generation (file 05 §4). The recursive loop's long-term half is aspirational |

## 5. Scorecard

| Aspect | Rating | Note |
|---|---|---|
| Cycle exists and runs per input | **Yes** | Verified in [`Model::iterate`](../src/model.rs) |
| Learning per cycle is real | **Yes** | Phases/amplitudes/lags change persistently |
| Self-teaching from dictionaries | **Yes** | Definition chains, working |
| Self-evaluation | **Yes, but self-referential** | Coherence tautology (file 14) |
| Gap remediation planning | **No** | Detection only |
| Long-term episodic recall | **No** | Memo never read back |

**Bottom line:** the cycle is the right architecture and the short-term loop
(detect → learn → score → persist) is fully working. The long-term loop
(plan gap remediation, recall episodes, verify closure) is scaffolding without
machinery — and is precisely where the highest-leverage roadmap items live
(file 16, tasks 6–7).
