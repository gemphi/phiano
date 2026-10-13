# Reading Rust as a sequence, calibrated

682 files: 557 train (of which 412 core + 145 dev), 125 test · 373196 train tokens · vocabulary 4925 · 21491 test positions, 5691 of them identifiers · weights fitted on dev in 3 epochs · base 4-gram · split 1 · relations per condition ≤ 8

## All tokens

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 18.4% | 37.6% | 0.268 | 103.9 |
| bigram (discounted) | 33.7% | 64.1% | 0.472 | 23.6 |
| trigram (discounted) | 49.4% | 76.0% | 0.610 | 11.2 |
| 4-gram (discounted) | 57.2% | 80.3% | 0.673 | 8.6 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 49.4% | 76.1% | 0.611 | 9.8 |
| Phient movement, α = 1, K = 8 | 44.3% | 65.4% | 0.543 | 21141.2 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 50.7% | 73.8% | 0.612 | 70.3 |
| linear mix: 0.95 · 4-gram + 0.05 · movement | 57.6% | 80.4% | 0.676 | 8.5 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 59.8% | 81.8% | 0.695 | 7.0 |
| the same + cache, λ = 0.9 | 59.8% | 81.5% | 0.695 | 6.6 |

## Identifiers only (names, not keywords or punctuation)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 0.0% | 0.0% | 0.011 | 2507.1 |
| bigram (discounted) | 2.6% | 14.3% | 0.095 | 530.9 |
| trigram (discounted) | 17.0% | 36.9% | 0.269 | 174.2 |
| 4-gram (discounted) | 27.8% | 50.4% | 0.382 | 103.6 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 17.0% | 36.5% | 0.269 | 108.1 |
| Phient movement, α = 1, K = 8 | 6.4% | 17.6% | 0.133 | 4756573185.4 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 12.7% | 31.0% | 0.223 | 25176.3 |
| linear mix: 0.95 · 4-gram + 0.05 · movement | 27.8% | 49.7% | 0.380 | 107.7 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 31.4% | 53.5% | 0.415 | 60.0 |
| the same + cache, λ = 0.9 | 31.3% | 52.7% | 0.412 | 46.3 |

## Size

Relations kept by the movement (posted (d, w) ⇒ next, with counts): 157548 across 39343 posted conditions, 1.0 per word per distance · n-gram entries: bigram 33619, trigram 76358, 4-gram 120837

## The learned weights α[d, sightings]

β (weight of the n-gram) = 0.63

| d | n<4 | <16 | <64 | <256 | <1024 | ≥1024 |
|---|---|---|---|---|---|---|
| 1 | 0.32 | 0.26 | 0.11 | 0.06 | 0.02 | 0.01 |
| 2 | 0.17 | 0.15 | 0.08 | 0.04 | 0.02 | 0.03 |
| 3 | 0.26 | 0.17 | 0.12 | 0.12 | 0.08 | 0.02 |
| 4 | 0.34 | 0.22 | 0.26 | 0.17 | 0.08 | 0.03 |
| 5 | 0.32 | 0.31 | 0.23 | 0.15 | 0.08 | 0.11 |
| 6 | 0.41 | 0.23 | 0.21 | 0.11 | 0.05 | 0.05 |
| 7 | 0.36 | 0.23 | 0.17 | 0.11 | 0.06 | 0.03 |
| 8 | 0.38 | 0.27 | 0.18 | 0.13 | 0.04 | 0.03 |

