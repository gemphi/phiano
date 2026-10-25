# 127 · Two paragraphs as a tree of knowing: how Phiano holds them, how a CLM would, and how it goes on

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 0. The two paragraphs

> He remained presumptuous even if the evidence was pointing to the direction he had been struggling to reach. It suddenly happened that she came to know exactly that she was in love with her boss, but only circumstances needed to be right.
>
> She suddenly had an idea: their new investment would demand them to meet, and she was going to make an irresistible move on this young CEO of an AI company.

Seventy-eight words, two paragraphs, one chapter. docs/126 said the levels need a corpus whose levels say more than its words; this is the smallest one. Everything below is run, not described: `story data\story --go "She suddenly"` (`bin/story`, `phiano::prose`, `phiano::relate::{Counts, levels, tree}`).

## 1. How a CLM holds them

A Context Language Model keeps the two paragraphs as the two paragraphs. They are text in its context file, `[[CTX_TURN 1 role=user]]` followed by the words, read in full by the whole model on every turn. To go on, the model generates the next tokens from everything in the file at once; to keep the story within budget as it grows, the model edits the file — collapses the first chapter into a summary it writes itself ("he is presumptuous; she realised she loves her boss; she plans to use the new investment to meet him") and reads the summary from then on. What it keeps is a judgment in its own words; what it knows about the story is whatever its weights already knew plus what the file says. Nothing in the model changed by reading the two paragraphs.

## 2. How Phiano holds them: the tree

The story is a tree on disk, and the tree is the condition hierarchy:

```
data/story/ch1/p1/s1.txt   He remained presumptuous …
data/story/ch1/p1/s2.txt   It suddenly happened …
data/story/ch1/p2/s1.txt   She suddenly had an idea …
```

Reading it makes 548 rows, nothing else — `(about, kind, other, weight)`, the quest's rows. Every scope is a word within its parent, and every word of the story is an occurrence told under its whole path:

```
scope:story/ch1        within    scope:story
scope:story/ch1/p1     within    scope:story/ch1
scope:story/ch1/p2     within    scope:story/ch1

suddenly@story/ch1/p2#1   has-word  suddenly
suddenly@story/ch1/p2#1   within    scope:story
suddenly@story/ch1/p2#1   within    scope:story/ch1
suddenly@story/ch1/p2#1   within    scope:story/ch1/p2
suddenly@story/ch1/p2#1   pair      pair:she suddenly
suddenly@story/ch1/p2#1   says      suddenly
suddenly@story/ch1/p2#1   next      had
```

Five kinds: `within` (the path, 238 rows), `has-word` (which word this is), `pair` (the word before it with it — D8, a word's condition is set by the words before it), `says` (what a scope's subtree said, for the cast), `next` (the word after). The conditions are `has-word`, `within` and `pair`; `next` is what is asked. Counted, not trained: the rows went into `Counts` the moment they were read, and the tree printed back says what each scope knows:

```
scope:story — 78 words told; says: to she an was had he suddenly that
  scope:story/ch1 — 78 words told
    scope:story/ch1/p1 — 45 words told; says: to he she that the was be been
    scope:story/ch1/p2 — 33 words told; says: an she to ai and ceo company demand
```

This is the tree of X ⇒ Y: `suddenly ⇒ had` under `pair:she suddenly` under `scope:story/ch1/p2` under `scope:story/ch1` under `scope:story`. Each condition is a word with rows of its own.

## 3. How it goes on

To continue from "She suddenly", the student does three things, all readings of the rows:

1. **Cast.** The words said so far are cast into the tree (`ScopeTree::cast`): from the root down, into the children whose subtree said them more than their parent did. "she suddenly" is drawn into `scope:story/ch1/p2`, because the second paragraph says *she* and *suddenly* more than the chapter does.
2. **Read under the path.** `next` is read under the levels in order — `scope:story` → `scope:story/ch1` → `scope:story/ch1/p2` → the last word `suddenly` → the pair `pair:she suddenly` — each level moving the distribution the level above arrived at (`Counts::read_levels`, D4 nested). A condition earns its post against the prior (D2, τ = 0.5 nats) and moves the reading from where the outer levels left it.
3. **Go on.** The best word is said; a word that would repeat three already said is passed over; the cast is redone as the words change, so the reading can leave one paragraph for another.

Run, with both paragraphs told:

```
She suddenly → she suddenly had an idea: their new investment would demand them to meet,
               and she was going to make an irresistible move on this young ceo of an ai company.
  after "she suddenly": drawn into scope:story/ch1/p2 (posts: scope:story/ch1/p2, suddenly, pair:she suddenly)

He → he had been struggling to reach. it suddenly happened that she was going to make an
     irresistible move on this young ceo of an ai company.
  after "he": drawn into scope:story/ch1/p1 (posts: he)
  after "that she was going": drawn into scope:story/ch1 (posts: going, pair:was going)
  after "going to make an": drawn into scope:story/ch1/p2 (posts: scope:story/ch1/p2, an, pair:make an)

Her boss → her boss, but only circumstances needed to be right.
```

With one story told, going on is recombining it: the second continuation crosses from the first paragraph to the second at "she was going", where the words draw the cast out of `p1` into the chapter and then into `p2`. That is the honest state of a student that has read seventy-eight words. It is also the whole mechanism; nothing changes as the tree grows except the counts.

## 4. Accumulating: the same ask, one paragraph earlier

The claim is that this is a matter of learning and accumulating. Measured: the same asks with only the first paragraph on disk, then with both.

| asked | after `p1` alone | after `p1` and `p2` |
|---|---|---|
| She suddenly | she suddenly **happened that she was in love with her boss**, but only circumstances needed to be right. | she suddenly **had an idea: their new investment** would demand them to meet, and she was going to make an irresistible move on this young ceo of an ai company. |
| She was going to | she was going to **the direction he had been struggling to reach.** it suddenly happened that she was in love with her boss … | she was going to **make an irresistible move** on this young ceo of an ai company. |

The second paragraph was a directory copied into the tree. No training, no edit, no summary: 266 rows more, and the cast has a second scope to be drawn into. A chapter two would be a directory `ch2/`; a second story a sibling of `story/`; the same reading would run over all of it, and a word heard in a new chapter is read under that chapter first.

## 5. Where this stands beside the CLM, said plainly

Both read the next word under conditions. The CLM's conditions are the text of the two paragraphs (and later the summary it writes of them), read whole; Phiano's are the path — story, chapter, paragraph — and the words before, each a condition with counts, read in order. The CLM goes on with everything a large model knows; Phiano goes on with what it has been told, which here is seventy-eight words, and it shows. What Phiano has that the CLM does not: nothing was summarised, so nothing can be lost in a summary; nothing was retrained, so a new paragraph is learned at once; the structure of the story (which paragraph, which chapter) is a condition the reading uses, not a region of text the model must decide to keep; and every step of going on is a row that can be shown (`posts: scope:story/ch1/p2, suddenly, pair:she suddenly`). What the CLM has that this does not: the language. The next step for the tree is the obvious one — tell it more (Phi-4 asked for the next paragraph, chapter by chapter, through the quest, the way the code student asks for the words it does not know), and measure going on by whether the teacher agrees (D20).

`story data\story [--rows rows.tsv] [--go "…"] [--words 40]` (the same two paragraphs also lie in `data\text\story\ch1.txt`, docs/128) · rows: `data\story\rows.tsv` · code: `src/prose/mod.rs`, `src/bin/story/main.rs`, `src/relate/{levels,tree}.rs`
