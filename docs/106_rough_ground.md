## The rough ground (§§107, 125): contradictions in Phi-4's answers

210 of 3630 rows take part in a contradiction:

| contradiction | rows |
|---|---|
| a is-a b and b is-a a | 39 (e.g. animal is-a mammal; animal is-a vertebrate; automobile is-a car; car is-a automobile) |
| a has b and a part-of b | 164 (e.g. animal-kingdom has taxonomy; animal-kingdom part-of taxonomy; appendage has limb; appendage has tail) |
| a part-of b and b part-of a | 12 (e.g. computer part-of device; computer part-of system; device part-of computer; drainage-basin part-of river) |

Trained on the told rows with the contradicting ones (2885) and without (2713); read on the 702 held-out rows that contradict nothing.

| seed | with contradictions: MRR / within | without: MRR / within |
|---|---|---|
| 7 | 0.084 / 0.111 | 0.080 / 0.112 |
| 8 | 0.075 / 0.107 | 0.076 / 0.107 |
| 9 | 0.073 / 0.128 | 0.076 / 0.123 |
| 10 | 0.083 / 0.114 | 0.094 / 0.127 |
