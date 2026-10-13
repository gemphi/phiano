# The Two Sides of a Word
*Phient papers, 3 of 16 · 2026-02-09 · Source and target: the two sides of a word, and why a directed reading needs them*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

A word in Phiano is a `SpectralPhasor`. It holds 64 relation channels, one byte of angle each, and a count per channel of how often the word was seen under that relation. Until format v5 a word had one angle per channel. That angle had two jobs. It was the angle the word showed when it came first in a pair, and the angle it showed when it came second. This paper is about what went wrong with that, and about the fix.

The defect was found by `bin/trace` on the Rust Book corpus. On the offset+1 channel the hundred most frequent words had dispersion 0.003 after two passes and after four. They all sat at one angle. The earlier report in docs/64 had called the channels healthy at 0.85–0.93, because it averaged over the whole vocabulary and the tail stayed spread.

The cause is a symmetry. With one angle, "a then b" and "b then a" are the same constraint. Attraction on every adjacent pair then drives the connected graph to one shared angle. The fix, in docs/69, gives each word a second array, `tgt`, next to `packed`. `packed` is the source side: the angle a word presents when it does the relating. `tgt` is the target side: the angle it presents when it is related to. Training moves the earlier word's source side and the later word's target side. The agreement of a pair on channel `k` becomes `cos(src_a[k] − tgt_b[k])`, which is directed.

The numbers that followed are mixed and all reported here. With the split and `count^0.75` negatives together, the head of the offset+1 channel holds at 0.391 source and 0.562 target after two passes. Composition on the pair's own channel went from 0.9× null to 2.1×. Next-word ranking on WikiText-2 with no operator at all, reading a source-side context against target-side words, reached 15.5× chance, against 6.7× for the best one-sided reading. Tail perplexity got worse: the mixture operator went from 134.16 to 142.79 on the Rust Book. Two fixes for the tail were built and measured. Tying a rare word's target side to its source side made everything worse. Boosting the rare target step three times made the operators worse and moved the mixture with the n-gram tables by a third of a point. Both ship disabled.

The paper names the types and functions, walks one sentence through the update, reads the tables in order, and lists what is still wrong.

## 2. The claim

The claim has three parts.

First, a relation between words is directed, and a representation that stores one angle per word per relation cannot hold a directed relation. On a channel with one angle per word, the agreement of `a` and `b` is `cos(θ_a − θ_b)`. Cosine is even. So `a` agrees with `b` exactly as much as `b` agrees with `a`. "The" before "borrow" and "borrow" before "the" are one fact to that representation. Language treats them as two facts. The Rust Book writes "the borrow checker" often, and the reverse order is a different and rarer fact.

Second, the symmetric representation does worse than lose direction. Under attraction it collapses. The update that pulls `θ_a` toward `θ_b` for every observed adjacent pair is a Kuramoto system on the co-occurrence graph. Its fixed point is total synchronisation. The most frequent words are adjacent to nearly everything, so they synchronise first. docs/69 measured this: top-100 dispersion 0.003 on offset+1. Drawing negatives by `count^0.75` and pushing both words moved the head from 0.003 to 0.004. docs/69's conclusion is that repulsion of any strength only slows it.

Third, two sides fix both problems, and a reader has to use them the right way round. With a source side and a target side, "a then b" moves `src_a` and `tgt_b`, and "b then c" moves `src_b` and `tgt_c`. The two updates share no variable. The reading that follows from this is the directed one: compare the earlier word's source side with the later word's target side. docs/70 tested four readings of the two-sided lexicon. The directed one led all four on every column of the ranking table. Reading only the source side of a two-sided lexicon was worse than reading the old one-sided lexicon: 4.7× chance against 6.7× on WikiText-2 for the identity. docs/70 calls any code that compares a context to a word's source side a bug from that point on.

The claim does not include perplexity. On the Rust Book the split made held-out perplexity over the whole vocabulary worse, and the two attempts to repair that were refuted. The claim also does not extend to generation quality. Generation was measured with the directed operator and it did nothing a trigram judge rewards. The split is a trade at this corpus size: better ranking and composition, worse tail perplexity. That sentence is docs/71's, and this paper keeps it.

What the claim rests on is small. It is one corpus for the collapse, the Rust Book, one seed, and two corpora for ranking. docs/72 states that differences under about 2% are noise. The sections below cite only numbers that clear that bar or say when one does not.

## 3. The mechanism

The type is `SpectralPhasor`, in `src/phasor/types.rs`. It has `phase`, `amplitude`, `band_n`, `count`, and three arrays. `packed: [u64; PACKED_WORDS]` holds the 64 source-side angles, one byte each, eight to a word. `tgt: [u64; PACKED_WORDS]` holds the 64 target-side angles in the same layout. `evid: [u64; EVIDENCE_WORDS]` holds one raw count per channel. A byte is 256 quanta of 2π (`CHANNEL_QUANTA = 256.0`).

Both constructors start the two sides equal. `SpectralPhasor::seeded(word, amplitude, band_n)` hashes the word with `fnv1a` and `mix64`, gives each channel its own hash stream spread by the golden angle, and then sets `p.tgt = p.packed`. So does `SpectralPhasor::new`. `from_packed` and `from_packed_with_evidence` read older files and also set `tgt: packed`. `from_packed_sides` is the v5 reader that takes both arrays. `sides_equal()` returns true while `tgt == packed`: a phasor never through a directed coupling.

The source side is read by `theta(k)` and written by `set_theta(k, θ)`. The target side has a stored reading and a served reading. `theta_target_raw(k)` returns the stored byte as an angle; it is what training moves. `theta_target(k)` goes through `target_byte(k)`, which returns the source byte when `evidence_count(k) < TARGET_TIE_FLOOR`. `TARGET_TIE_FLOOR` is 0, so the tie is off. `set_theta_target` quantises to the nearest byte and `nudge_target(k, δ)` rotates by `δ`.

The update is `Trainer::couple_relations` in `src/trainer/trainer.rs`. For each `i` and each `j` within `RELATION_WINDOW` after it, the channel is `facet.relations.channel(tokens, i, j)`. A learned marker takes precedence. Otherwise a pair at distance `d` of at most `OFFSET_CHANNELS = 8` goes to channel `d − 1`, so adjacent words land on channel 0, named offset+1. The code then reads `ta = a.theta(k)` and `tb = b.theta_target(k)`, sets `d = sin(tb − ta)`, and calls `nudge_by(k, rate·d)` on the earlier word and `nudge_target_by(k, −rate·d·boost)` on the later word. Both words `deposit(k, 1)`. The source side of the later word and the target side of the earlier word are not touched. With negatives on, one sampled word's target side is pushed away from `ta`, and the earlier word's source side is pushed away from it.

The readings are in `src/phasor/spectral_phasor_set_evidence_count.rs`. `on(other, k)` is `COS_BYTE[src_self − tgt_other]`, zero when either word has no evidence on `k`. `a.on(b, k)` means "a then b". `similarity_with(other, weights)` is an evidence-weighted inner product in which each channel contributes `0.5·(src + tgt)`: source against source, target against target.

`Facet::relational_fit(toks, i, cand)` in `src/facet/facet_relational_fit.rs` applies `on` in context. For a neighbour before `i` it reads `o.on(cp, k)`. For a neighbour after `i` it reads `cp.on(o, k)`. It returns the mean over evidenced pairs and their number. `Facet::calls_for(word, goal)` is the mean `a.on(g, k)` over the eight offset channels where both have evidence.

## 4. What was measured

Every number here is from docs/69 through docs/72. The corpus is `data/rust_book_corpus.txt` unless WikiText-2 is named. One seed throughout.

The collapse and the fix, offset+1 dispersion by frequency tier, from `bin/trace`:

```
words by frequency   one side (2)  one side (4)  src (2)  tgt (2)  src (4)  tgt (4)
top 100                 0.003         0.003       0.391    0.562    0.383    0.541
100–500                 0.026         0.016       0.329    0.917    0.275    0.873
500–2,000               0.723         0.609       0.787    0.940    0.634    0.906
2,000–6,165             0.890         0.770       0.966    0.963    0.935    0.905
```

The two-sided columns are with `count^0.75` negatives. The split alone with the capped pool gave 0.082 and 0.072 at the top. The negatives alone gave 0.004.

The same change seen through `bin/promoted`: control 44.5% before and after; evidence-weighted balanced 51.8% before and after; evidence-weighted raw 31.4% → 36.8%; best single relation 22.6% → 27.0%. Composition on the family channel fell from 4.1× to 3.4× null. On the pair's own channel it rose from 0.9× to 2.1×. Antonyms and comparatives on the family channel went from 1.9× and 2.6× to 9.7× and 8.2×. The similarity of `the` and `a` fell from +0.772 to +0.400.

Next-word ranking, held-out, MRR over chance (docs/70):

```
                   Rust Book (pool 778)            WikiText-2 (pool 2,240)
reading        identity pointwise operator    identity pointwise operator
one side (v4)      —        —        —           6.7x     18.9x    27.9x
source            2.0x     4.1x    13.1x         4.7x     12.2x    26.5x
target            3.2x     3.2x    12.4x         5.5x      8.7x    29.0x
both (128)        2.6x     3.7x     9.8x         5.5x     10.7x    27.9x
directed          4.5x     5.0x    13.6x        15.5x     18.2x    31.9x
```

Held-out perplexity on the Rust Book, 3 epochs, Kneser-Ney 132.07 (docs/71):

```
reading               identity   hinge    mixture   best mix
one side, v4           156.33    149.30   134.16    125.56 @ 0.1
source sides, v6       155.77    157.54   144.14    125.50 @ 0.1
directed, v6           158.48    153.36   142.79    125.52 @ 0.1
```

Tie below 16 observations, directed: 160.92, 160.04, 145.95. Rare target ×3, directed: 158.47, 157.21, 145.76, best mix 125.25 against 125.52.

Generation, 60 held-out prompts, judged by a Kneser-Ney model on the training split: unguided 13.6; directed operator re-weighting only 13.9; directed with 64 n-gram candidates at full weight 14.3 against 15.3 untrained. Prior-corrected proposals: 6,729.7 directed against 7,301.6 untrained.

Also from this pass: the v6 file round-trips through `Storage::save` and `load` at 5.07 MB for 6,221 words.

Not re-measured on two sides in the sources read here: the Austen operator results in PHIENT_FACTS.md. Which side those runs read is not stated.

## 5. The one idea

A relation has an end it starts from and an end it arrives at. A word takes part in relations at both ends. So a word needs two angles per relation, one for each end, and a reading of a pair must use the starting word's first angle and the arriving word's second.

The rest of this paper measures that sentence. The collapse is what happens when the two angles are forced equal and the trainer pulls pairs together. The recovery of the head of the offset+1 channel, from 0.003 to 0.391 and 0.562, is what happens when they are allowed to differ. The directed ranking result, 15.5× chance on WikiText-2 with no trained operator, is what happens when the reader uses the two angles the way the trainer wrote them. The loss of 4.7× against 6.7× for the source-only reading is what happens when the reader ignores one of them.

The idea has a precedent that docs/69 names. Word2vec keeps an input vector and an output vector for every word. A word's input vector is used when it is the centre and its output vector when it is the context. The split in Phiano is that arrangement on the circle: per channel, one byte of angle per side and one count.

The idea also fixes the sense of similarity. With one side, two words were similar when their angles matched on a channel. With two sides, `similarity_with` asks whether they present the same source side and the same target side. Two words are alike when they relate to the same things and are related to by the same things. That is a stricter test. Under it `the` and `a` fell from +0.772 to +0.400. docs/69 reads the drop as the collapse leaving: much of the old +0.772 was the shared angle the rule had forced on the head of the vocabulary.

The idea leaves one thing open, and it is the whole of the tail problem. A word's target side moves only when that word comes second in a pair. For a rare word that is seldom. The one-sided phase pooled both roles into one angle and so gave every word all its observations on one variable. The split divides them. docs/71 calls this half the evidence. Held-out perplexity is a sum over exactly the rare words, and it paid.

So the one idea, put as plainly as the measurements allow: direction is real in the data, a symmetric representation of it collapses under attraction, two sides store it, a directed reading uses it, and the cost falls on the words with the least data.

## 6. The collapse, in depth

docs/64 reported offset-channel dispersion of 0.85–0.93 and called the channels healthy. The number was an average over the vocabulary. `bin/trace` broke it down by frequency tier. After two passes the top 100 words sat at 0.003 on offset+1. The 100–500 tier sat at 0.026. The collapse reached further with more passes: the 500–2,000 tier fell from 0.723 at two passes to 0.609 at four, and the tail from 0.890 to 0.770. On the `of` channel the top 100 fell from 0.495 to 0.315.

The mechanism is in the update. With one angle per word per channel, the attraction step for an adjacent pair moves `θ_a` toward `θ_b`. A frequent word such as `the` is adjacent to many others, and each pair pulls both ways. Because the constraint is symmetric, a word that follows `the` and precedes `borrow` is pulled toward both, and so `the` and `borrow` are pulled toward each other through it. On a connected graph the only fixed point of this is every word at one angle. The words with the most edges get there first, which is why the head collapsed and the tail lagged.

Repulsion was already in the trainer. For each coupled pair, one sampled word not observed with the earlier word is pushed away on the same channel. The comment in `couple_relations` says why: attraction alone has one attractor, and without repulsion the offset channels measured 0.3–0.5 dispersion after four passes. With repulsion the tail stayed spread and the average looked honest. The head was never touched. The negative pool built by `Facet::rebuild_sample_pool` gives each word `sqrt(count)` copies clamped to between 1 and 8. docs/69 names the pair of causes: the pool caps any word at eight copies, and only the negative moved.

The first attempt at a fix drew negatives by `count^0.75` without a cap and pushed both words. The top-100 dispersion moved from 0.003 to 0.004. docs/69 draws the conclusion directly: repulsion of any strength only slows the synchronisation. It does not change the fixed point.

The second attempt was the split, with the capped pool. That gave 0.082 source and 0.072 target at the top. Better than 0.004, still a collapse. The split and the `count^0.75` negatives together held the head at 0.391 and 0.562 after two passes and 0.383 and 0.541 after four. The collapse stopped spreading with passes.

After the fix the frequent words' source sides are still the most similar tier, at 0.391 against 0.966 for the tail. docs/69 reads this as a fact about the corpus: the frequent words genuinely share successors. Under one side the closeness came from the rule. Under two sides it comes from the data.

## 7. One sentence through the update

The Rust Book corpus contains the sentence "It uses a borrow checker." Take its five words as tokens: `it`, `uses`, `a`, `borrow`, `checker`, leaving aside how the tokenizer treats case and the full stop. This section walks it through `couple_relations`. The angles of these words were not printed for this paper.

`RELATION_WINDOW` is 8, so every later word in the sentence is in range of every earlier word. The sentence gives ten ordered pairs. For each, `facet.relations.channel(tokens, i, j)` first asks whether a learned marker claims the pair. If none does, the channel is the distance minus one. `it → uses`, `uses → a`, `a → borrow` and `borrow → checker` land on channel 0, offset+1. `it → a` lands on channel 1, offset+2. `it → checker` lands on channel 3, offset+4. The relation vocabulary's own example of a marker: in "bread of wheat", `bread → wheat` is an `of` relation before it is offset+2.

Take `a → borrow` on offset+1. The code reads `ta = a.theta(0)`, the source side of `a`, and `tb = borrow.theta_target(0)`, the target side of `borrow`. It computes `d = sin(tb − ta)`. It moves `a`'s source side by `+rate·d` and `borrow`'s target side by `−rate·d`, so the two close on each other. Both words deposit one observation on channel 0. Because `k` is 0, `a.sync_phase()` also runs, keeping the legacy `phase` field equal to channel 0.

What does not move is the point. `a`'s target side and `borrow`'s source side on channel 0 stay where they are. The next pair, `borrow → checker`, moves `borrow`'s source side and `checker`'s target side. The two constraints share no variable. Under the one-sided rule `borrow` had a single angle on channel 0, so `a` and `checker` were both pulled toward it, and so toward each other. That transitive pull, repeated across a corpus, is the collapse in §6.

If negatives are on, one word is drawn for the pair from the relation pool and skipped if it appears in the sentence. Suppose it is `vector`. Its target side on channel 0 is pushed away from `a`'s source side by `rate·NEG_RATE·away`, and `a`'s source side is pushed away from it. `vector`'s source side does not move.

Now read the result. After training, `a.on(borrow, 0)` returns `cos(src_a[0] − tgt_borrow[0])`. `borrow.on(a, 0)` returns `cos(src_borrow[0] − tgt_a[0])`. These are different numbers. The first asks whether `a` points at `borrow` as its next word. The second asks whether `borrow` points at `a`. A plain text search of the corpus finds "a borrow" and never "borrow a". With two sides the lexicon can hold that difference. With one side it could not: `cos(θ_a − θ_borrow)` is one number for both orders.

## 8. The directed reading

A directed reading takes the source side of whatever comes first and the target side of whatever comes second. The code has three places where it does this, and one where it deliberately does not.

`SpectralPhasor::on(other, k)` is the primitive. It returns `COS_BYTE[src_self − tgt_other]`. `COS_BYTE` is a table of the 256 cosines of every byte difference, recomputed by `test_cos_table_matches_cos`. `on` returns zero when the product of the two evidences on `k` is effectively zero.

`Facet::relational_fit(toks, i, cand)` uses `on` in context. The code calls it the Liskov reading: a candidate may stand at position `i` if every relation the context demands still holds. For each neighbour `j` within `RELATION_WINDOW` of `i`, it finds the channel the sentence puts between them. If `j` is before `i`, the neighbour relates to the candidate, so it reads `o.on(cp, k)`: neighbour source against candidate target. If `j` is after `i`, the candidate relates to the neighbour, so it reads `cp.on(o, k)`: candidate source against neighbour target. Only pairs with evidence on both sides count. The result is the mean agreement and the number of pairs. In "It uses a borrow checker.", testing `borrow` at position 3 reads `src_a` against `tgt_borrow` on offset+1, `src_uses` against `tgt_borrow` on offset+2, and `src_borrow` against `tgt_checker` on offset+1, among others.

`Facet::calls_for(word, goal)` is the endgame reading. It averages `a.on(g, k)` over the eight offset channels where both words have evidence. It asks how much `word` calls for `goal` somewhere after it, up to eight positions on.

The analogy in `bin/promoted` is the third place. `a : b :: c : ?` was read with one side as a difference of angles. With two sides, docs/69 reads it as `tgt_d = src_c + (tgt_b − src_a)`. The relation is the step from `a`'s source side to `b`'s target side, and it is applied from `c`'s source side to find a target side. Under that reading the pair's own channel went from 0.9× null to 2.1×, and antonyms and comparatives on the family channel went from 1.9× and 2.6× to 9.7× and 8.2×. The family channel overall fell from 4.1× to 3.4×.

The one place that is deliberately not directed is `similarity_with`. Similarity is a relation between two words of the same kind, with no order. So it compares source with source and target with target, weighted by each word's evidence on the channel, and averages the two cosines with equal weight. The balanced similarity score in `bin/promoted` did not move with the split: 51.8% before and after. The raw evidence-weighted score went from 31.4% to 36.8%. The equal weighting is a choice in the code. No other weighting was measured.

## 9. Which side should the reader see

The operator in phinet reads a lexicon through `FunctionSpace`. Before the split it encoded one angle per channel. After the split there was a choice, and docs/70 measured all four options through `FunctionSpace::from_facet_sides` with the `Sides` type.

The four readings are these. Source encodes contexts and the ranked table from source sides, which is what the old code did. Target uses target sides for both. Both walks the circle twice: 128 samples, both sides, for context and table. Directed builds contexts from source sides and ranks words by their target sides. That is what a relation is under the split: the context relates, the next word is related to.

The results are in the ranking table in §4. This section follows docs/70's reading of them.

The directed reading is the right one. On WikiText-2, with no operator, it reached 15.5× chance. The best one-sided identity reading had been 6.7×. The source-only reading of the same two-sided lexicon reached 4.7×. So the split did more than stop the collapse. It made the lexicon predictive of the next word by itself. On the Rust Book, the smaller corpus, directed identity was 4.5× against 2.0× for source.

The operator gains less than the identity does. On WikiText-2 the operator went from 27.9× on the one-sided lexicon to 31.9× on the directed reading. On the Rust Book it went from 13.1× source to 13.6× directed. docs/70's reading is that the operator was already doing, approximately, what the directed reading does exactly: turning a context into the side a next word should present. Given the right sides, it has less left to learn.

Reading the source side of a two-sided lexicon is worse than reading a one-sided one. On WikiText-2 the identity went from 6.7× to 4.7×. A one-sided phase did both jobs badly. A source side does one job. Asked to do the other, it fails.

Walking the circle twice buys nothing. The both reading is 9.8× on the Rust Book operator, below every other reading, and 27.9× on WikiText-2, equal to the one-sided number. It doubles the parameters. The target reading beat source on the WikiText-2 operator, 29.0× against 26.5×, and lost on pointwise, 8.7× against 12.2×. Neither single side is consistent. The directed pairing is.

The shipped path follows the result. The reader in `command/generate` uses `Sides::Directed`. `OperatorGuide` in `src/generate.rs` encodes the loaded facet with a chosen `Sides` and refuses a lexicon without relation evidence. docs/71 then ran the perplexity harness under the source and directed readings. Directed beat source for both operators, 153.36 against 157.54 hinge and 142.79 against 144.14 mixture, and on every MRR. The identity was the one exception: 158.48 directed against 155.77 source.

## 10. What the split cost, and the two repairs

docs/71 put the two-sided lexicon through the held-out perplexity harness on the Rust Book. Ranking in docs/70 was over a pool of frequent words: 778 on the Rust Book, 2,240 on WikiText-2. Perplexity is a softmax over the whole vocabulary, and the rare words dominate it.

On that harness the one-sided lexicon was better. The mixture operator was 134.16 one-sided and 142.79 directed. Hinge was 149.30 and 153.36. The identity was 156.33 and 158.48. The mixture with the n-gram tables did not move: 125.56 against 125.52, both at γ = 0.1. docs/68's conclusion stands: on this corpus the tables cover what a 64-sample cosine can say.

docs/71 gave a reason for the loss. A rare word's target side is only moved when it is the later word of a pair. It has half the evidence the one-sided phase had. Ranking on the frequent pool never sees these words.

The first repair followed from that reason. Read a rare target side as the source side until the word has enough evidence: `target_byte` below `TARGET_TIE_FLOOR`. At 16 it made everything worse: directed identity 158.48 → 160.92, hinge 153.36 → 160.04, mixture 142.79 → 145.95. docs/71: a rare word's target side is a different angle with less evidence, and substituting the source side is a category error. `TARGET_TIE_FLOOR` ships at 0.

The second repair kept the sides separate and gave each rare target step more weight. `RARE_TARGET_BOOST = 3.0` on the target nudge while the word had fewer than `RARE_TARGET_FLOOR = 16` observations on the channel. Directed identity 158.48 → 158.47, no change. Hinge 153.36 → 157.21. Mixture 142.79 → 145.76. The one number that improved was the best mix with the n-gram tables, 125.52 → 125.25. docs/72 calls it the largest gain the phase term has shown over the tables' own 125.64, and still a third of a point. A bigger step on a noisy side is a noisier side. The constant ships at 1.0, which is off.

docs/72 draws the lesson. The tail is not short of updates in a way a rate can fix. It is short of data. WikiText-2, where the two sides win ranking outright, is the same lexicon on more data.

Generation was measured too. Re-weighting n-gram candidates did nothing: unguided 13.6, directed 13.9. Proposing its eight nearest target sides gave 181.1 against 10,728.4 untrained in docs/71, and the text broke: "to start well → Parameter to we in." Prior-corrected proposals in docs/72 over-corrected into the rare tail: "to start well → Trpljoin shouldnt extern come ability nothing 9 waiting…", 6,729.7 against 7,301.6 untrained. With 64 n-gram-legal candidates at full weight, trained 14.3 beat untrained 15.3 and trailed the count-driven pick at 13.6. `OPERATOR_WIDEN` ships at 0.

## 11. Where it sits in the PML family

The split lives in phiano, the word application. `SpectralPhasor`, `Facet`, `Trainer`, generation and the readings above are all phiano code. It reaches the rest of the family through a few specific points.

phinet is the parent engine: complex tensors `[b, w, s]`, `Tape` autograd, unitary FFT, `Fno`, joint training. The split changed what phinet is asked to read. phiano's `FunctionSpace::from_facet_sides` encodes the lexicon as samples on the circle, and the `Sides` value decides which array feeds contexts and which feeds the table. The directed reading is what the operator in phinet sees in `command/generate`. The same pass that made the split also changed phinet. docs/69 records `cosine_table` spread over rows with rayon, forward and backward, 86 → 51 ms per step on two cores at `[20]×[6221]×64`; `Tape::mobius(x, abcd)`, the exact per-channel Möbius map with gradients checked by finite difference; and `zeta_weights(modes, s)` with `Fno::scale_modes`. phinet's parity with a PyTorch reference is 62/62 tests. That number is not about the split.

phical is the geometry of the relation circle. At docs/69 the crate in `phiace/crates/phical` was an empty `lib.rs`. It became `phi-cal` with `sphere`, `mobius`, `q64`, `winding` and `heat`, and 13 tests. phiano depends on it and re-exports it as `phiano::phical::exact`. `Hexagram::phase_angle` gained its half-step, `2π(q+½)/64`, with a test that it is the same map as `phi_cal::q64::lift`. The same docs/69 table corrected a bound: the resonance loss under double quantisation is at most π/32 ≈ 0.098 per channel, and the 64-channel mean on random pairs measures under 0.01. The bound was stated in the same pass as the split. The sources do not restate it for the directed reading.

phiton, the light units, reads phases through Phical's formulas. docs/70 clamped `band_n` under every Phical and Phiton formula. That fix is independent of the split. How phiton should colour a word with two sides is not built. phifno 1.0.0, the spherical operator, and the sphere ties have their own results in PHIENT_FACTS.md. Whether they read one side or two is not stated in the sources read here. phinum has no link to the split in these sources. phiqlm is planned and nothing is built.

phiecs is the entity-component world and the quest. Its synthetic oracle records one negative that bears on this paper: the offset reading scored MRR 0.005 against chance 0.021, where attraction-only training with dithered rounding scored 0.594 and the wrong channel 0.015. Whether the quest used the directed `on` is not stated in the sources read here.

phiace hosts phical as a crate. The helix lays out the lexicon by codon, one per channel. It encodes a word's channels. Whether it carries `tgt` is not stated here, and the byte-exact round trip in PHIENT_FACTS.md was not re-checked against v6.

## 12. The next experiment

docs/72 §3 names the next step in generation, and docs/71 §3 names the next step for the tail. This section takes the one that tests this paper's claim most directly.

The claim that fails on the Rust Book is perplexity. docs/72 argues the tail is short of data. The test of that argument is WikiText-2 at the mixture objective, directed reading. docs/70 left it open, docs/71 listed it, and the parallel `cosine_table` made it about a tenth cheaper per core. It is not measured.

The experiment, as far as the sources specify it. Train the lexicon with `couple_relations` on the WikiText-2 corpus docs/70 used, `data/prose_corpus.txt`, with the split and `count^0.75` negatives. Run the perplexity harness with `phinet_sides` in its directed mode on held-out WikiText-2. Report identity, hinge, mixture at γ_train = 1, and the best mix with the n-gram tables, as in docs/71.

The control has to separate the split from the data. Two controls do that. The first is a one-sided lexicon on the same WikiText-2 split: train with the target side held equal to the source side at every step, which is what a v4 lexicon was. The comparison is then one-sided against directed on one corpus. The second is an untrained operator of the same shape on the directed lexicon, as in docs/71 and docs/72, so that any mixture gain can be credited to what the operator learned. The Kneser-Ney score of the held-out split is the reference for both, as 132.07 was on the Rust Book.

The prediction that would support docs/72 is that the gap between one-sided and directed perplexity closes or reverses on WikiText-2, while the ranking lead of docs/70 holds. If the directed mixture is still worse by a similar margin, the half-evidence argument is wrong or incomplete and the split costs something data does not repay.

A second, smaller experiment follows from §13. The evidence count is shared by the two sides. Both `RARE_TARGET_FLOOR` and `TARGET_TIE_FLOOR` test `evidence_count(k)`, which counts a word's observations on `k` as the earlier word and as the later word together. So the rare-target repairs in docs/71 and docs/72 fired on a count that was not a target-side count. A separate target-side count is not built. The experiment is to add one and rerun the rare-target boost on the Rust Book, with the docs/71 directed numbers, 158.48, 153.36 and 142.79, as the control. It is small, and it would say whether the boost was refuted or mis-gated.

One seed was used for every result in this paper. Any of these runs should be repeated on at least a second seed before a difference under the 2% noise line of docs/72 is read.

## 13. What is wrong or missing

The evidence count does not know which side it counts. `couple_relations` calls `deposit(k, 1)` on both words of a pair. `evid` has one count per channel. A word seen many times as the earlier word of offset+1 pairs and never as the later word has a large count on channel 0, and a target side that attraction never moved. `on` returns zero only when the evidence product is zero, so it will read that unmoved target side as if it were learned. `relational_fit` and `calls_for` gate on `evidence_count(k) > 0` for both words, with the same gap. The tie and the rare-target boost both test this shared count. The docs describe the tail as having half the evidence. The code cannot see that half.

The doc comments disagree with the code on evidence width. `types.rs` still describes `evid` as 16 bits packed four to a word, and `set_evidence_count` says it saturates at 16 bits. The constants say otherwise: `EVIDENCE_WORDS = PHASE_CHANNELS / 2`, `EVIDENCE_MAX = u32::MAX`, and `evidence_count` reads 32 bits. docs/70 records the change to 32-bit lanes and format v6. This paper follows the code.

docs/69 says `couple_relations` moves the earlier word's source side toward the later word's target side "and nothing else". The code also moves the later word's target side toward the earlier word's source side. The comment in the code says "and vice versa". This paper follows the code.

`from_packed` and `from_packed_with_evidence` set `tgt` equal to `packed`. A v4 file loaded through them reads as a lexicon with both sides equal. That is the stated design. A reader who loads an older file and then applies the directed reading is reading `cos(θ_a − θ_b)`, the symmetric reading under another name, with no warning. `sides_equal()` exists and is the check. The sources read here do not show it called on load.

`similarity_with` averages source and target cosines with equal weight. The weighting was not varied.

Everything rests on one seed. docs/72 puts the noise at about 2%. Several differences in §4 are under that: the source-versus-directed mixture, 144.14 against 142.79; the rare-target best mix, 125.52 against 125.25; the tables' own mixture across readings.

The generation judge is itself an n-gram model. docs/72 says so and names what would settle it: held-out human ratings or a large model's perplexity. Neither was run.

The Austen operator results in PHIENT_FACTS.md, from 7.9× for identity to 30.7× valid for softmax wide+deep, are the headline operator numbers of the family. The sources read for this paper do not say which side those runs read. Until that is checked they should not be cited as two-sided results.

The stub files `src/phinet/{tensor,operator}.rs` are still listed for deletion in docs/71.

## 14. How it was found

The history is in five docs.

docs/64 reported offset-channel dispersion of 0.85–0.93 over the Rust Book vocabulary and called the channels healthy. That is the report that turned out to be averaging over the problem.

docs/69 is the finding and the fix. It opens with `bin/trace` run with 2 and 4 passes, the dispersion split by frequency tier, and the numbers 0.003 at the top on offset+1 and 0.495 and 0.315 on the `of` channel. It tried `count^0.75` negatives alone, 0.003 → 0.004, and stated why that could not work: a symmetric attractive rule on a connected graph synchronises, and repulsion slows it. It then added `tgt` to `SpectralPhasor`, made `couple_relations` directed, made `on` directed, made `similarity_with` compare like sides, and bumped the format to v5 with v4 files reading back with both sides equal. It measured the fix by tier and through `bin/promoted`, and it read the analogy directed. It listed as open the operator numbers on the two-sided lexicon, the 65,535 saturation of `evid`, WikiText-2 at the mixture objective, and the stale phinet files. The same doc recorded the phinet and phical work in §11 and corrected the quantisation bound.

docs/70 took those open items in order. It added `Sides` and `FunctionSpace::from_facet_sides` and measured the four readings on both corpora. That is where the directed reading got its name and its 15.5×, and where reading source sides alone was declared a bug. It widened evidence to 32 bits in format v6, clamped `band_n` under Phical and Phiton, and put `OperatorGuide` into `generate.rs`, unmeasured.

docs/71 ran the perplexity harness under source and directed readings and found the tail cost. It proposed the tie, built it as `TARGET_TIE_FLOOR`, measured it at 16, and refuted it. It measured the operator in generation, re-weighting and proposing, and found nothing a trigram judge rewards, with the trained proposer sixty times better than an untrained one.

docs/72 ran the two experiments docs/71 named. The rare-target boost was built, measured and disabled. Prior-corrected proposals were built, measured, and over-corrected into the tail. The one positive arrangement, 64 candidates at full weight, ships at 0 with the mechanism in place.

The pattern across the five is the same. A number that looked healthy was broken down until it showed what it averaged over. A fix was stated with its reason. The reason was tested by building the fix and measuring it. Two of the fixes failed and stay in code, disabled, so their negative results can be rerun. The facet code carries a related lesson from earlier: two runs of one composition experiment returned analogy MRR 0.1066 and 0.0521 until the negative pool was sorted, which is why `rebuild_sample_pool` sorts.

## 15. What remains

The two sides are built and they stay. The collapse they fixed is measured and gone from the head of the offset channels. The directed reading is shipped in `command/generate`. The rest of this section lists what is open, in the order the sources give it, and says for each whether anything has been built.

WikiText-2 at the mixture objective, directed reading. Open since docs/69. Not measured. §12 sets it out with its controls. It is the experiment that says whether the tail cost is a data problem, as docs/72 argues, or a cost of the split itself.

More evidence per rare target side. docs/71 named two routes: a wider window for rare words, or a second pass that couples only pairs whose later word is rare. The boost in docs/72 was a third route and was refuted. The first two are not built.

A target-side evidence count. It follows from §13. Not built. Without it every rule that gates on a word's evidence on a channel gates on both sides together.

Generation from the operator's function. docs/72 §3 names two ways forward. One is to decode the next word as the nearest target side to `Fno(context)` among words the n-gram assigns any probability, with no count term, and judge it with something other than an n-gram. The other is to use the operator as a reranker of whole beams. Neither is measured. Both are directed by construction, since the thing decoded is a target side.

A judge that is not a trigram model. docs/72 names held-out human ratings or a large model's perplexity. Neither has been run. Until one is, the 14.3 against 13.6 comparison cannot say whether the operator's text is worse or only less trigram-like.

Checking the rest of the family for side. Which side the Austen operator runs read, whether the quest in phiecs used the directed `on`, whether the sphere ties and phifno read one side or two, and whether the helix carries `tgt`. None is stated in the sources read for this paper. Each is a question with a short answer in code.

A warning or refusal when a v4 lexicon, with equal sides, is fed to a directed reader. Not built.

Deleting `src/phinet/{tensor,operator}.rs`. Listed since docs/69.

What would change the claim. If WikiText-2 perplexity stays worse for the directed reading at the size where its ranking wins outright, the paper's trade becomes a standing cost and the split has to be argued on ranking and composition alone. If the gap closes, the half-evidence argument holds and the tail was waiting for data. Either result is one run away, and both are worth writing down.

## 16. Sources

The local files read for this paper, all under `/home/claude/phiano/`:

```
docs/papers/phient/BRIEF.md
docs/papers/phient/PHIENT_FACTS.md
docs/69_two_sides.md
docs/70_issues_closed.md
docs/71_generation_measured.md
docs/72_two_more_negatives.md
src/phasor/spectral_phasor.rs
src/phasor/spectral_phasor_set_evidence_count.rs
src/phasor/types.rs
src/facet/facet_relational_fit.rs
src/trainer/trainer.rs                    (couple_relations)
src/relational/relation_vocab.rs          (channel, name)
src/relational.rs                         (OFFSET_CHANNELS)
src/config/constants/channels_per_update.rs  (RELATION_WINDOW, RARE_TARGET_FLOOR, RARE_TARGET_BOOST)
src/config/constants/novelty_scale.rs     (PHASE_CHANNELS, CHANNEL_QUANTA)
data/rust_book_corpus.txt                 (the sentence in §7)
```

The six files named in the brief for this paper are `docs/69_two_sides.md`, `docs/72_two_more_negatives.md`, `src/phasor/spectral_phasor.rs`, `src/phasor/spectral_phasor_set_evidence_count.rs`, `src/facet/facet_relational_fit.rs` and `docs/papers/phient/PHIENT_FACTS.md`. The others were read to check a constant, a function body, or a claim the first six refer to. docs/70 and docs/71 are cited by docs/69 and docs/72 and supply the four-reading ranking table, the perplexity table, and the refuted tie. `types.rs` supplies `TARGET_TIE_FLOOR`, `EVIDENCE_WORDS`, `EVIDENCE_MAX` and the doc comment on `tgt`. `trainer.rs` supplies the body of `couple_relations` and the gating of `RARE_TARGET_BOOST`. `relation_vocab.rs` supplies the rule that a marker takes precedence over an offset and that offset `d` maps to channel `d − 1`.

docs/64, docs/65–68 and the Austen results are referred to only as the sources above cite them. They were not opened for this paper.

What each source contributed, in one line each. docs/69 gives the defect, the dispersion tables by tier, the split, the `bin/promoted` table, the directed analogy, the phinet and phical changes, and the corrected quantisation bound. docs/70 gives the four readings, the ranking table on both corpora, the 32-bit evidence lanes and format v6. docs/71 gives the perplexity table, the refuted tie at 16, and the first generation measurements. docs/72 gives the refuted rare-target boost, the prior-corrected proposals, and the 64-candidate arm. `spectral_phasor.rs` gives the constructors, the two sides' getters and setters, and `target_byte`. `spectral_phasor_set_evidence_count.rs` gives `deposit`, `evidence`, `similarity_with`, `on` and `shared_relation_with`. `facet_relational_fit.rs` gives `relational_fit`, `calls_for` and `rebuild_sample_pool`. PHIENT_FACTS.md gives the paradigm paragraph and the family numbers in §11 and §13.

The commands behind the numbers, as the docs give them:

```
cargo run --release --bin trace [corpus] [passes]
cargo run --release --bin promoted
cargo run --release --bin phinet [corpus] 20000 4
cargo run --release --bin phinet_sides data/rust_book_corpus.txt 3 directed
cargo run --release --bin generate_eval
cargo test --release
```

Every run cited in this paper used one seed. The Rust Book runs used `data/rust_book_corpus.txt`. The WikiText-2 runs in docs/70 used `data/prose_corpus.txt`.

Where two sources disagreed this paper preferred the later one and said so: the evidence width in §13, where the code and docs/70 supersede the 16-bit doc comments, and the scope of the update in `couple_relations`, where the code supersedes the "nothing else" in docs/69.
