### How strongly must a word move an answer to be heard? (τ swept, students of Phi-4, 5 folds by call)

| student | exact | exact, calls with a word heard in training | act | what | colour | count | place |
|---|---|---|---|---|---|---|---|
| bag of words + softmax | 70.5% | 76.5% | 77.6% | 97.7% | 92.1% | 85.2% | 95.7% |
| Phical counts, movement, τ = 0 | 44.0% | 47.8% | 72.5% | 89.1% | 82.2% | 67.4% | 84.0% |
| Phical counts, movement, τ = 0.02 | 44.0% | 47.8% | 72.3% | 89.1% | 82.7% | 67.4% | 84.0% |
| Phical counts, movement, τ = 0.05 | 44.0% | 47.8% | 72.3% | 88.8% | 83.5% | 67.4% | 84.0% |
| Phical counts, movement, τ = 0.1 | 46.1% | 50.0% | 72.8% | 88.8% | 85.0% | 67.2% | 86.0% |
| Phical counts, movement, τ = 0.25 | 49.9% | 54.1% | 67.7% | 87.5% | 88.5% | 75.1% | 89.6% |
| Phical counts, movement, τ = 0.5 | 60.3% | 65.5% | 66.4% | 89.1% | 91.6% | 85.2% | 93.9% |
| Phical counts, movement, τ = 1 | 54.5% | 59.1% | 66.4% | 78.1% | 92.1% | 87.3% | 95.7% |
| Phical counts, movement, τ per aspect (chosen in the training folds) | 65.1% | 70.7% | 70.2% | 88.8% | 92.1% | 87.3% | 95.7% |
| Phical counts, movement, weights learned per (aspect, post strength) (docs/112) | 66.4% | 72.1% | 73.8% | 88.5% | 91.1% | 87.3% | 95.7% |

393 calls, 362 with a word heard in training.

τ chosen per aspect (act, what, colour, count, place), fold by fold: [0.0, 0.5, 1.0, 1.0, 1.0] · [0.25, 0.5, 1.0, 1.0, 1.0] · [0.0, 0.25, 1.0, 1.0, 1.0] · [0.0, 0.0, 1.0, 1.0, 1.0] · [0.05, 0.0, 1.0, 1.0, 1.0]

Learned weights (last fold):

| aspect | β | α: <0.05 nats | <0.25 | <0.5 | <1 | ≥1 |
|---|---|---|---|---|---|---|
| act | 0.40 | 0.40 | 0.97 | 0.72 | 0.03 | 1.00 |
| what | 0.67 | 0.51 | 0.69 | 0.66 | 0.99 | 1.31 |
| colour | 1.92 | 0.86 | 0.21 | 0.38 | 0.31 | 2.82 |
| count | 1.59 | 1.00 | 0.07 | 0.33 | 0.30 | 1.68 |
| place | 1.87 | 0.46 | 0.34 | 0.02 | 0.13 | 2.20 |
