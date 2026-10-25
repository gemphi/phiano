# 148 · A unit read along its path: the tree's cost was its form, not its counting

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/145 §6 found that a strand's unit, read down its tree, reads worse than the same unit read straight under the whole. It guessed why: a span is chosen because its own tokens repeat, and its weight is then counted from those same tokens. docs/147 named the test, cross-fitting: count each weight on tokens that did not choose the span. This doc runs that test and one more, which turned out to be the one that decides it. The guess was mostly wrong. The cost is the *form* of the reading down the tree, and a different form — the unit read along its whole path, with weights counted for that unit — reads better than every other scoping we have, the true labels included.

## 1. The test (`strand --crossfit`)

The texts are the five suites of docs/145, 40 texts each, seeds 7 and 11. The folds are the same: every fifth token is held out. The four remaining fifths are dealt alternately into two halves, A and B. A strand is built on A and read three ways:

- **left out**: each weight counted on A's own tokens, each taken out in turn (docs/145);
- **cross-fit**: each weight counted on B's tokens, which the division never saw (`Strand::scopes_witnessed`);
- **on the read words**: each weight counted on the held-out words themselves. This is a diagnostic, not a reading. No weights can do better at reading those words.

| suite | tree − flat (nats a word), cross-fit | the same, weights on the read words |
|---|---|---|
| cross joins | +0.045 · +0.039 | +0.052 · +0.048 |
| synthetic λ = 0.4 | +0.028 · +0.021 | +0.031 · +0.026 |
| *Emma* joins | +0.003 · +0.002 | +0.004 · +0.003 |
| synthetic λ = 0.2 | −0.002 · −0.001 | −0.004 · −0.003 |

(seed 7 · seed 11; the strand on half the training text)

Cross-fitting does as well as the best weights could, so estimation was not the problem. Even with weights counted on the words being read, the tree loses to the flat units on mixed text and on clear topics. The cost is in the form. A unit's reading down the tree mixes its counts with its parent's *reading*, and the parent's weight was counted for the parent as a whole. A parent is two children, so the weight that serves it carries the sibling's words into each child. No weight a unit can choose for itself undoes that.

## 2. The path (`strand::Path`)

The path reading treats the unit, every scope above it to the whole, and the prior as one mixture:

`p(w) = Σ_j π_j · c_j(w)/n_j + π_prior · prior(w)`

The weights `π` are counted for this unit alone, on its own tokens. Each token is taken out of every count on the path it sits in, and the weights are the Laplace-ruled fixed point of the share each scope explains. This is deleted interpolation with one context per unit (Jelinek & Mercer 1980). A scope that carries the sibling's words gets little weight, because the unit's own tokens do not need it. A scope that helps gets what it earns. The path can always fall back to the flat reading (the unit, the whole and the prior), so it has no reason to read worse than it. `Scopes::path`, `Scopes::read_path` and `Strand::unit_paths` implement it, with tests.

## 3. What the path reads

These are the four fifths, left out. Nats a held-out word saved against reading the text as one scope (seed 7 · seed 11):

| suite | true labels | labels as a path | strand, flat | strand, down the tree | **strand, along the path** |
|---|---|---|---|---|---|
| cross joins | 0.324 · 0.352 | 0.457 · 0.490 | 0.396 · 0.418 | 0.304 · 0.319 | **0.551 · 0.577** |
| synthetic λ = 0.4 | 0.212 · 0.196 | **0.329 · 0.307** | 0.195 · 0.179 | 0.118 · 0.114 | 0.299 · 0.278 |
| *Emma* joins | 0.033 · 0.031 | **0.040 · 0.038** | 0.022 · 0.025 | 0.009 · 0.009 | 0.025 · 0.027 |
| synthetic λ = 0.2 | 0.049 · 0.057 | **0.082 · 0.091** | 0.016 · 0.020 | 0.007 · 0.008 | 0.034 · 0.040 |
| the talk | 0.000 (one label) | 0.000 | 0.069 | 0.043 | **0.086** |

- **The path beats the flat units in every suite**: by 0.155–0.159 nats a word on mixed text (all 80 texts), 0.099–0.103 on clear topics (78 of 80), 0.018–0.020 on faint topics (76 of 80), 0.017 on the talk, and 0.001–0.003 on *Emma* (57 of 80).
- **It is the best reading of mixed text we have.** It saves 0.55–0.58 nats a held-out word against the whole, against 0.32–0.35 for the true labels read the old way. A path is a better form for labels too, which gain 0.13–0.14. The strand's path still beats labels read as a path in 70 of 80 texts.
- **Where the truth is known**, labels read as a path stay ahead: the strand along its path reaches 91% of their gain on clear topics, and 41–44% on faint ones, where it supports few units (docs/144). On one novel's chapters it reaches 62–71%.
- **Down the tree is now the worst reading of the strand**, as docs/145 found. It stays in the code because it is the strand's own code, and the measurement of why it fails.

## 4. What this changes

The finding is about the form of a hierarchical reading, so it reaches beyond the strand. The tree of knowing (`Counts::read_levels`) reads its levels in the same chain form: each level moves what the levels above arrived at, with its own weight. docs/146–147 counted those weights better, but the form is the one measured here. A path reading of the levels — the scopes of the cast, the last word, the last pair, each a component with weights counted for the context being read — is the next measurement on held-out *Emma*, where the tree is still 10 perplexity points behind the flat reading (docs/146).

Next:

1. **The levels as a path.** Measure on docs/135's held-out paragraphs and on the relation board.
2. **Paper 25 §4.4.** The strand along its path is now the strand's reading.

`strand --crossfit --trials 40 --seed 7` · `--seed 11` (about 40 s each) · `cargo test -p phiplm strand`. Outputs are in `crates/phiano/data/talks/strand_bench/crossfit_seed{7,11}.txt`. The cross-joins pool reads the live workspace (docs/145); these runs saw it with docs/147 and this code in it.
