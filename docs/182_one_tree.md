# 182 · One tree

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## Round 182: the workspace reconciled, both directions

Cause: renames had been landing on the machine as additions, so old and new copies coexisted there; the duplicate example target (student.rs beside student/) stopped cargo entirely.

Taken IN from the machine and now building here: phical's examples (torch reference with its extra module, the degree filter, the two profilers, bench, ranges), phifno's sphere dump, phiecs reparse, phikit ask and watch, phiano english and the cast experiment — all repointed from the old phinet crate name to phistd, and renamed to single words: filter, apply, softmax, torch, dump, cast.

Parked in _to_delete/round182 on the machine (stale pre-rename copies, reachable from no crate root): phiano's old data/splits, kinds/kinds, metrics capacity/regularization/split/detection-shift, bin/rust/lite, the ch14 integration test (it imports a module that no longer exists), and phiplm's old relate mirror/model/wave trio.

Then the whole canonical tree — every crate's Cargo.toml, src, examples, plus docs and personas — mirrored to the machine and spot-verified by md5 on eight files across five crates: identical.

## Status

2026-10-24. Workspace builds with every example, 33 test binaries, 0 failures, no two-word file names, no orphans. The machine's tree now equals this one; cargo run --release -p phiano --example student works again.
