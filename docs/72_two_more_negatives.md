# 72 - Two More Hypotheses, Both Measured, Neither Kept

> _docs/71 §3 named the next two experiments. Here they are, with the numbers that closed them._

`cargo run --release --bin phinet_sides data/rust_book_corpus.txt 3 directed` and `cargo run --release --bin generate_eval`. One seed; differences under ~2 % are noise.

---

## 1. More evidence per rare target side

The tail-perplexity loss after the split (docs/71 §1: mixture operator 134 → 143) was attributed to rare words' target sides being under-trained. Tying them to the source side was refuted. The second idea: keep the sides separate and give each rare target step *more weight* — `RARE_TARGET_BOOST = 3.0` on the target-side nudge while the word has fewer than 16 observations on the channel.

```
directed reading             identity     hinge     mixture (γ_train = 1)    best mix
baseline (docs/71)            158.48     153.36        142.79               125.52 @ 0.1
rare target ×3                158.47     157.21        145.76               125.25 @ 0.1
                               MRR 0.042 → 0.045      0.150 → 0.150
```

Alone, the operators get worse (hinge +4, mixture +3); the identity does not move. The one number that improves is the mixture with the n-gram tables, 125.52 → 125.25 — the largest gain the phase term has ever shown over the tables' own 125.64, and still a third of a point. A bigger step on a noisy side is a noisier side; the tables can average it, the operator alone cannot. Disabled (`RARE_TARGET_BOOST = 1.0`), kept in code.

What this and the tie together say: the tail is not short of *updates* in a way a rate can fix. It is short of *data*. A rare word's target side needs rare words to occur more often, which is what a corpus twenty times larger provides and nothing in the trainer can. The WikiText-2 ranking result (docs/70: directed identity 15.5×) is the same lexicon on more data, and there the two sides win outright.

## 2. Prior-corrected proposals in generation

docs/71 §2: the operator's proposed candidates were all frequent target sides. The fix specified there — score a proposal by its cosine *minus its prior* (the operator's output for the mean context against that word), with a pseudo-count that decays with frequency — was built (`OperatorGuide::compute_prior`, `propose`).

```
arm                                            KN ppl    eval   distinct  tokens
unguided                                         13.6   0.730    0.966    20.0
directed operator, re-weighting only             13.9   0.729    0.950    20.0
untrained, 8 prior-corrected proposals        7301.6   0.723    0.996    20.0
directed, 8 prior-corrected proposals         6729.7   0.738    0.971    20.0
untrained, 64 n-gram candidates, w = 1           15.3   0.723    0.959    20.0
directed, 64 n-gram candidates, w = 1            14.3   0.727    0.955    20.0
```

The prior correction over-corrects: subtracting what a word scores against the average context leaves the words that fit *no* context — the rare tail — and the pseudo-count then lets them in. "to start well → Trpljoin shouldnt extern come ability nothing 9 waiting…" A trained operator is now barely distinguishable from an untrained one (6,730 vs 7,302). Proposals stay at 0.

The one arrangement that shows the operator doing something *good* in generation is the last pair: give the generator 64 n-gram-legal candidates instead of 16 and let the operator decide among them at full weight. The trained operator (14.3) beats the untrained one (15.3) by a clear margin — its ranking reaches the text — and still trails the count-driven pick (13.6) under a judge that is itself an n-gram model. `OPERATOR_WIDEN` ships at 0 with the mechanism in place; a judge that is not a trigram model (held-out human ratings, or a large LM's perplexity) is what would settle whether 14.3 is worse text or just less trigram-like text.

## 3. Where generation stands

The generator is a trigram sampler with phase-shaped tie-breaking, and every metric here is downstream of that. The operator can *only* re-rank what the n-gram offers, and the n-gram's top choice is what a Kneser-Ney judge likes best by construction. Two ways forward, both outside this pass: (a) generate from the operator's *function* directly — decode the next word as the nearest target side to `Fno(context)` among words the n-gram assigns any probability, without the count term at all, and judge with something other than an n-gram; (b) treat the operator as a reranker of whole beams rather than tokens, where its context reaches further than a trigram's. Neither is measured.

## 4. Changes in code

| where | what |
|---|---|
| `src/config/constants.rs` | `RARE_TARGET_FLOOR`, `RARE_TARGET_BOOST` (1.0 = off) |
| `src/trainer/mod.rs` | boosted target step below the floor |
| `src/generate.rs` | `OperatorGuide::{proposals, widen, compute_prior}`, prior-corrected `propose`, frequency-decayed pseudo-count, `Generator::keep`; `OPERATOR_WIDEN` |
| `src/bin/phinet_sides.rs` | `directed` argument runs one reading |
| `src/bin/generate_eval.rs` | the seven arms |
