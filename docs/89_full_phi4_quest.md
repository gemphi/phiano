# 89 · The full Phi-4 quest: 64 questions, one compressed model

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. docs/88 was eight
questions; this is the first full quest — seed `cat`, depth 3, 64 questions,
Phi-4 (14B, text only) under Ollama on the user's Windows CPU, about 75
minutes — and the first compressed model made from a real model's answers
(`data/quest/phi4_cat/`).

## 1. What came back

The run's own parser (the build before docs/88's last fixes) kept 560 rows
over 58 kinds. Re-read from the 64 saved pages with today's parser
(`examples/reparse`): **501 rows, 14 kinds, 64 words asked, 378 answer
words.** The difference is kinds the prompt did not offer: 44 names, 36 of
them used once (`owns`, `holds-title-to`, `authoritative-over`); only
`produces` recurs (9 rows). Four Phi-4 synonyms are now read as the offered
kind (`opposite` → `unlike`, `has-a` → `has`, `similar-to` → `like`). Most
pages have 8 relations; `allergies` has 40 and one page 64 — Phi-4 still
ignores "at most 8" when a word has many consequences.

## 2. The compressed model

| | |
|---|---|
| `relations.chroma` (the model) | **160 KB**, 392 words × 64 channels |
| Phi-4 q4 in Ollama | about 9 GB |
| helix (the same model as a DNA-coded tree) | 2.6 MB, 392 strands, 0.5 s to write |

It is not Phi-4 made small; it is what Phi-4 said about the 392 words this
quest reached, kept as angles. What it is worth is the next two tables.

## 3. Given back (quest_ingest, 64 epochs, board every 4)

| | value |
|---|---|
| kind-channel MRR / hit@1 / hit@5 | **0.762 / 63.5% / 92.4%** (chance 0.017) |
| within-kind MRR | 0.810 |
| wrong channel / offset reading | **0.010 / 0.021** (at chance) |
| better / worse / loss (epoch 64) | +0.989 / −0.005 / 0.006 |
| hexagram filings unchanged since the last board | 98% |

With 64 scored words the per-kind scoreboard is a result now. Every kind with
shared answers files its words coherently, each against 200 shuffles of its
own labels:

| kind | hexagrams | coherence | z | perfect filing |
|---|---|---|---|---|
| is-a | 35/64 | 37.6× | 34.9 | 53.1× |
| used-for | 32/64 | 38.3× | 21.3 | 125.3× |
| causes | 31/64 | 31.3× | 17.1 | 98.4× |
| part-of | 18/64 | 27.1× | 16.0 | 46.4× |
| belongs-to | 27/64 | 37.1× | 14.4 | 156.0× |
| has | 34/64 | 20.3× | 11.8 | 53.0× |
| done-to | 28/64 | 26.9× | 11.8 | 22.7× |
| like | 25/64 | 126.0× | 9.9 | 378.0× |
| does | 27/64 | 37.0× | 9.2 | 148.2× |
| made-of | 9/64 | 18.3× | 6.8 | 18.3× |

The centroid filing carries almost none of it (is-a 2.6×, z 0.9) and the
octave lines none (all |z| < 1.3), as on the synthetic quest (docs/86): the
sixty-four categories live on each kind's own channel.

## 4. Not given back: questions it was never told

`PHIANO_QUEST_HOLDOUT=0.2` keeps 126 rows out of training and asks them:

| | MRR | hit@5 |
|---|---|---|
| chance | 0.017 | |
| one hop (`w · kind · ?` read directly) | **0.005** | 0% |
| two hops (through a word it was told about) | 0.021 | 1.6% |
| two hops, `part-of` only (7 rows) | 0.151 | 14.3% |

The model does not infer relations it was not told. One hop is below chance —
a word's angle on a kind points at its own trained answers, so an untold
answer is ranked behind them. Two hops is at chance except `part-of`, on
seven rows. This is the same as the synthetic quest (docs/83) and the
number the overall goal is judged by: until it moves, the compressed model
is a lookup that files well, not a model that knows.

## 5. The one-angle tie, at size

```
cat · is-a      ⇒ mammal (1.00), allergies (1.00), group (1.00), calmness (1.00)
cat · has       ⇒ animal (1.00), fur (1.00), immune-system (1.00), tail (1.00)
fur · made-of   ⇒ keratin (1.00), cat (0.99), meow (0.83), sheep (0.83)
allergies · causes ⇒ difficulty-breathing, headache, itching, itchy-eyes (all 1.00)
tail · part-of  ⇒ body (1.00), wagging (1.00), ears (0.99), animal (0.98)
```

Phi-4 told the quest `cat is-a mammal` and nothing else under `is-a`; the
model reads back mammal first, tied with `allergies`, `group` and `calmness`,
which are other words' `is-a` answers that share cat's angle. The right answer
is always among the top; the ties are the one angle per (word, kind) that
docs/88 found, and negatives do not remove them (docs/88 §3).

## 6. Next

Two measured walls, in order: (1) several angles per (word, kind), so a word
holds all its answers and no one else's; (2) inference — held-out one hop at
chance. The first is a storage change in `SpectralPhasor`; the second is the
question of what, in this model, a relation *not* told could be read from.
