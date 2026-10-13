# Reading Rust as a sequence, calibrated

682 files: 557 train (of which 412 core + 145 dev), 125 test · 373196 train tokens · vocabulary 4925 · 21491 test positions, 5691 of them identifiers · weights fitted on dev in 3 epochs · base 4-gram · split 1 · relations per condition ≤ 16

## All tokens

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 18.4% | 37.6% | 0.268 | 103.9 |
| bigram (discounted) | 33.7% | 64.1% | 0.472 | 23.6 |
| trigram (discounted) | 49.4% | 76.0% | 0.610 | 11.2 |
| 4-gram (discounted) | 57.2% | 80.3% | 0.673 | 8.6 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 49.4% | 76.1% | 0.611 | 9.8 |
| Phient movement, α = 1, K = 8 | 49.3% | 70.1% | 0.589 | 3249.2 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 52.5% | 75.2% | 0.628 | 54.9 |
| linear mix: 0.9 · 4-gram + 0.10 · movement | 57.9% | 80.5% | 0.679 | 8.4 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 60.4% | 82.1% | 0.700 | 6.8 |
| the same + cache, λ = 0.9 | 60.4% | 82.0% | 0.699 | 6.4 |

## Identifiers only (names, not keywords or punctuation)

| reader | top-1 | top-5 | MRR | perplexity |
|---|---|---|---|---|
| unigram (frequency) | 0.0% | 0.0% | 0.011 | 2507.1 |
| bigram (discounted) | 2.6% | 14.3% | 0.095 | 530.9 |
| trigram (discounted) | 17.0% | 36.9% | 0.269 | 174.2 |
| 4-gram (discounted) | 27.8% | 50.4% | 0.382 | 103.6 |
| trigram + cache of the file (Tu et al. 2014), λ = 0.9 | 17.0% | 36.5% | 0.269 | 108.1 |
| Phient movement, α = 1, K = 8 | 10.8% | 22.3% | 0.175 | 827747272.8 |
| trigram ⊕ movement, α = 0.35, K = 4 (docs/109) | 15.9% | 32.3% | 0.244 | 28464.0 |
| linear mix: 0.9 · 4-gram + 0.10 · movement | 27.8% | 49.2% | 0.377 | 110.7 |
| 4-gram ⊕ movement, weights learned per (d, sightings), K = 8 | 32.8% | 53.9% | 0.425 | 56.9 |
| the same + cache, λ = 0.9 | 32.7% | 53.2% | 0.423 | 44.0 |

## Size

Relations kept by the movement (posted (d, w) ⇒ next, with counts): 192495 across 39343 posted conditions, 1.0 per word per distance · n-gram entries: bigram 33619, trigram 76358, 4-gram 120837

## The learned weights α[d, sightings]

β (weight of the n-gram) = 0.58

| d | n<4 | <16 | <64 | <256 | <1024 | ≥1024 |
|---|---|---|---|---|---|---|
| 1 | 0.34 | 0.28 | 0.15 | 0.10 | 0.02 | 0.02 |
| 2 | 0.22 | 0.19 | 0.14 | 0.05 | 0.05 | 0.04 |
| 3 | 0.27 | 0.18 | 0.17 | 0.13 | 0.12 | 0.03 |
| 4 | 0.34 | 0.27 | 0.26 | 0.19 | 0.11 | 0.03 |
| 5 | 0.31 | 0.26 | 0.24 | 0.19 | 0.12 | 0.08 |
| 6 | 0.39 | 0.22 | 0.28 | 0.13 | 0.08 | 0.04 |
| 7 | 0.36 | 0.25 | 0.18 | 0.12 | 0.06 | 0.01 |
| 8 | 0.37 | 0.26 | 0.23 | 0.16 | 0.06 | 0.04 |

