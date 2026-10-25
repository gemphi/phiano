# 88 · The first Phi-4 quest: eight questions, five answer shapes

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. Until today every
quest was asked of the synthetic oracle, which answers in exactly the shape
it is asked. This note is the first one asked of a real model: Phi-4 under
Ollama 0.34.4 on the user's Windows machine, CPU only, seed `cat`, depth 2,
budget 8 (`data/quest/phi4_cat/`).

## 1. What Phi-4 said, and how

Eight questions took 50–141 s each (the first includes loading the model).
Every answer was sensible; almost none was in the shape asked. The prompt
said `kind | other word | weight`, at most 8, one per line. Phi-4 answered:

| shape | example (verbatim) | pages |
|---|---|---|
| as asked | `has \| fur \| 0.9` | fur, tail, pet, cat (second run) |
| pipes dropped | `is-a mammal 0.9`, `does give birth 0.7` | cat (first run), mammal |
| one field per line | `does` / `sound` / `1.0`, blank line, … | purr |
| labelled | `kind: is-a, other word: aquatic, weight: 0.9` | fish |
| kind glued to answer | `has-bark \| bark \| 0.8`, `mammal \| mammal \| 0.9` | dog, cat |

and 5 to 20 lines where 8 was the limit. The parser (phiecs
`quest/parse.rs`) now reads all five; a compound answer of two or three words
is one word (`pet store` → `pet-store`), four or more is a phrase and goes; a
kind one `s` from an offered one is that one (`likes` → `like`). Two bugs in
the quest itself showed up and are fixed: every prompt offered *all* kinds
declared so far, so a misparse (`has-live` from `has live young`) was offered
back and Phi-4 used it; and the step pages rebuilt each prompt with the
current kinds instead of keeping what was sent. Prompts now offer only the
sixteen kinds a quest starts with, carry one example on another word
(`bread`), and each step keeps its prompt verbatim.

Answers are evidence and are kept verbatim, so nothing had to be asked again:
`examples/reparse` rebuilds `relations.tsv` from the pages. The run's own
parser kept 76 relations over 26 kinds (10 of them misparse inventions);
re-read, the same pages give **89 relations over 18 kinds**, the 4 invented
ones all from kinds the old prompt offered back.

A word named as its own relation is dropped, not guessed: `mammal | mammal |
1.0` asked about cat almost certainly means `is-a | mammal`, but the parser
does not know that, so **`cat · is-a` reads nothing**.

## 2. Ingested (quest_ingest, 64 epochs, board every 4)

| | value |
|---|---|
| kind-channel MRR / hit@1 / hit@5 | **0.790 / 61.8% / 100%** (chance 0.069) |
| within-kind MRR | 0.796 |
| wrong channel / offset reading | 0.045 / 0.057 |
| better / worse / loss (epoch 64) | +0.969 / +0.047 / 0.003 |
| scored words (asked about) | 8 |

The channel carries the quest and the wrong channel is at chance, as on the
synthetic quest. The scoreboard is not a result at this size: with 8 scored
words, `has` coherence moves between 0× and 11.2× from one board to the next.

**The ceiling was wrong for a real model.** It assumed one answer per word
per kind, which the synthetic oracle gives and Phi-4 does not (cat *has* fur
and tail). It kept each word's last answer and printed `of 0.0×`. It is now
the coherence of the filing that puts every chain of words sharing an answer
in one hexagram; on one-answer quests that is the old number (synthetic:
36.4× and 5.1×, unchanged). With several answers it is not an upper bound —
a filing that keeps only one sharing pair together can score higher (11.2×
against 7.0× here) — so the board now says `perfect`, not `of a possible`.

## 3. What the facet gives back (quest_ask)

```
tail · part-of ⇒ body (1.00)
fish · does    ⇒ breathe (1.00), swim (1.00), purr (1.00), cat (0.62)
pet · used-for ⇒ emotional-support (1.00), cat (0.99), tail (0.89), companionship (0.87)
cat · has      ⇒ fur (1.00), tail (1.00), bark (1.00), mammary-glands (1.00)
dog · has      ⇒ fur (1.00), tail (1.00), bark (1.00), mammary-glands (1.00)
```

The last two lines are the structural limit a real model exposes: a word has
**one angle per channel**, so four `has` answers of cat, dog and mammal are
pulled to one place and read back tied, and cat inherits dog's `bark`. The
synthetic quest never showed this because each word had one answer per kind.
Several answers per kind want either more than one phasor per (word, kind) —
the 32–64 relations of the paradigm, spent per answer rather than per kind —
or the `other`'s own target angle spread by repulsion (docs/86 §3), which here
would be ranking against separation again.

Negatives do not fix it (`PHIANO_QUEST_NEGATIVES=1`, same 85 rows after
unoffered kinds are dropped, 64 epochs): kind MRR 0.806 → 0.769, and the
ties break the wrong way — `cat · has` loses `bark` but gains `pet`, `collar`,
`owner` (pet's answers); `dog · has` gains `gills`, `scales` (fish's). Pushing
a word off its wrong answers moves it onto someone else's. The one-angle
limit stays until a (word, kind) can hold more than one angle.

## 4. Next

The full quest (`cat 3 64`, about an hour on this machine) with the fixed
parser and prompt; then whether `has` coherence and the tie hold at 64 scored
words, and whether one angle per (word, kind) is the wall it looks like.
