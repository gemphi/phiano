# 79 · Demo brief: what is done, what is not, and what large-data learning takes

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. A persona is which
conditions a writer puts words under, aggregated; a corpus is every condition a
language has been observed in. This brief is what a reader of the entire crate
finds — two independent read-throughs of all 524 source files, plus one new
measurement — written for a demo where every number must survive a question.

## 1. What can be shown, honestly, today

Every path below runs end to end on the data in `data/`, offline.

| show | what it is | the number |
|---|---|---|
| `bin/austen` | three novels → two-sided lexicon (19,649 words), three personas, attribution on held-out paragraphs, operator, learned scorer, compositions judged by a Kneser-Ney model that never saw them | attribution 64–70% at chance 33%; writes `docs/papers/15` |
| `bin/operator_sweep` | four training arms of the next-word operator, resumable | wide+deep softmax **30.7× chance** (hinge 17.4×), hit@10 0.348 (docs/78) |
| `bin/decode_eval` | trigram sampler vs decoding from the function space | operator text KN perplexity 97 → 67; the sampler repeats itself across prompts (26% shared 4-grams), the operator does not (10%) |
| `bin/persona_eval` (new, §3) | one shared lexicon, three personas, the generator conditioned on each | **53% read back as the persona asked for**, chance 33% |
| `bin/liskov` | substitution and the Liskov reading on Austen | KN 238 vs 254 (docs/75) |
| phinet | the engine: 29 tests, **62 PyTorch parity checks all pass** (max Δ 1e-15), the Colab notebook, the 3-D docs | `phinet/torch/`, PhiDoc `docs/phinet` |
| `bin/evaluate`, `experiment`, `learn`, `readout`, `order`, `pair_key`, `sentence`, `hexagram_eval`, `phinet*`, `relational`, `dimensions`, `frames`, `senses`, `marked`, `markers`, `promoted`, `bpe_check` | the measurement line on the Rust Book, WikiText-2 prose and Phi-4's tokenizer | most print a null or negative result and say so — that is the project's habit, and its credibility |

Do **not** demo: the REPL/server `chat`, `/api/chat`, `/api/reason*` — their
sentences are templates (§2.1). `compose` and `impersonate` now generate
(docs/80) and can be shown with that caveat. `bin/latency` — the
benchmark for the project's central claim (learn / recall / correct / unlearn
at p50 and p99) — cannot run: its inputs are absent (§2.3).

## 2. What is not done

Two read-throughs of every file, independently, agree on the same list. There
is no `todo!`, `unimplemented!`, `TODO` or `FIXME` anywhere in `src/`; the
incompleteness is structural, not marked. Ranked by what matters for learning
from large corpora and composing as a chosen persona.

### 2.1 Text that is not generated

1. **`Composer::compose` is a four-sentence template** (`src/compose/flow/compose.rs:30-82`): "In the beginning, the narrative opens with X and Y." / "As tension shifts across the manifold, the current evokes X." … The model fills the nouns; the syntax, the length and the frames are constant. Everything called *compose* or *impersonate* — `Impersonator`, `CompositionTuner`, the `better`/`worse` tournament, the REPL `compose`, `/api/compose` — is real machinery ranking 64 fillings of these four frames. **Fixed in the same round (docs/80):** `Composer::compose` now generates one goal-driven segment per sector through `Generator`, and `Impersonator` composes with the persona-conditioned generator — `bin/impersonate_eval` 39% → 56% read back as asked, KN perplexity 4,478 → 24.
2. **`/api/chat`'s general answer is a Mad-Lib** (`src/cognitive/word_selection.rs:124-178`: "{} concerns {} and {}."); an *Explanation* with a Wikipedia hit returns the extract verbatim under "Semantic Grounding / Attractor Resonance" (`src/server/chat_intent/chat_intent_grounded.rs:11-23`); the fixed markdown around the real generator's output makes unconditional claims ("Zero catastrophic forgetting"). The REPL `chat` path (`src/instruction`) is the one chat path that actually generates.
3. **The 16-agent Searle layer is inert**: `data/searle_markers.json` is absent, so every marker list is empty and every input classifies as `Assertive` (`src/cognitive/markers.rs:43-57`, `speech_acts.rs:20-39`). Several agent confidences are literals. `src/phinum` (~1,500 lines) is a second Searle stack with no callers; `src/reasoning` answers are templates and half of it is unreachable; `src/synthesis`'s `Map`/`Reduce`/`Compose` are no-ops.

### 2.2 The representation

4. **`Tokenizer::tokenize` deletes punctuation, case, apostrophes and hyphens** (`src/tokenizer.rs:13-23`); `split_sentences` splits on every `.` ("Mr. Darcy"). Style *is* punctuation and clause rhythm; the manifold cannot represent it, so a persona lives at the word-bag level whatever else improves.
5. **Only one of six objectives writes the target side.** `Trainer::couple_relations` is the sole writer of `tgt`; `train_predictive`, `train_sentence`, `Descent::settle` (`src/topdown/descent_votes.rs`), `Coherence::align`, `Conception::compose_anchored` and `phinet::joint::write_back` move source angles only — every pass that improves the manifold desynchronises the directed reading generation depends on.
6. **The shipped grounding path disables most of `conception`** (`src/cognitive/grounding/definition_grounder.rs:68-79`: `bind = false`, `strong = weak`, `graph = None`, `hold = None`): four documented, implemented, tested mechanisms bypassed in production.
7. **`joint::write_back` overwrites evidence counts with exponentiated learned magnitudes** (`src/phinet/joint.rs:76-83`) — after one write-back "evidence" is no longer an observation count, and `as u32` can saturate silently.
8. **Retrieval is O(V) per generated token**; the `SectorIndex` that fixes it is written and unused (`src/wave/wave_words_in_sector.rs`); `pool_ceiling` sorts the whole count vector per ray cast. `HierarchicalPhaseField::descend`/`candidates` claim a coarse-to-fine search they do not perform.

### 2.3 Inputs that are missing, so paths fail silently

9. Absent from `data/`: `chunks/`, `dictionary_corpus.txt`, `stop_words.txt`, `searle_markers.json`, `curriculum.json`, `arc_tasks.json`, `web/dist/`. Consequences: 12 bins cannot run (`latency`, `roles`, `topdown`, `types`, `context`, `bands`, `conception`, `relations`, `defstats`, `forgetting`, `ingest`, `bootstrap_facet`); every definition lookup is empty; **stop-word filtering is off in every fingerprint ever computed** (`PhiConfig::is_stop_word` is always false); the server's UI is a 404. `ChunkStore::load_all` returns an empty vector with no warning — the silent origin of most of this.

### 2.4 Bookkeeping that misleads

10. Beam search is implemented (`src/generate/beam.rs`) and never called; the greedy `decode` carries its doc comment. `Impersonator` compares two incompatible sector coordinates (`impersonate/mod.rs:101` legacy vs centred), so its `fit` number means nothing. 17 config constants have no reader (`ROTOR_DIMS`, `META_*`, `VERIFY_*`, `RECALL_*`, …), some describing subsystems that do not exist. `phase_lags` persists derived data as if observed. `permute_fillers` is a shuffle, not the derangement its doc claims — every `relation_type` null rests on it. `Memo::consolidate` is never called. `Generator::new` loads Rust-Book weights whatever corpus is loaded. `~220` lines of `phical::topology`/`evaluate`, `facet/spectral` colour methods, `TorusPhasor`, `Role::Context`, `RelationTypes::harmonic_mode` have no callers, kept quiet by `#[allow(dead_code)]`.

## 3. Can it compose as one of the personas? Measured today

Until this session the generator had no persona knob at all; "her voice" in
`bin/austen` was a *separate lexicon per author*. `Generator::with_persona`
(`src/generate/persona.rs`) now conditions **one shared lexicon** on a
persona in two ways: a sector prior (`weight · ln p(sector)/uniform` from the
fingerprint histogram, clipped ±2) and the persona's own bigram evidence
(`ngram_weight · ln(1 + count)` from its example sentences — the conditions
*this* writer put pairs under). `bin/persona_eval`: 24 prompts (8 held-out
sentences from each author, first three words), 30 tokens, trigram decoding on
the shared `austen.chroma`, judged by `World::match_text` — the attribution
reader — on the composed text alone.

| conditioning (sector, bigram) | read as asked | Austen | Chesterton | Carroll |
|---|---|---|---|---|
| none | 33% | 58% | 25% | 17% |
| sector prior 1 | 43% | 79% | 17% | 33% |
| **persona bigrams 1** | **53%** | 92% | 42% | 25% |
| both, 1 and 1 | 44% | 75% | 17% | 42% |
| persona bigrams 2 | 49% | 88% | 25% | 33% |

Chance is 33%. Unconditioned text reads as Austen 14 of 24 times whoever was
asked for — the judge and the shared tables both lean toward the author with
the most sentences. The persona's own pair evidence moves the verdict (53%,
Austen 92% with KN perplexity 24 under her own trigram judge); the sector
prior alone barely does (43%), because the three fingerprints overlap
(Austen–Chesterton similarity 0.98, docs/76). Samples for the same prompt
*chapter 11 the* under bigram conditioning: Chesterton — "little old man with
the first time in his own back garden might hear some new cannibal islands
said brown"; Carroll — "little golden key and hurried her into the garden wall
just as well go back to the same thing with you said the queen"; Austen —
"very first subject on which the sweet sounds of her mother was sincere it must
be more advisable". So: **yes, above chance, from one lexicon — and it is the
persona's pair conditions that do it, not its sector histogram.** What limits
it is §2.2 (a bag-of-words manifold and a judge that shares its bias), not the
conditioning.

## 4. Learning on large data sets: the parameters and what they cost

What is learned, and the knobs (`src/config`, `phinet::FnoConfig`):

| component | parameters | knobs |
|---|---|---|
| lexicon | one `SpectralPhasor` per word: 64 channels × (source angle, target angle, 32-bit evidence) = 64 × 12 B + bigram/trigram tables; 19,649 Austen words ≈ 15 MB on disk (v6) | `PHASE_CHANNELS` 64, `CHANNELS_PER_UPDATE` 16, `MARKER_CHANNELS_MAX` 24 (relation channels 8–31 chosen by the corpus), `LEARNING_RATE` 0.05, `AMPLITUDE_MAX` 2, passes (3), negative samples, `Sides::{Source,Target,Both,Directed}` |
| operator (phinet FNO) | 50,016 complex params at width 16 / 3 layers / 16 modes (8,360 at 8/2/12) | `FnoConfig{space 64, width, layers, modes, margin, lr}`, objective (hinge / softmax β), `Adam` or `AdamW + Schedule::Cosine`, batch 64, window 6, pool floor 20 |
| scorer | 8 log-linear weights (`learn::GenWeights`) | L2, early stopping, per-corpus file |
| persona | 64-bin histogram + its bigram table | `weight`, `ngram_weight` (§3) |
| phasor ranges | `Resolution {P16,P32,P64,P128}` × `Quantum {Q16,Q32,Q64}` (12 combinations, docs/phinet guide 09) | quantisation loss 0.0064/0.0016/0.0004 |

What it costs on this two-core machine, measured:

| step | measured | scales as |
|---|---|---|
| lexicon, three passes, 17k words, 360k words of text (`bin/austen` whole run incl. personas/compositions) | ≈ 400 s | linear in tokens × passes × `CHANNELS_PER_UPDATE` |
| operator, hinge epoch, 204k examples | 47 s | linear in examples |
| operator, softmax epoch, 204k examples, table 1,228 | 14 min (small) / 20 min (wide) | examples × table size × width |
| `cosine_table` 256 × 6,000 × 64 | 69 ms | rows × table × space |
| one 1 MB tensor op | 2–5 ms — **allocation/bandwidth bound** (phinet benchmarks page) | — |

Extrapolation, honestly labelled as such: WikiText-103 (103M tokens) is 290×
Austen's text. The lexicon pass is linear and would take about a day of
CPU at this rate; the softmax operator epoch is table-sized, and at a
100k-word pool the per-example cost grows 80× — that path needs (a) sampled
softmax or hinge with many decoys, (b) the engine's allocation fix (a buffer
pool or an f32 path — the profile shows the FFT is not the cost), and (c)
`SectorIndex` in retrieval. Nothing in the maths stops it; the engine is
written for correctness against PyTorch first (62/62 parity) and speed second.
`bin/operator_sweep` already checkpoints every quarter-epoch with the
optimiser state, so a large run resumes.

What "learning big time" needs that is not written: a streaming corpus reader
(everything loads whole files into memory); sampled-softmax training; the
target-side write in every objective (§2.2 item 5); a tokenizer that keeps
punctuation and case; and one lexicon format that carries several personas as
conditions rather than one lexicon per persona.

## 5. What was done this round

`Generator::with_persona` / `with_persona_examples`, `PersonaPrior`
(`src/generate/persona.rs`), `bin/persona_eval` (main + `world.rs`, personas
built exactly as `bin/austen` builds them), `Fingerprint::relational_sector`
made public; 220 lib tests pass; every file still under 200 lines.
