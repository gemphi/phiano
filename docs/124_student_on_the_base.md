# 124 · The student of code on the base: counts, movement, proportion, and the chain of requests

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 0. The correction

docs/117–123 built the code writer beside the base, not on it: its own token model, its own routes (family resemblance, proportion by diffing token sequences, glosses, a vote), none of them a relation in the project's sense, none of them a kind the quest could ask about, none of them with a knob. The base is three crates with one job each (docs/85, docs/93, docs/98): **phiecs asks** and writes rows `(about, kind, other, weight)`; **phinet learns and knows no words** — one operator for every kind over a table of phasor functions, scored against the prior; **phiano names** — `RelationModel` maps words and kinds to rows of that table, and now `Counts` reads the same rows by their counts alone. This doc puts the code student back on that base, measures it with the base's own scoreboard, and keeps the heuristic writer only as a line to compare against.

Two things are added to the base, both small:

- `phiano::code` (moved from the `code_relations` binary): Rust read by its own grammar into rows. Kinds: `takes`, `returns`, `calls`, `uses-op`, `has-literal`, `has-word`, `part-of`, `has`, `holds`, `implements`. `has-word` is the words of a name (`count_above has-word count`, `has-word above`; a one-word name is its own word); `uses-op` the binary operators; `has-literal` the numbers as written (`0`, `0.0`, `2`).
- `phiano::relate::Counts`: the D2/D4 reading of rows. A row is counted under its kind (the prior) and under each condition told about its subject (its words, its types). Asked `kind` of something new under conditions `c₁…cₙ`, the reading is `ln prior(v) + Σ ln(distᵢ(v) / prior(v))`, each condition heard only if it moves the prior by τ = 0.5 nats (a post, D2). No operator, no epochs: one told row is learned the moment it is told.

## 1. A new function from its words: the base's scoreboard

`code_relations --held-out family` (and `file`) holds out whole answers of Phi-4's 253 (docs/118). The task states the name and the signature, so the held-out function's `has-word`, `takes` and `returns` rows are told; its body rows (`calls`, `uses-op`) are the test, those no other answer states. Pool: every answer named under the kind. The prior is the bar, as in every scoreboard of `phinet::relate`. By family (89 of 253 answers held out, 812 rows told, 162 asked; 60 epochs; R the knob):

| reading | R = 8 | R = 16 | R = 32 | R = 64 |
|---|---|---|---|---|
| prior (the kind's count) | 0.285 | 0.285 | 0.285 | 0.285 |
| operator, own (the new name's random function) | – | – | 0.278 | 0.278 |
| operator, inheriting from its words along `has-word` (docs/97 §1) | 0.259 | 0.279 | 0.299 | 0.313 |
| operator, two hops through its words | – | – | 0.313 | 0.298 |
| **counts, movement by the given words and types, posts at τ = 0.5 (D2)** | **0.346** | **0.346** | **0.346** | **0.346** |
| counts, every condition weighs (τ = 0) | 0.330 | 0.330 | 0.330 | 0.330 |

(held-out MRR; hit@5: prior 46.3%, inheriting at R = 64 51.2%, counts with posts 56.8%.) Per kind at R = 64: `calls` (106 asked) prior 0.266, inherited 0.306, counts 0.334; `uses-op` (56) prior 0.323, inherited 0.352, counts 0.370.

On our own crates (713 files, 25,893 rows told, 3,596 asked by file, R = 32, 20 epochs): prior 0.144, operator own 0.109, inheriting 0.119, two hops 0.118, counts with posts 0.143, counts τ = 0 0.135. Nothing beats the prior there; `calls` has a pool of 3,713 and a new function's words say little about which of them.

**Reading.** docs/92 is confirmed a third time: the shared operator learns the answer *type*, not the answer, and a new name's own function is noise until something structures it. Inheritance along `has-word` is the base's way of structuring it and it helps, growing with R (0.259 → 0.313), but it needs epochs and still trails the counts. The counts reading with D2 posts is the best line at every R, as it was in the builders' game (docs/113: the counts student catches bag of words and keeps the rules). The τ = 0 line below it says the same thing docs/111–113 said: a word that does not move the prior should not be heard. The readings themselves are what the glosses of docs/120 asked Phi-4 for, now read off the rows: `count_below (below count) · calls ⇒ filter, iter, chars, count`; `char_frequency · returns ⇒ … HashMap`; `all_even · calls ⇒ filter, iter, chars, all`.

## 2. The student (`code_student`)

One mechanism, D4 movement over `Counts`, in three steps; learning is instant.

1. **Which answer to say again.** The told answers are a kind (`said-again`, one value per answer: four `sum`s are four answers). The new name's words and the signature's types are the conditions; each condition the answer was told under moves the choice toward it by its count. A condition the answer has that the new name lacks is paired with one the new name has that the answer lacks — `above` for `below`, `i32` for `f64` — and only the surplus moves the choice away. The pairs are the proportion the writing will make.
2. **What to exchange.** Types by their place in the signature; the name by the name. A token of the body of kind `uses-op`, `calls` or `has-literal` is exchanged when the proportion holds in the counts: the answer's own condition moves to the old token beyond what the shared conditions and the new condition do, and the new condition moves to the new token beyond what the shared and the old do (to move is to double the odds). So `above : > :: below : <`, `i32 : 0 :: f64 : 0.0`, `max : max :: min : min_by` — found in the rows, not in a table.
3. **Worse.** The compiler (docs/97 §2): a body that does not compile is not said; among those that compile, one that leaves a parameter unused is worse than one that does not (a condition of the order not heard).

Measured on the 248 answered tasks with audited checks (docs/119), the teacher's own answers through the same checks:

| test | compiles | works | said again: 1st / 2nd / 3rd / 4th / later candidate | heuristic writer (docs/121–122) | Phi-4 |
|---|---|---|---|---|---|
| a new type of a known task (the task held out, its type siblings told) | 225 of 248 (91%) | **187 of 248 (75%)** | 161 / 40 / 11 / 0 / 13 | 212 of 248 (85%) | 224 of 248 (90%) |
| a task never seen (the whole family held out) | 172 of 248 (69%) | **36 of 248 (15%)** | 74 / 46 / 19 / 11 / 22 | 67 of 248 (27%) | 90% |
| one pointing (the family held out, one of its answers told) | 205 of 248 (83%) | **152 of 248 (61%)** | 162 / 13 / 5 / 4 / 21 | – | 90% |

**Reading.** The base student is ten points behind the heuristic writer on the first test and twelve behind on the second, with a tenth of the machinery and nothing a quest could not ask about. The one-pointing test is the one the project cares about (docs/116: a word from one pointing): shown one answer of a family, the student writes 61% of the siblings, where the never-seen test gives it 15%. The gap between those two numbers is what one answer teaches. Among what works by exchange: `count_positive(i64)` said again from `count_negative` with `negative : < :: positive : >`; `count_above` from `count_below` with `below : < :: above : >`; `min` from `max` with the call exchanged. What it writes wrongly is honest: `sum_below` from `count_below` keeps `count`, because in 253 answers `below` was only ever heard with `count` — the counts cannot tell the word from the function it came in. That is not a bug to patch with a rule; it is the word not yet known.

## 3. The chain of requests

A word told in fewer than three different functions is not known. `code_student --unknown` lists those words of the task names (50 of them in the 253 answers: `below`, `equal`, `first`, `last`, `zero`, `negative`, `mean`, `power`, …), and `code_quest --uses` asks Phi-4 to show each in three functions whose names say it, saved one per file under `uses/<word>/` and told with the rest; a use whose function is a held-out task's own is held out with it. The new answers bring new words, which are asked in turn: `scripts\phi4_code.ps1` runs the round three times, unattended. Nothing is hard-coded: what the student asks is decided by its counts, what it learns is decided by the grammar, and what it writes is decided by movement. This is the quest of docs/83 turned on the student itself.

## 4. What the operator is for

Not for this. The counts are the relations with their counts (the paradigm's own words), and they win on every test here. The operator's place is where counts cannot reach: the laws between kinds (docs/92–97, closure), the proportions between words that were never told together (the phasor difference `F(count_above) − F(count_below)` against `F(above) − F(below)`), and the size knob. The next measurement is that one: whether the operator, inheriting along `has-word`, finds `above : > :: below : <` when `below` has been heard once, where the counts need it heard three times.

`code_student --phi4 data\phi4_code [--by-task | --one-pointing] [--ask "fn …"] [--unknown words.tsv]` · `code_quest <out> --uses words.tsv ollama run phi4-builder` · `code_relations <dir> --held-out family --r 8,16,32,64`
