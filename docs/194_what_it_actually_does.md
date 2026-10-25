# 194 · What the system actually does — real runs, verbatim

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Every line of output below is pasted from a run on 2026-10-25, unedited. Nothing in this page is hand-written where a number or an answer appears. The commands reproduce each section exactly.

## The claim, stated at its true size

The English book (Murphy, 22,197 sentences) was told to the phasor engine sentence by sentence, online, in 3.6 seconds, with no retraining. After that the system can (a) predict the next word of unseen text from the same book at 13.9% top-1 (the counts contrast does 21.4% — **the phasors lose to counts on prose**), (b) continue a prompt in the book's register for a few words before degrading, and (c) complete 15.7% of the Key's answer words without ever being told the Key (counts: 20.1%; docs/193). It does **not** understand grammar, hold a conversation, answer questions with answers, or learn English in another language. `talk` is continuation, not dialogue.

## 1 · Talking to the book — `talk` (continuation in the learned voice)

    cargo run --release -p phiano --example talk -- crates/phiano/data/person/murphy.txt --words 14 "i have been waiting" ...

Output, verbatim:

    22197 sentences told to the phasors in 3.6 s — on the fly, nothing retrained.

    Q: i have been waiting
    A: [i have been waiting] for the job of the to in the of

    Q: if i had known
    A: [if i had known] i wish t the any the every the

    Q: when do we use the present perfect
    A: [when do we use the present perfect] continuous past am to you a me from the box of chocolates

    Q: she has lived in london
    A: [she has lived in london] of the to in the of

    Q: do you think it will rain
    A: [do you think it will rain] ll t the any the every the

    Q: he said that he
    A: [he said that he] is t the any the every the

Read it honestly: the first three or four words after the prompt are in the author's voice — "waiting **for the job**", "if i had known **i wish**", "the present perfect **continuous past**" — and then every answer decays into the same fallback loop ("t the any the every the"). The construction is held; the sentence is not. That decay is the open problem (the fallback channels), and it is visible in every single answer above.

On Crick (2,557 sentences, 0.5 s):

    Q: it seems likely that
    A: [it seems likely that] cause the is the to use the flex and bison to be the for

## 2 · How it was learning — `person` (the curve, predict-first)

    cargo run --release -p phiano --example person -- crates/phiano/data/person/murphy.txt --chunks 8

Every next word is predicted BEFORE its chunk is told (prequential — the model never scores text it has already seen). 154,122 positions, floor 0.02%.

| chunk | counts | concepts | **phasors** |
|---|---|---|---|
| 1/8 | 17.2% | 20.0% | **14.9%** |
| 2/8 | 18.4% | 22.4% | **15.9%** |
| 3/8 | 18.6% | 22.8% | **15.7%** |
| 4/8 | 16.6% | 20.8% | **13.4%** |
| 5/8 | 18.1% | 22.4% | **13.4%** |
| 6/8 | 17.2% | 21.1% | **13.6%** |
| 7/8 | 18.0% | 23.2% | **13.3%** |
| 8/8 | 14.7% | 18.4% | **11.1%** |

Overall: counts 21.4%, phasors 13.9%; phasors first-to-last **−3.8 pts**. The example's own verdict line, verbatim: "NOT MET — reported straight." The learning is real and fast from chunk 1 (14.9% vs a 0.02% floor, nothing retrained), but it saturates and then slips as the book's later units differ from the early ones. On the glosses corpus the same engine LEADS counts (32.9% vs 29.9%, docs/190); the prose gap is the engine's current frontier, not a rounding story.

Continuations after each chunk show the voice forming — after chunk 1, "we use" → "we use **the present** of the book of"; the book's own phrase "the present" is picked up within the first 2,775 sentences.

## 3 · The workshop over the book — docs/193

129 units read in order; before each unit the student is tested on that unit's sentences (brought = what earlier units already gave it), then told the unit; after each unit it completes that unit's Key answers word by word, never having been told the Key. Whole course: brought 11.6% (14,697/126,259); **Key completion: phasors 15.7% (1,651/10,544), counts 20.1% (2,123/10,544)**. Per-unit pages with the prompts, the compiler tasks (every page's BCPL program judged by phicpl and verified equal), and both scores are in `docs/193_workshop.md` (65 pages).

    cargo run --release -p phiano --example workshop -- crates/phiano/data/person/murphy.txt --key crates/phiano/data/person/murphy_key.txt --pages 64 --out crates/phiano/docs/193_workshop.md

## What is demonstrable today, and what is not

Demonstrable, by command: online learning speed and the honest curve (`person`); continuation in a learned voice (`talk`); the course + Key grade (`workshop`); five models in one table (`arena`); the compiler running real BCPL (`phicpl`, `examples/run.rs`); the judge-taught writing loop (`writer`). Not demonstrable: dialogue (no agent exists), comprehension, second-language learning, or phasors beating counts on prose. The condition of satisfaction being served — learn a person from a little corpus, on the fly, keep learning, talk to it — is met on "learn on the fly, keep learning" and unmet on "talk to it" beyond continuation.

## Status

2026-10-25. talk/person/workshop outputs captured verbatim; workshop split into `examples/workshop/{main,key}.rs`; Key shipped as `data/person/murphy_key.txt`. Next rung, in order of what the numbers say: the fallback decay in generation (every `talk` answer shows it), then the prose gap.
