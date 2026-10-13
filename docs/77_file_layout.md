# 77 · File layout: every source file under 200 lines

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. The code that says so is
now laid out the same way — small pieces that relate, rather than one large
file that tries to be the point.

## The rule

Every `.rs` file in `phiano`, `phinet` and `phi-cal` is at most 200 lines,
including doc comments and tests. `find src -name '*.rs' | xargs wc -l | awk
'$1>200'` must print nothing; it printed nothing for all three crates when this
was written (phiano 524 files, phinet 31, phi-cal 6).

## How a module is laid out

A module `foo` that outgrows one file becomes `foo.rs` plus a directory `foo/`:

- `foo.rs` keeps the module doc, the `use` lines, the `struct`/`enum`/`const`
  declarations, and one `mod x;` + `pub use x::*;` pair per piece. Its private
  fields are `pub(super)` so sibling files can reach them.
- Each piece is one topic: an `impl Foo` block with the methods for one job
  (`phasor/spectral_phasor_nudge.rs`, `trainer/trainer_train_sentence.rs`,
  `topdown/descent_votes.rs`), a free function, or a trait impl. A piece starts
  with `use super::*;`, and what it defines for its siblings is `pub(super)`.
- Tests live in `foo/tests.rs`, and in `foo/tests_<topic>.rs` when there are
  many; helpers shared between test files are `pub(super)` in `tests.rs`.
- A module doc longer than the code goes to `foo/README.md`, pulled in with
  `#![doc = include_str!("foo/README.md")]` (`relation_type`).

Binaries follow the same shape under `src/bin/<name>/main.rs`: `main` is a
short sequence of phases — `setup`, `train`, one file per experiment section,
`report` — and the state that flows between them is passed explicitly, as a
small `struct` when several values travel together. Printed output is
byte-identical to the single-file version it replaced.

Modules whose `mod.rs` predates this rule keep it; a few (`generate`, `bpe`,
`phinet::compose`, and phinet's `tensor`, `operator`, `phasor`, `joint`) are
bound with `#[path = "x/mod.rs"]` because a stale flat file may still sit beside
them on a checkout that could not delete files. Those stale files, and the
`src/bin/<name>.rs` stubs, are safe to delete: `autobins = false` and every
binary is declared by path in `Cargo.toml`.

## Why

A 1,490-line `metrics/harness.rs` was the largest file in the crate. It is now
`harness.rs` (a 199-line declaration file) and ten pieces, the largest 171
lines, and `cargo test --release --lib` still passes 220 tests with no
warnings. Nothing about the model changed; the layout now matches the claim.
