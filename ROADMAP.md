# Phiano build order

Agent-facing work plan. Every task has an acceptance criterion that is a number,
because this project's results have only been trustworthy where they were
falsifiable. Written against `HEAD 8097133`, 137 tests passing.

Full results: `docs/how/RESULTS_4a-4g.md`.

---

## The strategic read

Six independent measurements now say the phase manifold does not beat a unigram
at next-word prediction: two training objectives × three context constructions ×
readout on/off, all γ\* = 0. That question is settled.

The same measurements say Phiano builds real relational structure from
definitions (analogy@1 0.62% → 10.49%), retains 98.1% after training on a new
domain, and edits a single fact in milliseconds.

**A system with those properties is an editable knowledge substrate, not a weak
language model, and it has been benchmarked against the wrong opponent.** The
recommendation is not to abandon the LM path but to stop treating perplexity as
the scoreboard: pair Phiano with an LM rather than replacing one. The LM
generates; Phiano holds what is known and can be corrected. Workstream C is
where that reframing turns into evidence.

---

## What holds, what does not, what is untested

| Mechanism | Result | Verdict |
|---|---|---|
| Multi-channel definition composition | analogy@1 0.62 → 10.49% | **Holds** |
| Mutual reinforcement between definers | MRR 0.0107 → 0.1177 | **Holds** |
| Retrofitting anchor (α) | dispersion 0.37 → 0.66 | **Holds as a dial** |
| Non-linear readout (HOW 16) | γ\* stays 0 at 99.5% coverage | **Refuted** |
| Positional binding inside definitions | pair/random −31.5pp | **Refuted** |
| dict2vec strong/weak split | 0.0935 vs 0.1177 control | **Untested** |
| Grounding-kernel scheduling | MRR 0.0126, ≈ baseline | **Untested** |
| Controlled negative sampling | MRR ±0.0000 | **Untested** |

The last three share one cause: `clean_definition` strips brackets and
apparatus but not Webster's quoted usage examples, so every definer set is
inflated. The graph comes out 47.5:1 weak:strong against dict2vec's ~9:1, and
the kernel at 49.6% of entries against the literature's ~10%. **One fix unblocks
three mechanisms** — hence task A1.

---

## A — Make the measurements trustworthy  *(blocks B, C, E)*

Until a number can be checked and its error bar is smaller than the effect being
claimed, no other workstream can report a result.

- **A1 — Strip quoted usage from dictionary entries.** Extend `clean_definition`
  to drop quotation sentences and citation attributions.
  *Accept:* weak:strong ≤ 15:1 · kernel ≤ 20% of entries · golden-file test over
  20 hand-checked entries.

- **A2 — Grow the relation benchmark past significance.** 23 usable pairs cannot
  support a 10-point claim. Add plural, comparative, verb tense, capital–country,
  hypernym, antonym families.
  *Accept:* ≥ 300 usable pairs across ≥ 8 families · every reported effect
  exceeds its own 95% interval · per-family breakdown printed.

- **A3 — Seeded multi-run reporting.** Training is deterministic as of `1ed9490`,
  which makes a seed meaningful.
  *Accept:* every experiment binary takes `--seed` and `--runs` · all result
  tables carry ± · CI computed, not asserted.

- **A4 — Publish the latency numbers.** The central claim is a latency claim with
  no benchmark. Time three paths: learn one word, unlearn one fact, answer one
  query. Compare against LoRA fine-tuning on the same single fact.
  *Accept:* `bin/latency` exists · p50 and p99 for all three · a stated baseline.

---

## B — Move the wins into the product  *(after A1, A2)*

Composition is the largest confirmed effect in the project and runs only in an
experiment binary. `conception` and `nonlinear` are reachable from `src/bin/*`
only — not from the `phiano` binary, not from the server.

- **B1 — Replace the grounder with composition at startup.** `model.rs` calls
  `DefinitionGrounder::ground_phases` on boot; put
  `Conception::compose_anchored` behind a flag, run both one release, retire the
  single-channel path.
  *Accept:* relation metrics improve in the shipped binary · dispersion floor
  ≥ 0.40 as a hard guard · boot time regression < 2×.

- **B2 — Choose the anchor operating point.** α is monotonic with no dominant
  setting: α = 0.25 best MRR, α = 1.00 best pair/random and a healthier manifold.
  A product decision, not a tuning one. Recommend α ≈ 0.5 with the dispersion
  floor binding.
  *Accept:* α fixed in config with the trade documented at the constant.

- **B3 — Make the recurrent context the default.** Measured best of three in both
  regimes (173.08 vs 182.69 under ranking). The 2-word context barely encodes
  order at all (swap-cosine 0.62) and is still the default.
  *Accept:* `ContextKind::Recurrent` default in scoring and generation · no
  perplexity regression · `bin/order` rerun and recorded.

- **B4 — Close the learn-anything loop online.** The pieces exist and are
  unconnected: `envision` detects a gap, `ApiSource` fetches a definition,
  `Conception` composes it. Wire gap → fetch → compose → persist.
  *Accept:* an unknown word is queryable within one turn of first mention ·
  learned words survive restart · path timed in A4.

---

## C — Benchmark the thing it is actually good at  *(parallel with B)*

Perplexity has been asked and answered six times. These three have never been
asked once.

- **C1 — Retrieval, not perplexity.** Given a query, return relevant known facts
  from the manifold. Baseline against BM25 and a small sentence-embedding model.
  Sector lookup is sublinear where a dense index is not.
  *Accept:* recall@10 and MRR against both baselines · query latency reported
  beside quality · a documented loss condition.

- **C2 — Edit locality.** `correction.rs` already applies and replays single-fact
  corrections. Measure what nothing else does cheaply: change one fact, then
  quantify how much of the rest moved.
  *Accept:* % of unrelated vocabulary whose phase moves > ε after one edit · edit
  applied and verified in < 100 ms · compared against retraining.

- **C3 — Harden and publish the forgetting result.** 98.1% retention is the
  project's strongest number and appears nowhere outside an internal doc.
  *Accept:* 3-domain sequential result with intervals under A3 · a named external
  comparison point · a standalone write-up.

---

## D — Retire what is not carrying weight  *(any time)*

An audit found five subsystems that compile but are reachable from nothing. Each
gets one of two outcomes and no third: wired with a test that fails when it
breaks, or deleted.

- **D1 — Resolve the duplicate Searle implementation.** `cognitive` holds a
  16-agent Searle core, wired at startup and reachable from the server.
  `phinum/searle.rs` is a second one inside a 13-file subtree with zero
  references. Keep the live one; decide the other's fate explicitly.
  *Accept:* exactly one Searle implementation reachable from a binary.

- **D2 — Wire or delete the orphans.** `phinum` (13 files, zero refs) ·
  `attention` (only consumer is a dead private fn) · `attention_cross.rs` (not
  declared in `lib.rs`; never compiled) · `synthesis` (test-only) ·
  `reasoning/{abstraction, counterfactual, planning, sorting}` (zero refs).
  *Accept:* every module either called from a binary or removed · no new
  dead-code allow attributes added to hide it.

- **D3 — Regression gate in CI.** The harness has falsified four claims, two of
  them its own. That only keeps working if it runs unprompted.
  *Accept:* CI runs `experiment`, `conception`, `relations`, `forgetting` ·
  thresholds committed as data · a deliberate change updates them in the same PR.

---

## E — The two questions still genuinely open  *(timeboxed)*

- **E1 — Is there any task where γ\* > 0?** Six attempts say no for next-word.
  Try word-sense disambiguation, relation classification, retrieval reranking. If
  γ\* stays at zero across all three, remove the phase term from the LM scoring
  path — it costs compute for nothing — and let the manifold serve retrieval only.
  *Accept:* three tasks tried, each with a γ sweep · a written decision either
  way · removal shipped if the answer is no.

- **E2 — What grounds the kernel?** A dictionary defines its core circularly, so
  composition can arrange meaning but cannot originate it — Harnad's symbol
  grounding problem in the form the dictionary's own structure takes. ~1% of the
  vocabulary must be grounded another way: corpus statistics, an external
  embedding as a one-time initialiser, or a sensorimotor proxy. Testable
  prediction: composition keeps paying until kernel coverage saturates, then
  plateaus.
  *Accept:* relation accuracy plotted against kernel coverage · the plateau
  observed or shown absent.

---

## Definition of done

A working model means all five, measured:

1. **Learns online.** Unknown word fetched, composed, queryable within one turn,
   surviving restart. (B4, timed by A4.)
2. **Retains.** ≥ 95% across three sequential domains, with intervals. (C3.)
3. **Edits locally.** One fact changed in < 100 ms with a measured bound on what
   else moved. (C2.)
4. **Retrieves competitively.** Recall@10 within reach of BM25 at materially
   lower query latency. (C1.)
5. **Holds its shape.** Phase dispersion ≥ 0.40 in the shipped configuration,
   enforced in CI. (B1, D3.)

## Stop conditions, decided in advance

- **Kill the phase back-off** if E1 returns γ\* = 0 on all three additional tasks.
- **Kill the composition line** if the analogy gain does not survive A2's
  300-pair benchmark with error bars. The current 10.49% rests on 23 pairs.
- **Kill the retrieval pitch** if C1 cannot beat BM25 on latency at comparable
  recall. Being novel is not a feature.
- **Do not kill on perplexity.** That question is settled and it is the wrong
  scoreboard. Losing to Kneser-Ney is not new information.

---

### Sources for the imported mechanisms

- Tissier, Gravier & Habrard, *Dict2vec* (EMNLP 2017)
- Faruqui et al., *Retrofitting Word Vectors to Semantic Lexicons* (NAACL 2015)
- Vincent-Lamarre et al., *The Latent Structure of Dictionaries* (TopiCS 2016)
- Harnad, *The Symbol Grounding Problem* (1990)
