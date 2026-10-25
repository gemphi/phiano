# Changelog

All notable changes to `phiano` are documented here.

## [Unreleased]

### Added
- `bin/phinet_residual` and `docs/68_residual_and_curriculum.md`: the operator
  trained on the mixture likelihood itself (`phinet::Fno::train_epoch_mixture`,
  `Tape::cosine_table`, `Tape::mixture_nll`, `PhianoLM::mixture_terms`) reads
  134 alone — the joint-training number without touching the lexicon — and
  lowering γ_train moves the mixture the right way by a sixth of a point; a
  depth curriculum buys level 4 from chance to 7× at a few percent on trained
  levels.
- `bin/phinet_more` and `docs/67_phinet_three_more.md`: frequency-proportional
  decoys do not help the mixture (hypothesis refuted); a cleanup trained on
  levels 1–3 recovers level 3 at 45× chance and level 4 at chance; one operator
  carries both the LM and the cleanup within 2–8% of the specialists.
  `phinet::Fno::train_epoch_with`, `phinet::FrequencySampler`.

### Changed
- **Phinet is its own crate.** The engine — `tensor`, `operator`, `joint`,
  `hrr` — moved to `../phinet` (no notion of words, generic over the circle's
  sample count, `phinet = { path = "../phinet" }`). `phiano::phinet` keeps the
  application layer: `space`, `reader`, `lm`, `joint` (examples + write-back),
  `compose`. `Example { input, target }` replaces `{ context, next }`;
  `FnoConfig` gains `space`.

### Added
- **Phinet in the loop** (`docs/66_phinet_in_the_loop.md`): the operator as a
  `PhianoLM` phase term (`phinet::lm`, `PhianoLM::gamma_sweep_scored`) — alone
  it reads 149 perplexity against the manifold's 177; operator and lexicon
  trained jointly (`phinet::joint`, word functions as tape parameters with a
  proximal tether, written back to the facet) — 134, two points from
  Kneser-Ney; recursive composition (`phinet::compose`) with the operator as
  learned cleanup — level-2 recovery 53× chance where phase subtraction gives
  8×. `bin/phinet_next` runs all three. `tensor.rs` gains `gather` and `unit`.
- **Phinet, the Fourier tensor engine** (`src/phinet/`): complex tensors with a
  reverse-mode tape and a differentiable unitary FFT (`tensor.rs`, gradients
  finite-difference checked); the lexicon as a function space on the relation
  circle (`space.rs`); a Fourier neural operator with spectral mode mixing,
  pointwise path and GELU, trained by cosine hinge (`operator.rs`); and a reader
  for the existing model files (`reader.rs`). `bin/phinet` measures identity vs
  pointwise control vs operator on next-word ranking; see `docs/65_phinet.md`.

### Fixed
- `data/manifold.chroma` (v1, 216k words) and `data/memory.chroma` did not load:
  both were written before fields were added, and bincode reads positionally so
  `#[serde(default)]` never applied. v1 facets are now parsed by hand across the
  historical record widths; `Memo::load_from_file` bounds allocation and falls
  back to the pre-`count` layout.

### Changed
- **`RelationalPhasor` promoted into the lexicon.** `SpectralPhasor` now carries a
  16-bit evidence count per channel — a word is 64 complex numbers, not 64 angles
  and one shared amplitude. New: `evidence`, `deposit`, `similarity_with`, `on`,
  `shared_relation_with`, `dominant_relation`. `resonance` is unchanged, and a
  phasor with no evidence compares exactly as before.
- `Facet` carries the relation inventory (`relations`, `learn_relations`) and the
  per-channel balance (`channel_balance`, `channel_dispersion`, `similarity`, `on`).
- `Trainer::couple_relations` runs inside `train`, `train_sentence` and
  `train_multi_epoch`: each pair within `RELATION_WINDOW` is coupled on the channel
  of the relation it was observed under (offset, or the marker word between them)
  and deposits evidence there, with a sampled negative pushed away on the same
  channel. `Trainer::without_relations()` is the control. `train_corpus` and the
  harness learn the marker inventory first.
- `.chroma` format v4: evidence and relation inventory on disk. v1–v3 files still
  load; v3 loads with empty evidence and fills on the next training pass.
- Channel ownership: the ranking and centroid steps move only channels no relation
  claims (`RelationVocab::unclaimed`, `Trainer::update_channels`);
  `MARKER_CHANNELS_MAX = 24` splits the torus 32 relation / 32 prediction channels.
- New `bin/promoted` measures the promotion on WikiText-2 prose; see
  `docs/64_promoted_lexicon.md`.

## [0.2.0] - 2026-08-25

### Added
- Formalized Chapter 14 specifications in `specs/` (Phasor Geometries, Kuramoto Order Parameter, Dual Cognition, and SSE Telemetry).
- Real-time Kuramoto phase synchronization metric $R(t)$ calculation.
- Continuous SSE event streaming endpoints for phase diagrams and token resonance.

### Changed
- Bumped package version to `0.2.0`.
- Standardized git author to `bdizha <bdizha@gmail.com>`.

## [0.1.0] - 2026-08-25
- Initial release of Phiano phase-instrument and phasor resonance engine.
