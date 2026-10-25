# 195 · The organisation — one repo per phi-name

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## What changed

Every phi-name is now a repository of its own under github.com/gemphi, composed by submodules; the gemphi account is the umbrella — no meta-repo above it.

**phicpl → phiclm.** The compiler crate is renamed: CLM, Compiler Language Model. Directory, package, every `phiclm::` path in living code; the docs before this one keep saying phicpl because they are the record of when it was called that.

**phient, the new crate at the top of phinet.** "Phient is our CLM": the phasor student ([`phiano::arena::Ecs`]) coupled with the compiler it thinks with ([`phiclm::judge`]). The crate is the SEAM, stated at its true size: `tell` streams sentences in on the fly, `continue_from` continues a prompt in the learned voice (continuation, not comprehension), `compute` judges a program — what must be computed, not recalled. One test holds both halves: eight tellings of one sentence continue it, and `6*7` comes back 42 through the judge.

**The split.** phiano, phifno, phiecs, phiclm, phient each became a repository (phiano grafted onto its existing GitHub history — PR #1, `fix/phiano-capacity-and-objective`, stays in its past; nothing lost). phinet keeps phistd, phical, phioml, phiplm, phikit, phiton in-tree and takes the five as submodules at `crates/<name>`, so the workspace builds unchanged (path dependencies see the same directories).

| repo | holds |
|---|---|
| phinet.git | the workspace: in-tree crates + the five submodules |
| phiano.git | the crate, on top of its old history |
| phifno.git / phiecs.git / phiclm.git / phient.git | their crates |
| phikit.git | the reality kits (the phiace Zed fork renamed): phiber, phiray, phiace-robotics |
| puikit.git | the UI apps (`apps/phiegg`, `apps/phihub`, …) and the fonts |
| phihub.git | the hub; its app today is `puikit/apps/phihub` |
| phixun.git | to be RENAMED phixum — the folder is phixum, the repo name is the typo |

pedium.git is an empty shell with no local folder found; it waits for its seed.

## Status

2026-10-25. Local commits exist for phiano (a09d46a), phifno (7620992), phiecs (17e3b8a), phiclm (c3b4595), phient (fbe097a) and the phinet superproject; pushes wait on GitHub access (connect GitHub to Claude, or push from PowerShell). Workspace tests green after the rename and the new crate.
