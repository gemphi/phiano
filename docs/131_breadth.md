# 131 · Breadth: a wide scope's trust, and what it changed

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. The fault docs/126 left

A condition's counts are smoothed toward their base with weight one — one sighting is never certainty — and that is right for a word. It is wrong for a directory with five hundred rows: a call it never made was read at base/500, six nats of punishment for being certain of what it never said, and reading a held-out file under its directory scored *below* reading it flat (0.138 against 0.149 on our crates, docs/126 §2).

## 2. Breadth

The weight a condition gives its base is now, for the levels, how many different values it has said (`Counts::dist_breadth`; Witten and Bell's estimator, no constant to set). A word that has said one thing once keeps weight one, as before. A chapter that has said a hundred different things in five hundred rows keeps its base at a hundred: what it never said is rare, not impossible. Only `read_levels` uses it — a level may be a whole chapter — and a word's own reading (`dist`, the proportions of the code student, the posts of the builders' game) keeps weight one. That split was measured, not chosen: breadth everywhere took the code student's never-seen test from 15% to 8%, because its exchange thresholds (a doubling of the odds) were earned against weight-one movements, and softening every word's movement left fewer exchanges and more bodies that did not compile. Breadth for the levels alone leaves the student at 15 / 75 / 61 exactly.

## 3. Measured

| | flat counts | levels, weight one (docs/126) | levels, breadth |
|---|---|---|---|
| our crates, held-out file, `calls` etc. (prior 0.169) | 0.148 | 0.138 | **0.157** |
| Phi-4's 253 by family (prior 0.338) | 0.390 | 0.390 | 0.390 |
| context tests, 25–2,000 turns | — | 10/10 · 20/20 · 20/20 | 10/10 · 20/20 · 20/20 |
| the story going on (`--in text/story`) | — | said back exactly | said back exactly |

On our crates the directory as a condition now helps (0.157 against 0.148 flat) where it hurt; both still read below the prior, which says what docs/124 said — a new function's words and place say little about which of 3,700 calls it makes — and is a separate question from the one this doc answers. Nothing else moved.

`code_relations <dirs> --held-out file --r 8 --epochs 2` (board line "levels") · `cargo test --lib relate::`
