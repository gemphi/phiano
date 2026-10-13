# The Paradigm
*Phient papers, 1 of 16 · 2026-01-12 · The paradigm: X ⇒ Y — a word is nothing but what it relates to, under what conditions*

"In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition."

## 1. Abstract

This paper states the paradigm the other fifteen papers rest on, and checks it against what has been measured. The paradigm says a word is nothing but what it relates to, under what conditions. In code, a word is a `SpectralPhasor` with 64 channels. Each channel is one relation. On each channel the word keeps two angles, one for when it is the source of the relation and one for when it is the target, and a count of how often the relation has been observed. There is no vector for the word as a whole.

Channels 0 to 7 are the offsets +1 to +8. Channels 8 to 31 are marker words learned from the corpus, up to `MARKER_CHANNELS_MAX = 24`. Channels 32 to 63 are left to whatever training objective claims them. Hence 32–64 relations. A condition is what a sentence puts between two words. `RelationVocab::channel(tokens, i, j)` turns that condition into a channel.

Learning moves sides. When a sentence puts a before b, the source side of a and the target side of b on the selected channel move toward each other. A third word, drawn in proportion to its count to the power 0.75, is pushed away on the same channel. Reading compares sides. `on(a, b, k)` is the cosine of the source angle of a against the target angle of b on channel k, and the evidence weight is ln(1 + n).

The measurements that bear on the paradigm directly are few and specific. Read directed, with a context's source sides against a candidate's target sides, the lexicon alone ranks the next word on WikiText-2 at 15.5× chance with no learned operator. The Fourier operator on top reaches 31.9×. On Austen, a stand-in word chosen relationally reads 237.9 under a Kneser-Ney judge. The most similar word reads 253.9 and a random pool word reads 254.3. A collapse of the hundred most frequent words onto one angle, dispersion 0.003, was found, explained and repaired by giving each relation a direction. After the repair the head holds at 0.391 on the source side and 0.562 on the target side.

The paradigm also has measured limits. On the tail of the vocabulary the two-sided lexicon loses about ten points of perplexity. On a small corpus the phase term adds a sixth of a point over n-gram tables. One byte per side sets a floor under how finely a relation can be learned. Sections 13 and 15 state what is wrong and what remains.

## 2. The claim

Phiano 1 §1 states the claim formally. Let R be a finite set of relations, with |R| = 64, and C a set of conditions. A word w is fully given by a map

```
w :  R × C  →  S¹ × S¹ × N,        w(k, c) = ( src_w[k], tgt_w[k], n_w[k] )
```

two angles and an evidence count for each relation k, where the condition c is what selects k. Nothing else is stored.

Three consequences follow, and each one can be checked in the code.

The first is that similarity is agreement of sides. Two words are alike to the extent that they turn the same angles toward the same relations, in both roles, weighted by how much each has been seen under them:

```
sim(a, b) = Σ_k w_k ev_a(k) ev_b(k) · ½[cos(src_a[k] − src_b[k]) + cos(tgt_a[k] − tgt_b[k])]
            / Σ_k w_k ev_a(k) ev_b(k)
```

The second is that learning is movement. Nothing is assigned to a word. When a sentence places a before b under a condition that selects k, the source side of a and the target side of b move toward each other by `sin(tgt_b[k] − src_a[k])`. A word is only ever moved.

The third is that silence is different from disagreement. A pair with no evidence on a channel reads exactly 0.000. A pair with evidence that disagrees reads negative. Section 7 shows both cases in real output.

The arrow in X ⇒ Y has a direction. A relation is "a relates to b", and so each side of the polygon is two angles. That split was not in the first design. Section 9 shows that a measurement forced it.

The claim sits in a known line. Firth (1957) wrote that you shall know a word by the company it keeps, and Harris (1954) stated the distributional hypothesis. Phiano keeps the company structured, by relation and by condition, instead of pooling it into one vector. Word2vec (Mikolov et al. 2013) supplies negative sampling, the count^0.75 distribution and the two-vector asymmetry. ComplEx (Trouillon et al. 2016) reads asymmetric relations by the same conjugation `on` uses.

The geometry is closed on purpose. The circle is the smallest closed space with a continuous symmetry, and the complex unit circle adds a multiplication. On it, binding is rotation and unbinding its inverse. A word is a point of the 64-torus (S¹)^64. Phinet 2 §1.1 closes each factor into a Riemann sphere so that evidence has a place too, as latitude.

The claim can fail. If a point-like reading did as well as a relational one on a task that depends on conditions, the paradigm would add nothing. Section 10 gives that test.

## 3. The mechanism

The representation is `SpectralPhasor` in `src/phasor.rs`. Its stored fields are these.

| field | type | meaning |
|---|---|---|
| `phase[k]` | u8 × 64, packed `[u64; 8]` | source side: the angle on channel k when this word is the one relating |
| `tgt[k]` | u8 × 64, packed `[u64; 8]` | target side: the angle when this word is the one related to (format v5) |
| `evid[k]` | u32 × 64, packed `[u64; 32]` | times relation k has been observed for this word (format v6; was u16) |
| `amplitude`, `band_n`, `count` | f64, u32, u32 | legacy salience, Phical's sub-band, token count |

An angle is one byte. That is 256 quanta of 2π, each 2π/256 ≈ 0.0245 rad.

The read path has one non-linearity, `evidence(k) = ln(1 + evid[k])`. The primitive is `on(a, b, k) = cos(src_a[k] − tgt_b[k])`. It compares one side and it is directed: "a then b". `similarity_with` compares source with source and target with target. `shared_relation_with` returns the channel on which two words agree most, weighted by evidence. `bind(a, r)[k] = a[k] + r[k]` and `unbind` is its inverse. Both are exact on the circle. A sum of several bound terms is not exact.

The conditions are in `RelationVocab`, in `src/relational.rs`. It fixes channels 0 to 7 as offsets and learns the markers as the most frequent words that stand between pairs in the training corpus, up to 24. `RelationVocab::channel(tokens, i, j)` returns the channel for the pair at positions i and j. The rule is the rarest marker word standing between the two, else the offset.

The learning rule is `couple_relations` in `src/trainer/mod.rs`. It is a Kuramoto step restricted to the channel a sentence exercises, with a repulsion on the same channel. The negative is drawn in proportion to count^0.75. `Trainer::update_channels` is the ranking objective, and it may only move the channels `unclaimed()` returns. Before that rule the ranking gradient overwrote the marker channels and composition fell to null.

The reading used in generation is `Facet::relational_score(ctx, cand)`. Every context word is read against the candidate on the channel the condition between them selects, source side against target side, timed by its own evidence. Nothing is averaged into a context vector first. A pair without evidence is silent.

The substitution reading is `Facet::relational_fit(toks, i, cand)`. It reads a candidate at position i against every neighbour in the window, each pair under its own condition, on the side the direction demands. The goal reading is `calls_for(cand, goal)`, used by `Generator::with_goal(word, weight)`.

The circle is also read at 64 points. `SECTOR_RESOLUTION = 64`, `Hexagram::from_phase` and `Phinum64` share that lattice, which Phiano 1 §2.3 calls Q64.

## 4. What was measured

Every number below comes from Phiano 1, Phinet 2 or the Phient facts sheet. All are single seed. Phiano 1 §5 asks that differences under about 2 % be treated as noise.

| measurement | corpus | result |
|---|---|---|
| head dispersion, top 100 words, one side, 2 passes | Rust Book | 0.003 |
| head dispersion, two sides + count^0.75 negatives, 2 passes | Rust Book | 0.391 src / 0.562 tgt |
| balanced similarity, pair against random | Rust Book | 51.8 % (control 44.5 %) |
| composition a:b::c:?, antonym / comparative families | Rust Book | 9.7× / 8.2× null |
| composition, pair's own channel | Rust Book | 0.9× one side, 2.1× two sides |
| next word, identity reader, directed | WikiText-2 | 15.5× chance |
| next word, Fourier operator, directed | WikiText-2 | 31.9× chance |
| next word, identity | Austen | 7.9× chance |
| next word, hinge, 8,360 params | Austen | 14.7× valid / 14.6× test |
| next word, softmax wide+deep, 50,016 params, 8 epochs | Austen | 30.7× / 30.3×, hit@10 0.348 |
| substitution, KN perplexity: written / fit / similar / random | Austen | 173.0 / 237.9 / 253.9 / 254.3 |
| quest, attraction-only + dithered rounding, chance 0.021 | phiecs oracle | 0.594 MRR |

The WikiText-2 pool is about 2,240 words, chance MRR 0.0037. The Austen pool is 1,228 words seen 20 or more times, chance MRR 0.0063.

Three results carry the paradigm. The first is the collapse and its repair. The second is the directed identity reader. It has no learned parameters and reaches 15.5× chance. Read with only source sides, the same lexicon gives 4.7×, and the one-sided lexicon gave 6.7×. The third is substitution, where only the relational stand-in is separated from a random word.

Some numbers go the other way. On the Rust Book perplexity harness the two-sided lexicon reads 142.8 directed with the mixture-likelihood operator, where the one-sided lexicon read 134.2 and Kneser-Ney reads 132.1. Mixed with the n-gram tables, everything reads 125.5 against 125.6 for a unigram under the trigram. Decoding on Austen, 40 held-out prompts of 30 tokens, gives Kneser-Ney perplexity 13.3 for the trigram sampler and 66.9 for operator nearest-k. The operator ranks well. It does not yet write well.

The operator numbers in Phinet 2 §3 to §5 were measured on the one-sided lexicon and are not re-run on format v5.

## 5. The one idea, restated

The paradigm has one idea. A word is a set of sides. A condition picks a side. Two words move only when a sentence puts them under a condition, and everything the model knows is the history of those movements.

Each part of that sentence has a counterpart in code. "A set of sides" is the 64 channels of `SpectralPhasor`, each with a source angle, a target angle and a count. "A condition picks a side" is `RelationVocab::channel(tokens, i, j)`. "Two words move" is `couple_relations`. "The history of those movements" is the angles where they stopped and the counts of how often they were moved.

The idea excludes three things, and the exclusions are what make it testable.

It excludes a point. No field of `SpectralPhasor` holds a location for the word. The legacy `amplitude` is a salience, and the learned scorer in Phiano 1 §7.4 gives the old single-phase reading a weight of +0.02 and the flow's collective phase +0.03. Fitted by convex descent, the machine found no use for them. It gave the relational reading +0.70 and the count of evidenced pairs +0.50.

It excludes averaging first. `relational_score` reads each context word against the candidate pair by pair, each under its own condition. Phiano 1 §7.3 calls this the Difference Engine reading, after Babbage. No wheel knows the sum. The sum is the state of all the wheels after the turn.

It excludes treating an unseen relation as a neutral one. The count is part of the relation. When a pair has never been observed on a channel, `on` returns an angle that was never moved and the evidence weight is zero. The lexicon can say "these disagree" and "I have never seen this" as two different things.

The idea also sets a budget. A word has 64 relations. A composition of words can carry only the few relations each word has evidence for. Phiano 1 §6.1 measured this. Bound to all of its qualifying conditions, 19.9 per word on average, a word could not be recovered from a superposition on the 64-sample circle. Bound to its four best-evidenced conditions, it could. The constant is `MAX_FILLERS = 4`.

Finally, the idea says where meaning is. It is in no single word. It is a side effect of pairs moving. That is why a word's similarity to another can come out negative when they fill the same slots and never present the same sides. *borrow* and *ownership* read −0.195 in the two-sided reading, where the old one-angle resonance read +0.078. That is what the paradigm predicts.

## 6. One sentence under its conditions

The plainest view of the paradigm is one sentence traced through a trained lexicon. `cargo run --release --bin trace` runs `src/bin/trace.rs`, which trains the lexicon on the Rust Book and prints what it knows about a sentence. The trace below is from Phiano 1 §3.1, on the one-sided (v4) lexicon. The sentence is "the borrow checker ensures that references are always valid". For every pair, the trace prints the channel, the agreement on it, and each word's evidence there.

```
the        -> borrow     offset+1   cos +0.964   evidence 35336 / 182
the        -> checker    offset+2   cos +0.933   evidence 23950 / 52
the        -> ensures    offset+3   cos -0.803   evidence 13052 / 10
the        -> that       offset+4   cos +0.981   evidence 23546 / 6704
borrow     -> checker    offset+1   cos +0.999   evidence   182 / 66
borrow     -> ensures    offset+2   cos -0.882   evidence    88 / 24
borrow     -> references offset+4   cos +1.000   evidence   142 / 360
checker    -> references that       cos +0.098   evidence     8 / 40
ensures    -> that       offset+1   cos +1.000   evidence    80 / 9068
ensures    -> references that       cos +0.672   evidence    76 / 40
ensures    -> are        that       cos +0.858   evidence    76 / 154
that       -> references offset+1   cos +0.989   evidence  9068 / 494
references -> are        offset+1   cos +0.999   evidence   494 / 2044
are        -> valid      offset+2   cos +1.000   evidence  1624 / 188
always     -> valid      offset+1   cos +0.999   evidence   216 / 380
```

Read this as the paradigm in numbers. The pair *checker → references* is three words apart, and the word *that* stands between them. So the condition is "that", and the channel is the "that" marker, channel 13 in the Rust Book table. On that channel the two words barely agree, at +0.098. The reason is in the evidence column. *checker* has been seen under "that" only eight times.

The pair *ensures → references* is read under the same condition and agrees at +0.672. *ensures* has 76 observations there. The same condition, the same target word and a different source word give a different reading, and the difference follows the evidence.

The pair *the → ensures* on offset+3 reads −0.803. The lexicon has learned that *ensures* does not sit three words after *the*. That is a negative side, learned by the repulsion term, with only ten observations behind it on the side of *ensures*. The frequent word *the* has 13,052 observations on the same channel.

Nothing in this trace is a word's location. Each line is two words, one condition, one cosine and two counts. That is the paradigm's level 2, a sentence as a path of conditions, in the ladder Phiano 1 §1.2 gives: word, phrase, sentence, story, concept.

## 7. One word as a polygon

The same trace prints a word's polygon: every relation it has evidence on, with the count and the angle. The format is `relation:count@angle`. These four are from Phiano 1 §3.2, on the one-sided lexicon.

```
borrow     dominant offset+1 (ln-evidence 5.21)
           offset+1:182@0.71  offset+2:88@4.76  offset+3:42@3.51  offset+4:142@4.00
           offset+5:118@3.31  offset+6:108@4.12  the:66@4.42  to:8@1.57  a:20@3.95  in:6@4.34
reference  dominant offset+1 (ln-evidence 6.79)
           offset+1:888@0.86  offset+2:290@4.71  offset+3:92@3.46  offset+4:660@3.98
           the:140@5.40  to:292@1.89  a:356@2.01  in:46@2.77
the        offset+1:35336@0.98  offset+2:23950@4.81  offset+3:13052@3.44  to:2558@1.96  in:2774@2.85
function   offset+1:2590@0.74   offset+2:1362@4.84   offset+3:406@3.36   the:824@5.33  a:374@1.82
```

*borrow* and *reference* agree on the "to" side, 1.57 against 1.89. On the "a" side they differ, 3.95 against 2.01. Their evidence-weighted similarity is +0.250, against +0.244 for the old one-angle resonance. Two other pairs move further. *borrow* and *ownership* come out at −0.195, where the old reading gave +0.078. *mutable* and *immutable* come out at −0.115, where the old reading gave +0.186. Words that occupy the same slots and never present the same sides now read as opposed.

The polygon also shows a defect. Look down the offset columns. Every one of these four words sits at about 0.8 on offset+1, about 4.8 on offset+2, about 3.4 on offset+3 and about 4.0 on offset+4. Their offset polygons are the same polygon. That is the collapse of Section 9, visible in four lines before it was measured by tier.

A word the lexicon has never been trained on in a sentence shows the third consequence of the claim. Phiano 1 §3.3 traces a held-out sentence, "RefCell<T> in Chapter 15 to allow us to mutate".

```
refcellt  -> in       offset+1  cos +1.000     chapter -> allow   to        cos -0.831
refcellt  -> chapter  in        cos -0.985     15      -> allow   to        cos  0.000
refcellt  -> 15       in        cos +0.989     to      -> us      offset+2  cos +1.000
```

The pair *15 → allow* under "to" reads exactly 0.000. The word *15* has no evidence on the "to" channel, and `on` reports an angle that was never moved. The pair *chapter → allow* under the same condition reads −0.831. Both lines are under the same condition. One is a disagreement and one is an absence, and the output keeps them apart.

The polygon is the paradigm's level 0, the word. It is drawn as 64 sides, with each side's length ln(1 + n). Channels 32 to 63 have no evidence on the Rust Book and dispersion 1.0. They are the unclaimed half, and the ranking objective moves them. A word with evidence on all 32 corpus channels and on some of the free ones is a polygon of between 32 and 64 sides. That is the 32–64 of the paradigm paragraph.

## 8. Conditions, in depth

A condition sets which side of a word is in play. In the current lexicon it is a pair of positions and the marker words between them. The channel table on the Rust Book (10,315 training sentences, 6,165 words, trained in 1.8 s, two passes) shows what the corpus chose.

| ch | relation | words with evidence | ch | relation | words with evidence |
|---|---|---|---|---|---|
| 0 | offset+1 | 6,165 | 8 | the | 2,754 |
| 3 | offset+4 | 6,081 | 9 | to | 1,831 |
| 7 | offset+8 | 5,052 | 13 | that | 1,411 |
| 16 | is | 1,321 | 19 | code | 657 |
| 26 | value | 520 | 28 | type | 494 |

The markers are the corpus's own. *code*, *value*, *type* and *use* are markers because this is the Rust Book. On WikiText-2 they are not. A condition therefore means something only relative to a lexicon's marker table, and that is why a persona built on one lexicon does not carry to another.

The code once had two definitions of the condition between two words. The composition bench keyed a pair by its offset. The trainer keyed it by `RelationVocab::channel`. Phiano 1 §6.1 records the repair. `Composition` now takes a `PairKey`, either `Offset` or `Channel(&RelationVocab)`, and the test `pair_key_is_one_definition` pins the composition's key to the trainer's.

The repair was measured with `bin/pair_key` on 704 composable words, chance MRR 0.0101. With the channel key and every qualifying filler, 19.9 per word, recovery at levels 1 to 4 fell to 6×, 5×, 2× and 1×. With the four best-evidenced channels per word it read 52×, 36×, 13× and 8× by phase subtraction, and 81×, 66×, 57× and 3× after a cleanup operator. The offset key read 59×, 19×, 8× and 3×, and 80×, 61×, 31× and 1× after cleanup. The trainer's key is a little worse at level 1 and much better at depth. A pair read under its actual condition survives superposition longer than a pair read under its distance.

The second source of conditions is Searle. A speech act has conditions of satisfaction. Phinum's `SearleClassifier`, in `src/phinum/searle.rs`, reads a sentence's syntax key into one of five classes, assertive, directive, commissive, expressive and declaration, and then into 16, 32 or 64 sub-variations on the same 64-point lattice. "Close the door" and "the door is closed" share every word relation on the offset channels. They differ in the act. The act is the condition under which the words' sides are read. No experiment has yet used the Searle class as a channel key. That is not built.

## 9. Direction, forced by a measurement

The first lexicon kept one angle per word per channel. `src/bin/trace.rs` also prints circular dispersion by frequency tier, where dispersion is 1 − |mean resultant| over the words with evidence, 1 is uniform and 0 is one angle. The global figure on offset+1 was 0.928 and looked healthy. By tier it was not.

| words by frequency | one side, 2 passes | one side, 4 passes | two sides + count^0.75 negatives, src / tgt, 2 passes | same, 4 passes |
|---|---|---|---|---|
| top 100 | 0.003 | 0.003 | 0.391 / 0.562 | 0.383 / 0.541 |
| 100–500 | 0.026 | 0.016 | 0.329 / 0.917 | 0.275 / 0.873 |
| 500–2,000 | 0.723 | 0.609 | 0.787 / 0.940 | 0.634 / 0.906 |
| 2,000–6,165 | 0.890 | 0.770 | 0.966 / 0.963 | 0.935 / 0.905 |

The hundred most frequent words sat at one angle on offset+1, and the collapse spread down the ranking with passes.

Phiano 1 §4 gives the reason. With one phase per word, "a then b" and "b then a" are the same constraint. Attraction on every adjacent pair is then a Kuramoto system on the connected co-occurrence graph, whose fixed point is total synchronisation. The frequent words are adjacent to everything and reach it first. Repulsion only slows it.

Two repairs failed alone. Negatives drawn in proportion to count^0.75 with a symmetric push moved the head from 0.003 to 0.004. The source and target split with the old capped negative pool moved it to 0.082 on the source side and 0.072 on the target side, still sliding with passes. Both together held it at 0.4 to 0.6 across passes. That is format v5, with `tgt: [u64; 8]` next to `phase`, and a negative pool drawn in proportion to count^0.75 with no cap. The similarity of *the* and *a* fell from +0.77 to +0.40, which is the collapse leaving.

The arrow in X ⇒ Y is this result. A relation has a source and a target, and a lexicon that stores it without direction cannot hold the frequent words apart. The analogy is now read directed, `tgt_d = src_c + (tgt_b − src_a)`.

The direction also changed how a context must be read. On WikiText-2, reading only the source sides of the two-sided lexicon gave 4.7× chance with the identity reader. The one-sided lexicon had given 6.7×. Reading a context's source sides against each candidate's target sides gave 15.5×. Any code that compares a context to a word's source side is now a bug. The log is `docs/69_two_sides.md`, and the reading rule is in `docs/70_issues_closed.md`.

## 10. Reading relationally, and the substitution test

The paradigm makes a prediction that a point-like model does not. If a word is what it relates to under conditions, then a word b may stand in for a word a wherever every relation the context demands of a still holds of b. Phiano 1 §7.5 names this after Barbara Liskov's substitution principle and measures it on the Austen voice lexicon with `bin/liskov`.

The test puts the best stand-in into her sentence, never the original, and asks a Kneser-Ney model of Austen how much it minds: geometric mean perplexity over 200 sentences.

| sentence | perplexity |
|---|---|
| as written | 173.0 |
| `relational_fit` stand-in | 237.9 |
| most similar word, by `similarity` | 253.9 |
| random pool word | 254.3 |

The similarity reading asks whether two words present the same sides. It is point-like, and it is indistinguishable from a random word here. The relational reading asks whether the candidate holds the relations its neighbours demand, each under its own condition. It is the only one of the three the judge separates from chance. Some of the stand-ins read as follows: *time* becomes *benwick* by fit and *impossible* by similarity; *must* becomes *feel* by fit and *shall* by similarity.

On cloze the relational reading alone is weaker than frequency. With a pool of 622 words seen 40 or more times and 200 held-out positions, chance MRR 0.0113, the most frequent word reads 5.8× chance. `relational_score` with left context only reads 2.0×. `relational_fit` with both sides reads 4.1×, and multiplied by ln(1 + pairs) it reads 4.7×. The learned scorer's own trade between count and relation, with no weights refit, reads 6.1×. The count and the relation together beat either alone.

The goal reading pulls generation toward a word the sentence reaches later. Over 40 held-out prompts and 30 tokens, no pull reaches the goal in 2 of 40, a pull of 2 in 6 of 40, and a pull of 4 in 5 of 40 in 8.8 tokens, at a perplexity cost of 12.4 to 13.3. The pull works and is small.

The facts sheet adds three operators in `phinet::liskov`: `better`, `worse` and `loss`, with `forcing_choice`. On decoding with k=32, nearest-k reads Kneser-Ney 66.9, forcing at λ=0.05 reads 69.7, at λ=0.25 68.5, and at λ=1 107.4. Forcing does not beat greedy on text. Scored by the same operators, the quest's 256 words occupy 61 of 64 hexagrams, with mean better −0.008, worse −0.008 and loss 0.108 at margin 0.1. The facts sheet reads this as flat, and puts it down to the byte ceiling. The log is `docs/86`.

## 11. Place in the PML family

PML stands for Phi machine learning. The paradigm is the part of it every member shares. What differs is which geometry each member puts the relations on, and what it reads them for.

phinet is the parent engine. It holds a complex `Tensor [b, w, s]`, a `Tape` for reverse-mode autograd, a unitary FFT, `Fno`, joint training of the operator with its table, holographic bind and unbind in `hrr`, 64 hexagram gates and the sphere. It passes 62 of 62 parity tests against a PyTorch reference, and `phinet::sphere` agrees with scipy's `sph_harm_y` to a maximum difference of 2.4e-15. The paradigm depends on phinet for everything learned above the identity reader. The 15.5× of the identity reader needs no phinet. The 31.9× does.

phical is the geometry of the relation circle. It supplies `phi_cal::mobius`, `phi_cal::sphere::{stereo, unstereo, lift}` and `phi_cal::winding`, and `Phical::quantum`, which clamps `band_n` to 13. When the paradigm says a relation lives on a closed circle, and on a sphere once evidence is latitude, phical is where that is written down.

phifno 1.0.0 is the spherical operator. The sphere tie, `phinet::Tie::Sphere`, gives one complex weight per spherical-harmonic degree over 64 = 8² channels read as Y_l^m with l < 8. It has 1,192 parameters and reads 12.8× valid and 13.1× test on Austen with hinge at 3 epochs, against the circle's 14.7× and 14.6× at 8,360 parameters. `Tie::Anisotropic`, one weight per (l, |m|) over 36 slots and 4,776 parameters, reads 16.0× and 15.6×. It beats the circle with fewer parameters.

phiton is light as units: `LightQuantum`, `SpectralBand`, `PhitonColor`. The helix gives each codon a PhiTon wavelength, band and hexagram. phinum carries `SearleClassifier` and `Phinum64`, the conditions from speech acts and the 64-point lattice. phiqlm is planned. Nothing of it is built.

phiano is the word application. It holds `Facet`, `Trainer`, generation, personas, quest ingest and the helix. The helix writes the lexicon as a DNA-coded tree: 64 codons as base-4 ACGT of the channel, with hexagram, sphere degree and order, and B-DNA coordinates at 10.5 codons per turn and 0.34 nm rise. It writes 18,496 words in 1.9 s, 75 MB, with a byte-exact round trip. Everything in Sections 6 to 10 is phiano.

phiecs is the entity-component world and the quest. Its synthetic oracle is where the paradigm is tested on typed relations: is-a, part-of, near, used-for and unlike.

phiace is named in the family. The sources read for this paper mention it once, as the `@phiace/puijs` package behind the PhiDoc pages for phinet. They say nothing more, and this paper makes no claim about it.

## 12. The next experiment

The paradigm's sharpest measured support is one test on one corpus. The relational stand-in beat the most similar word and a random word on Austen, and the point-like reading was at chance. The next experiment is to run the same test where the conditions are different, and to add a control that removes the conditions while keeping everything else.

The corpus is the Rust Book, `data/rust_book_corpus.txt`, on its 80/10/10 split with seed 42. It has a channel table of its own, with *code*, *value*, *type* and *use* among the markers, so the conditions a stand-in must satisfy there are not the ones Austen set. The harness is `bin/liskov` as it stands. It already produces the three readings: `relational_fit`, `similarity` and a random pool word, each judged by a Kneser-Ney model of the same corpus.

The control is the relations-off lexicon. Phiano 1 §5.1 already measures one, as the "control (relations off)" column, where balanced similarity reads 44.5 % against 51.8 % for the two-sided lexicon. The same trainer on the same sentences with relations off has no marker channels and no two sides to read. Its `relational_fit` stand-in should read like its similarity stand-in if the conditions are what carry the result. If the relations-off stand-in reads as well as the relational one, the paradigm's claim to substitution is not supported on this corpus.

The prediction is written down before the run. On the Rust Book the two-sided `relational_fit` stand-in reads lower perplexity than both the similarity stand-in and the random word, and the relations-off `relational_fit` stand-in does not. The measure is the geometric mean over 200 held-out sentences, as on Austen. A difference under about 2 % is noise, per Phiano 1 §5.

Two further readings come from the same run at no extra cost. The first is cloze on the Rust Book pool, with the frequency baseline and the learned trade from `learned.json`. On Austen the trade read 6.1× against 5.8× for frequency. The second is the same substitution with only offset channels kept, `PairKey::Offset` in effect. That separates the part of the result due to distance from the part due to the marker words between.

This experiment has not been run. The relations-off lexicon exists as the control column of Phiano 1 §5.1. `bin/liskov` has not been pointed at it, and substitution has not been measured on the Rust Book.

A second experiment waits behind this one. The operator numbers in Phinet 2 §3 to §5 were measured on the one-sided lexicon. Re-running them on format v5 is listed as open in both papers, with the one-sided numbers as the control. It is not run.

## 13. What is wrong or missing

The perplexity tail is worse on the two-sided lexicon. On the Rust Book harness the mixture-likelihood operator reads 142.8 directed and 144.1 source-only, where the one-sided lexicon read 134.2. A rare word's target side moves only when that word is the later word of a pair, and perplexity is a sum over exactly those words. Tying the target side to the source side below 16 observations made every number worse, 146.0 with the mixture. It ships disabled.

The phase term is redundant with n-gram tables on a small corpus. Mixed with the tables everything reads 125.5 against 125.6 for the unigram under the trigram. Phinet 2 gives that corpus as 6,468 sentences. Phiano 1 §5 gives the Rust Book as 12,894 sentences. The sources do not reconcile the two counts, and this paper does not try.

The operator ranks and does not yet write. Decoding on Austen gives the trigram sampler 13.3 and operator nearest-k 66.9, and shared 4-grams across prompts are 25.7 % for the sampler and 8.7 % for the operator. Proposals from the operator ship off.

A byte is a floor. A channel angle is 256 quanta of 2π. The nudge 0.06·sin d falls under half a quantum once sin d < 0.2, about 11°, and nearest rounding froze every pair 8 quanta short in the quest. The shipped trainer reads 0.082 MRR on the quest. Attraction-only coupling with dithered rounding reads 0.594. It costs corpus attribution 71 % → 65 %, so it ships off by default. The three Liskov operators on the quest read flat for the same reason.

Typed relations do not yet generalise. On the quest's held-out 20 %, one hop reads 0.011 MRR against chance 0.021. Two hops read 0.095 for is-a and 0.093 for used-for, and part-of, near and unlike are at about chance. The wrong channel reads 0.015 and the offset reading 0.005.

Superposition is short. A word bound to all of its conditions cannot be recovered on the 64-sample circle, and composition recovery halves per level.

The hexagram of the next word cannot be predicted. The operator's predicted hexagram matches 0.9 % of the time, under chance at 1.6 %.

One document in the sources describes an older model. `MASTER_CONNECTIONS.md` gives the phasor as `Z = A·exp(i(φ + nα))`, one phase per word, with a 16-sector voice fingerprint and a 16-layer memory. That single-phase model is the one Section 9 shows collapses. Phiano 1, revision 7, is later and is preferred here.

Every result is single seed. There is no Phi-4 run. The quest is a synthetic oracle only. The sphere at softmax and wide+deep is unrun.

## 14. How it was found

The history is in the measurement logs, `docs/64` to `docs/73`, and later `docs/86`, with Phiano 1 as the running summary at revision 7.

The oldest source read here is `MASTER_CONNECTIONS.md`. It maps the system before the relation channels: a tokenizer hashing words into `src/facet.rs`, a `SpectralPhasor` with one phase, amplitude and band, a Kuramoto update `φ_k ← φ_k + K sin(ψ − φ_k)` toward a sentence's mean phase, and a 16-layer memory in four octave bands. The paradigm is already there in the update rule. Words move toward each other. It is not yet there in the representation. A word still has one angle.

`docs/64` is the first log Phiano 1 cites. Its first draft quoted balanced similarity 51.7 % → 55.8 % and composition 3.0× from it. Phiano 1 §5 records that those were an earlier code state and should not be cited. The numbers that reproduce on the current code are 44.5 % for the control and 51.8 % with relations on.
Phinet 2 records the operator work that followed, on the one-sided lexicon. The hinge objective took ranking from 11.6× to 27.9× chance. Frequency-proportional decoys were tried and refuted, 149.3 → 153.6 → 175.3 perplexity. The mixture likelihood took perplexity from 149.3 to 134.2. The residual result, a sixth of a point over the tables, closed the question on that corpus.

`docs/69` is the two-sided fix. The trace in Section 7 had shown four words with one offset polygon. Dispersion by tier showed 0.003 in the head. Two partial repairs were tried and logged as failures before both were combined. The directed reading of a relation is from this log.

`docs/70` closes the issues that followed, among them the rule that a context must be read against target sides, the Q64 half-step, and the 16-bit evidence lanes that *the* had saturated. `docs/71` measures perplexity and generation on the two-sided lexicon and records the ten-point loss on the tail. `docs/72` records the proposal problem: proposals from the operator's average were dominated by frequent words, then by the rare tail. The relational reading closed it. `docs/73` is the learning machine, where a fitted scorer gave the old single-phase reading +0.02 and the relational reading +0.70.

Phiano 1 §6.1 and §7.5 record the pair key, the Lovelace reading of `relational_score` as a Difference Engine, and the Liskov readings of substitution and the end game. `docs/86` adds the three Liskov operators and scores the quest with them.

The pattern repeats. A reading looked healthy on average, a finer measurement showed where it was not, and the repair made the representation more relational.

## 15. What remains

The work that remains falls into three groups: measurements not yet re-run, experiments specified and not run, and parts not built.

Not re-run. The operator numbers of Phinet 2 §3 to §5 were measured on the one-sided lexicon and are not re-run on format v5. The held-out perplexity through `evaluate` on the two-sided lexicon is marked "not re-run" in Phiano 1 §5.1. WikiText-2 at the mixture objective is the experiment Phinet 2 §4 says would reopen the residual question. It is not run. The sphere tie at softmax and at wide+deep is unrun.

Specified and not run. Phinet 2 §6 lists Phase Operators. The Möbius layer, `Tape::mobius(x, abcd)`, is implemented and finite-difference checked. Training it on level-1 cleanup and reading the inverse role rotation off a/d is not run. The stereographic lift is implemented in exact form. The differentiable tape op is not written, and the check that the lifted identity reader reproduces the `similarity_with` ranking is not run. The zeta mode prior, `zeta_weights(modes, s)` with `Fno::scale_modes`, is implemented, and the sweep of s on the language task is not run. The winding number is implemented and tested, and its use as a feature is not measured. Cross-channel coupling `K_kl` is specified, with its test: offset-channel dispersion must stay at or above 0.85 and balanced similarity must not fall under 51.8 %. It is not built. The persona as a tethered copy of the table is specified and not measured.

Not built. phiqlm is planned and nothing of it is built. A Phi-4 run of the quest has not been done. The quest so far is a synthetic oracle only. The killer heuristic in the beam, from Phiano 1 §7.5, is not implemented. A Searle class used as a channel key is not built.

Open in the lexicon. Phiano 1 §9 lists more evidence per rare target side, through a wider window for rare words, and prior-corrected proposals with frequency-decayed pseudo-counts. It also lists the stub files `src/phinet/{tensor,operator}.rs`, which are on disk and should be deleted.

Open at the byte. The deadband sets how close two sides can be pulled. Dither lifts the quest and costs attribution, and it ships off. A way to cross the half-quantum floor without that cost is not found.

The paradigm itself is not in question in any of these items. Each one asks how far a word that is only its relations can be read, composed and written before the representation's budget runs out. The budget is known: 64 relations, a byte per side, four fillers per composition. What remains is to measure each reading against that budget on a second corpus, with a control that removes the conditions.

## 16. Sources

These are the local files read for this paper. No other file was used. Every number above is quoted from one of them. Where two of them disagree, the disagreement is stated in the section that uses the number, and the later document is preferred.

| path | what was taken from it |
|---|---|
| `/home/claude/phiano/docs/papers/phient/BRIEF.md` | the structure of the paper, the sixteen sections and their arc, and the rules of voice and citation |
| `/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` | the paradigm paragraph, quoted verbatim after the front matter; the PML family and what each member holds; the Austen operator numbers; the sphere and anisotropic ties; persona attribution; the quest oracle and its held-out numbers; the byte deadband; the helix; the three Liskov operators; the list of things not done |
| `/home/claude/phiano/docs/papers/11_PHIANO_1_PHASE_OSCILLATORS_ON_THE_RELATION_CIRCLE.md` | the formal claim; `SpectralPhasor` and its fields; `RelationVocab` and the Rust Book channel table; the sentence and word traces; the collapse by tier and its fix; similarity and composition on the Rust Book; WikiText-2 ranking by reading; perplexity on both lexicons; the pair key; Searle; the learned scorer's weights; `relational_score`, `relational_fit`, `calls_for`; the substitution, cloze and end-game tables; related work; the open list; the code map |
| `/home/claude/phiano/docs/papers/12_PHINET_2_PHASE_OPERATORS.md` | the closed-geometry argument; the Riemann sphere as the closed form of one channel; the phinet engine modules; the three objectives; the residual result and its corpus size; composition as cleanup; the Phase Operator proposals and which are implemented; the hexagram measurement; the open list |
| `/home/claude/phiano/docs/MASTER_CONNECTIONS.md` | the older system map with a single-phase `SpectralPhasor`, the Kuramoto update toward a sentence's mean phase, and the 16-layer memory, used in Sections 13 and 14 as the state before relation channels |

The measurement logs `docs/64` to `docs/73` and `docs/86` are cited by number because the sources above cite them. They were not read for this paper, and nothing is quoted from them directly.

Three statements in the sources were checked against each other and are recorded here so that later papers in the series do not repeat them as settled. The first is the corpus size behind the residual result: 6,468 sentences in Phinet 2 §4, and 12,894 sentences for the Rust Book in Phiano 1 §5. The second is the representation: one phase per word in `MASTER_CONNECTIONS.md`, and two sides per channel in Phiano 1, which is later. The third is the balanced similarity figure: 55.8 % in the first draft of Phiano 1, and 51.8 % on the current code, which Phiano 1 §5 states is the one that reproduces.

Nothing in this paper was measured for it.
