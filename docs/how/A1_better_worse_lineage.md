# APPENDIX A1 — The `better` / `worse` Lineage

> _Phiano's `compose/better.rs` and `compose/worse.rs` have a direct and specific
> ancestor: Barbara Huberman's 1968 Stanford PhD thesis, in which a chess endgame
> program is driven entirely by two predicates named `better` and `worse`. This
> appendix reconstructs her design from the original, compares it line for line
> with Phiano's, and extracts the one structural idea Phiano is missing._

---

## 1. Who wrote it

**Barbara Jane Huberman** — later **Barbara Liskov**, of the Liskov Substitution
Principle, CLU, and the 2008 Turing Award — completed her PhD at Stanford in 1968
under **John McCarthy**. The thesis is *"A Program to Play Chess End Games"*,
Stanford Artificial Intelligence Memo **AI-65**, Computer Science Department
report **CS-106**, August 1968. It survives as DTIC accession **AD0673971**.

It is very likely the first program in which the entire control structure is
organised around a pair of comparison predicates literally named `better` and
`worse` — which is exactly the shape of `src/compose/better.rs` and
`src/compose/worse.rs`.

The program handled K+Q, K+R, K+B+B and K+B+N against a lone king. The
knight-and-bishop mate is the hard one, and hers was for a long time the only
program that could execute it **from any starting position**. She also introduced
the **killer heuristic**, still standard in alpha–beta search nearly sixty years
later.

Two other systems get referenced below as controls, because they contribute
things Huberman's design deliberately did not have:

| System | Who | Contributes |
|:---|:---|:---|
| **Checkers player** (1959, 1967) | Arthur Samuel | learned evaluation weights; Alpha/Beta self-play; credit assignment |
| **MENACE** (1961) | Donald Michie | the minimal proof that negative reinforcement is load-bearing |
| **SARGON** (1978) | Kathe Spracklen with Dan Spracklen | the famous woman-authored *chess* program — but no learning at all |

---

## 2. Huberman's architecture

### 2.1 Model versus method

The thesis opens with a distinction Phiano would benefit from adopting
explicitly:

> _"The model, which is a representation of the structure of the problem,
> determines the overall logic of the program. The methods are the heuristics
> which the program uses within this structure."_ (p. 2)

The **model** — stages, measures, forcing tree — is identical across all three
endgames she implemented. Only the **methods** — the specific pattern-recognition
functions — change per endgame. That separation is why the same 200 lines of
control logic solve K+R, K+B+B and K+B+N.

### 2.2 Stages and measures — a lexicographic order

Every position `x` is assigned:

- **a stage** `st(x)` ∈ {0, 1, …, mate} — a coarse, discrete subgoal index,
- **a measure** `m_n(x)` — a fine-grained progress metric *within* stage `n`,
  where **smaller is better**.

Stages are disjoint subsets of position space defined by pattern recognition.
For the rook endgame, Capablanca's principle *"drive the opposing King to the
last line"* becomes a function `quad(x)` recognising the region the rook confines
the black king to, and `squad(x)` measuring that region's size.

### 2.3 `better` and `worse` — and they are not opposites

This is the part that matters most.

```
better(p,q) ≡ st(q) > st(p)  ∨  [ st(q) = st(p)  ∧  m_st(q)(q) < m_st(p)(p) ]     (3.6)

worse(p,q)  ≡ st(q) = 0      ∨  [ st(p) = st(q)  ∧  m_st(p)(p) < m_st(q)(q) ]     (3.8)
```

Read the structure, not the chess:

**(a) `better` is a lexicographic order.** Advance a stage and you are better,
*whatever happens to the measure*. Only within the same stage does the measure
decide. A gain in the fine metric can never buy a loss of the coarse subgoal.

**(b) `worse` is a separate predicate, not `¬better`.** Note the disjunct
`st(q) = 0`: falling back to stage zero is *worse*, unconditionally. And there is
a wide band of positions that are **neither better nor worse** — different stage
relationships where neither condition fires. That gap is deliberate and it is
where the search lives.

**(c) The two predicates have two different jobs.** From `BW(p,Q)` (Figure 2.1,
p. 13):

- accept `q` immediately if `¬worse(p,q) ∧ better(p,q)` — this is the
  **acceptance condition**: real progress, stop searching;
- otherwise keep the **non-worse** successors as the frontier for deeper search —
  this is the **pruning guard**: never explore through a position that gives
  ground.

So `better` says *stop, this is progress*. `worse` says *do not go there at all*.
One terminates, one prunes. Neither can do the other's job.

### 2.4 The forcing tree

> _"The basic premise of this method of play is that from p white is able to
> force a position q better than p. 'Force' means that white must be able to
> answer every black move with an eventual better position."_ (p. 14)

Breadth-first from `p`, with depth `n` meaning 2n−1 plies (n white, n−1 black). A
branch is accepted at depth n only when **every** black reply can be answered by
a forced better position. The tree is then **remembered** and played out
move-by-move until a terminal position is reached, at which point the program
recalculates.

Three properties worth naming: it is **adversarial** (worst case over the
opponent, not average case); it **commits to a plan** and executes it rather than
re-deciding each ply; and it **replans** at plan boundaries.

### 2.5 The two search heuristics

- **Redundant branch cut-off** (p. 21): discard positions whose white-piece
  placement duplicates one already seen on this branch.
- **Killer heuristic** (p. 22): try first the moves that produced a better
  position elsewhere in the search. A cross-branch memory of what worked. This
  one outlived the thesis and is still in every serious engine.

### 2.6 Where the knowledge came from

> _"It is not difficult to convert a principle into a pattern recognition
> function of positions because the pattern is inherent in the principle."_ (p. 4)

Endgame principles were read out of chess textbooks (Capablanca among them) and
hand-translated into stage predicates and measures.

### 2.7 What she said about learning

She was precise and unembellished about it:

> _"the programmer will do the translation"_ (p. 1)

No automated acquisition of new endgames was implemented. But §8 (p. 159, Figure
8.2) sketches how the program might "do some of the inductive learning" itself,
and the induction problem is stated exactly:

> _"Each example is considered representative of a large class of positions and a
> general rule must be defined for that class. If the example is accompanied by
> principles, this simplifies the induction by providing clues to important
> features."_ (p. 6)

That is learning-from-demonstration-with-advice, posed in 1968, and left as
future work.

**So the honest scoring of Huberman's learning capacity is: essentially none, by
design.** The thesis is about *representation of a problem so that search
succeeds*, not about acquisition. Its contribution is the model, and the model is
what Phiano should be borrowing.

---

## 3. Phiano's `better` / `worse`

From `src/compose/tune.rs :: CompositionTuner::refine`, per round:

```
1. PROPOSE   RiverFlow::generate_variations(facet, prompt, depth)   → 64 variants,
             one seeded per phase sector
2. EVALUATE  Evaluator::evaluate_variations(facet, &flows)          → ranked SectorScores
3. SELECT    Discarder::discard_and_train(facet, trainer, &scores)  → keep top 16,
             "discard" bottom 16, train the facet on the top 16
4. CONVERGE  has_converged(scores, prev_best)
5. RECURSE   with the re-tuned facet
```

Scoring (`better.rs`):

```rust
comp_score = eval.overall  * (W_COHERENCE + W_NOVELTY + W_RESONANCE)  // 0.55
           + diversity     * WEIGHT_DIVERSITY                         // 0.10
           + coverage      * WEIGHT_COVERAGE                          // 0.05
           + length_factor * WEIGHT_NOVELTY                           // 0.15
           + alignment     * WEIGHT_ALIGNMENT;                        // 0.30
```

Selection (`worse.rs`):

```rust
let discarded: Vec<u16> = scores[discard_start..].iter().map(|s| s.sector).collect();

for score in scores.iter().take(keep_end) {
    tokens_updated += trainer.train_sentence(facet, &score.text);   // winners only
}
```

Framed on Flower & Hayes (1981) — Planning / Translating / Reviewing, with
reviewing split into evaluating and revising. Structurally it is a **(μ, λ)
evolution strategy**: λ = 64 offspring, μ = 16 survivors, and "reproduction" is
Kuramoto training on the survivors.

---

## 4. Six comparisons

### 4.1 Scalar score vs lexicographic order

| Huberman | Phiano |
|:---|:---|
| `(stage, measure)`, compared lexicographically | one weighted sum of five terms |

Phiano's weighted sum can **trade any quality against any other**. A composition
that is semantically wrong but long, diverse and sector-spanning can outscore one
that is short and correct: `diversity·0.10 + coverage·0.05 + length·0.15 = 0.30`
of purchasable score before `alignment` is even consulted. Huberman's order makes
that impossible by construction — no amount of fine-measure improvement buys a
stage.

**The port:** give compositions a stage.

```rust
/// Coarse subgoal, checked before any fine score. Larger = further along.
fn stage(facet: &Facet, prompt: &str, text: &str) -> u8 {
    let toks = Tokenizer::content_words(text);
    if toks.len() < 3                                    { return 0; }  // degenerate
    if Wave::text(facet, text).norm() < 1e-6             { return 0; }  // cancels to nothing
    if !shares_content_word(prompt, text)                { return 1; }  // off-prompt
    if prompt_alignment(facet, prompt, text) < 0.5       { return 2; }  // weakly on-prompt
    3                                                                    // on-prompt
}

fn better(a: &SectorScore, b: &SectorScore) -> bool {
    b.stage > a.stage || (b.stage == a.stage && b.score > a.score)
}
```

Now `comp_score` only ever breaks ties *inside* a stage, which is the job a
weighted sum is actually good at.

### 4.2 `worse` prunes vs `worse` prints

| Huberman | Phiano |
|:---|:---|
| `worse` is a guard: the search never expands through a worse position, and `st(q)=0` is unconditionally worse | `discarded` is computed, returned in `DiscardResult`, printed by `print_summary`, and never acted on |

Phiano's `worse.rs` doc comment is candid about the reasoning:

> _"The discarder does NOT delete words from the facet. Instead, it trains on the
> winning texts... The 'worse' sectors naturally drift apart as their words get
> pulled toward the winning clusters."_

That does not follow, because `train_sentence` is attraction-only
([HOW 02](02_the_kuramoto_step.md)). Losers are not pushed anywhere; they merely
stop being pulled, while winners are pulled together every round. The result is
not separation — it is **concentration**.

This is also MENACE with bead removal deleted, which converges to random play,
and Samuel without the branch that rejects Alpha's changes when Alpha loses.

**Two ports, and take both:**

```rust
// (a) worse-as-guard: a stage-0 variant is never a survivor, whatever it scores
let survivors: Vec<&SectorScore> = scores.iter().filter(|s| s.stage > 0).take(keep_count).collect();

// (b) worse-as-penalty: reuse the repulsion `correct_mistake` already implements
const LOSER_REPULSION: f64 = 0.25;                       // π would be far too strong here
for score in scores.iter().skip(discard_start) {
    let target = centroid_phase(facet, &Tokenizer::tokenize(&score.text));
    for tok in Tokenizer::content_words(&score.text) {   // spare function words — HOW 10 §3b
        if let Some(p) = facet.lexicon.get_mut(&tok) {
            let away = -(target - p.phase).sin();
            p.phase = (p.phase + LOSER_REPULSION * trainer.learning_rate * away)
                .rem_euclid(TWO_PI);
        }
    }
}
```

### 4.3 Adversarial forcing vs average-case tournament

| Huberman | Phiano |
|:---|:---|
| a plan is accepted only if it works against **every** black reply | a variant wins on its own score, with no adversary |

There is no opponent in a composition task, but there is an equivalent: **the
worst case over inputs**. A composition strategy that scores well on this prompt
and collapses on the next is Phiano's analogue of a line that loses to one
refutation. Scoring each surviving sector against a small held-out prompt set,
and keeping the sector by its **minimum** rather than its mean, is the direct
port — and it is the same discipline as §4.5.

### 4.4 Plan-and-execute vs re-decide-every-step

Huberman's program builds a forcing tree, **remembers it**, and plays it out to
a terminal position before recomputing. Phiano's `Generator::decode` re-derives a
target phase and re-selects from scratch at every token
([HOW 11](11_generation.md)), so it has no notion of a plan it is partway
through — which is one reason its output wanders.

`PhaseFlow` is close to the right structure already: `flow.propagate(2)` computes
forward structure, and `record_step` logs each step. What is missing is committing
to a multi-step target trajectory and following it until a stage boundary.

### 4.5 The referee

| Huberman | Samuel | Phiano |
|:---|:---|:---|
| `better`/`worse` are grounded in chess ground truth — stages come from principles, checkmate is objective | "better" = **winning games against a frozen Beta** | fitness is `eval.overall`, computed by the model being trained |

Phiano's fitness is 40% `coherence`, and `coherence` is the Kuramoto order
parameter, which [HOW 08 §3](08_self_scoring.md) shows is **maximised by total
phase collapse**. So per round: score 64 variants by how synchronised they are →
keep the 16 most synchronised → Kuramoto-train the facet on those → begin round 2
from a more synchronised manifold.

That is not passive drift. It is selection pressure aimed at collapse, on top of a
trainer that was already collapsing on its own. The compose loop is the fastest
route to a degenerate manifold in the codebase.

And the loop already prints the alarm. `score_spread` is computed every round:

```
[round 3/8] best: 0.9412 (sector 27 amber) avg: 0.9350 spread: 0.0071
```

Spread → 0 means the 64 variants have become indistinguishable — nothing left to
select between. Worse, `has_converged` reads a stalled top score as **success**,
so a collapsed run and a converged run have the same signature.

```rust
if spread < 0.01 && round > 0 {
    eprintln!("  [WARN] sector spread {:.4} — variants degenerate; \
               the manifold may be collapsing (docs/how/08)", spread);
}
```

### 4.6 Cross-branch memory

Huberman's **killer heuristic** remembers moves that produced better positions
elsewhere in the search and tries them first. Phiano's `pick_ngram` has no
memory across sectors or across rounds; each of the 64 flows is generated
independently every time.

```rust
// killer table: words that appeared in a winning sector last round get a bonus
pub struct KillerWords { hits: HashMap<String, u32> }
let killer_bonus = 1.0 + 0.15 * (killers.hits.get(word).copied().unwrap_or(0) as f64).ln_1p();
let score = capped * (0.35 + 0.25 * phase_align + 0.40 * resonance) * content * killer_bonus;
```

Cheap, and it is a 1968 idea that has held up for six decades.

---

## 5. Where Phiano is genuinely ahead

Worth stating clearly, because on several axes it is not close.

| | Phiano | Huberman 1968 |
|:---|:---|:---|
| **Knowledge acquisition** | `DefinitionGrounder` converts dictionary text into geometry automatically; `learn_definition_chain` acquires new concepts recursively on demand | *"the programmer will do the translation"* (p. 1) |
| **Induction from examples** | `Composition::compose` trains on the teacher's examples before composing; `ChildCurriculum` runs staged acquisition | posed as future work in §8, not implemented |
| **Population** | 64 variants per round, structured — one per phase sector, so the proposal distribution covers the space by construction | a single search tree |
| **Update latency** | winners are trained into the model in the same round, microseconds | no update at all; the program does not change |
| **Domain generality** | not endgame-specific; the same loop composes text on any prompt | four endgames, each hand-modelled |
| **Diagnostics** | `average_score`, `score_spread` printed every round | none |

The first row is the important one. Huberman identified textbook-principle →
pattern-recognition-function as the key translation step and said plainly that a
human had to do it. Phiano automates exactly that step, from a different corpus.
That is a real advance over the ancestor, and it is
[HOW 05](05_definition_grounding.md)'s subject.

---

## 6. Scorecard

| Mechanism | Huberman 1968 | Samuel 1959–67 | MENACE 1961 | SARGON 1978 | Phiano `compose/` |
|:---|:---:|:---:|:---:|:---:|:---:|
| `better` as acceptance test | ✓ lexicographic | ✓ scalar | ✓ | — | ✓ scalar |
| **`worse` as a distinct pruning guard** | **✓** | ✓ (reject Alpha) | ✓ (remove beads) | — | **✗ — no-op** |
| Coarse subgoal / stage structure | **✓** | ✗ | ✗ | ✗ | **✗ — flat score** |
| Adversarial / worst-case acceptance | ✓ forcing tree | ✓ game outcome | ✓ | ✓ minimax | ✗ |
| Plan committed and executed | ✓ | ✗ | ✗ | ✗ | ✗ |
| Cross-branch memory | ✓ killer heuristic | ✓ rote table | ✓ | ✓ | ✗ |
| External referee | ✓ chess ground truth | ✓ Alpha/Beta | ✓ | n/a | **✗ — self-refereed** |
| Learns evaluation weights | ✗ | ✓ | ✓ | ✗ | ✗ |
| Sequential credit assignment | ✗ | ✓ early TD | ✗ | ✗ | ✗ (trace exists, unused) |
| **Automated knowledge acquisition** | ✗ | partial (book learning) | ✓ | ✗ | **✓** |
| Population diversity mechanism | ✗ | weight perturbation | stochastic | ✗ | **✓ sector-structured** |
| Update latency | n/a | between games | per game | n/a | **microseconds** ✓ |

Phiano wins three rows outright and loses the four that determine whether a
self-improving loop actually improves.

---

## 7. What to change

The 1968 lessons, ported, in order of value per hour:

| # | Change | From | Where | Effort |
|--:|:---|:---|:---|:---|
| 1 | **Make `worse` a guard.** Stage-0 variants are never survivors, whatever they score | Huberman §2.3 | `worse.rs` | 1 hour |
| 2 | **Penalise losers.** Reuse the repulsion `correct_mistake` already has; content words only | MENACE / Samuel | `worse.rs` §4.2 | 2 hours |
| 3 | **Alarm on vanishing `score_spread`**; stop reporting it as convergence | — | `tune.rs` §4.5 | 1 hour |
| 4 | **Fix the weights.** `WEIGHT_NOVELTY` is used twice (inside `base_weight`, and again on `length_factor`); the coefficients sum to **1.15**, not 1.0, so `comp_score` is off-scale and not comparable to `eval.overall` | — | `better.rs` | 30 min |
| 5 | **Lexicographic (stage, measure) ordering** replacing the flat weighted sum | Huberman §2.2 | `better.rs` §4.1 | 1 day |
| 6 | **Anchor fitness externally** — held-out likelihood, or worst-case over a small prompt set | Samuel Alpha/Beta | `better.rs` + [HOW 15](15_proving_it_works.md) | 2 days |
| 7 | **Killer table** across sectors and rounds | Huberman §2.5 | `generate.rs` §4.6 | half day |
| 8 | **Commit to a plan** across several tokens instead of re-deciding each step | Huberman §2.4 | `generate.rs` / `PhaseFlow` | 2 days |

Items 1–4 are under five hours and they stop the loop from actively degrading the
model. Item 5 is the structural idea worth having: **Phiano's 64 sectors are
currently 64 interchangeable seeds; Huberman's design suggests they should be
ordered stages.** A phase manifold with a lexicographic progress order over
sectors is a genuinely novel object, and it comes straight out of a 1968 thesis.

Item 6 is the lesson Samuel adds on top: **never let the thing being trained
decide that it has improved.**

---

## 8. Sources

- Barbara Jane Huberman, *A Program to Play Chess End Games*, Stanford AI Memo
  AI-65 / CS-106, August 1968 — [full PDF, DTIC AD0673971](https://apps.dtic.mil/sti/tr/pdf/AD0673971.pdf)
  · [Internet Archive mirror](https://archive.org/details/DTIC_AD0673971)
  · [Semantic Scholar record](https://www.semanticscholar.org/paper/A-program-to-play-chess-end-games-Huberman/cd0f20859e4ee9347f6d3b1a97026c07bcf27e0a)
- [Huberman — Chess Programming Wiki](https://www.chessprogramming.org/Huberman)
- [Barbara Liskov — Chess Programming Wiki](https://www.chessprogramming.org/Barbara_Liskov)
  · [Wikipedia](https://en.wikipedia.org/wiki/Barbara_Liskov)
- John McCarthy, [*Chess as the Drosophila of AI*](http://jmc.stanford.edu/articles/drosophila/drosophila.pdf)
- [Samuel's Checkers Player — Sutton & Barto, §11.2](http://www.incompleteideas.net/book/ebook/node109.html)
- [Kathe Spracklen — Chess Programming Wiki](https://www.chessprogramming.org/Kathe_Spracklen) · [Sargon](https://chessprogramming.org/Sargon)

Source files audited: `src/compose/better.rs`, `src/compose/worse.rs`,
`src/compose/tune.rs`, `src/compose/mod.rs`, `src/config/constants.rs`,
`src/generate.rs`.

---

**Back to** [the HOW index](00_index.md) · [the verdict](../EVALUATION.md)
