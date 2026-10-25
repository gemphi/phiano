# 168 · phinet is the holder; no plurals in the types; BCPL answers simple questions

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. phinet is the workspace, not a crate

docs/160's facade crate is gone. **phinet is the holder** — the workspace at `gemphi/phinet` — and it distributes its modes to the existing crates; nothing under it wears its name. `crates/phinet` was removed: `sys` (Agent, Being, the race, the circus, the set game, the Lisp, the trace, the eye) moved into **phiano**, the agents-and-applications crate, as `phiano::sys`; the 256 personas moved to `crates/phiano/personas`; the examples (race, circus, think, sets, lisp) and the ring test moved with them. The workspace Cargo.toml now carries the one table, organised by use: the numbers (phistd) · the sphere (phifno) · the operators (phical) · the world (phiecs) · the eye (phioml) · the language model (phiplm) · the body (phikit) · the light (phiton) · the agents and apps (phiano). On the device, `crates/phinet`'s engine site (README, docs, torch, web) moved home to `crates/phistd`.

## 2. No plurals in the types

A set of sets is a set; a type is named for the one thing it is. The renames, workspace-wide (393→389 tests all green, 0 warnings):

| was | is | why |
|---|---|---|
| `phiecs::Relations` | `Register` | the register of kinds a world knows |
| `phiecs::Edges` | `Graph` | the both-ways index of edges |
| `phiecs::Conditions` | `ConditionSet` | one set |
| `phiplm::Counts` | `Tally` | one tally of what was told |
| `phiplm::Weights` | `Weighting` | one weighting over levels |
| `phiplm::Scopes` (strand) | `ScopeSet` | |
| `phiplm::Seeds` | `Seeding` | the seeding function |
| `phiano::RelationTypes` | `KindTable` | |
| `sys::sets` module, `Pairs` | `sys::set`, **`Relation`** | Russell's own word: a set of ordered pairs *is* a relation |
| example-local `Counts`/`Weights` | `CountModel`/`Weighting` | |

Left as they are, logged: phikit's Apple-mirrored names (`RunOptions`, `CaptureSettings`, `ReconstructionOptions`, `Semantics`, `Dimensions`, `Services`, `Coefficients`) — they port Apple's own names, and a port keeps its source's spelling; the tail of experiment-local record structs (`EpochMetrics`, `ArcResults`, `BaselineScores`, `Moments`, `MixtureTerms`, `Labels`, `Readings`, `SearleMarkers`, `FelicityConditions`, `KillerWords`, `NgramStats`, `LexicalRules`, `Sides`, `Resources`, `GenWeights`, `WordPosts`) — renameable any time, none is load-bearing.

## 3. BCPL answers, simple by simple

`phiplm::code::bcpl` (under 200 lines, 1 test) reads `.b` files the way prose is read — each `LET`/`AND` function a paragraph under `bcpl/<file>/<fn>`, rows by `prose::paragraph`'s own convention — plus the three relations the language declares: `declares`, `gets`, `calls`. Nothing replicated: the sets folder, the Lisp, the Tally and the ladder run on BCPL unchanged. The Lisp gained `--bcpl <dir>`.

Asked of the compiler's front end (`cintcode/com`, 4,889 scopes), simple questions, simple answers:

```lisp
(members (relata (next "scope:bcpl/bcplfe") "let"))
; what LET binds across the front end: lex, lookupword, newvec, trans, tree,
; rdblockbody, hashval, nextlab, start … — the compiler's working vocabulary
(count (meet (next "scope:bcpl/bcplfe") (next "scope:bcpl/bcplcgcin")))   ; 1571 — two compiler phases share a dialect
(count (meet (next "scope:bcpl/bcplfe") (next "scope:text/emma")))        ; 420 — BCPL and Austen barely overlap
(members (relata (next "scope:bcpl/bcplfe/rbexp") "resultis"))            ; a, mk1, mk2, mk4, rnexp — what the expression reader yields
```

Simple learning, simple questions, honest answers from counts alone — the regime docs/166 measured (2% open stratum) is the regime where this reader is already strong.

## 4. Status

Done 2026-10-24: the holder reorganisation, the singular sweep, `code::bcpl`, the Lisp over BCPL. 389 tests, 0 warnings, every file under 200 lines. Next: the circus with a `bcpl` persona; phi-4 quests; the phasor convergence.

## 5. Addendum (same day): BCPL is a persona

`crates/phiano/personas/code/bcpl.md` opens the **code ring** — languages as personas, told from their own source rather than quested from an oracle. The spec's `sources` field now works: `Being::from_quests` reads any `.b` directory a persona names through `code::bcpl::read_coarse` — the persona's grain, what each function `declares`, `gets` and `calls` (~50 rows a function), not the corpus grain of occurrence rows (~5,000 a function; the first attempt told those and took ten minutes — the coarse reader takes 0.6 s for 21,262 rows). `sys::find` searches the `stage` and `code` subrings too.

In the circus, BCPL speaks its compiler and the weights are counts of fact: `start — calls → writef (612)`. Against Wittgenstein's stand-in world it is all subject changes — they share no vocabulary yet, which `think bcpl bertrand-russell` states exactly: meet 0 on every kind. The honest reading: a conversation needs a shared relation, and the race, the circus and the set game all say so with the same zeros. When phi-4 quests the thinkers for real, the meet stops being zero at exactly the words they share.
