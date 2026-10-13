# 86 · Better, worse, loss: Liskov's three operators, for words

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. Barbara Liskov's 1968
thesis (then Barbara Huberman, Stanford, under McCarthy) played chess endgames
with no evaluation number: a *forcing tree* coordinated with two functions,
**better** and **worse**. A move is kept if the position it reaches is better;
it is refuted if some reply makes it worse. Phient prefers that shape to a
score a position *has*, because it is the paradigm: a comparison a pair makes
under a condition. (Her later systems work — the Venus operating system, CLU,
the substitution principle — is what the sixteen Liskov papers read; she was
at MITRE and MIT, not IBM.)

## 1. The three operators (phinet::liskov)

| operator | what it compares | where it already was |
|---|---|---|
| `better(pred, a, b)` | `cos(pred, a) − cos(pred, b)`: is `a` the better next word than `b` under this prediction | the hinge's two cosines; the ranking MRR |
| `worse(pred, a, after, continuations)` | `cos(pred, a) − max_t cos(after, t)`: how far the best continuation after `a` falls below `a`'s own fit — a word that leads nowhere | nowhere; new |
| `loss(pred, target, decoy, margin)` | `max(0, margin − better)` | `Tape::hinge`; softmax is `−log` of `better` exponentiated over the table |

`forcing_choice` picks, among candidates, the largest `cos − λ·worse`; at
λ = 0 it is the greedy nearest word. Three unit tests: antisymmetry, a
dead-end move is refuted, and forcing prefers the move with a future.

## 2. Decoding by forcing (phiano `Decode::Forcing { k, lambda }`)

`OperatorGuide::nearest_forcing` takes the 2k nearest words, appends each to
the context, applies the operator once more, and re-ranks by `cos − λ·worse`
against the pool. `bin/decode_eval`, 40 held-out prompts × 30 tokens, k = 32:

| arm | KN perplexity | evaluator | distinct | shared 4-grams |
|---|---|---|---|---|
| trigram sampler | 13.3 | 0.671 | 0.884 | 26.2% |
| operator nearest-k | **66.9** | 0.673 | 0.889 | 9.9% |
| forcing λ = 0.05 | 69.7 | 0.676 | 0.881 | 10.5% |
| forcing λ = 0.25 | 68.5 | 0.678 | 0.880 | 10.5% |
| forcing λ = 1 | 107.4 | 0.712 | 0.777 | 22.3% |

Forcing does not beat the greedy operator on perplexity at any λ tried. At
λ = 1 it does what a refutation term must do on text: words with strong
continuations are the frequent function words, so it pulls toward them,
distinctness falls and cross-prompt repetition doubles, while the evaluator's
own quality rises to 0.712. The operator ships greedy; forcing is an arm.
The honest reading is that a one-step lookahead in a space where every word
has *some* continuation is not the endgame Liskov had, where most moves lead
nowhere; the place `worse` should bite is the quest, where a word either
has onward relations or does not.

## 3. The quest, scored by the three operators, filed by hexagram — and watched

`bin/quest_ingest` scores every `about` word: *better* is the truth's fit on
the kind's channel less the mean fit of every wrong word (the comparison the
trainer's hinge makes, in expectation over a random decoy), *worse* is the gap
between that fit and the truth's own strongest onward reading under any kind,
and *loss* is the hinge `max(0, margin − (fit − wrong))` averaged over the
wrong words (margin 0.1). The per-word table (`<table>.scores.tsv`) keeps each
word's centroid hexagram (`phinet::hexagram::hexagram_of`); the board and the
categories table below file by kind, which is where the structure is. A first version compared against the single best wrong word; on a
256-quantum channel that ties almost always and the table read flat (mean
better −0.008), so it measured the byte, not the learning.

The **scoreboard** runs during training: every `PHIANO_QUEST_BOARD` epochs
(default 8) it prints one line and appends a row per kind per occupied
hexagram to `<table>.board.tsv`.

**Which sixty-four categories.** Three filings were tested, each against 200
shuffles of its labels (`categories.rs`; *coherence* is how many times more
often two words that share an answer share a hexagram than two words at
random):

| filing | occupied | `is-a` pairs | `used-for` pairs |
|---|---|---|---|
| centroid (mean phase over all channels) | 61/64 | 0.95× (z −0.3) | 1.26× (z 3.3) |
| lines (six octave energies, `hexagram::lines`) | 7/64 | 0.97× (z −0.8) | 1.06× (z 4.2) |
| **by kind: the `is-a` channel in 64 sectors** | 24/64 | **22.2× (z 150)** | 0.73× |
| **by kind: the `used-for` channel** | 8/64 | 0.55× | **5.08× (z 167)** |

The centroid and the lines carry almost none of the quest; a kind's own
channel carries it all. So each kind of a quest files its words under its own
sixty-four hexagrams, and the board reports those. (`part-of`, `near` and
`unlike` give each word its own answer, so no pair shares one and there is
nothing to cohere.) On the synthetic quest, every 4 epochs:

```
▸ epoch 4:  better +0.492 · loss 0.206 · is-a 62/64 1.4× · used-for 64/64 1.2×
▸ epoch 12: better +0.866 · loss 0.053 · is-a 53/64 7.8× · used-for 58/64 1.8× · 11% kept their hexagram
▸ epoch 24: better +0.962 · loss 0.018 · is-a 34/64 20.0× · used-for 48/64 3.7× · 55% kept
▸ epoch 40: better +0.978 · loss 0.013 · is-a 27/64 22.6× · used-for 21/64 5.0× · 86% kept
▸ epoch 64: better +0.979 · loss 0.013 · is-a 24/64 22.2× of 36.4× · used-for 8/64 5.1× of 5.1× · 96% kept
```

The categories condense as the quest is learned: 62 → 24 hexagrams for 32
families, 64 → 8 for 5 tools. The ceiling (`of …×`) is the coherence of a
perfect filing, one hexagram per answer. `used-for` reaches it. `is-a` stops
at 22.2× of 36.4×, and the helix says why: **all 32 families are pure** (every
family's 8 members in one hexagram, none split), and 7 hexagrams hold two or
three families. Random placement of 32 heads in 64 sectors fills about 25; 24
are filled. Training pulls a family together and nothing pushes families
apart, because the quest trains attraction-only (docs/83).

Two ways to push the heads apart were tried (`repel.rs`; off by default).
*Repulsion during training* (`PHIANO_QUEST_REPEL`): after every epoch, each
answer's raw target side moves away from the other answers of its kind by the
mean of `sin` of their differences. *Spread before training*
(`PHIANO_QUEST_SPREAD=<passes>`): the same, run to convergence once, then left
alone.

| setting | `is-a` hexagrams | `is-a` coherence (of 36.4×) | kept | kind-channel MRR |
|---|---|---|---|---|
| neither (shipped) | 24/64 | 22.2× | 96% | **0.594** |
| repel 0.06 | **29/64** | **26.6×** | 95% | 0.532 |
| repel 0.25 | 33/64 | 24.2× | 85% | 0.525 |
| repel 1.0 | 30/64 | 20.9× | 86% | 0.577 |
| spread 50 or 200, no repel | 23/64 | 20.4× | 95% | 0.593 |
| spread + repel 0.06 | 28/64 | 22.8× | 83% | 0.555 |

Spreading first does nothing, and the reason is the coupling itself: a pair
moves *both* sides, so a head's target walks to the mean of its eight members'
random source angles and the spread is undone in the first epochs. Repulsion
during training at 0.06 is the best separation (29 hexagrams, 26.6×) and costs
a tenth of the channel's MRR; stronger repulsion moves heads faster than their
families can follow and families start to split (85% kept). Separation and
ranking trade off here. The shipped setting keeps ranking, and the collisions
are the price.

## 4. What remains

Forcing at depth two on the quest rather than on text; the hexagram table
on a Phi-4 quest; and `better`/`worse`/`loss` as the three names the
training loop exposes (today `loss` is the tape's hinge and `better` is
implicit in it).
