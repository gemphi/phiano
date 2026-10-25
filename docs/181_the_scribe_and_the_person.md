# 181 · The scribe — a language is a person; only the kinds differ

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The idea, the user's words

A programming language is also a person: the difference is KINDS only — the rest is the same, in the sense of doing things with words. Logic is doing things with words in both cases. A program is a speech act whose felicity condition is execution; the judge (docs/178) is to BCPL what the hearer is to a promise. So the writing student is the same machinery as every persona here — counts told in place, asked by kind — and at the end of this round its deeds ARE told to a `Being` with kinds `compiled, returned, wrote, failed`, and asked like anyone: `(ask scribe p0 returned)` answers from rows the judge wrote.

## The build: `phiano::scribe` (371 lines, one address)

- `memory.rs` — the token memory of docs/180, promoted from the example: depth-2 counts backing off to depth-1, exploration, `write`.
- `mold.rs` — the construction level. A kept program's MOLD is its skeleton with the open stratum as holes (docs/157 read for writing: names and numbers are BCPL's open stratum). **A name-hole carries its binding class** — every occurrence of one original name is one class — so filling renames CONSISTENTLY: substitution, not independent choice. `start` stays fixed (the runtime's own convention, closed stratum of the system). Numbers are free. At a small rate a mold is mutated through the token memory — how new buildings are minted.
- `mod.rs` — `Scribe`: both memories kept in step; `keep` returns whether the SKELETON was new (structural novelty, the stricter measure).

## Measured: three arms, one table, 10,000 rounds each, 0.45 s

| arm | ran | rate first→last | novel/1k | distinct kept | new skeletons |
|---|---|---|---|---|---|
| soup (no learning) | 0.0% | 0.0 → 0.0% | 0.0 | 0 | 0 |
| token (depth-2, docs/180) | 6.8% | 1.4 → 6.6% | 7.5 | 75 | **72** |
| mold (substitution) | **79.5%** | 78.0 → 80.5% | **252.0** | **2,520** | 25 |

And the step that got there is kept in the record, because it is the finding: the FIRST mold arm filled every name-hole independently and scored **0.5% with zero new skeletons** — worse than tokens — because the leaves of a program are not free, they are BOUND by their declarations. One principled change (consistent renaming per binding class) multiplied the run rate ×159 and the novelty ×51. Nothing else moved. The open stratum of code is substitutable only along its bindings — docs/157's lesson arriving a third time, now in writing.

The honest split that remains: the token arm still mints more NEW skeletons (72 vs 25) — wandering explores, substitution exploits. The two levels do different jobs, and the scribe needs both: molds fill proven buildings at scale, the token memory (through mutation and fallback) proposes buildings the molds don't have. That is the complete learning-system model running today, with no language model anywhere in it: shown two uses → writes → judged → novel keeps told back in place at both depths → its deeds told to a Being → asked by kind like any person.

## Conditions, against docs/179's fixed bar

Rate rises and beats soup: MET in both learning arms. Mold beats token on distinct programs: MET ×33. Mold beats token on new skeletons: **NOT MET — reported straight**; minting stays the token memory's job, and the next rung is letting a MINTED skeleton join the mold set mid-run and watching the two curves couple.

## Status

2026-10-24. `phiano::scribe` (memory/mold/mod, all under 200 lines), writer example a 137-line three-arm runner, workspace 33 test binaries, 0 failures. `cargo run --release -p phiano --example writer -- 10000`.
