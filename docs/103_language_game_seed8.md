# Language in action — the builders' game (seed 8, R = 32, 30 epochs)

3600 training episodes (3000 orders, 600 reports). Exact match: all five aspects of the action right.

Phient: 1593 relations kept from 3600 episodes · 22160 numbers · functions agree with counts on 88.7% · 189 s.

| learner | seen | new combinations (§8) | report, new stones (§21) | no name (§27) |
|---|---|---|---|---|
| lookup (whole utterance) | 86.8% | 0.0% | 1.3% | 100.0% |
| naming table (Augustine, §1) | 100.0% | 100.0% | 0.0% | 0.0% |
| bag of words + softmax | 100.0% | 100.0% | 100.0% | 100.0% |
| Phical counts, summed (no operator) | 85.0% | 71.3% | 60.3% | 100.0% |
| Phical counts, movement (no operator) | 100.0% | 100.0% | 100.0% | 100.0% |
| Phient relations, summed (phinet, R=32) | 40.2% | 13.7% | 10.3% | 100.0% |
| Phient relations, movement (phinet, R=32) | 100.0% | 100.0% | 100.0% | 100.0% |
| Phient two-speed: counts until 8 sightings (phinet, R=32) | 100.0% | 100.0% | 100.0% | 100.0% |
| Phient two-speed: counts until the function agrees (phinet, R=32) | 100.0% | 100.0% | 100.0% | 100.0% |

## Sepia (§28–30): a new colour-word, met only by pointing, then used in orders

| learner | 1 pointing | 3 pointings | 1 pointing + `colour` | 3 pointings + `colour` |
|---|---|---|---|---|
| lookup (whole utterance) | 0.0% | 0.0% | 0.0% | 0.0% |
| naming table (Augustine, §1) | 0.0% | 100.0% | 0.0% | 100.0% |
| bag of words + softmax | 0.0% | 0.0% | 0.0% | 0.0% |
| Phical counts, summed (no operator) | 0.0% | 47.0% | 18.3% | 47.0% |
| Phical counts, movement (no operator) | 21.0% | 39.7% | 100.0% | 100.0% |
| Phient relations, summed (phinet, R=32) | 0.0% | 0.0% | 0.0% | 0.0% |
| Phient relations, movement (phinet, R=32) | 5.3% | 4.7% | 100.0% | 100.0% |
| Phient two-speed: counts until 8 sightings (phinet, R=32) | 18.7% | 14.0% | 100.0% | 100.0% |
| Phient two-speed: counts until the function agrees (phinet, R=32) | 5.3% | 4.7% | 100.0% | 100.0% |

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
| lookup (whole utterance) | 14.7% | 17.5% | 20.7% | 24.3% | 29.6% | 35.6% |
| naming table (Augustine, §1) | 12.5% | 27.0% | 58.6% | 61.9% | 64.0% | 64.0% |
| bag of words + softmax | 33.0% | 50.1% | 77.4% | 94.7% | 100.0% | 100.0% |
| Phical counts, movement (no operator) | 13.3% | 33.8% | 55.4% | 70.4% | 92.7% | 97.4% |
| Phient relations, movement (phinet, R=8) | 8.6% | 19.6% | 47.0% | 62.6% | 86.7% | 98.1% |
| Phient two-speed: counts until 8 sightings (phinet, R=8) | 13.3% | 35.5% | 53.7% | 65.1% | 88.2% | 97.1% |
| Phient two-speed: counts until 32 sightings (phinet, R=8) | 13.3% | 33.8% | 55.4% | 69.5% | 92.7% | 97.4% |
| Phient two-speed: counts until the function agrees (phinet, R=8) | 9.4% | 35.4% | 51.8% | 63.7% | 92.4% | 96.8% |
| Phient relations, movement (phinet, R=32) | 13.0% | 36.0% | 52.6% | 65.1% | 90.2% | 99.0% |
| Phient two-speed: counts until 8 sightings (phinet, R=32) | 13.3% | 35.4% | 52.5% | 67.3% | 91.7% | 97.4% |
| Phient two-speed: counts until 32 sightings (phinet, R=32) | 13.3% | 33.8% | 55.4% | 70.4% | 92.7% | 97.4% |
| Phient two-speed: counts until the function agrees (phinet, R=32) | 10.6% | 35.7% | 55.3% | 64.6% | 91.6% | 97.3% |

## The stranger (§32): new words for a game already played — exact match on new-vocabulary orders

| learner | child, 10 | stranger, 10 | child, 20 | stranger, 20 | child, 40 | stranger, 40 | child, 80 | stranger, 80 |
|---|---|---|---|---|---|---|---|---|
| lookup (whole utterance) | 1.2% | 1.2% | 3.9% | 3.9% | 10.6% | 10.1% | 14.2% | 14.2% |
| naming table (Augustine, §1) | 8.2% | 13.9% | 30.2% | 39.6% | 58.1% | 69.2% | 82.4% | 89.8% |
| bag of words + softmax | 3.2% | 13.9% | 25.9% | 34.8% | 41.1% | 48.4% | 71.5% | 80.1% |
| Phical counts, movement (no operator) | 1.5% | 5.2% | 12.6% | 22.2% | 50.6% | 73.1% | 67.9% | 77.4% |
| Phient relations, movement (phinet, R=8) | 1.2% | 3.6% | 1.8% | 21.5% | 42.2% | 57.2% | 47.2% | 65.1% |
| Phient two-speed: counts until 8 sightings (phinet, R=8) | 1.5% | 4.9% | 12.2% | 18.4% | 47.6% | 57.5% | 64.8% | 66.0% |
| Phient two-speed: counts until 32 sightings (phinet, R=8) | 1.5% | 4.9% | 12.6% | 16.9% | 50.6% | 59.6% | 67.9% | 76.5% |
| Phient two-speed: counts until the function agrees (phinet, R=8) | 1.4% | 4.6% | 10.6% | 17.2% | 46.2% | 56.9% | 65.5% | 73.4% |
| Phient relations, movement (phinet, R=32) | 2.2% | 3.2% | 8.8% | 18.4% | 48.2% | 51.9% | 59.4% | 72.5% |
| Phient two-speed: counts until 8 sightings (phinet, R=32) | 1.5% | 5.0% | 8.9% | 16.6% | 47.0% | 59.4% | 67.8% | 72.2% |
| Phient two-speed: counts until 32 sightings (phinet, R=32) | 1.5% | 5.0% | 12.6% | 18.0% | 50.6% | 65.4% | 67.9% | 76.9% |
| Phient two-speed: counts until the function agrees (phinet, R=32) | 2.2% | 3.6% | 10.2% | 22.9% | 50.6% | 51.9% | 63.6% | 71.2% |
