# 82 · What is missing, and what is not being used

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. A codebase is the same:
what matters is which parts relate to which, and a part nothing relates to is
not a part of the system, whatever the file count says.

This is that inventory, taken mechanically rather than by reading: 541 source
files, 29,670 lines, 50 binaries. Two passes — the compiler with every
`#[allow(dead_code)]` stripped, and a reference count for every public item
against the whole crate, its examples and its tests. docs/79 was the
hand-written version a week ago; this supersedes it and records what the
fixes since then changed.

## 1. Missing inputs — the silent kind

Eleven paths the code names do not exist. Seven are outputs it will create on
first run (`api_cache`, `benchmark_history`, `corrections`, `experiment`,
`forgetting`, `relations.json`, `definitions.txt`). **Four are inputs, and
their absence is silent:**

| missing | what reads it | what actually happens |
|---|---|---|
| `data/chunks/` | `ChunkStore::load_all` | returns an empty vector, no warning: every definition lookup in chat, grounding, `conception` and `bin/defstats` finds nothing |
| `data/dictionary_corpus.txt` | 7 bins' default argument | those bins `exit(1)` — this one is at least loud |
| `data/stop_words.txt` | `PhiConfig::stop_words` | returns an empty set, so `is_stop_word` is **false for every word**; the content-word filter in every fingerprint ever computed did nothing |
| `data/searle_markers.json` | `SearleMarkers::load` | one stderr line, then empty marker lists — so `SpeechActAgent` classifies **every** input as `Assertive`, and the `speech_act` and `direction_of_fit` the API returns are constants |

Ten binaries cannot run today for want of the first two: `latency`,
`bootstrap_facet`, `roles`, `topdown`, `types`, `context`, `conception`,
`relations`, `defstats`, `forgetting`, `ingest`. `latency` is the one that
matters — learn / recall / correct / unlearn at p50 and p99 is the project's
central claim and there is no way to run its benchmark on this tree.

Everything else in `data/` is present, including the two new persona files
(`personas.chroma`, `personas.json`, docs/81).

## 2. Code nothing references

112 public items have **no reference anywhere** — not in the library, not in a
binary, not in a test. A further 162 are public but used only inside their own
module: not rot, but an API surface wider than the system.

| module | dead items | what they are |
|---|---|---|
| `config` | 33 | constants nothing reads (below) |
| `metrics` | 32 | `full_pipeline`, `suite_from_disk`, `shift_score`, `is_ood`, `phase_jitter`, `prune_low_amplitude`, `examples_to_competence`, `language_model` — a measurement API built wider than the harnesses that use it |
| `phical` | 16 | the whole of `phical::evaluate` (`evaluate_word/phrase/sentence/story/concept`, `layer_relation`, `story_complexity`) and half of `operators`/`manifold` |
| `persona` | 13 | mostly `PersonalityMapper` and comparison helpers; `PersonaStore::lexicon_of` is mine, written this week and not yet used |
| `phinum` | 9 | `SearleClassifier::classify_full`, `SpiderNet`, the 16/32/64 syntax variations — a second Searle stack, parallel to `src/cognitive`, wired to nothing |
| `lifelong` | 9 | `check_drift`, `check_regression`, `dispersion_trend` — the drift monitor is written and never called |
| `phiton` | 7 | wavelength/colour conversions |
| `reasoning` | 6 | `Counterfactual::what_if`, `sorting::by_amplitude/by_sector`, `Analogy::light_centric` |
| `compose`, `generate`, `phasor`, `data`, … | 24 | `decode_beam` (a full beam search, never called), `Memo::consolidate`, `words_in_sector`, `harmonic_spectrum`, `to_light_quantum`, `eval_with_memory`, `train_corpus_ranking`, `trigram_probability`, `normalize_wave`, the `data::splits` iterators |

Seven modules carry a file-wide `#![allow(dead_code)]` — `wiki_bulk`,
`reasoning`, `metrics`, `curriculum`, `instruction`, `data`, `synthesis` —
which is what keeps the compiler quiet about most of the above.

### The 33 constants

`config` is where this is most dangerous, because a constant reads as a knob.
Nothing reads any of: `ROTOR_DIMS`, `ROTOR_DIM_WEIGHT` (they describe a "rotor
manifold" that does not exist), `META_LR_MULT_MIN/MAX`, `META_MIX_MIN/MAX` (a
homeostatic self-tuner that does not exist), `VERIFY_REINFORCE_PULL`,
`VERIFY_REPULSE`, `VERIFY_SUPPORT_BIGRAM_P`, `VERIFY_SUPPORT_RESONANCE` (an
execution verifier that does not exist), `RECALL_BOOST`, `RECALL_PHASE_BIAS`,
`RECALL_TOP_K`, `NEGATIVE_SAMPLES_K`, `NEGATIVE_REPULSION`, `ANTI_PHASE_PULSE`,
`CORRECTION_JOURNAL_MAX`, `STUDY_BUDGET_DEFAULT`, `SYNTAX_LAG_DEFAULT`,
`PASSAGE_*`, `PHI_SQUARED`.

Six more went dead **this week, by my hand**: rewriting `Impersonator` onto the
generator (docs/80) orphaned `IMPERSONATE_QUALITY_OVERALL`,
`IMPERSONATE_QUALITY_DIVERSITY`, `IMPERSONATE_QUALITY_COVERAGE`,
`IMPERSONATE_QUALITY_LENGTH`, `IMPERSONATE_ADJACENT_FIT_FACTOR` and
`IMPERSONATE_CONVERGENCE_DELTA`. That is the normal way this list grows: not
from carelessness, from replacing something.

## 3. Capability still missing

What is absent as *capability*, after the fixes of docs/78–81, ranked by what
it costs a demo and a large-corpus run.

1. **The server and REPL chat are still templates.** `/api/chat`'s general
   answer comes from `WordSelector::synthesize` — "{} concerns {} and {}." —
   with the manifold supplying only the nouns; an *Explanation* with a
   Wikipedia hit returns the extract verbatim under "Semantic Grounding".
   `compose` and `impersonate` were fixed (docs/80); chat was not.
2. **No punctuation, no case.** `Tokenizer::tokenize` deletes both, and
   `split_sentences` splits on every `.`, so "Mr. Darcy" is two sentences.
   A persona cannot be told by its commas, and never will be until this
   changes.
3. **Five of six objectives write only the source side.** Only
   `Trainer::couple_relations` moves the target side; `train_predictive`,
   `train_sentence`, `Descent::settle`, `Coherence::align`,
   `Conception::compose_anchored` and `phinet::joint::write_back` do not — so
   every pass that improves the manifold desynchronises the directed reading
   (`on`, `relational_score`, `calls_for`) that generation and the dialogue
   depend on.
4. **No shared-lexicon operator.** `data/austen_operator.fno` is Austen's, so
   the three-persona dialogue decodes from the trigram tables and inherits
   their idiom; docs/78 measured decoding from the function space at 10%
   shared 4-grams against the sampler's 26%.
5. **No discriminative attribution.** The sector reader weights sectors by how
   often a writer uses them, not by how well they separate writers, and reads
   44% where a per-writer Kneser-Ney panel reads 72% (docs/81).
6. **Retrieval is O(V) per token**, and `SectorIndex` — written to fix exactly
   that — is called only from its own test.
7. **Large corpora**: no streaming reader (every corpus is loaded whole), no
   sampled softmax (an epoch costs table-size per example), and the engine is
   allocation-bound at these tensor sizes (phinet benchmarks).
8. **`joint::write_back` corrupts evidence counts**, exponentiating learned
   magnitudes into a `u32` that can saturate silently.
9. **The shipped grounding path disables `conception`** — `bind = false`,
   `strong = weak`, `graph = None`, `hold = None` — four implemented, tested
   mechanisms bypassed.

## 4. What I would do with this list

Delete nothing yet: a dead constant is cheap and a deleted intention is not.
But three things on it are worth acting on before the demo, in this order: the
chat path (it is the only remaining place where the system says something it
did not compose), `data/stop_words.txt` and `data/searle_markers.json` (two
small files that silently disable a filter and an entire classification layer),
and the shared-lexicon operator (the dialogue's idiom problem, already measured
elsewhere). The rest is a list to shrink deliberately, a module at a time,
with the reference count as the test.
