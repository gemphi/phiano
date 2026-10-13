# Reading Rust as a sequence, calibrated

682 files: 543 train (of which 388 core + 155 dev), 139 test · 367484 train tokens · vocabulary 4837 · 22891 test positions, 6117 of them identifiers · weights fitted on dev in 3 epochs · base 4-gram · split 0 · relations per condition ≤ 64

## All tokens

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 16.8% | 36.3% | 0.254 | 107.1 |
| bigram (discounted) | 32.3% | 63.1% | 0.459 | 25.2 |
| trigram (discounted) | 48.2% | 74.7% | 0.599 | 12.4 |
| 4-gram (discounted) | 55.7% | 78.3% | 0.658 | 10.0 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 48.3% | 74.9% | 0.600 | 10.5 |
| Phient movement, α = 1, K = 8 | 52.9% | 75.1% | 0.627 | 332.1 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 53.0% | 76.7% | 0.634 | 34.9 |
| linear mix: 0.8 · 4-gram + 0.20 · movement | 58.3% | 79.6% | 0.678 | 9.3 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 59.8% | 81.2% | 0.691 | 7.5 |
| the same + cache, λ = 0.95 | 59.8% | 81.1% | 0.692 | 6.9 |

## Identifiers only (names, not keywords or punctuation)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 0.0% | 0.0% | 0.011 | 2440.2 |
| bigram (discounted) | 2.3% | 13.5% | 0.091 | 581.1 |
| trigram (discounted) | 16.0% | 35.8% | 0.259 | 214.9 |
| 4-gram (discounted) | 25.1% | 46.6% | 0.354 | 150.9 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 16.0% | 36.0% | 0.259 | 119.6 |
| Phient movement, α = 1, K = 8 | 22.2% | 36.5% | 0.295 | 3869842.3 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 21.3% | 39.9% | 0.302 | 8055.7 |
| linear mix: 0.8 · 4-gram + 0.20 · movement | 27.8% | 47.9% | 0.374 | 151.8 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 31.8% | 52.2% | 0.412 | 70.3 |
| the same + cache, λ = 0.95 | 31.7% | 52.1% | 0.412 | 51.7 |

## Size

Relations kept by the movement (posted (d, w) ⇒ next, with counts): 245255 across 38641 posted conditions, 1.0 per word per distance · n-gram entries: bigram 32563, trigram 74093, 4-gram 117225

## The learned weights α[d, sightings]

β (weight of the n-gram) = 0.54

| d | n<4 | <16 | <64 | <256 | <1024 | ≥1024 |
|---|---|---|---|---|---|---|
| 1 | 0.35 | 0.22 | 0.22 | 0.17 | 0.01 | 0.03 |
| 2 | 0.17 | 0.24 | 0.13 | 0.10 | 0.04 | 0.06 |
| 3 | 0.23 | 0.25 | 0.19 | 0.13 | 0.11 | 0.03 |
| 4 | 0.37 | 0.34 | 0.29 | 0.23 | 0.14 | 0.07 |
| 5 | 0.34 | 0.32 | 0.28 | 0.18 | 0.15 | 0.10 |
| 6 | 0.25 | 0.28 | 0.25 | 0.19 | 0.14 | 0.04 |
| 7 | 0.21 | 0.18 | 0.23 | 0.17 | 0.10 | 0.07 |
| 8 | 0.29 | 0.19 | 0.20 | 0.16 | 0.12 | 0.05 |

