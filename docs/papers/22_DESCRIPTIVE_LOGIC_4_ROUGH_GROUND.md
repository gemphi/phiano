---
id: descriptive-logic-4
title: "Descriptive Logic IV: The Rough Ground"
description: Philosophical Investigations §§89–133 read in Phient's terms; contradictions in Phi-4's answers, a perspicuous map of its families, and methods rather than a method.
sidebar_position: 22
badge: paper
---

# Descriptive Logic IV: The Rough Ground

## Philosophical Investigations §§89–133, in Phient's terms

**Phiano Project** · `bin/family --rough` · the *Phi-4's Families* page · 2026-09-30

> In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

§§89–133 turn from language to philosophy itself. Philosophy is to
describe, not explain. It is a fight against being bewitched by our own
forms of expression. It needs friction: back to the rough ground. It
brings things into a view that can be surveyed at once, leaves everything
as it is, and has methods rather than one method. In our project the
philosophy is the code, so this chapter turns the remarks on the work of
Chapters I–III. It asks three questions:

1. Does the teacher's language meet the ideal of a consistent calculus?
2. Can its use be put in plain view?
3. Is there one method that wins?

As before, Wittgenstein is paraphrased and cited by remark, never quoted.

---

## 1. Three more definitions

- **D14 · Rough ground.** The teacher's answers are the ground. Read by
  the kinds' ideal meaning, some of them contradict each other. A
  descriptive learner takes them as use and does not repair them first
  (§107, §125).
- **D15 · Surveyable view.** A representation is surveyable when it lets
  the connections be seen at once, including the *intermediate cases*.
  For Phient, that means the posts table (Chapter I), the fibre chart and
  the words that bridge families (the *Phi-4's Families* page) (§122).
- **D16 · Methods, not a method.** No single learner is best on every
  test. Each works where its form of reading matches the game (§133).

## 2. The remarks, in Phient's terms

### §§89–92 — The sublime, and what lies hidden

**The remarks.** Logic seems to have a peculiar depth, as if it reached
to the essence of everything (§89). We feel we must see *into* phenomena,
when our inquiry is about the kinds of statement we make about them
(§90). This can give the idea of a final, completely exact analysis still
waiting to be found (§91). Then the essence seems hidden, something that
lies beneath the surface (§92).

**In Phient's terms.** Nothing in Chapters I–III lies beneath the
surface. A post is a count anyone can read. A family is overlaps anyone
can draw. A word's identity is spread over relations anyone can list. The
functions are the one place something is not on the surface. Chapter III
found they carry what the counts cannot (descriptions never told). But
what they carry is still *what the word relates to*, read back through
the operator. There is no second, deeper meaning in them.

### §§93–100 — The ideal must be in reality

**The remarks.** A proposition seems a remarkable thing. We are tempted
to sublime the logic of our language (§§93–94). Thought seems unique, the
order of the world's possibilities laid down in it (§§95–97). Yet every
sentence of our language is in order as it is. We are not striving after
an ideal, as if ordinary vague sentences had no sense yet (§98). A vague
sense seems to be no sense at all, like an enclosure with a gap. But an
enclosure with a hole in it is still an enclosure (§§99–100).

**In Phient's terms.** Chapter III measured the enclosure with a hole.
The post threshold works across a range (0.5–1 nat). A post is a boundary
with a blur, and it serves. Every episode of the builders' game was in
order as it was; the learners did not need a cleaner language first.

### §§101–108 — Crystalline purity, and friction

**The remarks.** We think the ideal must be found in reality, since we
think we already see it there (§101). The strict, clear rules of logic
seem to lie in the background, hidden in the medium of understanding
(§102). The ideal is fixed in our thinking like glasses on our nose,
through which we see whatever we look at (§103). The crystalline purity
of logic was not a *result* of investigation; it was a *requirement*
(§107). The conflict became unbearable. We have got onto slippery ice
where there is no friction, so in a sense the conditions are ideal — but
we cannot walk. We want to walk, so we need friction: back to the rough
ground (§107). The preconception of crystalline purity can only be
removed by turning the whole inquiry around (§108).

**In Phient's terms (D14): measured.** Read by the kinds' ordinary
meaning, some of Phi-4's answers contradict each other. `bin/family
--rough` counts them in the night table:

| contradiction | rows | examples |
|---|---|---|
| a is-a b, and b is-a a | 39 | animal is-a mammal (and mammal is-a animal); car / automobile |
| a has b, and a part-of b | 164 | appendage has limb, and appendage part-of limb |
| a part-of b, and b part-of a | 12 | computer part-of device, and device part-of computer |
| **total** | **210 of 3,630 rows (5.8%)** | |

Some of these are errors, such as animal is-a mammal. Many are the
teacher using *is-a* for "is another word for" (car and automobile), or
*has* and *part-of* loosely. So the "contradiction" belongs to our ideal
reading of the kinds, not to the teacher's use.

Does it stop the game? phinet is trained on the told rows with the
contradicting ones and without them. Both models are read on the same 702
held-out rows, none of which contradicts anything. Four seeds:

| seed | with: MRR / within kind | without: MRR / within kind |
|---|---|---|
| 7 | 0.084 / 0.111 | 0.080 / 0.112 |
| 8 | 0.075 / 0.107 | 0.076 / 0.107 |
| 9 | 0.073 / 0.128 | 0.076 / 0.123 |
| 10 | 0.083 / 0.114 | 0.094 / 0.127 |
| **mean** | **0.079 ± 0.006 / 0.115** | **0.082 ± 0.009 / 0.117** |

The difference is smaller than the spread between seeds. Removing the
contradictions does not measurably help. This fits docs/101: *imposing*
declared laws (the converse, transitivity) cost 0.016–0.027 MRR. Purity
as a requirement costs something. Leaving the ground rough costs nothing
we can measure.

### §§109–115 — Description, and a picture that held us captive

**The remarks.** Our considerations must not be scientific ones. We may
not advance any kind of theory. All *explanation* must disappear, and
description alone must take its place. Philosophy is a battle against
the bewitchment of our intelligence by means of language (§109). A
picture held us captive, and we could not get outside it, because it lay
in our language, which seemed to repeat it to us relentlessly (§115).

**In Phient's terms.** Our trial logs record where a picture held the
*code* captive:

- **Trial 1**, summed composition, was the picture that every word
  signifies something under every aspect.
- **Trial 8**, agreement judged over the whole language, was the picture
  of one global rule where there are only local games.
- **Trial 3**, which measured a function against no preference, was the
  picture that a function's reading is a probability like a count's.

Each was fixed by describing more closely what the episodes showed. We
did not add a theory.

### §§116–121 — Back to everyday use

**The remarks.** When philosophers use a word such as *knowledge*,
*being* or *object* and try to grasp its essence, we must ask whether the
word is ever used that way in the language-game that is its home. We
bring words back from their metaphysical to their everyday use (§116).
The results of philosophy are the bumps the understanding gets by running
its head up against the limits of language (§119). When I talk about
language, I have to speak everyday language (§120).

**In Phient's terms.** A word's home is the game where it holds its
posts. D3 carries a word only into games that agree with its home on an
aspect. This is §116 as an operation: a word read outside its home, where
the games disagree, falls silent, and the condition answers.

### §§122–124 — The surveyable view; leaving everything as it is

**The remarks.** A main source of our failure to understand is that we
do not *command a clear view* of the use of our words. A surveyable
representation produces the understanding that consists in seeing
connections, hence the importance of finding and inventing *intermediate
cases* (§122). A philosophical problem has the form: I don't know my way
about (§123). Philosophy may in no way interfere with the actual use of
language. In the end it can only describe it, and it leaves everything as
it is (§124).

**In Phient's terms (D15).** The *Phi-4's Families* page is a surveyable
view of the teacher's categories. For each of the 12 families it shows:

- the fibre chart: which member holds which relation, with no row full;
- who shares with whom;
- the words that bridge families.

The bridges are the intermediate cases: *engine* is filed under
component, device and machine; *dog*, *feline* and *monkey* under both
animal and mammal. The posts table of Chapter I is the same kind of view
for the builders' language.

§124 is the stance of the whole series. The learners describe Phi-4's use
and do not correct it. The one time Phient interfered, with declared laws
imposed as structure (docs/101), the student fitted the teacher's answers
worse.

### §§125–128 — The contradiction, and what everyone would admit

**The remarks.** Philosophy's business is not to resolve a contradiction
by a mathematical discovery. It is to get a clear view of the state of
affairs *before* the contradiction is resolved. We lay down rules for a
game, then find that when we follow them things do not turn out as we
assumed. We become entangled in our own rules (§125). Philosophy simply
puts everything before us (§126). Its work is assembling reminders for a
particular purpose (§127). If one tried to advance *theses* in
philosophy, everyone would agree to them (§128).

**In Phient's terms.** The 210 contradicting rows are laid out in plain
view (docs/106), not resolved. They cost nothing measurable. Our own
entanglement in our own rules is the trial log: the posts gate, the
game-agreement rule and the condition prior were each a rule we laid down
that did not turn out as assumed.

### §§129–133 — Objects of comparison; methods, like therapies

**The remarks.** The aspects of things that matter most to us are hidden
by their simplicity and familiarity (§129). Our clear, simple
language-games are not preliminary studies toward a future regularisation
of language. They are *objects of comparison*, meant to throw light on
our language by similarities and dissimilarities (§130). We avoid
dogmatism only by presenting the model as what it is, an object of
comparison, a measuring rod (§131). We want to establish an order for a
particular purpose, one of many possible orders (§132). There is not *a*
philosophical method, though there are methods, like different therapies
(§133).

**In Phient's terms.** The builders' game and the squares game are
exactly this: objects of comparison for Phi-4's language, not models of
it. Chapter III measured Phi-4's own answers against what the games
taught us to look for (families, cluster names). The next measuring rod
is Phi-4 playing the builders' game itself.

**D16, as a table: which method does what best**, collected from
Chapters I–IV:

| what is asked | best | not good at it | source |
|---|---|---|---|
| act from a known language, enough data | bag of words = counts = phinet (100%) | naming table (64%), lookup | ch. I |
| learn from fewest episodes | bag of words (95–100% at 160) | phinet (55–65% at 160) | ch. I |
| a word met once, pointed at, with the word `colour` | counts, phinet (100%) | bag of words (0%) | ch. I |
| composite words in new places | Phient reader (100%) | positional softmax (≤ 5%) | ch. II |
| a teacher who errs, errors marked | counts (100%) | bag of words (89–95%) | ch. II |
| a word from descriptions never told | phinet functions (25–28%) | counts (0%) | ch. III |
| unseen relations of Phi-4's words | phinet + inheritance (0.112 held-out MRR vs prior 0.053) | counts (no answer) | docs/99 |
| a contradicting teacher | all unchanged | — | ch. IV |

No row has the same winner as every other row. This is D16, and it is
also the design of Phient: counts first, functions for what the counts
cannot reach, and a condition that stays local to its game.

## 3. What Chapter IV adds

- **The teacher's language is rough ground, and that is fine.** 5.8% of
  Phi-4's answers contradict each other under the ideal reading of the
  kinds. Removing them does not measurably help (0.079 → 0.082 MRR, inside
  the seed spread). Imposing an ideal as structure did cost (docs/101).
- **A surveyable view of the teacher's categories**, as a page, with the
  intermediate cases in plain view.
- **Methods, not a method,** gathered into one table from four chapters.

## 4. Next

This ends the part of the *Investigations* that runs to §133. The next
part, §§134–242 and above all §§185–242, is about following a rule. It
asks what makes an answer the *right* continuation, when any finite set of
examples fits many rules. Phient's version of that question is its
central one: when the student continues where the teacher's examples
stop, what makes the continuation right? That chapter will need Phi-4
itself as the other player, so the run on the GPU machine comes first
(`scripts/phi4_builders.ps1`).

*Code:* `phiano/src/bin/family/rough.rs` (`--rough`). *Page:* *Phi-4's
Families* (a published artifact). *Raw tables:* docs/106.

*Run:* `cargo run --release --bin family -- data/quest/night_2026-09-29/all.tsv out.md --rough` (about 20 minutes, 4 seeds).
