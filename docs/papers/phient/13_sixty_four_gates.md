# Sixty-four gates
*Phient papers, 13 of 16 · 2026-06-29 · Sixty-four gates: the I Ching hexagrams as spectral operations on a word, and the hexagram a word is*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

This paper states what `phinet::hexagram` is, what it does to a word, and what it was measured to do. A hexagram is six lines, each yin or yang, so it is a six-bit number from 0 to 63. A word in phinet is a function of 64 complex samples, one per relation channel. The module reads the word's spectrum in six octaves and lets a hexagram name a gate: octave `j` passes when line `j` is yang. That gives sixty-four exact linear operations on any word. ䷀, the Creative, all six lines yang, is the identity. ䷁, the Receptive, all six yin, keeps only the mean.

The module also reads in the other direction. A word has a hexagram of its own: the sector, out of 64, that the phase of its centroid falls in. The code calls this the Q64 juncture. So a sentence is a walk of hexagrams, and so is the operator's prediction.

Three things were measured. As an oracle the hexagram failed. On the Rust Book the operator's predicted hexagram matched the next word's 0.9% of the time, against chance at 1.6% (docs/74 §3). As an operation in composition it did what it was built to do, at a small price. Passing every fourth prediction through gate `0b101010` moved long Austen compositions from evaluator 0.669 to 0.689 and from Kneser-Ney perplexity 11.6 to 12.2 (papers/15 §6). As a filing system for the quest it spread 256 words over 61 of 64 hexagrams, and the scores in every hexagram were flat (docs/86 §3).

Reading the code for this paper turned up a defect in the names. Three parts of the code attach hexagram pictures and numbers to ids under three different conventions. `phinet::hexagram` counts line 0 as the bottom line. The King Wen table in `phiano::phinum::iching` counts bit 0 as the top line. The glyph helpers in `helix/code.rs` and `quest_ingest/liskov.rs` print the Unicode character at `0x4DC0` plus the id, which follows the King Wen order and draws neither. The gates are exact. The labels on them disagree, and some tables in the docs carry a glyph that does not match the gate.

Last, an earlier paper in the repository, `papers/05`, described a "64-Layer Cognitive Octave Continuum" of four octaves with sixteen layers each. None of it was built. The function it shows, `resonate_depth_64`, appears in that paper and nowhere in the code. The sixty-four gates are what was built in its place. They are one module of 157 lines with three test functions, and this paper is about them.

## 2. The claim

The module makes two claims, and it keeps them apart.

The first claim is about operations. Any hexagram is an exact linear operation on a word's function. The word's 64 samples go through the unitary FFT. The modes are sorted into six octaves by frequency. The hexagram's six lines say which octaves survive. The inverse FFT brings the word back. Nothing is learned and nothing is approximated. The operation is a projection, so applying it twice is applying it once, and two gates compose by the AND of their lines. A composer can ask for a word with its fine structure removed, or its coarse structure removed, or any of sixty-four mixtures, and get it exactly and by name.

The second claim is about identity. A function has a hexagram: the sector its centroid phase falls in when the circle is cut into 64. This reading takes a word to one of sixty-four categories. It makes a checkable prediction possible. If the operator's predicted function has a hexagram, and the true next word has one, the two either match or they do not, and chance is 1 in 64. The module's doc comment states it this way: "whether the predicted hexagram matches the next word's is a measurable claim (chance 1/64)."

The first claim stands. The gates are tested, and they were used in composition. The second claim was measured, and the prediction it licensed failed. docs/75 records the verdict in one row: `phinet::hexagram`, "yes as operations; no as oracle". The category itself survives as a way to file words, which docs/86 uses to report the quest per hexagram.

A third claim sits behind both and belongs to the family. Sixty-four appears four times in the project. docs/84 lists them: "64 relation channels, 64 real spherical harmonics of degree < 8, 64 I Ching hexagrams (phinet already reads a spectrum's octaves as one), and 64 codons of DNA." The hexagram module is the place where the I Ching count was turned into code. The count agrees. Whether the agreement means anything about words was not measured, and this paper does not claim it does.

What the paper does not claim is also plain. It does not claim the I Ching predicts anything. It does not claim the King Wen order carries information about language; the one test of that, the King Wen successor, scored 1.1% against chance at 1.6%. It does not claim the six octaves are the right six bands for a word. They are the bands a power-of-two circle gives, and section 3 shows how unequal they are.

## 3. The mechanism

The module is `phinet/src/hexagram.rs`. It imports `fft_inplace` from `crate::tensor` and the types `c64` and `Function` from the crate root. Two constants fix the shape: `HEXAGRAMS = 64` and `LINES = 6`.

`octave(mode, space)` places a Fourier mode in an octave. Mode 0, the mean, returns `None` and always passes. Any other mode is folded to its positive partner, `m = space − mode` when the mode is above half, and its octave is `floor(log2 m)`, capped at 5. The top octave therefore takes everything from 32 up to Nyquist. On a 64-sample circle that is one mode. The table below is read directly off `octave` for `space = 64`.

| line `j` | positive modes | modes in the 64-sample spectrum | count |
|---|---|---|---|
| mean | 0 | 0 | 1 |
| 0 | 1 | 1, 63 | 2 |
| 1 | 2–3 | 2–3, 61–62 | 4 |
| 2 | 4–7 | 4–7, 57–60 | 8 |
| 3 | 8–15 | 8–15, 49–56 | 16 |
| 4 | 16–31 | 16–31, 33–48 | 32 |
| 5 | 32 | 32 | 1 |

The test `gates_partition_the_spectrum` checks the edges: `octave(1, 64)` is 0, `octave(3, 64)` is 1, `octave(32, 64)` is 5, and `octave(63, 64)` is 0 because "mode 63 is −1". It also checks that the six single-line gates together pass every non-zero mode exactly once.

`gate(id, space)` returns a mask of booleans, one per mode: true for the mean, and `(id >> j) & 1 == 1` for a mode in octave `j`. `apply(f, id)` copies the function, runs the forward FFT, zeroes every mode the mask rejects, and runs the inverse. Because the FFT is unitary, this is an orthogonal projection. `changing_lines(id, mask)` is `id XOR mask`, kept to six bits: the oracle's changing lines.

`hexagram_of(f)` sums the samples to a single complex number, takes its angle in `[0, 2π)`, and returns `floor(angle / 2π × 64)`. `phase_of(id)` is the inverse, `(id + ½) / 64 × 2π`, the middle of the sector. The half step makes the round trip exact, and a test checks it for all 64 ids.

`novelty(f, id)` is `1 − cosine(f, apply(f, id))`, using `operator::cosine`, which is the real part of the inner product over the two norms. It is zero for the Creative and grows as the gate removes what the word is made of. `lines(f)` returns the energy of `f` in each of the six octaves, normalised to sum to one, with the mean left out. It is the hexagram reading of a word: which of its lines are strong.

## 4. What was measured

Four measurements touch the hexagram module.

The first is the oracle test, `bin/hexagram_eval`, on the Rust Book (docs/74 §3, papers/15 §7). The binary trains a ranking-only facet on the corpus split with seed 42, builds the directed function space, loads the operator, and walks the first 2,000 held-out sentences. At each position it compares five guesses against the true next word's hexagram.

| guess | matches the next word's hexagram |
|---|---|
| operator's predicted function | 0.9% |
| context function (identity) | 5.8% |
| same hexagram as the previous word | 3.6% |
| next in the King Wen sequence | 1.1% |
| chance, 1/64 | 1.6% |
| the most frequent hexagram, always | 12.8% |

The operator is below chance. The King Wen successor is below chance. The strongest guess is a constant, because the centroid sectors are skewed. docs/74 closes the section with "A keyboard, not an oracle. The operations stay."

The second is the novelty arm in composition, `bin/austen`, eight Austen-shaped prompts, one fixed seed each, temperature 0.3 (papers/15 §6). The arm passes the operator's prediction through gate `0b101010` every fourth token.

| arm | KN perplexity | evaluator | distinct |
|---|---|---|---|
| short, learned + operator | 16.2 | 0.697 | 0.990 |
| short, learned + operator + I Ching novelty | 16.0 | 0.701 | 0.979 |
| long, learned + operator | 11.6 | 0.669 | 0.727 |
| long, learned + operator + I Ching novelty | 12.2 | 0.689 | 0.704 |

The third is the reading of six Austen words (papers/15 §7), given in section 7 below with their sectors, King Wen numbers, line energies and novelty under the same gate.

The fourth is the quest filed by hexagram (docs/86 §3, docs/83). The 256 `about` words of the synthetic phiecs quest occupy 61 of 64 hexagrams. Mean better is −0.008, mean worse −0.008, mean loss 0.108 at margin 0.1. The hexagram table is flat.

Two further numbers bear on the second reading. docs/74 §4 found persona fingerprint sectors bunched at 45–49, with histogram similarity 0.991 across authors. That is the same centroid skew that lets a constant guess score 12.8%.

What was not measured: the novelty arm with more than one seed per prompt; the novelty arm against a random mask of the same size; any gate other than `0b101010` in composition; the hexagram table on a Phi-4 quest; `hexagram_eval` on Austen. The source docs do not record the output of the binary's last block, the novelty of `borrow` under each single-line gate, so this paper gives no number for it.

## 5. The one idea

A word in phinet is sixty-four samples on a circle. The circle is the list of relation channels, and the value at each channel is a phasor. Such a list has a spectrum. Low modes describe how the word varies slowly from channel to channel. High modes describe how it varies from one channel to the next. The six lines of a hexagram are six switches, one per band of that spectrum. Flip a switch off and the word loses that band. Flip them all on and nothing happens. Flip them all off and the word is reduced to its average.

That is the whole of the first reading. It is a spectral filter bank with sixty-four settings, and the I Ching supplies the names. The names are convenient because they already come with an order of lines, a notion of opposite (all lines flipped), and a notion of change (some lines flipped). `changing_lines` is exactly that notion of change.

The second reading is separate. Add the sixty-four samples of a word together. The result is one complex number. Its angle is somewhere on the circle, and the circle cut into sixty-four sectors gives it a number from 0 to 63. That number is called the word's hexagram. The bits of a word's sector do not say which of its bands are strong. A word in sector 42 need not be strong on lines 1, 3 and 5.

The clearest example is the word "the" in Austen. Its sector is 42, which is `0b101010`. Gate 42 is the gate the Austen composer used for novelty. The word's line energies put 0.49 of its energy on line 4, and line 4 is one of the three lines gate 42 turns off. Read as a gate, the word's own hexagram removes the band the word is mostly made of. Its novelty under that gate is 0.414.

So the module holds two maps from words to sixty-four ids, and they share only the count. One is a question the composer asks of a word: what is left if these bands go. The other is a label the word carries: where its centroid points. The first is an operation and was kept. The second made a prediction and the prediction failed. It survives as a label for filing.

Both readings stay inside the paradigm. The gate acts on a word's relations, all sixty-four of them at once, and returns relations. The sector is read off those relations and nothing else. Neither stores a point for the word. Each is a way of looking at what the word relates to.

## 6. The gates, worked

The gates are best seen on single cases. The tests give the first three.

`creative_is_identity_and_receptive_is_the_mean` draws a random unit function of 64 samples with `unit_random(64, 3)`. Gate `0b11_1111` returns it unchanged to within 1e-12 on every sample. Gate `0` returns a constant function whose every sample equals the mean of the original, again to 1e-12. These are the two ends. The Creative is the identity. The Receptive is the projection onto the mean, one mode out of sixty-four.

`hexagram_round_trip_and_novelty` checks that `novelty(f, 0b11_1111)` is zero to 1e-12 and that `novelty(f, 0)` is above 0.5 for a random function: dropping five octaves and more is novel. It checks one walk of changing lines: `changing_lines(0b10_1010, 0b00_0011)` is `0b10_1001`. The bottom two lines of the first hexagram flip. Line 0 goes from yin to yang and line 1 from yang to yin. As gates, the first passes octaves 1, 3 and 5; the second passes octaves 0, 3 and 5.

The mode counts from section 3 say what each line costs. On 64 samples, line 4 governs 32 modes. Line 5 governs one. Line 0 governs two. So the six switches are very unequal. Turning off line 4 removes half the non-mean spectrum. Turning off line 5 removes a single mode, the Nyquist mode at 32. Gate `0b101010`, used in the Austen composer, keeps lines 1, 3 and 5. That is 4 + 16 + 1 = 21 modes, plus the mean: 22 of 64.

Because `apply` is an orthogonal projection and `cosine` is the real inner product over the norms, the novelty of a gate has a closed form. The inner product of `f` with its projection is the projection's energy. So `cos(f, H f)` is the square root of the fraction of `f`'s energy the gate keeps, and novelty is one minus that root. The line energies predict the novelty.

The Austen words in papers/15 §7 let this be checked by hand. For "elinor", lines 1, 3 and 5 carry 0.10, 0.19 and 0.02 of the non-mean energy, 0.31 together. One minus the root of 0.31 is about 0.44. The measured novelty under gate 42 is 0.434. For "marriage", the same lines carry 0.02, 0.36 and 0.03, 0.41 together, and one minus its root is about 0.36; the measured novelty is 0.360. The small gaps come from the mean, which the gate passes and `lines` leaves out.

This is what "novelty on demand" means in the code: an exact, predictable loss of chosen bands. The number is known before the gate is applied. The gate does not choose which words to change. It removes the same bands from every prediction it is given.

## 7. The hexagram a word is

`hexagram_of` is one line of arithmetic. The samples are summed, the angle of the sum is taken, and the angle is cut into sixty-four. The sum of a word's samples is the mode-0 coefficient of its spectrum. So the hexagram a word is depends only on the one mode every gate passes. No gate can change a word's sector. The two readings of section 5 are joined at this point and nowhere else.

papers/15 §7 gives six Austen words read through `bin/austen`. The King Wen column is `KING_WEN_MAP[id]` from `phiano::phinum::iching`. The lines are the octave energies, bottom line first. The last column is `novelty(f, 0b101010)`.

| word | Q64 id | King Wen | lines (octaves 0–5) | novelty of gate 42 |
|---|---|---|---|---|
| elinor | 38 | 17 | 0.06 0.10 0.08 0.19 0.56 0.02 | 0.434 |
| emma | 39 | 25 | 0.09 0.14 0.07 0.21 0.49 0.02 | 0.380 |
| love | 33 | 27 | 0.06 0.06 0.18 0.18 0.51 0.01 | 0.468 |
| marriage | 15 | 33 | 0.02 0.02 0.23 0.36 0.35 0.03 | 0.360 |
| said | 43 | 37 | 0.05 0.07 0.17 0.21 0.49 0.01 | 0.444 |
| the | 42 | 63 | 0.03 0.09 0.16 0.21 0.49 0.01 | 0.414 |

The lines column is close to the mode counts of section 3. Line 4 holds 32 of the 63 non-mean modes and carries 0.49 to 0.56 of the energy in five of the six words. Line 5 holds one mode and carries 0.01 to 0.03. A word whose energy were spread evenly over its modes would look much like this. The lines therefore say more about the octave widths than about the words. "marriage" is the one exception in the table, with more on line 3 (0.36) and less on line 4 (0.35).

Five of the six sectors sit between 33 and 43. This matches the skew docs/74 found twice: the most frequent hexagram alone scores 12.8% on the Rust Book, and persona sectors bunch at 45–49 with histogram similarity 0.991. A single angle from a sum of sixty-four phasors is a coarse summary, and on these lexicons it lands in a small arc of the circle.

The quest uses the same function differently. `function_of` in `quest_ingest/liskov.rs` builds a word's 64-channel function from the facet: a unit phasor at the word's source angle `p.theta(k)` on each channel with `evidence_count(k) > 0`, zero elsewhere, then `phinet::normalise`. Its hexagram is `hexagram_of` of that function. The magnitudes are dropped; only which channels have evidence, and at what angle, reach the sector.

## 8. The gates in composition

The gates reach generation through one field. `OperatorGuide` in `src/generate/guide.rs` carries `novelty: Option<(u8, usize)>`. Its doc comment reads: "every `every`-th emitted token the prediction is passed through the hexagram's spectral gate (`phinet::hexagram::apply`), so the candidates are read against a function with some of its octaves removed. `None` = no novelty." In the prediction path, after `self.fno.apply` returns `y`, the guide checks `self.history.len() % every == 0` and, when it holds, replaces `y` with `phinet::hexagram::apply(&y, hex)`.

`bin/austen` sets this in `src/bin/austen/sections.rs`. The constant is `novelty_hex: u8 = 0b10_1010`, and the third arm is `Some((novelty_hex, 4))`. Each arm loads `data/austen_operator.fno` against the Austen voice lexicon, with directed sides and a window of 6. The prompts include "Captain Wentworth had", "Elinor and Marianne", "Mr Knightley said", "The evening passed", "She could not help" and "Her mother was". Short compositions run 12 tokens and long ones 60.

The numbers in section 4 show what the gate does. On long compositions the evaluator rises from 0.669 to 0.689. Kneser-Ney perplexity under the Austen judge rises from 11.6 to 12.2. The distinct-token ratio falls from 0.727 to 0.704. On short compositions all three move by less: 16.2 to 16.0, 0.697 to 0.701, 0.990 to 0.979. docs/74 reads this as "novelty costs exactly what it should".

The same paper sets the scale for reading it. papers/15 §8 says: "One seed per prompt; eight prompts; treat differences under ~10% of perplexity as noise." The long-arm change from 11.6 to 12.2 is under 10%. By the paper's own rule it is noise. The evaluator change is small too, and no spread is reported for it. So the honest statement is that the gate runs inside generation without harm and moves the numbers in the direction the design expects, by amounts the experiment cannot separate from chance.

There is also a question the arm does not answer. Gate 42 removes line 4, the band holding about half of every prediction's energy, as section 7 shows. Any mask that removed half the spectrum would perturb the prediction by a similar amount. The arm has no control of that kind. It tells us a gate named by a hexagram behaves like a gate. It does not tell us the choice of hexagram matters.

The source gives no reason for every fourth token or for gate 42. Both are settings.

What the arm does show is the property that matters for a composer. The perturbation is exact, repeatable and named. A reader of the output can be told which tokens were read against a gated prediction, and which bands were gone.

## 9. The oracle, refuted

`bin/hexagram_eval` is the test of the second reading. Its doc comment states the question: "On held-out positions: how often does the predicted hexagram equal the true next word's, against chance (1/64), 'same as the previous word', and 'the next in the King Wen sequence'?"

The binary reads `data/rust_book_corpus.txt` by default. It splits sentences with `Tokenizer::split_sentences`, keeps those of three tokens or more, and splits them with `Harness::split(corpus, 42)`. It trains a facet with `Harness::train_ranking_only(&split, …, 4)`, builds `FunctionSpace::from_facet_sides(&facet, Sides::Directed)`, and loads the operator from `OPERATOR_FILE` with a window of 6. Every word in the table gets its hexagram once, in `hex_of_word`.

Then it walks the first 2,000 validation sentences. At each position `i` from 2, with both the previous and the next word in the space, it focuses the guide on the tokens so far and takes the prediction `y` from `predict_public`. It builds the plain context function from the same tokens with `space.context`. It records five hits: the prediction's hexagram, the context's hexagram, the previous word's hexagram, the King Wen successor of the previous word's hexagram, and, after the loop, the count of the most frequent true hexagram.

The King Wen successor maps the previous id to its number in `KING_WEN_MAP`, adds one with 64 wrapping to 1, and maps back through the inverse, `id_of_number`.

The operator's prediction lands in the right sector 0.9% of the time. That is below the 1.6% a uniform guess would get. The context function scores 5.8%, and the previous word 3.6%. Both beat chance: the sectors of nearby words are slightly correlated. The King Wen successor scores 1.1%. A constant guess, the most common sector, scores 12.8%.

The reason the operator does worse than the context is visible in the mechanism. The operator is trained to rank the next word by cosine over all sixty-four samples. The sector is the angle of one coefficient, mode 0. Nothing in the training asks mode 0 to point the right way. The operator can raise the cosine to the right word while turning its mean anywhere. The context function is an average of the words already there, so its mean is likely to sit where the corpus's means sit, in the crowded arc. This explanation was not measured.

The reason the constant does best is the skew. When most words' centroids fall in a few sectors, the most common sector is a strong guess. Any fair test of prediction by sector has to beat 12.8%. Beating 1.6% is not enough.

The conclusion in the docs is short and this paper keeps it. The hexagram of a word is a label and does not forecast the next label.

## 10. The hexagram as a filing system

The quest is where the second reading found a use. `src/bin/quest_ingest/liskov.rs` scores a quest with Liskov's three comparisons and files every word under its hexagram. The module's comment says why: "so a quest reports itself in sixty-four categories and the patterns that emerge are per hexagram, not per word."

`score_words(facet, rows)` takes each distinct `about` word. It builds `function_of` and records `hexagram_of(&f)` in a `WordScore`. For each row it finds the channel of the row's kind with `facet.marker_channel`. *Better* is the truth's fit, `facet.on(w, other, k)`, minus the best wrong word's fit on the same channel. *Worse* is the truth's fit minus the truth's strongest onward reading under any kind. *Loss* is the hinge `max(0, MARGIN − better)` with `MARGIN = 0.1`.

`liskov_report` writes two things: a TSV with one row per word, and a Markdown table with one row per occupied hexagram, giving the count of words and their mean better, worse and loss. It ends with a line of the form "61 of 64 hexagrams occupied by 256 words."

On the synthetic quest (docs/83) the table shows the flatness docs/86 names. Most occupied hexagrams read 0.000, 0.000, 0.100: the truth ties the best wrong word, and the loss equals the margin. A few depart. Id 2 holds 4 words at better −0.100 and loss 0.200. Id 23 holds 3 words at −0.133 and 0.233. Ids 5, 33 and 38 read −0.066, −0.040 and −0.050. Ids 54, 56 and 57 are empty.

docs/86 reads this as "the byte ceiling of docs/83 seen through the third operator". A channel is one byte, so fits that differ by less than a quantum round to a tie. When every fit ties, every hexagram's mean is the margin, and the filing has nothing to show. The table was built for a corpus that is not flat. That corpus is Phi-4's, and that run has not been made.

The helix files channels, not words, under hexagrams. `Codon::of(k)` in `src/helix/code.rs` gives channel `k` its codon, `k` in base four over `ACGT`, its hexagram glyph from `hexagram_glyph(k)`, its sphere degree and order from `phinet::sphere`, and its place on a B-DNA helix at 10.5 codons per turn and 0.34 nm rise. The test checks `codon(27)` is `CGT`, that positions 0 and 21 coincide in the plane, and that `hexagram_glyph(0)` is ䷀ and `hexagram_glyph(63)` is ䷿. docs/84 gives a worked reading: on the quest's helix, `w0` is active on `AGA` (is-a), `AGC` (near), `AGG` (unlike), `AGT` (part-of) and `ATA` (used-for), channels 8, 9, 10, 11 and 12, plus the first offset channel.

## 11. What it depends on, and what depends on it

The module lives in phinet, the parent engine. It depends on three things there. `tensor::fft_inplace` supplies the unitary FFT. phinet as a whole passes 62/62 parity tests against the PyTorch reference; the docs read here do not say which of them touch the FFT. `operator::cosine` supplies the score that `novelty` uses. The crate root supplies `c64`, `Function`, `normalise` and `unit_random`. docs/85 lists hexagram gates among phinet's parts, beside the tensor, tape, FFT, `Fno`, sphere, joint training and HRR.

Phiano is the main user. `src/generate/guide.rs` holds the novelty field of `OperatorGuide`. `bin/austen` runs the novelty arm and prints the word readings. `bin/hexagram_eval` runs the oracle test. `bin/quest_ingest` files quest words by `hexagram_of`. `phiano::helix` prints a hexagram per channel. `phiano::phinum::iching` holds `KING_WEN_MAP` and `archetype_name`, which `bin/austen` and `bin/hexagram_eval` use to print King Wen numbers. docs/85 also records that phiano's older `src/cognitive` has "I Ching syntax variants" that "duplicate `phinet::hexagram`", and proposes keeping only the variants table if `bin/hexagram_eval` reads it.

phinum is the child that already speaks I Ching. Its `iching` module carries the `Hexagram` type, the King Wen mapping and the sixty-four names. It is also where one of the conventions in section 13 lives.

phical, the geometry of the relation circle, meets the module at the juncture. The Q64 sector is `phi_cal::q64`, and docs/69 tied phiano's `Hexagram::phase_angle` to `phi_cal::q64::lift` with the half step that `phase_of` also uses. The phical crate sits in the phiace workspace at `phiace/crates/phical`.

phifno and the sphere share the count. On the sphere, 64 = 8² channels are harmonics `Y_l^m` with `l < 8`. `phinet::sphere` already has a degree gate, which docs/84 calls "resolution change on the sphere". That is the sphere's own filter bank, with eight degrees where the circle has six octaves. The helix prints both readings side by side for each channel. No sphere version of the hexagram gates was built.

phiton reads each codon of the helix as light: a wavelength, a band and a hex colour per codon (PHIENT_FACTS.md). It does not use the gates.

phiecs owns the quest oracle whose words the Liskov report files. The quest is synthetic: 256 words, families of 8, five kinds, 1,280 rows. Its flatness is the reason the hexagram table is flat.

phiqlm is planned and nothing is built, so nothing depends on it here.

Nothing downstream depends on the gates for a measured result that stands. The novelty arm is optional and ships off unless a binary sets it. The quest's hexagram table is a report. Removing the module would remove an option and a label, and no headline number would change.

## 12. The next experiment

The next experiment should ask the one question the Austen arm left open: does the choice of hexagram matter, or does any mask of the same size do the same?

The design uses what exists. The decode harness, `bin/decode_eval`, already runs 40 held-out prompts for 30 tokens each and reports Kneser-Ney perplexity, the evaluator, the distinct-token ratio and the share of 4-grams repeated across prompts (docs/86 §2). Its greedy operator arm, nearest-k at k = 32, reads 66.9. The novelty field on `OperatorGuide` is independent of the decoder, so the gate can be set on that arm without new code in the guide.

The arms would be these. The baseline is the operator nearest-k arm with `novelty = None`. The treatment is the same arm with `novelty = Some((0b101010, 4))`. The control is the same arm with a random mode mask applied every fourth token, drawn once per run, keeping the mean and 21 other modes, so that it passes the same 22 of 64 modes gate 42 passes but ignores the octaves. A second treatment would use a gate that keeps line 4, for example `0b010101`, which passes octaves 0, 2 and 4: 2 + 8 + 32 = 42 modes plus the mean. It removes much less energy, so its effect should be smaller, and the closed form in section 6 says by how much before the run.

The measure is the same four numbers per arm, with more than one seed per prompt so a spread can be given. The prediction written down in advance is simple. If the octave structure matters, gate 42 and the matched random mask will differ by more than the seed spread. If only the amount removed matters, they will not. The closed-form novelty can be computed for every prediction in every arm, so the two arms can also be compared at equal novelty.

A second, smaller experiment tests the oracle claim fairly. `bin/hexagram_eval` compares against chance at 1/64. The right floor is the constant guess at 12.8%. A predictor worth reporting must beat that. A useful variant would rotate every word's function so that its sector histogram is flat before the test, which removes the skew and leaves only the relative angles. This has not been designed in detail.

Before either run, the labels in section 13 should be made consistent, so the tables the runs print carry the right glyphs and King Wen numbers.

None of these runs has been made. The random-mask control and the rotated-sector variant do not exist in the code. The prediction above is a design. It has no result yet.

## 13. What is wrong or missing

The first problem is the labels. `phinet::hexagram` says line 0 is the bottom line, ䷀ is 111111 (id 63) and ䷁ is 000000 (id 0). The other code disagrees in three ways.

| place | what it does | what it gives for the ids in question |
|---|---|---|
| `phinet/src/hexagram.rs` | line 0 is the bottom line | ䷀ = id 63, ䷁ = id 0 |
| `phinum/iching/table.rs`, `KING_WEN_MAP` | reads bit 0 as the top line | `[1]` = 23, Splitting Apart, one yang line at the top; `[32]` = 24, Return, one yang line at the bottom |
| `helix/code.rs`, `hexagram_glyph(k)` | Unicode `0x4DC0 + k`, King Wen order | `hexagram_glyph(0)` = ䷀ |
| `quest_ingest/liskov.rs`, `glyph(id)` | the same | id 0 printed as ䷀ in docs/83 |
| `bin/austen/sections.rs` | names gate `0b101010` "䷘, King Wen 25" | `KING_WEN_MAP[42]` = 63; in docs/83, ䷘ is id 24 |

So ䷀ means id 63 in phinet and id 0 in the helix and the quest report. The doc comment on `hexagram_glyph` says the glyph shows the channel's "six bits as lines, bottom first", in the same reading `phinet::hexagram` gives, and docs/84 says "the hexagram is `k`'s six bits". The code draws neither. Within `KING_WEN_MAP` itself, reversing the six bits of 42 gives 21, and `KING_WEN_MAP[21]` is 64: under phinet's bottom-first reading, gate 42 is King Wen 64, under the table's reading it is 63, and sections.rs calls it 25. In the word table of papers/15, King Wen 25 is the number the map gives "emma", id 39. The King Wen successor in `hexagram_eval` was computed under the table's top-first reading. Whether a bottom-first reading changes its 1.1% is not measured.

The second problem is the octave widths. Line 5 governs one mode on a 64-sample circle and line 4 governs 32. The lines column of section 7 mostly reflects this. A gate bank with equal bands, or with the sphere's eight degrees, was not built or compared.

The third is the sector. It is the angle of one coefficient, and on these lexicons it bunches. `hexagram_of` of a function whose samples sum to zero returns id 0, since the angle of zero is taken as 0. No caller checks for this. The quest skips only functions with no energy at all.

The fourth is a count. docs/74 says "15 phinet tests" when the module landed; the file holds three test functions.

The fifth is `papers/05`. Its four octaves of sixteen layers, its sector counts, its top-down modulation and its `resonate_depth_64` call exist nowhere in the code, and it reports no measurement.

## 14. How it was found

The history runs through the numbered docs and the papers folder, and it ends where the paradigm says it should: with a measurement that refuted one reading and kept the other.

The earliest sixty-four in the repository is `papers/05`, "The 64-Layer Cognitive Octave Continuum: Multi-Scale Phase Manifolds in Phiano". It proposed four octaves of sixteen layers each, from phonemes at the bottom to "Persona Identity & Epistemic Stance" at the top, with sector counts shrinking from 1024 to 4 and a top-down phase potential to settle ambiguous words like "bank". It gave equations and a code sketch. Nothing was built from it and nothing in it was measured. It is the aspiration the later work replaced with something small enough to test.

phinum's `iching` module holds the older I Ching code. It carries `KING_WEN_MAP`, the sixty-four archetype names, and a `Hexagram` type with a `phase_angle`. The docs read here do not date it against the gates. docs/69 records the half-step fix to that `phase_angle`, `2π(q+½)/64`, proved equal to `phi_cal::q64::lift`. That is the juncture `hexagram_of` and `phase_of` use.

docs/74 §3 introduced `phinet::hexagram` as it stands: six lines gating six octaves of a 64-sample spectrum, with `apply`, `gate`, `changing_lines`, `hexagram_of`, `phase_of`, `novelty` and `lines`. The same section reported `bin/hexagram_eval` and its refutation, and the novelty field on the guide, then in `generate.rs`. docs/74 §4 put the gate into the Austen persona and found the persona sector skew at 45–49.

papers/15 gave the full Austen run: §6 the three composition arms with gate `0b101010` every fourth token, §7 the six word readings, §8 the noise rule of about 10%.

docs/75, "where we are", entered the module in its status table with the verdict "yes as operations; no as oracle".

docs/83 and docs/86 brought the hexagram into the quest. docs/86 introduced Liskov's three operators in `phinet::liskov`, used them in `quest_ingest/liskov.rs`, and filed the 256 quest words by hexagram: 61 of 64 occupied, flat. docs/83 carries the full per-hexagram table.

docs/84 set the hexagram beside the other sixty-fours, the channels, the spherical harmonics and the codons, and built the helix, which prints a hexagram for every channel. docs/85 placed the hexagram gates in phinet's list of parts and noted the duplicate I Ching code in `src/cognitive`.

This paper added one finding by reading the code: the three label conventions of section 13. It ran nothing new. Its checks of novelty against line energies in section 6 are arithmetic on numbers papers/15 already printed.

## 15. What remains

The first task is the labels. One convention should hold across the family, and phinet's is the one the gates obey: line 0 is the bottom line and the lowest octave. `hexagram_glyph` and `glyph` should draw the hexagram whose lines are the id's bits, which means looking up the King Wen number first and then the Unicode character, as phinum's own glyph code already does from a King Wen number. `KING_WEN_MAP` should either be re-indexed bottom-first or documented as top-first with a function that converts. The name "䷘, King Wen 25" in `bin/austen` should be corrected to whatever the fixed map gives for id 42. docs/83 and papers/15 would need their tables regenerated. None of this changes a measured number except, possibly, the King Wen successor score.

The second is the control of section 12. The Austen arm showed the gate is harmless and moves the numbers the expected way, within noise. A matched random mask and several seeds would say whether the octaves matter. Until then, novelty on demand is a working feature with no evidence that its structure helps.

The third is the fair oracle test. The floor should be 12.8%, the constant guess, and a skew-free variant should be tried. The operator was never trained to place a word's mean. If the second reading is to predict anything, a model would have to be trained to predict the sector itself, and none has been.

The fourth is the quest table on a real corpus. docs/86 names it: the hexagram table on a Phi-4 quest. No Phi-4 run has been made. Until the quest is not flat, the per-hexagram report cannot show a pattern.

The fifth is the band question. Six unequal octaves are what a power-of-two circle gives. The sphere offers eight degrees with `2l+1` harmonics each, and `phinet::sphere` already has the degree gate. Whether a gate bank on degrees, or on equal bands of the circle, does better as novelty is open. No such bank was built for this comparison.

The sixth is housekeeping. The duplicate I Ching code in `src/cognitive`, which docs/85 flags, should be reduced to what `bin/hexagram_eval` reads. `papers/05` should carry a note that it describes an unbuilt design, so no reader takes `resonate_depth_64` for a function in the tree.

What does not remain is the oracle as it was first posed. It was measured on the Rust Book at 0.9% against chance at 1.6%, and the docs closed it. The sixty-four gates stay as operations: exact, cheap, named and composable. The hexagram a word is stays as a label for filing, and a label is all it has been shown to be.

## 16. Sources

These are the local files read for this paper. Every number above comes from one of them or from PHIENT_FACTS.md.

Six sources were assigned: `hexagram.rs`, docs/74, `papers/05`, `helix/code.rs`, `quest_ingest/liskov.rs` and PHIENT_FACTS.md. The others were read to check a name, a number or a claim those six make. docs/86, docs/83 and papers/15 hold the measured tables the assigned docs cite. `hexagram_eval.rs`, `sections.rs` and `table.rs` were read to see how the King Wen numbers are computed, which is where the label conflict of section 13 was found. `guide.rs`, `sampler.rs` and `lib.rs` were read to confirm where the gate enters generation and what `cosine` and `normalise` compute. No binary was run for this paper and no source file was changed. The arithmetic in section 6 uses only numbers printed in papers/15.

| path | what it gave |
|---|---|
| `/home/claude/phiano/docs/papers/phient/BRIEF.md` | the rules for this paper |
| `/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` | the paradigm paragraph; the family; parity, quest, helix and Liskov numbers |
| `/home/claude/phinet/src/hexagram.rs` | `octave`, `gate`, `apply`, `changing_lines`, `hexagram_of`, `phase_of`, `novelty`, `lines`, and the three tests |
| `/home/claude/phiano/docs/74_pair_key_lovelace_hexagrams_austen.md` | §3 the module and the oracle result; §4 the Austen arm and the sector skew |
| `/home/claude/phiano/docs/papers/05_THE_64_LAYER_COGNITIVE_OCTAVE_CONTINUUM.md` | the unbuilt 64-layer design |
| `/home/claude/phiano/src/helix/code.rs` | `codon`, `hexagram_glyph`, `helix_position`, `Codon::of`, and its tests |
| `/home/claude/phiano/src/bin/quest_ingest/liskov.rs` | `function_of`, `score_words`, `liskov_report`, `glyph`, `MARGIN` |
| `/home/claude/phiano/docs/86_liskov_three_operators.md` | the three operators; decode arms; the quest filed by hexagram |
| `/home/claude/phiano/docs/83_quest_ingest_table.md` | the per-hexagram quest table |
| `/home/claude/phiano/docs/84_sphere_and_helix.md` | the four sixty-fours; the helix layout; `w0`'s codons |
| `/home/claude/phiano/docs/69_two_sides.md` | the half-step fix and `phi_cal::q64::lift` |
| `/home/claude/phiano/docs/75_where_we_are.md` | the status row for `phinet::hexagram` |
| `/home/claude/phiano/docs/85_PML_STRUCTURE.md` | phinet's parts; the duplicate I Ching code in `src/cognitive` |
| `/home/claude/phiano/docs/papers/15_JANE_AUSTEN_PERSONA.md` | §6 composition arms; §7 word readings; §8 the noise rule |
| `/home/claude/phiano/docs/papers/phient/07_the_helix.md` | the earlier finding on the helix glyph |
| `/home/claude/phiano/src/bin/hexagram_eval.rs` | the oracle test's procedure and the King Wen successor |
| `/home/claude/phiano/src/bin/austen/sections.rs` | the novelty arm's settings and the "King Wen 25" label |
| `/home/claude/phiano/src/phinum/iching/table.rs` | `KING_WEN_MAP` and the archetype names |
| `/home/claude/phiano/src/generate/guide.rs` | `OperatorGuide.novelty` and where the gate is applied |
| `/home/claude/phinet/src/operator/sampler.rs` | `cosine`, the real inner product over the norms |
| `/home/claude/phinet/src/lib.rs` | `normalise` |
