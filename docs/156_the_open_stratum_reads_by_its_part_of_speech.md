# 156 · The open stratum reads, by its part of speech

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/152 left one problem standing: the **open stratum** — positions on held-out *Emma* whose next word was never told after the last word or the last pair. There, every reader the system had read at or below frequency. docs/153–154 added a symmetry and a gain and did not move it. This doc moves it, and says exactly by how much and why.

## 1. The ladder (`bin/ladder`)

One binary reads held-out *Emma* and turns the reader's levels on one rung at a time, reporting top-1 · MRR split by stratum: **told** (the word was told after the last word or pair) and **open** (it never was). Nothing is trained; the counts are learned once and read. The rungs:

0. the prior (frequency)
1. + the path's scopes
2. + the last word
3. + the last pair
4. + the last word's kin (`Counts::kin_base`, docs/138)
5. + dictionary neighbours (WordNet synonyms/hypernyms, spreading rung 3)
6. + the **POS frame**: the last word predicts the next word's part of speech
7. + both: the told levels moving from the POS-framed prior
8. mix: half the told levels, half the frame
9. router: the told levels where they concentrate, the frame where they don't

## 2. What each rung reads (held-out *Emma*, seed 7, 1,000 positions)

| rung | told | open |
|---|---|---|
| 0 · prior (frequency) | 0.0% · 0.052 | 0.0% · **0.026** |
| 1 · + path scopes | 0.0% · 0.052 | 0.0% · 0.026 |
| 2 · + last word | 17.7% · 0.279 | 0.0% · 0.018 |
| 3 · + last pair | 19.6% · 0.305 | 0.0% · 0.018 |
| 4 · + kin base | 19.6% · 0.301 | 0.0% · 0.022 |
| 5 · + dictionary (WordNet) | 19.7% · 0.288 | 0.0% · 0.014 |
| 6 · **+ POS frame** | 2.3% · 0.092 | **0.9% · 0.038** |
| 7 · + both (levels on the frame) | **19.9% · 0.306** | 0.0% · 0.022 |
| 8 · mix (levels + frame) | 19.7% · 0.301 | 0.0% · 0.021 |
| 9 · router | 16.9% · 0.261 | 0.0% · 0.027 |

## 3. What this says

**The told stratum was a solved problem; the levels climb it.** The last word buys 17.7%, the last pair 19.6%, and framing the prior by part of speech (rung 7) lifts it to a new best, **19.9%**. Everything the levels do — word, pair, kin — is a told-stratum reader.

**The open stratum is a data problem, as the counts show.** Rungs 2–5 all read the open stratum *below* frequency (0.014–0.022 against 0.026): the word, the pair, the kin, and synonym-smoothing each pull mass toward what was told, which is never the open truth. Kin (docs/138) and dictionary synonyms do not help here, because the open truth is rarely the synonym of a specific told continuation — it only adds blur. No amount of co-occurrence counting reads a word never told in its context.

**The part of speech is the abstraction that reads it.** Rung 6 is the first reader ever to beat frequency on the open stratum: **0.038 against 0.026 (+46%)**, and the first non-zero open top-1 (0.9%). The mechanism is exactly the two pieces the system was missing data for. The **closed-class last word** — one of ~220 function words, dense in counts — predicts the *part of speech* of the slot: after *the*, a noun; after *has*, a verb or participle. WordNet gives every content word its part of speech. Re-weighting the prior by that frame ranks *threshold* far above its global frequency rank, where punctuation and function words dominate. The function-word core sets the frame; the dictionary fills each word's part of speech; the two together read the hole.

**One reader can clear frequency on both strata.** The levels and the frame are two regimes — the levels sharp where there is a told continuation, the frame broad where there is not — and mixing them dilutes both (rung 8). Routing by the levels' own confidence (rung 9) keeps the told reading well above frequency (16.9%) and lifts the open stratum past it (0.027). The pure frame still holds the best open reading (0.038); the router trades a little of it for the told stratum.

## 4. What this does not yet do, and the next rungs

The open-stratum lift is real but small: the part of speech narrows the hole, it does not fill it. The frame is coarse — six buckets — and conditions only on the last word. The data the abstraction was waiting for is now identified and partly built:

- **A finer frame.** Condition the part-of-speech frame on the last *pair* and on the scope, not the last word alone; use WordNet's finer senses (the ~45 supersenses: noun.location, noun.quantity, verb.motion) and the is-a chain as the bucket, not the six top-level classes.
- **The word's own dimensions, recursively.** Each word carries its gloss words, synset, is-a chain and antonyms as relations in the tree itself (the ~220 function words are the recursive bottom; content words are defined in their terms). `scripts/export_pos.py` and `export_near.py` build the first two dimensions from Open English WordNet; the rest follow.
- **Usage from more than one novel.** Austen alone is thin. The readers should be trained per author — Joyce, Henry and William James, and others — as separate scopes, so a word's company is counted across styles, not one.
- **The operator on the frame.** The Joint FNO (docs/152) and the pumped mirror (docs/154) read the told stratum; the POS-framed prior is the base they should move from, so the operator's reading is corrected by the part of speech where it has no count.

`ladder data/text --in text/emma --n 1000 [--gate 0.12] [--alpha 0.5]` · `python3 scripts/export_pos.py` · `python3 scripts/export_near.py` (Open English WordNet via the `wn` package). The lexical data is in `data/lex/` (regenerated by the scripts, not in git); the function-word core is `data/lex/function.txt`.
