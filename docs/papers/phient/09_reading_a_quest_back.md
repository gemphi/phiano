# Reading a quest back: the bridge to phiano, the byte that would not move, and dithered rounding

*Phient papers, 9 of 16 · 2026-05-04 · how `bin/quest_ingest` turns a phiecs table into a phiano facet, why the first run could not give it back, and what `Rounding::Dither` changed*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

A quest in phiecs writes one table. Each row holds four fields: `about`, `kind`, `other`, `weight`. A row says that one word stands in one relation to another, and how firmly the oracle held it. Paper 8 described how that table is filled. This paper describes the way back. Phiano's `bin/quest_ingest` turns every row into a three-token document, lets the kinds become marker channels, trains a facet on those documents, and then asks the facet to give the table back. The answer is a rank. It is scored against a pool of targets, against the wrong channel, and against the offset channels the dialogue reads today.

The first run lost almost everything. The shipped trainer reached MRR 0.082 after 64 epochs, against a chance of 0.021. A probe showed why. Every one of the 256 `part-of` pairs had stopped at `|sin d| = 0.195`, eight quanta short of its target. A channel is one byte, 256 quanta of 2π. A nudge of `0.06 · sin d` falls under half a quantum once `sin d < 0.2`, which is 11°. Nearest rounding then writes back the byte it read, and the pair never moves again.

The fix is `phasor::Rounding::Dither`. It rounds up with probability equal to the fractional quantum, so the expected movement equals the nudge. The coin is a hash of the phasor's own bytes and its evidence count, so a run stays reproducible. With dither on and negative sampling off, the same 1,280 rows came back at MRR 0.594, hit@1 34.8%, hit@5 97.4%, and 0.747 within kind. The wrong channel read 0.015 and the offset reading 0.005. Both sit at or below chance.

Dither has a cost. Re-running persona attribution with it on dropped the three-writer score from 71% to 65%. So it ships off by default, and the quest turns it on. A fifth of the rows held out came back at chance on one hop, MRR 0.011. On two hops they came back at 0.095 for `is-a` and 0.093 for `used-for`, the two kinds whose structure is shared across words. The other three kinds stayed at chance.

Two readings by relation name were added to the facet: `Facet::calls_for_under` and `Facet::under`. `bin/quest_ask` opens the saved facet and asks it `word · kind · ?`. Nothing generates from those readings yet. The oracle throughout was `phiecs::quest::Synthetic`, which answers from arithmetic. No language model has been asked, and no number here says anything about what a model's answers would look like on a channel.

## 2. The claim

The claim is small. It has three parts, and each has a limit.

The first part is that a quest needs no adapter to enter a facet. The table is written as documents of the form `about kind other`, and nothing else is written. Phiano's relation inventory, `RelationVocab::learn`, already reads a word that stands between two others as a marker. The quest's kinds stand between their pairs by construction. So `is-a`, `part-of`, `near`, `used-for` and `unlike` become marker channels with no mapping table anywhere in the code. Docs/83 records `is-a` on channel 8, because `is-a` is the word most often found standing between two others. The bridge adds no vocabulary of its own. It relies on the same rule a corpus relies on.

The second part is that a facet trained this way carries the kind on the kind's channel and nowhere else. The evidence is three readings of the same rows. On the kind's own channel the truth ranks well. On the next kind's channel it ranks at chance, 0.013 to 0.015 against 0.021, in every arm. On the offset channels, read through `Facet::calls_for`, it ranks at or under chance. If the channels carried adjacency, the wrong channel would score as well as the right one. It scores at chance.

The third part is about the phasor. A byte channel under nearest rounding has a deadband. Small nudges are lost. On a quest where each pair is heard a few times per epoch, the deadband caps what a channel can hold at about 32 distinguishable targets, however long it trains. Dithered rounding removes that cap in expectation. On a corpus, where a word takes tens of thousands of nudges, the same dither adds a random walk and costs attribution six points. So the paper claims that the rounding rule belongs to the trainer and should be chosen per task. It does not claim that dither is better in general.

What the paper does not claim should be as plain. It does not claim that a language model's answers carry structure a channel can hold. The oracle answers from arithmetic on a word's index, so the truth is known exactly, and that is all the measurement can show. It does not claim that phiano can infer relations it was never told, beyond the two-hop reading on the two kinds whose structure many words share. It does not claim that the generator or the dialogue use any of this. They read offset channels only. And it does not claim the fix is complete. The per-side evidence gap and the sub-quantum residue that would serve both a quest and a corpus are recorded and not built.

## 3. The mechanism

The binary lives in `src/bin/quest_ingest`. It has five modules: `main.rs`, `rows.rs`, `measure.rs`, `holdout.rs` and `liskov.rs`. The last scores words with Liskov's three operators and belongs to docs/86. The first four are this paper.

`rows.rs` reads the table. `read_rows` splits each line on tabs and parses the fourth field as the weight, clamped to 0 through 1. A header row is skipped because its weight does not parse. `documents` turns each `Row` into `vec![about, kind, other]` and pushes it `repeats(weight)` times, which is `((weight * WEIGHT_REPEATS).round()).max(1)` with `WEIGHT_REPEATS = 4.0`. A claim held at 0.9 is heard four times as often as one held at 0.2. `target_pool` collects every `other` in the table, sorted and deduplicated. `kind_pool` does the same for one kind.

`main.rs` builds a `Facet`, calls `facet.learn_relations(&docs)` to learn the marker inventory, and calls `get_or_init` on every word. Then it builds the trainer. `Trainer::new(LEARNING_RATE)` is the shipped trainer, with negative sampling. `Trainer::attraction_only(LEARNING_RATE)` sets `neg_samples` to zero. Either is given `.with_seed(7)` and `.with_rounding(rounding)`. Two environment variables pick the arm. `PHIANO_QUEST_NEGATIVES=1` turns negatives on. `PHIANO_QUEST_ROUNDING=nearest` selects `Rounding::Nearest`. Left unset, the binary runs attraction-only with dither.

Each epoch shuffles the document order with a Fisher–Yates pass on a splitmix stream seeded at `7 + epoch`. It then calls `trainer.couple_relations(&mut facet, &docs[i])` on every document. `couple_relations` asks `facet.relations.channel(tokens, i, j)` for the channel of each pair. For `about` and `other` that is the kind's marker channel. The pair moves on that channel alone, and the move is directed: the head's source side goes toward the other's target side. Evidence for the relation is deposited on both words.

The nudge lands through `SpectralPhasor::nudge_by` for a source side and `nudge_target_by` for a target side. Each computes the new position in quanta, `theta_byte(k) + delta / TWO_PI * CHANNEL_QUANTA`, and hands it to `quantise`. Under `Rounding::Nearest`, `quantise` calls `round()`. Under `Rounding::Dither`, it takes the floor and the fraction, draws a coin in [0, 1) from `mix64(salt ^ quanta.to_bits())`, and adds one quantum when the fraction exceeds the coin. The source salt is `packed[k / 8] ^ evid[k / 2] ^ (k << 56)`. The target salt uses `tgt`, shifts `k` by 48 and mixes in `0xD17`. The result is written back into the channel's byte.

After every epoch, `measure` reads the rows back and prints one line of the table. At the end the facet is saved with `Storage::save` beside the table as `<table>.chroma`.

## 4. What was measured

All measurements use one quest. The oracle is `phiecs::quest::Synthetic`: 256 words in families of eight, standing in five relations fixed by arithmetic on the word's index. A quest of depth 12 and budget 600 walked all of it and wrote 1,280 rows naming 293 target words, 256 rows per kind. With weights applied the table became 3,584 documents. The pool is all 293 targets, and chance MRR over it is 0.021.

`rank_of` breaks ties against the truth: a tied truth ranks last among its ties. A reading that says nothing therefore scores no better than chance.

The first run used the shipped trainer for eight epochs. It read kind-channel MRR 0.057, hit@1 0.7%, 5× chance. Sixty-four epochs gave 0.082. Every reading came back at 1.00 for the wrong word. Docs/83 then ran four arms, 64 epochs each, seed 7:

| negatives | rounding | kind MRR | hit@1 | hit@5 | within kind | wrong channel | offset | s |
|---|---|---|---|---|---|---|---|---|
| on (shipped) | nearest (shipped) | 0.082 | 0.5% | 6.2% | 0.303 | 0.013 | 0.030 | 4 |
| on | dither | 0.177 | 3.8% | 29.0% | 0.404 | 0.014 | 0.018 | 4 |
| off | nearest | 0.092 | 1.0% | 7.0% | 0.330 | 0.014 | 0.013 | 4 |
| off | dither | 0.594 | 34.8% | 97.4% | 0.747 | 0.015 | 0.005 | 4 |

"Within kind" ranks against only the words the oracle ever named under that kind. It is the quest's own question.

Dither was then tried on a corpus. `bin/persona_ingest` was re-run with the docs/81 setup, three writers, attribution on held-out paragraphs:

| rounding | Chesterton | Austen | Carroll | all |
|---|---|---|---|---|
| nearest (shipped) | 74% | 65% | 86% | 71% |
| dither | 74% | 55% | 74% | 65% |

A fifth of the rows was held out with `PHIANO_QUEST_HOLDOUT=0.2`, which kept 245 rows out of training. One hop read MRR 0.011 over all of them, against chance 0.021. Two hops read 0.045 over all, with hit@5 4.1%. By kind, two hops read 0.095 for `is-a` and 0.093 for `used-for`, and 0.009, 0.009 and 0.015 for `part-of`, `near` and `unlike`.

Each quest arm took 4 s. The held-out run, training on the remaining rows, took 3.5 s and ended at epoch 64 with kind MRR 0.608, hit@1 33.9%, hit@5 99.5%, within kind 0.801. PHIENT_FACTS.md and docs/83 agree on every number they share. Sections 8 to 10 give the per-kind tables and the curves.

## 5. The one idea, restated

A relation in Phient is a pair of words moved on the channel of its condition. The quest writes the condition down as a word. Phiano already reads a word standing between a pair as the name of a channel. So a quest row and a sentence of text enter the facet by the same door. The only difference is that a quest row has exactly one marker, and it sits in the middle.

Once a pair is on its channel, learning is a sequence of nudges. Each nudge turns the head's source side toward the other's target side by `0.06 · sin d`, where `d` is the angle still between them. The nudge shrinks as the pair closes. On a continuous circle it would shrink forever and the pair would converge. On a byte it cannot. The circle has 256 positions. A move smaller than half of one position is rounded away. So there is an angle, about 11°, inside which a pair stops being able to close. The first run found every `part-of` pair parked at the edge of that zone, eight quanta out.

Dither does one thing. It makes the rounding fair in expectation. A move of a quarter quantum becomes a move of one quantum a quarter of the time. Over many steps the pair drifts the distance the continuous rule would have taken it. The dither test in `src/phasor/dither.rs` shows this plainly: a quarter-quantum step taken 4,000 times moves about 1,000 quanta dithered, within 150, and exactly zero under nearest rounding. A whole-quantum step is exact either way.

Fairness in expectation is paid for in variance. On the quest, the dithered arm without negatives is the one that converged. On a corpus, a word is nudged tens of thousands of times by many partners, and a coin at each step is a random walk added on top of whatever the pair was doing. There the deadband had been acting as a low-pass filter, and removing it cost six points of attribution.

So the one idea is this. A relation is written by moving a pair on its condition's channel, and read by asking where the head's source side points on that channel. Whether small moves are kept is a property of the task. A quest needs them kept. A corpus is better with them dropped. The phasor offers both, and the trainer chooses. Everything else in this paper is the measurement of that choice, and of what the facet can then say when it is asked the quest's own question back.

## 6. From a row to a channel

Take one row of `data/quest/synthetic.tsv`: `w48`, `near`, `w49`, with its weight. `read_rows` turns it into a `Row`. `repeats` turns the weight into a count, at least one and at most four. `documents` pushes `["w48", "near", "w49"]` that many times.

`facet.learn_relations(&docs)` then builds the `RelationVocab` from these documents, if the facet has none yet. It returns the number of marker channels. On this quest that number is five, one per kind, as the held-out run prints: `5 marker channels`. A word that is found standing between two others often enough is given a channel. In a quest table every kind word stands between its pair on every line where it appears. `about` and `other` words never stand between anything. So the five kinds are the only markers, and every word of the 256 and every target is left as a word.

`couple_relations` walks the pairs of each document. For the pair `w48` and `w49`, `facet.relations.channel` returns the `near` channel, because `near` stands between them. The trainer moves `w48`'s source side on that channel toward `w49`'s target side. It deposits `near` evidence on both. No other channel of either word is touched by this row.

The reading is `facet.on(about, w, k)`: how much `about` calls for `w` on channel `k`. `measure` computes four ranks per row with it. The kind rank ranks the truth among all 293 targets on the kind's channel. The within rank ranks it among the words named under that kind only. The wrong rank uses the channel of the next kind in first-seen order, `channel[(ki + 1) % kinds.len()]`. The offset rank uses `facet.calls_for(about, w)`, the reading the dialogue uses today, which looks only at the offset channels.

The wrong channel is the control that matters most. If the ingestion had written adjacency, any marker channel would recover the pair, and the wrong channel would score near the kind channel. It reads 0.013 to 0.015 in every arm. The kind channel carries the kind. The offset reading never rises above chance either. A quest's relations are therefore invisible to generation until the generator reads marker channels.

The read-back column in docs/83 shows what a hit looks like: `w48 near → w49`, `w232 unlike → w104`, `w76 is-a → f9`. The held-out run prints twelve rows sampled evenly across its table. `w81 unlike` reads back `w209`, which is what the quest said. `w65 near` reads `w66`. `w34 used-for` reads `t4`. `w109 near` reads `w110`. `w219 unlike` reads `w91`. Those are five of the twelve on the whole pool, all at a score of 1.00. The other seven are misses on the whole pool, and section 9 reads them.

## 7. The byte that would not move

The facts are few. A channel is one byte. The circle is 256 quanta of 2π, 0.0245 rad each. The relation nudge is `RELATION_RATE · sin d`, and `RELATION_RATE` is 0.06. Nearest rounding keeps a move only when it is at least half a quantum.

Put together, the nudge falls under half a quantum once `sin d < 0.2`, which is 11°. Inside that angle `round()` returns the byte it was given. The pair's angle does not change, so `d` does not change, so the next nudge is the same size and is lost the same way. The pair is frozen. This explains the first run's shape. Eight epochs gave 0.057 and sixty-four gave only 0.082.

The probe that found it printed `|sin d|` for every one of the 256 `part-of` pairs. It printed the same number every time: 0.195. Eight quanta of 0.0245 rad is the angle whose sine that is. Every pair had closed from wherever its seed put it to the edge of the deadband and stopped there. Docs/83 draws the consequence. A channel under nearest rounding can hold at most about 32 distinguishable targets, however long it trains. That puts a ceiling on the question `bin/dimensions` had asked, how many relations a channel can carry. Nobody had measured that ceiling before this run.

The doc comment on `Rounding` in `src/phasor/dither.rs` restates the same account and the same numbers. The probe was run on the quest only. Whether a corpus trainer's pairs were also held by the deadband was not probed.

Negative sampling is the other half of the ceiling. The shipped trainer, for each pair, also kicks the source away from a random word's target by `0.5 · rate · sin`. Once the pair is within 30°, that kick is the same size as the attraction. The pair jitters instead of settling. On a corpus the repulsion does useful work. The trainer's own note records that without it the offset channels ended at 0.3 to 0.5 dispersion after four passes, agreeing about everything. On a quest graph, where every source has one target per channel, the repulsion has nothing to disperse and only adds noise.

The four arms separate the two causes. Nearest rounding with negatives off reads 0.092, barely above the shipped 0.082. Dither with negatives on reads 0.177. Dither with negatives off reads 0.594. Neither change alone is enough. The deadband stops the pair from closing, and the negatives shake it loose whenever it gets close. Only with both removed does the pair settle on its target. The quest ships attraction-only and dithered.

## 8. Dithered rounding

`Rounding` is an enum in `src/phasor/dither.rs` with two variants. `Nearest` is the default and is documented as what every objective did before docs/83. `Dither` is documented as a coin on the fractional quantum, every step.

`quantise(quanta, salt, rounding)` is the only place the two differ. Under `Nearest` it returns `q.round()` modulo 256. Under `Dither` it splits `q` into floor and fraction and draws a coin. The coin is `mix64(salt ^ quanta.to_bits()) >> 11`, scaled into [0, 1) by 2⁵³. If the fraction is larger than the coin, the result is the floor plus one. Otherwise it is the floor. The expected result is `q` itself.

The salt decides reproducibility. It is built from the phasor's own packed bytes for that channel group, its evidence word for the channel, and the channel index shifted high. Every coupling deposits evidence, so the evidence count changes between one nudge and the next on the same channel. The same state is therefore never asked the same coin twice, and yet a run with the same seed and the same document order produces the same facet. The target side mixes in a different shift and the constant `0xD17`, so the two sides of one channel do not share coins.

`nudge` and `nudge_target` keep nearest rounding, so every caller written before docs/83 is unchanged. The trainer's choice is set by `with_rounding`.

The training curve of the held-out run shows how dither accumulates. It trained on the four fifths of the rows not held out.

| epoch | kind MRR | hit@1 | hit@5 | within kind MRR |
|---|---|---|---|---|
| 1 | 0.026 | 0.1% | 2.3% | 0.145 |
| 8 | 0.131 | 3.9% | 18.8% | 0.316 |
| 16 | 0.338 | 15.6% | 55.0% | 0.579 |
| 32 | 0.556 | 30.8% | 90.5% | 0.790 |
| 64 | 0.608 | 33.9% | 99.5% | 0.801 |

The rise is steady for about thirty epochs and flat after about forty-five. Within kind first reads 0.800 at epoch 36 and holds at 0.800 to 0.801 from epoch 45 on. Hit@1 wanders between 31.1% and 35.9% from epoch 40 on. Docs/83 attributes where hit@1 stops to collisions on the circle, and section 9 reads them.

The cost on a corpus is the persona table in section 4. Austen fell from 65% to 55% and Carroll from 86% to 74%. On 247,000 words of Austen a word takes tens of thousands of nudges. A coin per step is a random walk of √N quanta on top of whatever the pair was doing. The deadband that costs a 1,280-row quest its convergence acts on a corpus as a low-pass filter.

## 9. Per kind, collisions and ties

With negatives off and dither on, docs/83 breaks the result down by kind. The whole-pool reading ranks against all 293 targets. The within-kind reading ranks against the words the oracle named under that kind.

| kind | rows | MRR | hit@1 | hit@5 | within pool | within MRR | within hit@1 | within chance |
|---|---|---|---|---|---|---|---|---|
| is-a | 256 | 0.507 | 21.9% | 100% | 32 | 0.875 | 75.0% | 0.127 |
| part-of | 256 | 0.622 | 35.2% | 99.2% | 256 | 0.622 | 35.2% | 0.024 |
| near | 256 | 0.639 | 39.5% | 99.6% | 256 | 0.639 | 39.5% | 0.024 |
| used-for | 256 | 0.603 | 45.7% | 89.1% | 5 | 1.000 | 100% | 0.457 |
| unlike | 256 | 0.600 | 32.0% | 99.2% | 256 | 0.600 | 32.0% | 0.024 |

Hit@5 is at or near 100% on four kinds. Hit@1 stops at a third. Docs/83 names the reason. It is the byte again, from the other side. There are 256 `part-of` targets and 256 quanta on the circle. Targets collide. A collision is a tie, and `rank_of` scores a tie against the truth.

The within-kind column shows what the quest itself asked. `is-a` has 32 family words as targets, and reads 75.0% hit@1 over them. `used-for` has 5 tools and reads 100%.

Docs/83 names the reading's weakness in bold: evidence is not per side. A word that was only ever a source on a channel still has evidence there, and its target side keeps a seeded, random angle. Nothing in the phasor records that it was never a target. So on the whole pool such a word can score 1.00 for a relation it was never named in. The example is `w0 is-a → w29 (1.00)`. `w29` was never named under `is-a`, and its seeded target angle sits on `f0`'s.

The held-out run's sampled rows show the same effect. `w183 is-a` should be `f22`. The whole pool reads `w50`, and within kind reads `f22`. `w180 used-for` should be `t0`. The whole pool reads `w128`, and within kind reads `t0`. `w15 is-a` should be `f1`. The whole pool reads `w255`, and within kind reads `f15`, which is a family word but the wrong one. `w203 part-of` should be `w144`, and both readings give `w181`.

The held-out run's own per-kind table, on the rows it trained on, reads higher within kind: `is-a` 0.908 and 81.7%, `part-of` 0.716, `near` 0.714, `unlike` 0.673, `used-for` 1.000. Its whole-pool MRRs are 0.582, 0.666, 0.676, 0.639 and 0.471. The two tables are from different training sets and are not a comparison.

## 10. Reading by name, and rows the facet was never told

`src/facet/facet_under.rs` adds three readings. `Facet::marker_channel(marker)` returns the channel a marker word was given, or `None`. `Facet::calls_for_under(word, marker, goal)` is the directed reading on that one channel, X ⇒ Y under c, and returns zero when the marker names no relation. `Facet::under(word, marker, pool)` returns every word in the pool that `word` calls for under `marker`, strongest first, zeros dropped, ties broken by spelling. Its test trains `cat is-a animal` and `cat has whiskers` for 64 passes, attraction-only and dithered, and checks that `under` puts `animal` first under `is-a` and `whiskers` first under `has`.

`bin/quest_ask` opens the `.chroma` that `quest_ingest` saved beside the table and asks `word · kind · ?` over every word in the lexicon. It answers "not a kind this quest learned" for an unknown kind and "not a word this quest met" for an unknown word. Docs/83 shows its output:

```
w0 · is-a ⇒ f0 (1.00), w29 (1.00), …      w0 · near ⇒ w1 (1.00), …
w0 · unlike ⇒ w128 (1.00), …             w176 · part-of ⇒ w211 (1.00), …
f0 · is-a ⇒ w150 (1.00), …               ← f0 was only ever a target: its source side is its seed
```

The first four are the truth. The last is the per-side evidence gap read from the other end.

`holdout.rs` splits the rows by an FNV-1a hash of `about`, `kind` and `other`, so the split is deterministic. At `PHIANO_QUEST_HOLDOUT=0.2` it held out 245 rows. One hop reads the held-out row on its own channel, which never saw it. Chance is the honest expectation, and the reading came in under it at 0.011, because the truth ties with every unevidenced word and ties count against it. `two_hop` tries each kind, takes the three strongest intermediates `m` that `about` calls for under it, and scores `s1 · on(m, w, k)`, keeping the best. It knows nothing of the synthetic structure.

| kind | held out | one hop MRR | two hops MRR | two hops hit@5 |
|---|---|---|---|---|
| is-a | 43 | 0.003 | 0.095 | 11.6% |
| used-for | 56 | 0.003 | 0.093 | 7.1% |
| part-of | 48 | 0.017 | 0.009 | 0.0% |
| near | 50 | 0.021 | 0.009 | 0.0% |
| unlike | 48 | 0.013 | 0.015 | 2.1% |
| all | 245 | 0.011 | 0.045 | 4.1% |

`is-a` is shared by a family and `used-for` by every fifth word, so a neighbour's answer is evidence. `part-of`, `near` and `unlike` are one target per word by construction, and no reading can infer them. Docs/83 calls four and a half times chance a small number and the right one.

## 11. Place in the PML family

Phinet is the parent engine: a complex `Tensor [b,w,s]`, `Tape` autograd, a unitary FFT, the Fno, joint training, HRR, 64 hexagram gates and the sphere. It is checked against PyTorch by 62 of 62 parity tests. The bridge described here imports nothing from phinet. `main.rs` uses `phiano::facet::Facet`, `phiano::phasor::Rounding`, `phiano::storage::Storage`, `phiano::trainer::Trainer` and `phiano::config::LEARNING_RATE`. The deadband lives in phiano's byte `SpectralPhasor`. Whether any phinet path has a like deadband is not measured.

Phical is the geometry of the relation circle. This paper adds one fact to that geometry as phiano stores it: a circle cut into 256 positions has a dead zone around every target, about 11° wide under a nudge of `0.06 · sin d`, and the rounding rule decides whether that zone holds a pair. The finding is about the stored byte, 256 quanta of 2π, and was measured only there.

Phifno 1.0.0, the spherical operator, and the sphere ties in phinet were measured on Austen. The best sphere result is the anisotropic tie at 16.0× valid and 15.6× test, which beats the circle's 14.7× and 14.6× with 4,776 parameters against 8,360. None of that touches the quest. No quest has been read back through a sphere tie. PhiTon gives light units, and the helix gives each codon a PhiTon wavelength, band and hexagram. No quest has been written into the helix in any source read here. Phinum is a child of phinet, and nothing in the sources for this paper touches it. Phiqlm is planned and nothing is built.

Phiano is where this work lives. The bridge feeds it a new kind of text, a table, and the facet grew `calls_for_under` and `under` to read it. The trainer grew `with_rounding` and `attraction_only`. The phasor grew `Rounding`, `quantise`, `nudge_by` and `nudge_target_by`. Persona attribution, the phiano measurement that gives 71%, is what set dither's default to off.

Phiecs is the other side of the bridge. Its quest writes `relations.tsv`, and `Quest::from_tsv` reads such a table back into a world, so a quest can be continued from where an earlier run stopped. The bridge reads the same table in phiano. Phiecs does not read the `.chroma`.

What depends on this paper today is one thing. `quest_ingest` also runs Liskov's three operators over every word and files each under its hexagram (docs/86). There the 256 words occupy 61 of the 64 hexagrams. Mean better and mean worse are both −0.008, and mean loss is 0.108 at margin 0.1. Docs/86 calls that flat, and names the byte ceiling again.

Phiace does not appear in any source read for this paper. No link between the bridge and phiace is claimed.

## 12. The next experiment

The experiment docs/83 names is the Phi-4 run. The same binaries are pointed at a model instead of at arithmetic. On the phiecs side that is the quest example with `ollama run phi4` as the oracle. On the phiano side it is `quest_ingest` on the table that walk writes, with the same four arms and the same held-out fraction. Docs/83 calls it the first honest measurement of what a model's answers look like on a channel. It has not been run.

The control is the synthetic quest in this paper, read by the same binary with the same seed, 7, and the same 64 epochs. That control fixes what the pipeline can do when the truth is exact: 0.594 on the kind channel, 0.747 within kind, 0.015 on the wrong channel, 0.005 on the offsets. A model's table will differ in size, in kinds and in pool. So the comparison has to be made in multiples of each table's own chance MRR, which `chance_mrr` computes from the pool size. The wrong-channel reading is the control inside the run. If a model's kinds are real conditions, the wrong channel stays at chance while the kind channel rises. If the wrong channel rises with it, the model's kinds are not separating anything.

Two cautions follow from the code. A model's kind words will not be five fixed strings. `RelationVocab::learn` will give each word standing between a pair a channel, and a model may use many spellings for one relation. The binary will measure whatever inventory it learns, and the per-kind table will show how many there were. The second caution is about ground truth. With a model there is none, so kind MRR measures recall of the model's own table. The held-out reading is the only one that asks the facet something it was not told.

The second experiment is the phasor change docs/83 argues for and does not make. It is a per-side sub-quantum residue. Docs/83 says two bits per channel would do, carrying the fraction that nearest rounding drops. Its test has two controls, and both are already measured. On the quest the control is the dithered arm, 0.594. On the corpus it is nearest rounding on persona attribution, 71%. The residue earns its place only if it matches the first without losing the second. Per-side evidence is a separate change with its own control. It would let the whole-pool reading drop words that were never targets on a channel, and the control is the gap between whole-pool and within-kind hit@1 on `is-a`, 21.9% against 75.0%.

Neither experiment has been run, and neither has a number.

## 13. What is wrong or missing

The oracle is arithmetic. Every structure the facet recovered was put there by index arithmetic in `phiecs::quest::Synthetic`, one target per word per kind for three kinds, a family for `is-a`, and a tool for every fifth word for `used-for`. This shows that the pipeline does not lose structure. It shows nothing about whether a language model's answers have structure to keep.

Most of the numbers read back rows the facet was trained on. Kind MRR, within kind, wrong channel and offset are all measured on the training rows. They measure recall of a table. Only the held-out reading measures inference, and it is at chance for three of five kinds by construction.

There is one seed. Every arm uses seed 7 and one document order per epoch. Variance across seeds is not measured. The held-out curve shows hit@1 moving between 31.1% and 35.9% from one epoch to the next at the plateau, and a single final number carries that wobble.

Evidence is not per side. A word that was only a source on a channel reads there with a random target angle, and a word that was only a target reads with a seeded source angle. `f0 · is-a ⇒ w150 (1.00)` is the plain case. The whole-pool reading pays for this in ties. Per-side evidence is not built.

Hit@1 is capped, and docs/83 reads the cap as collisions: 256 `part-of` targets on a 256-quantum circle, and a collision is a tie scored against the truth. More channels per kind, or a finer grid, are the obvious ways past it. Neither is built or measured.

Dither is a trade, and no rule serves both tasks. The persona cost is six points overall and twelve on Carroll. The sub-quantum residue that might serve both is recorded and not built.

The generator and the dialogue do not read marker channels. `Facet::under` exists and nothing generates from it. A quest ingested today changes nothing a user of the dialogue would see.

The saved facet is overwritten. `main.rs` calls `Storage::save` at the end of every run, holdout runs included, and writes `<table>.chroma` beside the table. Docs/83 lists a held-out run against `data/quest/synthetic.tsv` next to the `quest_ask` command. So `data/quest/synthetic.chroma` is whichever run wrote it last. The two read-backs of `w0 is-a` differ, `f0` first in the `quest_ask` output and `f29` in the held-out run's table, which is consistent with two different facets.

Two small inconsistencies sit in docs/83. Its prose says the offset reading never rises above 0.06, while its own table tops out at 0.030. Its per-kind table labels a pool of 293 where the binary prints the kind's own target count. Neither changes a result.

## 14. How it was found

Docs/81 set up persona attribution: three writers, held-out paragraphs, 71% overall. That number is the one dither was later tested against. `bin/dimensions` had asked earlier how many relations a single channel can carry. Nobody had put a ceiling on that question. Paper 8 describes the phiecs quest and its synthetic oracle, which was built so the truth would be known exactly.

Docs/83 is the record of this work, in order. It opens with the bridge. Rows become documents, kinds become markers, and `couple_relations` moves each pair on its kind's channel. The first measurement came next: eight epochs of the shipped trainer at 0.057, sixty-four at 0.082, every reading at 1.00 for the wrong word. That result prompted the probe. It printed `|sin d|` for every `part-of` pair and got 0.195 two hundred and fifty-six times. From that single repeated number the deadband was worked out: a byte channel, a nudge of `0.06 · sin d`, and half a quantum reached at 11°.

The fix was written into the phasor as `Rounding::Dither`, with the coin hashed from the phasor's own state. It was kept out of `nudge` and `nudge_target` so no existing caller changed. The comment on `Rounding::Nearest`, "what every objective did before docs/83", dates the change. The tests `small_steps_accumulate_in_expectation` and `whole_steps_are_exact_either_way` pin its behaviour.

The four arms came next. They were run to separate the deadband from the negatives, and they showed that both had to go. Then came the obvious question: whether to make dither the default. `bin/persona_ingest` was re-run on the docs/81 setup and lost six points. The answer was no. Dither became the trainer's setting, on for a quest and off for a corpus, and the residue that would serve both was recorded and not built.

The reading by name came after the numbers. The measurement had used `facet.on` on a channel index directly. `Facet::marker_channel`, `calls_for_under` and `under` put that reading behind the relation's name, and `bin/quest_ask` was written to open the saved facet and ask it. The output exposed the per-side evidence gap from the target end, `f0 · is-a ⇒ w150`.

In docs/83's order, the held-out run was last. `PHIANO_QUEST_HOLDOUT=0.2` was added with `holdout.rs`. Its full output, the 64-epoch curve, the per-kind table on the trained rows, twelve sampled read-backs and the one-hop and two-hop table, is kept as `docs/83_quest_ingest_holdout.md`.

Later, docs/86 ran Liskov's three operators inside the same binary and filed the quest's words by hexagram, and found the scores flat. PHIENT_FACTS.md carries all of these numbers and agrees with docs/83 on every one it shares.

## 15. What remains

The first thing that remains is a model. The same quest walk pointed at `ollama run phi4`, and the same ingestion of its table, has not been run. Until it is, the only oracle that has fed a phiano facet is arithmetic. Every number in this paper is a statement about the pipeline and about the byte. None is a statement about what a model knows.

The second is the phasor. Two changes are argued for and not made. The first is per-side evidence. A channel would record that a word was a source, a target or both there. The whole-pool reading could then drop the ties that come from seeded angles. The second is a sub-quantum residue, two bits per channel by docs/83's estimate. It would keep the fraction that nearest rounding drops without adding a coin, so one rule could serve a quest and a corpus. Both change the stored form of a word, so both touch `.chroma` and every reader of it.

The third is the generator. `Facet::under` answers the quest's own question, and nothing generates from it. The dialogue reads `calls_for`, which reads offset channels only, and which this paper measured at or below chance on every quest arm. A reply that used a quest's relations would need a generator that reads marker channels. That generator does not exist.

The fourth is collisions. With 256 targets on a 256-quantum circle, hit@1 stopped near a third while hit@5 reached 97.4%. How many distinct targets a channel can hold under dither is not measured. The collision count is not measured either. The only evidence is the plateau in the held-out curve.

The fifth is variance. One seed was run. The spread of the final numbers across seeds is not measured. The only hint is the epoch-to-epoch wobble at the plateau, several points of hit@1.

The sixth is inference. The two-hop reading is generic and uses three intermediates per kind. It recovers `is-a` and `used-for` to about four and a half times chance over all held-out rows, and nothing on the one-to-one kinds, as their construction requires. Whether a deeper or weighted hop does better on a model's table, where structure is unknown, is not measured.

The seventh is the saved file. `quest_ingest` overwrites `<table>.chroma` on every run, including held-out runs. A run that should leave the reference facet in place has no way to say so. This is a small change in `main.rs` and it has not been made.

What does not remain is the question that started this. The byte deadband is found, measured, and bypassed for the quest. Whether it should be bypassed for everything is answered by the persona table, for now, with no.

## 16. Sources

Every path below was read for this paper. No other file was used for a number.

| path | what was taken from it |
|---|---|
| `/home/claude/phiano/docs/papers/phient/BRIEF.md` | the rules and the sixteen-section arc |
| `/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` | the paradigm paragraph; the family; the quest, deadband, dither, persona, sphere and Liskov numbers |
| `/home/claude/phiano/docs/83_quest_ingest.md` | the bridge; the first run and the probe; the four arms; the per-kind table; the persona cost; reading by name; the holdout table; the run commands; what remains |
| `/home/claude/phiano/docs/83_quest_ingest_holdout.md` | the 64-epoch curve; the per-kind table on trained rows; twelve sampled read-backs; the held-out table |
| `/home/claude/phiano/src/bin/quest_ingest/main.rs` | the arms and their environment variables; seed 7; the shuffle; the reports; `Storage::save` of `<table>.chroma`; `chance_mrr` |
| `/home/claude/phiano/src/bin/quest_ingest/rows.rs` | `Row`, `read_rows`, `WEIGHT_REPEATS`, `repeats`, `documents`, `kinds`, `target_pool`, `kind_pool` |
| `/home/claude/phiano/src/bin/quest_ingest/measure.rs` | `Score`, `rank_of` and its tie rule, `Readings`, `measure`, `best_on` |
| `/home/claude/phiano/src/bin/quest_ingest/holdout.rs` | `split`, `two_hop`, `measure_holdout` |
| `/home/claude/phiano/src/phasor/dither.rs` | `Rounding`, `quantise`, `nudge_by`, `nudge_target_by`, the salts, the two tests |
| `/home/claude/phiano/src/facet/facet_under.rs` | `marker_channel`, `calls_for_under`, `under`, and the cat test |
| `/home/claude/phiano/src/bin/quest_ask/main.rs` | `quest_ask`'s arguments, stdin mode and answers |
| `/home/claude/phiano/src/trainer/trainer.rs` (searched) | `with_rounding`, `couple_relations`, `attraction_only` |
| `/home/claude/phiano/src/facet/facet.rs` (searched) | `learn_relations` |
| `/home/claude/phiano/src/config/constants/channels_per_update.rs` (searched) | `RELATION_RATE = 0.06` |

Three files were searched for single definitions and not read in full: the trainer, the facet's `learn_relations`, and the constants file. They are marked "searched" above. Docs/81, docs/84, docs/86 and `bin/dimensions` are cited only as docs/83 and PHIENT_FACTS.md describe them. They were not opened for this paper. Paper 8 of this series is cited for context and supplies no number here. Nothing on the web was used.

The run commands in docs/83 are `cargo run -p phiecs --release --example quest -- w0 12 600 synthetic 256` to walk the quest, `cargo run --release --bin quest_ingest -- <dir>/relations.tsv 64 out.md` to ingest and measure it, `cargo run --release --bin quest_ask -- data/quest/synthetic.chroma w0 is-a` to ask the saved facet, and the same ingestion with `PHIANO_QUEST_HOLDOUT=0.2` for the held-out rows. None was re-run for this paper. Every number above is quoted from the files as they stand.

Where PHIENT_FACTS.md and docs/83 give the same number, they agree: 0.082, 0.594, 34.8%, 97.4%, 0.747, 0.015, 0.005, 0.011, 0.095, 0.093, and 71% to 65%. Numbers found only in `83_quest_ingest_holdout.md` are attributed to the held-out run where they appear. The per-kind pools in that file follow the binary's column, which docs/83 relabels.
