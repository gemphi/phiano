# 96 · Learning overnight

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. docs/95 measured that
more of Phi-4's answers is what moves the operator past the answer's type
(within-kind held out 0.088 at 704 rows, 0.127 at 1,148). A night on the
user's CPU is about 900 more questions at ~30 s each.

## 1. What runs

`scripts/overnight.ps1` (one command, unattended):

1. keeps the PC awake (`SetThreadExecutionState`, released at the end);
2. runs one one-kind-at-a-time quest (docs/94) per seed word — bread, river,
   hammer, doctor, tree, car, computer, music; 110 questions each, depth 2,
   kinds is-a, has, part-of, made-of, used-for, does — each saving every
   answer as it arrives; a seed whose table exists is skipped, so a rerun
   resumes;
3. merges every table of the night and `phi4_laws` into `all.tsv`;
4. trains `kind_operator` on it (`fno`, `fno+closure`, laws at 30%) with seeds
   7–10 and writes `summary.txt`;
5. logs everything to `night.log`.

Output: `data/quest/night_<date>/`. Estimated 7–8 h for the quests and under
an hour for the training. The merge, the summary and the script's syntax were
checked in PowerShell 7 before the night; the quests and the training are the
same commands already run today.

## 2. What the morning reads

`summary.txt`: rows, prior, and held-out MRR / within kind for both arms at
four seeds. The question it answers: does within-kind held out keep rising
with rows (0.088 → 0.127 → ?) when the rows come from eight unrelated
neighbourhoods rather than one.
