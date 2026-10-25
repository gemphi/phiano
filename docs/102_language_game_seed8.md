# Language in action — the builders' game (seed 8, R = 32, 30 epochs)

3600 training episodes (3000 orders, 600 reports). Exact match: all five aspects of the action right.

Phient: 1593 relations kept from 3600 episodes · 22160 numbers · 97 s.

| learner | seen | new combinations (§8) | report, new stones (§21) | no name (§27) |
|---|---|---|---|---|
| lookup (whole utterance) | 86.8% | 0.0% | 1.3% | 100.0% |
| naming table (Augustine, §1) | 100.0% | 100.0% | 0.0% | 0.0% |
| bag of words + softmax | 100.0% | 100.0% | 100.0% | 100.0% |
| Phical counts, summed (no operator) | 85.0% | 71.3% | 60.3% | 100.0% |
| Phical counts, movement (no operator) | 100.0% | 100.0% | 100.0% | 100.0% |
| Phient relations, summed (phinet, R=32) | 40.2% | 13.7% | 10.3% | 100.0% |
| Phient relations, movement (phinet, R=32) | 100.0% | 100.0% | 100.0% | 100.0% |

## Sepia (§28–30): a new colour-word, met only by pointing, then used in orders

| learner | 1 pointing | 3 pointings | 1 pointing + `colour` | 3 pointings + `colour` |
|---|---|---|---|---|
| lookup (whole utterance) | 0.0% | 0.0% | 0.0% | 0.0% |
| naming table (Augustine, §1) | 0.0% | 100.0% | 0.0% | 100.0% |
| bag of words + softmax | 0.0% | 0.0% | 0.0% | 0.0% |
| Phical counts, summed (no operator) | 0.0% | 47.0% | 18.3% | 47.0% |
| Phical counts, movement (no operator) | 21.0% | 39.7% | 100.0% | 100.0% |
| Phient relations, summed (phinet, R=32) | 0.0% | 0.0% | 0.0% | 0.0% |
| Phient relations, movement (phinet, R=32) | 5.3% | 2.0% | 0.0% | 52.0% |

## The toolbox (§11–17): where each word holds a post (order game, Phical counts)

| word | holds a post under | moves the answer to |
|---|---|---|
| `a` | count | count = 1 |
| `away` | act, what, colour, count, place | act = clear, what = -, colour = -, count = -, place = - |
| `b` | count | count = 2 |
| `beam` | what | what = beam |
| `block` | what | what = block |
| `blue` | colour | colour = blue |
| `c` | count | count = 3 |
| `d` | count | count = 4 |
| `e` | count | count = 5 |
| `f` | count | count = 6 |
| `g` | count | count = 7 |
| `h` | count | count = 8 |
| `help` | act, what, colour, count, place | act = come, what = -, colour = -, count = -, place = - |
| `i` | count | count = 9 |
| `j` | count | count = 10 |
| `no` | act, what, colour, count, place | act = undo, what = -, colour = -, count = -, place = - |
| `pillar` | what | what = pillar |
| `red` | colour | colour = red |
| `slab` | what | what = slab |
| `there` | place | place = p2 |
| `yellow` | colour | colour = yellow |
| `⟨point:beam⟩` | what | what = beam |
| `⟨point:block⟩` | what | what = block |
| `⟨point:p1⟩` | place | place = p1 |
| `⟨point:p2⟩` | place | place = p2 |
| `⟨point:p3⟩` | place | place = p3 |
| `⟨point:p4⟩` | place | place = p4 |
| `⟨point:p5⟩` | place | place = p5 |
| `⟨point:pillar⟩` | what | what = pillar |
| `⟨point:slab⟩` | what | what = slab |

## Training (§5–6): exact match on all four test sets, by episodes seen

| learner | 20 episodes | 40 episodes | 80 episodes | 160 episodes | 320 episodes | 640 episodes |
|---|---|---|---|---|---|---|
| lookup (whole utterance) | 14.6% | 17.5% | 20.7% | 23.8% | 29.6% | 35.6% |
| naming table (Augustine, §1) | 12.5% | 27.0% | 58.6% | 61.9% | 64.0% | 64.0% |
| bag of words + softmax | 33.0% | 50.1% | 77.4% | 94.7% | 100.0% | 100.0% |
| Phical counts, movement (no operator) | 13.3% | 33.8% | 55.4% | 70.4% | 92.7% | 97.4% |
| Phient relations, movement (phinet, R=8) | 2.2% | 23.0% | 28.6% | 61.6% | 66.3% | 87.7% |
| Phient relations, movement (phinet, R=32) | 4.2% | 32.2% | 32.6% | 65.4% | 53.8% | 89.2% |

## The stranger (§32): new words for a game already played — exact match on new-vocabulary orders

| learner | child, 10 | stranger, 10 | child, 20 | stranger, 20 | child, 40 | stranger, 40 | child, 80 | stranger, 80 |
|---|---|---|---|---|---|---|---|---|
| lookup (whole utterance) | 1.2% | 1.2% | 3.9% | 3.9% | 10.1% | 10.1% | 14.2% | 14.2% |
| naming table (Augustine, §1) | 8.2% | 13.9% | 30.2% | 39.6% | 58.1% | 69.2% | 82.4% | 89.8% |
| bag of words + softmax | 3.2% | 13.9% | 25.9% | 34.8% | 41.1% | 48.4% | 71.5% | 80.1% |
| Phical counts, movement (no operator) | 1.5% | 5.2% | 11.6% | 22.2% | 50.0% | 73.1% | 67.9% | 77.4% |
| Phient relations, movement (phinet, R=8) | 4.5% | 3.4% | 2.4% | 9.1% | 50.0% | 51.2% | 16.4% | 63.0% |
| Phient relations, movement (phinet, R=32) | 1.8% | 1.2% | 8.5% | 17.5% | 50.9% | 51.1% | 43.1% | 73.4% |
