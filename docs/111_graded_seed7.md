### How strongly must a word move an answer to be heard? (τ swept, students of Phi-4, 5 folds by call)

| student | exact | exact, calls with a word heard in training | act | what | colour | count | place |
|---|---|---|---|---|---|---|---|
| bag of words + softmax | 54.2% | 58.1% | 67.5% | 93.5% | 91.6% | 82.9% | 90.8% |
| Phical counts, movement, τ = 0 | 31.7% | 33.7% | 62.3% | 88.9% | 82.7% | 65.0% | 76.2% |
| Phical counts, movement, τ = 0.02 | 32.0% | 34.0% | 62.9% | 88.9% | 82.7% | 65.0% | 76.2% |
| Phical counts, movement, τ = 0.05 | 31.2% | 33.1% | 61.8% | 88.9% | 82.4% | 65.0% | 76.2% |
| Phical counts, movement, τ = 0.1 | 33.3% | 35.5% | 62.9% | 88.6% | 83.5% | 65.3% | 77.2% |
| Phical counts, movement, τ = 0.25 | 40.4% | 43.0% | 63.1% | 88.6% | 88.3% | 71.5% | 85.1% |
| Phical counts, movement, τ = 0.5 | 46.6% | 49.7% | 61.2% | 87.8% | 91.6% | 81.3% | 88.6% |
| Phical counts, movement, τ = 1 | 41.2% | 43.9% | 60.4% | 76.7% | 92.1% | 83.2% | 89.7% |
| Phical counts, movement, τ per aspect (chosen in the training folds) | 49.3% | 52.6% | 62.9% | 88.3% | 91.9% | 83.2% | 89.7% |

369 calls, 344 with a word heard in training.

τ chosen per aspect (act, what, colour, count, place), fold by fold: [0.05, 0.5, 1.0, 1.0, 1.0] · [0.25, 0.0, 1.0, 1.0, 1.0] · [0.02, 0.1, 1.0, 1.0, 1.0] · [0.1, 0.25, 0.5, 1.0, 1.0] · [0.1, 0.5, 1.0, 1.0, 1.0]
