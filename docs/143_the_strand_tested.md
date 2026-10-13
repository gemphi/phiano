# 143 · The strand, tested: does it find a text's true structure?

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/142 claimed the strand measures how many units a text's information supports. The user said "test it". Unit tests show only that the code does what was written, so this doc asks the empirical question three ways: against texts whose true boundaries are known, against texts with no structure, and against the talk's own seams. It first checks that the code tested here is the code on the user's disk.

## 0. The code under test is the code on disk

The user's machine has no Rust toolchain in its sandbox (2 cores, 3 GB), so the workspace was built and tested in the container. Every source file was checksummed on both sides: 820 files on disk and 812 in the container.

The differences are all accounted for:
- **Present only on disk, and not compiled:** `.sync/backup/`, `refs/`, stale build output under `target_cli/`, the three phikit modules moved to `_to_delete/`, and the 253 Phi-4 answers under `data/phi4_code/`, which are data the code reads.
- **Present only in the container:** a stray `crates/phiton/Cargo.lock` that the workspace does not use.
- **One compiled file present only on disk:** `examples/meaning_demo.rs`. It was copied into the container, compiled, and run.

With those set aside, the digests of every compiled source file are identical (`a082c52b…` for phiano, the rest file by file). On that exact file set: **388 tests pass, 0 fail, across 87 test binaries; 0 warnings.**

## 1. Texts with known boundaries (`strand --bench`)

Each trial picks `k` chapters of *Emma* (2 to 8) and takes a run of whole paragraphs, about 120 tokens or more, from each. The runs are joined, so the true boundaries are known. The prior is *Emma* **minus those k chapters**, so nothing tested was counted. The strand divides the joined text, and the result is scored with:
- **Pk** (Beeferman et al. 1999): the probe error, 0 perfect and about 0.5 chance.
- **Boundary F1** within ±10 tokens.

Each score is compared against equal-length parts and against random cuts (20 draws), both given the true `k`. All segments come from one novel, with the same characters and the same style, so this is the hard case.

| run | trials | tokens a text | units vs k: correlation · exact · within one · mean \|u−k\| | Pk: strand at k · equal · random | F1 at k: strand · equal · random |
|---|---|---|---|---|---|
| seed 7 | 60 | 950 | 0.74 · 8/60 · 29/60 · 2.10 | **0.310** · 0.390 · 0.474 | **0.390** · 0.158 · 0.078 |
| seed 11 | 60 | 1,079 | 0.81 · 15/60 · 30/60 · 2.33 | **0.310** · 0.401 · 0.469 | **0.345** · 0.162 · 0.080 |

- **Boundaries.** Given the true count, the strand's cuts are 2.2–2.5 times as often right (F1) as equal parts, and 4–5 times as often as random. Pk falls from about 0.40 to 0.31.
- **The count** rises with `k` (correlation 0.74–0.81), but **it over-counts.** In seed 7, `k = 8` gives 11.9 units and `k = 5` gives 6.9. Over-counts outnumber under-counts 37:15 and 39:6.

## 2. Is the over-count noise, or finer structure?

Two checks.

**A single chapter (`--kmin 1`, seed 3, 120 trials).** One run from one chapter, about 230 tokens, supports 1.61 units on average. The excess over `k` grows with the length of text:

| k | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|
| tokens | 231 | 428 | 555 | 866 | 961 | 1,187 | 1,388 | 1,807 |
| units supported | 1.61 | 2.53 | 3.88 | 5.00 | 6.92 | 8.47 | 8.31 | 10.33 |
| excess | +0.6 | +0.5 | +0.9 | +1.0 | +1.9 | +2.5 | +1.3 | +2.3 |

That is roughly one extra unit per 500–800 tokens of same-novel prose. It is the shape expected if chapters have topical sub-structure: a paragraph about Mr. Elton is unlike its neighbour about Harriet's letter.

**Where the extra cuts fall.** Of the strand's cuts that are not chapter boundaries, 14% (seed 7) and 10% (seed 11) fall within two tokens of a paragraph start. Chance would give 4% and 3%, so the extra cuts land on paragraph starts **about 3.5 times as often as chance**. But most extra cuts, 86–90%, fall inside paragraphs. Honestly, then: part of the over-count is finer structure the chapter labels do not mark; most of it is the strand dividing inside paragraphs where a reader would not.

## 3. Text with no structure

The same texts with their tokens **shuffled**: mean 1.03 / 1.05 / 1.06 units, and **one unit in 59/60, 59/60 and 117/120 trials.** The strand does not invent structure where there is none. The MDL pruning is what holds this: a division must pay for itself, its subtree included.

## 4. The talk's own seams

The seven beats of docs/142 begin at tokens 31, 60, 189, 364, 438 and 518. Read at each:

- **364** — "and that's it . ‖ thank you very much": **the end of the talk proper**, exactly.
- **518** — "can engineers really put them in ? ‖ no , you would have to learn": **the turn from the question to the answer**, exactly.
- **438** — "where you also talked about ‖ mpc": inside the questioner's turn, where the question turns to MPC. It is a topic boundary, not a turn boundary; the turn itself begins earlier, at "thanks for the talk".
- **189** — "and certainly , ‖ if you're interested in making real progress in ai": a topic shift, after a comma.
- **60** — "sample efficiency . ‖ and i know there are people here": after a full stop.
- **31** — "i've been saying forever to ‖ abandon reinforcement learning": mid-sentence.

Two of six are exactly on a structural seam (end of talk; question to answer), found with no speaker labels. Three more are on topic shifts. One is mid-sentence.

## 5. What this means for the claim

The strand's boundaries are real signal: clearly better than equal or random cuts on the hardest case we have, and exact on the talk's two structural seams. Its silence on structureless text is reliable: 235 of 240 shuffled trials give one unit. Its **count is a calibrated relative measure but biased upward on long same-style prose**, by about one unit per 500–800 tokens, mostly from cuts inside paragraphs. So, as first read, "the talk supports seven units" looked like an upper estimate of its top-level beats. **docs/144 revises this:** on synthetic text with known structure the criterion almost never over-counts (11 in 480 trials), and the excess on *Emma* and on documents is topical structure inside the labelled units.

Next, in the order the numbers point:
1. **Pair-aware code.** Score segments with pairs as well as words (the D2 post), so a division inside a phrase costs what it breaks.
2. **A cross-domain suite.** Joins of different sources, where the count should be near-exact.
3. **Paragraph starts as marks.** Make them part of the cut codebook alongside sentence marks, which the tokenizer currently drops.

`strand --bench --trials 60 --seed 7` · `--seed 11` · `--kmin 1 --trials 120 --seed 3` (each under 10 s) · `cargo test -p phiano --bin strand`
