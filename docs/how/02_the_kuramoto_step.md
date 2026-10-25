# HOW 02 — The Kuramoto Step

> _One sentence arrives. Five angles move. This document computes exactly how far,
> in which direction, and what happens after the ten-thousandth sentence._

---

## 1. The mechanism

Training is **Kuramoto–Sakaguchi phase coupling**. For each token _i_ in a
sentence:

$$\theta_i \leftarrow \theta_i + \eta \Big[\, 0.7 \underbrace{\sin(\bar\theta - \theta_i)}_{\text{semantic}} + 0.3 \underbrace{\tfrac{1}{m}\!\!\sum_{j \in \{i-1,i+1\}}\!\! \sin(\theta_j - \theta_i \pm \beta_{ij})}_{\text{syntactic}} \Big]$$

where

- $\bar\theta = \operatorname{atan2}\!\big(\sum_i A_i \sin\theta_i,\ \sum_i A_i \cos\theta_i\big)$ — the amplitude-weighted centroid of the sentence,
- $\eta$ = `LEARNING_RATE` = 0.05,
- $\beta_{ij}$ = the learned directional lag (HOW 03),
- $m$ = number of syntactic neighbours (1 at the ends, 2 in the middle).

**In the source:** `src/trainer/mod.rs :: Trainer::train_sentence`.

---

## 2. Worked example — "the cat sat on the mat"

A completely fresh facet. Every token is unseen, so §HOW-01 seeding applies:

| token | length | θ (rad) | A |
|:--|--:|--:|--:|
| `the` | 3 | 4.854102 | 1.0 |
| `cat` | 3 | 4.854102 | 1.0 |
| `sat` | 3 | 4.854102 | 1.0 |
| `on`  | 2 | 3.236068 | 1.0 |
| `mat` | 3 | 4.854102 | 1.0 |

Note `the` appears twice but is one lexicon entry, so we have 5 tokens over
4 distinct words.

### Step 1 — the centroid

$$\sum A\cos\theta = 4\cos(4.854102) + \cos(3.236068) = 4(0.14231) + (-0.99546) = -0.289345$$
$$\sum A\sin\theta = 4\sin(4.854102) + \sin(3.236068) = 4(-0.98982) + (-0.09442) = -5.044212$$

$$\bar\theta = \operatorname{atan2}(-5.044212,\ -0.289345) = -1.628095 \text{ rad} \;(= 4.655090 \text{ mod } 2\pi)$$

### Step 2 — semantic force per token

$$F^{\text{sem}}_i = \sin(\bar\theta - \theta_i)$$

| token | θ_i | $\bar\theta - \theta_i$ | $F^{\text{sem}}$ | η·F (η=0.05) |
|:--|--:|--:|--:|--:|
| `the`/`cat`/`sat`/`mat` | 4.854102 | −0.199012 | **−0.197701** | −0.009885 |
| `on` | 3.236068 | 1.419022 | **+0.988504** | +0.049425 |

### Step 3 — what actually moved

`on` — the single outlier — is dragged **+0.0494 rad** toward the pack. The four
3-letter words drift **−0.0099 rad**, a fifth of the distance, in the opposite
direction.

After one sentence:

| token | θ before | θ after (semantic only) | moved |
|:--|--:|--:|--:|
| `the`,`cat`,`sat`,`mat` | 4.854102 | 4.844217 | −0.0099 |
| `on` | 3.236068 | 3.285493 | +0.0494 |

The syntactic term (§HOW 03) adds a smaller correction on top; on the first pass
all `β_ij` are at their default π/16 = 0.19635 and the neighbour phases are mostly
identical, so it contributes little here.

### Step 4 — the order parameter

The Kuramoto order parameter — which the evaluator reports as `coherence` — is

$$R = \frac{1}{N}\Big|\sum_i e^{i\theta_i}\Big| = \frac{\sqrt{0.289345^2 + 5.044212^2}}{5} = \frac{5.052503}{5} = \mathbf{1.0105}$$

clamped to **1.0**.

Stop and look at that number. On a brand-new model that has learned nothing,
before a single update, `coherence` reads **1.00 — the maximum score**. Not
because the sentence is meaningful, but because four of its five tokens are three
letters long and therefore sit at the identical seed angle.

That is the interaction between HOW 01's seeding and HOW 02's metric, and it is
the first thing to fix before any benchmark number from this system means
anything.

---

## 3. The collapse dynamic

Now run the same rule over a corpus.

The Kuramoto model with **all-positive coupling** has one globally stable
attractor: full synchronisation. That is not a bug in the implementation; it is
the theorem the model is named after. Every training step moves every token
*toward* the centroid. Nothing anywhere in `train_sentence` ever pushes two words
apart.

### Worked example — the collapse

Take three sentences, trained in round-robin:

```
the cat sat on the mat
the dog ran in the park
the sun set on the sea
```

There is no shared content between "cat" and "sun" — but `the` appears in all
three. So:

1. Sentence 1 pulls {`the`, `cat`, `sat`, `on`, `mat`} toward centroid c₁.
2. Sentence 2 pulls {`the`, `dog`, `ran`, `in`, `park`} toward c₂ — and `the` has
   just moved, so c₂ is dragged toward c₁.
3. Sentence 3 pulls its tokens toward c₃, which `the` has again dragged.

`the` acts as a **coupling bridge between every sentence in the corpus**. Because
every sentence contains high-frequency function words, the entire vocabulary is
transitively coupled, and the fixed point of transitively-coupled positive
Kuramoto is one point.

Empirically this shows up as: variance of `facet.lexicon[*].phase` decreasing
monotonically with training, `centroid().norm()` increasing toward
`vocabulary_size()`, and `coherence` rising toward 1.0 **on every input,
including nonsense**.

### The tell

Coherence going up during training is currently read as the model learning. It is
equally consistent with the model collapsing. The two are distinguished by one
extra measurement:

```rust
// add to src/trainer/metrics.rs
pub fn phase_dispersion(facet: &Facet) -> f64 {
    let n = facet.lexicon.len() as f64;
    let (sx, sy): (f64, f64) = facet.lexicon.values()
        .fold((0.0, 0.0), |(x, y), p| (x + p.phase.cos(), y + p.phase.sin()));
    1.0 - (sx.hypot(sy) / n)   // 1.0 = uniformly spread, 0.0 = fully collapsed
}
```

Log `phase_dispersion` next to `coherence` every epoch. If coherence rises while
dispersion falls, the model is not learning — it is synchronising.

---

## 4. The two anti-collapse devices already present

The codebase does contain two brakes. Both are real, both are insufficient.

**(a) `band_n` increment.** When `|F^sem| < 0.0005`, `band_n += 1`, shifting
effective phase by α. As computed in HOW 01, this is 1/13.45 of a sector — it
separates numerically identical phasors but not semantically distinct ones. It
buys precision, not discrimination.

**(b) `PHASE_REPULSION` on correction.** `Trainer::correct_mistake` applies a
π-radian anti-phase pulse. This is genuine negative coupling — but it fires only
on explicit user correction, never during corpus training. It is a manual brake
on an automatic process.

---

## 5. What this buys

- **Single-pass online learning.** One `sin()` per token, no gradient tape, no
  batch. `train_online` genuinely learns from one sentence, immediately.
- **Symmetric and stable.** No exploding gradients, no learning-rate cliff; the
  update is bounded by η because `|sin| ≤ 1`.
- **Warmup and convergence detection** (`train_multi_epoch`) are correctly
  implemented — LR ramps over `warmup` epochs, and training stops when
  `max_shift < CONVERGENCE_THRESHOLD`. This is well-built.
- **The right family.** Coupled-oscillator learning is a real and respected model
  class (synchronisation networks, oscillatory neural networks, Ising machines).
  The instinct is sound.

---

## 6. The ceiling

The update rule is **attraction-only**, and attraction-only rules do not learn
representations — they perform agglomerative clustering with one cluster.

Compare with the rule that made word embeddings work — skip-gram with negative
sampling:

$$\mathcal{L} = -\log\sigma(u_c \cdot v_w) \;-\; \sum_{k=1}^{K}\log\sigma(-u_{n_k}\cdot v_w)$$

The second term is the whole game. It samples K words that did **not** occur in
context and pushes them away. Without it, SGNS also collapses to a point — this
is well known and is why the negative term exists.

Phiano has the first term and not the second.

---

## 7. How it generalises

### Fix 1 — negative sampling (the essential one)

```rust
// src/trainer/mod.rs, inside train_sentence, after the positive pass
const K_NEG: usize = 5;
const REPEL_RATE: f64 = 0.5;   // relative to learning_rate

for token in &tokens {
    for _ in 0..K_NEG {
        let neg = facet.sample_by_amplitude();          // frequency-biased draw
        if tokens.contains(&neg) { continue; }
        let (tp, np) = (facet.lexicon[token].phase, facet.lexicon[&neg].phase);
        let repel = -(np - tp).sin();                    // note the sign
        if let Some(p) = facet.lexicon.get_mut(&neg) {
            p.phase = (p.phase + self.learning_rate * REPEL_RATE * repel)
                .rem_euclid(TWO_PI);
        }
    }
}
```

Now the fixed point is no longer a single point. It is the configuration where
phase difference tracks pointwise mutual information — which is exactly the
structure that makes an embedding useful.

### Fix 2 — make the target predictive, not central

The centroid is a *descriptive* target: it says "be like your neighbours". A
predictive target says "be positioned so that the next word is retrievable from
you", which is the objective that forces a model to encode syntax, semantics and
facts. In this substrate that is a ranking rule, still online, still no backprop:

```rust
// context wave → should rank the TRUE next word first under ray_cast
let ctx  = Wave::sentence(facet, &tokens[..i]);
let true_z = facet.lexicon[&tokens[i]].to_complex();
let neg_z  = facet.lexicon[&sampled_wrong_word].to_complex();

// perceptron-style: only update when the ranking is wrong
if (ctx - true_z).norm_sqr() > (ctx - neg_z).norm_sqr() {
    // rotate true word toward ctx, wrong word away
    align(facet, &tokens[i], ctx.arg(),  self.learning_rate);
    align(facet, &sampled_wrong_word, ctx.arg(), -self.learning_rate * 0.5);
}
```

This is a hinge loss on next-word retrieval, updated online, one `sin()` per
update. It is the smallest change that converts Phiano from a clustering system
into a language model — and it is the change that makes the perplexity numbers in
HOW 15 possible to report at all.

### Fix 3 — downweight function words in the centroid

`Tokenizer::is_function_word` already exists. Excluding closed-class words from
the centroid computation (or weighting them by 0.1) removes most of the
transitive coupling described in §3, at a cost of one boolean check:

```rust
let w = if Tokenizer::is_function_word(token) { 0.1 } else { 1.0 };
sum_x += w * phasor.phase.cos() * phasor.amplitude;
```

This alone measurably slows collapse and is a two-line change.

---

## 8. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Update is attraction-only | `train_sentence` — every term is `+η·sin(target − θ)` |
| Centroid is amplitude-weighted atan2 | `compute_centroid_phase` |
| 0.7/0.3 semantic/syntactic split | `combined_error` in `train_sentence` |
| Coherence = Kuramoto order parameter | `src/eval.rs` — `wave.norm() / known` |
| Fresh model scores coherence 1.0 on "the cat sat on the mat" | compute R from the table in §2 |
| No negative coupling in training | grep `PHASE_REPULSION` — only `correct_mistake` uses it |

---

**Next:** [HOW 03 — Learning Word Order](03_learning_word_order.md) — how a
symmetric circle acquires a direction.
