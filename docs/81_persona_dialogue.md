# 81 · Ingesting the personas, and talking to one

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. A persona is which
conditions a writer puts words under, aggregated — so a dialogue with one is
not retrieval. The question names a condition; the persona's manifold says what
that condition calls for; the answer is generated toward it.

## 1. Ingestion: all of it, once, saved

Until now personas were rebuilt on every run from **600 sampled sentences**
each (`bin/austen`), and the lexicon under them was trained on Austen's
training split plus the contrast authors' *whole books* — their held-out text
leaked into the manifold. `bin/persona_ingest` fixes both: every author is
split 80/10/10 by sentence hash (seed 7), the shared lexicon is trained on the
three training splits **only**, and each persona is ingested from its whole
training split.

| author | books | sentences | ingested from | held out | words |
|---|---|---|---|---|---|
| G. K. Chesterton | 3 | 11,219 | 8,975 | 1,122 | 162,752 |
| Jane Austen | 3 | 17,114 | 13,691 | 1,712 | 247,252 |
| Lewis Carroll | 1 | 1,378 | 1,102 | 138 | 18,305 |

Shared lexicon: 18,496 words, all with relation evidence, 24 marker channels,
three passes — 12 s. Persona ingestion — 8 s for all three. Total 53 s, written
to `data/personas.chroma` + `data/personas.json` (`PersonaStore`), which
`bin/persona_chat` opens read-only in about a second.

What the full ingestion bought, measured the same way both times — attribution
on held-out paragraphs of four sentences, 150 per author, the persona with the
highest likelihood wins:

| persona | sampled 600 | all sentences |
|---|---|---|
| G. K. Chesterton | 73% | 74% |
| Jane Austen | 52% | 65% |
| Lewis Carroll | 57% | **86%** |
| **all** | **62%** | **71%** |

Chance is 33%. Carroll gains most: 1,102 sentences is all he has, and sampling
600 of them was throwing away half his evidence.

## 2. The dialogue

`persona::Dialogue` (`src/persona/dialog.rs`) answers a turn in four steps.

1. **The condition.** The rarest content word of the question that the lexicon
   knows — a word the corpus uses everywhere names no condition in particular.
2. **What it calls for.** Over the persona's own 600 most-used content words,
   the one with the strongest directed reading from that condition
   (`Facet::calls_for`, X ⇒ Y). This is the persona's answer *word*, and it is
   the persona's: *love* ⇒ **regard** for Austen, ⇒ **smiling** for Chesterton,
   ⇒ **notice** for Carroll.
3. **The reply.** Nine candidates from `Generator`, conditioned on the persona
   (sector histogram and its own pair evidence, docs/79 §3) and pulled toward
   that word (`with_goal`). The nine start from three different places in the
   question — its tail, the condition alone, the condition with what it calls
   for — because nine walks from one context are one answer.
4. **The choice.** Best by the persona's own likelihood of the reply's
   fingerprint and the evaluator's quality, less what the dialogue has already
   said: a candidate repeating more than a quarter of its 4-grams is refused
   outright, and the least repetitive one speaks if none is fresh enough. The
   context wave carries across turns, so the sixth answer is spoken after the
   first five.

Nothing is templated, and the lexicon is never written: a dialogue does not
train the persona on its own words.

## 3. What it says

`cargo run --release --bin persona_chat` — all three personas, the same six
questions, 17 s. Two judges read every reply alone: the engine's own sector
attribution, and a Kneser-Ney trigram model per writer (`Panel`), trained on
that writer's sentences, least surprised wins.

> **you** · what do you think about marriage
>
> **G. K. Chesterton** · And since then went on with some surprise but much more
> than once brought home such accounts of the world is not to be said father
> brown was. <sub>under *marriage* ⇒ *sudden*</sub>
>
> **Jane Austen** · Days that in the very first subject was so much more than she
> had been long enough to be very well as you know what she had been.
> <sub>under *marriage* ⇒ *former* · KN panel: Austen, ×4.8</sub>
>
> **Lewis Carroll** · Tone herself up and began to be said the king said alice
> very much like keeping so close to her sister were happy together said the
> king and. <sub>under *marriage* ⇒ *melancholy* · KN panel: Carroll, ×9.4</sub>

> **you** · tell me about money and family
>
> **G. K. Chesterton** · Order may be said father brown was not only to say that
> he had been very well said syme with gravity of small sums of money on one.
> <sub>under *money* ⇒ *staring* · KN panel: Chesterton, ×15.2</sub>
>
> **Jane Austen** · In her own heart was not to be sure you must have been so
> much more than she had never seen him and her father was quite out.
> <sub>under *money* ⇒ *decided* · KN panel: Austen, ×5.6</sub>

> **you** · and what of love
>
> **G. K. Chesterton** · And in all this time he was not only to be said father
> brown with level and piercing eyes were still talking to him in such things
> that. <sub>under *love* ⇒ *smiling* · KN panel: Chesterton, ×6.4</sub>
>
> **Jane Austen** · For her father and sister were to be sure you will not like
> him in her eyes towards jane and emma could not be very much to say.
> <sub>under *love* ⇒ *regard* · KN panel: Austen, ×13.7</sub>

Father Brown and Syme turn up for Chesterton, Alice and the King for Carroll,
Emma and Jane for Austen — the persona conditioning is choosing the company
each writer keeps, not just the topic.

| over 18 replies | read back as the persona who spoke |
|---|---|
| the engine's sector attribution | 44% |
| Kneser-Ney panel, one model per writer | **72%** |
| chance | 33% |

Both are above chance and they disagree, which is the useful part: the sector
reader calls almost everything Austen (docs/79 §3 predicted exactly this — she
has the most text and the three fingerprints overlap at 0.98), while a
per-writer language model separates them at 72%. The engine's own attribution
is now the weakest link in the persona chain, not the generation.

## 4. What remains

A discriminative attribution reader — sectors weighted by how well they
separate writers rather than by how often each writer uses them — is the next
thing, and the KN panel is the yardstick it has to beat. Below that: the
replies still carry the trigram tables' idiom ("to be sure you will not like
it"), which decoding through the operator rather than the tables would loosen
(docs/78: 10% shared 4-grams against the sampler's 26%) — the operator in
`data/austen_operator.fno` is Austen's, so a shared-lexicon operator has to be
trained before the dialogue can use one. And the manifold still has no
punctuation or case (docs/79 §2.2), so no persona can be told by its commas.
