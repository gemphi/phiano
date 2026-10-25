---
id: descriptive-logic-5
title: "Descriptive Logic V: Following a Rule"
description: Philosophical Investigations §§134–242 read in Phient's terms, with Phi-4 as the other player in the builders' game, three runs, and students who learn its practice.
sidebar_position: 23
badge: paper
---

# Descriptive Logic V: Following a Rule

## Philosophical Investigations §§134–242, in Phient's terms

**Phiano Project** · `bin/language_game` (`ask`, `follow`, `graded`, `compare`) · `bin/code_reader` · 2026-10-01

> In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Chapters I–IV played the builders' game with teachers we wrote, then read
Phi-4's own relations as rough ground. This chapter puts Phi-4 *in* the
game as the other player. It is told the set-up (the stones, the letters
used as numerals, the places, the form of an answer) and never the rules.
Then it is asked, call by call, what it does.

§§134–242 ask what it is to follow a rule. When does a continuation count
as *going on in the same way*? Where does understanding show itself? And
why can no interpretation fix a rule by itself? In our project these
questions have numbers. Phi-4 was asked the same 400 calls in three
settings:

1. **sampled:** Ollama's default sampling;
2. **temperature 0:** always the likeliest answer;
3. **temperature 0 with the roles stated:** the scene also says who
   fetches and who only takes note.

Students then learned to go on as Phi-4 does.

As before, Wittgenstein is paraphrased and cited by remark, never quoted.

---

## 1. Four more definitions

- **D17 · Practice.** A teacher's answers form a *practice* when the same
  call gets the same answer. Its measure is *steadiness*: the share of
  pairs of answers to a repeated call that are identical (§§198–199).
- **D18 · Going on differently.** A divergence from our rule is *going on
  differently*, not error, when it follows the words of the call: it can
  be predicted from them, and it repeats. Divergence that follows nothing
  is noise (§185).
- **D19 · Reading, not interpreting.** A teacher's notation is read in
  the game's terms only where the call gives no word for the aspect (`-`
  for no colour is the game's `any`). Every further reading would be one
  more interpretation of a rule by a rule (§§198, 201).
- **D20 · Agreement in judgments.** Teacher and game agree in judgments on
  an aspect when, read as in D19, the teacher's answers are the game's in
  at least 95% of calls. That agreement is not given by any definition
  the prompt contains (§242).

## 2. The data

Each cell is one run of 400 calls (`data/builders/`), read by `follow` and
compared by `compare`:

| run | replies read | exact (game's terms) | act | stone | colour | count | place | steadiness (D17) |
|---|---|---|---|---|---|---|---|---|
| sampled | 369 | 50.7% | 63.1% | 94.9% | 96.2% | 89.7% | 89.7% | 35.8% (95 of 265) |
| temperature 0 | 393 | 66.4% | 74.3% | 98.2% | 100% | 94.1% | 96.4% | 88.9% (295 of 332) |
| temperature 0 + roles | 380 | **88.2%** | **99.5%** | 97.4% | **100%** | 93.7% | **97.1%** | **100%** (276 of 276) |

Students learned Phi-4's practice from four fifths of the calls and
predicted its answers on the fifth (folds by call, so a repeated call is
never on both sides). The exact match:

| student of Phi-4 | sampled | temperature 0 | t0 + roles |
|---|---|---|---|
| our rules, as a predictor | 50.7% | 66.4% | 88.2% |
| bag of words + softmax | **54.2%** | **70.5%** | **89.7%** |
| counts, D2 (τ = 0.5) | 46.6% | 60.3% | 85.3% |
| counts, τ per aspect, chosen in training (docs/111) | 49.3% | 65.1% | 89.5% |
| counts, two-speed posts (docs/113) | 47.2% | 70.2% | 84.5% |

## 3. The remarks, in Phient's terms

### §§134–142 — The general form; what fits

**The remarks.** "This is how things are" looks like the general form of
a proposition. But it is itself a sentence that belongs to our language,
and its use is shown by how it is used (§§134–136). Whether a word *fits*
a sentence is shown by the use, not by a picture that comes before the mind
when the word is understood (§§138–141). Such pictures could suggest
other uses. Normal cases make the use clear; abnormal cases make it
unclear what to say (§142).

**In Phient's terms.** Phi-4 knows the words. It names the stone in
94.9–98.2% of calls in every run, and the colour in 96.2–100%. What fits
in the slot is not the problem. The problem was the act. In the first two
runs Phi-4 knew what *slab* means and still did not know what to *do*
with it. The word fitted; the use did not.

### §§143–155 — The series, and "now I can go on"

**The remarks.** A pupil is taught to write a series. At some point he
can continue it himself, and we say he has understood the system (§§143–146).
Understanding looks like a state from which the right use flows. But the
criterion for it is the use itself, not an inner state (§§146–150). When
someone says *now I can go on*, no process need be going on in him. The
words are justified by the circumstances (§§151–155).

**In Phient's terms.** The code reader (docs/109–110) is a pupil of the
series. Given 367,000 tokens of our Rust, it is asked to continue each
file where the text stops. Its *understanding* is nothing but its
continuations: 60.0% / 62.0% of next tokens exactly right on two splits of
unseen files, against the 4-gram's 55.7% / 57.2%. There is no state in it
apart from the counts and the weights. The weights say that what it adds
to the 4-gram is the context four to eight tokens back. That is where its
"going on" comes from.

### §§156–184 — Reading, and being guided

**The remarks.** Reading machines are taken as an example. The criterion
for whether someone *reads* is what they do, not an inner experience of
deriving the words from the signs (§§156–171). Being guided, and the
"because" of reading, is a family of cases, not one experience
(§§172–178). *Now I can do it* is not a report of a state either
(§§179–184).

**In Phient's terms.** Phi-4's replies are read by `builders::parse`. In
the first run the parser was the bottleneck: 31 of 400 replies were
unread, because of terminal escapes, markdown table rules and letter
numerals. Whether Phi-4 *read* the call is shown only by the replies we
can read. Once the parser was fixed, 393 and 380 of 400 replies were read
in the later runs. The 7 and 20 left have no line in the form that was
asked for.

### §§185–190 — The pupil who goes on differently

**The remarks.** A pupil has learned to add 2, and is right up to 1000.
Asked to go on past 1000, he writes 1004, 1008. Told he is wrong, he says
he went on *in the same way*. He understood our order the way we would
understand *add 2 up to 1000, 4 up to 2000*. No explanation we gave
excluded that reading in advance (§185). What we meant was not a mental
act that ran ahead through all the steps (§§186–188). How the steps are
determined is shown by how we were trained and how we use the formula
(§§189–190).

**In Phient's terms.** In the first two runs, Phi-4 was this pupil.
Called to *bring*, it answered *record* in 85 orders (sampled) and 76
(temperature 0). It also answered *bring* to 25 reports. The divergence
follows the words (D18). In temperature-0 orders that name a colour,
Phi-4 never answers *record* (0 of 136). When the order names a slab or a
pillar, it answers *record* about 45% of the time. When the call begins
with *slab*, it answered *record* 9 times out of 9.

So this is not noise; it is going on differently. Phi-4 read *A calls
out to you* as compatible with taking note, and the prompt never excluded
that. The further explanation, *A tells you what to do and you do it; you
do not take notes*, did exclude it. Divergence on the act fell from 101
calls to 2, and the act rose to 99.5%. As in §185, the first explanation
was not wrong. It left open a way of going on that we did not mean, and
the second explanation closed it.

### §§191–197 — The rails laid in advance

**The remarks.** It can seem that the steps are already taken, in a
queer way, when we grasp a rule at a stroke (§§191–197). The use seems
present all at once. But that "all at once" is a picture, and its use is
our ordinary use.

**In Phient's terms.** A Phient word has no rails in it. It is 32–64
relations with counts. Its future uses are not stored; they are
recomposed each time from the relations that hold a post under the call
(D2, D4). What looked like Phi-4's grasp of *slab* at a stroke did not
contain the act.

### §§198–202 — Signposts, custom, the paradox, and privately

**The remarks.** A rule stands like a signpost. Whether it leaves room
for doubt depends on how it is used (§198). Following a rule is not
something one person could do only once. It is a custom, a use, an
institution (§199). Here is the paradox: no course of action could be
fixed by a rule, because any course can be made to accord with it by some
interpretation. The answer is that there is a way of grasping a rule that
is not an interpretation, shown in what we call obeying it and going
against it in actual cases (§201). Therefore following a rule is a
practice, and one cannot follow a rule privately (§202).

**In Phient's terms.** D17 measures the custom. The sampled Phi-4 gave
the same answer to the same call only 36% of the time: there was no
practice to learn, and no student beat 54%. At temperature 0, steadiness
rose to 89% and the best student to 70%. With the roles stated it reached
100%, and the best student 89.7%. A rule that is not kept twice cannot be
taught.

D19 is our guard against the paradox. Phi-4 writes `-` where our game
says `any`. We read `-` as `any` *only* where the call had no colour-word.
That one reading takes exact match from 59.7% to 88.2% in the roles run.
We did not let further readings rescue other divergences. With enough
readings, every answer could be made to accord with our rules. The
divergences are left standing, and the students learn them: on the roles
run the best students (89.5–89.7%) predict Phi-4 *better* than our own
rules do (88.2%).

### §§203–216 — Obeying an order; training; the same

**The remarks.** Following a rule is like obeying an order. We are
trained to it and react to it in a particular way (§206). Imagine people
of an unknown language: the common behaviour of mankind is the system of
reference by which we interpret it (§206). Explanation by examples is not
second-best. The examples, with gestures and a *and so on*, are the
explanation (§208). *The same* is not given by a table of identity that
fits every case (§§215–216).

**In Phient's terms.** The students are trained on examples only: no
rules, no prompt, just calls and what followed. The student that does
best depends on the teacher:

- On the teacher of **tendencies** (temperature 0, no roles), the
  two-speed posts did best among the counts students (70.2%, against bag
  of words at 70.5%). They let a word that has been seen often learn its own
  weight, so `slab` and `pillar` acquired their lean toward *record*.
- On the teacher of **rules** (roles stated), τ chosen per aspect in
  training tied with bag of words (89.5% against 89.7%), and the two-speed
  posts only added noise (84.5%).

What counts as *the same* step differs between the two teachers, and the
students show it. No single setting fits both.

### §§217–221 — Bedrock; obeying blindly

**The remarks.** If I have exhausted the justifications, I have reached
bedrock: my spade is turned, and I say *this is simply what I do* (§217).
The rails invisibly laid to infinity are only a picture (§218). When I
obey a rule I do not choose. I obey it blindly (§219).

**In Phient's terms.** For a Phient student, the reasons give out at the
counts. *Why bring?* Because `slab` holds a post under the act and moves
it toward *bring*. *Why that post?* Because its counts move the answer by
more than τ. *Why that τ?* Because, on these calls, that τ answered best in
the training folds. Below that there is nothing but the told episodes.

Phi-4 at temperature 0 with the roles stated obeys blindly in the
measurable sense: 276 of 276 repeated calls get identical answers.

### §§222–237 — Agreement and rule are cousins

**The remarks.** The word *agreement* and the word *rule* are related,
cousins (§224). To teach someone the use of one is to teach the use of
the other. What *the same* means is learned with the rule (§§225–227).
The rule can seem to tell us what to do as if from outside, but we follow
it as a matter of course (§§228–237).

**In Phient's terms.** Agreement and rule are measured by the same
numbers here. *Phi-4 follows our rule* is the first row of the student
table, and *a student follows Phi-4's practice* is every other row. On the
roles run, our rules agree with Phi-4 at 88.2%, and Phi-4's practice,
learned by a student, agrees with Phi-4 at 89.7%. The residue is 11.8%
divergence from our rules. Most of it is notation:

- *i* counted as 10 (13 calls);
- the colour written into the stone, as in `yellow block` (10 calls);
- a place `here` where the rule has none (11 calls).

### §§238–242 — Agreement in judgments, and in form of life

**The remarks.** For the rule to seem to produce all its consequences in
advance, they must be a matter of course to me (§238). Mathematicians do
not come to blows over whether a rule was obeyed (§240). Whether people
agree is not decided by their opinions. They agree in the language they
use, which is agreement in form of life, not in opinions (§241).
Communication by language needs agreement in definitions and also, odd
as it may sound, agreement in judgments (§242).

**In Phient's terms.** D20 sorts the aspects:

| aspect | sampled | t0 | t0 + roles | D20 (≥ 95%) |
|---|---|---|---|---|
| stone (*what*) | 94.9% | 98.2% | 97.4% | agrees from t0 |
| colour | 96.2% | 100% | 100% | agrees in every run |
| place | 89.7% | 96.4% | 97.1% | agrees from t0 |
| count | 89.7% | 94.1% | 93.7% | not yet: `i` read as 10 |
| act | 63.1% | 74.3% | 99.5% | agrees only with the roles stated |

The prompt gives definitions: the stones, the colours, the letters as
numerals. Agreement in judgments on stone, colour and place came with
those definitions. Agreement on the act did not. It needed more of the
form of life: who fetches, who takes note. The prompt's definitions
could not supply that, and Phi-4's training had not.

The count shows the other side of §242. The prompt *defines* *i* as 9,
and Phi-4 still judges *i* as 10 in 13 calls. Definitions were given and
agreed to, but the judgments did not follow from them.

## 4. What Chapter V adds

- **Practice can be measured.** Steadiness went 36% → 89% → 100%, and
  what a student could learn followed it: 54% → 70% → 90%.
- **Going on differently is predictable.** Phi-4's *record* for *bring*
  followed the stone named and the presence of a colour-word. One more
  sentence of explanation, as in §185, took it from 101 calls to 2.
- **The student learns the practice, not our rules.** On the roles run
  the best students predict Phi-4 better than our rules do.
- **Which student is best depends on the teacher** (D16 again): two-speed
  posts for a teacher of tendencies, τ per aspect for a teacher of rules.
  Counts students keep one thing bag of words cannot do: learn a word from
  one pointing (Chapter I, sepia: 100% against 0%).

## 5. Postscript: Phi-4 taught by pointing (§§28–30, docs/116)

Phi-4 was taught `sepia` by one or three pointings, with or without the
word *colour*. It was then given 50 orders that use the word, at
temperature 0, with the roles stated:

| teaching | Phi-4: colour = `sepia` | Phi-4: act | Phi-4: exact | counts student: exact | student + Phi-4's reading |
|---|---|---|---|---|---|
| 1 pointing | 64.9% | 54.1% | 29.7% | 32% | 100% |
| 3 pointings | 90.0% | 48.0% | 34.0% | 32% | 100% |
| 1 pointing + *colour* | 63.6% | 59.1% | 27.3% | 100% | 100% |
| 3 pointings + *colour* | 90.0% | 24.0% | 16.0% | 100% | 100% |

Phi-4 learns the word: three pointings put it in the colour slot in 90%
of orders. But being taught moves it out of the game. Its act falls from
99.5% to 24–59%, and with three pointings and *colour*, 38 of 50 orders
are answered *record*. The scene of the teaching leaks into the scene of
the order: §29's question of which game a word is placed in, answered
wrongly.

Our counts student keeps the pointing in its own game (D3). It does not
need Phi-4's skill, only its reading. When Phi-4 takes the word as a
colour in most orders, the student is told so and keeps everything else
for itself. Exact is then 100% in all four teachings. This is the
division of labour for the 4% model: the word's aspect from the large
model, the game from the relation model.

## 6. Next

Next, the student learns from Phi-4's practice and from its night
relations together. That is the first step of distilling Phi-4 into a
relation model at the scale of the size knob.

*Code:* `phiano/src/bin/language_game/` (`teacher.rs`, `follow.rs`,
`graded.rs`, `weighted.rs`, `wordposts.rs`, `compare.rs`),
`phiecs/src/quest/builders.rs`, `phiano/src/bin/code_reader/`.
*Docs:* 107, 109–115. *Data:* `data/builders/phi4_seed7*`,
`phi4_t0_seed7*`, `phi4_t0_seed7_roles*`.

*Run:* `scripts\phi4_builders.ps1` (add `-Sampled` or `-Roles`), then
`language_game compare data\builders\runs.md data\builders\phi4*.tsv`.
