# 71 - The Perplexity Harness on Two Sides, and Generation Measured

> _The two items docs/70 left open, done. One holds up, one does not, and both are numbers._

`cargo run --release --bin phinet_sides` (Rust Book, 3 epochs, one seed; Kneser-Ney 132.07) and `cargo run --release --bin generate_eval` (60 held-out prompts, seeds fixed per prompt across arms).

---

## 1. The perplexity harness under both readings

docs/70 measured next-word *ranking* on the frequent pool and found the directed reading (source context → target table) far ahead: 15.5× chance for the bare identity. Held-out *perplexity* over the whole vocabulary is a different question, and it answers differently:

```
reading                          identity      hinge        mixture (γ_train = 1)     best mix
manifold coherence (old reader)   174.40         —             —                     125.64 @ γ=0
one side, v4 (docs/68)            156.33      149.30        134.16                   125.56 @ γ=0.1
source sides, v6                  155.77      157.54        144.14                   125.50 @ γ=0.1
directed, v6                      158.48      153.36        142.79                   125.52 @ γ=0.1
                                   MRR 0.031 → 0.042 (identity), 0.137 → 0.150 (hinge)
```

* **Directed beats source on the two-sided lexicon** for both operators (153 vs 158 hinge; 142.8 vs 144.1 mixture) and on every MRR, as the ranking result predicted.
* **Both are worse than the one-sided lexicon was** (134 → 143 for the mixture operator). The split moved the offset channels of the frequent words apart — the thing it was for — and the full-vocabulary softmax pays for it: a rare word's target side is only ever moved when that word is the *later* word of a pair, so it has half the evidence a one-sided phase had, and perplexity is a sum over exactly those words. Ranking on the frequent pool never sees them; perplexity is nothing but them.
* **The mixture with the n-gram tables does not move** (125.5 either way). Nothing here changes docs/68's conclusion: on this corpus the tables cover what a 64-sample cosine can say.

So the split is a trade at this corpus size: better ranking and composition, worse tail perplexity.

**The obvious fix, tried and refuted.** If the tail's problem is halved evidence, *tie* the sides for low-evidence words — read `tgt` as `src` until a word has 16 observations on the channel — and the rare words get their pooled evidence back without the frequent words getting their collapse back. `TARGET_TIE_FLOOR = 16`, same harness:

```
reading (tied below 16)     identity      hinge      mixture
source sides                 156.20      160.29      145.00
directed                     160.92      160.04      145.95
```

Worse everywhere: directed identity 158.5 → 160.9, hinge 153.4 → 160.0, mixture 142.8 → 146.0. A rare word's target side is not a noisy copy of its source side; it is a different angle with less evidence, and substituting the source side is a category error the harness can see. The constant ships at 0 (disabled) with the machinery in place, so the negative result stays reproducible. What the tail actually needs is more evidence per target side — a wider window for rare words, or a second pass that only couples pairs whose later word is rare — and that is open.

## 2. Generation with the operator — measured, and it does nothing (yet)

`OperatorGuide` re-weights every n-gram candidate by `(1 − w) + w·(1 + cos)/2`. Sixty held-out prompts, 24-token continuations at temperature 0.15, judged by a Kneser-Ney model on the training split (a judge neither system can see), by the engine's own `Evaluator`, and by distinct-token ratio:

```
arm                                 KN ppl     eval    distinct   tokens
unguided                              13.6     0.730     0.966     20.0
untrained operator (control)          13.4     0.725     0.961     20.0
directed operator, weight 0.5         13.7     0.729     0.957     20.0
directed operator, weight 1.0         14.2     0.729     0.953     19.9
```

Nothing. The candidates the generator scores are the n-gram's top 12–16 continuations, and at this temperature the count term decides among them; a factor in [0.5, 1] does not change which one wins. (KN perplexity of 13.6 on generated text, against 132 on real held-out text, is the measure of how much this generator is a trigram sampler.)

**Proposals.** So the operator was let *propose*: its eight nearest target sides among words seen ≥ 10 times, merged into the candidate list with a middling pseudo-count.

```
arm                                 KN ppl     eval    distinct   tokens
untrained operator (control)      10728.4     0.704     0.925     20.0
directed operator, weight 0.5       181.1     0.734     0.922     10.9
directed operator, weight 1.0       180.2     0.738     0.913     11.1
```

Two things are true. The trained operator proposes *sixty times* better than an untrained one of the same shape (181 vs 10,728), so what it has learned is real and reaches generation. And its proposals still wreck the text: "to start well → Parameter to we in." The nearest target sides to any context are the frequent function words and a few high-evidence nouns, because that is what next-word likelihood trains an operator to predict; handed a pseudo-count they win, the n-gram has no continuation from them, and generation stops at eleven tokens. The engine's own evaluator prefers these (0.734 vs 0.730), which says more about the evaluator than the text.

`OPERATOR_PROPOSALS` therefore ships at 0. What proposals need is a *prior-corrected* score — the cosine minus what that word scores against an average context, so a word is proposed for fitting *this* context rather than every context — and a pseudo-count that decays with the word's frequency. Both are cheap; neither is measured.

Also in this pass: beam search now points the guide at each hypothesis's own history (`operator_focus`), and the v6 format round-trips through `Storage::save` / `load` (5.07 MB for 6,221 words, `bin/generate_eval` does it every run).

## 3. What is open, precisely

* ~~Tie target sides to source sides below an evidence floor~~ — measured, refuted (§1). Next: more evidence per rare target side (wider window for rare words, or a rare-target pass).
* Prior-corrected proposals with frequency-decayed pseudo-counts; re-measure §2.
* WikiText-2 at the mixture objective, directed reading.
* Delete the stub files `src/phinet/{tensor,operator}.rs`.
