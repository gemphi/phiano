# 147 · Counted weights as the default: what changed, and what held

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/146 measured a better rule for how much a level trusts its own counts, on held-out prose, and kept the old rule as the default until everything else that reads with levels had been measured under the new one. This doc measures the rest: the code student (docs/124), the relation board (docs/124, docs/126), the context tests (docs/129) and kin (docs/138). Nothing that held under the old rule breaks under the new one, and two readings that the old rule hurt are repaired. **`Weights::Counted` is now the default.** The breadth rule stays selectable for comparison: `story --breadth`, `code_student --breadth`, and its own line on the relation board.

## 1. The code student (`code_student`, Phi-4's 253 answers)

The answers were staged from the user's disk (`data/phi4_code`, 260 files) and read in every test under three rules.

| test | breadth (before) | counted | no levels (`--flat`) |
|---|---|---|---|
| a new type of a known task (`--by-task`) | 187 of 248 (75%) | 187 of 248 (75%) | 187 of 248 (75%) |
| a task never seen (by family) | 36 of 248 (15%) | 36 of 248 (15%) | 36 of 248 (15%) |
| one pointing (`--one-pointing`) | 152 of 248 (61%) | 152 of 248 (61%) | 152 of 248 (61%) |

The results are identical to the answer, down to which candidate was said again (161 / 40 / 11 / 0 / 13 by task). The levels never decide the student's answer. A held-out task's own scope was never told, so the cast draws it at most to its family or the root, and the movement there does not reorder the told answers that the name's words and types already rank. The student's numbers stand under either rule; this also says honestly that the scope tree has not yet done anything for the code student.

## 2. The relation board (`code_relations --held-out file | family`)

The operator lines were run with one epoch only, so they are not comparable with docs/124. The counts lines do not depend on the operator and are compared here.

**Our own crates, by file** (41,200 rows told, 5,764 asked, pool 5,532):

| reading | MRR | hit@5 |
|---|---|---|
| prior (the kind's count) | 0.159 | 25.7% |
| counts, posts at τ = 0.5 | 0.136 | 20.9% |
| counts, every condition weighs | 0.132 | 20.2% |
| counts under the file's scopes, breadth (docs/126) | 0.143 | 21.6% |
| **counts under the file's scopes, counted** | **0.159** | **24.6%** |

docs/126 found the levels line the worst of the counts readings. It named the cause: a directory with five hundred rows was read as certain of what it never said. It also named what had to be built: "the smoothing weight as a count of its own". This is that weight. The levels line rises from 0.143 to the prior's 0.159. Here nothing yet beats the prior: a new file's calls are mostly not predicted by its words, as docs/124 found. But the levels no longer cost anything.

**Phi-4's answers, by family** (1,573 rows told, 90 asked): levels under breadth 0.404, counted 0.389 (hit@5 64.4% → 63.3%); the prior is 0.292 and the posts reading 0.403. Over 90 rows the difference is 1.4 reciprocal-rank units in all, about one answer moved one place. A held-out family's scope was never told, so only the words-and-types level reads. We record the difference and do not read a trend into it.

## 3. The context tests (`context`, docs/129)

| turns · seed | needles | keys | updated keys | read in |
|---|---|---|---|---|
| 400 · 7, 11 | 10 of 10 | 20 of 20 | 20 of 20 | 9 ms |
| 2,000 · 7, 11 | 10 of 10 | 20 of 20 | 20 of 20 | 18–22 ms |
| 5,000 · 11 | 10 of 10 | 20 of 20 | 20 of 20 | 23 ms (breadth on the same machine today: 24–26 ms) |

Everything held is still found, and the counted weight costs no time. Paper 25's 16 ms for 5,000 turns came from a quieter run; on today's machine both rules read in about 24 ms.

## 4. Kin (`story --kin`, docs/138), seed 7

| reading | top-1 | MRR | perplexity: breadth (docs/138) → counted |
|---|---|---|---|
| flat | 20.6% → 20.9% | 0.309 → 0.310 | 159.0 → **139.3** |
| flat, moving from the last word's kin | 20.6% | 0.309 | 165.1 → **140.5** |
| tree by counts, moving from the last word's kin | 20.1% → 20.2% | 0.304 → 0.306 | 209.2 → **151.3** |

docs/138 found that the kin, as a base, cost perplexity (159 → 165). The cause it gave: a rare kin's small table, divided by its small total, puts its whole weight on the one or two words it was followed by. That is the same over-trust, in the same rule. Under counted weights the kin base's cost falls from 6.1 to 1.2 perplexity points. The kin still do not help as a base: 140.5 against 139.3, and top-1 0.3 points lower. In the open stratum, positions whose word was never told after the last word or pair, they still read below the prior (0.5% against 3.4%). The strata otherwise stand: the counts read 26.8% where they were told and 1.2% where they were not (26.5% and 1.1% under breadth).

## 5. The change

- `phiplm::relate::Weights` defaults to `Counted`. Every `read_levels` (the tree of knowing, the context tests, the code student, the relation board) now reads each level with the weight its rows count by leaving out.
- `story --breadth` and `code_student --breadth` read again under the old rule. `code_relations` prints both levels lines.
- The flat reading of a single condition (`dist`, weight one) and the posts (D2, earned against the prior) are unchanged.

What the default changed, as measured:
- held-out prose perplexity: −12 to −13% flat and −25 to −26% on the tree, on two draws (docs/146);
- the levels line on our crates rises to the prior;
- the kin base nearly stops costing.

What it left as it was: every count of the code student, every needle and key of the context tests, top-1 on prose.

Next:
1. **Cross-fitting** (docs/145 §6, docs/146 §2): the cast and the strand's tree choose their scopes on the same words they read. Count each weight on rows the choice did not see.
2. **The open stratum** (docs/137–138): 23% of held-out positions, where nothing yet beats frequency. Kin and the operator both fall below the prior there, and the reading rule was not the cause.

`code_student --phi4 data/phi4_code [--by-task | --one-pointing] [--flat | --breadth]` (about 2 min each) · `code_relations crates --held-out file --r 32 --epochs 1` (11 min) · `code_relations crates/phiano/data/phi4_code --held-out family --r 32 --epochs 1` · `context --turns 400,2000 --seed 7` · `story data/text --in text/emma --held-out 0.05 --seed 7 --kin [--breadth]`. Outputs are in `data/code_student/` and `data/heldout/kin_counted_seed7.txt`.
