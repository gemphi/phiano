# 94 · One kind at a time: a quest shaped for laws

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. A law `k₁ ⇒ k₂` can
only be counted on a told `w k₁ v` when `w` and `v` were both asked `k₂`
(docs/93). The first full Phi-4 quest asked 64 of the 442 words it met, so
each kind pair had 11–19 testable triples and no law reached 80%.

## 1. The quest

`Quest::one_kind_at_a_time(&kinds)` (phiecs `quest/per_kind.rs`): a
question is a (word, kind) pair on the `ONE_KIND` template — at most three
answers, one line each — and **every word an answer names is asked every
kind**, breadth first to the quest's depth. Claims under any other kind are
dropped. `examples/quest` turns it on with `PHIQUEST_KINDS`; `reparse` reads
a one-kind page against its own kind. 1 new test (31 in phiecs, all passing).

## 2. On the synthetic oracle

`PHIQUEST_KINDS=is-a,near,used-for,part-of`, `w0`, depth 8: 908 questions,
764 rows. `bin/kind_operator` on the result, 30 epochs, 144 held out:

| | held out MRR | held-out is-a (pool / within) | law hops is-a |
|---|---|---|---|
| prior | 0.129 | 0.066 / 0.066 | — |
| operator | 0.166 | 0.056 / 0.059 | 0.293 |
| operator + closure | **0.218** | **0.319 / 0.487** | **0.592** |

`near ⇒ is-a` is found at 100% of 90, nothing else above 28%: the quest
built one kind at a time recovers the same law and the same inference as the
full synthetic table (docs/93), from 60% of its rows.

## 3. For Phi-4

```
$env:PHIQUEST_OUT = "..\phiano\data\quest\phi4_laws"
$env:PHIQUEST_KINDS = "is-a,has,part-of,made-of,used-for,does"
cargo run -p phiecs --release --example quest -- cat 2 400 ollama run phi4
```

then `bin/kind_operator` on `phi4_laws/relations.tsv`, with
`PHIANO_LAW_MIN=0.3` as well as the default: laws in language (a cat has
fur because a mammal has fur) are softer than the synthetic one, and Phi-4
names the same thing in different words. What rate they hold at is the
measurement.
