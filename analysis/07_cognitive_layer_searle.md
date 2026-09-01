# 07 — The Cognitive Layer: 16 Searle Agents, Speech Acts, and "Intentionality"

> Files examined: [`src/cognitive/`](../src/cognitive) (all 16 modules),
> [`data/searle_markers.json`](../data/searle_markers.json),
> [`docs/papers/02_JOHN_SEARLE_INTENTIONALITY_AND_RESONANCE.md`](../docs/papers/02_JOHN_SEARLE_INTENTIONALITY_AND_RESONANCE.md).

The repository's flagship philosophical claim (docs/papers/02) is that Searlean
intentionality — *aboutness*, direction of fit, speech acts — is implemented in phase
space, resolving the symbol-grounding problem. This file audits each claimed component
against its code.

---

## 1. The "16" Agents (What Each Computes)

[`CognitiveCore::process`](../src/cognitive/mod.rs) runs the agent suite per prompt and
aggregates confidence-weighted contributions. **Counting note:** the code numbers 15
agents — slot 14 is absent from both comments and construction order — so "16-agent"
in the README/PLAN is off by one. What actually runs:

| # | Agent | Actual computation |
|---|---|---|
| 1 | Intentionality | phase angle of the prompt's known-word superposition (the "aboutness" direction) |
| 2 | Aboutness | top-3 lexicon words by phase proximity to each prompt word |
| 3 | Reference | first 5 prompt tokens → dictionary snippets (chunk store) |
| 4 | SpeechAct | **keyword/substring matching** against `searle_markers.json` marker lists |
| 5 | DirectionOfFit | fixed mapping speech-act → words-to-world / world-to-words |
| 6 | Satisfaction | known-token fraction + per-act canned text |
| 7 | Background | context wave amplitude / 50, capped at 1.0 |
| 8 | Network | top-3 bigram followers of first 4 tokens |
| 9 | CollectiveIntention | mean confidence + circular sum of prior agents' phase contributions |
| 10 | Awareness | Evaluator scores (coherence/novelty/resonance) |
| 11 | Semantics | pairwise phase distances binned into synonym/related/antonym by thresholds (0.3 / 1.0 / 2.5 rad) |
| 12 | Syntax | bigram probability walk (greedy 8 steps) |
| 13 | SocialOntology | counts of institutional-vs-brute markers; `counts_as` rules from JSON |
| 15 | ObserverRelativity | first/second/third-person pronoun counting |
| 16 | MentalCausation | speech-act classification → **template** `IntentionalState`s → `WordSelector` → **fixed sentence templates** |

## 2. The Speech-Act Classifier: The Load-Bearing Component

[`SpeechActAgent::classify`](../src/cognitive/speech_acts.rs) is the pipeline's
keystone, and it is a **rule-based keyword matcher**: indirect-speech patterns first,
then commissive/expressive/declarative/directive marker substring checks, defaulting to
Assertive. [`data/searle_markers.json`](../data/searle_markers.json) supplies the
lexicons; felicity conditions, perlocutionary effects, and propositional content are
hard-coded string templates.

**Assessment:** as a *linguistic classifier* this is fine — comparable to classic
intent-classification systems, deterministic and auditable. As *intentionality*, it is
Searle's taxonomy used as a labeling scheme, which is precisely the move Searle's
Chinese-Room argument criticizes: syntax (keyword matching) is being presented as
semantics (understanding). The repository's own flagship paper thus overstates itself
in exactly the way its cited philosopher warned against.

## 3. Where Phase Space Genuinely Enters Cognition

Two agents use the phase manifold in a way no keyword system can:

- **IntentionalityAgent**: "what is this about?" = the *direction* of the prompt's
  superposition wave — a geometric answer computed from learned phases, not stored
  strings. If a user has taught the system that `Maya`, `daughter`, `peanut` co-occur,
  prompts about any of them pull the aboutness-vector toward that cluster.
- **SemanticsAgent's synonym/antonym bins**: phase thresholds (< 0.3 rad, > 2.5 rad)
  over learned geometry — meaning-adjacency that *changes* as the manifold learns.

These are small but real instances of **grounded-by-geometry**: relations derived from
the system's own experience rather than lookup. That is the defensible kernel of the
"intentionality" claim — and it is lexical-scale, not sentence-scale.

## 4. Word Selection and Synthesis (The Output Side)

[`WordSelector::select_words`](../src/cognitive/word_selection.rs): prompt content words
→ TorusPhasor resonance top-4 (Belief states) → trigram/bigram followers (Desire/
Intention), capped at 12. [`synthesize`](../src/cognitive/word_selection.rs) then fills
**hard-coded templates** per speech act ("X is connected to Y.", "I will address X…").

**Verdict:** the selection half is a reasonable phase+statistics retrieval; the
synthesis half is canned text. Cognitive output quality is therefore bounded by the
template library, which is small and English-specific.

## 5. The Phase Attention Modules (Modules 21–22 of PLAN.md)

[`src/attention.rs`](../src/attention.rs): 8 fixed heads, each a phase-sector spotlight
(`query = 0.7·head_center + 0.3·context_phase`), token scores
`cos(φ − query)·(1 + 0.1A)`, softmax τ=0.5, head-average output. **No learned
parameters** — attention weights are a fixed function of the manifold state.
[`src/attention_cross.rs`](../src/attention_cross.rs): same idea across prompt↔generated
token pairs. The unused `attention_pick` reranker sits behind `#[allow(dead_code)]`.

**Verdict:** a legitimate *deterministic attention-like readout* over learned phases —
useful, cheap, explainable. But "Multi-Head Phase Sector Self-Attention" invites
transformer comparison, and against a transformer this is attention with the learning
removed: fixed kernels, no value projection, no training signal. Its honest name is
"phase-sector spotlight pooling."

## 6. Scorecard

| Claim | Implementation reality | Grade |
|---|---|---|
| Speech-act taxonomy | keyword rules + JSON lexicons | B (as a classifier) |
| Direction of fit | fixed enum mapping | C+ |
| BDI intentional states | templates instantiated per act | C− |
| Symbol grounding "resolved" | lexical phase geometry (§3) — partial, lexical-scale | C (as claimed: F) |
| Intentionality as aboutness-vector | wave direction from learned phases — small but genuine novelty | B− |
| Multi-head attention | fixed spotlight pooling, no learned weights | C+ (as mechanism), misleading (as named) |
| 16-agent synthesis | confidence averaging → template fill | C |

**Bottom line:** the cognitive layer is a well-organized rule-based NLP pipeline with
two genuinely phase-native organs (aboutness-vector, phase-binned semantic relations)
— sitting under branding that promises far more. Its honest value: deterministic,
auditable pragmatics classification that a phase-manifold router can steer. Its
dishonest value: the claim to have implemented Searlean intentionality. The strongest
upgrade path is not more agents but **learned** classifiers over phase features
(file 16, task 5).
