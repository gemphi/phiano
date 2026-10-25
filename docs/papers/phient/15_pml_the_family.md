# PML: Phinet the Parent, Its Children, and What Nothing References
*Phient papers, 15 of 16 · 2026-08-10 · PML: phinet the parent, its children, and what nothing references — the family as a structure, and the inventory of what is missing*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

PML is the name for the family of `phi*` crates in the gemphi workspace. The letters stand for Phi machine learning. This paper describes the family as a structure and takes the inventory of what is missing from it. It rests on two documents. Docs/85 lists everything `phi*` in the tree and proposes one shape for it. Docs/82 counts the code that nothing references and the input files that do not exist. Docs/77, the file layout rule, sets the grain at which the counting is done.

The structure has one parent. Phinet is the engine: the complex `Tensor` of shape `[b, w, s]`, the `Tape` for autograd, the unitary FFT, `Fno`, joint training, HRR, the 64 hexagram gates and the sphere. Its children are phifno 1.0.0, the spherical operator; phical, the geometry of the relation circle; phiton, light as units; phinum, the number and syntax layer; and phiqlm, which is planned and has nothing built. Three applications sit beside the family. Phiano is the word application. Phiecs is the entity-component world and the quest. Phiace is the UI.

The tree does not yet have that shape. Docs/85 finds one parent and one real child crate. Phical, phinum and phiton are modules inside phiano. There are two things called phical. Phinum's nine dead items are wired to nothing. Phiton's seven conversions had no reader until the helix read them.

The inventory in docs/82 was taken mechanically over 541 source files, 29,670 lines and 50 binaries. It finds 112 public items with no reference anywhere and 162 more used only inside their own module. It finds four missing input files whose absence is silent. One of them, `data/stop_words.txt`, makes `is_stop_word` false for every word. Another, `data/searle_markers.json`, makes every input `Assertive`. Ten binaries cannot run. Among them is `latency`, the benchmark for the project's central claim.

None of the five moves docs/85 proposes is done. The helix, phifno and the sphere are done. This paper states the structure, the inventory, the one rule that decides where a thing goes, and the next move with its control. It names the actual types and paths from the sources. Every number is taken from a source as it stands there. Where the sources disagree, the paper says so and prefers the later document. Where a thing was proposed and not built, the paper says not built. Where a thing was not measured, it says not measured.

## 2. The claim

The claim is taken from the paradigm and turned on the code. A word is nothing but what it relates to, under what conditions. Docs/85 says a codebase is the same: a crate is what it relates to, and a crate nothing depends on is not in the system. Docs/82 says it at the finer grain of items. A part nothing relates to is not a part of the system, whatever the file count says.

That gives two claims, one for each document. The first is structural. The `phi*` crates should settle into one family with one parent, and the rule for membership can be stated exactly. A child of phinet is anything that reads the 64 channels its own way and needs no words. Geometry reads them as a circle and a Riemann sphere per channel. Units read them as light. Number and syntax read them as hexagram variants and speech-act classes. The sphere reads them as spherical harmonics. Quantum would read them some other way, once it exists. Anything that needs a `Facet` stays in phiano. Anything that needs a window stays in phiace.

The second claim is about absence. A system's real extent is the set of items that something references. File counts and line counts overstate it. The honest inventory is taken by two passes: the compiler with every `#[allow(dead_code)]` stripped, and a reference count for every public item against the whole crate, its examples and its tests. What survives both passes is the system. What does not is listed by name.

Neither claim is a model result. There is no MRR in them and no perplexity. They are claims about which code is read by which, and they are checked by compiling, testing and counting. This paper keeps the distinction docs/85 keeps between what is done and what is only proposed. The structure in section 7 is a proposal. The inventory in sections 9 and 10 is a measurement of the tree. The five moves in docs/85 are not built.

The claim is also narrow in what it leaves out. It does not say dead code is wrong. Docs/82 ends with "Delete nothing yet: a dead constant is cheap and a deleted intention is not." It does not say the family is complete. Phiqlm is a name with nothing behind it. It does not say the proposed structure has been tested against another shape. No other shape was built or compared. It says three things only. The tree has a shape it could take. The reason for the shape is the paradigm's own reason. And the inventory tells which parts are in the system now and which parts only sit in the files.

## 3. The mechanism, plainly

Three mechanisms produce what this paper reports: the layout rule, the dead-code pass, and the dependency table.

The layout rule is in docs/77. Every `.rs` file in `phiano`, `phinet` and `phi-cal` is at most 200 lines, doc comments and tests included. The check is one command:

```sh
find src -name '*.rs' | xargs wc -l | awk '$1>200'
```

It must print nothing. It printed nothing for all three crates when docs/77 was written, with phiano at 524 files, phinet at 31 and phi-cal at 6. A module `foo` that outgrows one file becomes `foo.rs` plus a directory `foo/`. The file `foo.rs` keeps the module doc, the `use` lines, the declarations, and one `mod x;` with `pub use x::*;` per piece. Private fields are `pub(super)` so sibling files can reach them. Each piece is one topic: an `impl` block for one job, a free function, or a trait impl. Tests live in `foo/tests.rs`. Binaries live under `src/bin/<name>/main.rs` as a short sequence of phases, `setup`, `train`, one file per experiment section, and `report`.

The rule matters for the inventory because it makes the unit of counting small. A dead item in a 200-line file is visible as a dead item. It no longer hides inside a 1,490-line file like the old `metrics/harness.rs`.

The dead-code pass is in docs/82. It strips every `#[allow(dead_code)]` and compiles. Then it counts, for every public item, the references to it across the library, the binaries, the examples and the tests. An item with zero references is dead. An item referenced only inside its own module is counted apart, as surface wider than the system. The pass also takes every path the code names and checks whether it exists. A missing path the code will create on first run is an output. A missing path the code only reads is an input.

The dependency table is in docs/85. For each `phi*` entry in the tree it gives the location, what it holds, its size or state, and what it depends on. Phifno depends on phinet. Phiano depends on phinet and phi-cal. Phinet depends on num-complex, serde and rayon. Phiecs depends on nothing in the family and reaches phiano through TSV files.

The structure is then read off the table by the one rule. The types that decide each case are named in the sources. `Facet` is phiano's word type. `PhitonColor`, `LightQuantum`, `Chiton` and `SpectralBand` are phiton's. `sphere` and `Tie::Sphere` are phinet's. `Word` and `config()` are phifno's. `Phical::quantum(band_n)` is the joint between phical and phiton.

## 4. What was measured

The measurements here are of two kinds: counts of the tree, and the model results that the members of the family produce.

| measure | value | source |
|---|---|---|
| phiano source files | 541 | docs/82, docs/85 |
| phiano lines | 29,670 | docs/82 |
| phiano binaries | 50 (docs/82); 52 (docs/85) | both |
| phiano lib tests | 226 | docs/85 |
| phinet tests | 37; torch parity 62/62 | docs/85 |
| phifno tests | 2 + doctest | docs/85 |
| phiecs tests | 14 | docs/85 |
| public items with no reference | 112 | docs/82 |
| public items used only in their own module | 162 | docs/82 |
| missing paths the code names | 11 (7 outputs, 4 inputs) | docs/82 |
| binaries that cannot run | 10 | docs/82 |
| modules with a file-wide `#![allow(dead_code)]` | 7 | docs/82 |
| dead `config` constants | 33 | docs/82 |

The counts disagree across documents. Docs/77 counted phiano at 524 files and 220 passing tests. Docs/82 and docs/85 are later, and this paper uses 541 files and 226 tests. Docs/82 counts 50 binaries and docs/85 counts 52 bins. Docs/85 is later and records the helix as new, so 52 is preferred.

The family's model results come from PHIENT_FACTS. Phinet matches a PyTorch reference in 62/62 parity tests. The operator on Austen, with a pool of 1,228 words and chance MRR 0.0063, reaches 14.7× chance on valid and 14.6× on test with the hinge at 8,360 parameters. Softmax wide and deep at 50,016 parameters reaches 30.7×/30.3× at 8 epochs. The sphere tie reaches 12.8×/13.1× with 1,192 parameters. The anisotropic tie reaches 16.0×/15.6× with 4,776 parameters, above the circle. `phinet::sphere` agrees with scipy's `sph_harm_y` to a maximum difference of 2.4e-15. The helix codes 18,496 words in 1.9 s at 75 MB with a byte-exact round trip. The phiecs quest oracle reaches 0.594 MRR with attraction only and dithered rounding, against 0.082 for the shipped trainer.

What was not measured belongs in the record too. No move in docs/85 was made, so nothing about a moved tree was measured. Phiqlm produces no number. The mapping from phifno's degrees to `SpectralBand` was not done. PHIENT_FACTS lists the sphere at softmax and at wide and deep as unrun, and in another line gives a softmax β=8 sphere result of 13.2×/13.5× from later the same day. The later line is preferred: the sphere at wide and deep is unrun. The `latency` benchmark cannot run on this tree, so learn, recall, correct and unlearn at p50 and p99 have no current numbers.

## 5. The one idea, restated

A thing is what relates to it. For a word that is 32 to 64 relations, each a complex phasor on a closed circle with a count. For a crate it is the edges in its `Cargo.toml` and the references in the source. For a public item it is the count of places that name it. When the count is zero, the item takes no part in anything the system does, however carefully it was written.

The same idea gives both the structure and the inventory. The structure asks what each crate reads and what reads it. Phinet reads nothing in the family, and phifno and phiano both read it. That makes it the parent. Phifno reads phinet and nothing else. That makes it a child. Phiano reads phinet and phi-cal and needs words. That makes it an application. The inventory asks the same question of every public item in phiano. `phical::evaluate` is read by nothing. `SectorIndex` is read only by its own test. `phase_to_color` was read by nothing until the helix strand line printed it.

The rule for the family follows directly. The 64 channels are the shared object. A child of phinet reads them its own way and needs no words. The rule is useful because it can be applied to a type without argument. Does it need a `Facet`? Then it belongs to phiano. Does it need a window? Then it belongs to phiace. Does it read the channels as geometry, units, number, sphere or quantum? Then it is a child of phinet.

The idea also explains how dead code accumulates. Docs/82 records that six `config` constants went dead in one week "by my hand", when `Impersonator` was rewritten onto the generator in docs/80. The constants did not change. What related to them was replaced. They lost their meaning without being touched. The document calls this the normal way the list grows, "not from carelessness, from replacing something." A relation-first view predicts exactly this. A thing defined by its relations loses its meaning when its partners move, whether or not it moves itself.

The last consequence is about names. Two things called phical is an ambiguity in the tree, and docs/85 says docs/82 warned that such ambiguity turns into rot. In the paradigm a word that points two ways under one condition reads as noise. In the tree a name that points at a crate and at a module makes every import a question for the reader. The proposed fix gives the name one referent. The crate keeps it. The module's manifold is folded into phiano's `facet`, and its `evaluate` is deleted. That fix is proposed in docs/85 and not built.

## 6. The tree as it stands

Docs/85 took the inventory of everything `phi*` in gemphi from the tree on 2026-09-27. Its table has thirteen rows. Eight name crates, four name modules inside phiano, and one names documentation.

`phinet/` is crate 0.1.0. It holds `Tensor`, `Tape`, the unitary FFT, `Fno`, joint training, HRR, hexagram gates, phasor ranges, and the new `sphere` with `Tie::Sphere`. It has 37 tests and 62/62 torch parity. It depends on num-complex, serde and rayon. Nothing in the family sits below it.

`phifno/` is crate 1.0.0 and new. It holds `config()`, `Word` on the sphere, and `examples/filter.rs`. It has 2 tests and a doctest. It depends on phinet only. It is the one child that is already a crate.

`phiano/` is crate 0.2.0. It holds `Facet`, `Trainer`, generation, personas, quest ingest and the new `helix`, with 52 bins, 226 lib tests and 541 files. It depends on phinet and phi-cal. Inside it are four modules that carry family names. `phiano/src/phical/` holds the manifold and `evaluate`, and `evaluate` is entirely unreferenced. `phiano/src/phinum/` holds a second Searle stack, I Ching syntax variants and lexicon config, with 9 dead items wired to nothing. `phiano/src/phiton/` holds `LightQuantum`, `Chiton`, `SpectralBand` with 16 bands, `PhitonColor`, and phase to wavelength to colour with the fine-structure shift. It had 7 dead conversions and is now read by the helix. It depends on `phical::quantum`. `phiano/src/phinet/` is the application layer over the engine: `FunctionSpace`, the reader, joint and the LM. Every operator bin uses it.

`phiace/crates/phical/` is the crate `phi-cal`. It holds exact geometry on the closed relation circle: the Riemann sphere per channel and Möbius maps. Phiano uses it. `phiace/crates/phiecs/` is the entity-component world for language and the quest, with 14 tests and no crate dependencies. Phiano reaches it through TSV.

The remaining rows lie outside PML. `phiace/crates/{phiber,phiray,phiphx,apple}` is the Blender-parity UI, its renderer, the .blend reader and the Apple mirrors, on gpui and wgpu. `phi/crates/*` holds Apple-shaped mirrors such as realitykit, reality-foundation, coreml, simkit and phi-oml, the source for `phiace/crates/apple`. `phixum/crates/*` is trading: pricing, risk, strategy. Docs/85 calls it a different product. `phient/`, `docphi/`, `puijs/` and `phidoc/` are the agent runtime in Python, a doc site, the React library and the docs hub. Docs/85 marks them as docs and not crates.

Docs/85 reads two facts from the table at once. The engine has one parent and one real child. Three of the children the user names are modules buried inside the application, and two of those are dead.

## 7. The structure, and the rule that decides it

Docs/85 proposes this shape:

```text
phinet            the parent: tensor, tape, FFT, Fno, sphere, hexagram, joint, hrr
├── phifno 1.0.0  the spherical operator (done)
├── phical        relation circle, Riemann sphere per channel, Möbius, quantum(band_n)
├── phiton        LightQuantum, Chiton, SpectralBand, PhitonColor, phase↔λ↔colour
├── phinum        I Ching variants, Searle classes, configs
└── phiqlm        quantum (planned; nothing exists yet)

phiano            the word application: Facet, Trainer, generation, personas, helix, bins
phiecs            the entity-component world and the quest
phiace            the UI (phiber, phiray, phiphx, apple mirrors)
```

The shape has two levels and a side. Phinet is the parent. Five children hang from it. Three applications sit beside the family and depend on it. The applications are kept out of the family because each needs something the channels alone do not give. Phiano needs words. Phiecs needs entities. Phiace needs a window.

Each child's place follows from the rule. Phifno reads the 64 channels as spherical harmonics: 64 = 8² channels as Y_l^m with l < 8, one complex weight per degree under `Tie::Sphere`. It depends on phinet only. Phical reads each channel as a Riemann sphere and maps it by Möbius transforms. It comes from `phiace/crates/phical`, moved up. Phiton reads a phase as a wavelength and a colour. It comes from `phiano/src/phiton`, lifted out. Phinum reads the channels as I Ching variants and Searle classes. It comes from `phiano/src/phinum`, lifted out, and wired or deleted. Phiqlm would read the channels as quantum states. It is planned, and docs/85 says nothing exists yet.

The rule also says where things stay. The helix codes the lexicon as a DNA tree of 18,496 words. It needs words, so it stays in phiano, even though it reads phiton's colours. The sector palette reads phiton too, and stays in phiano for the same reason. `phiano/src/phinet/`, with `FunctionSpace`, the reader and the LM, carries the parent's name but needs words, so it stays in phiano. The UI crates need a window and stay in phiace. The Apple mirrors in `phi/crates` are phiace's concern and not PML's.

One edge in the proposal runs between siblings. Phiton depends on `phical::quantum`. `Phical::quantum(band_n)` clamps the lexicon's convergence counter into the fine-structure shift. So in the proposed shape phiton depends on phinet's `c64` and on phical's `quantum`, and on nothing else. No other edge between children is recorded.

The shape is a proposal. Of the five children, only phifno exists as a child crate in the tree. The other four are a crate in the wrong workspace, two modules inside phiano, and a name.

## 8. The buried children, and the name that points two ways

Phical is the first case. The crate `phi-cal` in `phiace/crates/phical` holds exact geometry on the relation circle, and phiano uses it. The module `phiano/src/phical/` holds the manifold and `evaluate`. Docs/82 counts 16 dead items in the module: the whole of `phical::evaluate` (`evaluate_word/phrase/sentence/story/concept`, `layer_relation`, `story_complexity`) and half of `operators` and `manifold`. Docs/85 proposes one name. Move the crate to `gemphi/phical` as the child. Fold `phiano/src/phical/manifold.rs` into phiano's `facet`, because it is a facet operation. Delete `phical::evaluate`, which has been unreferenced since docs/79.

Phinum is the second case. It holds `SearleClassifier::classify_full`, `SpiderNet`, and the 16/32/64 syntax variations. Docs/82 counts 9 dead items and describes the module as a second Searle stack, parallel to `src/cognitive`, wired to nothing. Docs/85 finds two duplications. Its Searle classifier duplicates `src/cognitive`. Its I Ching syntax variants duplicate `phinet::hexagram`. The proposal is to wire it or delete it: keep the variants table if `bin/hexagram_eval` reads it, and delete the rest.

Phiton is the third case, and the one that found a reader. Its types are `LightQuantum`, `Chiton`, `SpectralBand` with 16 bands, and `PhitonColor`. Its conversions take a phase to a wavelength to a colour, with the fine-structure shift. Docs/82 counted 7 dead items in it, all wavelength and colour conversions. Docs/85 records that the helix now reads it. The helix strand line carries each codon's `PhitonColor` beside its bytes. The word `love` has codon `AAA`, and that codon now reads `418.5 nm indigo #5900F8` where it used to read `26`. Docs/85 says this is the first time `phase_to_color` has been read by anything that ships. The proposal is to lift phiton to its own crate, at `phinet/…/phiton` or `gemphi/phiton`, depending only on phinet's `c64` and phical's `quantum`, with phiano re-exporting it.

Docs/85 orders the moves by what each removes against what each risks:

| move | removes | risks |
|---|---|---|
| 1. phiton to a crate | one buried module; seven dead items found a reader | nothing |
| 2. phical, one name | a name collision | every phiano import |
| 3. phinum, wire or delete | nine dead items | one bin |
| 4. phiqlm stays a name | nothing | nothing |
| 5. Cargo paths | the old paths | none recorded |

Move 4 is stated as a rule. Phiqlm stays a name until there is one measurement it would produce. Move 5 points `phiace/Cargo.toml` and `phiano/Cargo.toml` at the new paths. Each move is meant to be one compile-and-test. None of the five is done in the commit docs/85 describes. The helix, phifno and the sphere are done in that commit.

## 9. What nothing references

Docs/82 finds 112 public items with no reference anywhere: not in the library, not in a binary, not in a test. A further 162 are public but used only inside their own module. Docs/82 does not count those as rot. It counts them as an API surface wider than the system. The two numbers answer different questions. The 112 say what the system never does. The 162 say what the system does only privately, while declaring it in public.

| module | dead items | what they are |
|---|---|---|
| `config` | 33 | constants nothing reads |
| `metrics` | 32 | `full_pipeline`, `suite_from_disk`, `shift_score`, `is_ood`, `phase_jitter`, `prune_low_amplitude`, `examples_to_competence`, `language_model` |
| `phical` | 16 | the whole of `phical::evaluate`, half of `operators`/`manifold` |
| `persona` | 13 | mostly `PersonalityMapper`; `PersonaStore::lexicon_of` |
| `phinum` | 9 | `SearleClassifier::classify_full`, `SpiderNet`, the 16/32/64 syntax variations |
| `lifelong` | 9 | `check_drift`, `check_regression`, `dispersion_trend` |
| `phiton` | 7 | wavelength/colour conversions |
| `reasoning` | 6 | `Counterfactual::what_if`, `sorting::by_amplitude/by_sector`, `Analogy::light_centric` |
| `compose`, `generate`, `phasor`, `data`, … | 24 | `decode_beam`, `Memo::consolidate`, `words_in_sector`, `harmonic_spectrum`, `to_light_quantum`, … |

Some rows describe work that was finished and never called. `decode_beam` is a full beam search. The drift monitor in `lifelong` is written and never called. `metrics` is a measurement API built wider than the harnesses that use it. `PersonaStore::lexicon_of` was written in the week of docs/82 and was not yet used.

Seven modules carry a file-wide `#![allow(dead_code)]`: `wiki_bulk`, `reasoning`, `metrics`, `curriculum`, `instruction`, `data` and `synthesis`. Docs/82 says this is what keeps the compiler quiet about most of the list.

The `config` row is the dangerous one, because a constant reads as a knob. `ROTOR_DIMS` and `ROTOR_DIM_WEIGHT` describe a rotor manifold that does not exist. `META_LR_MULT_MIN/MAX` and `META_MIX_MIN/MAX` describe a homeostatic self-tuner that does not exist. The four `VERIFY_*` constants describe an execution verifier that does not exist. `RECALL_BOOST`, `RECALL_PHASE_BIAS`, `RECALL_TOP_K`, `NEGATIVE_SAMPLES_K`, `NEGATIVE_REPULSION`, `ANTI_PHASE_PULSE`, `CORRECTION_JOURNAL_MAX`, `STUDY_BUDGET_DEFAULT`, `SYNTAX_LAG_DEFAULT`, `PASSAGE_*` and `PHI_SQUARED` are read by nothing. A reader who sets one of them changes nothing and gets no warning.

Six more constants went dead in the week of docs/82. Rewriting `Impersonator` onto the generator in docs/80 orphaned `IMPERSONATE_QUALITY_OVERALL`, `IMPERSONATE_QUALITY_DIVERSITY`, `IMPERSONATE_QUALITY_COVERAGE`, `IMPERSONATE_QUALITY_LENGTH`, `IMPERSONATE_ADJACENT_FIT_FACTOR` and `IMPERSONATE_CONVERGENCE_DELTA`.

The table and its total do not agree. Its rows sum to 149, while the text above it gives 112. Docs/82 does not reconcile the two, and this paper does not guess which is right.

## 10. What is missing: the silent inputs and the absent capabilities

Docs/82 finds eleven paths the code names that do not exist. Seven are outputs the code creates on first run: `api_cache`, `benchmark_history`, `corrections`, `experiment`, `forgetting`, `relations.json` and `definitions.txt`. Four are inputs, and three of the four are silent.

| missing | what reads it | what actually happens |
|---|---|---|
| `data/chunks/` | `ChunkStore::load_all` | returns an empty vector, no warning; every definition lookup finds nothing |
| `data/dictionary_corpus.txt` | 7 bins' default argument | those bins `exit(1)` |
| `data/stop_words.txt` | `PhiConfig::stop_words` | empty set; `is_stop_word` is false for every word |
| `data/searle_markers.json` | `SearleMarkers::load` | one stderr line, then empty marker lists; every input is `Assertive` |

The worked consequences are plain. Because `is_stop_word` is false for every word, the content-word filter in every fingerprint ever computed did nothing. Because the marker lists are empty, `SpeechActAgent` classifies every input as `Assertive`, and the `speech_act` and `direction_of_fit` that the API returns are constants. Because `data/chunks/` is absent, every definition lookup in chat, grounding, `conception` and `bin/defstats` finds nothing.

Binaries fail for want of the first two inputs. Docs/82 says ten, and names `latency`, `bootstrap_facet`, `roles`, `topdown`, `types`, `context`, `conception`, `relations`, `defstats`, `forgetting` and `ingest`. `latency` is the one that matters. Learn, recall, correct and unlearn at p50 and p99 is the project's central claim, and there is no way to run its benchmark on this tree.

Docs/82 then ranks the capabilities still absent after docs/78 to docs/81. The server and REPL chat are templates. `/api/chat` answers from `WordSelector::synthesize` with the pattern `"{} concerns {} and {}."`, and the manifold supplies only the nouns. `Tokenizer::tokenize` deletes punctuation and case, and `split_sentences` splits on every `.`, so "Mr. Darcy" is two sentences. Five of six objectives write only the source side. Only `Trainer::couple_relations` moves the target side, so every pass that improves the manifold desynchronises the directed reading that generation depends on. There is no shared-lexicon operator: `data/austen_operator.fno` is Austen's alone. There is no discriminative attribution: the sector reader reads 44% where a per-writer Kneser-Ney panel reads 72%. Retrieval is O(V) per token, and `SectorIndex`, written to fix that, is called only from its own test. Large corpora have no streaming reader and no sampled softmax. `joint::write_back` exponentiates learned magnitudes into a `u32` that can saturate silently. The shipped grounding path disables `conception` with `bind = false`, `strong = weak`, `graph = None` and `hold = None`.

## 11. What it depends on, and what depends on it

The family has a direction. In the proposed shape every child depends on phinet, and phinet depends on nothing in the family. In the tree today, phifno and phiano depend on phinet, while phi-cal and phiecs record no dependency on it. Phinet's own dependencies are num-complex, serde and rayon. So any change to phinet's `Tensor`, `Tape`, FFT, `Fno`, sphere or hexagram gates reaches every child and every application. The 62/62 parity with PyTorch and the 37 tests are what the whole family stands on.

Phifno depends on phinet only. It reads `sphere` and `Tie::Sphere` from it. The sphere results in PHIENT_FACTS, 12.8×/13.1× for the sphere tie and 16.0×/15.6× for the anisotropic tie on Austen, and MRR 1.000 against the circle's 0.662 on the synthetic degree filter, are results of phinet's sphere. Phifno packages that operator for words.

Phical, as the crate `phi-cal`, depends on nothing recorded in the table. Phiano depends on it. In the proposed shape phiton depends on it too, through `quantum`. Moving it to `gemphi/phical` changes every phiano import, which is why docs/85 rates that move the riskiest.

Phiton depends on `phical::quantum` and, once lifted, on phinet's `c64`. Its two consumers are the helix and the sector palette, both in phiano. Phinum depends on nothing that reads it. Phiqlm depends on nothing and is depended on by nothing, because it does not exist.

Phiano depends on phinet and phi-cal and is read by the binaries, the server and the helix. Phiecs has no crate dependency. It exchanges TSV files with phiano, so no `Cargo.toml` edge records the quest. A reference count cannot see that edge either. Phiace depends on gpui and wgpu for the UI and holds phi-cal and phiecs in its `crates` folder today.

Docs/85 adds an instruction that crosses the family: do not recreate `f64`. Where the engine reports a bare number that the family already has a unit for, it should use the unit. The helix strand line is the first case done. Phifno's `Word::energy()` is a rotation-invariant spectrum by degree, and the natural next step is `SpectralBand` per degree, 16 bands over 8 degrees. That is not done, because it would be a mapping without a measurement. The helix header's `band` is the convergence counter that `Phical::quantum(band_n)` clamps, and docs/85 says it should print the quantum beside it. That is not done either. Docs/85 also says what not to do. Units do not go inside the tape. Autograd is arithmetic on complex numbers, and a unit type there would be a cost with no reading.

## 12. The next experiment

The next experiment is move 1 of docs/85: lift `phiano/src/phiton` into its own crate. Docs/85 rates it the move that removes a buried module and risks nothing. It is the right first test of the structure, because it is the smallest move that changes a `Cargo.toml` edge.

The control is the current tree, with phiton as a module inside phiano 0.2.0. The treatment is the same tree with phiton as a crate that depends only on phinet's `c64` and phical's `quantum`, and with phiano re-exporting it. Nothing else changes between the two.

The measures are all already defined in the sources. The layout check from docs/77 must print nothing in the new crate:

```sh
find src -name '*.rs' | xargs wc -l | awk '$1>200'
```

The 226 phiano lib tests must pass in both arms. The helix must produce the same output in both arms. The sources give three facts to compare: 18,496 words coded, a byte-exact round trip, and `love`'s `AAA` reading `418.5 nm indigo #5900F8`. Docs/77 sets the standard that printed output is byte-identical to what it replaced, and the same standard applies here. The sector palette, phiton's second consumer, must print the same colours.

The last measure is the one docs/85 asks for directly. The dead conversions come with the move and are counted again in the next docs/82. The control count is 7 dead items in `phiton` from docs/82, with docs/85 recording that the helix now reads them. The treatment count is the reference count of every public item in the new crate against phiano, its binaries, its examples and its tests. The prediction is that the count falls, because the helix and the palette now name phiton's items across a crate boundary. The result is not measured.

Two outcomes would say the structure is wrong. If lifting phiton forces a dependency on phiano, then phiton needs words and fails the rule. If the helix output differs by one byte, the move changed behaviour and the structure is not free.

Docs/82 names a different first priority for the demo. Its order is the chat path, then `data/stop_words.txt` and `data/searle_markers.json`, then the shared-lexicon operator. That order is about what the system says to a user. Move 1 is about the shape of the family. Both are open, and neither has been run.

## 13. What is wrong or missing

The structure is proposed and not built. Of the five moves in docs/85, none is done. Of the five children, one is a crate in the family's own place. Phical is a crate in the UI workspace and a module of the same name in phiano. Phinum is a module with 9 dead items. Phiton is a module with one real reader. Phiqlm is a name.

The rule for membership has not been tested against any other shape. No alternative structure was built or compared. The claim that a child needs no words is checked only by reading the types.

The inventory counts one crate. Docs/82 counts phiano's 541 files. No dead-code pass over phinet, phifno, phi-cal or phiecs is recorded in the sources. The phiecs link runs through TSV and has no crate edge, so neither the dependency table nor a reference count sees it.

The sources disagree with each other in six places. Docs/82's dead-items table sums to 149, while its text says 112. Docs/82 says ten binaries cannot run and names eleven. Docs/82 counts 50 binaries and docs/85 counts 52 bins. Docs/77 counts 524 phiano files and docs/82 counts 541. Docs/77 reports 220 passing tests and docs/85 reports 226. Docs/82 cites docs/78 for decoding from the function space at 10% shared 4-grams against the sampler's 26%, while PHIENT_FACTS gives operator 8.7% and sampler 25.7%. PHIENT_FACTS is later and is preferred. The date of the inventory in docs/85 is 2026-09-27, later than this paper's date. The paper cites it as it stands.

The tree itself has faults that docs/82 lists and that remain. Three missing inputs are silent. The content-word filter has never run. The speech-act layer returns constants. The `latency` benchmark cannot run. Chat is a template. Five of six objectives write only the source side. `joint::write_back` can saturate a `u32` silently. Seven modules suppress the compiler with a file-wide `#![allow(dead_code)]`.

The `f64` instruction is followed in one place, the helix strand line, and not in the two others docs/85 names. And the sources contain no measurement of what any move costs in build time or test time.

There is also a gap between what the paradigm promises and what this paper can show. The paradigm says meaning is a side effect of relations. This paper applies that to code by counting references. A reference count says whether an item is used. It does not say whether the use is correct. The helix reads `phase_to_color`, and the round trip is byte-exact, but no source checks the colours against anything outside the code.

## 14. How it was found

The history runs through the numbered docs, and each one found part of what this paper states.

Docs/77 set the layout. Every `.rs` file in phiano, phinet and phi-cal became 200 lines or fewer. The 1,490-line `metrics/harness.rs` became a 199-line declaration file and ten pieces, the largest 171 lines, and `cargo test --release --lib` still passed 220 tests with no warnings. Nothing about the model changed. The layout made later counting possible at the grain of one file and one topic.

Docs/78 measured decoding from the function space against the trigram sampler. Docs/82 cites it for shared 4-grams, and that measurement is why the shared-lexicon operator is on the missing list.

Docs/79 was the hand-written inventory, a week before docs/82. It is where `phical::evaluate` was first recorded as unreferenced. Docs/85 dates the unreferenced status from docs/79.

Docs/80 rewrote `Impersonator` onto the generator and fixed `compose` and `impersonate`. The rewrite orphaned the six `IMPERSONATE_*` constants. Docs/82 records that as the normal way the dead list grows.

Docs/81 added the persona files `personas.chroma` and `personas.json` and measured attribution: 44% for the sector reader against 72% for a per-writer Kneser-Ney panel. That gap is item 5 on the missing list.

Docs/82 replaced docs/79 with a mechanical pass. It stripped every `#[allow(dead_code)]`, compiled, and counted references for every public item. It found the 112 dead items, the 162 module-only items, the eleven missing paths and the ten binaries that cannot run. It ended with the rule to delete nothing yet and act on three things before the demo.

Docs/85 took the same method up one level, from items to crates. It listed every `phi*` entry in gemphi, found two things called phical, found that three named children are modules in phiano, and proposed the family shape with phinet as the parent. It recorded the helix reading phiton's `PhitonColor`, the first shipped reader of `phase_to_color`. It set out the five moves and marked none of them done.

Docs/86, cited in PHIENT_FACTS, came after. It scored the quest with Liskov's three operators and filed 256 words across 61 of 64 hexagrams. It does not change the structure.

The method is the same in every step. Docs/77 made the files small. Docs/79 counted by hand. Docs/82 counted by machine. Docs/85 counted crates instead of items. Each step asked which parts relate to which.

## 15. What remains

The five moves remain. Phiton is to become a crate that depends on phinet's `c64` and phical's `quantum`. Phical is to have one name, with the crate moved to `gemphi/phical`, the manifold folded into `facet`, and `evaluate` deleted. Phinum is to be wired or deleted, keeping the variants table only if `bin/hexagram_eval` reads it. Phiqlm is to stay a name until there is one measurement it would produce. The two `Cargo.toml` files are to point at the new paths. Each is to be one compile-and-test, and the reference count is to be taken again after each.

The three priorities of docs/82 remain. The chat path is still a template, and docs/82 calls it the only remaining place where the system says something it did not compose. The two small files `data/stop_words.txt` and `data/searle_markers.json` are still missing, and with them a filter and a whole classification layer. The shared-lexicon operator is still absent, and the dialogue still decodes from trigram tables.

The rest of the missing list remains, to be shrunk a module at a time with the reference count as the test. That list includes the 33 dead `config` constants, the unused drift monitor, `decode_beam`, `SectorIndex`, the seven file-wide `#![allow(dead_code)]` markers, punctuation and case in the tokenizer, target-side writes in five objectives, the `u32` in `joint::write_back`, and the four disabled mechanisms in the grounding path.

The `latency` benchmark remains unrunnable until `data/dictionary_corpus.txt` or `data/chunks/` exist. Until it runs, learn, recall, correct and unlearn at p50 and p99 have no number on this tree.

The unit work remains. `SpectralBand` per degree for phifno's `Word::energy()` is not done, and docs/85 says it needs a measurement before it is a mapping worth making. The quantum beside the helix header's `band` is not printed.

Measurements across the family remain open as PHIENT_FACTS records them. The sphere at wide and deep is unrun. There is no Phi-4 run; the quest is a synthetic oracle only. Liskov's forcing does not beat greedy on text. None of these is a structural question, and each would be run inside the structure this paper describes.

The accounting also remains. The dead-items total of 112 and the table sum of 149 in docs/82 need one count that agrees with itself. A dead-code pass over phinet, phifno, phi-cal and phiecs has not been recorded. When those counts exist, the family can be described by the same test at every level: what relates to what, and what relates to nothing.

## 16. Sources

The local files read for this paper, all under the phiano docs tree:

```text
/home/claude/phiano/docs/papers/phient/BRIEF.md
/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md
/home/claude/phiano/docs/85_PML_STRUCTURE.md
/home/claude/phiano/docs/82_missing_and_unused.md
/home/claude/phiano/docs/77_file_layout.md
```

What each supplied. PHIENT_FACTS.md gave the paradigm paragraph, the family, and every model result cited: the 62/62 parity, the operator on Austen, the sphere and anisotropic ties, the helix, the quest, the byte deadband, and the list of things not done. It also gave the docs/86 result on Liskov's operators.

Docs/85, "PML — one structure for the phi* crates", gave the tree as of 2026-09-27, the dependency table, the family shape with phinet as the parent, the rule that a child reads the 64 channels its own way and needs no words, the instruction not to recreate `f64`, the helix colour for `love`'s `AAA`, and the five moves in order with what each removes and risks. It states that none of the five moves is done and that the helix, phifno and the sphere are.

Docs/82, "What is missing, and what is not being used", gave the mechanical inventory of phiano: 541 files, 29,670 lines, 50 binaries, eleven missing paths with four inputs, the binaries that cannot run, 112 unreferenced public items and 162 module-only items, the per-module dead table, the seven file-wide `#![allow(dead_code)]` modules, the 33 dead constants and the six orphaned by docs/80, the nine missing capabilities, and the rule to delete nothing yet.

Docs/77, "File layout: every source file under 200 lines", gave the layout rule, the check command, the file counts of 524, 31 and 6, the split of `metrics/harness.rs`, and the 220 passing tests at the time.

The other numbered docs, 78, 79, 80, 81 and 86, were not read for this paper. They are cited only as docs/82, docs/85 and PHIENT_FACTS cite them, and nothing is claimed about them beyond what those three documents say.

The paper's own date, 2026-08-10, and its place as 15 of 16 come from the assignment for this paper. The inventory date, 2026-09-27, comes from docs/85. The two are not reconciled here, and the inventory is reported as docs/85 reports it.

No source file was modified. No number in this paper comes from anywhere else. Where two sources gave different numbers for the same thing, both are shown and the later one is used, as section 13 lists. Where a source disagreed with itself, both figures are shown and neither is chosen.
