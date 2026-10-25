# The Operator on Austen
*Phient papers, 5 of 16 · 2026-03-09 · The operator on Austen: from hinge to softmax, 14.7× to 30.7×, and what decoding from it sounds like*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

This paper reports one sweep and what it did to the text. The operator is phinet's Fourier neural operator, `phinet::operator::Fno`, trained in Phiano to move a word's function toward the function of the word that follows it. The corpus is Austen: Emma, Persuasion, and Sense and Sensibility, split with seed 7. The pool is the 1,228 words seen 20 or more times. There are 204,592 training examples, 2,000 validation examples and 2,000 test examples. Chance mean reciprocal rank (MRR) over the pool is 0.0063. Reading with no operator at all, the identity, scores 7.9× chance on validation and 8.1× on test.

The shipped operator was trained by hinge against one decoy. At 8,360 parameters and three epochs it reached 14.7× chance on validation and 14.6× on test. Changing only the objective, to cross-entropy over the whole table at β = 8, gave 18.0× and 18.8× on the same parameters. Making the operator wider and deeper on that objective, at 50,016 parameters, gave 27.6× at three epochs. Continued to eight epochs it reached 30.7× on validation and 30.3× on test, with a test hit@10 of 0.348. That operator ships as `data/austen_operator.fno`.

The second half of the paper is decoding. `bin/decode_eval` writes 30 tokens after each of 40 held-out prompts, with k = 32, and scores the result under a Kneser-Ney model built on the training split. With the hinge operator the operator arm read 97.2. With the eight-epoch softmax operator it reads 66.9, and the union arm 66.8. The trigram sampler reads 13.3. Austen's own held-out sentences read 187.5. A lower number means the text is closer to what a trigram judge expects, so none of these numbers is a score of quality on its own.

The sampler repeats itself across prompts. Of its 4-grams, 25.7% also occur in another prompt's continuation. The operator's share is 8.7%. The operator writes from the function space and repeats a third as much, and it is still far from Austen's fluency. The last five epochs of ranking, 27.6× to 30.7×, bought seven points of perplexity. The ranking is no longer the whole ceiling.

The paper names the types and functions involved, gives the tables, quotes the only two fragments of generated text the sources record, lists where the sources disagree, and names the next experiment with its control. Nothing was run for this paper. Every number is from docs/71, docs/76, docs/78, `data/sweep/results.md`, the phinet guide, or PHIENT_FACTS.md.

## 2. The claim

The claim has three parts, and each is a measured row.

The first part is that the objective was the larger lever at fixed size. The hinge arm and the softmax arm share everything: the lexicon, the pool, the split, the seed, the batch of 64, three epochs, Adam, and 8,360 parameters. They differ in the loss. The hinge compares the operator's output with its target and with one decoy drawn uniformly from the pool. The softmax compares it with all 1,228 target sides at once. The hinge reached 14.7× valid and 14.6× test. The softmax reached 18.0× and 18.8×. Test hit@10 rose from 0.216 to 0.246. docs/78 adds a comparison across epochs: the shipped hinge operator at eight epochs was 17.4× valid, 17.1× test, hit@10 0.248. Three epochs of softmax already pass eight epochs of hinge.

The second part is that size then pays, on that objective. Width 16, three layers and 16 modes raise the operator to 50,016 parameters. On cross-entropy over the table it reached 27.6× on both validation and test at three epochs, with a test hit@10 of 0.335. That is a third of held-out targets in the top ten of 1,228 words. Resumed from its epoch-3 checkpoint and run to eight epochs, it reached 30.7× valid and 30.3× test, hit@10 0.348. The validation curve over epochs 3 to 8 was 27.6, 29.3, 29.7, 29.8, 30.7, 30.7. It is flattening.

The third part is about the text, and it is weaker. Before the sweep, docs/76 wrote that the new ceiling on the composer was the operator's ranking quality. The sweep tested that sentence. Replacing the hinge operator with the wide+deep softmax operator moved the operator arm's Kneser-Ney perplexity from 97.2 to 73.6 at three epochs and to 66.9 at eight. That is a third of the way toward the trigram sampler. The step from 27.6× to 30.7× was worth 73.6 to 66.9, seven points. docs/78 draws the conclusion in one line: the ranking is no longer the whole ceiling.

The claim stops there. The operator's text is not claimed to be good. The evaluator barely moves across arms: 0.671, 0.673, 0.670. The text is not claimed to be Austen's either. Her own sentences read 187.5 under the same judge, and every generated arm sits nearer the judge's expectation than she does. Softmax is not claimed to beat hinge in general. The scope is this corpus, this pool, this seed and these sizes. One seed was run per arm and no arm was repeated. The claim is the table in section 4 and nothing wider.

## 3. The mechanism

The operator is `phinet::operator::Fno`, the one-dimensional periodic form of the Fourier neural operator of Li et al. (2020). A lift takes the single input function to `width` features. Then each layer does this:

```
h ← σ( F⁻¹( R · F(h) )  +  W h + b )
```

`F` is the unitary FFT along the sample axis. `R` is a learned complex `width × width` matrix per kept Fourier mode, a filter in the spectral domain. `W` is a pointwise linear map across the width. `σ` is GELU on real and imaginary parts. A projection brings the result back to one function. Keeping only the low `modes` on each side of zero frequency forces the operator to be smooth over the circle. That is the regulariser the guide names.

The configuration type is `FnoConfig`. The guide's default is `space: 64, width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3`, and with `space = 64, width = 8, layers = 2` the operator has 8,360 parameters. That is the hinge arm's count. The wide+deep arms use width 16, three layers and 16 modes, and have 50,016 parameters.

The input is a word's function from the shipped voice lexicon, read directed. The table is the target sides of the 1,228 pool words. `Tape::cosine_table` scores the operator's output by cosine against every row. The two objectives read those scores differently:

```
hinge   (Fno::train_epoch)           loss = max(0, margin − cos_target + cos_decoy)
softmax (Fno::train_epoch_softmax)   loss = −log softmax(β · scores)[target],  β = 8
                                     Tape::cross_entropy
```

The hinge draws one decoy uniformly from `pool`. The softmax uses every row. docs/76 records the softmax path as built with a finite-difference test, PyTorch parity 0.0, and a test that it learns a shift. The optimiser is `Adam`. `Adam::with_decay` gives AdamW, `AdamState` holds the moments, and `Schedule::{Constant, Cosine, Step}` sets the learning rate. docs/76 records AdamW parity with torch at 0.0.

Decoding lives on Phiano's `Generator`. `Decode::Trigram` proposes candidates from the n-gram tables. `Decode::Operator{k}` proposes the k nearest target sides to the operator's prediction, through `OperatorGuide::nearest`. `Decode::Union{k}` merges both lists. In every arm the guide then re-weights each candidate by its cosine against the prediction. docs/71 gives that re-weighting as `(1 − w) + w·(1 + cos)/2`. That is why the trigram arm's number moves when the operator changes. `OperatorGuide::anchor` adds the prompt's mean function to each step's context with weight α, and `PHIANO_ANCHOR` sets α from the environment. It ships at α = 0.

The winner is saved as `data/austen_operator.fno` in format v2. `bin/decode_eval` and `OperatorGuide` read that file. Every sweep arm checkpoints to `data/sweep/<arm>.ckpt` with its optimiser moments and step count.

## 4. What was measured

`bin/operator_sweep` ran every arm for three epochs, batch 64, seed 7. The best epoch was chosen by validation MRR and test was scored once on that epoch. Each arm differs from its neighbour in one thing. Chance MRR is 0.0063. Identity is 7.9× valid and 8.1× test. The table is docs/78's.

| arm | params | objective | optimiser | best epoch | valid ×chance | test ×chance | test hit@10 | s |
|---|---|---|---|---|---|---|---|---|
| hinge (shipped) | 8,360 | hinge vs one decoy | Adam | 3 | 14.7× | 14.6× | 0.216 | 142 |
| softmax β=8 | 8,360 | cross-entropy over the table | Adam | 3 | 18.0× | 18.8× | 0.246 | 395 |
| softmax, wide+deep | 50,016 | cross-entropy over the table | Adam | 3 | 27.6× | 27.6× | 0.335 | 1,157 |
| softmax, wide+deep, AdamW+cosine | 50,016 | cross-entropy over the table | AdamW 1e-3, warm-up + cosine | 3 | 24.5× | 24.3× | 0.311 | 984 |
| softmax, wide+deep, continued to 8 | 50,016 | cross-entropy over the table | Adam | 8 | 30.7× | 30.3× | 0.348 | 3,189 |

The decoding harness is `bin/decode_eval`: 40 held-out prompts, 30 tokens each, k = 32. The judge is a Kneser-Ney model on the training split. The table shows docs/78's numbers, with docs/76's hinge-operator column for comparison.

| arm | KN, 3-epoch → 8-epoch operator | evaluator | distinct | novel bigrams | docs/76 KN |
|---|---|---|---|---|---|
| trigram sampler | 13.9 → 13.3 | 0.671 | 0.884 | 0.0% | 13.6 |
| operator nearest-k | 73.6 → 66.9 | 0.673 | 0.889 | 0.4% | 97.2 |
| union | 67.5 → 66.8 | 0.670 | 0.897 | 0.3% | 86.2 |

Her own held-out sentences score 187.5 under the same judge. Repetition across prompts was measured as the share of a continuation's 4-grams that also occur in another prompt's continuation, over all 40 prompts.

| arm | shared 4-grams |
|---|---|
| trigram sampler | 25.7% |
| operator nearest-k | 8.7% |
| union | 11.3% |

Three readings follow from the tables. The objective moved the 8,360-parameter operator from 14.7× to 18.0× on validation. Size on that objective moved it to 27.6×, and five more epochs to 30.7×. The operator arm's perplexity fell from 97.2 to 66.9 while its distinct-token ratio, 0.889, rose above the sampler's 0.884.

## 5. The one idea, restated

A word in Phiano is a function on a closed circle. It is what it relates to under a condition. The operator is one condition made into a map: the condition of standing one position earlier than another word. It takes the function of a context and returns a function that should lie near the target side of the next word. The table is where those target sides live. Ranking asks one question of the map. Among the 1,228 target sides, how near the top does the true next word come?

The decoder asks the same question. `OperatorGuide::nearest` takes the prediction and returns the 32 nearest rows of the table. Those rows become the candidates the generator chooses from. A decoding step is a ranking over the whole table, cut at k. So the objective that trains the operator and the use that decodes from it can be compared directly.

The hinge asked a smaller question. It asked whether the target is nearer than one word drawn at random from the pool, by at least a margin of 0.2 in cosine. Once that holds for most random words, the example stops producing gradient. The words that crowd the target at decoding time are the near rivals, and a uniform decoy is rarely one of them. The softmax asks the decoder's question every step. It pushes the target up against all 1,228 rows at once, weighted by β = 8, so the rows that score close to the target carry the most gradient. This is a reading of the two losses as written. The sweep did not measure which rows drew the gradient.

The idea, then, is one sentence. Train the operator on the question the decoder asks. On the same 8,360 parameters, doing that moved validation from 14.7× to 18.0×. Once the question is right, capacity pays: 50,016 parameters reached 27.6× in the same three epochs, and 30.7× at eight.

The paradigm says meaning is a side effect of pairs of words moving under a condition. Here the pair is a context and the word that follows it. The condition is the next position. The side effect that can be measured is a rank. A rank does not say whether a sentence sounds like Austen, which is why the second half of this paper exists. The rank moved from 14.7× to 30.7× across the sweep. The text improved by a third of the distance to the sampler, by one judge. Those are different quantities and the paper keeps them apart. Whether a better ranker makes better prose is the question sections 9 and 10 answer as far as the sources allow.

## 6. Hinge and softmax, in depth

Take one training example in words. docs/78 records an attractor phrase from the samples: "morning to think of some time for it". Suppose the context is "to think of some" and the target is "time". The phrase is used here only to name the parts. The sources record no scores for it.

Under the hinge, `Fno::train_epoch` maps the context function through the operator and compares the output with the target side of "time" and with one decoy side drawn uniformly from the 1,228 pool words. If the decoy is a word unrelated to the context and the cosine to "time" already leads by 0.2, the loss is zero and nothing moves. The decoy that would matter is a word that could also follow "of some", and a uniform draw finds such a word rarely. On this reading the hinge teaches the operator to beat the pool on average. The sweep did not measure it.

Under the softmax, `Fno::train_epoch_softmax` scores the output against all 1,228 target sides with `Tape::cosine_table`, multiplies by β = 8, and takes `Tape::cross_entropy` at the index of "time". Every row contributes to the normaliser. The loss is never zero, so every example keeps teaching.

The price is compute. The three-epoch hinge arm took 142 s of wall clock and the softmax arm 395 s. docs/78 also says a full-table softmax epoch costs 14 minutes here. Section 13 takes up that disagreement.

Decoys were tried before. The objectives guide records `Fno::train_epoch_with`, which takes any `Fn(target, seed) -> usize` as a sampler, and `FrequencySampler::new(items, weights, alpha)`, which draws decoys in proportion to `weight^alpha`. Frequency-proportional decoys made the operator worse as a language-model term: perplexity went 149 → 154 → 175 as `alpha` rose. The guide concludes that ranking inside the frequent pool is not what a full-vocabulary softmax rewards.

A third objective exists, and it is not the one swept here. `Fno::train_epoch_mixture` trains the operator on a mixed reader's own likelihood, with the trigram and unigram as fixed per-position constants carried in `MixtureExample { input, target, k, coef, other }`. The same operator that read 149 under the hinge read 134 under that loss. It costs about twenty times the hinge. That measure is perplexity over the whole vocabulary on another harness. It cannot be set beside the Austen MRR column.

The sweep chose pure cross-entropy over the pool's table. It is the loss that matches the ranking harness and the decoder's candidate step. The guide's warning on early stopping still applies. An 8,360-parameter operator on 100k examples overfits between epochs two and four. On Austen's 204,592 examples every three-epoch arm had its best validation at epoch 3.

## 7. Wide, deep, and the schedule

The 8,360-parameter operator has width 8 and two Fourier layers, keeping 12 modes each side of zero. The wide+deep arms double the width to 16, add a third layer and keep 16 modes. That gives 50,016 parameters against 8,360. The guide explains what each part buys. Width is the number of features each Fourier mode mixes through its complex matrix `R`. Modes are how much high-frequency structure on the circle the filter may use. Layers are how many times the filter and the GELU compose.

On the softmax objective the extra parameters paid at once. At three epochs the wide+deep operator read 27.6× valid and 27.6× test, with hit@10 0.335. The 8,360-parameter softmax arm read 18.0× and 18.8× over the same examples. The sweep did not run wide+deep under the hinge. How much of the jump belongs to size alone is not measured.

The guide gives a related control that is measured, on a corpus it does not name. `modes = 0` removes the spectral path, so nothing can move between samples on the circle. In Phiano's measurements the gap between that pointwise control and the full operator was 2.8× → 11.6× chance on a flat function space and 18.9× → 27.9× on an evidenced one. In those measurements the spectral filter accounts for the gap. The Austen sweep has no `modes = 0` arm.

The fourth arm tested the optimiser. It used AdamW at 1e-3 with a warm-up and a cosine decay, built from `Adam::with_decay` and `Schedule::Cosine`. It reached 24.5× valid and 24.3× test, below plain Adam's 27.6×. docs/78 explains this from the schedule's shape: the warm-up spends the first epoch at a lower rate, and three epochs are too few to pay that back. docs/78 also says the result is not evidence against the schedule at eight epochs. That run has not been done.

The fifth arm resumed the wide+deep Adam arm from its epoch-3 checkpoint and ran to epoch 8. Validation went 27.6, 29.3, 29.7, 29.8, 30.7, 30.7. Test at the best epoch was 30.3×, hit@10 0.348. The last step added nothing on validation, so the curve is flattening. The eight-epoch operator is the one that ships.

docs/78 notes that validation and test agree closely in every row, and takes that as evidence the number is not an artefact of picking the best epoch on validation. The widest gap in the table is the softmax β=8 arm, 18.0× against 18.8×, and there test is the higher number. A selection artefact would push validation above test.

## 8. How the sweep was run

The sweep's main difficulty was time. docs/78 says a full-table softmax epoch costs 14 minutes on this machine, and the two wide arms about 20. So `bin/operator_sweep` was made resumable.

Each arm checkpoints every quarter-epoch to `data/sweep/<arm>.ckpt`, in save format v2. The checkpoint holds the operator, the optimiser moments and the step count. The run stops when its time budget is spent and continues on the next call. An epoch's example order is fixed by its seed, and the quarters are consecutive slices of that order, so the arithmetic is that of one uninterrupted run. The `s` column is the sum of wall clock over the quarters. A finished arm becomes a row in `data/sweep/results.md`.

The format work that made this possible is in docs/76. `to_bytes` gained a version byte: format 2 is `PHINET` plus a version, and v1 still loads. Format 3 stores f32 through `to_bytes_compact`, at half the size and under 1e-10 drift. `to_bytes_with` saves the optimiser state alongside the weights. The continued eight-epoch arm depends on exactly that: it resumed from the epoch-3 checkpoint of the wide+deep Adam arm with its moments intact.

Two engine changes made the sweep affordable. The first is in `Tape::cosine_table`. Its backward pass no longer computes a gradient for a side that needs none. While the operator trains, the table of target sides is a constant, so the gradient into it was pure waste. The second is a finding, and nothing has been changed for it yet. The profile that found the waste showed the engine is allocation-bound at these sizes. PHIENT_FACTS.md gives the scale: at these sizes a 1 MB operation takes 2–5 ms. docs/76 records an earlier speed-up to `apply`, by slice loops and chunking, from 916 to 88 µs per function, and a hinge epoch from 3.0 to 1.8 s on its own benchmark. A real batched complex GEMM is still open, and docs/76 puts about 10× more on `apply` behind it.

The outputs of the sweep are three kinds of file. `data/sweep/<arm>.ckpt` holds each arm's resumable state. `data/sweep/results.md` holds the finished rows. `data/austen_operator.fno` holds the winner, format v2, 50,016 parameters. The decoder reads the last one, so replacing it changes every decoding arm at once. That is how docs/78 swapped the hinge operator for the softmax one without touching `bin/decode_eval`.

The same results file also carries three sphere rows on the same harness, which docs/78 does not mention: the sphere tie under hinge and under softmax β=8, and the anisotropic tie under hinge. Section 11 reports them.

## 9. What decoding sounded like before the sweep

The operator did not reach the text on its first attempt. docs/71 measured generation on the Rust Book, with 60 held-out prompts and 24-token continuations at temperature 0.15. The judges were a Kneser-Ney model on the training split, the engine's own `Evaluator`, and the distinct-token ratio. At that stage `OperatorGuide` only re-weighted the n-gram's candidates, by `(1 − w) + w·(1 + cos)/2`.

| arm (docs/71) | KN ppl | eval | distinct | tokens |
|---|---|---|---|---|
| unguided | 13.6 | 0.730 | 0.966 | 20.0 |
| untrained operator (control) | 13.4 | 0.725 | 0.961 | 20.0 |
| directed operator, weight 0.5 | 13.7 | 0.729 | 0.957 | 20.0 |
| directed operator, weight 1.0 | 14.2 | 0.729 | 0.953 | 19.9 |

Nothing moved. The candidates were the n-gram's top 12 to 16 continuations, and at that temperature the count term decided among them. A factor between 0.5 and 1 did not change which candidate won. docs/71 reads the 13.6 against 132 on real held-out text as the measure of how much that generator was a trigram sampler.

So the operator was allowed to propose. Its eight nearest target sides among words seen 10 or more times were merged into the candidate list with a middling pseudo-count. The trained operator proposed about sixty times better than an untrained one of the same shape: 181.1 at weight 0.5 against 10,728.4 for the control. The text broke. The one sample recorded is "to start well → Parameter to we in." Generation stopped at about eleven tokens (10.9 and 11.1). The nearest target sides to any context were the frequent function words and a few high-evidence nouns. Given a pseudo-count they won, and the n-gram had no continuation from them. `OPERATOR_PROPOSALS` shipped at 0. docs/71 proposed a prior-corrected score and a frequency-decayed pseudo-count. Neither is measured in the sources read here.

docs/76 then built decoding from the operator as its own arm. `Decode::Operator{k}` takes the k nearest rows through `OperatorGuide::nearest`. The corpus was Austen, 40 prompts of 30 tokens, k = 32. The trigram sampler read 13.6, the operator arm 97.2, the union 86.2, and her own sentences 187.5. The evaluator read 0.679, 0.682 and 0.678. Distinct read 0.867, 0.802 and 0.818. Novel bigrams were 0.0%, 0.2% and 0.2%. docs/76 put it in two sentences. The composer now writes from the function space. The text is no longer stitched and not yet hers. It named the new ceiling as the operator's ranking quality, at 16.1× chance. That sentence set up the sweep in docs/78.

## 10. What decoding sounds like after it

docs/78 replaced the hinge operator with the wide+deep softmax operator and changed nothing else in `bin/decode_eval`. With the three-epoch operator, the operator arm read 73.6 and the union 67.5. With the eight-epoch operator they read 66.9 and 66.8. The trigram sampler moved from 13.6 to 13.9 and then 13.3. It moved because the guide re-weights every candidate by its cosine against the prediction, whichever arm proposes it.

Distinctness rose. The operator arm's distinct ratio rose from 0.802 in docs/76 to 0.889, above the sampler's 0.884. The union read 0.897. Novel bigrams rose a little, to 0.4% for the operator arm. The evaluator stayed flat at 0.671 to 0.673.

The sources record one phrase from these samples: "morning to think of some time for it". Reading the first three samples suggested that one attractor owned every prompt. docs/78 measured it. For each continuation, the harness counted the share of its 4-grams that also occur in some other prompt's continuation. The trigram sampler shared 25.7%. The operator shared 8.7%. The union shared 11.3%. The phrase came from three prompts out of forty. The arm that repeats itself across prompts is the sampler.

Next came an attempt to keep continuations near their prompts. The prompt anchor adds the prompt's mean function to every step's context with weight α, as a standing condition the words keep moving under. At α = 0.5, 1 and 2 it made everything worse. Operator KN went to 86, 98 and 119. Distinct fell to 0.84, 0.78 and 0.75. Novel bigrams rose to as much as 4%. docs/78's reading: a standing pull toward the prompt's neighbourhood narrows the candidate set at every step instead of steering it. `OperatorGuide::anchor` ships at α = 0, and `PHIANO_ANCHOR` sets another value for anyone who wants to try.

A later pass, docs/86 through PHIENT_FACTS.md, tried Liskov's three operators on the same `decode_eval` at k = 32, with the nearest-k arm at 66.9. Forcing at λ = 0.05 read 69.7, at λ = 0.25 68.5, and at λ = 1 107.4, with evaluator 0.712, distinct 0.777 and 22.3% shared 4-grams at the last setting. Forcing does not beat greedy on text.

What decoding from the operator sounds like, then, can be said only in numbers and one phrase. Full samples are not reproduced in the sources read for this paper. The text is less repetitive across prompts than the sampler's and more varied in its tokens. Under a trigram judge it sits at 66.9, between the sampler's 13.3 and Austen's own 187.5. docs/78 calls it far from stitched-Austen fluency. No listener has judged it.

## 11. What it depends on, and what depends on it

The operator is a phinet object trained on a Phiano table, and it sits in the middle of the PML family.

phinet is the parent engine. It supplies everything the sweep used: the complex `Tensor [b,w,s]`, the `Tape` autograd with `cosine_table` and `cross_entropy`, the unitary FFT, `Fno` itself, `Adam` with decay and `Schedule`, and the versioned save formats. The engine is tested against a PyTorch reference in 62 of 62 parity checks. The softmax loss and AdamW used here were each checked against torch at parity 0.0 in docs/76. The engine also has HRR, 64 hexagram gates and the sphere, none of which this sweep touched.

phifno 1.0.0 is the spherical operator, a child of phinet. The sphere ties were measured on this same Austen harness, and the rows sit in `data/sweep/results.md`. `phinet::Tie::Sphere` keeps one complex weight per spherical-harmonic degree, with the 64 channels read as Y_l^m for l < 8. At 1,192 parameters and hinge for three epochs it read 12.8× valid and 13.1× test, hit@10 0.182, in 89 s. Under softmax β=8 it read 13.2× and 13.5×, hit@10 0.189, in 408 s. `Tie::Anisotropic` keeps one weight per (l, |m|), 36 slots and 4,776 parameters. Under hinge for three epochs it read 16.0× valid and 15.6× test, hit@10 0.254, in 85 s. That beats the circle's hinge arm at 14.7× and 14.6× with 8,360 parameters. The sphere at wide+deep softmax is unrun.

phical is the geometry of the relation circle, the closed circle every function lives on. The operator's smoothness regulariser is a statement about neighbouring samples on that circle. phinum is a child of phinet. phiton gives light as units, and PhiTon's wavelength, band and hexagram appear per codon in the helix. phiqlm is planned, and nothing of it is built. None of phinum, phiton or phiqlm reads the Austen operator in the sources read here.

phiano is the word application that owns this operator. It holds `Facet`, the `Trainer`, generation, personas, quest ingest and the helix. The table here is its shipped voice lexicon, read directed. The `Generator` with `Decode` and `OperatorGuide` consumes `data/austen_operator.fno`. Persona work in docs/76 shares the lexicon: centring the sectors on `Fingerprint::lexicon_mean` moved attribution from 64% to 70%. That is a separate measurement and does not use the operator.

phiecs is the entity-component world and the quest. Its synthetic oracle has its own trainer, and the sources record no use of this operator there. The docs/86 pass scored the quest with the same Liskov operators used on the text. phiace is not described in the sources read for this paper, and no dependency between it and the operator is recorded there.

## 12. The next experiment

docs/78 names the first item that remains: the schedule at eight epochs. It is the cheapest open question with a clean control.

The arm is the wide+deep operator, 50,016 parameters, cross-entropy over the table at β = 8, trained with AdamW at 1e-3, warm-up and cosine decay, for eight epochs. The schedule's shape depends on its length. So the fair arm is a fresh run with the cosine laid over eight epochs from the start. Resuming the three-epoch AdamW checkpoint would carry a schedule that already decayed.

The control is the row that ships: softmax, wide+deep, Adam, continued to eight, at 30.7× valid, 30.3× test, hit@10 0.348, 3,189 s. Everything else stays fixed. That means the Austen split at seed 7, the shipped voice lexicon read directed, the pool of 1,228 words, the 204,592 training examples, batch 64, best epoch by validation MRR, and test scored once. One thing differs, the optimiser and its schedule, as in every arm of the sweep. The run is resumable through `data/sweep/<arm>.ckpt` at quarter-epochs. The continued arm's `s` column, 3,189, gives the scale of the cost.

The outcome has two readings, and both are useful. If AdamW with the cosine beats 30.7× on validation and holds on test, the winner changes and `data/austen_operator.fno` is replaced. If it does not, docs/78's explanation holds, and the three-epoch shortfall was a short-run effect. Either way the question closes.

A second measurement follows the first, with its own control. If the operator changes, `bin/decode_eval` is rerun at 40 prompts, 30 tokens and k = 32. Its control is the docs/78 eight-epoch rows: sampler 13.3, operator 66.9, union 66.8, with shared 4-grams at 25.7%, 8.7% and 11.3%. docs/78 found that three epochs of ranking, 27.6× to 30.7×, bought seven points. A further gain in rank will show how much of the remaining gap to the sampler the ranking still owns.

Two other experiments are named in the sources and are not next. The first is the sphere at wide+deep softmax, which is unrun. Its control would be the circle's wide+deep arm. The anisotropic tie at 4,776 parameters already beats the circle's hinge arm, so it is the tie to try first. The second is docs/71's prior-corrected proposal score with frequency-decayed pseudo-counts. It was proposed for n-gram decoding on the Rust Book and is not measured in the sources read here. Its control would be `OPERATOR_PROPOSALS` at 0.

The sweep's own discipline carries over. One change per arm. The best epoch chosen by validation. Test read once.

## 13. What is wrong or missing

The sources disagree in five places, and a reader should know them.

First, the hinge operator's ranking. docs/76 names the composer's ceiling as the operator's ranking quality at 16.1× chance. docs/78 says the shipped hinge operator at eight epochs, citing docs/76, was 17.4× valid and 17.1× test. The number 17.4× does not appear in docs/76. This paper prefers the later document and uses 17.4× and 17.1×. The sources do not say whether the 16.1× and the 17.4× describe the same operator.

Second, timing. docs/78 says a full-table softmax epoch costs 14 minutes and the wide arms about 20. The `s` column gives 395 s for three softmax epochs and 1,157 s for three wide+deep epochs. Those two statements do not agree. The `cosine_table` backward fix may account for the difference, but docs/78 does not say which timing came before it.

Third, docs/78's own "What remains" section cites the gap as 73.6 against 13.9. Those are the three-epoch operator's numbers. The operator that ships reads 66.9 against 13.3. This paper uses the eight-epoch numbers.

Fourth, `data/sweep/results.md` has no row for the wide+deep Adam arm at three epochs. Its row was replaced by the continued eight-epoch row. The 27.6× test figure and the 1,157 s exist only in docs/78's table.

Fifth, PHIENT_FACTS.md lists "sphere at softmax/wide+deep unrun" under things not done, and it also gives a softmax β=8 sphere row at 13.2× and 13.5×. The results file has that row. The later entry wins: the sphere at softmax β=8 was run, and the sphere at wide+deep is unrun.

Beyond the disagreements, several things are missing. One seed per arm, and no variance estimate. Three Austen novels only. docs/76 lists the three missing novels as open. No wide+deep hinge arm, so size and objective are not fully separated. No `modes = 0` arm on Austen. No schedule at eight epochs.

The decoding measures have limits of their own. The Kneser-Ney judge is a trigram-family model, and the trigram sampler scores 13.3 under it while Austen scores 187.5. A lower number is nearer the judge, which is a different thing from nearer Austen. The engine's `Evaluator` preferred the broken proposals of docs/71 (0.734 against 0.730), and docs/71 said that tells more about the evaluator than the text. Shared 4-grams measure repetition across prompts and say nothing about sense. No human has read the samples against the prompts, and no sample is reproduced in the docs beyond one phrase.

The engine is allocation-bound. A batched complex GEMM is not built, and f32 compute is not built.

## 14. How it was found

The history runs through five documents and a results file.

docs/68 measured the one-sided lexicon. docs/71 cites its mixture operator at 134 perplexity on the Rust Book. docs/70 measured next-word ranking on the frequent pool, found the directed reading far ahead, and gave the bare identity 15.5× chance. The sources read here do not state that corpus, so the figure cannot be set against Austen's 7.9×. docs/70 left two items open.

docs/71 closed both. Its first half measured perplexity under the one-sided and two-sided readings. Directed beat source on every MRR, and the hinge operator's MRR went 0.137 → 0.150 under the directed reading. It also refuted a tie of target sides to source sides below an evidence floor of 16. Its second half first put the operator into generation. Re-weighting changed nothing. Proposals were sixty times better than an untrained control and still broke the text. `OPERATOR_PROPOSALS` shipped at 0. The same pass added `operator_focus` to beam search and round-tripped the v6 format through `Storage::save` and `load`.

docs/75 listed what the engine still lacked. docs/76 built it: `Tape::cross_entropy` and `Fno::train_epoch_softmax`, `Adam::with_decay`, versioned save formats with optimiser state, the faster `apply`, `Module`, `Fno::at_resolution`, the phasor ranges, `Tape::fft2`, and the learning-rate `Schedule`. That round closed at 29 tests and 62 torch parity checks, all passing. On the Phiano side docs/76 built `Decode::{Trigram, Operator{k}, Union{k}}` and `bin/decode_eval`, measured 97.2 for the operator arm, and named ranking as the ceiling. It also declined the killer heuristic, since the beam scores every candidate and does no alpha-beta pruning. It would change nothing measurable.

docs/78 is the sweep. It took docs/76's sentence about the ceiling as a hypothesis. It wrote `bin/operator_sweep`, made it resumable because the softmax arms were slow, trained five arms that each differ from a neighbour in one thing, and shipped the winner. Then it reran `decode_eval` and found the ranking no longer the whole ceiling. It read three samples and suspected a single attractor. It measured shared 4-grams and found the suspicion wrong: the sampler is the repeater. It tried the prompt anchor at three strengths and shipped it off.

The sphere rows in `data/sweep/results.md` use the same harness, and docs/78 does not mention them. docs/86 is the latest document cited here. It ran Liskov's forcing on `decode_eval` and found forcing does not beat greedy on text.

The method is the same in every step. A sentence in one document is treated as a hypothesis in the next. A thing believed from reading samples is measured before it is written down. A negative result ships with its switch set off, so it stays reproducible.

## 15. What remains

The list is short, and each item is from the sources.

The operator's schedule at eight epochs remains. The sweep ran AdamW with warm-up and cosine for three epochs only, at 24.5× valid, and docs/78 says that is not evidence against the schedule at eight. Section 12 gives the arm and its control.

The gap between the operator's text and the sampler's remains. At the eight-epoch operator it is 66.9 against 13.3. docs/78 names the sampler's repetitiveness, 25.7% shared 4-grams, as the price to avoid in closing it. The union arm reads 66.8 with 11.3% shared, between the two on repetition and level with the operator on perplexity. It has not closed the gap. The prompt anchor was one attempt to steer, and it narrowed instead. Liskov's forcing was another, and it did not beat greedy.

The ranking itself remains open above 30.7×. The validation curve flattened over its last epochs. Whether more epochs or a different structure raises it further is not measured. The anisotropic sphere tie beat the circle's hinge arm with fewer parameters. The sphere at wide+deep softmax is unrun.

The author-separating sector remains. It is listed in docs/76 and docs/78. Centring sectors on the lexicon mean moved attribution from 64% to 70%, and the dominant sectors were still 44 to 48. The operator does not address it.

The three missing Austen novels remain. The corpus is Emma, Persuasion, and Sense and Sensibility. More text means more evidence per target side, and docs/71 found the rare target sides short of evidence in the two-sided lexicon.

The engine items remain. A real batched complex GEMM is not built, and docs/76 puts about 10× on `apply` behind it. f32 compute is not built. docs search is not built. The engine is allocation-bound at these sizes, which is where docs/78 puts the next speed-up.

Some things this work does not claim. There is no human judgment of the text. There is no second seed. No operator was trained on a corpus other than Austen for this sweep. There is no Phi-4 run yet; that line has a synthetic oracle only. There is no weather work of any kind. There is no partnership with anyone. The claim in section 2 stands on one table, one decoding harness and one seed, and it is only as wide as they are.

## 16. Sources

Seven local files were read for this paper, besides the brief. Nothing else was consulted and nothing was run.

`/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` gave the paradigm paragraph, quoted verbatim after the front matter, and the family map used in section 11. It also gave the summary numbers for the sweep and for decoding, the sphere and anisotropic tie rows, the phinet parity count of 62 of 62, the note that the engine is allocation-bound with a 1 MB operation taking 2–5 ms, the Liskov forcing results from docs/86, and the list of things not done.

`/home/claude/phiano/docs/78_operator_sweep.md` is the primary source. It gave the corpus, the split, the pool of 1,228 words, the 204,592 training examples and the chance MRR of 0.0063. It gave the five-arm table, the epoch curve from 27.6 to 30.7, the decoding table with three-epoch and eight-epoch columns, the shared 4-gram table, the prompt anchor results, the resumable checkpointing, the `cosine_table` backward change, and the list of what remains.

`/home/claude/phiano/data/sweep/results.md` gave the finished rows as written to disk. That includes the three sphere rows, and it lacks the three-epoch wide+deep Adam row.

`/home/claude/phiano/docs/76_completion_round.md` gave the engine work that made the sweep possible. That means `Tape::cross_entropy`, `Fno::train_epoch_softmax`, `Adam::with_decay`, the save formats, the `apply` speed-up and `Schedule`. It also gave the first decoding table from the hinge operator, the 16.1× ceiling sentence, the persona centring result and the killer heuristic decision.

`/home/claude/phiano/docs/71_generation_measured.md` gave the first generation measurements on the Rust Book. Those are the re-weighting formula, the table in which nothing moved, the proposals that were sixty times better than a control and still broke the text, the one sample "to start well → Parameter to we in.", and the prior-corrected score, which was proposed and not measured.

`/home/claude/phinet/docs/guide/02-fourier-operator.md` gave the layer equation, the meaning of `F`, `R`, `W` and `σ`, the smoothness argument, `Fno::at_resolution`, `FnoConfig` with its defaults and the 8,360 parameter count, and the pointwise control with its measured gaps.

`/home/claude/phinet/docs/guide/03-objectives.md` gave the hinge objective and its signature, `Fno::train_epoch_with` and `FrequencySampler` with the 149, 154 and 175 perplexities, the mixture likelihood with `MixtureExample` and its move from 149 to 134 at about twenty times the hinge's cost, and the early-stopping note.

The documents cited by number inside these files (docs/68, docs/70, docs/75, docs/86) were not read directly. What this paper says of them comes from the files above.
