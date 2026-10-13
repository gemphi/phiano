# 145 · Reading in the strand's units: the units are good scopes, and the reading rule was the weak part

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/144 left one question open. The strand counts topical units, and a label (a chapter, a file, a turn) may hold several. Are those units the scopes a reading should use? This doc asks it directly. If a text is read with the strand's units as conditions, does it predict its own held-out words better than when it is read with no levels, or with its true labels only?

The answer is yes, but only once the reading rule is fixed. The first rule tried, Witten–Bell (the code the strand divides with, and the breadth smoothing the levels use, docs/131), made *every* finer scope read worse, true labels included. So the fault was in the rule, not in the units. Under weights the text counts itself, every scoping helps. The strand's units then beat equal parts of the same count everywhere they have something to find. They beat the true labels on mixed text. And the level the strand supports is within 0.01 nats a word of the best of its sixty-four levels.

§23 says the uses of a word are many, and none is fixed once for all. Here the question is narrower: which stretch of a text a word's use should be read against.

## 1. The test (`strand --read`)

- **The folds.** Five folds: in fold `r`, every token `i` with `i mod 5 = r` is held out. The strand is built on the other four fifths with the prior (*Emma*, without any chapter used). Every held-out word is read in the scope that holds the token before it. A held-out word is never counted anywhere, so nothing leaks.
- **The readers.** They differ only in their scopes:
  - `whole`: the text as one scope;
  - `labels`: the true segments under the whole;
  - `equal parts`: as many equal parts as the strand supports;
  - `strand flat`: the strand's supported units, each straight under the whole;
  - `strand`: the same units read down the strand's tree, through every ancestor;
  - `strand at k`: the tree cut to as many units as there are labels.
  
  `prior` (English alone) is the floor. All sixty-four levels are read as well.
- **The suites.** Each has 40 texts per seed, with seeds 7 and 11, about 80,000 held-out words each:
  - *Emma* joins, as in docs/143;
  - cross joins, as in docs/144;
  - synthetic topics at λ = 0.4 and λ = 0.2, where the labels are the truth;
  - the talk, which has no labels (its `labels` reader is the whole).

What this measures is the word distribution of a scope, not the next word. It is the reading the strand codes with, and the reading the tree of knowing's levels give a word before the pairs are consulted.

## 2. Witten–Bell over-trusts a small scope

Nats a held-out word saved against reading the text as one scope (positive is better), seeds pooled:

| suite | prior | labels | equal parts | strand flat | strand |
|---|---|---|---|---|---|
| *Emma* joins | **+0.169** | −0.248 | −0.281 | −0.270 | −0.845 |
| cross joins | −2.156 | +0.204 | +0.096 | +0.262 | −0.534 |
| synthetic λ = 0.4 | −0.811 | −0.014 | −0.132 | −0.037 | −0.517 |
| synthetic λ = 0.2 | −0.051 | −0.305 | −0.351 | −0.352 | −0.534 |
| the talk | −0.935 | — | −0.133 | −0.120 | −0.768 |

On *Emma* even the whole text reads worse than English alone. Even topics that are the truth (λ = 0.4) read slightly worse than the whole. Reading down the tree costs about half a nat a word everywhere. The rule is the problem. Witten–Bell gives a scope the weight `n/(n + T)` on its own counts whatever its words are, which is about two thirds for a few hundred tokens. That is right for coding a text once in order, where a new word escapes. It is too trusting when a word is read that the scope did not happen to say, and the trust compounds down a chain of scopes. The levels plotted under this rule only rise (the orange curve in the figure): the finest reading that helps is the coarsest.

## 3. Weights the text counts itself (`strand::Scopes`)

A scope reads a word as `p_s(w) = λ_s · c_s(w)/n_s + (1 − λ_s) · p_parent(w)`. The weight `λ_s` is counted, not set: each of the scope's tokens is left out in turn and read against the scope without it and the parent without it. `λ_s` is then the weight under which those tokens are best predicted (deleted interpolation, Jelinek & Mercer 1980). It is the fixed point of the expected share own counts explain, Laplace-ruled like the prior (`(r + 1)/(n + 2)`). Scopes are fitted from the root down. A scope whose words repeat leans on itself. A scope whose words never repeat gets almost nothing and leans on its parent. The reading is a distribution: what the vocabulary holds plus what runs down to the unseen sums to one (tested).

Code: `phiplm::prose::strand::reading` (`Scopes::fit`, `Scopes::read`, `Strand::scopes`). `Strand::read` is the Witten–Bell reading, kept as the strand's code.

## 4. Under counted weights

Pooled over both seeds, nats a held-out word saved against the whole:

| suite | prior | labels | equal parts | **strand flat** | strand (tree) | strand / labels | flat beats equal · labels |
|---|---|---|---|---|---|---|---|
| cross joins | −2.188 | +0.329 | +0.254 | **+0.397** | +0.306 | **1.21** | **76/80 · 72/80** |
| synthetic λ = 0.4 | −0.876 | +0.204 | +0.119 | **+0.188** | +0.116 | 0.92 | 65/80 · 6/80 |
| *Emma* joins | −0.024 | +0.032 | +0.018 | +0.024 | +0.009 | 0.75 | 46/80 · 17/80 |
| synthetic λ = 0.2 | −0.231 | +0.053 | +0.017 | +0.018 | +0.007 | 0.34 | 23/80 · 1/80 |
| the talk | −0.954 | — | +0.071 | +0.069 | +0.043 | — | — |

- **Every scoping now helps**, and the whole beats English everywhere. Labels beat the whole in 73/80 (*Emma*), 80/80 (cross), 71/80 (λ = 0.4) and 67/80 (λ = 0.2) texts.
- **Where the units fall matters.** At the same count, the strand's units beat equal parts in 76 of 80 cross joins and in 65 of 80 clear-topic texts. On same-novel prose (*Emma*) and faint topics they are even with equal parts: there is little to find, and the strand supports few units there (docs/144).
- **On mixed text the strand reads better than the labels**: 1.21 times the labels' gain, and better in 72 of 80 texts. This is docs/144's finding in reading terms: the topical units inside a document (a table, a command, a function) are scopes a reading should use, and a document label hides them.
- **Where the truth is known**, the strand recovers 92% of the true labels' gain when topics are clear. When they are faint it recovers 34%, because it stays silent rather than guess.

## 5. The level the strand supports is the level a reading wants

Against the best of the sixty-four levels, picked with hindsight on the held-out words, reading at the level the strand supports loses **0.002–0.008 nats a word** in every suite.

| suite | units supported | units at the best level | best level coarser · same · finer |
|---|---|---|---|
| cross joins | 12.5 | 11.2 | 40 · 20 · 20 |
| synthetic λ = 0.4 | 4.0 | 4.0 | 5 · **59** · 16 |
| *Emma* joins | 5.0 | 3.8 | 38 · 25 · 17 |
| synthetic λ = 0.2 | 2.4 | 2.5 | 14 · 37 · 29 |
| the talk | 6.4 | 8 | 0 · 0 · 1 |

Where the topics are known and clear, the supported level is the best level in 59 of 80 texts. On real prose the best reading is a little coarser, by about one unit. This is the test docs/144 set for pair-aware coding (D2): if the units were too fine, D2 would be the next step. They are not too fine by any amount that matters here (0.01 nats a word), so **D2 is not called for by this test.**

## 6. Down the tree costs more than straight under the whole

Read through every ancestor, the strand's units do worse than read straight under the whole. Under counted weights the tree wins only 1 (cross), 11 (λ = 0.4), 12 (*Emma*) and 26 (λ = 0.2) of 80 texts. The weights along one chain show why. One example from a cross join: a unit of 8 tokens (weight 0.24) sits under a span of 20 (weight **0.74**), under 317 (0.50), under 853 (0.06), under the whole (0.56). The cut chose the 20-token span *because* its tokens repeat, and its weight is then counted from those same tokens. Small spans chosen on the data look more self-similar to themselves than to new words. Labels and equal parts are not chosen on the data, so they carry no such bias; the flat reading has one chosen level instead of five. This is our reading of the diagnostic, not yet a measurement. The measurement is to fit the weights on tokens the cut never saw.

## 7. What this changes, and what comes next

- **The units are good scopes.** On mixed text they beat the labels. The level the strand supports is the level a reading wants, to within 0.01 nats a word. Paper 25 §4.4 carries this.
- **Witten–Bell is a coding rule, not a reading rule.** `relate::Counts::dist_breadth` (docs/131) reads the tree of knowing's levels with the same rule. If it over-trusts small scopes there as it does here, the levels have been costing the reading part of what they should give.

Next, in order:

1. **Counted weights in the tree of knowing.** Replace the breadth smoothing in `relate::levels` with weights counted by leaving out, and measure on docs/135's held-out paragraphs (next word: top-1, MRR, perplexity).
2. **Weights the cut never saw.** Fit each span's weight on tokens held out of the division (cross-fitting), so that reading down the tree is not biased by the spans having been chosen on the same tokens.
3. **D2 (pair-aware code)**: not needed by this test; it stays for a test that asks about order inside a unit.

Reproducibility note: the cross suite reads the live workspace (`docs/*.md`, `phiplm/src/*.rs`), so its pool grows as docs are added: 200 documents for the tables here, 196 in docs/144. The first run of this doc drew every suite from one generator, so the growing pool also moved the synthetic draws after it. Each suite now draws from its own generator (`seed ^ suite << 32`), and the tables above are from that run. The findings did not move; the numbers moved by up to 0.05 nats a word.

`strand --read --trials 40 --seed 7 --out crates/phiano/data/talks/strand_bench/read_seed7` · `--seed 11` (about 50 s each) · `cargo test -p phiplm strand`. Per-suite curves and per-text tables are in `read_seed{7,11}/`. The figure is `docs/papers/fig/f25v_reading.png` (`figs25v.py`, `fig_reading`).

![which scopes read a text's held-out words best](papers/fig/f25v_reading.png)
