# 105 · Family resemblance, Moses and blurred posts, in Phi-4's answers

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

The write-up is paper 21, `docs/papers/21_DESCRIPTIVE_LOGIC_3_FAMILY_RESEMBLANCE.md` (and `.pdf`), which reads Philosophical Investigations §§65–88. Raw tables: `105_family_seed{7,8}.md` and `105_posts_seed{7,8}.md`.

- **Family resemblance (§§65–67)** — `bin/family`, night table, 3,630 rows, 12 categories of 5 or more members:
  - No category has one relation that all its members hold (0 of 12).
  - The most widely held relation reaches 54% of members on average, against 31% in random sets.
  - Members overlap about 7× more than random sets do (0.050 against 0.007).
  - Chains of shared relations join 85% of members, against 41% in random sets.
- **Moses (§79)** — identifying a word from some of its relations, among 209 words:
  - With 75% of its told relations dropped, the word is still picked out 92–94% of the time; with 90% dropped, 74–75%.
  - From held-out relations never told: counts 0%, phinet functions 25–28% top-1 (MRR 0.34–0.39), more than 50× chance.
- **Post threshold τ (§§69–71, §88)** — `language_game posts`:
  - Works between 0.5 and 1 nat.
  - At 0.1 nats or below: 74–86% on the rule tests, and the one-pointing sepia test fails.
  - At 2 nats or above: 20% or less.
  - The a-priori 0.5 stays, because choosing it from this table would be tuning on the tests.
