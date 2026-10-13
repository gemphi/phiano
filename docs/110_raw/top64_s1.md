# Reading Rust as a sequence, calibrated

682 files: 557 train (of which 412 core + 145 dev), 125 test · 373196 train tokens · vocabulary 4925 · 21491 test positions, 5691 of them identifiers · weights fitted on dev in 3 epochs · base 4-gram · split 1 · relations per condition ≤ 64

## All tokens

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 18.4% | 37.6% | 0.268 | 103.9 |
| bigram (discounted) | 33.7% | 64.1% | 0.472 | 23.6 |
| trigram (discounted) | 49.4% | 76.0% | 0.610 | 11.2 |
| 4-gram (discounted) | 57.2% | 80.3% | 0.673 | 8.6 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 49.4% | 76.1% | 0.611 | 9.8 |
| Phient movement, α = 1, K = 8 | 54.5% | 76.0% | 0.641 | 245.8 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 54.0% | 77.2% | 0.644 | 29.9 |
| linear mix: 0.8 · 4-gram + 0.20 · movement | 59.5% | 81.4% | 0.692 | 8.0 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 61.5% | 82.7% | 0.708 | 6.6 |
| the same + cache, λ = 0.95 | 61.5% | 82.7% | 0.708 | 6.2 |

## Identifiers only (names, not keywords or punctuation)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 0.0% | 0.0% | 0.011 | 2507.1 |
| bigram (discounted) | 2.6% | 14.3% | 0.095 | 530.9 |
| trigram (discounted) | 17.0% | 36.9% | 0.269 | 174.2 |
| 4-gram (discounted) | 27.8% | 50.4% | 0.382 | 103.6 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 17.0% | 36.5% | 0.269 | 108.1 |
| Phient movement, α = 1, K = 8 | 25.6% | 39.3% | 0.325 | 1953520.1 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 22.6% | 42.1% | 0.318 | 5455.3 |
| linear mix: 0.8 · 4-gram + 0.20 · movement | 30.8% | 51.9% | 0.407 | 102.4 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 36.1% | 55.7% | 0.452 | 50.1 |
| the same + cache, λ = 0.95 | 36.0% | 55.4% | 0.452 | 39.5 |

## Size

Relations kept by the movement (posted (d, w) ⇒ next, with counts): 250870 across 39343 posted conditions, 1.0 per word per distance · n-gram entries: bigram 33619, trigram 76358, 4-gram 120837

## The learned weights α[d, sightings]

β (weight of the n-gram) = 0.53

| d | n<4 | <16 | <64 | <256 | <1024 | ≥1024 |
|---|---|---|---|---|---|---|
| 1 | 0.32 | 0.30 | 0.16 | 0.14 | 0.04 | 0.02 |
| 2 | 0.22 | 0.21 | 0.17 | 0.12 | 0.03 | 0.06 |
| 3 | 0.30 | 0.20 | 0.19 | 0.13 | 0.13 | 0.04 |
| 4 | 0.32 | 0.26 | 0.24 | 0.23 | 0.16 | 0.07 |
| 5 | 0.30 | 0.27 | 0.23 | 0.19 | 0.15 | 0.11 |
| 6 | 0.38 | 0.20 | 0.28 | 0.15 | 0.12 | 0.06 |
| 7 | 0.35 | 0.25 | 0.20 | 0.17 | 0.12 | 0.05 |
| 8 | 0.34 | 0.25 | 0.24 | 0.14 | 0.11 | 0.08 |

