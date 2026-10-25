# 84 · The word on the sphere, and the lexicon as a helix

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. Sixty-four is the
number this note is about, because it is not one number but four: 64 relation
channels, 64 real spherical harmonics of degree < 8, 64 I Ching hexagrams
(phinet already reads a spectrum's octaves as one), and 64 codons of DNA.

The family, as the user names it: **phinet** is the parent — tensor, tape,
the operator — and its children read the same 64 channels their own way:
**phical** (the paradigm), **phinum**, **phiqlm** (quantum), and now
**phifno 1.0.0** (the spherical operator). Together, PML: Phi machine
learning. Phiano is the word-level application; phiecs the entity-component
world; the quest the way it learns from an oracle.

## 1. The sphere (phinet::sphere, phinet::Tie::Sphere, phifno)

The spherical Fourier neural operator (NVIDIA `torch-harmonics`, `makani` —
FourCastNet 3's engine, cloned and read on 2026-09-27) forecasts the
atmosphere as spherical-harmonic coefficients and learns **one complex weight
per degree `l`**, shared by the `2l+1` orders, so the learned operator
commutes with every rotation of the globe. `Σ_{l<8}(2l+1) = 64`: a word's 64
channels are also a band-limited function on the sphere, `k = l² + l + m`.

What was built: `phinet::sphere` — the orthonormal real harmonics, a
Gauss–Legendre × equiangular grid, analysis/synthesis exact on the band
(1e-9), rotation about the pole, a degree gate (resolution change on the
sphere), degree energy (a rotation-invariant spectrum); `Tape::tie` (expand
eight weights to sixty-four, gradient summed back, finite-difference
checked); `FnoConfig::tie = Tie::Sphere` (no FFT, the samples *are* the
coefficients, `modes` = degrees kept; `#[serde(default)]`, so every saved
operator still loads). phifno is the child crate that names it.

Measured:

| task | circle (FFT, 12 modes) | sphere (one weight per degree) |
|---|---|---|
| params | 8,360 | **1,192** |
| degree filter, 200 items, 40 epochs (`phifno/examples/filter.rs`) | MRR 0.662 (identity 0.646) | **1.000** |
| Austen next word, hinge, 3 epochs, pool 1,228 (`bin/operator_sweep`) | 14.7× / 14.6× chance, hit@10 0.216, 142 s | 12.8× / 13.1×, hit@10 0.182, **89 s** |
| Austen, softmax β=8, 3 epochs | 18.0× / 18.8× (8,360 params, 395 s) | 13.2× / 13.5×, hit@10 0.189 (408 s) |
| Austen, hinge, 3 epochs, **anisotropic tie** (one weight per `(l, |m|)`, 36 slots, 4,776 params) | — | **16.0× / 15.6×, hit@10 0.254, 85 s** |
| Austen, softmax, wide+deep (width 16, 3 layers), 3 epochs | 27.6× / 27.6× (50,016 params, 1,157 s; `results_3ep.md`) | anisotropic: 16.5× / 16.6×, hit@10 0.257 (28,512 params, 912 s) |

Where the relation is per scale the sphere is the exact operator and the
circle is not; on Austen the isotropic sphere keeps 88–90% of the circle's
ranking with a seventh of the parameters, and the softmax objective that
lifts the circle from 14.7× to 18.0× lifts it only to 13.2×: eight weights
per channel pair is a ceiling, not a start. The **anisotropic tie** — one
weight per `(l, |m|)`, cosine and sine orders sharing, 36 slots — is the
finding: **16.0× valid / 15.6× test with 4,776 parameters, beating the
circle's 14.7× / 14.6× at 8,360**, in 85 s against 142. Reflection symmetry
(`φ → −φ`) is worth keeping on a word; full rotation symmetry is not. The
shipped operator (softmax, wide+deep, 30.7×) is still the circle. At full size it does not: the anisotropic tie at softmax, width 16, 3 layers reaches **16.5× / 16.6×** with 28,512 parameters against the circle's **27.6×** at the same shape and epochs. Width, depth and softmax bought the anisotropic tie half a point; they bought the circle thirteen. The likely reason is that the circle's FFT runs along the channel *index*, and phiano's channel order is meaningful (offsets +1…+8 in order, then marker relations ranked by frequency); reading the 64 channels as `Y_l^m` discards that order. The circle stays the shipped operator; the sphere ties are a small-model option (16.0× at 4,776 parameters beats the circle at 8,360) and a ceiling above it. A rotation of
the sphere itself is *not* what a per-degree operator can do (a phase per
order is), which the first version of the test got wrong and the shipped
test says so. `phinet::sphere` agrees with `scipy.special.sph_harm_y` to
2.4e-15 on all 64 harmonics over the 8 × 16 grid (`examples/sphere_parity.py`).

## 2. The helix (phiano::helix, bin/helix)

DNA stores no point: a gene is 64 codons — three bases of four — and what a
codon means is fixed by where it sits on a helix that puts every position in
three-dimensional space. `bin/helix write <facet.chroma> <dir>` writes a
lexicon as that:

```text
helix/
  README.md                the code, in words, with the facet's own relation names
  codons.tsv               k  codon  hexagram  degree  order  x y z  relation
  strands/<aa>/<word>.dna  64 lines: codon  source-byte  target-byte  evidence
  codons/<CODON>/words.tsv the words with most evidence on that channel
```

`codon(k)` is `k` in base four over `ACGT`; the hexagram is `k`'s six bits;
degree and order are the sphere's; `(x, y, z)` is the codon's place on a
B-DNA helix (10.5 codons per turn, 0.34 nm rise). The two bytes per codon are
the two strands — a word's source side and target side on that channel,
which phiano has kept apart since docs/72 — and the evidence is the count.
`bin/helix read` inverts it: the round trip is byte-exact on both sides,
evidence and relation table (tested); n-grams are not in the helix.

The persona lexicon (18,496 words, 24 marker relations) writes in 0.6 s to
75 MB, and the queries are the shell's:

```
cat codons/AGG/words.tsv          # `of`: who carries it (channel 10)
awk -F'\t' '$4>0' strands/ca/cat.dna    # a word's active codons
grep -l $'^AGG\t.*\t[1-9]' strands/*/*.dna | wc -l   # every word with `of` evidence
```

On the synthetic quest's helix, `w0`'s active codons are exactly its five
relations plus offset+1 — `AGA` (is-a), `AGC` (near), `AGG` (unlike), `AGT`
(part-of), `ATA` (used-for) — which is the quest, read as a gene.

What this is not: a claim that language *is* DNA. It is a file format whose
64-fold code lines up with three others by count, chosen so that a word's
intelligence is a text file a person can read and a shell can query, and so
that "every code is a position in space" is literally true of the tree.

## 3. What remains

A tie that keeps the channel order (per-degree weights on the circle's
modes, or the circle with a sphere-shaped prior);
placing a word's phasors on the sphere by *relation kind* rather than channel
index, which is where the degree would start to mean scale of relation; the
helix as the quest's own output format (a quest writes strands directly); and
the first Phi-4 run.
