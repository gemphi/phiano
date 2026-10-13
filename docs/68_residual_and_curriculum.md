# 68 - The Residual Loss, and Depth as a Curriculum

> _What `docs/67` left open. One partial confirmation, one modest result._

`cargo run --release --bin phinet_residual` (`data/rust_book_corpus.txt`, 3 epochs, one seed; Kneser-Ney 132.07, unigram under the trigram affine 125.64). The mixture objective costs about 7 minutes an epoch — a full-vocabulary softmax with gradients into every word — against 20 seconds for the hinge.

---

## 1. Train on what is measured

Every LM operator until now was trained by hinge against a decoy and *read* through `softmax(β·cos)` mixed with a unigram under the trigram affine. `docs/67` established that the decoy is not the problem. So the loss is now the reader itself: `Fno::train_epoch_mixture` scores the operator's output against the whole table (`Tape::cosine_table`), softmaxes at the harness's β, mixes with each position's unigram at `γ_train` under that position's affine `(k, coef)`, and minimises the negative log-likelihood of the true word (`Tape::mixture_nll`; both ops are in the finite-difference check). The n-gram constants are held fixed, so the gradient reaches the operator only through the probability mass the other components fail to place.

```
loss                                 alone (γ = 1)     best mix           valid MRR (pool)
hinge against uniform decoys           149.30         125.64 at γ = 0        0.1425
mixture likelihood, γ_train = 1.0      134.16         125.56 at γ = 0.1      0.1486
mixture likelihood, γ_train = 0.5      136.14         125.49 at γ = 0.1      0.1329
mixture likelihood, γ_train = 0.2      140.39         125.47 at γ = 0.1      0.0962
```

Three things are true at once.

* **The likelihood loss is worth 15 points on its own.** At `γ_train = 1` the operator alone reads 134.16 — what joint training reached in `docs/66`, now without touching the lexicon. Training the full softmax instead of a one-decoy hinge is the whole of that gap.
* **The residual does what it was meant to, and it is small.** Lowering `γ_train` makes the operator specialise in what the unigram misses: its own perplexity worsens (134 → 140) and its pool MRR drops by a third, while the mixture *improves*, 125.56 → 125.47. The direction is right. The magnitude is a sixth of a point.
* **So the phase term is redundant with the n-gram tables for what they can predict.** On 6,468 training sentences, a 64-sample cosine between a context function and a word has almost nothing to add to a unigram under a trigram affine once it is forced to add only that. That is a statement about this corpus size and this reader as much as about the operator; it is the number, and it is the one this line of work needed. The mixture question is closed here: the way to a γ that matters is a corpus the n-gram tables cannot cover, not another loss.

`data/phinet_residual.fno` is the `γ_train = 0.5` operator.

## 2. Depth as a curriculum

Levels added one at a time (1 for the first third of training, 1–2 for the second, 1–3 for the last) against all three levels from the start, same total epochs, held-out words.

```
level    all levels at once    curriculum 1 → 2 → 3
1          0.590   58x           0.553   55x
2          0.494   49x           0.449   44x
3          0.348   34x           0.341   34x
4          0.021    2x           0.071    7x
```

The curriculum costs a little at every trained depth — the epochs spent on level 1 alone are epochs the deeper levels do not get — and buys a little at the untrained one: level 4 goes from chance to 7×. A cleanup that met its levels in order generalises one step further than one that met them all at once, but not far. Depth is still bought by training; the order of purchase changes the price by a few percent.

## 3. What changed in the code

| Where | What |
|---|---|
| `phinet/src/tensor.rs` | `Tape::cosine_table` (`[b,1,s] × [V,1,s] → [b,V,1]`, gradients to both), `Tape::mixture_nll` with `MixtureTerms`; both finite-difference checked. |
| `phinet/src/operator.rs` | `MixtureExample`, `Fno::train_epoch_mixture(table, examples, γ, β, …)`. |
| `phiano/src/metrics/harness.rs` | `PhianoLM::mixture_terms(a, b, c)`, `PhianoLM::BETA`. |
| `phiano/src/bin/phinet_residual.rs` | The two measurements. |

## 4. Where this leaves the language-model question

Manifold coherence 177 → function space 156 → operator 149 → likelihood-trained operator 134, against Kneser-Ney 132, with the mixture pinned at 125.5 by the n-gram tables on a corpus small enough for them to cover. The phase side of Phiano is now a language model of roughly Kneser-Ney's strength that has been shown, by construction, to carry little the tables do not. The next question is not a training trick; it is whether that changes on WikiText-2, where the tables are thin, and `bin/phinet_residual data/prose_corpus.txt` is the run — at seven minutes an epoch on 6k words it will want the cosine table on more than one core first.
