# The squares game (§48, §64), seed 8

Exact match: all nine squares right.

| learner | training sentences | plain names only | one composite, trained place | one composite, new place (§64) | two or more composites |
|---|---|---|---|---|---|
| lookup (whole sentence) | 50 | 0.0% | 0.0% | 0.0% | 0.0% |
| naming table (Augustine, §1) | 50 | 100.0% | 1.3% | 5.7% | 0.0% |
| positional softmax (word@position) | 50 | 16.3% | 3.0% | 0.7% | 0.0% |
| Phient reader (relations under the reader's place) | 50 | 100.0% | 100.0% | 100.0% | 100.0% |
| lookup (whole sentence) | 200 | 0.0% | 0.0% | 0.0% | 0.0% |
| naming table (Augustine, §1) | 200 | 100.0% | 1.3% | 5.7% | 0.0% |
| positional softmax (word@position) | 200 | 35.0% | 11.3% | 2.0% | 0.3% |
| Phient reader (relations under the reader's place) | 200 | 100.0% | 100.0% | 100.0% | 100.0% |
| lookup (whole sentence) | 1000 | 0.3% | 0.7% | 0.0% | 0.0% |
| naming table (Augustine, §1) | 1000 | 100.0% | 1.3% | 5.7% | 0.0% |
| positional softmax (word@position) | 1000 | 64.3% | 16.7% | 5.0% | 0.0% |
| Phient reader (relations under the reader's place) | 1000 | 100.0% | 100.0% | 100.0% | 100.0% |

## What each word came to relate to (Phient reader, 1,000 sentences)

| word | P(width 1 / 2 / 3) | colour at offset 0 | colour at offset 1 |
|---|---|---|---|
| `B` | 1.00 / 0.00 / 0.00 | B (1.00) | — |
| `G` | 1.00 / 0.00 / 0.00 | G (1.00) | — |
| `R` | 1.00 / 0.00 / 0.00 | R (1.00) | — |
| `U` | 0.00 / 0.99 / 0.00 | R (1.00) | G (1.00) |
| `V` | 0.00 / 0.99 / 0.00 | W (1.00) | B (1.00) |
| `W` | 1.00 / 0.00 / 0.00 | W (1.00) | — |
| `X` | 0.00 / 0.99 / 0.00 | G (1.00) | W (1.00) |
| `Y` | 0.00 / 0.99 / 0.00 | B (1.00) | R (1.00) |
