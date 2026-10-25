# HOW 08 — Self-Scoring

> _The model grades its own work on three axes. This document asks whether the
> grades measure anything outside the model — and shows that one of them rises as
> the model gets worse._

---

## 1. The mechanism

```rust
// src/eval.rs :: Evaluator::eval
resonance = known_tokens / total_tokens;
coherence = if known == 1 { alignment_with_centroid * 0.5 + 0.25 }
            else { (wave.norm() / known as f64).clamp(0.0, 1.0) };
novelty   = 1 - exp(-(angular_dist/π) · NOVELTY_SCALE · 5);
overall   = PhiConfig::eval_overall(coherence, novelty, resonance);
verdict   = Verdict::from_scores(coherence, novelty, resonance);
```

with `NOVELTY_SCALE = 0.3`, and weights `EVAL_WEIGHT_COHERENCE = 0.4`,
`EVAL_WEIGHT_NOVELTY = 0.3`, `EVAL_WEIGHT_RESONANCE = 0.3`.

---

## 2. Worked example — all three scores by hand

Facet: `rust` θ=1.00, `memory` θ=1.10, `safety` θ=1.20, all A=1.0.
Facet centroid: from HOW 06 Case A, arg = 1.10001.

**Input: `"rust memory safety"`**

- **resonance** = 3/3 = **1.000**
- **wave** = 1.35626 + 2.66472i, |Z| = 2.99000
- **coherence** = 2.99000/3 = **0.99667**
- **novelty**: arg Z = 1.10001, centroid arg = 1.10001, angular distance ≈ 0
  → normalised 0 → 1 − e⁰ = **0.000**
- **overall** = 0.4(0.99667) + 0.3(0.000) + 0.3(1.000) = **0.69867**
- **verdict**: coherence > 0.7 and novelty < 0.3 → **CoherentGrounded**

**Input: `"rust memory quantum"`** where `quantum` is unknown

- **resonance** = 2/3 = **0.667**
- known words `rust`, `memory`: Z = 0.99390 + 1.73268i, |Z| = 1.99750
- **coherence** = 1.99750/2 = **0.99875**
- **novelty**: arg Z = 1.04999 vs centroid 1.10001 → dist 0.05002,
  normalised 0.01592, novelty = 1 − e^{−0.02388} = **0.0236**
- **overall** = 0.4(0.99875) + 0.3(0.0236) + 0.3(0.667) = **0.60653**
- **verdict**: coherence > 0.7, novelty < 0.3 → **CoherentGrounded**

Note what just happened. `quantum` — a word the model has never seen — was
**silently dropped** by `Wave::sentence`'s `filter_map`, and coherence *rose*
from 0.99667 to 0.99875. Adding an unknown word made the sentence score better on
the coherence axis. Only `resonance` noticed anything was wrong.

---

## 3. The coherence problem

`coherence = |Z| / N` is the **Kuramoto order parameter**. Two facts about it,
both load-bearing:

1. It is the quantity the training rule **maximises**. `train_sentence` moves
   every phase toward the sentence centroid, which is precisely the gradient
   direction of R.
2. It is maximised at **total collapse**. If every word in the lexicon has the
   same phase, R = 1.0 for every possible input, including random word salad.

So coherence is not an evaluation of the model. It is a readout of how far the
training rule has run. Training will always increase it, and the way to make it
1.0 everywhere is to destroy all the information in the lexicon.

### The demonstration

```rust
#[test]
fn coherence_rewards_collapse() {
    let mut facet = Facet::new();
    for w in ["alpha","beta","gamma","delta","epsilon"] { facet.get_or_init(w); }
    for p in facet.lexicon.values_mut() { p.phase = 2.0; }   // total collapse

    let e = Evaluator::new();
    // a sentence, and pure nonsense, both score identically at the ceiling
    assert!((e.eval(&facet, "alpha beta gamma").coherence - 1.0).abs() < 1e-9);
    assert!((e.eval(&facet, "gamma gamma epsilon alpha").coherence - 1.0).abs() < 1e-9);
}
```

If that test passes — and by construction it does — coherence cannot be used to
compare models, compare checkpoints, or validate training. Every reported
coherence figure in the project needs this caveat attached until the metric is
paired with a dispersion measure (HOW 02 §3) or replaced.

---

## 4. The novelty problem

Novelty compares the input wave's direction to the **whole-lexicon centroid**.
As the lexicon grows past a few thousand words, the centroid becomes a stable
average that barely moves. And under the collapse dynamic it converges to the
same point every word is converging to. So:

- Early training: centroid noisy, novelty erratic.
- Late training: centroid ≈ collapse point ≈ every input's direction → novelty ≈ 0
  for everything.

Combined with §3, the long-run behaviour of the evaluator is: coherence → 1.0,
novelty → 0.0, verdict → `CoherentGrounded` **for all inputs**. The scoring
system converges to a constant.

### A novelty measure that survives

Novelty should be measured against **experience**, not geometry. The 16-layer
`Memo` already stores every past interaction's wave:

```rust
pub fn novelty(memo: &Memo, z: c64) -> f64 {
    let nearest = memo.entries.iter()
        .map(|e| (z - c64::new(e.superposition_wave.0, e.superposition_wave.1)).norm())
        .fold(f64::MAX, f64::min);
    1.0 - (-nearest).exp()     // far from everything seen ⇒ novel
}
```

This is a real novelty signal: "unlike anything I have processed before". It uses
data the system already collects and stores, and it does not degrade as the
lexicon grows.

---

## 5. The weights don't sum to the documented total

`src/config/constants.rs` declares:

```rust
EVAL_WEIGHT_COHERENCE = 0.4;  EVAL_WEIGHT_NOVELTY = 0.3;  EVAL_WEIGHT_RESONANCE = 0.3;   // = 1.0 ✓
```

while `src/eval.rs:115`'s doc comment says *"coherence 45%, resonance 40%,
novelty 15%"*. The behaviour comes from `PhiConfig::eval_overall`
(`src/config/functions.rs:116-118`), which uses the **constants**. So the doc
comment is wrong on all three weights: coherence is 40% not 45%, resonance 30%
not 40%, novelty 30% not 15%.

That is a small thing, but in a scoring path it is exactly the kind of small
thing that makes results unreproducible. Make `eval_overall` the single source of
truth and delete the prose weights.

Separately, the composition weights in the same file sum to
0.25+0.15+0.15+0.10+0.05+0.30 = **1.00** ✓ — so the pattern is inconsistency, not
a systematic error.

---

## 6. What the benchmarks currently measure

This section matters more than the rest of the document.

### `src/metrics/arc.rs`

```rust
let predicted = format!("{} relates to the pattern", task.test_input);
let is_correct = eval_res.coherence > 0.5
    && predicted.to_lowercase().contains(task.expected.split_whitespace().next().unwrap_or(""));
```

`predicted` is a **fixed string template**. It does not depend on any inference —
no synthesis, no search, no reasoning. The correctness test asks whether the first
word of the expected answer happens to appear inside that template. A task is
scored "correct" when coherence > 0.5 and the expected answer begins with a word
in `"<input> relates to the pattern"`.

Whatever number this produces, it is not an ARC score, and it should not be
reported as one. ARC-AGI is a program-synthesis benchmark over grid
transformations; `src/synthesis/` is the module that would actually attempt it.

### `src/metrics/baseline.rs`

```rust
let response: String = words.iter().take(8).cloned().collect::<Vec<_>>().join(" ");
evaluator.eval(facet, &response).coherence
```

All three "baselines" build a word salad and score its **coherence** — the metric
from §3. So the baseline comparison is: how synchronised is a random word list vs
a high-amplitude word list vs a nearest-phase word list. Under collapse, all
three converge to 1.0.

### `src/metrics/generalization.rs`

`local_score` and `extreme_score` evaluate the template sentences
`"{word} is related to the topic"` and `"{word} is a new concept"`. The scores
differ mainly because the templates differ and because the word sets are selected
*by phase distance* — the same quantity being measured. It is close to circular.

### What to replace them with

| Current | Replace with |
|:---|:---|
| ARC via template string | held-out next-word top-1/top-5 accuracy |
| coherence-of-word-salad baselines | Kneser–Ney trigram perplexity on the same held-out split |
| generalization via templates | perplexity on an out-of-domain corpus vs in-domain |
| — | ablation: with vs without the phase re-ranker (HOW 04 §3) |

All four are standard, all four are cheap, and all four produce numbers that can
go down as well as up — which is the property the current metrics lack.

---

## 7. What this buys

The evaluator is not worthless — it does two things well:

- **`resonance` is a genuine, honest metric.** Fraction of tokens known is exactly
  what it says, it cannot be gamed by collapse, and it is the right trigger for
  the envision loop (HOW 09).
- **The `Verdict` enum is good UX.** Nine named states with clear thresholds,
  `Display` implemented, immediately legible in a REPL. Turning three floats into
  "Coherent and novel — insightful" is real interface work and the thresholds in
  `Verdict::from_scores` are sensibly laid out.

---

## 8. How it generalises

The honest scoreboard, in priority order:

1. **Held-out perplexity.** The one number that cannot be gamed by collapse.
   Split `data/rust_book_corpus.txt` 90/10, never train on the 10.
2. **Phase dispersion**, logged beside coherence every epoch (HOW 02 §3). If
   coherence rises while dispersion falls, stop and fix the trainer.
3. **Sector occupancy Gini** (HOW 07 §6). One number for manifold health.
4. **Memory-based novelty** (§4), replacing centroid-based novelty.
5. **Next-word top-k accuracy** against a Kneser–Ney trigram baseline.
6. **The phase-layer ablation.** Without it, no claim about the manifold's
   contribution is supportable.

Keep `resonance` and `Verdict` — they are the parts that work.

---

## 9. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| coherence = Kuramoto order parameter | `wave.norm() / known` in `Evaluator::eval` |
| training maximises coherence | `train_sentence` moves phases toward centroid |
| unknown words raise coherence | `filter_map` in `Wave::sentence` drops them from both numerator and N |
| novelty uses whole-lexicon centroid | `facet.centroid()` in `Evaluator::eval` |
| eval weights disagree between file and doc | `constants.rs` vs `eval.rs` doc comment vs `functions.rs` |
| ARC prediction is a fixed template | `let predicted = format!(...)` in `metrics/arc.rs` |
| baselines score word salad coherence | `metrics/baseline.rs` |

---

**Next:** [HOW 09 — Envision](09_envision_curiosity.md).
