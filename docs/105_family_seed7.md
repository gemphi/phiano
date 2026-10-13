# Family resemblance and Moses in Phi-4's answers

/home/claude/night/all.tsv: 3630 rows, 209 words. Seed 7.

## Family resemblance (§§65–67)

| category | members | longest fibre | mean overlap (Jaccard) | thread | random sets: fibre / overlap / thread |
|---|---|---|---|---|---|
| animal | 11 | 55% | 0.048 | 100% | 25% / 0.007 / 46% |
| component | 5 | 40% | 0.026 | 60% | 36% / 0.007 / 40% |
| device | 5 | 80% | 0.080 | 100% | 36% / 0.007 / 40% |
| food | 10 | 40% | 0.033 | 90% | 26% / 0.007 / 42% |
| machine | 7 | 86% | 0.103 | 100% | 32% / 0.008 / 42% |
| mammal | 9 | 78% | 0.089 | 89% | 28% / 0.007 / 42% |
| material | 11 | 27% | 0.011 | 91% | 25% / 0.006 / 43% |
| organism | 5 | 60% | 0.036 | 100% | 35% / 0.007 / 40% |
| process | 7 | 43% | 0.018 | 71% | 32% / 0.007 / 42% |
| service | 5 | 40% | 0.044 | 60% | 35% / 0.006 / 39% |
| system | 6 | 50% | 0.069 | 67% | 33% / 0.007 / 38% |
| tool | 9 | 56% | 0.043 | 89% | 27% / 0.007 / 40% |
| **mean over 12 categories** | | **54%** | **0.050** | **85%** | 31% / 0.007 / 41% |

Categories with one relation every member holds: 0 of 12.

## Moses (§79): which word do some of its relations pick out? (209 candidates, chance top-1 0.48%)

| relations kept from | dropped | counts: top-1 / MRR | phinet functions: top-1 / MRR | words |
|---|---|---|---|---|
| told | 0% | 100.0% / 1.000 | 100.0% / 1.000 | 204 |
| told | 50% | 99.5% / 0.997 | 99.0% / 0.993 | 204 |
| told | 75% | 93.6% / 0.970 | 92.6% / 0.954 | 204 |
| told | 90% | 75.0% / 0.874 | 73.5% / 0.839 | 204 |
| held out (never told) | 0% | 0.0% / 0.009 | 27.9% / 0.385 | 111 |
