# 177 · The split lane, and the module that was never in the build

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The split lane

docs/176 left phifno out of the arena, on purpose: the Joint operator is trained by epochs over a table of functions, so it cannot guess-then-learn one position at a time, and a fabricated online number for it would be worth nothing. This round gives it an honest way in rather than a number.

`Arena::read_split(lines, train)` tells the first `train` lines to every reader and scores them for nobody; then calls `Reader::ready()` — the one new line in the contract, a no-op for a counter — where a batch model trains once on everything it has been told; then scores every reader on the rest. The operator gets its epochs and the counters are scored on exactly the same positions, so the table is still one comparison.

`phiano::arena::operator::Operator` is the reader: rows `(w, next, u)` collected while told, then `RelationModel` over a table of functions, examples through `phical::relate::example`, trained with Adam for `--epochs`, read by `answers(word, "next", pool, 1)`.

## The run — five models, one stream

3,000 sentences of real English, 1,500 told first and unscored, 4,510 words of vocabulary, **13,735 scored positions**, floor 0.02%, 69 seconds, 10 epochs for the operator.

| model | top-1 | first → last slice | lift | verdict |
|---|---|---|---|---|
| phiplm concepts (the family, backing off) | **32.8%** | 32.5% → 34.2% | +1.7 | learning |
| phiplm counts (the word before) | 30.5% | 29.5% → 32.1% | +2.5 | learning |
| phiecs (rows into a World) | 30.5% | 29.5% → 32.1% | +2.5 | learning |
| phifno (the Joint operator, batch-trained) | 18.1% | 18.9% → 18.1% | −0.7 | saturated |
| phical conjugate (the relation as a filter) | 0.3% | 0.3% → 0.3% | −0.0 | saturated |

**phifno learns, and loses.** 18.1% is 900× the floor, and it grows with the budget it is given — 11.6% at 3 epochs, 19.6% at 10 epochs on 1,200 lines, 18.1% at 10 epochs on 3,000 — so nothing is wrong with the operator; it is simply beaten by counting on open-vocabulary next-word top-1, while being the only reader here that is *compressing* (a table of functions and one operator against a count table that grows with every pair seen). The advantages run the other way too and are printed under the table rather than hidden: phifno and phical are both handed the vocabulary in advance, and phifno trains by epochs on a half the counters read once.

**The concept hierarchy leads again** (+2.3 over the counts), as in docs/176, now on the split lane as well — the result does not depend on the lane.

## The module that was never in the build

An audit for unreachable files — every `.rs` whose stem no crate root declares — found the workspace clean and `crates/phiano/src/extension/` (written 12 October, documented in docs/90 §2: channels past the base 64, answer slots so `cat · is-a` cannot tie `mammal` with `allergies`) declared by nothing, in no build, for twelve days. It is now `pub mod extension;` and its three tests pass. The only repair it needed was its import: the dithered quantiser it was written against had become private inside `phasor::dither`, so `phasor` re-exports it `pub(crate)` again under the name the module expects. Functionality that is not in the build is not functionality, and the audit is cheap enough to repeat.

## Status

2026-10-24. `Reader::ready` + `Arena::read_split` (tested), `phiano::arena::operator`, `phiano::extension` back in the build (228 tests in phiano, 0 failures), workspace 31 test binaries ok. `cargo run --release -p phiano --example arena -- <dir of .txt> [--lines 3000] [--epochs 10]`.
