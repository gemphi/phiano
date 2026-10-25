# 196 · The fold — the kit comes home to phiecs, and phiecs to phikit

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The ruling

A crate must not share a name with the repo that owns it from outside ("you can't embed the same crate into itself"), the kit's content belongs to phiecs, and phiecs's home is phikit. So:

**The phikit crate is gone.** Its body, track and device live at `phiecs::kit` (`whole.rs` carries the `Kit` of rows; `tell`, `watch`, `words`, `body/`, `track/`, `device/` unchanged in substance). The row-builders came down with it, beside the one intermediate: `scopes`, `nest`, `SAYS`, `SCOPE` at `phiecs::scope`; `tokens`, `paragraph`, `END` at `phiecs::speak` — phiplm re-exports every one at its old address, so `phiplm::relate::scopes` and `phiplm::prose::paragraph` still answer and no caller changed. The counting half of the old Kit went UP, where counts live: `phiplm::relate::kit` (`Casting` — `knowing`, `cast` — and `said_by`), because the kit's rows are phiecs's business and reading them by counts is the measurement floor's. phiecs takes phiplm as a dev-dependency only (cargo allows the cycle for tests; the face test still reads frames back through `prose::Knowing`).

**phiecs is on phikit.** The reality-kit repo takes phiecs as its third submodule (crates/phiber, crates/phiray, crates/phiecs), so cloning phikit brings the worlds, the renderer, the physics and the phasor world that holds a body as rows. phinet keeps consuming the same phiecs repo as its submodule — one repo, two superprojects, no copy.

## Also this round

Every stale paradigm line — "a word (w) is nothing but what it relates to" — found and set right across 45 files: a word is a seed. READMEs written for phinet, phiano, phifno, phiecs, phiclm, phient, phiber, phiray, phikit, phiace, pedium, phihub, each honest about what is measured and what is not.

## Status

2026-10-25. 33 test binaries green after the fold; device tree synced (s196, md5-verified), crates/phikit deleted there. Commits: phiecs 527aa09, phiano 03f05e4, phinet cf97d76, phikit 19138b2 (with phiecs pinned at 527aa09). Pushes wait on GitHub access.
