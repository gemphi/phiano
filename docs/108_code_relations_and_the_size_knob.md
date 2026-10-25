# 108 · Code as relations, and the size knob in persona files

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. The size knob in persona files (outstanding since docs/91 and docs/100)

A persona file can now declare `relations: N`, the number of relations per word in its relation model. This is the one size knob of docs/100.

- The default is 64. Any value works, including 128 and above.
- `persona_from_quest` builds the model with that many relations, with R/4 operator modes up to a maximum of 16.
- phiecs `Persona::relations` is new, and its test covers both the default and 128.
- Check run: the demo persona Town with `relations: 128` gave held-out MRR 0.124 against a prior of 0.070, and within kind 0.159 against a prior of 0.073.

## 2. Code as relations (the coding goal, first step; docs/97 §2)

`bin/code_relations <dir>…` reads Rust sources with Rust's own grammar (syn 2, already in the build) and writes one row per distinct relation. It then trains a relation model and saves `<out>.relate`, which `relate_ask` can query. The relations are:

| kind | meaning |
|---|---|
| `calls` | what a function calls |
| `takes` | the type of each argument a function takes |
| `returns` | the type a function returns |
| `has` | a type's fields, or an enum's variants |
| `holds` | the types of a struct's fields |
| `implements` | a trait a type implements |
| `part-of` | the type or trait a method belongs to |

Run on phinet, phiano and phiecs: 676 files, 33,037 relation sightings, 20,886 distinct relations.

| kind | distinct relations |
|---|---|
| calls | 12,954 |
| takes | 2,471 |
| has | 1,708 |
| part-of | 1,539 |
| returns | 1,300 |
| holds | 845 |
| implements | 69 |

R = 64, 30 epochs. A fifth of the rows is held out, and there are two seeds.

| reading of relations never told | held-out MRR | hit@5 |
|---|---|---|
| prior (how often the kind's answer occurs) | 0.157 | — |
| functions alone | 0.096 / 0.097 | 15.5 / 15.6% |
| movement: the kind's count × the name's function | 0.157 / 0.151 | 24.8 / 24.6% |

What this shows:

- **Told relations are held.** Asked about names it was told, the model answers as the code does:
  - `Episode · has ⇒ composites_at, action, words`;
  - `score · returns ⇒ Vec, Option, f64, Score`;
  - `Counts · implements ⇒ Learner`.
- **Untold relations of code are not yet inferred beyond frequency.** Which functions a new function calls, or which types it takes, is predicted no better than by how common each answer is. The functions alone fall below the prior. Adding them to the counts (movement) only brings it back to the prior.
- **The type law is not the missing piece.** Restricting type slots to types changes little (within kind 0.158 under the law against 0.158 without), because the kind's own answers are already almost all types.
- **What is missing is conditions.** A method's calls depend on its type, its module and its arguments, and here a name meets only its kind. The next step for code is the condition: a method under its owner type, read as inheritance along `part-of` (docs/99's structure). That is the Phient answer to "under what conditions".

Run: `cargo run --release --bin code_relations -- ../phinet/src src ../phiace/crates/phiecs/src --out data/code/gemphi.tsv` (about 8 minutes). Raw output: `108_code_relations_seed{7,8}.txt`.

## 3. The owner as condition, measured: not supported

This tests the idea from §2 that a method should be read under the type it
belongs to. `code_relations` now also derives the owner's relations from
the told rows only: for each `f part-of T` and each `f calls/takes/returns
v`, one `T … v` row, 19,718 in all. It reads each held-out `f k ?` two ways:

- **counts under the owner:** by what f's sibling methods relate to;
- **phinet inheriting along part-of:** `calls(f) = own(f) ⊕ calls(T)`, the
  docs/99 structure.

The sources now include the new files, so the prior differs slightly from §2.

| reading (seeds 7 / 8) | held-out MRR | hit@5 |
|---|---|---|
| prior (with owner rows) | 0.155 / 0.155 | 24.9 / 24.9% |
| counts under the owner | 0.143 / 0.143 | 23.7 / 23.7% |
| phinet inheriting along part-of | 0.132 / 0.129 | 21.3 / 20.2% |
| movement, the kind's count × the name's function (no owner) | 0.167 / 0.155 | 26.0 / 24.8% |

Per kind, counts under the owner are 0.33 for `takes` and `returns` and 0.36
for `holds`. Those are good numbers, but they are close to the kind's own
frequency.

- **The owner does not predict a method's relations better than how common
  each answer is.** Sibling methods of one type do not call, take or return
  the same things often enough to help.
- **The only reading at or above the prior is movement** (+0.016 on seed 7,
  +0.004 on seed 8). That is a small gain, and not a settled one.
- **Code at the level of names is dominated by frequency.** Which function a
  new function calls is set by what its body does, which is a sequence, not
  a name. The coding goal needs the relation model inside a reader of token
  sequences (the squares reader of paper 20 is the smallest example of
  one), not name-level relations alone. That is the next design question
  for code.

Raw output: `108_code_owner_seed{7,8}.txt`.
