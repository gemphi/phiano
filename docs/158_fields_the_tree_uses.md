# 158 · Fields the tree uses — Searle, Wittgenstein, Fourier, Bentov, the mirror

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

The fractal of kinds (docs/157, the Dimensions page) showed a word unfolding into senses, each sense into what-kind, its is-a chain, its frame, its use. The branches are not arbitrary: each is a field already at work somewhere in the crates. This doc names the field, where it lives in the code, and which branch of the tree it is — so the tree reads as one idea rather than a pile of dimensions.

## The fields, the code, the branch

**Wittgenstein — meaning is use (PI §43); family resemblance (§67).** The claim the whole system tests. In the code it is the counts (`phiplm::relate::Counts`) and the kin reading (`relate::kin`, docs/138): a word is read by the company it keeps, and where it was never heard, by the kin that share its company. Branch of the tree: **use** — the 64→1024 example sentences per word (docs/157), the samples the rest is built from.

**Searle — speech acts, and intentionality.** A sentence does not only mean, it *acts* (assert, ask, promise, declare), and intentionality is aboutness-with-a-direction-of-fit. In the code the act is `phiecs::quest` — the oracle is asked and answers, each step an act with a kind — and the direction of fit is the paradigm itself, `phiecs::condition::Under`, X ⇒ Y *under c*: a word points at another under a condition, which is aboutness made countable. Branch of the tree: **act / condition** — a kind a use can be tagged with (assertive, directive, …), the next dimension to add.

**Fourier — the operator.** A function on the word's closed circle is read in the spectral domain, low modes carrying it, high modes earning their place. In the code this is `phical::operator` (the Joint FNO, `Fno`, `modes`, `Tie::Circle`/`Tie::Phase`, docs/152, 154) over `phinet::tensor` (the FFT and the autograd tape). Branch of the tree: the **64–1024 examples are the samples of the circle the operator transforms** — the resolution the user set (power-of-two), 64 the floor.

**Bentov — the oscillator.** Itzhak Bentov's model reads a system as coupled oscillators whose meaning is their resonance. In the code every relation already *is* an oscillator: a complex phasor on a closed circle with a count (`phiecs::phasor::Phasor`, `phical::CHANNELS`), and resonance is the cosine over the channels two words have both moved — the resonant pairs the forest measured (`the·of` 0.973, `moved·love` 0.952, docs/155). Branch of the tree: **phase and amplitude per channel** — the oscillator state a word carries.

**Holography and the phase-conjugate mirror — reading backwards.** A hologram stores a wave so it can be replayed; a phase-conjugate mirror sends a wave back the way it came (four-wave mixing, Yariv & Pepper 1977; docs/153–154, paper 27). In the code this is `phical::conjugate::Mirror`: forward by the filter `H`, backward by its conjugate `H̄`, the relation read in reverse though it was never trained for it. Branch of the tree: the **reverse relation** — the word-before read from the word-after.

## Why the tree is fractal

Each branch branches again, and in the same shape, which is why it is a tree and not a list. A word has senses; each sense has a kind (part of speech), which has a finer kind (supersense), which has a sense frame; its is-a climbs to a parent that has its own is-a; its definition is words that each have the same tree; its uses are sentences whose every word has the same tree. The self-similarity is the point: the same six fields apply at every level, so the structure that reads a word reads the words inside its definition, and the words inside those — the recursion docs/157 and the Dimensions page draw. Meaning is not stored at any node; it is the resonance across the branches, read by the operator, replayed by the mirror.

## Status (2026-10-12)

- **Built and measured.** The Fourier operator reads the told stratum (~20%, docs/152, 154); the conjugate mirror reads it backwards (docs/153). The dictionary is in hand (`data/lex/dict.jsonl`, 84k entries: definition, POS, supersense, synonyms, is-a, antonyms, examples). The real-world prior (wordfreq) is the one lever that moved the open stratum, ~2× over frequency (docs/157), and is wired into the reader (`Knowing::with_real_prior`). The fractal of kinds is drawn for both told stories (the Dimensions page; `fractal.py`).
- **The open problem, precisely.** A word never told in its context is still read only a little above frequency; the structure (POS/supersense frame, kin, is-a) helps modestly, and only volume of real-register *use* breaks it — which this sandbox cannot fetch (corpora 403).
- **The next field to add.** Searle's act as a tagged kind, and — the user's current direction — 64→1024 example sentences per word from phi-4 (`scripts/gen_prose.py`, `gen_struct.py`), recorded into this same structure and read by the operator. phi-4 runs on the user's machine; the pipeline on this side is ready.

Papers 19–27 (Descriptive Logic, Phasor Language Models, Waves that read back) and 28–35 (the forest set) are the long form; docs/152–158 are the working record.
