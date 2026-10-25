# Reading Rust as a sequence, calibrated

682 files: 557 train (of which 412 core + 145 dev), 125 test · 373196 train tokens · vocabulary 4925 · 21491 test positions, 5691 of them identifiers · weights fitted on dev in 3 epochs · base 4-gram · split 1 · relations per condition ≤ 4

## All tokens

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 18.4% | 37.6% | 0.268 | 103.9 |
| bigram (discounted) | 33.7% | 64.1% | 0.472 | 23.6 |
| trigram (discounted) | 49.4% | 76.0% | 0.610 | 11.2 |
| 4-gram (discounted) | 57.2% | 80.3% | 0.673 | 8.6 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 49.4% | 76.1% | 0.611 | 9.8 |
| Phient movement, α = 1, K = 8 | 38.5% | 64.9% | 0.502 | 73994.3 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 48.2% | 73.7% | 0.597 | 64.7 |
| linear mix: 0.95 · 4-gram + 0.05 · movement | 57.5% | 80.0% | 0.675 | 8.6 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 59.8% | 81.5% | 0.693 | 7.3 |
| the same + cache, λ = 0.9 | 59.8% | 81.2% | 0.693 | 6.8 |

## Identifiers only (names, not keywords or punctuation)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 0.0% | 0.0% | 0.011 | 2507.1 |
| bigram (discounted) | 2.6% | 14.3% | 0.095 | 530.9 |
| trigram (discounted) | 17.0% | 36.9% | 0.269 | 174.2 |
| 4-gram (discounted) | 27.8% | 50.4% | 0.382 | 103.6 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 17.0% | 36.5% | 0.269 | 108.1 |
| Phient movement, α = 1, K = 8 | 4.4% | 16.8% | 0.114 | 6199632338.0 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 12.0% | 30.9% | 0.221 | 10603.7 |
| linear mix: 0.95 · 4-gram + 0.05 · movement | 27.7% | 49.6% | 0.380 | 108.1 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 30.0% | 52.1% | 0.402 | 65.2 |
| the same + cache, λ = 0.9 | 29.9% | 51.5% | 0.401 | 49.5 |

## Size

Relations kept by the movement (posted (d, w) ⇒ next, with counts): 118502 across 39343 posted conditions, 1.0 per word per distance · n-gram entries: bigram 33619, trigram 76358, 4-gram 120837

## The learned weights α[d, sightings]

β (weight of the n-gram) = 0.67

| d | n<4 | <16 | <64 | <256 | <1024 | ≥1024 |
|---|---|---|---|---|---|---|
| 1 | 0.30 | 0.19 | 0.05 | 0.01 | 0.01 | 0.02 |
| 2 | 0.16 | 0.13 | 0.04 | 0.02 | 0.03 | 0.04 |
| 3 | 0.26 | 0.12 | 0.09 | 0.14 | 0.06 | 0.02 |
| 4 | 0.32 | 0.26 | 0.22 | 0.14 | 0.06 | 0.06 |
| 5 | 0.33 | 0.25 | 0.17 | 0.11 | 0.05 | 0.10 |
| 6 | 0.43 | 0.22 | 0.18 | 0.06 | 0.07 | 0.06 |
| 7 | 0.37 | 0.24 | 0.12 | 0.07 | 0.01 | 0.04 |
| 8 | 0.37 | 0.21 | 0.18 | 0.07 | 0.02 | 0.03 |

