# 109 · Reading Rust as a sequence: Phient movement against n-grams

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Why.** docs/108 read code as a *structure* (who calls what, what owns what), and untold relations did no better than the frequency prior. Coding, as Phi-4 does it, is first a *sequence*: what comes next. This doc asks the plainest question for that: can Phient conditions predict the next token of real Rust better than the standard counting readers?

**Setup** (`cargo run --release --bin code_reader -- <dirs…> <out.md>`)

- Corpus: our own `phinet`, `phiano` and `phiecs` sources, 681 files. Tokens come from the `proc-macro2` lexer. Literals become ⟨str⟩ / ⟨char⟩ / ⟨num⟩, and comments are dropped.
- Split by file hash: 138 test files; the rest train (388 core + 155 dev). Training has 367,200 tokens, and the vocabulary is 4,837.
- Settings are chosen on dev (core → dev), then scored once on test with core + dev as training.
- **Baselines:** unigram, and bigram and trigram with interpolated absolute discounting (D = 0.75).
- **Phient movement (D4):** the condition for token *v* is each of the last K tokens at its distance (`next@d`: *w* stood *d* places back). Score = ln P(v) + α · Σ ln P(v | w@d) / P(v) over the conditions that hold a post (KL ≥ τ, D2).
- **trigram ⊕ movement:** the same sum, with the trigram in place of the unigram as the base.

## Results (test, 22,411 positions)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram | 16.8% | 36.3% | 0.254 | 107.7 |
| bigram | 32.4% | 63.2% | 0.460 | 25.2 |
| trigram | 48.5% | 74.9% | 0.602 | **12.3** |
| movement, α = 1 (D4 as it stands) | 51.2% | 75.0% | 0.619 | 33.8 |
| movement, α = 0.35, K = 4 (chosen on dev) | 48.9% | 75.4% | 0.605 | 13.8 |
| **trigram ⊕ movement** | **54.0%** | **78.2%** | **0.646** | 16.1 |

**Identifiers only** (5,995 positions: names, not keywords or punctuation):

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| trigram | 16.3% | 36.0% | 0.261 | **215.5** |
| movement, α = 1 | 26.0% | 45.5% | 0.354 | 2825 |
| movement, α = 0.35, K = 4 | 11.8% | 35.9% | 0.232 | 352 |
| **trigram ⊕ movement** | **26.0%** | **47.4%** | **0.360** | 549 |

## Reading

- **Ranking goes to movement.** Pairs of tokens under a positional condition, summed as movement, pick the next token better than the trigram does:
  - 54.0% against 48.5% top-1, once combined with the trigram;
  - on identifiers, 26% against 16%.

  A name used three or eight tokens back is a strong condition on what comes next. The trigram cannot see that far.
- **Calibration goes to the trigram.** Movement is overconfident. At α = 1, perplexity is nearly 3× the trigram's. Damping to α = 0.35 brings it close (13.8 against 12.3), but the ranking gain goes with it. No setting tried wins on both.
- **Dev chose by perplexity.** On dev MRR, K = 8 with α = 0.5 was best (0.655), so the ranking numbers above understate what a ranking-selected reader would do. Selecting on test was not done.
- **Caveats.**
  - The corpus is our own code, and many bins share idioms. Near-duplicates may flatter every reader, the n-grams as much as movement.
  - These are counts only. No phinet functions or two-speed learner were used here yet.

## What this means for the 4% model

For code, the relation model's first job is a condition set the n-gram lacks: *what was said d places back*. Kept as counts per (w, d), it already beats the trigram at choosing the next token. What it still needs is calibration. The next step is to learn α per condition, or to gate conditions by post strength, instead of using one global α. Then the same reader goes on Phi-4's own code answers (Stage B), where the question becomes how many relations per word are enough.
