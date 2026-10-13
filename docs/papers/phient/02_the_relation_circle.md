# The Relation Circle
*Phient papers, 2 of 16 · 2026-01-26 · The relation circle: 64 channels, markers and offsets, evidence as a count*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

This paper describes the relation circle as it is built in phiano. A word in the lexicon is 64 channels. Each channel is a complex number. Its angle says where the word sits under one relation. Its magnitude says how much evidence the corpus has given for the word standing in that relation at all. The relations are not written by hand. Two label-free sources supply them. The first is the offset between two words in a sentence, from 1 to 8, which fills channels 0 to 7. The second is the marker: the word standing between two others, such as `of`, `in`, `to` or `as`, ranked by how often it links pairs in the corpus. Markers fill the channels after the offsets. The channels no relation claims are left to the ranking objective.

Training couples two words on one channel only, the channel of the relation under which they were seen. Each coupling adds one unit of evidence to both words on that channel. Evidence is a 16-bit count per channel, read as `ln(1 + count)`. A word is therefore a polygon whose sides are its relations and whose side lengths are its counts.

The idea was first tested as a side experiment, `RelationalFacet`, and lost on a similarity benchmark: 50.1% pair-over-random against 61.5% for the shipped representation in that run. It was then promoted into the lexicon type, `SpectralPhasor`, with a repulsion term the side experiment lacked. On WikiText-2 prose the promoted lexicon gave no gain and no cost on similarity (52.3% balanced against a 51.7% control). On composition, reading the step on the pair's own channel scored 2.0× its null over 1,536 probes, where the dense control sat at 1.2×. Once the ranking objective was kept off the relation channels, the dense reading composed at 3.0× its null and held-out perplexity on the Rust book fell from 233.54 (baseline) to 182.23.

These are single-seed numbers. The split of 24 marker channels that ships is a default; docs/64 does not call it a finding. On the synthetic quest oracle the same channels reach 0.594 MRR only with attraction-only coupling and dithered rounding, and the shipped trainer reaches 0.082. The byte that stores each angle is the limit found there. The paper states what was built, what was measured, what disagrees between the sources, and what is still open.

## 2. The claim

The claim is small and exact. A relation is a place. Channel `r` belongs to relation `r`, and nothing else is allowed to shape it. Once that holds, a word's value on channel `r` is a statement about the word under that relation, and a word's magnitude on channel `r` is a statement about how often the corpus has put it there.

Before the relation circle, this was false in two ways, and both were measured. `bin/dimensions` showed that the channels could not specialise. Seeds were hashes, the Kuramoto step treated every channel alike, and `channel_kernel` varied only decay and frequency. Nothing distinguished channel 17 from channel 42. Scoring a pair by its best single dimension came out at 43.9% against a 50% chance line. `bin/constructed` showed that the magnitude could not matter. Amplitude was one number per word, shared across all 64 channels, so it factored out of every normalised inner product. Phase-only and full-complex scoring agreed to the decimal place in every arm. Half of every phasor could not influence a result.

The relation circle fixes both with one change. A word carries an angle and a magnitude per channel, and channel `r` is owned by relation `r`. A word never observed under relation `r` has magnitude near zero there and contributes nothing to comparisons along it. The complex number now does work the angle alone cannot.

The claim has a cost. Relation-specific channels make comparison sparse. Two arbitrary words almost never co-occur under `of`. So on a similarity task, where any two words must be compared, the only channels they share are the offset channels, which carry where a word sits rather than what it means. Sparse comparison loses to dense averaging on similarity. The benefit the architecture is built for is composition: working deliberately inside one relation. A similarity benchmark measures the cost of the trade and not its benefit.

So the claim, as tested, has three parts. First, an observation moves one channel and leaves the other 63 alone; unit tests prove this. Second, per-channel magnitude changes a similarity, which the old representation could not do. Third, a step taken on the channel a pair shares composes above its null where the dense control does not. The first two are mechanical and hold. The third holds on the WikiText-2 families whose step has a surface form the corpus marks: gender, comparative, quality and past tense. It does not hold for number, agent or negation, which sit at or below null in every arm of the promoted run.

The claim does not extend to prediction. The phase layer still does not pay for itself against Kneser-Ney on held-out text.

## 3. The mechanism

The inventory is `RelationVocab`, in `src/relational.rs`. It holds one field, `marker_channel: HashMap<String, usize>`. Offsets are not stored; they always hold channels `0..OFFSET_CHANNELS`, and `OFFSET_CHANNELS` is 8. `RelationVocab::learn(docs)` builds the marker table from a corpus. `RelationVocab::marker(toks, i, j)` returns the marker channel for a pair, if a word between them names their relation. `RelationVocab::channel(toks, i, j)` returns the marker's channel if there is one, otherwise `d - 1` for an offset `d` up to 8, otherwise `None`. `unclaimed()` returns the channels above `OFFSET_CHANNELS + markers()`. `name(k)` names a channel. `entries()` and `from_entries()` serialise the table.

The constants beside it in `relational.rs` are these.

| Constant | Value | Meaning |
|---|---|---|
| `OFFSET_CHANNELS` | 8 | channels reserved for relative offset |
| `WINDOW` | 8 | how far apart two words may be and still count as related |
| `RATE` | 0.06 | coupling strength per observation |
| `CHANNEL_DISPERSION_FLOOR` | 0.35 | below this circular dispersion a channel has collapsed |
| `GUARD_MIN_VOCAB` | 200 | below this vocabulary the collapse guard does not run |
| `MARKER_MAX_GAP` | 3 | widest gap at which a word between a pair may name it |

The evidence lives in the lexicon type. Per docs/64, `SpectralPhasor` in `phasor.rs` gained 64 × 16-bit evidence counts, `evid`, packed four to a `u64`. `evidence(k)` is `ln(1 + count)`. `deposit(k, n)` adds to a count. `evidence_total` and `dominant_relation` read the profile. `similarity_with(other, weights)` is a normalised complex inner product with evidence as the magnitude. It compares only the relations both words take part in, and falls back to `resonance` when either side has none. `on(other, k)` is agreement on one relation. `shared_relation_with(other, weights)` returns the channel the corpus most often put the pair under.

The `Facet` holds `relations: RelationVocab`, learned by `learn_relations`. `channel_balance()` returns 1 over the mean evidence per channel, so a relation every pair feeds does not outvote one few pairs feed. `channel_dispersion(k)`, `similarity`, `on` and `relation_name` complete it.

The trainer, in `src/trainer/trainer.rs`, does the moving. `Trainer::couple_relations(facet, tokens)` visits every pair within `RELATION_WINDOW`, asks `facet.relations.channel` for `k`, and applies one directed Kuramoto step on `k` alone. Both words receive `deposit(k, 1)`. A sampled negative is then pushed away on the same channel. `Trainer::update_channels` draws the ranking and centroid steps only from `unclaimed()` when relations are on. `Trainer::ensure_relations` learns the inventory if the facet has none. `Trainer::without_relations()` is the control arm and reproduces the old manifold to the bit. `Trainer::with_rounding` sets how each nudge is rounded to its byte.

The `.chroma` v4 format stores the evidence and the relation table. Versions 1 to 3 still load, with empty evidence.

## 4. What was measured

Three runs measure the relation circle in phiano, and one measures it on the quest.

The side experiment, `bin/relational`, ran `RelationalFacet` on WikiText-2 prose: 36,577 words, 56 marker channels plus 8 offset channels.

```text
similarity                                pair>random
shipped: mean of 64 channels                    61.5%
relational: evidence-weighted, all              50.1%
relational: best single dimension               38.8%
relational: mean of best 8                      41.6%
```

Only gender (+0.094) and agent (+0.058) cleared the −0.004 noise floor. Of the ten relation families, every one's best channel was an offset channel covering 100% of its pairs. The marker channels covered almost none. The side experiment was attraction-only and muted collapsed channels, which the shipped trainer cannot do.

The promotion, `bin/promoted`, ran on `data/prose_corpus.txt`: 95,957 sentences, 36,577 words. Control and promoted differ only in whether relations are on.

```text
control:  dense resonance (relations off)      51.7%
promoted: dense resonance                      49.7%
promoted: evidence-weighted, balanced          52.3%
promoted: evidence-weighted, raw               47.1%
promoted: best single relation                 27.5%
```

The two documents disagree on the shipped dense score: 61.5% in `relational.rs`, 51.7% for the control in docs/64. They are different runs of different binaries. This paper prefers docs/64, the later document.

Composition, `a : b :: c : ?`, by mean reciprocal rank against each arm's own decoy null:

```text
                       pair's channel     dense promoted    dense control
gender                 0.01730   8.1x     0.01342   7.6x    0.00189   0.8x
comparative            0.01587  13.0x     0.00120   0.9x    0.00352   1.1x
quality                0.01505  10.9x     0.00668   8.5x    0.00095   0.6x
past_tense             0.00560   7.0x     0.00104   1.0x    0.00092   0.8x
ALL (1536 probes)      0.01084   2.0x     0.00304   1.2x    0.00194   1.2x
```

The channel-ownership sweep varied how many channels relations own:

```text
relation / prediction channels   similarity   dense composition   pair's channel
16 / 48   (8 markers)                50.8%     0.00228   2.0x         1.1x
32 / 32   (24 markers)               55.8%     0.00314   3.0x         1.3x
48 / 16   (40 markers)               50.9%     0.00635   3.3x         1.3x
64 /  0   (56 markers, shared)       52.3%     0.00304   1.2x         2.0x
control (relations off)              51.7%     0.00107   0.9x           —
```

Held-out perplexity on the Rust book (`bin/evaluate data/rust_book_corpus.txt 4`) went from 233.54 (baseline, Phiano best valid) to 191.25 promoted, and to 182.23 at 32/32. The phase manifold alone went from 245.44 to 207.13. Kneser-Ney trigram stayed at 131.02.

On the phiecs quest oracle (256 words, 1,280 rows, chance MRR 0.021), the shipped trainer scored 0.082 MRR. Attraction-only with dithered rounding scored 0.594, hit@1 34.8%, hit@5 97.4%; 0.747 within kind; 0.015 on the wrong channel; 0.005 by offset reading.

## 5. The one idea, restated

A word is sixty-four places, each owned by one relation, and each place carries two things: where the word stands there, and how many times it was seen there.

Everything else in this paper follows from keeping those two things apart and keeping each place owned. The angle is the relation's arrangement of the vocabulary. The count is the word's share in that relation. An angle with no count behind it says nothing and weighs nothing. A count with no angle behind it would be a bag of relation frequencies. The pair of them is a phasor that can be compared along one relation or across all of them, and the comparison is weighted by how much the corpus actually said.

Ownership is the part that took longest to see. In the first promoted run every channel was claimed by a relation, 8 offsets and 56 markers, and the ranking objective still moved sixteen channels per token with no regard for what they meant. It undid on each step what the coupling had built. The fix was to let relations own a block of channels and let prediction own the rest. With `MARKER_CHANNELS_MAX = 24`, channels 0 to 7 are offsets, channels 8 to 31 are markers, and channels 32 to 63 belong to the ranking objective. The dense analogy then composed at 3.0× its null where it had sat at 1.2×.

The count is the other part. Evidence is deposited, one unit per observation, and never learned by gradient. It is the only quantity in the lexicon that is a plain count of the text. That makes it auditable. The polygon for `the` in docs/64 shows `offset+1=65535`, `offset+2=65535`, `offset+3=65535`: every side saturated at the top of a 16-bit count. The polygon for `access` has 39 sides, loudest `offset+1=568`, `offset+4=424`, `offset+5=364`, `offset+6=320`, `to=300`. Anyone can read those numbers and check them against the corpus. A content word has the sides its uses gave it. A function word has every side.

The idea also says what it cannot do. It cannot make two words that never share a relation comparable. It cannot tell `eats` from `a` except by how often each links. It cannot move an angle by less than one quantum of its byte. Each of these is a measured limit and each appears again in section 13.

The paradigm sentence says meaning is a side effect of pairs of words moving under a condition. On the relation circle, the condition is literal. It is the channel index, chosen by the offset or the marker between the pair, and the movement is one Kuramoto step on that index and no other.

## 6. The sixty-four channels

The channel count is `PHASE_CHANNELS`, 64. The layout is fixed by rule and filled by the corpus.

| Channels | Owner | How assigned |
|---|---|---|
| 0–7 | offsets | always; channel `d - 1` for offset `d` |
| 8 to 8 + markers − 1 | markers | by linking count, most frequent first |
| the rest | ranking and centroid objectives | `RelationVocab::unclaimed()` |

The number of markers is `MARKER_CHANNELS_MAX.min(PHASE_CHANNELS - OFFSET_CHANNELS)`. With 64 channels and 8 offsets, the ceiling is 56. That ceiling is what the side experiment and the first promoted run used. docs/64 §5 swept it and shipped 24, which leaves 32 channels unclaimed.

The ordering of marker channels carries meaning by construction. `learn` sorts by linking count, descending, with ties broken by the word so the assignment is reproducible. Channel 8 goes to the word that links most often, channel 9 to the next. So a higher channel index is always a rarer marker. `bin/markers` printed the absolute ranking on its corpus as `the and of to in a was by on for with that as from at`. If that ranking holds on the training corpus, `the` takes channel 8, `and` 9, `of` 10, `to` 11, `in` 12 and `a` 13. That fact is used directly by the marker rule in section 8, which picks the highest channel index among the words between a pair.

`name(k)` makes the channels readable. `name(0)` is `offset+1`, `name(7)` is `offset+8`, a marker channel returns its word, and an unclaimed channel returns `ch{k}`, for example `ch40`.

The channels are also where the rest of the family attaches. In the helix, the lexicon is written as a DNA-coded tree in which each of the 64 channels is a codon, the base-4 ACGT spelling of its index, with a hexagram, a sphere degree and order, and a PhiTon wavelength and band. On the sphere, the same 64 channels are read as 8² spherical harmonics `Y_l^m` with `l < 8`. Neither reading changes which relation owns which channel.

Three ownership states follow from the table. A channel below 8 is always a relation and is always fed, because every adjacent pair has an offset. A marker channel is fed only by pairs with that word between them. An unclaimed channel receives no relation coupling and no evidence at all; the ranking objective alone moves it. `update_channels` takes `CHANNELS_PER_UPDATE` channels from that pool, stepping by 7 from a seeded offset. When relations are off, or when every channel is claimed, it falls back to the whole torus, which is the original behaviour.

## 7. Offsets

The offset is the one relation always available. Two words in a sentence are always some distance apart. `RelationVocab::channel` returns `d - 1` for a gap `d` from 1 to 8, so the words next to each other land on channel 0, `offset+1`, and words eight apart land on channel 7, `offset+8`. Beyond 8 there is no offset channel and the pair is skipped unless a marker names it.

Take the sentence `the dog eats the donut`. The pairs are the positions `(i, j)` with `j > i` inside `RELATION_WINDOW`. The pair `(the, dog)` has `d = 1` and nothing between, so it lands on `offset+1`. The pair `(dog, eats)` also lands on `offset+1`. The pair `(dog, the)` at positions 1 and 3 has `eats` between it; if `eats` has a marker channel, it goes there, and otherwise to `offset+2`. The pair `(the, the)` at positions 0 and 3 is skipped, because `couple_relations` ignores a pair of identical words. The pair `(dog, donut)` at positions 1 and 4 has a gap of 3, so `eats` and `the` stand between it, and the marker rule decides.

Offsets are fed by every sentence, and that is their problem. The side experiment found it twice. First, channel traffic is wildly unequal. Every adjacent pair feeds an offset channel while a marker channel sees only pairs joined by that word. The offset channels outvoted everything by two orders of magnitude, and the first run scored 35.9%, below chance. `RelationalFacet::balance` equalised the vote and recovered 14 points. In the promoted lexicon the same job is done by `channel_balance()`, 1 over the mean evidence per channel. The balanced evidence-weighted similarity is 52.3%; the raw one is 47.1%.

Second, offsets collapse. Attraction-only Kuramoto has one attractor, every word at one angle, and a channel every adjacent pair feeds reaches it fast. After four passes on WikiText-2 prose, `offset+1` had dispersion 0.32 and `offset+4` 0.36. Every word agreed with every other there. The repulsion term in `couple_relations` brought the same channels to 0.89 and 0.93. It doubled the coupling pass, 31 s to 70 s over 96k sentences and four passes.

Offset channel 0 has one extra duty. When `k == 0`, the trainer calls `sync_phase()` on the source word after the nudge, so the adjacent-word channel keeps the word's scalar phase in step.

The offset profile shows the marker rule working. In docs/64, `offset+3` is quieter than `offset+4` for every printed word. A pair three apart with `of` or `to` between it is diverted to the marker's channel. Nothing at offset 4 or beyond is ever diverted, because `MARKER_MAX_GAP` is 3. So `offset+3` loses traffic that `offset+4` keeps. For `about`: `offset+1=1388`, `offset+4=1016`, `offset+2=948`.

## 8. Markers

A marker is the word between two others that names their relation. `bread of wheat` is an `of` relation first and an offset-2 relation second, and `channel` gives the marker precedence because it is the more specific claim.

The inventory is read from the corpus by `RelationVocab::learn`. For every position `i`, it looks at the pairs `(i, j)` with a gap of 2 or 3, the same gaps `marker` will consult, and counts each word standing between them. Words are ranked by that absolute count. Two alternatives were tried and rejected. Restricting the count to words between two content words failed on English: in `the dog eats the donut` the neighbours of `eats` are `dog` and `the`, so `eats` could never be counted as linking, and the sentence that motivated the rule became the one sentence it could not see. Ranking by the share of a word's occurrences spent linking gave `nautical stoppani cu facto botanist`, words that occur a dozen times and happen to fall between two rare words each time. A ratio rewards a small denominator, and the words that name relations have the largest denominators in the language. Absolute count gives `the and of to in a was by on for with that as from at`. No minimum count applies.

The rule for which marker names a pair is rarity. Among the words between the pair, the marker is the one that links least often across the corpus. Because channels are handed out in rank order, a higher index is a rarer marker, and the rule is a maximum over channel indices:

```rust
((i + 1)..j)
    .filter_map(|m| self.marker_channel.get(&toks[m]).copied())
    .max()
```

The source works two cases. Between `president` and `states` stand `of` and `the`. `of` links thirty-two thousand times against `the`'s forty thousand, so `of` has the higher channel and wins. Between `dog` and `donut` in `a dog eats a donut` stand `eats` and `a`. `a` links fourteen thousand times and `eats` a handful, so `eats` is the more specific claim and wins, provided `eats` holds a channel.

It replaced two earlier rules. The first fired only when exactly one word separated the pair, so `dog eats donut` was an `eats` relation and `a dog eats a donut` was an offset-3 relation. A unit test in `phase_flow` caught it by asserting the marker was `eats` and getting `offset+3`. The second fix was a filler list of frequent words to skip, and it deleted `of`, `in` and `to`, which are frequent and are the clearest relational markers in the language. Rarity settles both cases without a list. A hand-written list of English function words is a claim about English that the module has no way to check.

## 9. Evidence as a count

The magnitude of a channel is a count of observations. It is kept as a 16-bit integer per channel, 64 of them per word, packed four to a `u64`. `evidence(k)` reads it as `ln(1 + count)`. `deposit(k, n)` adds to it. Nothing else writes it.

The trainer deposits exactly one unit per coupling, on both words, on the coupling's channel. In `couple_relations`, the source word gets `p.deposit(k, 1)` after its nudge and the target word gets `p.deposit(k, 1)` after its own. The sampled negative is nudged and receives no deposit, because it was not observed under that relation. So after training, the count on channel `k` of word `w` is the number of times `w` stood in a pair the channel rule assigned to `k`, within the window, across all passes. It is a count of the text.

The logarithm is there so that a function word cannot drown a content word by volume alone. The 16-bit width is a ceiling. `the` reaches 65535 on `offset+1`, `offset+2` and `offset+3` in docs/64, which is the top of the range. Once a count saturates, it stops distinguishing. For `the` that costs little; every side is full and the word is everywhere. For a content word on a common channel it has not been measured.

The count does work the old amplitude could not. The old `SpectralPhasor` had one amplitude for all 64 angles. In a normalised inner product that amplitude factors out exactly. With a count per channel, the magnitude varies across channels and survives normalisation. `similarity_with(other, weights)` compares two words only on the relations both take part in, with each channel weighted by both counts and by `weights`, which is normally `channel_balance()`. Two words that share no relation fall back to `resonance`, the dense comparison, so a lexicon that never deposited any evidence compares exactly as before.

The count also picks the channel for composition. `shared_relation_with(other, weights)` returns the channel the corpus most often put the pair under, balanced. In the promoted run it is the "pair's channel" column: for each probe `a : b :: c : ?`, the channel is chosen without supervision by `shared_relation_with(balance)`, and the step `θ_c + θ_b − θ_a` is taken on that channel alone. That reading scored 2.0× its null over all 1,536 probes, and 13.0× for comparative, 10.9× for quality, 8.1× for gender, 7.0× for past tense.

Three polygons in docs/64 make the count concrete.

```text
the      64 sides; loudest: offset+1=65535, offset+2=65535, offset+3=65535 ...
about    54 sides; loudest: offset+1=1388, offset+4=1016, offset+2=948 ...
access   39 sides; loudest: offset+1=568, offset+4=424, offset+5=364, offset+6=320, to=300
```

`access` has a marker side, `to=300`, among its loudest five. That is `access to`, counted.

## 10. Coupling on one channel

The coupling step is where a relation becomes an arrangement. For each pair `(i, j)` on channel `k`, `couple_relations` reads the earlier word's source angle `ta = a.theta(k)` and the later word's target angle `tb = b.theta_target(k)`, and takes `d = sin(tb − ta)`. The source side of word `i` is nudged by `rate · d`. The target side of word `j` is nudged by `−rate · d · boost`. Both get one unit of evidence on `k`.

The step is directed. Source and target are different angles, so "a then b" and "b then c" share no variable and cannot chain into one global angle.

Rare target sides get more weight. If the target word's count on `k` is below `RARE_TARGET_FLOOR`, its nudge is multiplied by `RARE_TARGET_BOOST`, per docs/72. A word seen few times after its neighbours would otherwise barely move.

The repulsion term follows. One word is drawn from the relation pool by `sample_relation_negative`, seeded by the sentence hash, the trainer seed and the pair positions. If the sampled word appears anywhere in the sentence it is skipped. Otherwise its target side on `k` is pushed away from word `i`'s source side by `rate · NEG_RATE · away`, where `away = −sin(ta − tn)`, and word `i`'s source side is pushed the other way. This is the centroid objective's contrastive rule restricted to one channel. It is what took the offset channels from 0.32 and 0.36 dispersion to 0.89 and 0.93.

A worked case. In `the president of the states`, the pair `(president, states)` has gap 3. `of` and `the` stand between. The marker rule picks `of`'s channel, 10 under the `bin/markers` ranking. `president`'s source angle on channel 10 moves toward `states`'s target angle there by 0.06 · sin d. Both words gain one `of` count. One sampled word not in the sentence is pushed away from `president` on channel 10 only. Channels 0 to 7, the other markers, and the 32 unclaimed channels are untouched by this pair.

The angle is stored as one byte: 256 quanta of 2π. Each nudge is rounded to its byte under `Trainer::with_rounding`. PHIENT_FACTS records what that does. A nudge of 0.06 · sin d falls under half a quantum once sin d < 0.2, about 11°. With nearest rounding every pair froze 8 quanta short of agreement. Attraction-only coupling with dithered rounding scores 0.594 MRR on the quest where the shipped trainer scores 0.082. Dither costs persona attribution 71% to 65%, so it ships off by default.

Channel ownership completes the step. `update_channels` draws the ranking step only from `unclaimed()`, so channel 10 is shaped by `of` and nothing else.

## 11. What it depends on and what depends on it

The relation circle sits in phiano, the word application of the PML family. It depends on phinet below it and it feeds the children and the applications around it.

phinet is the parent engine: complex tensors `[b, w, s]`, Tape autograd, unitary FFT, the Fourier neural operator, joint training, HRR, 64 hexagram gates and the sphere. phinet matches its PyTorch reference on 62/62 parity tests. The relation circle does not use phinet to train its channels; `couple_relations` is a Kuramoto step in phiano. The phinet operator results on Austen are: identity 7.9× chance, hinge 14.7× valid and 14.6× test, softmax β=8 18.0× and 18.8×, wide and deep 30.7× and 30.3× at 8 epochs, against a chance MRR of 0.0063.

phical is the geometry of the relation circle itself: the closed circle, the phasor with a count, the pair moving under a condition. This paper describes the phiano implementation of what phical states.

phifno 1.0.0 is the spherical operator. Its tie, `phinet::Tie::Sphere`, reads the 64 channels as 8² spherical harmonics. At 1,192 params it scored 12.8× valid and 13.1× test on Austen hinge against the circle's 14.7× and 14.6× at 8,360 params. The anisotropic tie, one weight per `(l, |m|)`, 4,776 params, scored 16.0× and 15.6× and beats the circle. On the synthetic degree-filter task the circle scored MRR 0.662 and the sphere 1.000. The sphere needs the channels to mean something by index; the relation circle is what gives them an index with a meaning.

phiton is light as units. The helix gives each of the 64 channels a PhiTon wavelength, band and hexagram, with a base-4 ACGT codon. 18,496 words round-trip byte-exact in 1.9 s at 75 MB. phinum is a child with no measurement recorded in the sources for this paper. phiqlm is planned and nothing is built.

phiecs is the entity-component world and the quest. Its synthetic oracle has five kinds: is-a, part-of, near, used-for, unlike. Quest ingest reads them back into phiano, scored within kind (0.747) and on the wrong channel (0.015). That is where the byte deadband was found. On held-out rows, one hop scored 0.011 and two hops scored 0.095 for is-a and 0.093 for used-for, with part-of, near and unlike at about chance. Liskov's three operators, scored on the quest and filed by hexagram, put 256 words in 61 of 64 hexagrams, with flat means: better −0.008, worse −0.008, loss 0.108.

phiace hosts the family. Personas, dialogue and generation in phiano read the same lexicon: persona attribution 71% over three writers, dialogue 44% by the sector reader and 72% by the Kneser-Ney panel.

## 12. The next experiment

docs/64 names it. The ranking objective now stays off the relation channels. It does not use them. Its per-position context term could deposit on the offset channel it corresponds to. A ranking step that predicts the word at offset `d` would then add one unit of evidence on `offset+d` and nudge that channel, and prediction and relation would become the same operation on the same angle. This is not built.

The experiment is one change against one control. The control is the shipped trainer: `MARKER_CHANNELS_MAX = 24`, relation coupling on, ranking restricted to the 32 unclaimed channels, nearest rounding, the same seed. The treatment adds the offset deposit to the ranking step and nothing else. A second control, `Trainer::without_relations()`, fixes the floor, since it reproduces the old manifold to the bit.

The measurements are the ones docs/64 already uses, so the result is comparable to every earlier row. On `data/prose_corpus.txt`, 95,957 sentences: similarity, balanced evidence-weighted, against the shipped 55.8%; composition MRR, dense and pair's channel, against 3.0× and 1.3× their nulls, per family and over all 1,536 probes. On the Rust book: held-out perplexity against 182.23, with Kneser-Ney at 131.02 as the line the phase layer has not yet crossed.

Two things must be controlled that were not controlled before. First, seeds. Every number in docs/64 is one seed, and the similarity column is within noise of itself across the sweep. The experiment should run at least the same few seeds in both arms, and report the spread next to each mean. Second, traffic. Offset channels are already the busiest channels. Depositing ranking evidence on them raises their counts further, and `channel_balance()` will lower their weight in return. The run should print, for both arms, the evidence total per channel and the dispersion of `offset+1` and `offset+4`, so that a collapse like the one docs/64 §2 found at 0.32 and 0.36 would be seen directly.

What would count as a result is stated in advance. If the treatment raises dense composition above 3.0× its null on more than one seed and does not raise perplexity above 182.23, the deposit ships. If composition holds and perplexity rises, the two objectives are fighting on the offset channels and the deposit does not ship. If nothing moves, the offset channels already carry what the ranking step would add.

A second experiment is queued behind it and uses the same control. The quest found that nearest rounding freezes pairs 8 quanta short. Dither ships off because it costs attribution. The prose composition benchmark has never been run with dither on. That run is one flag, `with_rounding`, and it is not measured.

## 13. What is wrong or missing

The sources disagree on the shipped dense similarity: 61.5% in `relational.rs`, 51.7% in docs/64. Both are said to be WikiText-2 prose at 36,577 words. The later figure is preferred here, but the difference is ten points and it is not explained in either document.

The composition gain is narrow. It holds for gender, comparative, quality and past tense. It fails for number, agent and negation in every arm. The supervised "family channel" reading does not beat its null in the promoted lexicon, so the 5.6× that `bin/relational` reported for the isolated `RelationalFacet` is not what the promoted lexicon gives.

Every figure in docs/64 is one seed. The 24-marker split is a default.

The phase layer still loses to Kneser-Ney. The mixing sweep prefers `gamma = 0`. 182.23 is closer to 131.02 than 233.54 was, and it is still behind.

Reading the trainer code turns up three defects. None has been measured.

First, the rate expression in `couple_relations` is `RELATION_RATE.min(self.learning_rate.max(RELATION_RATE))`. The inner `max` is never below `RELATION_RATE`, so the outer `min` always returns `RELATION_RATE`. The trainer's learning rate has no effect on relation coupling. Whether that was intended is not stated.

Second, the marker example depends on the inventory being large. With 24 marker channels, a verb seen "a handful" of times cannot hold a channel under absolute-count ranking. In `a dog eats a donut`, `eats` then has no channel and `a` does, so the maximum over channel indices picks `a`. The determiner names the relation, which is the case the rarity rule was written to prevent. The worked example in the source assumes a channel for `eats` that the shipped split does not give it.

Third, `update_channels` steps through the unclaimed pool by 7. With 32 unclaimed channels, 7 and 32 share no factor and the sixteen picks are distinct. If the pool size were a multiple of 7, as it would be at 49 markers, every pick would land on one channel. The shipped split is safe. The code does not guard it.

`learn` also computes a total count per word and then discards it (`let _ = &total;`).

The byte limits the angle. A nudge of 0.06 · sin d is lost under nearest rounding below about 11°. The shipped trainer scores 0.082 on the quest where attraction-only with dither scores 0.594. The shipped trainer has repulsion and nearest rounding; the separate contribution of each to that gap is not stated in the sources read here.

The 16-bit count saturates. `the` sits at 65535 on three channels. The effect of saturation on content words is not measured.

Relation-specific channels make similarity sparse. That is a trade and it stays.

## 14. How it was found

The history runs through the binaries named in `relational.rs` and through the numbered docs.

It began with two negative results. `bin/dimensions` scored pairs by their best single dimension and got 43.9% against a 50% chance line: the channels were symmetric and could not specialise. `bin/constructed` scored phase-only and full-complex and got identical results in every arm: the shared amplitude factored out. The same fix answered both. A magnitude per channel, and a relation per channel.

`relational.rs` built that fix as a side type, `RelationalPhasor`, with `RelationalFacet` to hold it, and `bin/relational` to run it. The relations came from offsets and markers because the author refused a written list. Along the way the marker rule changed three times. A single-word gap missed `a dog eats a donut`, and a unit test in `phase_flow` caught it by asserting `eats` and getting `offset+3`. A filler list deleted `of`, `in` and `to`. The rarity rule replaced both. The ranking changed too. Share-of-occurrences gave `nautical stoppani cu facto botanist`; `bin/markers` printed both rankings side by side and absolute count won. A minimum count was removed after two unit tests failed on small corpora.

The first measurement came in at 35.9%, below chance, because offset channels outvoted markers by two orders of magnitude. `RelationalFacet::balance` recovered 14 points. A second error was in the measurement itself: averaging a channel's agreement over a whole family scored it zero wherever a pair was unseen. It was fixed, and the conclusion did not change. The side experiment finished at 50.1% against 61.5%, and the module header records the result as a loss on similarity.

docs/64, "The Promoted Lexicon: A Word Is Sixty-Four Complex Numbers", moved the idea into `SpectralPhasor`, `Facet` and `Trainer`. Its first run found the offset channels collapsed at 0.32 and 0.36 dispersion, which led to the repulsion term. Its composition table was the first positive result: 2.0× on the pair's channel. Its §5 sweep found that ownership mattered and shipped `MARKER_CHANNELS_MAX = 24`.

docs/72, "Two More Hypotheses, Both Measured, Neither Kept", is cited in `trainer.rs` for the rare-target boost that now sits in the coupling step.

docs/83, "Reading a quest back: phiecs → phiano, and the byte that would not move", read the phiecs oracle onto relation channels and found the byte deadband: nearest rounding froze every pair 8 quanta short. Dither lifted the quest and cost attribution, so it ships off.

docs/84, "The word on the sphere, and the lexicon as a helix", laid the sphere and the helix over the same 64 channels. docs/85 set out the PML structure. docs/86, "Better, worse, loss: Liskov's three operators, for words", scored the quest and found it flat, the byte ceiling again.

## 15. What remains

Five things remain open.

The first is ownership inside the ranking step, listed first in docs/64 §6. Section 12 sets out the experiment. It is not built.

The second is recursion. `Roles::identity` composes a word from lexicon fillers and never from a composed identity. With evidence now carried through `bind` and `unbind`, the recovery depth of nested `unbind` is measurable. It is not measured.

The third is senses. `bin/senses` split a word into per-context variants and found no gain on next-word prediction. Its stated hypothesis was that senses should pay on relations. The relation channels exist to test that on. It is not measured.

The fourth is the byte. The angle on each channel is one byte, and the quest showed what that costs: a nudge under half a quantum disappears, and pairs stop 8 quanta short. Dither recovers the quest and costs persona attribution six points. A wider angle, or a rounding that keeps the expected move without the attribution cost, is not built. The Liskov scores on the quest are flat for the same reason, so the three operators cannot be judged on the relation circle until the byte is resolved.

The fifth is held-out generalisation. On the quest, one hop held out scored 0.011 MRR against a chance of 0.021. Two hops scored 0.095 for is-a and 0.093 for used-for, and the other kinds sat at about chance. The relation channels store what they were shown. They do not yet infer an unseen row of the same kind from its neighbours. No experiment for that is designed in the sources.

Beyond those, several numbers that would describe the relation circle are simply absent. The sphere tie has not been run at softmax or wide and deep. No Phi-4 run exists; the quest is a synthetic oracle only. The effect of 16-bit saturation on content words is not measured. The ten-point disagreement between 61.5% and 51.7% on the shipped dense similarity is not explained. Every docs/64 figure is one seed. The prose composition benchmark has not been run with dither.

The three code readings in section 13 are open questions for the author. None changes a reported number until it is measured.

What does not remain open is the mechanism. An observation moves one channel. A count per channel changes a similarity. A step on a pair's own channel composes above its null for the families the corpus marks. Those three are built, tested and measured, at one seed, on one corpus, and the paper stops there.

## 16. Sources

Every number in this paper was taken from the five files below, read in full, or from the brief that set the form of the series. Nothing was taken from the web. No source file was modified.

| Path | What was taken from it |
|---|---|
| `/home/claude/phiano/docs/papers/phient/BRIEF.md` | the form of the series and the rules for this paper |
| `/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` | the paradigm paragraph; the family; the Austen operator, sphere, anisotropic tie, helix, persona, dialogue, quest, byte deadband and Liskov figures; the list of things not done |
| `/home/claude/phiano/src/relational.rs` | `RelationalPhasor`, `RelationVocab`, `RelationalFacet`; the constants `OFFSET_CHANNELS`, `WINDOW`, `RATE`, `CHANNEL_DISPERSION_FLOOR`, `GUARD_MIN_VOCAB`, `MARKER_MAX_GAP`; the `bin/dimensions`, `bin/constructed` and `bin/relational` results; the 35.9% first run and the balance fix |
| `/home/claude/phiano/src/relational/relation_vocab.rs` | `is_empty`, `unclaimed`, `entries`, `from_entries`, `learn`, `marker`, `channel`, `channel_of`, `markers`, `name`; the marker rankings; the `dog`, `donut`, `president`, `states` and `bread of wheat` examples; the linking counts for `a`, `of` and `the` |
| `/home/claude/phiano/docs/64_promoted_lexicon.md` | the promotion into `SpectralPhasor`, `Facet`, `Trainer` and `.chroma` v4; the similarity, composition, perplexity and ownership tables; the collapse and repulsion figures; the polygons for `the`, `about` and `access`; the open items |
| `/home/claude/phiano/src/trainer/trainer.rs` | `Trainer::new`, `with_rounding`, `without_relations`, `couple_relations`, `update_channels`, `ensure_relations`, `with_seed`, `with_definitions`, `attraction_only`; the directed step, the rare-target boost, the repulsion term, the `sync_phase` on channel 0 |

The docs named in section 14 other than docs/64 were not read in full for this paper. Their titles were read to name them correctly. Their figures, where cited, come from PHIENT_FACTS.md.

Where two sources disagreed, the disagreement is stated in the text and the later document is preferred. The one disagreement found is the shipped dense similarity: 61.5% in `relational.rs` and 51.7% in docs/64. docs/64 is preferred.

Values of `RELATION_WINDOW`, `RELATION_RATE`, `NEG_RATE`, `RARE_TARGET_FLOOR`, `RARE_TARGET_BOOST` and `CHANNELS_PER_UPDATE` are defined outside the files read. They are named in this paper and their values are not given, with two exceptions. docs/64 states that the ranking objective moves sixteen channels per token, and PHIENT_FACTS.md gives the relation nudge as 0.06 · sin d. `PHASE_CHANNELS` is 64 throughout the sources, and `MARKER_CHANNELS_MAX = 24` is stated in docs/64 §5.

The three code defects in section 13 were found by reading `trainer.rs` and `relation_vocab.rs`. They are readings of the code and have not been measured.

The channel numbers used in the worked examples, `the` on 8, `of` on 10, `a` on 13, follow from the ranking printed by `bin/markers` and the assignment rule in `learn`. They hold only if that ranking holds on the training corpus. The sources do not print the shipped table.
