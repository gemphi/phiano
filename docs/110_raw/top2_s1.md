# Reading Rust as a sequence, calibrated

682 files: 557 train (of which 412 core + 145 dev), 125 test · 373196 train tokens · vocabulary 4925 · 21491 test positions, 5691 of them identifiers · weights fitted on dev in 3 epochs · base 4-gram · split 1 · relations per condition ≤ 2

## All tokens

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 18.4% | 37.6% | 0.268 | 103.9 |
| bigram (discounted) | 33.7% | 64.1% | 0.472 | 23.6 |
| trigram (discounted) | 49.4% | 76.0% | 0.610 | 11.2 |
| 4-gram (discounted) | 57.2% | 80.3% | 0.673 | 8.6 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 49.4% | 76.1% | 0.611 | 9.8 |
| Phient movement, α = 1, K = 8 | 35.2% | 62.2% | 0.474 | 124133.9 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 46.3% | 75.4% | 0.587 | 51.1 |
| linear mix: 0.95 · 4-gram + 0.05 · movement | 57.4% | 80.1% | 0.674 | 8.7 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 58.5% | 81.0% | 0.685 | 7.5 |
| the same + cache, λ = 0.9 | 58.5% | 80.9% | 0.685 | 7.0 |

## Identifiers only (names, not keywords or punctuation)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 0.0% | 0.0% | 0.011 | 2507.1 |
| bigram (discounted) | 2.6% | 14.3% | 0.095 | 530.9 |
| trigram (discounted) | 17.0% | 36.9% | 0.269 | 174.2 |
| 4-gram (discounted) | 27.8% | 50.4% | 0.382 | 103.6 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 17.0% | 36.5% | 0.269 | 108.1 |
| Phient movement, α = 1, K = 8 | 3.1% | 13.0% | 0.093 | 3741554571.6 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 10.8% | 36.3% | 0.221 | 3620.9 |
| linear mix: 0.95 · 4-gram + 0.05 · movement | 28.0% | 49.5% | 0.381 | 108.2 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 29.0% | 51.5% | 0.393 | 68.2 |
| the same + cache, λ = 0.9 | 28.8% | 51.0% | 0.391 | 51.4 |

## Size

Relations kept by the movement (posted (d, w) ⇒ next, with counts): 76214 across 39343 posted conditions, 1.0 per word per distance · n-gram entries: bigram 33619, trigram 76358, 4-gram 120837

## The learned weights α[d, sightings]

β (weight of the n-gram) = 0.69

| d | n<4 | <16 | <64 | <256 | <1024 | ≥1024 |
|---|---|---|---|---|---|---|
| 1 | 0.23 | 0.14 | 0.01 | 0.02 | 0.02 | 0.02 |
| 2 | 0.12 | 0.08 | 0.00 | 0.03 | 0.01 | 0.03 |
| 3 | 0.17 | 0.08 | 0.08 | 0.11 | 0.06 | 0.03 |
| 4 | 0.30 | 0.26 | 0.14 | 0.11 | 0.09 | 0.02 |
| 5 | 0.33 | 0.25 | 0.16 | 0.08 | 0.10 | 0.06 |
| 6 | 0.27 | 0.18 | 0.11 | 0.07 | 0.07 | 0.03 |
| 7 | 0.29 | 0.21 | 0.10 | 0.07 | 0.04 | 0.03 |
| 8 | 0.33 | 0.23 | 0.11 | 0.03 | 0.07 | 0.04 |

