# 87 · Which channels a reply reads, and whether it reaches its goal

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. A dialogue turn is that
sentence in miniature (docs/81): the question names a condition, the persona's
manifold says what that condition calls for, and the reply is generated toward
it. This note asked which channels "calls for" should read, and found first
that the reply never got where it was sent.

## 1. The switch

`Facet::calls_for` averages the directed reading over the eight **offset**
channels only; the 24 **marker** channels a corpus teaches (`of`, `with`,
`to`, `her` …) and the kinds a quest teaches (`is-a`, `used-for`) never
entered a reply. `Facet::calls_for_with(word, goal, Reading)` reads
`Offsets`, `Markers` or `All`, and both call sites use it: the dialogue's
choice of what the topic calls for (`persona/dialog/reading.rs`) and the
generator's pull toward that goal (`Generator::goal_bonus`).
`PHIANO_READING=offsets|markers|all`; the default is `offsets`, unchanged.

## 2. The measurement, and what it found first

`bin/persona_chat` with `PHIANO_CHAT_SCRIPT=long`: eighteen questions to each
of three personas, 54 replies per setting, the same two judges as docs/81
(the engine's sector attribution; a Kneser-Ney panel, one model per writer),
and a third count — how many replies contain the word their topic called for.

| reading | goal chosen differs from offsets | sector | KN panel | reached goal |
|---|---|---|---|---|
| offsets (shipped) | — | 50% | 78% | **0/54** |
| markers | 54/54 | 52% | 83% | 1/54 |
| all | 48/54 | 44% | 83% | 2/54 |

The readings choose entirely different goals (`love ⇒ regard` under offsets,
`love ⇒ spoke` under markers), and the judges cannot tell them apart —
because no reply reaches its goal. The pull (`DIALOG_GOAL_PULL` = 1.5) is added
to a candidate's score, and the candidates are the trigram and bigram
continuations of the last words, cut to 12 and 16. A goal is a rarer content
word than any of them, so it is never a candidate and the pull has nothing to
act on. docs/81 said the reply is "generated toward" the goal; it was generated
*near* it, through `calls_for` on the other candidates, and never arrived.

## 3. The fix: propose the goal where a bigram licenses it

`Generator::goal_propose` adds the goal to the candidate list whenever the
corpus has seen it follow the current last word (the bigram count is its
count), and only then; no transition the corpus never made is invented. On by
default; `PHIANO_GOAL_PROPOSE=0` turns it off.

| reading | sector | KN panel | reached goal | quality | fit | repeat |
|---|---|---|---|---|---|---|
| offsets, before | 50% | 78% | 0/54 | 0.713 | 0.780 | 22.7% |
| **offsets, goal proposed (shipped)** | 46% | 81% | **12/54** | 0.714 | 0.780 | 21.4% |
| markers, goal proposed | 54% | 87% | 11/54 | 0.716 | 0.781 | 13.7% |
| all, goal proposed | 48% | 80% | 11/54 | — | — | — |

A fifth of replies now reach their goal, at no measured cost to either judge
or to the evaluator's quality. What reaching looks like:

> **G. K. Chesterton**, *and what of love* (love ⇒ smiling) · And in all this
> time he was not only to be said father brown with level and **smiling** head
> did not even in that he said to be.

> **Lewis Carroll**, *what should a young woman do with her afternoon*
> (afternoon ⇒ hot) · Began to be sure said her sister that there was nothing
> more than his first thought was **hot** he added with the same thing as if it
> was.

## 4. Six seeds each

One run could not separate the readings, so `PHIANO_DIALOG_SEED` now offsets
every generator a dialogue runs (default 0 = the shipped replies, unchanged),
and each reading was run over seeds 0–5, 54 replies each, goal proposal on:

| reading | seeds | sector | KN panel | reached goal |
|---|---|---|---|---|
| offsets (shipped) | 6 | 51.5% ± 3.2 | 84.6% ± 3.0 | 10.8 ± 1.7 of 54 |
| markers | 6 | 48.8% ± 3.4 | 82.1% ± 4.0 | 11.0 ± 2.3 of 54 |
| all | 6 | 49.1% ± 6.7 | 84.6% ± 3.8 | 11.7 ± 1.2 of 54 |

The readings do not differ. Paired by seed, offsets minus markers on the KN
panel is −3, 7, 0, 1, 2, 1 replies (mean 1.3, sd 3.3), and the single-run lead
markers showed in §3 (87%) was one draw. Which channels choose the goal
changes *which* word a reply is sent toward, and neither judge can tell. The
default stays `offsets`. The spread across seeds is itself worth recording:
the same setting ranges 81–89% on the panel, so any dialogue comparison built
on one run of 54 replies can move by eight points without anything changing.

## 5. What remains

12/54 is a floor set by the bigram licence: a goal the corpus never saw after
the current word still cannot be reached, and reaching it by a two-step path
(a word the corpus has seen before the goal) is the next step and is not
built. The readings may matter where the channels differ more than a
corpus's function-word markers do from its offsets — a quest's `is-a` is the
obvious case, and needs a quest ingested into a persona's lexicon first.
