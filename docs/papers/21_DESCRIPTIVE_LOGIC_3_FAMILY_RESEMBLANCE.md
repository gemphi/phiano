---
id: descriptive-logic-3
title: "Descriptive Logic III: Family Resemblance and Blurred Edges"
description: Philosophical Investigations §§65–88 read in Phient's terms; family resemblance and Moses counted in Phi-4's own answers, and how exact a post must be.
sidebar_position: 21
badge: paper
---

# Descriptive Logic III: Family Resemblance and Blurred Edges

## Philosophical Investigations §§65–88, in Phient's terms

**Phiano Project** · `bin/family`, `bin/language_game posts` · 2026-09-30

> In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Chapters I and II measured the remarks in games we built. This chapter
measures them in the language we did not build: **Phi-4's own answers**.
These are the 3,630 relations of the night quest (docs/96–99), 209 words
and six kinds. §§65–67 say a concept holds together by overlapping
similarities rather than by one shared feature. §79 says a name is used
without a fixed set of descriptions. Both can be counted in the teacher's
answers. §§69–71 and §88 say a blurred concept is still a concept, and that
exactness depends on purpose. We measure that on Chapter I's post
threshold.

As before, Wittgenstein is paraphrased and cited by remark, never quoted.

---

## 1. Three more definitions

- **D11 · Family.** A category is a family when its members are joined by
  chains of shared relations. No one relation needs to run through all of
  them. The *longest fibre* is the share of members holding the most
  widely held relation. The *thread* is the share joined, through such
  chains, to the largest group (§67).
- **D12 · Cluster name.** A word's identity is spread over its relations.
  It survives the loss of any of them, and of most (§79).
- **D13 · Working exactness.** A post threshold has a working range, not an
  ideal value. Too blurred and every word votes on everything. Too exact
  and no word holds a post (§§69–71, §88).

## 2. The remarks, in Phient's terms

### §§65–67 — Games, and the thread

**The remarks.** Wittgenstein's interlocutor objects: you speak of all
these language-games, but never say what the essence of a language-game
is (§65). Look at games: board games, card games, ball games, games of
skill and of luck. Don't say there must be something common to them. Look,
and you will find similarities and relationships that crop up and
disappear, criss-crossing and overlapping (§66). He calls these *family
resemblances*. A concept is like a thread: its strength lies not in one
fibre running its whole length, but in many fibres overlapping (§67).

**In Phient's terms (D11).** A category, such as *tool* or *animal*, is
every word Phi-4 filed under it (is-a). A member's relations are its
other answers: what it has, is made of, is used for, and does. If there is
an essence, some relation is held by all the members. If there is a
family, the members are joined by overlaps.

**Measured** (`bin/family`, the night table). There are 12 categories with
5 or more members, each member having at least 3 non-is-a relations. Each
is compared with 200 random sets of the same size:

| category | members | longest fibre | mean overlap | thread | random: fibre / overlap / thread |
|---|---|---|---|---|---|
| animal | 11 | 55% | 0.048 | 100% | 25% / 0.007 / 46% |
| machine | 7 | 86% | 0.103 | 100% | 32% / 0.008 / 42% |
| mammal | 9 | 78% | 0.089 | 89% | 28% / 0.007 / 42% |
| tool | 9 | 56% | 0.043 | 89% | 27% / 0.007 / 40% |
| food | 10 | 40% | 0.033 | 90% | 26% / 0.007 / 42% |
| material | 11 | 27% | 0.011 | 91% | 25% / 0.006 / 43% |
| **mean of 12** | | **54%** | **0.050** | **85%** | **31% / 0.007 / 41%** |

(All 12 rows are in docs/105. Seeds 7 and 8 give the same category figures;
the random baseline differs by at most one point.)

- **No category has an essence in Phi-4's answers.** In 0 of 12 does one
  relation hold for every member. The longest fibre averages 54%.
- **Every category is a family.** Members overlap about 7 times more than
  random sets do (0.050 against 0.007). Chains join 85% of members, against
  41% for random sets. *material* is the plainest case: its longest fibre
  reaches only 27%, barely above random (25%), yet chains join 91% of its
  members.
- **What this does not show.** The quest asked each word only a few
  questions per kind, so an essence Phi-4 knows but was never asked about
  would not appear. The claim is about the answers as given. It is also
  why a student learning from Phi-4 should expect families, not
  definitions.

### §68 — Number, and the extended concept

**The remarks.** We extend the concept of number as we spin a thread, by
twisting fibre on fibre. We can draw a boundary for a special purpose, but
we don't need one for the concept to be usable (§68).

**In Phient's terms.** Chapter I's stranger test and the learning demo
(docs/100) are this kind of extension. A new word or persona is added to a
model that keeps its old relations. No boundary was drawn, and the old
categories read *better* afterwards.

### §§69–71, §§76–77 — Blurred edges

**The remarks.** How would we explain to someone what a game is? By
describing games, and adding "and similar things" (§69). Is a concept with
blurred edges a concept at all? Is an indistinct photograph a picture of
someone? Often the indistinct one is exactly what we need (§71). "Stand
roughly there" is a perfectly good instruction. When a blurred picture is
redrawn as a sharp one, many sharp ones fit, and the more blurred the
original, the more anything goes (§§76–77).

**In Phient's terms (D13).** A post is a blurred edge. A word holds a post
under a kind when it moves the answer by at least τ nats. Chapter I fixed
τ = 0.5 before any test was read. Here τ is swept.

**Measured** (`language_game posts`, Phical counts with movement, seed 7 /
seed 8):

| τ (nats) | four rule tests | sepia, 1 pointing + `colour` | after 160 episodes |
|---|---|---|---|
| 0 | 77.6 / 74.0% | 0 / 0% | 37.4 / 42.0% |
| 0.1 | 86.0 / 82.9% | 0 / 0% | 37.4 / 44.2% |
| 0.25 | 98.0 / 99.1% | 79.7 / 100% | 45.4 / 53.3% |
| **0.5** | **100 / 100%** | **100 / 100%** | **69.8 / 70.4%** |
| 1 | 100 / 100% | 100 / 100% | 74.0 / 70.4% |
| 2 | 21.8 / 19.1% | 26.0 / 25.3% | 7.4 / 4.8% |
| 4 | 1.8 / 1.5% | 0 / 0% | 1.8 / 1.8% |

- **There is a working range, from 0.5 to 1 nat, and no ideal value.**
  Below it, words that say nothing about an aspect vote on it anyway:
  §10's error from Chapter I returns. Above it, too few words hold posts,
  and the condition alone answers every call.
- **The a-priori choice of 0.5 lies in the range but is not its best
  point** on seed 7 with little data (69.8% against 74.0% at τ = 1). We
  keep 0.5, because choosing on this table would be tuning on the tests.

### §§72–75 — Seeing what is common

**The remarks.** Seeing what is common to a set of samples does not mean
seeing a single picture of it (§§72–74). Knowing what a game is shows in
what we do: giving examples, playing, telling games from non-games. It is
not an inner definition (§75).

**In Phient's terms.** Phical stores no point, so there is no stored
prototype for *tool*. What exists are the tool-words' relations, which
overlap in a family. Knowing what a tool is shows in how the model reads
each tool-word: in the relations it gives back.

### §§78–79 — Mont Blanc, and Moses

**The remarks.** Knowing how high Mont Blanc is differs from knowing how a
clarinet sounds (§78). Take "Moses did not exist". We might mean that the
Israelites had no single leader, or that no one did all the things told of
Moses, or something else again. There is no fixed set of descriptions a
name stands for. If some prove false, we don't give up the name. We use
the name without a fixed meaning, and that doesn't spoil its use (§79).

**In Phient's terms (D12).** A word's identity is its relations. Keep only
some of them and ask which of the 209 words they pick out. There are two
readers:

- **the counts:** how many of the kept relations a candidate was told;
- **phinet's functions:** the sum of cos(F(candidate + kind), other) over
  the kept relations, read from a model trained on the told rows only.

There are also two sources: relations that were told, and relations held
out of training, which are descriptions the reader never heard.

**Measured** (`bin/family`, seed 7 / seed 8; chance top-1 is 0.48%):

| relations kept | dropped | counts: top-1 | functions: top-1 | functions: MRR |
|---|---|---|---|---|
| told | 0% | 100 / 100% | 100 / 100% | 1.000 / 1.000 |
| told | 50% | 99.5 / 99.5% | 99.0 / 99.0% | 0.993 / 0.994 |
| told | 75% | 93.6 / 93.6% | 92.6 / 91.7% | 0.954 / 0.948 |
| told | 90% | 75.0 / 75.0% | 73.5 / 75.0% | 0.839 / 0.835 |
| **held out, never told** | 0% | **0 / 0%** | **27.9 / 25.2%** | **0.385 / 0.337** |

- **No relation is the defining one.** With three quarters of its
  relations gone, a word is still picked out 92–94% of the time. With nine
  tenths gone, 74–75%. That is §79 in numbers.
- **The functions pick out a word from descriptions never told.** The
  counts cannot do this at all (0%), because they only know what they were
  told. The functions pick out the right word in 25–28% of cases, more
  than 50 times chance, and have it near the top (MRR 0.34–0.39). This is
  the clearest measurement so far of what the functions add. A word's
  function holds more than its told relations: it holds what the word
  *would* relate to. Chapter I found the functions slower to learn than
  the counts; here they do what the counts cannot.

### §§80–88 — The disappearing chair, rules, and signposts

**The remarks.** Imagine a chair that vanishes and reappears. Do we have
rules for whether to call it a chair? No, and we don't miss them (§80).
Wittgenstein looks back on the idea that logic is a normative science,
dealing with ideal languages, as if our language only approximated a
calculus (§81). A rule is what someone proceeds by, and sometimes we play
and make up the rules as we go along (§§82–83). A rule stands like a
signpost. Does a signpost leave no doubt about the way? Sometimes it
does, sometimes not (§§84–85). A table can be read by a schema of arrows,
but then the schema needs a rule for reading it, and so on. An
explanation serves to remove or head off a misunderstanding, not every
possible one (§§86–87). "Stand roughly here" can work perfectly. There is
no single ideal of exactness; it depends on what we are doing (§88).

**In Phient's terms.** Phient has no rules for every case, and it does not
miss them:

- A word never met under a condition is carried from another game only
  where the games agree (D3). Otherwise it falls silent, and the condition
  answers.
- The declared laws of docs/101 were the one place Phient tried to follow
  rules as a calculus. Where the teacher's own answers did not follow
  them, the rules cost accuracy.
- The post threshold (§§69–71 above) is §88 measured: it works across a
  range, and neither the sharpest nor the most blurred setting works.

## 3. What Chapter III adds

- **Two findings about the teacher, not about games we built.** Phi-4's
  categories are families, not definitions. Its words are clusters: no
  relation is the defining one.
- **The clearest case so far for the functions.** From descriptions never
  told, phinet picks out the word 25–28% of the time, against the counts'
  0%. Chapters I and II found the counts faster and steadier on data they
  had seen. This chapter finds where the functions are needed: on what was
  never seen. Taken together with Chapter I's two speeds, the three
  chapters say the same thing. Counts come first. Functions carry what the
  counts can't say.
- **A post threshold has a working range** (0.5–1 nat), which D2 now
  states.

## 4. Next

- **Phi-4 in the builders' game** (`scripts/phi4_builders.ps1`): §54 and
  §79 now say what to look for in its mistakes.
- **Descriptive Logic IV, §§89–133:** philosophy leaves everything as it
  is; language on holiday; the demand for crystalline purity. Its measure
  could be how far each of our fixes (trials 1–9) moved toward or away from
  describing rather than prescribing.
- **Family resemblance as a quest kind:** ask Phi-4 directly what two
  members of a category share, and compare with the counted overlap.

*Code:*

- `phiano/src/bin/family/{data, resemblance, moses, main}.rs`;
- `phiano/src/bin/language_game/experiments.rs` (`posts`).

*Raw tables:* docs/105_*.md.

*Run:*

- `cargo run --release --bin family -- data/quest/night_2026-09-29/all.tsv out.md --seed 7` (about 8 minutes);
- `cargo run --release --bin language_game -- posts out.md --seed 7` (seconds).
