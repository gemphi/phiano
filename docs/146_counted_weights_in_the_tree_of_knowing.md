# 146 · Counted weights in the tree of knowing: the levels read held-out *Emma* better

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/145 found that Witten–Bell, used as a reading rule, trusts a small scope's own counts too much, and that a weight counted from the scope's own rows, each left out in turn, reads a text's held-out words better at every scope. The tree of knowing reads its levels with the same Witten–Bell rule (`Counts::dist_breadth`, docs/131): each level's conditions — the story, the chapter, the paragraph, the last word, the last pair — move the reading the level above arrived at. This doc puts the counted weight into those levels and measures it on the held-out set of docs/135.

## 1. The change (`relate::weights`)

A level reads condition `c` as `d(v) = λ · c(v)/n + (1 − λ) · base(v)`, with `base` what the levels above arrived at. There are now two rules for `λ`:

- **Breadth**: `λ = n/(n + T)`, `T` the values `c` has said (docs/131).
- **Counted**: each of `c`'s rows is left out in turn. It is read against `c`'s share without it, `(c(v) − 1)/(n − 1)`, and against the base. `λ` is the Laplace-ruled fixed point of the share the rows' own counts explain (deleted interpolation, Jelinek & Mercer 1980).

The weight is counted per reading, against the base that reading arrived at, so a chapter is trusted as far as it adds to what the story already says *here*. One approximation remains: the base is not itself read with the row left out, since a paragraph's rows are also in its chapter's counts. That makes the counted weight slightly cautious, not bold.

`Counts::weights` selects the rule (`Weights::Breadth` by default). `Counts::dist_counted` and `Counts::dist_level` carry it into `read_levels`. `relate::left_out_weight` is the one fixed point, shared with the strand's `Scopes` (docs/145). Nothing is set by hand; the weight is what the rows count.

## 2. Held-out *Emma* (`story --held-out 0.05 --counted`)

The set is docs/135's: 5% of *Emma*'s paragraphs are held out, and the next word is read at every position from the words before it. Seed 7 is docs/135's draw (101 paragraphs, 10,578 positions). Seed 11 is a second draw (116 paragraphs, 10,031 positions).

| reading | seed | top-1 | MRR | perplexity |
|---|---|---|---|---|
| flat (the last word, then the last two), breadth | 7 · 11 | 20.6% · 20.3% | 0.309 · 0.308 | 159.0 · 148.4 |
| **flat, counted** | 7 · 11 | **20.9% · 20.4%** | **0.310 · 0.308** | **139.3 · 129.7** |
| tree (the scopes a cast finds, then the words under them), breadth | 7 · 11 | 20.1% · 19.7% | 0.304 · 0.303 | 201.5 · 186.3 |
| **tree, counted** | 7 · 11 | **20.4% · 19.8%** | **0.307 · 0.304** | **149.9 · 139.3** |

- **Perplexity falls by 12–13% on the flat reading** (159.0 → 139.3; 148.4 → 129.7) **and by 25–26% on the tree** (201.5 → 149.9; 186.3 → 139.3), on both draws. This is the largest single gain on held-out prose since docs/135 set the ceiling.
- **The rank barely moves**: top-1 is up 0.1–0.3 points and MRR by at most 0.003. The word ranked first is decided by the largest counts, which both rules keep. What changes is how much probability is left for the word that came when it was not the first. Breadth threw that probability away on small scopes; the counted weight keeps it.
- **The tree is still below the flat reading.** The gap closes from 42.5 to 10.6 perplexity points on seed 7, and from 37.9 to 9.6 on seed 11. Most of what the tree lost in docs/135 was the reading rule, not the scopes. What remains is the cast: the scopes it draws a new sentence into are chosen on the words said so far, the bias docs/145 §6 saw in the strand's tree.

So the honest ceiling of paper 25 §4.7 moves in perplexity, not in top-1. Twenty-one percent top-1 is what the counts' largest entries give; a better reading of the rest does not change which entry is largest.

## 3. Why the default stays breadth for now

The code student and the code reader read their scopes with the same levels (`code_student`'s levels mode, `code_relations`). Their measurements (docs/124: 15% → 61%; 75% of new types of known tasks) were made under breadth. Phi-4's 253 answers they read live on the user's disk (`data/phi4_code`), not in the workspace where the tests are run. Until they are measured under the counted rule, the default does not change: measure before asserting. `--counted` gives the counted reading wherever `story` reads held-out text.

## 4. Next

1. **The code student under counted weights**: stage `data/phi4_code` and run `code_student --phi4 data/phi4_code --by-task` under both rules. If it holds or gains, make counted weights the default.
2. **The cast's bias**: draw scopes on the words said so far, but count their weights on rows the cast did not choose them by. This is the same cross-fitting docs/145 named for the strand's tree.
3. **The strata of docs/137–138** (`--operator`, `--kin`) under counted weights. The operator's base and kin's base are read with the same levels.

`story data/text --in text/emma --held-out 0.05 --seed 7 --counted` · `--seed 11` (about 17 min each; about half of that is the two counted readings) · `cargo test -p phiplm weights`. Outputs are in `crates/phiano/data/heldout/counted_seed{7,11}.txt`.

Also in this round, in `strand --read` (docs/145): each suite now draws from its own generator. Before, the cross suite's growing pool (it reads the live workspace) moved the synthetic draws that came after it. docs/145's tables are re-run on that code; the findings did not move.

**Superseded in part by docs/147.** The code student, the relation board, the context tests and kin were measured under counted weights; nothing that held broke, so `Weights::Counted` is now the default. `story --counted` became `story --breadth`, which reads again under the old rule.
