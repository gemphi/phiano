# 123 · Assessment: what we can demo, what we cannot claim

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Written 2026-10-03 after docs/107–122. Every number is from a run in this record; nothing is projected.

## 1. The claim, and how much of it is backed

**The claim.** A relation model at about 4% of Phi-4 re-creates Phi-4's abilities (generation, chat, coding) by observing it, keeping relations rather than data, with the number of relations per word as the one size knob, and it can learn new things.

| part of the claim | state | evidence |
|---|---|---|
| observes Phi-4 and keeps relations | **backed** | night quest 3,630 relations (docs/99–101); builders' game 400 calls × 3 runs (docs/107–115); 253 code answers + 72 glosses (docs/117–122) |
| learns language in action from Phi-4 | **backed, narrow** | predicts Phi-4's play at 89.5%, bag of words 89.7%, our own rules 88.2% (docs/115); learns a word from one pointing, which Phi-4 itself cannot keep in the game (docs/116) |
| coding: a known task in a new type | **backed** | 85% working functions, Phi-4 90%, on 248 tasks with audited checks; the student corrects Phi-4's commonest slip in 13 tasks (docs/122) |
| coding: a task never seen | **weak** | 27% against Phi-4's 90% (docs/121) |
| reading code: next token | **backed, small margin** | 60–62% top-1 against the 4-gram's 56–57%; on Phi-4's code 45–57% against 42–50% (docs/110, 118) |
| generation and chat | **not started** | nothing measured |
| the size knob | **shown once, on code reading** | R = 2 relations per condition already beats the 4-gram (docs/110); not shown on writing or on the game |
| about 4% of Phi-4 | **not a meaningful number yet** | what we have is 12,300 tokens of counts plus 253 answers; it is tiny, but it is not a model of Phi-4's language, only of 83 task families |
| learns new things | **backed on words, not on skills** | a new colour-word from one pointing; a new type of a known function; not a new algorithm |

## 2. What the coding student is

It is a case-based system over Phi-4's answers, composed by relations:

- the same task in another type (the answer said again, with the type exchanged);
- proportion between names (as `sum_even` : `sum_odd`, so `count_even` : `count_odd`);
- proportion between types (as `max(i32)` : `max(f64)`, so `min(i32)` : `min(f64)`);
- glosses told by Phi-4 (`above ⇒ >`);
- a vote among the routes, then the first body that compiles.

It is honest to call this a relation model in the project's sense: nothing is stored but told answers and the edits between them, and every new body is a composition of relations under a condition. It is not honest to call it a language model, and it does not generate anything it has no neighbour for.

## 3. What is demo-ready

Both run on the user's machine with no GPU and no Phi-4, in under ten minutes.

1. **`scripts\demo_learning.ps1`** (docs/107 §4): the squares reader at 100% in a second; the builders' game learned from scratch; the student predicting Phi-4's play. About 7 minutes.
2. **`scripts\demo_code.ps1`** (new): six functions the student was never shown, each written from Phi-4's answers, with the routes, the votes, compilation and a test printed. Strong moments:
   - `sum_above(i64)`: the student's vote chooses `filter(y > x).sum()` (13 votes) over Phi-4's own slip `…map(y * 2).sum()` (12 votes); the test passes for the student and fails for the teacher's body.
   - `count_zero(f64)`: proportion between types turns `x == 0` into `x == 0.0`, which is what makes it compile.
   - `max_of(f64)`: 23 votes from three kinds of route agree.
   - `sum_negative`: **the student fails**, and the demo shows why. Phi-4's gloss `negative ⇒ -` produces `x - 2 != 0`, which outvotes the right proportion `x < 0` (3 votes, test passes). Shown as is; a one-token gloss is too narrow, and the expression quest replaces it.

   The demo prints the failure. That is deliberate: the audience should see the vote, not a curated success.

## 4. What should not be said in a demo

- Not "it beats Phi-4". It is 5 points behind on the test it does best on, 63 behind on the other, and the tasks are ours.
- Not "4% of Phi-4". No measurement supports a ratio.
- Not "it codes". It writes one-function answers in families it has watched. The 18 string tasks of one variant each are where it fails on the easy test.
- Not "general". The checks were fooled once (docs/119: 200 coincidences, fixed by audit). Any new family needs its check audited the same way.

## 5. The shortest path to a stronger demo

1. **Phi-4's expressions** (`code_quest --exprs`, one unattended run): should lift the never-seen test from 27% and fix `sum_negative` in the demo.
2. **Siblings for the string tasks**: a `Vec<char>` or `&[u8]` variant of each, so the by-task test covers them. Expected to take the by-task score from 85% toward Phi-4's 90%.
3. **One generation task**, measured the same way (a held-out family of short prose completions from Phi-4), so the claim's first word has a number.

`scripts\demo_code.ps1` · `code_write --ask "<signature>" --test "<asserts>" --phi4 data\phi4_code`
