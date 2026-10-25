---
id: descriptive-logic-1
title: "Descriptive Logic I: Language in Action"
description: Philosophical Investigations §§1–32 read in Phient's terms, and the builders’ game played by five kinds of learner, measured.
sidebar_position: 19
badge: paper
---

# Descriptive Logic I: Language in Action

## Philosophical Investigations §§1–32, in Phient's terms

**Phiano Project** · `bin/language_game` · 2026-09-29

> In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

This chapter opens a series of papers called **Descriptive Logic**. It
reads the first thirty-two remarks of Wittgenstein's *Philosophical
Investigations* next to Phient's paradigm. Then it does what those remarks
keep asking for: it looks at language in action. We build the builders'
game of §2, extended as §8 extends it. We let five kinds of learner watch it. Then
we ask them to act.

Wittgenstein is paraphrased and cited by remark number (§), never quoted.
The translation is still under copyright, and the argument does not need
his words, only his moves. Where this chapter says "Wittgenstein says", it
means our reading of that remark, and the reader should check it against
the book.

---

## 1. What a descriptive logic is

A logic of meaning can be *prescriptive*: it says what a word must mean,
by definition, and then derives. Or it can be *descriptive*: it records
how a word is used, under which conditions, and derives only what that
record supports. §§1–32 argue, remark by remark, that the prescriptive
picture ("every word names an object") describes only one narrow system.
The work of meaning is done by use inside activities. Wittgenstein calls
such an activity a *language-game* (§7), woven into a form of life (§19,
§23).

Phient's paradigm is already a descriptive logic. We state it as five
definitions, each measurable from observed use. In each one, an *episode*
is something said, in some game, and what followed.

- **D1 · Use.** Under a condition *c*, a word *w* relates to what follows.
  Its meaning under *c* is how it moves the answer: from *P(· | c)* (what
  the condition alone says) to *P(· | w, c)*.
- **D2 · Post.** *w* holds a post under kind *k* when that move is large
  enough: KL(*P(·|w,k)* ‖ *P(·|k)*) ≥ τ nats, with τ = 0.5. The grammar of a
  word is the list of its posts, the "place in grammar" of §29.
- **D3 · Game.** A condition is an aspect of the action inside a game.
  Two games *agree* on an aspect when the words that hold a post there
  point to the same answer in both. A word can be carried from one game
  into another only where the games agree.
- **D4 · Composition by movement.** The act is the value *v* that
  maximises ln *P(v|c)* + Σ ln *P(v|w,c)* / *P(v|c)*, summed over the words
  of the utterance that hold a post under *c*. The condition supplies what
  is not said, and each word contributes only its move.
- **D5 · Naming**, as a special case. Some words hold exactly one post and
  move it to exactly one value, and the game never changes. Such a word is
  a name. The Augustinian picture of §1 is the descriptive logic in which
  every word is D5 and there is one game.

Everything in D1–D4 is counted from episodes, so nothing is prescribed. The
declared laws of the personas are the one prescriptive part of Phient, and
docs/101 shows what happens to them. They are claims about the world, and
the teacher's answers support some and not others.

## 2. The game

`bin/language_game`. A builder, A, and an assistant, B, as in §2. The
stones are block, pillar, slab and beam. §8 adds:

- numerals, the letters a–j;
- `there`, with a pointing at one of five places;
- `this`, with a pointing at a stone;
- three colour words.

§27 adds `help`, `away` and `no`, which call for an act rather than name a
thing. §21 adds a second game with the same sentence forms: a *report*, in
which B tells A what is there and A records it.

An episode is the tokens said (pointing gestures appear as tokens, e.g.
`⟨point:p3⟩`), the game, and what followed. What followed is an action in
five aspects: **act · what · colour · count · place**. For example, `d red
slab there ⟨point:p2⟩` in the order game is followed by *bring · slab ·
red · 4 · p2*. No learner is ever told what a word means.

We generate 3,600 training episodes (3,000 orders, 600 reports). Some
cases are held out of training:

- numerals g and h with beams, blue pillars, and slabs brought to p5 —
  these are new combinations;
- pillars, beams and `this` in the report game;
- the word `sepia` everywhere.

Five tests follow from this:

| test | remark | what it asks |
|---|---|---|
| seen | §2, §8 | new episodes of kinds that were trained |
| new combinations | §8 | numeral × stone × place pairings never heard together |
| report | §21 | stones never reported, in the report game |
| no name | §27 | `help`, `away`, `no` |
| sepia | §28–30 | a colour word met only by pointing (once or three times, with or without the word `colour`), then used in orders |

## 3. The learners

1. **Lookup.** Remembers whole utterances. Unseen utterances get the game's
   most frequent action.
2. **Naming table (§1).** Each word names the one object it co-occurs with
   most, found cross-situationally. A sentence is a combination of names.
   The act is always *bring*. There are no games and no posts, because
   Augustine's picture has none.
3. **Bag of words + softmax.** Per aspect, multinomial logistic regression
   over the tokens and the game. This is the standard discriminative
   answer, and the strongest baseline here.
4. **Phical counts.** For every relation `(word, aspect|game) ⇒ value`,
   the count. There is no operator. Two compositions are compared: summed
   (each word votes with its firmness) and movement (D2–D4).
5. **Phient relations (phinet).** The same relations, each kept once with
   its count: 1,585 relations from 3,600 episodes. They are learned as
   functions on the circle by one phinet operator, with R relations per
   word, and read out with the same two compositions. Counts decide the
   posts and read the condition; the operator gives the words' readings.
   A two-speed variant reads a relation from its count until it has been
   told N times (§5).

## 4. The remarks, in Phient's terms, and what the game showed

### §1 — The Augustinian picture, and the shopkeeper

**The remark.** A picture of language: every word names an object, and
sentences are combinations of names. Wittgenstein then gives a shopkeeper
a slip reading "five red apples". The shopkeeper looks up *apples* in a
drawer, compares *red* with a colour chart, and counts up to *five* while
taking apples out. Three words, and three quite different things are done
with them. Asked what *five* means, Wittgenstein answers only by how it is
used.

**In Phient's terms.** Phical stores no point. A word is not attached to an
object. It is what it relates to under conditions. The shopkeeper's three
procedures are three *kinds*: the same slip, read by kind. This is D5, and
it is D5's limit.

**What the game showed.**

- The naming table is perfect on things: 100% on seen episodes and on new
  combinations, on both seeds. Where only objects are at stake, the
  picture works.
- It scores 0% on reports and 0% on `help`/`away`/`no`, on both seeds.
  Nothing in the picture can make the same words do something else, and
  nothing in it can make a word do something rather than name something.
- Over the training curve it plateaus at **64%**: the share of the tests
  that are about objects only. That ceiling is the picture's own.

### §2–4 — A complete primitive language, and the script

**The remarks.** For the builders' language, Augustine's description *is*
right (§2). It is a whole language, but only a narrow one (§3). Taking it
for all of language is like taking letters to stand only for sounds, when
some stand for emphasis or punctuation (§4).

**In Phient's terms.** In our game, the tokens do not all do the same kind
of work:

- `slab` names a stone;
- `⟨point:p3⟩` is a gesture;
- the game token is not said at all, yet it relates more firmly than any
  word to the act.

Every token is a row of the same table and a function on the same circle.
Only their relations tell them apart (§11 below).

### §5–7 — Training, and the language-game

**The remarks.** Children learn their first words by training, not by
explanation (§5). Pointing at a thing while saying a word does set up an
association. But what that association *does* depends on the rest of the
training, as a lever is a brake only when it is linked to the rest of the
machine (§6). The whole — the words and the actions they are woven into —
is the *language-game* (§7).

**In Phient's terms.** An episode is the language-game made data: tokens,
game, and what followed. The learners are trained and never told. Movement
(D4) is the lever linked to the machine. A word counts only through how it
moves the answer under a condition, and it has no meaning apart from that
move.

**What the game showed.** Episodes needed to reach exact match on all four
in-game tests (seed 7 / seed 8):

| learner | 80 | 160 | 320 | 640 |
|---|---|---|---|---|
| bag of words | 82.7 / 77.4% | 99.8 / 94.7% | 100 / 100% | 100 / 100% |
| Phical counts, movement | 50.8 / 55.4% | 69.8 / 70.4% | 96.6 / 92.7% | 98.7 / 97.4% |
| Phient (phinet, R = 32) | 43.0 / 52.6% | 54.5 / 65.1% | 77.9 / 90.2% | 99.4 / 99.0% |
| Phient (phinet, R = 8) | 44.4 / 47.0% | 44.2 / 62.6% | 82.4 / 86.7% | 94.1 / 98.1% |
| Phient two-speed (counts until 32 sightings, R = 32) | 50.8 / 55.4% | 69.8 / 70.4% | 87.0 / 92.7% | 98.4 / 97.4% |
| naming table | 60.9 / 58.6% | 64.0 / 61.9% | 64 / 64% | 64 / 64% |
| lookup | 21.9 / 20.7% | 25.8 / 23.8% | 30.7 / 29.6% | 35.6 / 35.6% |

The discriminative baseline learns fastest, and we say so plainly. Counts
come next. The phinet operator needs the most training, and it is
behind the counts below 640 episodes. The relations-per-word knob (R 8 vs
32) matters only once there is enough data (docs/100 found the same on
Phi-4's tables). Two-speed learning (§5) follows the counts while data is
scarce and the functions once it is not.
With all 3,600 episodes, all three reach 100% on every in-game test.

### §8–10 — Numerals, *there*, *this*, and samples

**The remarks.** Extend the builders' language with numerals, with `there`
and `this` used together with a pointing gesture, and with colour samples
(§8). Each is taught differently (§9). To the question "what do the words
signify?", the only honest answer shows how each is used. Saying "it
signifies…" makes them look alike, while their uses are unlike (§10).

**In Phient's terms.** The posts (D2) are that answer. The Phical counts
learner reads them off the episodes. Nobody wrote them:

| word | post | moves the answer to |
|---|---|---|
| `a` … `j` | count | 1 … 10 |
| `block`, `pillar`, `slab`, `beam` | what | that stone |
| `red`, `blue`, `yellow` | colour | that colour |
| `there` | place | *away from here* (its top value, p3, is only the most frequent of five) |
| `⟨point:p1⟩` … `⟨point:p5⟩` | place | that place |
| `⟨point:slab⟩` … | what | that stone |
| `help`, `away`, `no` | act, and every other aspect | come / clear / undo, and "none" |
| `this` | **none** | — |

The last line is the finding of this section. `this` holds no post: it
moves nothing. The pointing that goes with it does the work. In our
episodes the same is true of `there`, whose move is only "not here", while
the pointing picks the place. §8–9 say these words are used *with* a
gesture. The counts say the gesture carries the content and the word marks
that a gesture is being used. This came out of the counts, not out of our
design: we gave `this` and `there` no special treatment.

**What the game showed.** New combinations (§8): numerals and places
paired with stones they were never heard with.

- lookup: 0%;
- naming table, bag of words, Phical counts (movement), Phient
  (movement): 100%, both seeds;
- Phient with summed composition: 0–14%.

Composition is where the method matters, and §10 says why. Summing treats
every word as signifying something under every aspect, so words that
signify nothing under an aspect still vote there. Movement lets them fall
silent.

### §11–17 — Tools, handles, and the uniform look

**The remarks.** Words are like the tools in a toolbox: hammer, pliers,
saw, glue, nails. Their functions differ as much as the tools do (§11).
The handles in a locomotive cab look alike, yet one is a crank, one a
switch, one a brake (§12). "Every word signifies something" says nothing
until we say which distinction we mean (§13). Classifying words is like
classifying tools or chess pieces: it depends on the aim (§17).

**In Phient's terms.** This is the paradigm's own picture. Every word is
the same kind of thing: a row in one table of functions on the circle,
with R relations — the uniform handle. What differs is what each relates
to under each kind — what the handle does. The posts table above is the
toolbox. D1 alone ("a word moves the answer") is §13's empty saying. It
becomes a claim only when the kinds are named. And which kinds we name is
our aim (§17). Here they are the five aspects of the builders' action; in
the Phi-4 quests they are is-a, has, part-of, made-of, used-for and does.

### §18–20 — The old city, a form of life, and "Slab!"

**The remarks.** Language is like an old city: a maze of lanes and old
houses, with newer, regular districts added around it (§18). To imagine a
language is to imagine a form of life. Is the call "Slab!" a word or a
sentence, and is it short for "Bring me a slab" (§19)? The two have the
same sense only in the use made of them. Calling one "elliptical" measures
it against our longer form. In its own game it lacks nothing (§20).

**In Phient's terms.** In D4 the condition supplies what is not said. `slab`
alone, in the order game, is followed by *bring · slab · any colour · 1 ·
here*:

- the game says *bring*;
- the counts say *1* and *here* when no numeral or pointing moves them;
- the word moves only *what*.

No hidden longer sentence is reconstructed. The call lacks nothing,
because the condition does its share. The newer districts of the city are
learning that keeps (docs/100). New personas are added to one model, with
replay, and on every seed the older districts read *better* afterwards.

### §21–22 — Order and report

**The remarks.** Someone may *report* "five slabs", or *order* it, with the
same words. The difference is not in the words. It is in the part the
utterance plays in the language-game (§21). Frege's idea that every
assertion contains an assumption that is then asserted adds a component
that is not there (§22).

**In Phient's terms.** The game is the condition *c*. The act aspect is not
carried by any said word. It belongs to the game token, which is never
spoken, and it is *not* carried across games (D3). The counts find this:
the words that hold a post under `what`, `colour`, `count` or `place` point
the same way in both games, and under `act` they do not. So D3 lets `slab`
carry *what it is* into the report game but not *what is done*. There is
no separate assertion part, only the condition.

**What the game showed.** Reports of stones never reported:

- naming table: 0%;
- lookup: 0.7–1.3%;
- Phical counts, summed: 60%;
- bag of words, counts (movement), Phient (movement): 100%, both seeds.

Getting here took three trials (§6).

### §23–25 — The many games

**The remarks.** How many kinds of sentence are there? Countless kinds of
use (§23): giving orders and obeying them, describing, reporting,
guessing, making a joke, asking, thanking, cursing, greeting, praying,
and more. Commanding, asking and telling stories belong to our natural
history as much as walking or eating (§25).

**In Phient's terms.** Each quest kind Phiano puts to Phi-4 is a small
language-game with the teacher. Examples are "name one thing a cat
*has*" and "two sentences in the Naturalist's voice". The persona files
are lists of games (docs/98). §23 is a warning to the plan: a student that
re-creates Phi-4 needs the games, not only the facts. A later chapter
takes §23's list as the next set of quest kinds.

### §26–27 — Naming as preparation, and *Help!*

**The remarks.** Naming is only a preparation for using a word (§26). We
think we name things and then talk about them. But think of *Water!*,
*Away!*, *Ow!*, *Help!*, *Fine!*, *No!* Are these names of objects (§27)?

**In Phient's terms.** `help`, `away` and `no` hold posts under *act*, and
under every other aspect they move the answer to "none". They relate, as
every word does, but not to a thing.

**What the game showed.** No-name words: naming table 0%. Every learner
that has an act aspect scores 100%.

### §28–30 — Ostension, *two*, and *sepia*

**The remarks.** Point at two nuts and say "two". The pointing can be
taken as naming the group, the kind of nut, a colour, a number, or a
direction (§28). Say instead "*this number* is called two", and the word
*number* shows the post at which the new word is stationed. But *number*
needs its own explanation, and explanations must end somewhere (§29). A
pointing explains a word's use only when the overall role of the word is
already clear. "That colour is called *sepia*" helps someone who knows
what a colour-word is. One must already know something to ask for a name
(§30).

**In Phient's terms.** A pointing is an episode in which every aspect is
present at once: a stone, a colour, a number, a place. It tells the new
word under every kind, and that is the ambiguity. The word `colour`
stations the new word at one post, so it is told under the colour kind
only. That is §29 in D2. Only a learner that *has* posts can use the
station: the naming table has only names, and hearing `colour` gives it
one more name.

**What the game showed.** Exact match on orders that use *sepia*, after
pointing only (seed 7 / seed 8):

| learner | 1 pointing | 3 pointings | 1 + `colour` | 3 + `colour` |
|---|---|---|---|---|
| naming table | 0 / 0% | **100 / 100%** | 0 / 0% | 100 / 100% |
| bag of words | 0 / 0% | 0 / 0% | 0 / 0% | 0 / 0% |
| Phical counts, movement | 26 / 21% | 26 / 40% | **100 / 100%** | **100 / 100%** |
| Phient (phinet), movement | 0 / 5% | 4 / 5% | **100 / 100%** | **100 / 100%** |

All three remarks can be read off this table:

- **§28.** One pointing is ambiguous for every learner.
- **Cross-situational learning.** Three pointings at different sepia things
  let the naming table find what they share. That is its best case, and it
  is Augustine's own method.
- **§29–30.** With the grammar word, Phical counts learn the word from a
  single pointing, on both seeds. The naming table cannot use the station.
- **Bag of words** learns nothing from pointing in any column. One example
  is too little against thousands.
- **The phinet learner** learns the stationed word from one pointing too,
  on both seeds. An earlier version reached 100% on seed 7 but 0% on seed
  8. The fault was not the new word: the new word was read right, but the
  *condition* (what an order does when no word says otherwise) was read
  from the operator, and one retraining turned every order into `undo`.
  Trial 7 (§6) fixes that.
- **Where one pointing fails, it fails exactly as §28 says.** Without the
  grammar word, every movement learner gets the colour right (100%) and
  errs on count and place. The one pointing also told them a number and a
  place, and they took those as part of what *sepia* means.

### §31–32 — The chess king, and the stranger

**The remarks.** Showing someone the king and saying "this is the king"
tells them how to use it only if they already know the game, or have
learned it by watching (§31). Someone arriving in a strange country learns
the language from explanations and guesses. Augustine describes a child
learning language as if the child were such a stranger. The child already
has a language, only not this one, and can already think, only not yet
speak (§32).

**In Phient's terms.** Our learners learn the game by watching (§31), and
none is told a rule. For §32 we test it directly. The same game is played
in a new vocabulary: every word is replaced, while the pointing gestures
and the game are unchanged. The *child* meets only the new words (k
episodes). The *stranger* has played the game in the old words (3,600
episodes) and then meets the same k new episodes. In Phiano this is
`RelationModel::extend` with replay (docs/100).

**What the game showed.** Exact match on new-vocabulary orders, child →
stranger (seed 7 / seed 8):

| learner | 40 episodes | 80 episodes |
|---|---|---|
| naming table | 40→59% / 58→69% | 58→69% / 82→90% |
| bag of words | 45→57% / 41→48% | 56→68% / 72→80% |
| Phical counts, movement | 47→67% / 50→73% | 65→81% / 68→77% |
| Phient (phinet, R = 32) | 51→48% / 48→52% | 60→75% / 59→73% |
| Phient two-speed (counts until 32) | 47→53% / 51→65% | 65→81% / 68→77% |

From 20 new episodes on, knowing the old game helps almost every learner. The exception is phinet on seed 7 at 40 episodes (51→48%). With only 10, it sometimes hurts: counts on seed 7 fall from 26% to 12%, because 3,600 old episodes outweigh ten new ones before the new words hold posts. Most of
the gain comes from what is shared: the pointing and the game itself
already hold their posts, so the new words only have to find theirs.
§32's diagnosis applies to our learners too. They are all strangers in the
sense of §32: the aspects of the action are given to them. A real child
would also have to find those.

## 5. What Phient adds, and what it does not

**Against the other methods, honestly:**

- In the game, the bag of words is as accurate as Phient's movement
  composition (100% on all four tests) and needs fewer episodes.
- Phient's methods differ from it in three places:
  1. the *station* of §29–30, where Phical counts learn a new word from one
     pointing and the bag of words never does;
  2. the stranger at 40 episodes, where counts lead;
  3. legibility. The posts table is a grammar anyone can read. The softmax
     weights are not.
- The naming table is right exactly where the Augustinian picture is
  right, and wrong exactly where §1–27 say it will be.

**Between counts and functions.** Here, the relations as counts beat the
same relations as phinet functions on data efficiency: at 160 episodes,
70% against 55–65%. The functions catch up at 640 episodes (99% on both
seeds at R = 32). The functions compress: 22,160 numbers at R = 32 hold
1,585 relations. They also generalise across words, which is what the
Phi-4 quests need (docs/99–100). But they cannot say *never*, and a spread
relation reads to them as a peaked one.

**Two speeds (docs/103).** A relation acts from its count at once, and is
read from the operator's function once it has been told N times. The
operator trains on every relation all along; only the reading switches.

- With N = 32, the learner follows the counts while data is scarce
  (69.8 / 70.4% at 160 episodes, counts 69.8 / 70.4%) and the functions
  once it is not (98.4 / 97.4% at 640).
- For the stranger it matches the counts at 80 episodes.
- A second rule, *consolidate when the function agrees with the count*,
  measures how much the functions already hold. With 3,600 episodes the
  functions name the same top answer as the counts for **90.6–90.8 / 88.7%** (the seed-7 figure moved after ties were made to break by name, docs/104) of
  the relations, so about nine relations in ten could keep only their
  function. The rest are the ones that still need their count.

## 6. The trials and errors

The results above are from the last trial. Every earlier trial is in the
code history, and none was tuned on the tests. Each trial below is scored
on seed 7 as seen / new combinations / reports.

**Phical counts:**

1. **Summed** (each word votes its reading, weighted by firmness): 87 / 63
   / 60%. Numerals and places lost to the many words that say "1" and
   "here". This is §10's error, made in code: it treats every word as
   signifying something everywhere.
2. **Movement against the condition** (naive-Bayes form of D4): 100 / 56 /
   0%. Missing counts vetoed answers. A numeral never heard with *beam*
   forbade beam, though it has nothing to say about stones.
3. **Posts (D2) and games (D3), strictly:** 100 / 100 / 5%. A word was
   carried across games only if *every* word agreed across them, and the
   diffuse words never do.
4. **Games agree where the firm words agree; every move measured from this
   game's condition:** 100 / 100 / 100%.

**Phient (phinet), with the same composition as counts:**

1. **Summed:** 1–56% seen, depending on epochs.
2. **Movement:** 46 / 14 / 71%. The operator had learned answer *types* (a
   colour, not which colour), as docs/92 found for Phi-4's relations.
3. **Decoys drawn from the same aspect:** 99 / 92 / 89%.
4. **Posts decided by the counts, readings by the functions:** 100 / 100 /
   89%. The operator reads a diffuse relation as a peaked one; the count
   knows better.
5. **Readings measured against no preference instead of the condition —
   rejected:** 56 / 37 / 7%, and 0% on no-name. It was meant to stop the
   operator's small doubts, read against a condition that never saw a
   value, from turning into large moves. It made things worse and was
   reverted.
6. **A borrowed word's post is judged in the game where it was told**
   (the word `this`, diffuse there, no longer looks informative in the
   report game): 100 / 100 / 100%.
7. **The condition is read from its count, not from the operator.** A game
   token relates to every answer and holds no post. The operator read that
   spread relation as one answer, and after one retraining (seed 8 with
   one pointing) it read every order as `undo`. The four main tests were
   unchanged (100%), and phinet got better everywhere else:
   - sepia with the grammar word, 0 → 100% (seed 8);
   - 320 episodes, 62.7 / 53.8 → 77.9 / 90.2%;
   - 640 episodes at R = 8, 85.8 / 87.7 → 94.1 / 98.1%.
8. **Two speeds** (§5): counts first, functions after N sightings.

## 7. Next

- **Phi-4 in the game** (built, not yet run: `scripts/phi4_builders.ps1`).
  The phiecs quest kind `builders` puts Phi-4 in the game as B. Its
  actions become the episodes a student learns from, and the rule's
  actions are kept beside them. The report says how often Phi-4 did what
  the rules say, and what a student learns from it.
- **Forget what the function holds.** Drop the count of every relation
  whose function agrees with it, and measure the loss.
- **§23 as quest kinds.** Order, describe, report, guess, ask, thank. Each
  is a game with its own condition; D3 says which aspects carry between
  them.
- **Descriptive Logic II** (paper 20): §§33–64 — names and bearers, samples,
  simples and composites (§48, §64), and rules read off practice with mistakes (§54).

*Code:*

- `phiano/src/bin/language_game/{world, split, learners, naming, bow, compose, phient, experiments, teacher, student, main}.rs`;
- phiecs `quest/builders.rs`;
- phinet `relate/{inherit, structure}.rs`.

*Raw tables:* docs/103_language_game_seed7.md and docs/103_language_game_seed8.md (these supersede the docs/102 tables).

*Run:* `cargo run --release --bin language_game -- out.md --seed 7`, about
20 minutes on one core.
