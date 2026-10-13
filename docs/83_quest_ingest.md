# 83 · Reading a quest back: phiecs → phiano, and the byte that would not move

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. A quest (phiecs) is
that sentence asked of an oracle one word at a time and written down as a
table — `about`, `kind`, `other`, `weight`. This note is the bridge back:
the table goes into a phiano facet, and the facet is asked to give it back.

## 1. The bridge

`bin/quest_ingest` reads a phiecs `relations.tsv` and does nothing a corpus
would not: every row becomes the three-token document `about kind other`,
repeated by its weight (×4, never fewer than once). The kind word stands
*between* the pair, which is exactly what `RelationVocab::learn` reads as a
marker, so the quest's kinds become marker channels with no mapping written
anywhere — `is-a` is channel 8 because `is-a` is the word most often found
standing between two others. `Trainer::couple_relations` then moves each
pair on its kind's channel alone, directed: the head's source side toward
the other's target side.

The measurement is the rank of the true `other` when `on(about, ·, k)` is read
over a pool, on the kind's own channel — against the same reading on the
*wrong* channel (the next kind's), and against `calls_for`, the offset-channel
reading the dialogue uses today. The oracle is `phiecs::quest::Synthetic`, a
hidden structure of 256 words in families of eight standing in five relations
by arithmetic on their index, so the truth is known exactly; a quest of depth
12 and budget 600 walked all of it: 1,280 rows, 293 target words. Nothing
here is about Phi-4 yet; it is about whether the path from a quest to a facet
loses anything, and the answer was that it lost almost everything.

## 2. What the first run found

Eight epochs, the shipped trainer: MRR 0.057 on the kind channel, hit@1
0.7%, 5× chance. Sixty-four epochs: 0.082. Every reading came back at 1.00
for the wrong word. The probe that explained it printed, for every one of the
256 `part-of` pairs, the same number: `|sin d| = 0.195`. Every pair had
stopped exactly eight quanta short of its target and stayed there.

A channel is one byte — 256 quanta of 2π, 0.0245 rad each — and a nudge is
`RELATION_RATE · sin d = 0.06 · sin d`. Once `sin d < 0.2` (11°) the nudge is
under half a quantum, `round()` writes back the byte it read, and the pair
never moves again. So a channel could hold at most ~32 distinguishable
targets however long it trained, and `bin/dimensions`' question — how many
relations a channel can carry — had a ceiling nobody had measured.

The fix is to round with dither: up with probability equal to the fractional
quantum, so the *expected* movement equals the nudge and small steps
accumulate. The coin is a hash of the phasor's own bytes and the channel's
evidence count (which every coupling increments), so a run is still
reproducible. It is `phasor::Rounding::Dither`, set on the trainer with
`with_rounding`; `nudge`/`nudge_target` keep nearest rounding.

## 3. Four arms

Sixty-four epochs each, 3,584 documents, seed 7, pool of all 293 target words
(chance MRR 0.021). "Within kind" ranks against only the words the oracle ever
named under that kind — the quest's own question.

| negatives | rounding | kind MRR | hit@1 | hit@5 | within kind | wrong channel | offset | s |
|---|---|---|---|---|---|---|---|---|
| on (shipped) | nearest (shipped) | 0.082 | 0.5% | 6.2% | 0.303 | 0.013 | 0.030 | 4 |
| on | dither | 0.177 | 3.8% | 29.0% | 0.404 | 0.014 | 0.018 | 4 |
| off | nearest | 0.092 | 1.0% | 7.0% | 0.330 | 0.014 | 0.013 | 4 |
| **off** | **dither** | **0.594** | **34.8%** | **97.4%** | **0.747** | 0.015 | 0.005 | 4 |

Three things the table says.

The wrong channel reads at chance in every arm (0.013–0.015 against 0.021),
and the offset reading never rises above 0.06: the kind channel carries the
kind, not adjacency, and what the dialogue reads today (`calls_for`, offset
channels only) sees none of it. A quest's relations are invisible to
generation until the generator reads marker channels.

Negative sampling is the other half of the ceiling. A negative kicks the
source away from a random word's target by `0.5 · rate · sin`, the same size
as the attraction once the pair is within 30°, so the pair jitters instead
of settling; on a graph where every source has one target per channel the
repulsion has nothing to disperse and only adds noise. The quest ships
attraction-only. This is not a claim about corpora, where the trainer's own note records
what happened to the offset channels without it (0.3–0.5 dispersion after
four passes, agreeing about everything).

Dither with the negatives off is where the numbers move: hit@5 97.4%, and
hit@1 at a third. Per kind:

| kind | rows | pool | MRR | hit@1 | hit@5 | within kind: pool | MRR | hit@1 | chance |
|---|---|---|---|---|---|---|---|---|---|
| is-a | 256 | 293 | 0.507 | 21.9% | 100% | 32 | 0.875 | 75.0% | 0.127 |
| part-of | 256 | 293 | 0.622 | 35.2% | 99.2% | 256 | 0.622 | 35.2% | 0.024 |
| near | 256 | 293 | 0.639 | 39.5% | 99.6% | 256 | 0.639 | 39.5% | 0.024 |
| used-for | 256 | 293 | 0.603 | 45.7% | 89.1% | 5 | 1.000 | 100% | 0.457 |
| unlike | 256 | 293 | 0.600 | 32.0% | 99.2% | 256 | 0.600 | 32.0% | 0.024 |

Where hit@1 stops is the byte again, from the other side: 256 `part-of`
targets on a 256-quantum circle collide, and a collision is a tie the
measurement scores against the truth. The read-back column in the run
(`w48 near → w49`, `w232 unlike → w104`, `w76 is-a → f9`) shows what a hit
looks like and what a tie looks like (`w0 is-a → w29 (1.00)`, a word never
named under `is-a` whose seeded target angle happens to sit on `f0`'s). That
last case is the whole-pool reading's honest weakness: **evidence is not per
side**, so a word that was only ever a source on a channel has evidence there
and a random target angle, and nothing in the phasor says it was never a
target. Within kind — against words that *were* named under the relation —
`is-a` reads 75% hit@1 over 32 heads and `used-for` 100% over 5 tools.

## 4. What dither costs a corpus

The obvious next question was whether to make dither the default, so
`bin/persona_ingest` was re-run with it on (docs/81 setup, three writers,
attribution on held-out paragraphs):

| rounding | Chesterton | Austen | Carroll | all |
|---|---|---|---|---|
| nearest (shipped) | 74% | 65% | 86% | **71%** |
| dither | 74% | 55% | 74% | 65% |

Six points worse, Carroll twelve. On a corpus a word takes tens of thousands
of nudges, and a coin per step is a random walk of √N quanta on top of
whatever the pair was doing; the deadband that costs a 1,280-row quest its
convergence is, on 247,000 words of Austen, a low-pass filter. So dither is
the trainer's choice, not the phasor's default: on for a quest, off for a
corpus, and a per-side sub-quantum residue (two bits per channel would do)
is the version that would serve both — recorded, not built.

## 5. Reading by name, and what the facet was never told

`Facet::marker_channel(kind)`, `calls_for_under(word, kind, goal)` and
`under(word, kind, pool)` are the readings by relation *name* that the
offset-only `calls_for` is not; `bin/quest_ask` opens the `.chroma` that
`quest_ingest` now saves beside the table and puts the quest's own question
back to it:

```
w0 · is-a ⇒ f0 (1.00), w29 (1.00), …      w0 · near ⇒ w1 (1.00), …
w0 · unlike ⇒ w128 (1.00), …             w176 · part-of ⇒ w211 (1.00), …
f0 · is-a ⇒ w150 (1.00), …               ← f0 was only ever a target: its source side is its seed
```

The first four are the truth; the last is the per-side evidence gap again,
read from the other end.

With a fifth of the rows held out (`PHIANO_QUEST_HOLDOUT=0.2`, 245 rows), the
channel that never saw a row reads it at chance, as it must (one hop, MRR
0.011 against chance 0.021 — below, because the truth ties with every
unevidenced word and ties count against it). A generic second hop — what
the word's relatives relate to, best of three intermediates per kind, no
knowledge of the structure — recovers some of what has structure to
recover and nothing of what has none:

| kind | held out | one hop MRR | two hops MRR | hit@5 |
|---|---|---|---|---|
| is-a | 43 | 0.003 | **0.095** | 11.6% |
| used-for | 56 | 0.003 | **0.093** | 7.1% |
| part-of | 48 | 0.017 | 0.009 | 0.0% |
| near | 50 | 0.021 | 0.009 | 0.0% |
| unlike | 48 | 0.013 | 0.015 | 2.1% |
| all | 245 | 0.011 | 0.045 | 4.1% |

`is-a` and `used-for` are shared by a family and by every fifth word, so a
neighbour's answer is evidence; `part-of`, `near` and `unlike` are one
target per word by construction, and no reading can infer them. Four and a
half times chance is a small number and it is the right one.

## 6. How to run it

```
# phiecs: a quest with a known answer, 256 words, all relations reached
cargo run -p phiecs --release --example quest -- w0 12 600 synthetic 256
# (PHIQUEST_OUT=<dir>; the same command with `ollama run phi4` asks a model)

# phiano: ingest it and measure; the table is appended to the given file
cargo run --release --bin quest_ingest -- <dir>/relations.tsv 64 out.md
#   PHIANO_QUEST_NEGATIVES=1  PHIANO_QUEST_ROUNDING=nearest   for the other arms
```

`data/quest/synthetic.tsv` is the quest measured here, `synthetic.chroma` the
facet it became, and `Quest::from_tsv` (phiecs) reads any such table back
into a world, so a quest can be continued from where an earlier run stopped.

```
cargo run --release --bin quest_ask -- data/quest/synthetic.chroma w0 is-a
PHIANO_QUEST_HOLDOUT=0.2 cargo run --release --bin quest_ingest -- data/quest/synthetic.tsv 64
```

## 7. What remains

The generator and dialogue still read offset channels only; `Facet::under`
exists now and nothing generates from it yet. Per-side
evidence, or a sub-quantum residue, is the phasor change this note argues
for and does not make. And the oracle here answers from arithmetic — the
same binary, pointed at `ollama run phi4`, is the first honest measurement
of what a model's answers look like on a channel, and it has not been run.
