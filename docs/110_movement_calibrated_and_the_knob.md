# 110 · Movement calibrated, and the size knob on code

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Where docs/109 left it.** Movement over positional conditions (`w` stood `d` tokens back) ranked the next Rust token better than the trigram, but it was overconfident. A single global α kept either the ranking or the calibration, never both.

**What changed** (`code_reader`, new module `calibrate.rs`):

1. **Weights per condition kind.** Each condition now has a weight α[d, n], where n is how many times the word was seen at distance d (six buckets: <4, <16, <64, <256, <1024, more). The base n-gram has its own weight β. These 49 numbers are fitted by likelihood on the dev files (3 epochs of Adagrad), never on the test files.
2. **Stronger readers to beat.**
   - the 4-gram;
   - the file cache of Tu, Su & Devanbu (2014), *On the localness of software*: how often each token occurred in the last 1,000 tokens of the same file, mixed with the trigram;
   - a plain linear mix of the n-gram and movement.

   Each mixing λ is chosen on dev.
3. **Two splits of the files.** Split 0 has 139 test files and 22,891 positions; split 1 has 125 test files and 21,491 positions.
4. **The size knob.** Each condition keeps only its R strongest relations (`--top R`).

## Results (test files; each cell is split 0 / split 1)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| trigram | 48.2 / 49.4% | 74.7 / 76.0% | 0.599 / 0.610 | 12.4 / 11.2 |
| trigram + file cache (Tu et al.) | 48.3 / 49.4% | 74.9 / 76.1% | 0.600 / 0.611 | 10.5 / 9.8 |
| **4-gram** | 55.7 / 57.2% | 78.3 / 80.3% | 0.658 / 0.673 | 10.0 / 8.6 |
| movement alone, α = 1, K = 8 | 53.3 / 55.3% | 76.9 / 78.1% | 0.637 / 0.655 | 68.6 / 49.3 |
| trigram ⊕ movement, α = 0.35 (docs/109) | 53.6 / 54.9% | 78.0 / 78.6% | 0.643 / 0.655 | 16.3 / 13.8 |
| 0.8 · 4-gram + 0.2 · movement (linear) | 58.9 / 60.4% | 80.5 / 82.4% | 0.685 / 0.700 | 8.8 / 7.6 |
| trigram ⊕ movement, **weights learned** | 58.0 / 59.5% | 80.6 / 82.3% | 0.679 / 0.695 | 8.1 / 7.3 |
| **4-gram ⊕ movement, weights learned** | **60.0 / 62.0%** | **81.5 / 83.1%** | **0.695 / 0.713** | 7.4 / 6.6 |
| the same + file cache | 60.0 / 62.0% | 81.6 / 83.1% | 0.695 / 0.713 | **6.8 / 6.1** |

**Identifiers only** (6,117 / 5,691 positions):

| reader | top-1 | top-5 | perplexity |
|---|---|---|---|
| 4-gram | 25.1 / 27.8% | 46.6 / 50.4% | 150.9 / 103.6 |
| trigram + file cache | 16.0 / 17.0% | 36.0 / 36.5% | 119.6 / 108.1 |
| 4-gram ⊕ movement, weights learned | **34.0 / 38.6%** | **54.1 / 58.0%** | 64.5 / 45.6 |
| the same + file cache | 33.9 / 38.5% | 54.1 / 57.8% | **46.4 / 35.4** |

## The size knob: relations kept per condition (4-gram base)

For scale, the 4-gram model itself holds 224k / 231k entries across its bigram, trigram and 4-gram tables.

| R per condition | relations kept | top-1 | identifiers top-1 | perplexity |
|---|---|---|---|---|
| none (4-gram alone) | 0 | 55.7 / 57.2% | 25.1 / 27.8% | 10.0 / 8.6 |
| 2 | 75k / 76k | 57.3 / 58.5% | 26.2 / 29.0% | 8.5 / 7.5 |
| 4 | 116k / 119k | 57.7 / 59.8% | 27.1 / 30.0% | 8.2 / 7.3 |
| 8 | 154k / 158k | 58.2 / 59.8% | 28.2 / 31.4% | 8.0 / 7.0 |
| 16 | 188k / 192k | 58.7 / 60.4% | 28.7 / 32.8% | 7.7 / 6.8 |
| 64 | 245k / 251k | 59.8 / 61.5% | 31.8 / 36.1% | 7.5 / 6.6 |
| all | 342k / 350k | 60.0 / 62.0% | 34.0 / 38.6% | 7.4 / 6.6 |

## Reading

- **Calibration is solved, and the ranking kept.** With learned weights, movement over the 4-gram wins on every measure on both splits:
  - top-1 is +4.3 / +4.8 points over the 4-gram;
  - perplexity is 26% / 23% lower;
  - on identifiers, top-1 is +8.9 / +10.8 points and perplexity is less than half.

  It also beats the linear mix of the same two readers. So how the relations are composed matters, not only that they are there.
- **The weights say what the relations are for.**
  - Over a 4-gram, distances 1–3 get small weights (≤ 0.2 once a condition is common), because the 4-gram already sees them. Distances 4–8 keep 0.2–0.36.
  - Over the trigram (first run), it was distances 1–2 that went small, and d = 3 was the largest.

  In both cases, movement supplies the context beyond the n-gram's reach.
- **Rarely seen conditions get more weight per unit, not less.** I expected, from the two speeds of paper 19, that conditions seen often would earn more trust. The data say the opposite: weights fall from about 0.3 (n < 4) to about 0.05–0.1 (n ≥ 1024). A common condition (`;` or `(` d places back) moves many tokens a little and mostly repeats the n-gram. A rare one (a particular name) is specific. The two-speed idea is not refuted here, since this is a weight on counts, not a function, but it is not what these weights show.
- **The file cache adds calibration, not ranking.** It lowers perplexity (6.8 against 7.4) but leaves top-1 unchanged. Movement already carries most of what the cache knows, and more, because its conditions are learned across files.
- **The knob works as a knob.** Even R = 2 relations per condition beats the 4-gram on every measure, at a third of the 4-gram's size added. Gains then rise steadily with R, most of all for identifiers: 26 → 34% on split 0, and 29 → 39% on split 1. Names are where the long tail of relations pays.

## Caveats (honest)

- **Size is not held equal.** At R = all, movement adds 150% of the 4-gram's entries. A 5-gram, or a 4-gram with a cache, at equal size has not been tried. R = 2 (+33% size) is the fairest row, and it still wins: +1.3 to +1.6 top-1, and perplexity 13–15% lower.
- **One corpus, our own code, including the reader itself.** The numbers move by a few tenths as the code is edited. Near-duplicate files flatter every reader.
- **Counts only.** Phinet functions (phasors that generalise to untold relations) are not used yet. This is the next test: can functions let R stay small without losing the identifier gains?

`cargo run --release --bin code_reader -- <dirs…> --base 4 [--split 0|1] [--top R]` · raw outputs: `110_raw/`.
