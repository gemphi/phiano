# phiano — the applications

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

phiano is where the model meets a task: the students (prose, code, the code reader), the games (the squares game, the language game, hexagrams), the personas, the story and its strata, the Phi-4 quests, the papers and the docs — sixty-eight binaries over the crates below it. It re-exports `phiplm::{relate, prose, code}` (the model), builds on `phical` (operators), `phifno` (the sphere), `phinet` (the engine), `phiecs` (words, relations, worlds, systems), `phikit` (the body, the space, the device) and `phiton`.

| where | what |
|---|---|
| `src/bin/story` | the story tree: told, held out, strata (counts, operator, kin, prior), the operator as base (docs/126–138) |
| `src/bin/context` | the context tests: one pointing, kept over 400 and 2,000 turns (docs/129) |
| `src/bin/code_student`, `code_reader`, `code_relations` | Rust read as rows, the student asked and taught |
| `src/bin/anatomy` | the body as a tree of words (phikit, docs/140–141) |
| `src/bin/strand` | a story as a strand of 64 levels, and its tests: `--bench`, `--synthetic`, `--cross`, `--read` (docs/142–145) |
| `src/bin/persona_*`, `src/persona` | personas: from a quest, ingested, in chat, evaluated |
| `src/bin/quest_*`, `relate_ask`, `src/bin/kind_operator` | Phi-4 observed into rows; kinds as operators |
| `src/facet`, `src/phasor`, `src/physics`, `src/helix` | the earlier phasor machinery, the exact physics (renamed from `phical`), the helix |
| `docs/` | the numbered docs, 1–141, each a measurement; `docs/papers/`, the papers with their figures |
| `data/` | the texts, the Phi-4 answers, the story's relations |

```sh
cargo run --release --bin context -- --turns 2000 --seed 2      # the context test
cargo run --release --bin story -- --heldout 0.05                # held-out Emma
cargo run --release --bin anatomy -- --ask "jaw open"            # the body's tree
cargo run --release --bin strand -- --read --trials 40 --seed 7  # the strand's units as reading scopes
```

phiano keeps its own git at `crates/phiano/.git`; the root repository tracks the other crates. Every doc opens with the paradigm paragraph and measures before it asserts.
