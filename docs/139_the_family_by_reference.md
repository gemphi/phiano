# 139 · The family laid out by what it read: phinet, phifno, phical, phiplm, phiano

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Yes, we borrowed — and now each crate says from whom

Three libraries were cloned and read for this project, and the ideas in them are in our code: Li et al.'s Fourier neural operator (`neuraloperator`) is the operator phinet has trained since docs/78; Bonev et al.'s spherical harmonics and the one-weight-per-degree tie (`torch-harmonics`, the engine of FourCastNet 3 in `makani`) are `Tie::Sphere` and the sixty-four harmonics of docs/84; and Shao et al.'s Context Language Models (`context-language-models`) are the comparison paper 24 and paper 25 were written against — their context is our condition. Until today the copies lived only in the container where they were read, and the crates that answer them were laid out by history, not by reference: the operator, the joint and the relation operators sat in phinet beside the tensor engine; the sphere sat in phinet while phifno re-exported it; phical held only exact geometry; and the language model itself — counts, levels, the cast, the field, kin — was a module of the application crate.

This doc lays the family out one crate per reference, so that a reader who knows torch, torch-harmonics, neuraloperator or the CLM repository knows where to look, and a reader of ours knows what each crate owes. The copies are on disk under `gemphi\ext\`, each with a `PROVENANCE.txt` (source, commit, when cloned, which of our docs read it).

![The family, one crate per reference library.](papers/fig/f25_family.png)

| crate | holds | answers | depends on |
|---|---|---|---|
| **phinet** | the engine: complex `Tensor [b, w, s]`, `Tape` autograd, unitary FFT, `Adam`, `Function` helpers (`normalise`, `unit_random`, `unit_phase`, `cosine`, `stack`, `fnv1a`, `mix64`), `hrr`, `hexagram`, the phasor ranges | torch | — |
| **phifno** | the sphere: the 64 real spherical harmonics of degree < 8, Gauss–Legendre × equiangular grids, analysis/synthesis, degree gate and energy, `Word` on the sphere | torch-harmonics | phinet |
| **phical** | the operators: `Fno` with `Tie::{Circle, Sphere, Anisotropic}` and `FnoConfig::{for_space, sphere, anisotropic, circle64}`, `Joint`, `relate` (laws, closure, `Inherit`, score), `liskov`; and the exact geometry (Riemann sphere, Möbius, Q64, winding, heat) | neuraloperator, makani — and our quantum operators | phinet, phifno |
| **phiecs** | the entity-component world for words; the quest | — | — |
| **phiplm** | the Phasor Language Model: `relate::{Counts, levels, ScopeTree, Field, kin, RelationModel}`, `prose::{Knowing, teach, heldout, unknown}`, `code` | context-language-models | phinet, phical, phiecs |
| **phiton** | light as the unit system | — | — |
| **phiano** | the applications: students, games, personas, servers, 67 binaries; re-exports `phiplm::{relate, prose, code}` | — | all of the above |

Dependencies run one way, `phinet ← phifno ← phical ← phiplm ← phiano`, with phiecs under phiplm and phiton under phiano. No crate reaches up.

## 2. What moved, and what did not change

- `phinet::sphere` → `phifno::sphere`, with its tests and `examples/sphere_dump.rs`, `sphere_parity.py`. phifno no longer re-exports the operator; its three operator configurations became `phical::FnoConfig::{sphere, anisotropic, circle64}`, and the degree-filter example moved to phical as `degree_filter`.
- `phinet::{operator, joint, relate, liskov}` → `phical::{operator, joint, relate, liskov}`, with the operator examples (`bench`, `profile_apply`, `profile_softmax`, `ranges`, `torch_reference`). `cosine`, `stack` and `unstack` stayed in phinet, where `hrr`, `hexagram` and the phasor ranges need them; phical re-exports the engine pieces its operators are written against, so `crate::` paths inside them and `phical::` paths outside read the same.
- The package `phi-cal` is `phical` (lib `phical`), 0.2.0; phiace's `apple` crate, which depends on it by path, is pointed at the new name.
- `phiano::{relate, prose, code}` → the new crate `phiplm`, re-exported from phiano under the same paths, so every binary and script reads as before. The one entanglement cut: `RelationModel::new` took phiano's `Facet` to seed a word's function from its angles; it now takes `seeds: Option<&dyn Fn(&str) -> Option<Function>>`, and phiano's facet offers `operator_seeds()`. `fnv1a` is phinet's now (phiano's re-exports it), so every seed in the family is one hash.
- phiano's own module `phical` — PhiTon's physics calculus of colour and time, misnamed for the crate — is `phiano::physics`, so the crate name is unambiguous inside phiano.
- 42 files in phiano changed an import line and nothing else.

Nothing in any reading changed: the context tests (400 turns: 10/10 · 20/20 · 20/20, 5 ms), the story said back from "She suddenly", the code student's `count_odd` answer, and the workspace's 355 tests and zero warnings are as they were in docs/138. Every source file is under 200 lines, no module is named after its directory, nothing dangles.

## 3. Why one crate per reference

A crate is a namespace with a boundary, and a boundary is a claim: what is inside stands on its own. Laying the family out by reference makes three claims we can defend. phinet stands without the operator — it is an engine, and the engine's tests do not know what an FNO is. phical stands on phinet and phifno the way neuraloperator stands on torch and torch-harmonics, and it owns every operator, learned or exact, so a reader who wants "the operators" opens one crate. phiplm stands on the three and on phiecs with no application in it: the Phasor Language Model of paper 25 is a library, 2,100 lines, that a program other than phiano can use — which is what the CLM repository is to its harness, and what we were missing.

`cargo build --release` at `gemphi\phinet`; `cargo test --workspace`; the binaries land where they did.
