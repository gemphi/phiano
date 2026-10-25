# 179 · phicpl — the full structure, the related docs, and what is left

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The full directory structure (1,757 lines, every file under 200)

```text
crates/phicpl/                      the compiler crate — deps: phiecs alone
├── Cargo.toml
├── examples/
│   └── run.rs              29      judge any .b from the shell; --rows prints its O-code as rows
└── src/
    ├── lib.rs              49      the map of the crate; the doctest is fact(6) = 720
    ├── token.rs           165      the lexer: KEYWORDS (the one copy in the workspace), strings
    │                               with *n escapes, #x numbers, $( $) and { }, tagged sections,
    │                               // and /* */; newlines suppressed inside ( )
    ├── tree.rs             70      the AST: Expression, Command, Element, Declaration
    ├── parse/
    │   ├── mod.rs         151      the program grammar: SECTION/GET skipped, LET…AND,
    │   │                           MANIFEST/GLOBAL tables, the Parser helpers
    │   ├── expression.rs  168      precedence per Crick App. D: -> | & relations shifts
    │   │                           + - * / rem, monads - ! @ ~ abs, postfix call and a!i
    │   └── command.rs     131      blocks, IF/UNLESS/TEST, WHILE/UNTIL/REPEAT*/FOR,
    │                               RESULTIS/RETURN/FINISH/BREAK/LOOP, := and routine calls
    ├── trn.rs             135      the pass, keeping BCPL's name: slots, frames (0 saved P,
    │   │                           1 saved PC, params from 2), constants, op 0 a guard Finish
    │   ├── (trn/)express.rs 130    expressions → O-code: one value on the stack, always
    │   └── (trn/)order.rs 197      commands → O-code: stack left as found; loop patching
    ├── ocode.rs            76      the tiny intermediate: ~35 stack ops + Program
    ├── machine.rs         190      the word machine: one linear store, frames IN it (! and @
    │                               are real addresses), the global vector, step limit, no panics
    ├── global.rs           82      BLIB's heart: writef/writes/newline/writen/wrch as
    │                               negative global entries; output to a buffer the judge reads
    ├── judge.rs            55      execution as the judge: judge(src) → Verdict → rows
    ├── tell.rs             40      the compiled O-code told as rows: (fn, mnemonic, operand)
    └── tests.rs            89      the five conditions of satisfaction of docs/178, all green
```

Wired into the rest: `phiplm::code::bcpl::role` re-exports `phicpl::token::KEYWORDS` (one address); `phiano`'s Lisp has the 47th primitive `(judge "src")`; both crates list phicpl in their manifests; the workspace table gains the row "the compiler | crates/phicpl | …| answers: Richards' cintsys".

## The related docs, in reading order

- **docs/166** — BCPL measured against English by one reader: the 2% vs 24% open stratum, why code is learnable.
- **docs/169** — what we steal from BCPL: the five principles (one tiny intermediate; closed kinds over open names; the compiler written in itself; execution as the judge; typeless).
- **docs/170** — the three questions and their conditions of satisfaction (Q1 code structure, Q2 concepts, Q3 the person).
- **docs/172** — the one protocol (`Study`) and the student; the honest 37.1% BCPL number.
- **docs/176–177** — the arena and the split lane: every model on one stream, phifno entering honestly.
- **docs/178** — the phicpl plan: the architecture mapping, the five conditions, and the measured status.
- **docs/179** — this map.

## What is left, ranked

1. **The modelless learning loop** — the point of the whole build: a student WRITES BCPL (from the grammar it learned in docs/172), `phicpl::judge` runs it, what executed is kept, the verdict's rows feed the Tally/World. Every part now exists; the loop itself is unwritten. Condition to fix before building: the share of generated programs that compile AND run must rise as verdict rows are told back, against a floor of grammar-free token soup.
2. **phicpl's next language rungs**, each a test first: `SWITCHON/CASE/DEFAULT/ENDCASE`; byte strings (`%` subscript, `getbyte`, packed strings) so `GET "libhdr"` and the real `com/*.b` sources run; `GET` with a header path; local functions; multi-assignment; `TABLE`.
3. **Cross-validation against the real machine**: the same `.b` sources through our judge and through `cintsys64` (built by `make sys64` on your machine — the 32-bit image segfaults on 64-bit hosts), outputs diffed. Condition: byte-identical output on every source both can run.
4. **The live quest loop** (docs/174): `unsure` → `family_prompt` → ollama on your machine, answers told back; purity holds ≥85% while families pass 100.
5. **Q3 at scale**: `persona ingest` a real person's corpus in chunks, the Study curve rising chunk over chunk, no retrain; talk via the circus, query via `(ask friend i next)`.
6. **Arena growth**: BCPL beside prose in one table; batch lane entries for the phasor/facet readers; phicpl-judge as a reader's reward signal.
7. **External evaluation** (Kiro): entry points `student`, `arena`, `phicpl run`; the audit trail is docs/170–179.

## Status

2026-10-24. Everything above the line is built, tested (33 binaries, 0 failures) and synced; everything below the line has its condition stated before its build starts.
