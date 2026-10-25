# 178 · phicpl — BCPL's architecture, replicated as one of ours

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The name

CPL is the **Combined Programming Language** (Cambridge + London, 1963, Strachey) — too big to implement, so Richards cut it to the bootstrappable core in 1967: **Basic CPL, BCPL**, which begat B, which begat C. **phicpl is Phi CPL**: the same architecture, written as a crate of this workspace, so the lineage runs CPL → BCPL → B → C → … → phicpl.

## Why replicate it at all

docs/169 named five things to steal; three were already ours, one was cited, and the last — **execution is the judge** — needs a compiler and a machine we own. Emulating `cintsys` from the outside gives a binary; replicating the architecture gives the loop: write → compile → run → keep what executed, with a deterministic 1960s-sized judge instead of a 9 GB oracle, callable from any crate, on any machine, offline.

## The architecture, piece by piece, and where each file goes

| BCPL (Richards' distribution) | what it is | phicpl file | why there |
|---|---|---|---|
| `bcplfe.b` `declsyswords` + lexer | the closed vocabulary of structure; source → tokens | `src/token.rs` | ONE address for the keywords: `phiplm::code::bcpl::role` re-reads them from here instead of keeping its own copy (the Fourier rule of docs/173, applied to BCPL) |
| `bcplfe.b` syntax analyser | tokens → the tree | `src/parse/mod.rs`, `src/parse/expression.rs`, `src/parse/command.rs` | the grammar is three small grammars (program, expression, command); one file each keeps every file under 200 lines |
| the tree `trn` walks | the AST | `src/tree.rs` | one `Expression` and one `Command` enum — operators, not a class per node |
| `trn.b` | tree → **O-code** (the tiny stack intermediate) | `src/trn.rs` (BCPL's own name for it) | the translation is the compiler's heart; it keeps BCPL's name |
| O-code itself | ~40 stack ops | `src/ocode.rs` | the fifty-year-old proof that a tiny intermediate beats a big target — the same bet as `Row` |
| `cg.b` → Cintcode | bytecode compaction | **not built in v1** | BCPL itself bootstrapped by interpreting the intermediate directly (INTCODE); compaction is an optimisation, not architecture — it comes when a measurement asks for it |
| `cinterp.c` / `cintsys.c` | the machine: ONE word type, a linear store, P-frames, the global vector, `!` and `@` | `src/machine.rs` | the typeless word and `!`/`@` only mean something against a real addressable store, so the machine owns a flat `Vec<i64>` and frames live IN it |
| `BLIB` + the global vector | the library | `src/global.rs` | `writef`/`writes`/`newline`/`writen` as native globals with negative entries, exactly the global-vector trick |
| — (docs/169 §4) | **execution as the judge** | `src/judge.rs` | `judge(source) → Verdict{compiled, out, result, steps, error}` and the verdict told as rows: the learning hook every student uses |
| — (docs/169 §1) | the compiler readable by its student | `src/tell.rs` | the compiled O-code told as `Row`s, so the student reads what the compiler did, not just what the source said |

**Position in the ladder:** phicpl depends on phiecs alone (for `Row`); phiplm adds a dependency on phicpl (keywords, and later the judge in the code student). `phistd ← phifno ← phical ← {phiecs ← phioml, phiplm, **phicpl**} ← phikit ← phiano`.

## What v1 compiles and runs (and what it honestly does not)

Runs: `LET f(a, b) = e` and `LET r(a) BE c` (top level = global vector entries, `start()` the entry point as in cintsys); `VALOF`/`RESULTIS`; `TEST/IF/UNLESS … DO`; `WHILE/UNTIL/REPEAT … DO`; `FOR i = a TO b [BY k] DO`; `BREAK/LOOP/RETURN/FINISH`; blocks `$( $)` and `{ }`, tagged sections; local `LET`/`AND` mid-block; `:=`; the conditional expression `e -> a, b` (CPL's gift); calls as expressions and commands; `VEC k`, dyadic and monadic `!`, `@` of a local; `true/false`; `& | ~ = ~= < <= > >= << >> + - * / REM`, keyword forms `logand/logor/eq/ne/…`; strings with `*n *t ** *"` escapes; `//` and `/* */` comments; decimal and `#x…` numbers; `writef (%n %s %c)`, `writes`, `newline`, `writen`.

Not in v1, stated: `GLOBAL/MANIFEST` (parsed and honoured as names/constants but no separate compilation), `GET` (skipped — no header search), `SWITCHON/CASE`, string packing by bytes (`%`, `getbyte`; our strings are one char per word), multi-assignment, floating point. Each is a later rung with a test, not a silent gap.

## The conditions of satisfaction (fixed before building)

1. `fact(6) = 720` recursively and `sum 1..100 = 5050` by loop, through the whole pipeline (lex → parse → trn → machine).
2. `VEC` + `!` + `@`: write through an address, read it back.
3. `writef("fact(%n) = %n*n", 5, fact(5))` produces exactly `fact(5) = 120\n` in the captured output.
4. A source with an error **compiles false with a reason**, never panics: the judge must be callable on generated code.
5. The whole of phicpl under 2,000 lines — BCPL's own front end is 6,644 lines of BCPL; ours is smaller because the subset is, and the count is reported, not hidden.

## Status — built and measured, same round

All five conditions of satisfaction MET: (1) `fact(6) = 720` recursively and `sum 1..100 = 5050` by loop, through lex → parse → trn → machine; (2) `VEC` + `!` + `@` write through an address and read it back; (3) `writef("fact(%n) = %n*n", 5, fact(5))` yields exactly `fact(5) = 120\n` in the captured output; (4) a truncated source, infinite recursion, and `1/0` each return a Verdict with its reason — no panics; (5) **1,728 lines** against the 2,000 cap (BCPL's own front end is 6,644 of BCPL).

The power extracted, as asked: `phiplm::code::bcpl::role` now reads `KEYWORDS` from `phicpl::token` (one address, the docs/173 rule); the Lisp gained the 47th primitive `(judge "src")` — `(judge "LET start() = 6 * 7")` answers `(("judged" "compiled" "yes") ("judged" "returned" "42"))` — so every being can be TOLD a verdict's rows; `examples/run.rs` runs any `.b` through the pipeline and prints the verdict (`--rows` prints the O-code as rows). One machine detail worth keeping: op 0 is a guard `Finish`, so an unset global-vector slot (0) can never be entered.

Workspace after: 33 test binaries, 0 failures. Next rungs, in order: `SWITCHON/CASE`; string packing (`%`, `getbyte`) so `GET "libhdr"` sources run; then the loop that was the point — a student WRITES BCPL, the judge keeps what executed, the verdict rows feed the Tally: the first learning loop here with no language model inside it.
