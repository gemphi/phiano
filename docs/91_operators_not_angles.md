# 91 · Operators, not angles: what the weather work says the 8M model should build

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. The sixteen
Anandkumar write-ups (`docs/papers/anandkumar/`) read FNO, AFNO, FourCastNet,
the spherical FNO and FourCastNet 3 for Phient. Read against docs/90, they say
one thing plainly: the model today stores **answers** (an angle per word per
kind); the weather models store **operators** (one learned map, applied to any
state). Nothing below is measured unless a number is quoted from those docs.

## 1. The shift

| | today | from the weather work |
|---|---|---|
| a word | 64 one-byte angles + counts | a function on the circle (or sphere): 64 complex coefficients |
| a relation kind | a channel; each word's angle on it | an **operator** K_kind mapping a word's function to its answer's (spectral weights per mode, `05`, `13`) |
| learning | Kuramoto nudges on one pair, one channel | gradient on the operator through phinet's `Tape`, over all rows of the kind at once |
| an untold relation | unreadable (held out 0.005, chance 0.017; docs/89) | K_kind applied to a word it was never asked about — the operator is shared, so it generalises or visibly does not |
| several answers | blur into one angle (docs/89 §5) | predict a **distribution** — an ensemble from a noise input scored with CRPS (`16` §5/§12: "the single prediction that minimizes squared error is their average, which is smoother than any of them") |
| two hops | a reading trick | **composition** K_part-of ∘ K_is-a, trained by rollout (`09` §12, `16` §15) with a closure loss (`07` §12) |
| a condition c | a channel region | the operator's weights made to depend on c (AFNO as "a conditional operator", `08` §13), or a smooth field (`15` §12) |
| budget | angles | word coefficients + a few thousand weights per kind operator |

## 2. Suggestions, in the order they bear on the walls

1. **Kind operators** — one spectral operator per kind over word functions
   (phinet `Fno`, circle tie; anisotropic tie as the small arm). Headline test:
   held-out one-hop MRR, today 0.005 against chance 0.017. This is wall 2.
2. **Distribution, not point** — noise input, an ensemble of 16, CRPS on the
   circle (`16`). This is wall 1 (several answers).
3. **Rollout and closure** — train X⇒Y⇒Z against Y and Z; penalise
   K₂∘K₁ − K₁₂ on is-a/used-for (`07`, `09`, `16`). Synthetic held-out two
   hops are already 0.095/0.093 against 0.011 one hop (PHIENT_FACTS).
4. **Local + global blocks** — FourCastNet 3's best ratio was "four local
   blocks to one global block" (`16` §9); the sphere tie is the global block
   alone, which may be why it saturates at 16.5× against the circle's 27.6×.
5. **Split the Fourier path from W** — `05` §7: "W is the only part of the
   layer that can tell one channel from another"; kinds may live in W. Sweep
   `modes`, which has never been swept.
6. **Controls every run**: a low-rank 64×r kernel (`06` §9) and a dense
   64×64 kernel — if either matches, the FFT is not what is working.
7. **Conditional weights (AFNO)** for scope, speaker and topic — the
   operator's mixing weights computed from c (`08` §12); block-diagonal
   complex weights and a soft threshold at the byte's quantum.
8. **A family level** between words (`04`): 32 family nodes; is-a is dense
   and low-rank, part-of sparse and high-rank.
9. **Measure before fixing the channel count** (`01` §13): the rank of the
   channels' second and third moments — "the number of channels is the rank
   of that tensor, a measured quantity. Sixty-four would then be a
   hypothesis." This decides how far past 64 the extension should go.

## 3. What changes in docs/90's order of work

The extension registry (`src/extension/`, 3 tests) stays: kinds, code
relations and conditions still need channel ids past 64. Answer *slots* on it
are a storage fix for wall 1 and are kept as the control arm, not the plan.
Step 1 becomes **kind operators with held-out one-hop MRR as the headline**;
distribution (2) and rollout/closure (3) follow on the same Phi-4 rows.
