# Reading Rust as a sequence, calibrated

682 files: 543 train (of which 388 core + 155 dev), 139 test · 367484 train tokens · vocabulary 4837 · 22891 test positions, 6117 of them identifiers · weights fitted on dev in 3 epochs · base 4-gram · split 0 · relations per condition ≤ 16

## All tokens

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 16.8% | 36.3% | 0.254 | 107.1 |
| bigram (discounted) | 32.3% | 63.1% | 0.459 | 25.2 |
| trigram (discounted) | 48.2% | 74.7% | 0.599 | 12.4 |
| 4-gram (discounted) | 55.7% | 78.3% | 0.658 | 10.0 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 48.3% | 74.9% | 0.600 | 10.5 |
| Phient movement, α = 1, K = 8 | 48.2% | 68.6% | 0.576 | 4802.6 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 51.6% | 74.2% | 0.617 | 65.9 |
| linear mix: 0.9 · 4-gram + 0.10 · movement | 56.6% | 78.8% | 0.666 | 9.7 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 58.7% | 80.5% | 0.683 | 7.7 |
| the same + cache, λ = 0.95 | 58.6% | 80.4% | 0.682 | 7.2 |

## Identifiers only (names, not keywords or punctuation)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 0.0% | 0.0% | 0.011 | 2440.2 |
| bigram (discounted) | 2.3% | 13.5% | 0.091 | 581.1 |
| trigram (discounted) | 16.0% | 35.8% | 0.259 | 214.9 |
| 4-gram (discounted) | 25.1% | 46.6% | 0.354 | 150.9 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 16.0% | 36.0% | 0.259 | 119.6 |
| Phient movement, α = 1, K = 8 | 9.3% | 19.7% | 0.156 | 1552369162.6 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 14.7% | 30.9% | 0.231 | 41708.8 |
| linear mix: 0.9 · 4-gram + 0.10 · movement | 24.9% | 45.6% | 0.348 | 161.6 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 28.7% | 50.2% | 0.388 | 78.6 |
| the same + cache, λ = 0.95 | 28.6% | 50.0% | 0.387 | 59.0 |

## Size

Relations kept by the movement (posted (d, w) ⇒ next, with counts): 187728 across 38641 posted conditions, 1.0 per word per distance · n-gram entries: bigram 32563, trigram 74093, 4-gram 117225

## The learned weights α[d, sightings]

β (weight of the n-gram) = 0.60

| d | n<4 | <16 | <64 | <256 | <1024 | ≥1024 |
|---|---|---|---|---|---|---|
| 1 | 0.32 | 0.19 | 0.18 | 0.11 | 0.01 | 0.02 |
| 2 | 0.15 | 0.22 | 0.11 | 0.06 | 0.04 | 0.03 |
| 3 | 0.20 | 0.21 | 0.17 | 0.11 | 0.08 | 0.03 |
| 4 | 0.41 | 0.35 | 0.25 | 0.20 | 0.11 | 0.03 |
| 5 | 0.34 | 0.34 | 0.28 | 0.19 | 0.11 | 0.06 |
| 6 | 0.26 | 0.30 | 0.23 | 0.14 | 0.08 | 0.04 |
| 7 | 0.24 | 0.18 | 0.24 | 0.14 | 0.08 | 0.03 |
| 8 | 0.29 | 0.22 | 0.18 | 0.12 | 0.08 | 0.03 |

