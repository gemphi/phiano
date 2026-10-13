# Reading Rust as a sequence

681 files: 543 train (of which 388 core + 155 dev), 138 test · 367200 train tokens · vocabulary 4837 · chosen on dev: α = 0.35, K = 4 · 22411 test positions, 5995 of them identifiers

## All tokens

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 16.8% | 36.3% | 0.254 | 107.7 |
| bigram (discounted) | 32.4% | 63.2% | 0.460 | 25.2 |
| trigram (discounted) | 48.5% | 74.9% | 0.602 | 12.3 |
| Phient movement, α = 1 (D4 as it stands) | 51.2% | 75.0% | 0.619 | 33.8 |
| Phient movement, α = 0.35, K = 4 (chosen on dev) | 48.9% | 75.4% | 0.605 | 13.8 |
| trigram ⊕ movement, α = 0.35 | 54.0% | 78.2% | 0.646 | 16.1 |

## Identifiers only (names, not keywords or punctuation)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 0.0% | 0.0% | 0.011 | 2484.5 |
| bigram (discounted) | 2.2% | 13.3% | 0.089 | 592.5 |
| trigram (discounted) | 16.3% | 36.0% | 0.261 | 215.5 |
| Phient movement, α = 1 (D4 as it stands) | 26.0% | 45.5% | 0.354 | 2825.4 |
| Phient movement, α = 0.35, K = 4 (chosen on dev) | 11.8% | 35.9% | 0.232 | 352.0 |
| trigram ⊕ movement, α = 0.35 | 26.0% | 47.4% | 0.360 | 549.0 |

## Development (core → dev)

| setting | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| dev K=2 α=1 | 43.8% | 71.7% | 0.562 | 23.8 |
| dev K=2 α=0.5 | 42.2% | 72.2% | 0.553 | 15.8 |
| dev K=2 α=0.35 | 38.4% | 69.5% | 0.519 | 17.7 |
| dev K=2 α=0.25 | 34.4% | 64.6% | 0.480 | 22.0 |
| dev K=4 α=1 | 51.5% | 75.1% | 0.619 | 33.6 |
| dev K=4 α=0.5 | 52.0% | 77.5% | 0.632 | 13.4 |
| dev K=4 α=0.35 | 48.8% | 75.6% | 0.606 | 12.9 |
| dev K=4 α=0.25 | 44.5% | 71.3% | 0.567 | 15.3 |
| dev K=8 α=1 | 53.1% | 77.2% | 0.637 | 68.4 |
| dev K=8 α=0.5 | 55.1% | 78.7% | 0.655 | 17.4 |
| dev K=8 α=0.35 | 54.1% | 78.0% | 0.646 | 13.5 |
| dev K=8 α=0.25 | 50.4% | 75.0% | 0.614 | 13.7 |
