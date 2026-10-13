## Phi-4 in the builders' game: 400 calls, 369 replies read

### How far it goes on as our rules go on

| reading | exact | act | what | colour | count | place |
|---|---|---|---|---|---|---|
| its own notation | 28.7% | 63.1% | 94.9% | 66.7% | 76.4% | 69.9% |
| notation read in the game's terms | 50.7% | 63.1% | 94.9% | 96.2% | 89.7% | 89.7% |

### Where it goes on differently (game's terms): the commonest divergences

| aspect | rule ⇒ Phi-4 | calls |
|---|---|---|
| act | bring ⇒ record | 85 |
| act | record ⇒ bring | 25 |
| act | undo ⇒ clear | 2 |
| what | block ⇒ yellow block | 6 |
| what | pillar ⇒ yellow pillar | 3 |
| what | - ⇒ what | 2 |
| colour | - ⇒ any | 4 |
| colour | any ⇒ - | 4 |
| colour | - ⇒ colour | 2 |
| count | 9 ⇒ 10 | 9 |
| count | 1 ⇒ - | 4 |
| count | 8 ⇒ 1 | 4 |
| place | - ⇒ here | 9 |
| place | here ⇒ p1 | 8 |
| place | here ⇒ - | 4 |

### Does it go on the same way twice?

Calls asked more than once: 60. Pairs of answers to the same call that are identical (game's terms): 95 of 265 (35.8%).

### Can a student learn to go on as Phi-4 does? (5 folds by call)

| predictor of Phi-4's action | exact | act | what | colour | count | place |
|---|---|---|---|---|---|---|
| our rules (the game as we wrote it) | 50.7% | 63.1% | 94.9% | 96.2% | 89.7% | 89.7% |
| lookup (whole utterance) | 0.5% | 53.1% | 25.7% | 49.3% | 47.4% | 53.1% |
| naming table (Augustine, §1) | 46.1% | 55.3% | 94.9% | 92.1% | 84.3% | 90.8% |
| bag of words + softmax | 54.2% | 67.5% | 93.5% | 91.6% | 82.9% | 90.8% |
| Phical counts, movement (no operator) | 46.6% | 61.2% | 87.8% | 91.6% | 81.3% | 88.6% |
| Phient relations, movement (phinet, R=32) | 45.0% | 60.4% | 87.8% | 91.1% | 77.0% | 86.2% |
| Phient two-speed: counts until 8 sightings (phinet, R=32) | 43.6% | 61.2% | 87.5% | 91.1% | 78.6% | 85.4% |
| Phient two-speed: counts until 32 sightings (phinet, R=32) | 46.9% | 61.2% | 88.1% | 91.6% | 81.3% | 88.6% |
| Phient two-speed: counts until the function agrees (phinet, R=32) | 45.0% | 61.2% | 87.8% | 90.5% | 78.9% | 88.3% |
