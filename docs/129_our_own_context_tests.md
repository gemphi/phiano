# 129 · Our own context tests: a session kept as a tree, asked what ContextBench asks

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 0. The test

Paper 24 §7 named needle retention as the place the two designs meet. ContextBench (Shao et al. 2026) isolates what a Context Language Model must do to its context file: keep a needle under pressure, keep a key-value store, read the latest value. Here the same is asked of a session kept as a tree of conditions, with nothing edited and nothing summarised. `bin/context` writes a session to disk — `session/b<block>/t<turn>.txt`, one turn per file, a hundred turns per block, filler from *Emma*'s paragraphs cut to forty words — and tells, at random turns, ten needles ("the code for the vault is 4471.") and twenty keys ("remember that alpha is seventeen."); in the update test every key is set again later ("now alpha is forty.") and the latest must be read. After all the turns, each is asked as a prompt ("the code for the vault is", "alpha is") and the answer is the next word `prose::Knowing` reads — the cast into the tree, then `next` under the path, the last word and the pair (docs/127–128), unchanged.

## 1. Measured

| turns | words told | rows | learned in | needles found | keys read | updated keys read (latest) | one ask |
|---|---|---|---|---|---|---|---|
| 25 | 1,159 | 17,335 | 0.02 s | 10 of 10 | 20 of 20 | 20 of 20 | 2 ms |
| 100 | 3,805 | 56,875 | 0.07 s | 10 of 10 | 20 of 20 | 20 of 20 | 3 ms |
| 400 | 14,054 | 210,010 | 0.34 s | 10 of 10 | 20 of 20 | 20 of 20 | 10 ms |
| 2,000 | 69,470 | 1,038,050 | 1.8 s | 10 of 10 | 20 of 20 | 20 of 20 | 22 ms |
| 5,000 | 172,780 | 2,581,700 | 4.8 s | 10 of 10 | 20 of 20 | 20 of 20 | 33 ms |

(seed 11; seed 7 the same at every length.) Five thousand turns is 173k words, more than a 272K-token context budget holds of this text; nothing was dropped to keep it.

## 2. What it took, and what each thing is

Three things had to be true of the cast for this to hold, and each is a definition already in the record, now in the code:

- **Recency** (phiecs's `Condition`: `now` over `was`). Among scopes that say as many of the asked conditions, the one told later stands first. Scopes are named in the order told (`t0001`, `t0002`; `p1`, `p2`), and the cast's order is: most conditions said, then the latest, then the most attracted. The counts themselves have no order of telling; the tree does.
- **Locality** (D3: the condition is local). A word is counted as said by a scope only if at most half of its siblings say it. `is` is said by every turn and draws nowhere; `alpha` is said by two of two thousand and draws exactly. Without this, a turn that happened to say *is* four times outdrew the turn that said the key.
- **Pairs are said too.** *Emma* says *November* eleven times and *papa* often; a cast on the word alone was drawn to the month. A scope now says its pairs as well as its words (`pair:november is`), and the recent pairs are cast with the recent words. Keys that collide with the language are then found by the two words together.

None of these is a rule about keys or needles; the story of docs/128 goes on exactly as before under them.

## 3. Beside the CLM, plainly

The CLM paper's needle and key-value tasks measure whether a model's own editing of its context keeps what it will need; the RL rewards an edit that kept it cheaply. Here there is no editing to measure: the tree keeps every turn as rows, the cost of keeping is the rows (2.6 million at five thousand turns, about a third of a gigabyte), and reading is a cast and a nested reading in under half a second. The honest limits: this is retrieval of what was told, in the words it was told in — a needle paraphrased ("the vault's combination") is not found, because no relation beyond `next` and `says` has been counted for it; the time of one ask grows slowly with the tree (the cast now reads only the asked conditions' counts at each scope and builds a whole distribution only for the scopes it enters: 2 ms at 25 turns, 36 ms at 5,000); and the filler is one novel. What a CLM does that this does not is everything the large model does with what it kept. What this does that a CLM cannot is keep without choosing.

`context [--turns 25,100,400,2000,5000] [--needles 10] [--keys 20] [--seed 7]` · `CONTEXT_DEBUG=1` prints the misses; `PHIANO_CAST_DEBUG=1` prints every cast
