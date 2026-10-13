# Language in action — the builders' game (seed 7, R = 32, 30 epochs)

3600 training episodes (3000 orders, 600 reports). Exact match: all five aspects of the action right.

Phient: 1585 relations kept from 3600 episodes · 22160 numbers · functions agree with counts on 90.6% · 103 s.

| learner | seen | new combinations (§8) | report, new stones (§21) | no name (§27) |
|---|---|---|---|---|
| lookup (whole utterance) | 87.6% | 0.0% | 0.7% | 100.0% |
| naming table (Augustine, §1) | 100.0% | 100.0% | 0.0% | 0.0% |
| bag of words + softmax | 100.0% | 100.0% | 100.0% | 100.0% |
| Phical counts, summed (no operator) | 87.4% | 63.3% | 59.7% | 100.0% |
| Phical counts, movement (no operator) | 100.0% | 100.0% | 100.0% | 100.0% |
| Phient relations, summed (phinet, R=32) | 56.4% | 0.0% | 7.3% | 100.0% |
| Phient relations, movement (phinet, R=32) | 100.0% | 100.0% | 100.0% | 100.0% |
| Phient two-speed: counts until 8 sightings (phinet, R=32) | 100.0% | 100.0% | 100.0% | 100.0% |
| Phient two-speed: counts until the function agrees (phinet, R=32) | 100.0% | 100.0% | 100.0% | 100.0% |

## Sepia (§28–30): a new colour-word, met only by pointing, then used in orders

| learner | 1 pointing | 3 pointings | 1 pointing + `colour` | 3 pointings + `colour` |
|---|---|---|---|---|
| lookup (whole utterance) | 0.0% | 0.0% | 0.0% | 0.0% |
| naming table (Augustine, §1) | 0.0% | 100.0% | 0.0% | 100.0% |
| bag of words + softmax | 0.0% | 0.0% | 0.0% | 0.0% |
| Phical counts, summed (no operator) | 0.0% | 23.7% | 0.0% | 23.7% |
| Phical counts, movement (no operator) | 26.3% | 26.3% | 100.0% | 100.0% |
| Phient relations, summed (phinet, R=32) | 0.0% | 0.0% | 0.0% | 12.0% |
| Phient relations, movement (phinet, R=32) | 0.3% | 4.3% | 100.0% | 100.0% |
| Phient two-speed: counts until 8 sightings (phinet, R=32) | 2.7% | 11.7% | 100.0% | 100.0% |
| Phient two-speed: counts until the function agrees (phinet, R=32) | 1.7% | 4.3% | 100.0% | 100.0% |

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
| `there` | place | place = p3 |
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
| lookup (whole utterance) | 11.8% | 18.2% | 21.9% | 25.8% | 30.7% | 35.6% |
| naming table (Augustine, §1) | 23.0% | 34.4% | 60.9% | 64.0% | 64.0% | 64.0% |
| bag of words + softmax | 26.2% | 51.8% | 82.7% | 99.8% | 100.0% | 100.0% |
| Phical counts, movement (no operator) | 19.4% | 37.9% | 50.8% | 69.8% | 96.6% | 98.7% |
| Phient relations, movement (phinet, R=8) | 15.2% | 29.0% | 44.4% | 44.2% | 82.4% | 94.1% |
| Phient two-speed: counts until 8 sightings (phinet, R=8) | 19.4% | 37.1% | 52.6% | 53.1% | 85.6% | 94.4% |
| Phient two-speed: counts until 32 sightings (phinet, R=8) | 19.4% | 37.9% | 50.8% | 69.8% | 87.0% | 94.6% |
| Phient two-speed: counts until the function agrees (phinet, R=8) | 21.3% | 38.6% | 41.0% | 53.4% | 85.6% | 97.9% |
| Phient relations, movement (phinet, R=32) | 18.1% | 34.1% | 43.0% | 54.5% | 77.9% | 99.4% |
| Phient two-speed: counts until 8 sightings (phinet, R=32) | 19.4% | 36.5% | 51.5% | 56.8% | 79.4% | 99.0% |
| Phient two-speed: counts until 32 sightings (phinet, R=32) | 19.4% | 37.9% | 50.8% | 69.8% | 87.0% | 98.4% |
| Phient two-speed: counts until the function agrees (phinet, R=32) | 18.7% | 34.3% | 43.0% | 57.9% | 86.8% | 98.5% |

## The stranger (§32): new words for a game already played — exact match on new-vocabulary orders

| learner | child, 10 | stranger, 10 | child, 20 | stranger, 20 | child, 40 | stranger, 40 | child, 80 | stranger, 80 |
|---|---|---|---|---|---|---|---|---|
| lookup (whole utterance) | 1.5% | 1.5% | 2.8% | 2.8% | 5.2% | 5.2% | 17.9% | 17.9% |
| naming table (Augustine, §1) | 18.1% | 35.9% | 29.2% | 43.1% | 40.2% | 58.6% | 57.8% | 68.8% |
| bag of words + softmax | 13.0% | 28.6% | 21.2% | 39.1% | 45.4% | 56.9% | 55.5% | 67.6% |
| Phical counts, movement (no operator) | 23.1% | 12.2% | 26.8% | 28.9% | 47.1% | 66.5% | 65.1% | 80.5% |
| Phient relations, movement (phinet, R=8) | 12.4% | 9.5% | 22.1% | 15.9% | 45.2% | 43.9% | 44.2% | 57.8% |
| Phient two-speed: counts until 8 sightings (phinet, R=8) | 23.1% | 12.2% | 26.8% | 22.0% | 41.9% | 53.0% | 61.3% | 67.9% |
| Phient two-speed: counts until 32 sightings (phinet, R=8) | 23.1% | 12.2% | 26.8% | 22.0% | 47.1% | 53.2% | 65.1% | 80.5% |
| Phient two-speed: counts until the function agrees (phinet, R=8) | 23.1% | 10.1% | 24.2% | 15.1% | 49.0% | 53.5% | 60.9% | 73.8% |
| Phient relations, movement (phinet, R=32) | 24.1% | 10.1% | 25.5% | 25.1% | 51.2% | 47.6% | 59.8% | 75.1% |
| Phient two-speed: counts until 8 sightings (phinet, R=32) | 23.1% | 12.2% | 26.8% | 22.5% | 44.9% | 49.8% | 62.3% | 80.5% |
| Phient two-speed: counts until 32 sightings (phinet, R=32) | 23.1% | 12.2% | 26.8% | 22.5% | 47.1% | 53.2% | 65.1% | 80.5% |
| Phient two-speed: counts until the function agrees (phinet, R=32) | 24.1% | 10.1% | 27.6% | 25.1% | 51.2% | 47.6% | 60.1% | 75.1% |
