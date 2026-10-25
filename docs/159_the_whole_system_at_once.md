# 159 · The whole system at once — levels, entities, and what to cut

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

This is the architecture audit the user asked for: see all eight crates at once, say how the entities interact, say which field works at which level, and name every piece of code that is not being used or is written twice. It was built from a full survey of the workspace (39k lines; phiano alone 26k in 48 modules and 74 bins).

## 1. The finding: two generations sharing one house

phiano contains **two systems that never merged**:

- **Generation 1 ("chroma")** — `Facet` / `SpectralPhasor` / Kuramoto `Trainer` / `metrics::harness` / the axum `server` + 16-agent `cognitive` (Searle) / `persona` fingerprints / `compose` / `physics` / `oscillator` / `reasoning`, and ~40 experiment bins. It is the REPL, the HTTP API, the chat path. Its types are phiano's own.
- **Generation 2 (docs/100–157)** — `Row` → `phiplm` (`Counts`, `ScopeTree`, kin, levels, `Knowing`, strand, code) + `phiecs` (`World`, `Phasor`, `Relation`, `Under`, `Quest`) + `phical` (`Fno`, `Mirror`). It is the forest, the ladder, the games, the operator runs. Its types are the lower crates'.

They share every *concept* and almost no *type*. That is the whole problem, and it is why the same idea is written 2–4 times:

| idea | written as |
|---|---|
| a word's phasor | `phiecs::Phasor` · `phiano::phasor::SpectralPhasor` · `phiano::relational::RelationalPhasor` — **three, and their `seeded()` hash differently, so the same word starts at different angles in each** |
| quantise a phase to 64 sectors | `phical::q64` · `phinet::phasor::Quantum::Q64` · `phinet::hexagram::hexagram_of` · `phinum::Hexagram::from_phase` — four |
| hash / mix / wrap | `phinet::{mix64, fnv1a}` + phiano copies `widen::mix64`, `splitmix`, `memory::fnv1a_hash`; `phical::wrap` + `trainer::wrap_signed` + `envision::wrap` |
| the constants | `phiton::units::{ALPHA, PHI, TWO_PI}` redefined in `phiano::config` (a test asserts they're equal); `PHASE_CHANNELS` = `phical::CHANNELS` = `phiecs::MAX_KINDS` = 64, three names |
| counts | `phiplm::relate::Counts` · `bin/language_game/learners.rs::Counts` · `bin/code_reader/models.rs::Counts` |
| tokenizer | `phiano::tokenizer` · `phiplm::prose::tokens` · `phikit::words::split` · `bin/code_reader/lex.rs` |
| held-out measurement | `phiplm::prose::heldout` · `phiano::metrics::harness` · `data::DataSplits` · ~12 bin `setup()`s |
| Searle speech-act classifier | `cognitive::speech` (data-driven) · `phinum::searle` · `phinum::Hexagram::speech_act` |
| "X ⇒ Y under c" | `phiecs::condition::Under` (context-conditioned) · `facet::under` (kind-conditioned — a different thing with the same name) |
| persona | `phiecs::persona` (the spec) · `phiano::persona::PersonaStore` (the fingerprint) — two stores |
| HRR binding | `phinet::hrr` · `SpectralPhasor::bind/unbind` |

## 2. The levels (how the crates should stack)

```
L5  phiano    the APPLICATION   server · CLI/REPL · persona dialogue · generate · live bins
L4  phikit    DOMAINS           body / tracking / device vocabularies told as Rows; the Watch system
L3  phiplm    LANGUAGE          Row → Counts + ScopeTree → levels · kin · field · strand · code reader → a reading
L2  phiecs    ENTITIES          Word · World · Phasor (64 ch) · Relation = channel · Under (X⇒Y under c) · Quest · act
L1  phical    OPERATORS         Fno (Fourier) · Joint · Mirror (conjugate, backwards) · Tie
L0  phinet    NUMBERS           c64 · Tensor/Tape/FFT/Adam · HRR · the 64: codon = hexagram = channel · ONE q64
    phifno    (beside L1)       the 64 as spherical harmonics l<8
    phiton    (beside L0)       light: phase ↔ wavelength/band/colour; Chiton standing wave; ALPHA/PHI/TWO_PI
```

Dependency is already clean downward (phinet ← phifno ← phical ← phiplm ← phikit ← phiano; phiecs ← phiplm). The work is to make each level *own* its idea once.

## 3. How the entities interact (the one flow)

**In.** An oracle answer, a paragraph of prose, a story strand, a Rust file, or a phikit vocabulary all become the same thing: `Row {about, kind, other, weight}` (`phiecs::quest::Row`, used 53 times across the workspace). It is the spine.

**Entity.** A spelling is coined into a `Word` (`phiecs::Lexicon`). `World::relate(from, kind, to)` does three things at once: adds an `Edge`, counts the kind in `Relations`, and `Phasor::couple`s both words toward each other on channel *kind* (rate 0.08). **A relation kind is a channel. Meaning is the phasors' agreement per channel.** That coupling *is* the oscillator model (Kuramoto): Bentov's field, done as code. Conditional claims go into `Under` keyed by the condition word — X ⇒ Y *under c* — which is Searle's direction-of-fit / aboutness made countable. The `Quest` is the act loop: frontier word → prompt → claims → edges/phasors → newly opened words → frontier.

**Language.** `phiplm::Counts::learn(rows)` interns every name as a `Word` and counts `(condition, kind) → value`; `within` rows build the `ScopeTree`. To read, `Knowing::next(said)` casts the context down the scope tree, then `read_levels` moves a base distribution through scope → last word → last pair (D4, nested). The base is the prior, the real-world prior (`with_real_prior`, docs/157), the kin mixture (family resemblance, §67), or an operator's scores.

**Operator.** `phical::Fno` reads a word's 64 channels as a function on the closed circle in the spectral domain (Fourier); `Mirror` replays the relation backwards by the conjugate filter (the phase-conjugate mirror / hologram). `phinet::hexagram` is a gate on the octaves of that spectrum (I Ching, line j yang ⇒ octave j passes). `phinet::codon` names channel *k* in base 4 over ACGT (DNA; the helix writes a Facet as 64-codon strands).

**Out.** The reading ranks the next word (the ladder, docs/156–157), or the world answers `calls_for` / `calls_for_under`, or the persona speaks. Only `bin/forest` and `phikit::Kit::world` push data *back* into a live `World`; everything else works on `Row`s.

So every field the user named has a level: **Bentov / Kuramoto at L2** (`Phasor::couple`), **Searle at L2** (`Quest` = act, `Under` = direction of fit; and `cognitive::speech` at L5 tags the act), **Wittgenstein at L3** (counts + kin), **Fourier at L1** (`Fno`), **the mirror at L1** (`Mirror`), **I Ching at L0** (`hexagram` gate on the spectrum), **DNA at L0** (`codon` names the 64). Nothing new is needed to wire them — they are wired; they are just also *re-wired* two or three times each.

## 4. DNA and I Ching, exactly

**DNA.** `phinet::codon`: codon *k* = 16a + 4b + c over ACGT, AAA = 0 … TTT = 63. Two readings of the 64: in `phiano::helix`, codon *k* is relation channel *k* of a word; in `phiplm::strand`, codon *k* is text level *k* (word → story). `helix::write` turns a Facet into `codons.tsv` + `strands/<word>.dna` + `codons/<CODON>/words.tsv`, with B-DNA geometry (2πk/10.5, 0.34 nm) and PhiTon colour; `read` round-trips. Users: `bin/helix`, the `bin/strand` family, `bin/forest` (tree #12). No training or generation path touches it — it is an IO format. **Keep `phinet::codon`; keep `helix` in phiano as IO.** Bug: `codon::hexagram_glyph(k)` uses U+4DC0 + k, but that Unicode block is in King Wen order, not binary — `phinum` has the correct map.

**I Ching.** Three mechanisms, one real. (1) **`phinet::hexagram` is the real one:** a 6-bit id is a spectral gate — FFT, octave *j* passes iff line *j* is yang, iFFT; `changing_lines` is XOR; `hexagram_of(f)` reads the centroid phase into 64 sectors. Live use: `generate/guide.rs` gates the FNO prediction every N steps (only `bin/austen` sets it); read-only in persona fingerprints and three bins. (2) `phinum::iching::Hexagram` is symbolic — King Wen number, trigrams, `from_phase` floors to 64 sectors, `speech_act()` maps the upper trigram to a Searle act. Dead except `KING_WEN_MAP` borrowed by two bins. (3) `kinds/permute`, `kinds/modes`, `flow/propagate` are **not** I Ching (a shuffled null, standing-wave rotation labels, Kuramoto). **Keep the gate in `phinet::hexagram`, move King Wen/trigram tables there, fix the glyph, delete the rest of `phinum`.**

## 5. What is dead (safe to delete) — by crate

- **phiano:** `extension` (0 refs); `phinum` (library-dead; keep its King Wen table); `data/` (only a dead module uses it); `metrics::{split, capacity, regularization, report, detection::shift}`; 48 of 132 `config` constants + 6 config fns; `train_rust_book_lite` (a copy); orphan `bin/waves/strata.rs` (not in Cargo.toml, never compiles); bins `markers`, `trace`, `experiment`, `defstats`; dead items `facet::{bigram_discounted, trigram_probability, word_color}`, `trainer::train_corpus_ranking`, `phasor::{rotate_all, harmonic_spectrum, to_light_quantum}`, `wave::{sentence_compound, words_in_sector}`, `generate::{decode_beam, reset_state}` + 12 dead `pub use` constants, `eval::eval_with_memory`.
- **phical:** the "exact geometry" cluster `mobius`, `sphere`, `winding`, `heat::circle`; `liskov::{better, loss, forcing_choice}`; `Module`/`Sequential` (tests only); `zeta_weights`, `scale_modes`, `wrap`, `TAU`.
- **phinet:** `phasor::{table, quantisation_loss}`, `hexagram::{gate (internal), phase_of}`.
- **phifno:** everything but the index arithmetic (`Word`, `Sphere::synthesis/analysis`, `Grid`, `gauss_legendre`, `degree_gate/energy`) — only `degree_of/order_of/DEGREES/HARMONICS` are consumed.
- **phiton:** `Chiton::{standing_wave_amplitude, wave_number, to_quantum, from_phase}`, four `PhitonSpectrum` fns, `wavelength_to_phase`, `center_nm`.
- **phiecs:** `world::{Rule, plan*, reach, keeps}`, `query::{Related, With2}`, `FnSystem`, `Resources`, `Dense`, and the **whole Quest driver has no runner** — `examples/persona_quest` is referenced and missing; the `phiano-facet` feature points at a module that doesn't exist.
- **phikit:** `Watch`, `Kit::world`, `Kit::with`, and all runtime `track::*` / `device::*` machinery (`Session`, `Registry`, …) — only their `told()` rows are consumed.

## 6. What moves where (the wiring plan)

**To phinet (L0):** the one `q64` (`phasor::Quantum::Q64` owns quantise/lift/sector distance; `phical::q64`, `phinum::from_phase` go); `hexagram` + King Wen/trigram tables (glyph fixed); `mix64`/`fnv1a`/`splitmix`/`c64` (delete phiano copies); `kinds::harmonic_rotation` (pure geometry that justifies 64).
**To phical (L1):** nothing new; `wrap` is the one angle-wrap. Delete the exact cluster.
**To phiecs (L2):** **the one `Phasor`** (retire `RelationalPhasor`; `SpectralPhasor` becomes its packed on-disk form); **`RelationVocab`** → `phiecs::relation` (the channel ↔ kind inventory helix/flow/facet/storage all need); `Row` to top level (`phiecs::row::Row`); `World::from_rows` (replaces `Kit::world` and `Quest::from_rows`); the one persona record (spec + fingerprint ref); `facet::under` → `under_kind` beside `Under`.
**To phiplm (L3):** `phical::relate` + `liskov` (language-level); the KN baseline and the one split/harness; the row-convention helpers (`phikit::tell::{row, scope_says}`) so `prose::paragraph` and `tell` share one definition of `within`/`says`/`pair`; bin-local `Counts`/`tokens`/`softmax` replaced by phiplm's; `relate::{model, wave, mirror}` → `phiplm::operator` so `relate` means counts only; `prose::{teach, unknown::uses_prompt}` → app.
**To phiton:** phiano imports `ALPHA/PHI/TWO_PI/BAND_N_EFFECTIVE_MAX` instead of redefining them; `PhitonColor` is presentation.
**phiano keeps (L5):** `server`, `command`/`model` (REPL), `persona` (fingerprint, dialogue, impersonate), `generate`, `cognitive::grounding` (the live part — recursive definition grounding), `helix` (IO), `storage`, `sources`, `chunker`, `conception`, `flow`, `memory`, `correction`, `curriculum`, `envision`, and the live bins. **Legacy-but-used, keep behind the server only:** `cognitive` agents, `compose`, `physics`, `oscillator`, `reasoning`, `synthesis`, `lifelong`, `layers`, `instruction`, `synthetic`, `wiki`, `gemgum`. **Bins-only experiments → `examples/` or delete:** `frames`, `kinds`, `topdown`, `bpe`, `roles`, `relational`, and the ~20 facet-measurement bins that each re-implement the same `setup()`.

## 7. Two bugs the audit found

1. **Three `seeded()` phasors hash differently** (`phiecs::Phasor::seeded` FNV+LCG; `phiano::relational::seeded` FNV+mix64; `phiano::phasor::spectral::seeded`) — the same word is born at different angles in the two generations. One phasor fixes it.
2. **`phinet::codon::hexagram_glyph` is in the wrong order** (binary vs King Wen). Visible in `codons.tsv`.

## 8. Order of work

1. One `Phasor`, one `RelationVocab`, one `q64`, one hash — the four unifications that make the two generations share types (phiecs + phinet).
2. Delete the dead list (§5) — no behaviour change, ~5k lines lighter.
3. Move the language-level code down (§6, phiplm) and the row conventions into one place.
4. Restore `examples/persona_quest` (the Quest runner) and fix the `phiano-facet` feature.
5. Collapse the experiment bins into `examples/` with one shared `setup()`.

Each step keeps the workspace green; the measurements (docs/152–157) are the regression tests.

### Status (2026-10-12)

**Step 1 — done.** One seed (`phinet::{seed_phase, seed_angles, GOLDEN_ANGLE}`), one hash (`phinet::{fnv1a, mix64}`), one quantiser (`phical::q64` over `phinet::phasor::Quantum::Q64`), one hexagram table (`phinet::hexagram::{KING_WEN_MAP, glyph}`); phiecs depends on phinet. Two bugs fixed: a word born at different angles in the two generations; `hexagram_glyph` in binary rather than King Wen order. 390 tests.

**Step 2 — done.** The §5 list deleted (phiano `extension/`, three `metrics` modules, `data/splits`, five bins and `bin/experiment/`; phical `mobius`, `sphere`, `winding`). Two the survey called dead were live (`metrics::report`, `bin/waves/strata.rs`) and were kept; the compiler, not the survey, decides. 375 tests, 69 bins, 0 warnings.

**Step 3 — done.** `Row` hoisted to `phiecs::row` with `Row::new` / `Row::weighted`, and `World::tell` / `from_rows` as the one Row→World loop (`Quest::from_rows`, `Kit::world` delegate; `bin/quest_ingest` had its own `Row` and parser — replaced). `RelationVocab` is a view over `phiecs::relation::Relations` — offsets declared first as `offset+1..=8`, markers after in rank order, `.chroma` entries unchanged (`relational/{vocab,learn,store}.rs`). The scope-nesting rows every source wrote by hand are `phiplm::relate::nest`; `phikit::tell::thing` and `prose::paragraph` share it (`scope_says` stays in the kit: a naming convention, not a prose one). `relate::{model, wave, mirror}` → `phiplm::operator` (`relate` re-exports them, so no caller moved; phiano re-exports `operator`). The Kneser-Ney counting → `phiplm::baseline` over tokenized sentences; phiano's `metrics::baseline::kn` is a 48-line shim that tokenizes with its own `Tokenizer`, so every baseline number is what it was. Deferred as debatable: `phical::relate` + `liskov` (the laws are read over a function table phical owns). 383 tests, 0 warnings, every `.rs` under 200 lines (`ladder` split into `main/rungs/lex`, `Knowing::go` into `prose/going.rs`).

While re-running the ladder after the split, `data/lex/near.tsv` was found missing on both sides, so rung 5 had been silently equal to rung 3. Regenerated and synced with `freq/pos/sense.tsv`: rung 5 is now a measurement, and it says WordNet smoothing *hurts* slightly on both strata (told 0.298→0.281, open 0.020→0.018), which is docs/157's verdict again — the dictionary helps as structure, not as mass.

**Step 4 — done.** The Quest runner exists again: `crates/phiecs/examples/persona_quest.rs` (`cargo run -p phiecs --example persona_quest -- <root> <persona.md>… [--model] [--oracle prog [--arg a]…] [--only seed] [--dry]`), one `Quest::for_persona` per seed, resumable (a seed folder with a `relations.tsv` is skipped), writing exactly what `phiplm::relate::read_quests` and `bin/persona_from_quest` read. Beside it, `examples/sailor.md` (a small persona) and `examples/oracle.py` (a stand-in oracle answering in the one-kind format from a fixed pool) run the chain without ollama. Proven end to end on the stand-in: persona → 2 seeds × 12 questions → 54 rows + voice sentences → `persona_from_quest` → relate model (MRR 0.450 vs prior 0.257 on 46 toy rows, held out 8) → `personas.chroma` / `personas.json`. `persona_from_quest` had its own copy of `read_quests` — replaced by phiplm's. The `phiano-facet` feature was already removed in step 1. 383 tests, 0 warnings.

**Step 5 — next.** Then the phasor convergence (`SpectralPhasor` ↔ `phiecs::Phasor`), which the one channel registry now makes possible.

Built from a four-part survey of the workspace on 2026-10-12. Companion: docs/155 (the sixteen trees), docs/157 (data), docs/158 (fields).
