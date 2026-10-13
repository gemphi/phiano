# 80 · Impersonation and composition through the generator

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. A persona is a set of
conditions; composing as one is generating under them — which, until this
round, the `impersonate` and `compose` paths did not do.

## What was wrong

`Composer::compose` (`src/compose/flow/compose.rs`) was four fixed sentence
frames — "In the beginning, the narrative opens with X and Y." — with words
from the river flow's sector banks filled in. Every `compose`, every
`impersonate`, the REPL and the server all rode on it; the tournament in
`compose/better` ranked 64 fillings of the same four frames. docs/79 §2.1
lists it as the first gap.

## What replaced it

`Composer::compose` now generates: the prompt, then one segment of 8 tokens
per sector on the river's path, each segment produced by `Generator` with
that sector's bank word as its goal (`with_goal`, pull 1.0) and the last six
tokens as context — the words move under the sector as the condition — and
a closing segment pulled toward the most resonant word. Three flows from
"the morning was" on the Austen lexicon take under 50 ms.

`Impersonator::impersonate_with` composes six candidates with the generator
conditioned on the persona (`with_persona_examples`, sector weight 0, pair
evidence 1 — the setting `bin/persona_eval` measured best), scores each by
the persona's likelihood of the candidate's own fingerprint and the
evaluator's quality, and returns the best. It no longer trains the lexicon
on its own output, and its `fit` is now computed in the same sector
coordinates as the fingerprint (the old one compared a legacy sector with a
centred one). `impersonate` (fingerprint only, no examples) uses the sector
prior alone.

## Measured: `bin/impersonate_eval`

Shared three-author lexicon, the three personas as `bin/austen` builds them,
12 prompts (4 held-out sentences per author, first three words), judged by
`World::match_text` on the composed text alone; Kneser-Ney on Austen's
training split for fluency.

| | read back as asked | KN perplexity (Austen persona) | distinct |
|---|---|---|---|
| before — four frames | 39% (Carroll every time) | 4,478 | 0.53 |
| **after — generator** | **56%** | **23.6** | 0.86 |

Per persona after: Austen 92%, Carroll 58%, Chesterton 17% (read as Austen 9
of 12 — the judge's bias, docs/79 §3). Chance is 33%. The before row's KN
perplexity is that of a template that no author would write; the after row is
Austen's own held-out range under her own judge.

Samples for *chapter 11 the*: Austen — "whole of the most extraordinary
conduct is felt by every thing that was not without strong hopes of her own
room again this evening shews what she had been very much obliged to be done
without any compromise of propriety"; Carroll — "same thing with you very
much the most extraordinary errand that it was his first thought that she had
been long enough for the other two were using it as well say added the queen
was silent for many years".

## What remains

Chesterton is the persona the judge cannot see (17%): his fingerprint sits
between the other two (docs/76 similarity 0.98 with Austen). The judge itself
is the next thing to fix — a cross-author Kneser-Ney panel, or attribution
trained to discriminate rather than to model — before the conditioning can be
tuned against it. And the sentence frames are gone, but the manifold is still
a bag of words without punctuation (docs/79 §2.2).
