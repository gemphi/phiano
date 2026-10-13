# 85 · PML — one structure for the phi* crates

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. A codebase is the same
(docs/82): a crate is what it relates to, and a crate nothing depends on is
not in the system. This is the inventory of everything `phi*` in gemphi, taken
from the tree on 2026-09-27, and the one structure it should settle into —
**phinet the parent; phical, phinum, phiqlm, phifno, phiton its children; PML
the name for the family** — with what is done and what is only proposed kept
apart.

## 1. What is in the tree

| where | what | lines / state | depends on |
|---|---|---|---|
| `phinet/` (crate 0.1.0) | the engine: `Tensor`, `Tape`, unitary FFT, `Fno`, joint training, HRR, hexagram gates, phasor ranges, **`sphere` + `Tie::Sphere` (new)** | 37 tests, 62/62 torch parity | num-complex, serde, rayon |
| `phifno/` (crate **1.0.0**, new) | the spherical operator for words: `config()`, `Word` on the sphere, `examples/filter.rs` | 2 tests + doctest | phinet |
| `phiano/` (crate 0.2.0) | the word-level application: `Facet`, `Trainer`, generation, personas, quest ingest, **`helix` (new)**, 52 bins | 226 lib tests, 541 files | phinet, phi-cal |
| `phiano/src/phical/` (module) | the manifold + `evaluate` (word/phrase/sentence/story) | `evaluate` is **entirely unreferenced** (docs/82) | — |
| `phiano/src/phinum/` (module) | a second Searle stack, I Ching syntax variants, lexicon config | 9 dead items, **wired to nothing** (docs/82) | — |
| `phiano/src/phiton/` (module) | light as units: `LightQuantum`, `Chiton`, `SpectralBand` (16), `PhitonColor`, phase ↔ wavelength ↔ colour with the fine-structure shift | 7 dead conversions before today; **now read by the helix** | phical::quantum |
| `phiano/src/phinet/` (module) | the app layer over the engine: `FunctionSpace`, reader, joint, LM | used by every operator bin | phinet |
| `phiace/crates/phical/` (crate `phi-cal`) | exact geometry on the closed relation circle: the Riemann sphere per channel, Möbius maps | used by phiano | — |
| `phiace/crates/phiecs/` (crate) | the entity-component world for language and the quest | 14 tests | — (phiano via TSV) |
| `phiace/crates/{phiber,phiray,phiphx,apple}` | the Blender-parity UI, its renderer, the .blend reader, Apple mirrors | the app | gpui, wgpu |
| `phi/crates/*` | Apple-shaped mirrors (realitykit, reality-foundation, coreml, simkit, phi-oml …) | source for `phiace/crates/apple` | — |
| `phixum/crates/*` | trading (pricing, risk, strategy …) | a different product | — |
| `phient/`, `docphi/`, `puijs/`, `phidoc/` | the agent runtime (Python), a doc site, the React library, the docs hub | docs, not crates | — |

Two things the table says at once. The engine has one parent and one real
child; **three of the children the user names are modules buried inside the
application** (phical, phinum, phiton), where two of them are dead and the
third was until the helix read it today. And there are *two* things called
phical — a crate about the Riemann sphere per channel and a module about
evaluation — which is the kind of ambiguity docs/82 warned turns into rot.

## 2. The structure

```text
phinet            the parent: tensor, tape, FFT, Fno, sphere, hexagram, joint, hrr
├── phifno 1.0.0  the spherical operator (done)
├── phical        the paradigm's geometry: relation circle, Riemann sphere per channel,
│                 Möbius, quantum(band_n)          ← phiace/crates/phical, moved up
├── phiton        light as the unit system: LightQuantum, Chiton, SpectralBand,
│                 PhitonColor, phase↔λ↔colour      ← phiano/src/phiton, lifted out
├── phinum        the number/syntax layer: I Ching variants, Searle classes, configs
│                 ← phiano/src/phinum, lifted out — and wired or deleted
└── phiqlm        quantum (planned; nothing exists yet — a name, not a crate)

phiano            the word application: Facet, Trainer, generation, personas, helix, bins
phiecs            the entity-component world and the quest
phiace            the UI (phiber, phiray, phiphx, apple mirrors)
```

The rule that decides where a thing goes: **a child of phinet is anything that
reads the 64 channels its own way and needs no words** — geometry (phical),
units (phiton), number/syntax (phinum), the sphere (phifno), quantum (phiqlm).
Anything that needs a `Facet` stays in phiano. Anything that needs a window
stays in phiace.

## 3. "Do not recreate f64"

The user's instruction, taken literally: where the engine reports a bare
number that the family already has a unit for, use the unit. Today:

- the helix's strand line now carries each codon's `PhitonColor` — wavelength,
  band, hex — beside its bytes, so `love`'s `AAA` reads `418.5 nm indigo
  #5900F8`, not `26`. This is the first time `phase_to_color` has been read
  by anything that ships.
- phifno's `Word::energy()` is a rotation-invariant spectrum by degree; the
  natural next step is `SpectralBand` per degree (16 bands, 8 degrees — two
  per band or one per octave) so a word's spectrum by scale is named in
  PhiTon's bands. Not done: it would be a mapping without a measurement.
- `Phical::quantum(band_n)` already clamps the lexicon's convergence counter
  into the fine-structure shift; the helix header's `band` is that counter,
  and should print the quantum beside it.

What not to do: replace `f64` inside the tape. Autograd is arithmetic on
complex numbers; a unit type there would be a cost with no reading.

## 4. Moves, in order, each one a compile-and-test

1. **phiton → crate `phinet/…/phiton` or `gemphi/phiton`**, depending only on
   phinet's `c64` and phical's `quantum`. phiano re-exports it. The helix and
   the sector palette are its two consumers; the dead conversions come with
   it and are counted again in the next docs/82.
2. **phical: one name.** Move `phiace/crates/phical` (phi-cal) to
   `gemphi/phical` as the child; fold `phiano/src/phical/manifold.rs` into
   phiano's `facet` (it is a facet operation) and delete `phical::evaluate`
   (unreferenced since docs/79).
3. **phinum: wire or delete.** Its Searle classifier duplicates
   `src/cognitive`; its I Ching syntax variants duplicate `phinet::hexagram`.
   Keep the variants table if `bin/hexagram_eval` reads it; delete the rest.
4. **phiqlm** stays a name until there is one measurement it would produce.
5. `phiace/Cargo.toml` and `phiano/Cargo.toml` point at the new paths; the
   Apple mirrors in `phi/crates` are phiace's concern, not PML's.

None of 1–5 was done in the commit that wrote this note; the helix, phifno and
the sphere were.

**Move 1 done (2026-09-28).** `gemphi/phiton` is its own crate (0.1.0, one
dependency: serde; 4 tests). Its constants (`ALPHA`, `PHI`, `TWO_PI`,
`BAND_N_EFFECTIVE_MAX`) and `quantum` live in `phiton::units`; phiano depends on
it, re-exports it as `phiano::phiton` so all ten `crate::phiton::…` call sites
are untouched, and `Phical::quantum` delegates to it. A phiano test
(`phiton_units_match_phiano_config`) keeps the shared constants equal. The
helix written from the persona lexicon is byte-identical before and after.
`phiano/src/phiton/` is no longer compiled and can be deleted.
The order is by how much each move removes (phiton: one buried module,
seven dead items found a reader; phical: a name collision; phinum: nine dead
items) against how much it risks (phiton: nothing; phical: every phiano
import; phinum: one bin).

**Relation operators placed (2026-09-28, docs/93).** The first work that runs
through phiecs, phinet and phiano at once is split by the rule above:
`phinet::relate` (operators over rows, laws, closure, scores — no words),
`phiano::relate` (words and kinds to rows, facet start, `.relate` files),
and phiecs's `parse_tsv` as the one reader of a quest table (phiano now
depends on phiecs; phiecs still depends on nothing). phi-cal is not used by
it and stays the geometry child of move 2.

