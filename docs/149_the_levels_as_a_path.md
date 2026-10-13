# 149 · The levels as a path: the tree of knowing read as one mixture

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/148 found that a nested reading in the chain form — each level moving what the levels above arrived at — costs a hierarchy its place, whatever its weights. A parent's weight serves the parent, and it carries the sibling's words into every child. The path form was better everywhere on the strand: the unit, every scope above it and the prior read as one mixture, with weights counted for the unit. The tree of knowing (`Counts::read_levels`) reads its levels in the chain form. This doc gives it the path form and measures it on everything that reads with levels.

## 1. The change (`relate::path`, `Weights::Path`)

A context's levels are read as

`p(v) = π_base · base(v) + Σ_j π_j · c_j(v)/n_j`

over the base (the prior, or the operator's or kin's distribution) and every condition that earns its post (D2, against the prior, unchanged). The weights are counted for this context, on the rows of its innermost posted condition: the most specific thing the context has said before (the last pair, or the pair under the paragraph). Each row is left out of every count it could be in. They are the Laplace-ruled fixed point of the share each component explains, `relate::mixture_weights`, which is also the strand's path rule. A chapter whose other paragraphs say other things keeps only the rule's floor.

`Weights::Path` is now the default. `Breadth` and `Counted` stay selectable: `story --weights breadth|path`, `code_student --weights …`, and a line each on the relation board.

## 2. Held-out *Emma* (`story --held-out 0.05 --weights path`)

| reading | seed | top-1 | MRR | perplexity: counted (docs/146) → path |
|---|---|---|---|---|
| flat (the last word, then the last two) | 7 | 20.9% → 20.8% | 0.310 | 139.3 → **137.0** |
| flat | 11 | 20.4% | 0.308 | 129.7 → **128.2** |
| tree (the scopes a cast finds, then the words under them) | 7 | 20.4% → 20.3% | 0.307 → 0.306 | 149.9 → **141.5** |
| tree | 11 | 19.8% → 19.9% | 0.304 | 139.3 → **131.7** |

- **The tree gains most**: −5.6% and −5.5% in perplexity. Its gap to the flat reading falls from 10.6 to 4.5 (seed 7) and from 9.6 to 3.5 (seed 11). Since docs/135 the tree's perplexity has gone 201.5 → 149.9 → 141.5 on seed 7.
- **The flat reading gains 1.2–1.7%.**
- **Top-1 moves by 0.1 point either way.** As in docs/146, the reading rule decides what is kept for the word that was not first, not which word is first.
- What still keeps the tree behind is the cast: the scopes are drawn from the words said so far, and a held-out paragraph's own scope was never told.

## 3. Everything else that reads with levels

| test | counted (docs/147) | path |
|---|---|---|
| code student: new type · never seen · one pointing | 75% · 15% · 61% | 75% · 15% · 61% (identical to the candidate) |
| relation board, our crates by file (MRR · hit@5) | 0.158 · 24.5% | **0.165 · 25.8%**; prior 0.159 · 25.5% |
| relation board, Phi-4 by family (90 asked) | 0.389 · 63.3% | 0.388 · 61.1% |
| context tests: 400 / 2,000 turns (seeds 7, 11), 5,000 (seed 11) | all found | all found; 7–12 ms, and 19 ms at 5,000 turns (23 ms counted) |

**On our own crates the levels beat the prior for the first time** (0.165 against 0.159; 5,834 functions asked). docs/124 found nothing that beat the prior there, and docs/126 found the levels the worst of the counts readings (0.138–0.143). On Phi-4's families the path is level with the counted rule in MRR and 2 answers lower in hit@5, over 90 rows. The code student and the context tests do not move. The path form is also a little faster, since a level no longer renormalises the reading.

## 4. Where this leaves the ceiling

On held-out prose, the readings now stand at:
- flat: 128–137 perplexity;
- tree: 132–142 perplexity;
- top-1: still the trigram's 20–21%.

The open stratum is unchanged: the 23% of positions whose word was never told after the last word or pair, where the counts read 1.2% and frequency 3.4%. The rule for how the levels combine is not the open stratum's problem. What reads that stratum must reach beyond what was told. The operator is the part of the model built for it, and it is the part that trains: docs/150 is its long run.

`story data/text --in text/emma --held-out 0.05 --seed 7 --weights path` (about 16 min) · `code_student --phi4 data/phi4_code [--by-task | --one-pointing] --weights path` · `code_relations crates --held-out file --r 32 --epochs 1` (14 min) · `context --turns 400,2000 --seed 7` · `cargo test -p phiplm relate`. Outputs are in `data/heldout/path_seed{7,11}.txt` and `data/code_student/relations_*_path.txt`.
