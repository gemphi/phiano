# 66 - Phinet in the Loop: Prediction, Joint Training, Recursion

> _The three things `docs/65_phinet.md` left open, done and measured._

`cargo run --release --bin phinet_next [corpus] [epochs]` reproduces every number here (`data/rust_book_corpus.txt`, 5 epochs, one seed). Same split as `bin/evaluate`: 6,468 train / 808 valid sentences, 6,221 words, Kneser-Ney trigram on the same split 132.07.

---

## 1. The operator in the language-model path

`PhianoLM` scores the vocabulary by mean phase coherence with a context vector, softmaxes it at β = 8, and mixes it with a unigram under a trigram affine. `PhianoLM::gamma_sweep_scored` now takes that phase term from outside, so the operator is measured on exactly the manifold's footing: same β, same affine, same γ grid, same held-out sentences. `phinet::lm::OperatorScorer` supplies `cosine(Fno(context function), word function)` for every word; its identity variant supplies the context function itself, which is the engine's superposition read in function space.

```
phase term                                   alone (γ = 1)     best mix
manifold coherence (PhianoLM, existing)          176.67        125.64 at γ = 0
function-space identity (superposition)          156.33        125.33 at γ = 0.1
fourier operator                                 149.30        125.64 at γ = 0
jointly trained operator (section 2)             134.09        125.46 at γ = 0.1
Kneser-Ney trigram                               132.07
unigram under the trigram affine (no phase)      125.64
```

Reading the function space instead of 16 raw channels is worth 20 points on its own. The operator is worth another 7. Trained jointly with the lexicon it is worth 42 — the phase side of the model, used alone, now sits two points from Kneser-Ney where it sat forty-five points away. The mixture still prefers the unigram: γ = 0.1 shaves a fifth of a point. The phase term has become a respectable language model on its own and is still not adding what the unigram lacks, which is the next thing to look at, and it is a question about *what* the term should predict rather than how well.

Early stopping is on the valid split's ranking. Without it the operator's held-out perplexity went 149 → 161 between two and four epochs while its training loss kept falling; with 8,360 parameters on 104k examples that is the expected shape.

## 2. Operator and lexicon trained together

`phinet::joint::Joint` puts the table of word functions on the operator's tape as a parameter. The context is built on the tape too — `gather` the preceding six words, weight `1/distance`, sum, `unit` — so a word gets gradient as a target (be findable after this context) and as context (make what follows findable). `write_back` returns angle to `theta` and magnitude to evidence, keeping each word's total evidence so only its shape changes.

Two things had to be true before it worked, and both were found by measuring:

* **The table memorises.** 6,221 × 64 complex numbers against 8,360 operator parameters: the first run lowered training loss 0.065 → 0.043 and raised held-out perplexity 149 → 157. `Joint::with_tether` pulls every word back toward its trained position after each step (a proximal term, 0.02) and the table's Adam runs at a quarter of the operator's rate.
* **Every word must be a target.** Training targets from the frequent pool alone improved ranking *inside* the pool (valid MRR 0.1425 → 0.1508) and made perplexity over the whole vocabulary worse (149 → 168): the 5,500 words outside the pool moved only as context and drifted into the softmax's way. With all words as targets and decoys: 134.09.

The written-back facet, scored by the manifold's own coherence, reads 177.77 against 176.67 before — the learned shapes are not what a 16-channel raw-phase dot wants, and that is fine: the operator is now the reader.

## 3. Recursive composition

`phinet::compose` builds identities the way `roles.rs` does — fillers bound to roles by phase multiplication, superposed, normalised — and nests them. The roles are corpus structure: role `d` is "the word that most often stands `d` positions after this one", `d = 1..4`; 704 words have all four fillers. Level 1 binds a word's fillers; level 2 binds their level-1 identities; level 3 theirs. Recovery at level `L` for role `d` is `unbind(I_L(w), d)` ranked against every composable word's level-`(L−1)` object. 563 words train the cleanup, 141 are held out; chance MRR 0.0101.

```
level    phase unbind        + operator cleanup      + operator cleanup
                             trained on level 1      trained on levels 1+2
1        0.292   16.3%       0.717   59.6%           0.722   58.2%
2        0.086    4.1%       0.031    0.7%           0.541   48.6%
3        0.047    1.1%       0.024    0.2%           0.013    0.0%
                             (MRR, hit@1; chance 0.0101)
```

Three findings, in order of importance:

* **Unbinding through the operator works.** At level 1 the learned cleanup takes recovery from 29× chance to 71×, and from one in six exact hits to three in five. This is `Roles::query` with the crosstalk removed.
* **The recursion is real, and it is bounded by what the cleanup has seen.** A cleanup trained only on level 1 is *worse than nothing* at level 2 (0.031 against 0.086): it has learned to snap anything toward a plain word, and at level 2 the answer is an identity, not a word. Trained on levels 1 and 2 it recovers level-2 fillers at 53× chance — the recovery depth that `docs/64` said was unmeasured is now two, with a number on it — and falls to chance at level 3, which it never saw.
* **Phase subtraction alone decays geometrically:** 29× → 8× → 5×. That is the arithmetic of superposition, and no choice of roles changes it. Depth comes from the cleanup, and the cleanup has to be trained at the depth it is used.

So "words as functions within functions" is now: an identity is a function on the relation circle; nesting is binding identities into identities; recursion is unbinding through a learned operator, one level per level it was trained on.

## 4. What is in the code

| Where | What |
|---|---|
| `tensor.rs` | `gather` (scatter-add backward) and `unit` (row normalisation), both in the finite-difference check. |
| `lm.rs` | `OperatorScorer`: the operator (or identity) as a vocabulary scorer aligned to `PhianoLM`'s index. |
| `metrics/harness.rs` | `PhianoLM::vocabulary`, `PhianoLM::gamma_sweep_scored`. |
| `joint.rs` | `IndexExample`, `index_examples`, `Joint` (table on the tape, tether), `write_back`. |
| `compose.rs` | `role`, `bind`, `unbind`, `compose`, `Composition` (fillers from corpus offsets, nested identities, recovery, per-level cleanup examples). |
| `operator.rs` | `attach_param`, `reset`, `mask_modes`. |
| `bin/phinet_next.rs` | The three measurements. Saves `data/phinet_joint.fno`. |

## 5. Next

* The mixture. The phase term at 134 and the unigram at 126 combine to 125.5: they are predicting nearly the same thing. Train the operator against the *residual* of the trigram affine — what the n-gram tables cannot predict — instead of the next word outright.
* Depth three. Train the cleanup on levels 1–3 and see whether the pattern (recovery at every trained depth, chance one deeper) holds, or whether it saturates.
* One operator. The LM operator and the cleanup are separate networks with the same architecture. Train one on both objectives and see if either loses.
