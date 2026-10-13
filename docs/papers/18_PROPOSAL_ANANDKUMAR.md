# Proposal: a spherical operator for words — Phient × Anima Anandkumar

*Prepared by Phient, 2026-09-27. Nothing here has been sent or agreed; it is
what we would put in front of her, with every number one she could re-run.*

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition.

## 1. Why her

Professor Anandkumar (Bren Professor, Caltech; UN Scientific Advisory Board,
March 2026) made the two things Phient runs on. The **Fourier Neural Operator**
(Li et al., 2020) is the engine of phinet — our operator learns in the
frequency domain over 64-channel word functions exactly as an FNO learns over
a PDE's solution. **FourCastNet** (2022) and its spherical successors put an
operator on the sphere and made a week's forecast a two-second job; phifno
1.0.0 puts the same construction — one complex weight per spherical-harmonic
degree — on a word. Her stated programme, *"foundation models for physics, not
just language"* and *"take what works in deep learning and make it
principled"*, is the programme Phient's measurement pages try to keep. A
full profile is in `17_ANIMA_ANANDKUMAR_PROFILE.md`.

## 2. What we bring

- **A different function space, same operator family.** FourCastNet's function
  is the atmosphere on S²; Phical's is a word: 64 relation channels, each a
  phasor with a count, source and target sides kept apart. The operator is
  trained to move a word's function toward the next word's and ranked on
  held-out text — 30.7× chance MRR on Austen (softmax, wide+deep, phinet
  docs/78), 62/62 parity with a PyTorch reference.
- **The sphere already works on it.** `phinet::Tie::Sphere`: 64 channels read
  as `Y_l^m`, `l < 8`; one weight per degree; 1,192 parameters against the
  circle's 8,360; 88–90% of the circle's ranking on Austen in 63% of the time,
  and exact (MRR 1.000 vs 0.662) where the relation is per scale
  (phiano docs/84). A rotation-equivariant operator has never been asked to
  rank words.
- **A learning loop with a known answer.** phiecs's quest asks an oracle
  (Phi-4 locally, or a synthetic structure) one word at a time and writes a
  relation table; `quest_ingest` puts it on the kind's channel and reads it
  back at 97% hit@5 — after the byte-quantisation deadband it found
  (docs/83) was fixed with dithered rounding. Everything is small, in Rust,
  dependency-free, and re-runnable in minutes on a laptop.
- **Sixty-four.** Channels, harmonics of degree < 8, hexagrams, codons: the
  lexicon writes as a helix of codon files a shell can query (docs/84 §2).
  It is a file format, not a metaphor, and it makes the model inspectable in
  a way weights are not.

## 3. What we ask

1. **One reading.** An hour with phinet docs/76–78 and phiano docs/83–84, and
   her view of whether a per-degree tie on a word's channels is the right
   equivariance, or whether relation *kind* should place a phasor on the
   sphere first.
2. **One joint experiment.** The spherical operator at FourCastNet 3's
   settings — anisotropic tie per `(l, |m|)`, softmax objective, width 16 —
   against the circle at equal parameters, on the Austen ranking task and
   on a second corpus of her choosing. We run it; she names the controls.
   Success is a number either side can reproduce; failure is reported the
   same way (docs/78 reports the cosine schedule that did not help).
3. **Torch-harmonics parity.** phinet's `sphere` against `th.RealSHT` on the
   same grid to 1e-9, added to the 62-test parity suite, so anything we claim
   on the sphere is claimed in her library's terms.
4. **A name on the paper if it earns one.** Authorship follows contribution;
   the proposal is for co-authorship of the operator paper only if the joint
   experiment is hers as much as ours.

## 4. What we do not ask

Endorsement, time beyond the reading, compute, or her name on anything she
has not read. Phient has no weather results and claims none; the connection
is the operator and the sphere, not the forecast.

## 5. The first message (draft)

> Professor Anandkumar — we build small Rust neural operators over a function
> space you have not seen: a word as 64 spherical-harmonic coefficients, one
> complex phasor per relation channel. Your FNO is the engine; your spherical
> FNO, put on a word instead of the atmosphere, ranks next words at 88% of the
> circle's accuracy with a seventh of the parameters (numbers and code
> attached, re-runnable in ten minutes). We would value one reading and one
> joint experiment with controls you name. — Phient

## 6. Attachments

`phinet` (engine, 37 tests), `phifno` 1.0.0 (sphere, `examples/filter.rs`),
`phiano/docs/78`, `83`, `84`, `phiano/docs/papers/17` (her profile, sourced),
`data/sweep/results.md` (every arm, with wall-clock).
