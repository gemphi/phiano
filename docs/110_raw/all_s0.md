# Reading Rust as a sequence, calibrated

682 files: 543 train (of which 388 core + 155 dev), 139 test · 367484 train tokens · vocabulary 4837 · 22891 test positions, 6117 of them identifiers · weights fitted on dev in 3 epochs · base 4-gram · split 0 · relations per condition ≤ all

## All tokens

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 16.8% | 36.3% | 0.254 | 107.1 |
| bigram (discounted) | 32.3% | 63.1% | 0.459 | 25.2 |
| trigram (discounted) | 48.2% | 74.7% | 0.599 | 12.4 |
| 4-gram (discounted) | 55.7% | 78.3% | 0.658 | 10.0 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 48.3% | 74.9% | 0.600 | 10.5 |
| Phient movement, α = 1, K = 8 | 53.3% | 76.9% | 0.637 | 68.6 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 53.6% | 78.0% | 0.643 | 16.3 |
| linear mix: 0.8 · 4-gram + 0.20 · movement | 58.9% | 80.5% | 0.685 | 8.8 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 60.0% | 81.5% | 0.695 | 7.4 |
| the same + cache, λ = 0.95 | 60.0% | 81.6% | 0.695 | 6.8 |

## Identifiers only (names, not keywords or punctuation)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 0.0% | 0.0% | 0.011 | 2440.2 |
| bigram (discounted) | 2.3% | 13.5% | 0.091 | 581.1 |
| trigram (discounted) | 16.0% | 35.8% | 0.259 | 214.9 |
| 4-gram (discounted) | 25.1% | 46.6% | 0.354 | 150.9 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 16.0% | 36.0% | 0.259 | 119.6 |
| Phient movement, α = 1, K = 8 | 31.1% | 49.4% | 0.398 | 14198.9 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 25.8% | 47.2% | 0.358 | 549.7 |
| linear mix: 0.8 · 4-gram + 0.20 · movement | 31.1% | 52.1% | 0.410 | 120.3 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 34.0% | 54.1% | 0.434 | 64.5 |
| the same + cache, λ = 0.95 | 33.9% | 54.1% | 0.433 | 46.4 |

## Size

Relations kept by the movement (posted (d, w) ⇒ next, with counts): 341559 across 38641 posted conditions, 1.0 per word per distance · n-gram entries: bigram 32563, trigram 74093, 4-gram 117225

## The learned weights α[d, sightings]

β (weight of the n-gram) = 0.59

| d | n<4 | <16 | <64 | <256 | <1024 | ≥1024 |
|---|---|---|---|---|---|---|
| 1 | 0.32 | 0.20 | 0.19 | 0.12 | 0.01 | 0.01 |
| 2 | 0.13 | 0.19 | 0.08 | 0.05 | 0.02 | 0.03 |
| 3 | 0.22 | 0.20 | 0.15 | 0.11 | 0.09 | 0.05 |
| 4 | 0.36 | 0.32 | 0.29 | 0.24 | 0.15 | 0.09 |
| 5 | 0.36 | 0.32 | 0.28 | 0.18 | 0.17 | 0.13 |
| 6 | 0.24 | 0.28 | 0.25 | 0.20 | 0.15 | 0.05 |
| 7 | 0.21 | 0.20 | 0.22 | 0.16 | 0.15 | 0.09 |
| 8 | 0.30 | 0.20 | 0.21 | 0.16 | 0.15 | 0.10 |

