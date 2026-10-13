# 64 - The Promoted Lexicon: A Word Is Sixty-Four Complex Numbers

> _`RelationalPhasor` moved from an experiment beside the lexicon to the type the lexicon stores._

---

## 1. What changed

Before this, the lexicon stored `SpectralPhasor`: 64 phase angles and **one** amplitude shared by all of them. `bin/constructed` showed the consequence — a shared magnitude factors out of every normalised comparison, so half of every phasor could not influence a result. `relational.rs` wrote the fix on a side type (`RelationalPhasor`: an angle *and* a magnitude per channel, channel `r` belonging to relation `r`) and `bin/relational` measured it, but the engine never stored it.

Now the lexicon's own type carries it. A word is a polygon whose sides are the relations it has been observed under, each side's length the evidence for that relation.

| Where | What |
|---|---|
| `phasor.rs` | `SpectralPhasor` gains 64 × 16-bit evidence counts (`evid`, packed four to a `u64`). `evidence(k) = ln(1 + count)`, `deposit(k, n)`, `evidence_total`, `dominant_relation`. **`similarity_with(other, weights)`** — normalised complex inner product with evidence as magnitude, compares only relations both words take part in, falls back to `resonance` when either side has none. **`on(other, k)`** — agreement on one relation. **`shared_relation_with(other, weights)`** — the channel the corpus most often put the pair under. `bind`/`unbind` carry evidence through. |
| `facet/mod.rs` | `relations: RelationVocab` (offset channels 0–7 always; marker channels learned by `learn_relations`), `channel_balance()` (1 / mean evidence per channel, so a relation every pair feeds does not outvote one few do), `channel_dispersion(k)`, `similarity`, `on`, `relation_name`. |
| `trainer/mod.rs` | `Trainer::couple_relations` — every pair within `RELATION_WINDOW` is coupled by one Kuramoto step **on the channel of its relation only** and deposits one unit of evidence on both words. A sampled negative is pushed away on the same channel. Runs inside `train`, `train_sentence`, `train_multi_epoch`. `Trainer::without_relations()` is the control arm; `train_corpus` and the harness learn the marker inventory first. |
| `storage.rs` | `.chroma` v4 carries evidence and the relation table. v1–v3 still load; v3 loads with empty evidence and a note. |
| `bin/promoted.rs` | The measurement below. |

`resonance` is untouched. A phasor with no evidence compares exactly as before, so every pre-existing measurement is reproducible from a lexicon that never deposited any, and `Trainer::without_relations()` reproduces the old manifold to the bit.

## 2. The one thing the side experiment did not have

`RelationalFacet::train` is attraction-only and then **mutes** channels that collapsed (`silence_collapsed`). On the shipped trainer that is not an option — a muted channel is a channel the ranking objective loses too. The first promoted run measured it directly: after four passes on WikiText-2 prose, `offset+1` had dispersion 0.32 and `offset+4` 0.36. Every word agreed with every other there.

The project already knew the rule (`trainer/mod.rs`, top): Kuramoto with all-positive coupling has one attractor. So `couple_relations` pushes one sampled non-co-occurring word away from the anchor on the same channel, at `RELATION_RATE · NEG_RATE`. With that term the same channels read 0.89 and 0.93. It doubles the cost of the coupling pass (31 s → 70 s over 96k sentences, four passes) and it is not optional.

## 3. What it measures

`cargo run --release --bin promoted` on `data/prose_corpus.txt` (WikiText-2 via `bin/prepare_prose`, 95,957 sentences, 36,577 words). Control and promoted differ in exactly one thing: `relations` on or off. Same seed, same passes, same probes, same candidate pools.

### Similarity (pair > random, `RelationBenchmark` families)

```
control:  dense resonance (relations off)      51.7%
promoted: dense resonance                      49.7%
promoted: evidence-weighted, balanced          52.3%
promoted: evidence-weighted, raw               47.1%
promoted: best single relation                 27.5%
```

Coupling the relation channels costs the dense comparison two points. The evidence-weighted comparison, balanced per channel, gets them back and a little more. These families sit near chance on this corpus in every arm, so the honest reading is "no cost, no gain" on similarity — which is what `bin/relational` predicted for dense-vs-sparse comparison.

### Composition (`a : b :: c : ?`, mean reciprocal rank, each arm against its own decoy null)

```
                       pair's channel     dense promoted    dense control
gender                 0.01730   8.1x     0.01342   7.6x    0.00189   0.8x
comparative            0.01587  13.0x     0.00120   0.9x    0.00352   1.1x
quality                0.01505  10.9x     0.00668   8.5x    0.00095   0.6x
past_tense             0.00560   7.0x     0.00104   1.0x    0.00092   0.8x
antonym                0.01165   1.9x     0.00162   1.9x    0.00078   1.0x
nationality            0.01600   1.8x     0.00158   0.5x    0.00054   0.3x
hypernym               0.02226   1.7x     0.00291   0.4x    0.00614   2.9x
number / agent / negation          at or below null in every arm
ALL (1536 probes)      0.01084   2.0x     0.00304   1.2x    0.00194   1.2x
```

"Pair's channel" is unsupervised: `shared_relation_with(balance)` picks, per probe, the channel the corpus most often placed `a` and `b` under, and the step `θ_c + θ_b − θ_a` is taken on that channel alone. It is the reading of the representation the architecture exists for, and it is the only one of the three that composes above its null overall. The dense control — the shipped manifold, relations off — sits at its null, as `bin/relational` found it before.

The gain is confined to families whose step has a surface form the corpus marks — gender, comparative, quality, past tense — and the "family channel" column (supervised: the channel the family's own pairs agree on most) does *not* beat its null here, so the 5.6× that `bin/relational` reported for the isolated `RelationalFacet` is not what the promoted lexicon gives. The two objectives share channels, and the ranking objective still moves sixteen of them per token with no regard for what they mean. That is the next thing to measure, and it is one number: `CHANNELS_PER_UPDATE` restricted to channels the relation inventory has not claimed.

### Held-out perplexity (`bin/evaluate data/rust_book_corpus.txt 4`, clean baseline build vs this)

```
                                     baseline    promoted
phase manifold alone (gamma = 1.0)     245.44      207.13
  predictive-heavy                     239.95      203.37
Phiano best valid perplexity           233.54      191.25
Kneser-Ney trigram                     131.02      131.02
unigram back-off (no phase)            148.92      148.92
```

The manifold used as a back-off distribution predicts held-out text 16–18% better than before; the mixing sweep still prefers `gamma = 0`, so the phase layer is still not paying for itself against Kneser-Ney. It is closer.

## 4. What a polygon looks like

```
the      64 sides; loudest: offset+1=65535, offset+2=65535, offset+3=65535 ...
about    54 sides; loudest: offset+1=1388, offset+4=1016, offset+2=948 ...
access   39 sides; loudest: offset+1=568, offset+4=424, offset+5=364, offset+6=320, to=300
```

`offset+3` is quieter than `offset+4` everywhere, and that is the marker rule working: a pair two words apart with `of` or `to` between them is an `of` pair, not an offset-3 pair, and nothing at offset 4 or beyond is ever diverted. Function words have every side; a content word has the sides its uses gave it.

## 5. Channel ownership

The measurement above was taken with every channel claimed by a relation (8 offsets + 56 markers), so the ranking objective's rotating sixteen channels per token landed on relation channels and undid what the coupling had put there. `Trainer::update_channels` now draws the ranking and centroid steps only from `RelationVocab::unclaimed()`, and `MARKER_CHANNELS_MAX` decides the split. Same corpus, same seed, same probes; the only thing varied is how many channels relations own.

```
relation / prediction channels   similarity (balanced)   composition, dense promoted   pair's channel
16 / 48   (8 markers)                    50.8%                   0.00228   2.0x           1.1x
32 / 32   (24 markers)                   55.8%                   0.00314   3.0x           1.3x
48 / 16   (40 markers)                   50.9%                   0.00635   3.3x           1.3x
64 /  0   (56 markers, shared)           52.3%                   0.00304   1.2x           2.0x
control (relations off)                  51.7%                   0.00107   0.9x             —
```

Once the ranking objective stops touching relation channels, the **dense** analogy — all 64 channels, the reading generation and retrieval already use — composes at 3× its null, where it sat at its null in every arm before (gender 10.4×, number 6.2×, quality 6.1×). The single-channel traversal loses its edge at the same time, which is consistent: the relation channels are no longer the only place structure survives. Held-out perplexity on the Rust book moves 191.25 → 182.23 at 32/32 (baseline 233.54).

`MARKER_CHANNELS_MAX = 24` ships. It is one seed and the similarity column is within noise of itself across the sweep, so the split is a default, not a finding; the composition column and the perplexity are the finding.

## 6. What is still open

* **Ownership inside the ranking step.** The ranking objective now stays off the relation channels; it does not yet *use* them. Its per-position context term could deposit on the offset channel it corresponds to, which would make prediction and relation the same operation on the same angle.
* **Recursion.** `Roles::identity` composes a word from lexicon fillers and never from a composed identity. With evidence carried through `bind`, the recovery depth of nested `unbind` is now measurable.
* **Senses.** `bin/senses` split a word into per-context variants and found it did not pay on next-word prediction. Its stated hypothesis was that it should pay on relations. The relation channels now exist to test that on.
