# 172 · The student, the one protocol, and discovery

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## What changed

**One protocol for every method.** `phiplm::study::Study` is the step-by-step learner's scorecard, shared by all subjects: predict FIRST, learn AFTER (prequential), so every position is unseen when scored and no number can be fudged by testing on what was trained. The stream is sliced in reading order into a curve; the condition of satisfaction (docs/170) is fixed in advance — beat twice the floor, and rise. `bcpl::learn` now runs on it instead of its own counting; so do the prose and person lessons. The three `learn_*` examples are gone; `examples/student.rs` is the one student, four lessons.

**Telling is a module.** `Knowing::fresh` + `Knowing::tell(path, rows)` (`prose/telling.rs`): a tree grows paragraph by paragraph in place, so a question asked between two tells is answered by everything told so far — the on-the-fly half of docs/132, now first-class. `learn_tree` and `learn_except` are thin loops over it.

**Curiosity is a module.** `prose/curious.rs`: `interest` (of the candidate paragraphs, the one with the most unknown words — what to read next; interest grows because every tell changes what is unknown), `place` (the scope whose vocabulary covers the words), `fit` (the scope that best READS the words, scored only on the closed stratum), `skeleton` (a sentence's known words — where its grammar lives, docs/157), `construction` (adjacent pairs of known words — "does not", "has not", "is not": the two-word units a grammar is made of).

**No two-word files anywhere.** Bins merged under their nouns with the verb as a subcommand: `persona <chat|ingest|quest>`, `quest <ask|ingest>`, `relate`; examples likewise: `code <quest|reader|relations|student|write>`, `net <more|next|residual>`, and the flat renames decode, flow, generate, hexagram, meaning, pairing, sides, impersonate, kind, builders, squares, demo, mirror, operator, sweep, persona, riley (In C), quest (phiecs), bench and profile (phistd).

## The student's four lessons (`cargo run --release -p phiano --example student`)

**Grammar under concepts (Q2, the cast).** Murphy's units told as a concept tree (`paragraph` rows under `grammar/<unit>/eN`), one example at a time, next word predicted UNDER its concept before each tell. 109 positions, floor 1.2%: curve 0.0 → 8.3% (+8.3), overall 2.8% — MET, small corpus honestly small. The practice questions against the book's Key: the one cue family resolved by concept — "it is not"→raining ✓, "it did not"→rain ✓, "it has not"→rained ✓; "it does not"→drink ✗ (key: grow — "julie does not drink" outweighs "rice does not grow" inside the unit; a real miss, kept).

**A person, line by line (Q3).** A Being told five lines, next word-choice predicted before each: 34 positions, floor 4.2%, curve 0 → 40% (+40) — on the fly, nothing retrained.

**Code by depth (Q1).** BCPL, 4746 functions online: 1,049,977 positions, floor 16.7%, overall 37.1% (2.2× floor), first slice 36.0% → last 41.0% — nesting max 328, mean 29. Note against docs/170: the ~46% there was the shallow role-trigram reading, which saturates at 36 contexts; the depth-16 building reader measures **37.1%**, consistent with docs/170's own sweep finding that a single deep path over-specialises past depth 8. 37.1% is the honest number for this reader; the lift the hierarchy owes must come from backing off ACROSS levels (the cast), not from one deep n-gram.

**Discovery (structure can only be discovered).** The same examples WITHOUT their unit labels. The student listens first in the order its own interest picks, then groups by shared RECURRING constructions (union-find over adjacent known-word pairs seen in ≥2 sentences). Every step on the way is kept, because each wrong turn taught something:

| grouping rule | scopes (21 ex) | purity | scopes (41 ex) | purity |
|---|---|---|---|---|
| vocabulary overlap ≥0.5 | 20 | 95%* | — | — |
| vocabulary overlap ≥0.3 | 14 | 71% | — | — |
| cast `fit` ≥0.25 + overlap | 18 | 86% | 22 | 76% |
| skeleton Jaccard vs union | 11 | 62% | 21 | 88% |
| skeleton core-intersection | 11 | 62% | 25 | 76% |
| recurring constructions | **17** | **90%** | **22** | **85%** |

*95% at 20/21 scopes is singletons — trivially pure, i.e. fudged-looking; reported to show why purity must be read against scope count.

What the iterations taught: (1) same-unit sentences share grammar, not vocabulary — overlap fragments; (2) the cast's next-word fit is 0.0 early because the closed stratum doesn't exist before reading — listen first, then organise; (3) skeletons carry the grammar (has/did/is/are/not) but drown it in pronouns and prepositions, and "do/does" sits BELOW the KNOWN=3 cut — the structure words are mid-frequency; (4) the construction is a PAIR — (does,not), (did,not), (has,not), (she,is), (we,are) — and recurring pairs link the tense families correctly, while collocations like (a,lot), (to,the) cross tenses and cause the remaining impurity. That last point is not noise: prepositional collocations are a different concept axis (Murphy has units for them too) — **meaning's hierarchy is a DAG, not a tree**, and a sentence sits under several concepts at once. That is exactly what the cast's multiple scopes are for, and it is the open edge.

## Where the questions stand

Q1 MET (structure learnable online, by depth). Q2 half-met: resolution under GIVEN concepts works (the practice table, 3/4 with one honest miss); label-free DISCOVERY of the concepts finds the construction families but fragments on crossing axes — next: let one sentence join several scopes (the DAG), and let the quest fetch more uses of the specific constructions it is unsure about (`unknown` → `uses_prompt` → ollama on the machine). Q3 mechanism proven twice (learn_person before, the person lesson now, +40 pts over 5 lines).

References beside us in `gemphi/ext`: `context-language-models` (Shao 2026) learns by EDITING one context — our tree-of-counts is the same bet with counts instead of text; `makani`/`neuraloperator`/`torch-harmonics` train operators offline by epochs — the opposite protocol, kept as the contrast line, not the method.

## Status

2026-10-24. Workspace: 225+ tests green, every crate's examples build, no two-word file names, no plural type names. The student is the one entry: `cargo run --release -p phiano --example student`.
