# 163 · Step 5: the experiments on a bench, phinum behind a door — and the stage ring

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. The experiments moved to `examples/`

docs/159 step 5. Of phiano's 69 binaries, 53 were experiments — a measurement run once for a paper — and 16 are tools a person uses (`phiano`, `train_rust_book`, `bootstrap_facet`, `ingest`, `prepare_prose`, `persona_ingest`, `persona_chat`, `persona_from_quest`, `quest_ingest`, `quest_ask`, `relate_ask`, `word_json`, `bpe_check`, `bpe_lexicon`, `helix`, `evaluate`). The 53 now live under `crates/phiano/examples/`, where cargo discovers them by name (`examples/ladder/main.rs` is `--example ladder`; `examples/liskov.rs` is `--example liskov`), `cargo test` and `cargo check --all-targets` still compile every one, and the default build no longer pays for them. The 53 `[[bin]]` stanzas are gone; the cross-directory `#[path]` includes (`story/operator.rs`, `waves/strata.rs`, the `code_*` family) moved together and still resolve. Every `--bin <experiment>` in the scripts became `--example`; the papers keep the commands as they were run.

## 2. One bench: `phiano::lab`

The "one shared `setup()`": `lab::flag` and `lab::switch` (the argument helpers twelve binaries had each written), `lab::sentences_of` (the Austen sentence filter, 4..=60 tokens, written five times byte-for-byte), `lab::austen()` (the three novels), `lab::dir_sentences`, `lab::chrono_free_date`. `bin/austen/util.rs`, which four binaries included by path, is gone; they `use phiano::lab as util`. No experiment's numbers moved: the helpers are the same code in one place.

## 3. phinum behind a feature

phinum (1,473 lines: the Gen-0 model — hexagrams over a syntax parser, a Searle classifier, a spider net) was live in exactly three places: `KING_WEN_MAP` (two experiments — now from `phistd::hexagram`, the one table), `SentenceType::classify` (the `in_c` experiment's syntactic prior), and the juncture test (phinum's hexagram lift agrees with `phical::q64::lift`). It is now `#[cfg(feature = "phinum")]`: the default phiano has no phinum, `in_c` declares `required-features = ["phinum"]`, and `cargo test -p phiano --features phinum` runs the juncture test (it passes). Deleting the body outright would have deleted a reproducible experiment; a door is the honest compromise, and the door is marked.

## 4. The stage ring

Mid-step the user asked for a second ring: `crates/phinet/personas/stage/` — 128 dancers, actors and the celebrity stage: forty dancers (Astaire, Rogers, Kelly, Nureyev, Baryshnikov, Fonteyn, Pavlova, Nijinsky, Duncan, Graham, Cunningham, Ailey, Fosse, Michael Jackson, Josephine Baker, the Nicholas Brothers, Bausch, Glover, Copeland, Beyoncé, Madonna…), eighty actors (Marilyn Monroe, Chaplin, Keaton, Bogart, Brando, Streep, De Niro, Pacino, Day-Lewis, Mifune, Bachchan, Shah Rukh Khan, Deneuve, Michelle Yeoh, Tony Leung…), and eight from the celebrity stage with the Kardashian-Jenners as the team `kardashian`. Their context is `body/face, body, track`: their work is the body, so they watch the kit's face and joints first. `tests/personas.rs` now checks both rings (128 each, well-formed, no slug in both, the stage watches the body). Taught with the stand-in oracle in the container; with phi-4 on the user's machine by pointing `persona_quest` at the folder.

## 5. Status

Done 2026-10-13: 384 tests by default (phinum's nine behind the feature), 0 warnings, every `.rs` under 200 lines. docs/159's five steps are complete. Next: the phasor convergence (`SpectralPhasor` ↔ `phiecs::Phasor`), a `Body` agent on live tracking rows, and both rings quested by phi-4.
