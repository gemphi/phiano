# Reading Rust as a sequence, calibrated

682 files: 543 train (of which 388 core + 155 dev), 139 test · 367484 train tokens · vocabulary 4837 · 22891 test positions, 6117 of them identifiers · weights fitted on dev in 3 epochs · base 4-gram · split 0 · relations per condition ≤ 2

## All tokens

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 16.8% | 36.3% | 0.254 | 107.1 |
| bigram (discounted) | 32.3% | 63.1% | 0.459 | 25.2 |
| trigram (discounted) | 48.2% | 74.7% | 0.599 | 12.4 |
| 4-gram (discounted) | 55.7% | 78.3% | 0.658 | 10.0 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 48.3% | 74.9% | 0.600 | 10.5 |
| Phient movement, α = 1, K = 8 | 34.2% | 60.8% | 0.461 | 174800.7 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 45.5% | 73.8% | 0.577 | 57.3 |
| linear mix: 0.95 · 4-gram + 0.05 · movement | 56.0% | 78.2% | 0.659 | 10.0 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 57.3% | 79.3% | 0.671 | 8.5 |
| the same + cache, λ = 0.95 | 57.3% | 79.4% | 0.671 | 7.7 |

## Identifiers only (names, not keywords or punctuation)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 0.0% | 0.0% | 0.011 | 2440.2 |
| bigram (discounted) | 2.3% | 13.5% | 0.091 | 581.1 |
| trigram (discounted) | 16.0% | 35.8% | 0.259 | 214.9 |
| 4-gram (discounted) | 25.1% | 46.6% | 0.354 | 150.9 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 16.0% | 36.0% | 0.259 | 119.6 |
| Phient movement, α = 1, K = 8 | 2.2% | 11.5% | 0.080 | 5652699773.9 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 9.7% | 33.5% | 0.207 | 4645.1 |
| linear mix: 0.95 · 4-gram + 0.05 · movement | 25.2% | 45.8% | 0.352 | 157.8 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 26.2% | 47.6% | 0.363 | 93.7 |
| the same + cache, λ = 0.95 | 26.2% | 47.7% | 0.364 | 67.6 |

## Size

Relations kept by the movement (posted (d, w) ⇒ next, with counts): 74753 across 38641 posted conditions, 1.0 per word per distance · n-gram entries: bigram 32563, trigram 74093, 4-gram 117225

## The learned weights α[d, sightings]

β (weight of the n-gram) = 0.71

| d | n<4 | <16 | <64 | <256 | <1024 | ≥1024 |
|---|---|---|---|---|---|---|
| 1 | 0.23 | 0.10 | 0.02 | 0.02 | 0.00 | 0.02 |
| 2 | 0.15 | 0.05 | 0.03 | 0.05 | 0.04 | 0.03 |
| 3 | 0.27 | 0.12 | 0.08 | 0.08 | 0.04 | 0.02 |
| 4 | 0.36 | 0.26 | 0.18 | 0.10 | 0.11 | 0.01 |
| 5 | 0.33 | 0.25 | 0.13 | 0.09 | 0.06 | 0.03 |
| 6 | 0.23 | 0.28 | 0.16 | 0.05 | 0.05 | 0.02 |
| 7 | 0.17 | 0.20 | 0.14 | 0.09 | 0.05 | 0.03 |
| 8 | 0.28 | 0.21 | 0.17 | 0.03 | 0.04 | 0.12 |

