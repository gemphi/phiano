# Better, worse, loss: the three operators, the forcing choice, and a quest filed under sixty-four hexagrams

*Phient papers, 10 of 16 · 2026-05-18 · Liskov's better and worse as phinet's three operators, the forcing decoder that uses them, and the quest scored by them and filed by hexagram*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

This paper states what Phient took from Barbara Liskov's 1968 thesis, what it built from it, and what the building measured. The thesis, written as Barbara Huberman at Stanford under John McCarthy, played three chess endgames with no evaluation number. It gave its search two predicates on pairs of positions, *better* and *worse*, and a forcing tree. Phient took the two names and added a third. The module is `phinet::liskov`. It holds four functions: `better`, `worse`, `loss` and `forcing_choice`.

Two things were built on the module. The first is a decoding mode in phiano, `Decode::Forcing { k, lambda }`, served by `OperatorGuide::nearest_forcing`. It takes the 2k words nearest the operator's prediction, looks one step past each, and re-ranks them by fit less λ times refutation. The second is a scoring pass in `bin/quest_ingest`. It scores every `about` word of a quest by the three operators and files each word under the hexagram its 64-channel function falls in, using `phinet::hexagram::hexagram_of`.

The measurements are short. On text, with `bin/decode_eval` over 40 held-out prompts of 30 tokens at k = 32, the greedy operator's Kneser-Ney perplexity is 66.9. Forcing gives 69.7 at λ = 0.05, 68.5 at λ = 0.25 and 107.4 at λ = 1. Forcing does not beat greedy at any λ tried. At λ = 1 the evaluator's quality rises to 0.712, distinctness falls to 0.777, and shared 4-grams across prompts rise to 22.3%. On the synthetic quest, 256 words occupy 61 of 64 hexagrams. Mean better is −0.008, mean worse −0.008, mean loss 0.108 at a margin of 0.1. The table is flat. docs/86 reads the flatness as the byte ceiling of docs/83 seen through a third operator.

So the shape is in the code and absent from the gains. The operator ships greedy and forcing is an arm. The hexagram table exists for a corpus that is not flat, which is Phi-4's, and that run has not been made.

The paper is in sixteen sections. Sections 2 to 5 give the claim, the mechanism, the numbers and the idea. Sections 6 to 10 go through each operator and each use in depth, with the real code, the real unit tests, the real decoding table and the real hexagram table. Section 11 places the module in the PML family. Section 12 names the next experiment and its controls. Section 13 says what is wrong. Section 14 is the history, section 15 what remains, and section 16 the sources.

## 2. The claim

The claim is about shape first and about results second.

The shape claim is this. A word in Phical has no value of its own. It has relations, and a relation is what a pair of words does under a condition. Liskov's endgame program made the same refusal for chess positions. It stored no score in a position. It compared pairs of positions, and the comparison depended on the stage the pair was in. Phient claims that her two predicates are the paradigm in miniature, and that the natural names for what its models compute are hers. `better` is the comparison of two candidates under a prediction. `worse` is the refutation of a candidate by what follows it. `loss` is what training minimises so that `better` on held-out text agrees with the corpus. The module comment in `phinet/src/liskov.rs` states it in one line: "Nothing here is a number a position *has*; everything is a comparison a pair of functions makes under a prediction. That is the paradigm."

The second part of the shape claim is that two of the three names already existed in the engine under other names. `better` was the difference of the hinge's two cosines and the quantity the ranking MRR sorts by. `loss` was `Tape::hinge`, and softmax is `−log` of `better` exponentiated over the table. Only `worse` was new. docs/86 records this in its first table, in the column "where it already was", and the entry for `worse` is "nowhere; new".

The result claim is narrower and is mostly negative. Phient claims that the forcing decoder was built, run and measured on text, and that it did not beat the greedy operator on perplexity at any λ tried. It claims that the three operators were run over the synthetic quest as scores, and that the scores are flat. It does not claim that forcing helps anywhere, or that the hexagram filing reveals structure in any corpus.

There is a third claim, about where the shape should help. docs/86 says a one-step lookahead in a space where every word has some continuation is not the endgame Liskov had. In her endgames most moves lead nowhere. On text every word leads somewhere. The place `worse` should bite is the quest, where a word either has onward relations or does not. That claim is a hypothesis. It is not measured, and section 12 is the experiment that would measure it.

Phient has never played chess. No tree deeper than two has been built, and no Phi-4 quest has been scored.

## 3. The mechanism, plainly

The module is `phinet::liskov`. It uses three things from the crate root: `c64`, the complex type; `cosine`, which returns the cosine between two complex functions; and `Function`, a word's vector of phasors.

`better(pred: &[c64], a: &[c64], b: &[c64]) -> f64` returns `cosine(pred, a) - cosine(pred, b)`. Positive means `a` wins. It is antisymmetric by construction.

`worse(pred: &[c64], a: &[c64], after: &[c64], continuations: &[Function]) -> f64` computes `now = cosine(pred, a)`, then `best_reply`, the maximum of `cosine(after, t)` over every `t` in `continuations`. It returns `now - best_reply`. If there are no continuations the maximum is not finite and it returns 0.0. `after` is the operator's prediction once `a` is in the context. Positive means the best reply is weaker than the move, and the doc comment says "the move leads downhill".

`loss(pred, target, decoy, margin) -> f64` returns `(margin - better(pred, target, decoy)).max(0.0)`. It is zero when the target already beats the decoy by the margin.

`forcing_choice(pred, candidates: &[(usize, Function)], continuations, lambda, after_of: impl FnMut(usize) -> Option<Function>) -> Option<(usize, f64)>` walks the candidates. For each it computes a refutation by calling `after_of(i)` and then `worse`, but only when `lambda != 0.0`. Its score is `cosine(pred, f) - lambda * refuted`. It keeps the largest. At λ = 0 it is the greedy nearest candidate.

In phiano, `Decode` has a variant `Forcing { k: usize, lambda: f64 }`. In `generator.rs` that variant routes to `OperatorGuide::nearest_forcing(k, OPERATOR_PROPOSAL_FLOOR, lambda)`, and every other operator variant routes to `nearest`. `nearest_forcing` in `guide_read.rs` calls `self.predict()` to get `y`. It builds the pool of words whose count is at least the floor, scores each by `phinet::cosine(&y, table[i])`, sorts, and truncates to `2 * k`. For each of those candidates it appends the candidate's index to the history, builds a context with `self.space.context(&ext, ext.len(), self.window)`, and applies `self.fno.apply` to get `after`. It then calls `phinet::liskov::worse(&y, f, &a, &continuations)`, where `continuations` is the whole pool. Each candidate is re-ranked by `cosine − lambda * refuted`, and the top `k` words are returned. The generator's scorer then chooses among them.

In `bin/quest_ingest/liskov.rs` there is a `WordScore` struct with `word`, `hexagram: u8`, `rows`, `better`, `worse` and `loss`, and a constant `MARGIN = 0.1`. `function_of(facet, word)` builds a word's 64-channel function: a unit phasor at the source angle `p.theta(k)` on every channel with evidence, zero elsewhere, normalised. `score_words(facet, rows)` scores each `about` word over its rows. `liskov_report(scores)` returns a TSV per word and a markdown table per hexagram. `glyph(id)` returns the character at `0x4DC0 + id`.

`phinet::hexagram::hexagram_of(f)` sums the function's phasors, takes the phase of the sum on [0, 2π), and returns the floor of that phase over 2π times 64. The hexagram is the sector of the centroid.

## 4. What was measured, and the numbers

Two runs bear on the three operators. Both are in docs/86 and the facts file.

The first is `bin/decode_eval`. It loads the Austen operator from `data/austen_operator.fno`, runs 40 held-out prompts for 30 tokens each, and reports five arms at k = 32.

| arm | KN perplexity | evaluator | distinct | shared 4-grams |
|---|---|---|---|---|
| trigram sampler | 13.3 | 0.671 | 0.884 | 26.2% |
| operator nearest-k | 66.9 | 0.673 | 0.889 | 9.9% |
| forcing λ = 0.05 | 69.7 | 0.676 | 0.881 | 10.5% |
| forcing λ = 0.25 | 68.5 | 0.678 | 0.880 | 10.5% |
| forcing λ = 1 | 107.4 | 0.712 | 0.777 | 22.3% |

The sources disagree on two cells. PHIENT_FACTS.md, in its decoding line, gives shared 4-grams of 25.7% for the sampler and 8.7% for the operator. docs/86 gives 26.2% and 9.9% in the table above. docs/86 is the later document and the run it reports is the one that includes the forcing arms, so this paper uses 26.2% and 9.9% when comparing against forcing. The perplexities agree: 13.3 for the sampler and 66.9 for the operator, which the facts file notes was 97.2 before.

At small λ, forcing adds under three points of perplexity and changes little else. At λ = 1 perplexity rises by more than forty points, distinctness drops from 0.889 to 0.777, and shared 4-grams go from 9.9% to 22.3%, which docs/86 calls a doubling. The evaluator's score rises from 0.673 to 0.712. It is the only column forcing leads, and the evaluator is Phient's own.

The second run is `bin/quest_ingest` on the synthetic quest. The quest is phiecs's oracle: 256 words in families of 8, five kinds (is-a, part-of, near, used-for, unlike), 1,280 rows, 293 target words, chance MRR 0.021. The trainer used is attraction-only with dithered rounding. Its measured quest numbers are MRR 0.594, hit@1 34.8%, hit@5 97.4%, within-kind 0.747, wrong channel 0.015 and offset reading 0.005. docs/83 prints the epoch table ending there at epoch 64, with chance 0.0214.

The three operators over that trained facet, from the report line in docs/83:

| quantity | value |
|---|---|
| words scored | 256 |
| margin | 0.1 |
| mean better | −0.008 |
| mean worse | −0.008 |
| mean loss | 0.108 |
| hexagrams occupied | 61 of 64 |

Mean loss 0.108 is the margin plus 0.008, which is what the hinge gives when `better` averages −0.008. The scores per word are written to `data/quest/synthetic.scores.tsv`, which this paper did not read.

Beyond these two runs and three pass-or-fail unit tests, nothing about the module has been measured.

## 5. The one idea, restated

A word has no value. A pair of words has a comparison under a condition. Choosing a word is keeping the candidate that the comparison favours, unless what follows it refutes it. Training is lowering the price of the comparison coming out wrong.

That is the whole of `phinet::liskov`, and each clause is one function. "A pair of words has a comparison under a condition" is `better(pred, a, b)`, where the condition is the prediction `pred` and the comparison is a difference of two cosines. "Unless what follows it refutes it" is `worse(pred, a, after, continuations)`, where what follows is `after`, the operator's prediction once `a` is in the context, and the refutation is how far the best reply falls below the move. "Lowering the price" is `loss(pred, target, decoy, margin)`, the hinge over `better`. "Choosing" is `forcing_choice`, which keeps the largest `cos − λ·worse`.

The idea has two parts that pull in different directions. The first part is what the three operators share. None of them reads a property of one word. Each needs at least two functions and a prediction. This is the part Phient is sure of, because it is only a restatement of what the engine already did. The hinge's two cosines were `better` before the name existed.

The second part is what `worse` adds. It is the only operator that looks forward, and the only one that was new. It says a candidate is worth less if nothing follows it well. This is the part taken most directly from the thesis, where a line that passes through a worse position is not a line that forces. It is also the part that has not been shown to help. On text, where every word has some continuation, the words with the strongest continuations are the frequent function words, and a refutation term pulls toward them.

Put together, the idea predicts its own test. `worse` should matter where continuations are scarce. It should matter little where they are everywhere. Text is the second case, and the λ sweep agrees: nearly nothing changes at small λ, and the text degrades at λ = 1. The quest is supposed to be the first case, because a quest word either has onward relations under some kind or has none. On the synthetic quest as trained, the operators cannot see that difference, because the truth and the best wrong word tie at nearly every row. So the idea has been tested once in the place it should fail and has not yet been tested in the place it should succeed.

## 6. What better and worse were in 1968

This section reads her predicates from sections 1 to 5 of the Liskov series' first paper, and from nowhere else.

The thesis, "A Program to Play Chess End Games", Stanford CS 106 and AI Memo AI-65, is dated August 19, 1968. It plays three endgames: king and rook, king and two bishops, and king, bishop and knight, each against a lone king. Its abstract says: "Embedded in the model are two predicates, better and worse, which contain the heuristics of play, different for each end game."

Her predicates are built from a stage function st, which gives each position an integer from a lowest stage holding stalemate positions to a highest holding checkmate, and one measure per stage, m_i, where a smaller measure is a more advanced position in that stage. Definitions 3.6 and 3.8 are:

| predicate | definition |
|---|---|
| better(p, q) | st(q) > st(p) ∨ [st(q) = st(p) ∧ m_st(q)(q) < m_st(q)(p)] |
| worse(p, q) | st(q) = 0 ∨ [st(p) = st(q) ∧ m_st(p)(p) < m_st(p)(q)] |

So q is better than p if it reaches a higher stage or improves the measure within the stage. It is worse if it is in stage 0 or lets the measure slip within the stage. The forcing tree grows from p until, along every black line, a position better than p and not worse has been reached. Then the first white move is played. For the rook endgame the thesis reports a maximum depth of 8 in stage 2, with 5 as average, and 5 in stage 3. It reports no timings and no node counts.

Beside `phinet::liskov`, three differences show.

The first is arity. Her `better(p, q)` compares a position to a successor reached from it. Phient's `better(pred, a, b)` compares two candidates against one prediction. Hers is progress along a line. Phient's is choice among lines.

The second is kind. Her predicates are booleans over a lexicographic order, stage first and measure second. That order has no magnitude. Phient's `better` is a difference of cosines between −2 and 2. That lets a margin be set and a loss be taken. The series paper warns that to say her program "has no ordering would be false".

The third is role. Her *better* is the target at the leaves and her *worse* is the pruning rule along the way. Phient's `worse` never prunes. It subtracts λ times a gap from a candidate whose replies are poor. Her `worse` has a stage 0 that is absolutely bad. Phient's has no floor.

Her move memory, later called the killer heuristic, has no built counterpart in Phient.

## 7. Better and loss, worked through

`better` and `loss` were in the engine before they were named.

The first unit test in `phinet/src/liskov.rs` is `better_is_antisymmetric_and_loss_is_zero_past_the_margin`. It draws three unit-random functions of length 16 with seeds 1, 2 and 3 and calls them `p`, `a` and `b`. It asserts that `better(&p, &a, &b) + better(&p, &b, &a)` is below 1e-12 in absolute value. That is antisymmetry. It then asserts `loss(&p, &p, &b, 0.5)` equals `0.0f64.max(0.5 - better(&p, &p, &b))`, which is the definition checked at a case where the target is the prediction itself. Last it asserts `loss(&p, &a, &a, 0.0)` is 0.0. A candidate compared with itself has `better` of zero, and at a margin of zero the hinge is zero.

The quest's hexagram table gives worked examples with real values. In `quest_ingest`, `better` is computed per row as `fit - best_wrong.max(-1.0)`. `fit` is `facet.on(w, &r.other, k)`, the facet's reading of the true pair on the kind's channel `k`. `best_wrong` is the largest `facet.on(w, x, k)` over every target `x` in the pool other than the truth. The `loss` per row is `(MARGIN - better).max(0.0)` with `MARGIN` at 0.1. Both are averaged over a word's rows and then over the words in a hexagram.

Take hexagram 2, glyph ䷂, four words. Mean better is −0.100 and mean loss is 0.200. The truth reads on average 0.1 below the best wrong word, and the hinge charges the margin plus that gap: 0.1 − (−0.100) = 0.200. Take hexagram 23, ䷗, three words: mean better −0.133, mean loss 0.233. Take hexagram 12, ䷌, one word: mean better 0.001, mean loss 0.099. There the truth edges the best wrong word by a thousandth and the hinge charges the margin less that. Most rows of the table read better 0.000 and loss 0.100. That is the tie. The truth and the best wrong word read the same, and the hinge charges exactly the margin.

The same `better` sits under the Austen results, unnamed. The hinge operator, 8,360 parameters, reached 14.7× chance MRR on validation and 14.6× on test. Softmax at β = 8 reached 18.0× and 18.8×. The wide and deep softmax, 50,016 parameters, reached 30.7× and 30.3× at 8 epochs, with hit@10 0.348. docs/86 says both hinge and softmax are `loss` written over `better`. On those numbers the softmax form, which prices `better` against the whole table, beats the hinge form, which prices it against one decoy.

`better` ranks two words. A ranking over the pool is `better` against every word, which is what MRR sorts. The quest's form is one against the strongest rival, so a `better` of zero means some rival reads as well as the truth.

## 8. Worse and the forcing choice, worked through

`worse` was new. Two unit tests state what it is for.

`a_move_that_leads_nowhere_is_refuted` draws `p` with seed 4 and sets `a` to a clone of `p`, so the move fits the prediction perfectly and `cosine(p, a)` is 1. The table of continuations is two unit-random functions, seeds 5 and 6. First the test sets `after` to a function drawn with seed 7, which matches nothing in the table. `worse(&p, &a, &dead, &table)` must be above zero. The move fits, and the best reply after it is weak, so the move is refuted. Then the test sets `after` to `table[0]` exactly. Now the best reply has cosine 1, the same as the move, and `worse` must be at most 1e-12. A move whose future is as good as its present is not refuted.

`forcing_prefers_the_move_with_a_future` builds `near` as a clone of `p`, seed 8, with 0.05 added to its first component and renormalised. It builds `far` from seed 9 and a one-entry table from seed 10. The candidates are `(0, near)` and `(1, far)`. With λ = 0 and an `after_of` that returns `None`, `forcing_choice` must pick 0, since `near` fits better. With λ = 2.0 and an `after_of` that returns a seed-11 function for candidate 0 and `table[0]` for candidate 1, it must pick 1. The nearer move leads nowhere and the farther one leads into the table. That is the thesis's claim in sixteen complex numbers: a move that looks good and is refuted loses to a move that looks worse and forces.

`nearest_forcing` does the same at word scale without calling `forcing_choice`. It calls `worse` for each candidate and sorts, since it returns `k` words, and it computes the refutation even at λ = 0.

For each token the guide makes one prediction, then one operator application per candidate, `2k` of them, each followed by a cosine against every pool word. At k = 32 that is 64 extra operator applications per token. The facts file says the engine is allocation- and bandwidth-bound at these sizes, with a 1 MB operation taking 2 to 5 ms. The decode run's wall time is not reported.

What `nearest_forcing` returns is a candidate set. It is not the chosen word. `generator.rs` hands those `k` words to the scorer, which uses counts, relations and learned weights, and `select.rs` passes them through `keep_content`. The generator also carries `FUNCTION_STREAK_LIMIT = 4`, the number of consecutive function words after which the next pick must be a content word. Forcing moves the words that reach the scorer. The scorer and the filters decide what is written.

## 9. Forcing on text

In the decode table one forcing arm moves and two barely do.

`worse` is a word's fit now less the best fit any pool word achieves after it. The pool is every word seen at least `OPERATOR_PROPOSAL_FLOOR` times, and almost every one of them can be followed by something. So `best_reply` is rarely small for any candidate, and `worse` varies across candidates mainly through how strong the best reply is. docs/86 names the words with the strongest replies: they are the frequent function words. A term that subtracts `worse` favours candidates with strong replies, so it favours the words that lead into function words and the function words themselves.

At λ = 0.05 the term is a twentieth of the refutation. It can move only candidates that sit near the cut at 32, and the scorer still chooses among the 32 that pass. Perplexity moves from 66.9 to 69.7, distinctness from 0.889 to 0.881, shared 4-grams from 9.9% to 10.5%. At λ = 0.25 the numbers are 68.5, 0.880 and 10.5%. Neither improves on greedy.

At λ = 1 the refutation weighs as much as the fit. The candidate set that reaches the scorer is now built as much from what leads on as from what fits. The results follow that. Perplexity rises to 107.4. Distinctness falls to 0.777. Shared 4-grams across the 40 prompts rise to 22.3%, which is close to the trigram sampler's 26.2%. The sampler is a count-only decoder, and at λ = 1 forcing repeats itself across prompts nearly as often as a count-only decoder does. The evaluator's quality score rises to 0.712, the highest in the table. The evaluator is a Phient model, and that column is not read as a gain.

docs/86 gives the reading this paper stands by. A one-step lookahead in a space where every word has some continuation is not the endgame Liskov had. Her `worse` has a stage 0, stalemate, that is bad absolutely. Nothing on text is stalemate. A sentence can always go on.

Two more limits apply. The run is one step deep. The thesis's trees in the rook endgame went to depth 5 on average and 8 at most in stage 2. And it is one corpus, one operator file and one k. No other k was run, λ was tried at three values, and no λ between 0.25 and 1 was tried. Whether some λ in that gap gives a perplexity at or below 66.9 is not measured. On the numbers there are, forcing does not beat greedy on text. The operator ships greedy, and `Decode::Forcing` remains an arm of `decode_eval`.

## 10. The quest, filed under sixty-four hexagrams

`score_words` turns the three operators into a report.

For each distinct `about` word `w` it builds `function_of(facet, w)` and files the word under `hexagram_of` of it, the sector of the function's centroid phase.

For each row about `w` it finds the kind's channel with `facet.marker_channel(&r.kind)`. `better` is the fit on that channel less the best wrong target's reading on it. `worse` is built from `onward`: for each kind, the first reading `facet.under(&r.other, kk, &pool)` returns for the truth `r.other`, and the maximum of those across kinds. `worse` is `fit - onward`. It asks whether the truth leads anywhere once reached. `loss` is the hinge over `better` at 0.1.

docs/83 prints read-back rows that show what the operators see. For `w48 near w49`, the whole pool reads back `w49 (1.00)`, and so does the within-kind reading. For `w0 is-a f0`, the whole pool reads back `w29 (1.00)` while the within-kind reading is `f0 (1.00)`. For `w176 part-of w211`, both readings return `w230 (1.00)`, a wrong word. Every reading printed is 1.00. When the truth reads 1.00 and some wrong word also reads 1.00, `better` is zero and `loss` is the margin.

The hexagram table is the same fact at a coarser grain. 256 words fall in 61 of the 64 hexagrams. Ids 54, 56 and 57 are empty. The fullest is id 36, ䷤, with 13 words. Ids 28 and 32 hold 11 each, id 33 holds 10 and id 29 holds 9. Eight ids hold one word each. Five hexagrams have a mean better below −0.01: id 2 at −0.100, id 5 at −0.066, id 23 at −0.133, id 33 at −0.040 and id 38 at −0.050. Every other row reads better between −0.000 and 0.001 and loss between 0.099 and 0.100.

In every one of the 61 rows the mean better and mean worse agree to within 0.001. The code shows how that can happen. If the best wrong word and the truth's strongest onward reading both read 1.00, then `better` and `worse` are both `fit − 1.00`. The read-back rows are consistent with that. The per-word TSV was not read, so this reading was not checked word by word.

docs/86 calls this the byte ceiling of docs/83 seen through the third operator. A channel is one byte, 256 quanta of 2π. The attraction nudge 0.06·sin d falls under half a quantum once sin d is below 0.2, about 11°, and with nearest rounding every pair froze 8 quanta short. The read-back rows printed after dithered training still read 1.00 throughout, and a truth that ties its rival scores flat. The table is there for a corpus that is not flat.

## 11. What it depends on, and what depends on it

`phinet::liskov` sits in the parent engine and is used by one child application. This section traces it both ways across the PML family.

phinet is the parent. It holds the complex `Tensor [b,w,s]`, the `Tape` autograd, the unitary FFT, `Fno`, joint training, HRR, the 64 hexagram gates and the sphere. The liskov module depends on three things from its root: `c64`, `cosine` and `Function`. Its tests use `unit_random` and `normalise`. `loss` is the same quantity as `Tape::hinge`, though the module does not call the tape. The hexagram filing depends on `phinet::hexagram`, whose `hexagram_of` is also the sector function the phasor module tests against at the Q64 quantum. phinet matched a PyTorch reference on 62 of 62 parity tests. The facts file does not say whether any of them covers `liskov`.

phical is the geometry of the relation circle. Everything in `liskov` reduces to a cosine between phasor functions, which is phical's reading of a phase difference. The module has no geometry of its own.

phifno 1.0.0 is the spherical operator. In `nearest_forcing`, `after` comes from `self.fno.apply`, the operator applied to the extended context. The forcing decoder therefore depends on an operator, and in `decode_eval` that operator is loaded from `data/austen_operator.fno`. The sphere tie and the anisotropic tie have not been run under forcing.

phiton gives light as units: `LightQuantum`, `SpectralBand`, `PhitonColor`. In the helix, each codon carries a PhiTon wavelength, band and hexagram, so the hexagram a quest word is filed under has a light reading beside it. Nothing about that pairing has been measured.

phinum is a child of phinet. Nothing in the sources read ties it to `liskov`. phiqlm is planned and nothing is built.

phiano is the word application and the only user. `OperatorGuide::nearest_forcing` and `Decode::Forcing` live in its generator. `bin/quest_ingest` holds the scoring and the hexagram report. `bin/decode_eval` holds the text measurement. A search of the workspace finds `phinet::liskov` called from `guide_read.rs` alone, and named in comments in `generator.rs` and the quest ingest.

phiecs is the entity-component world and the source of the quest. Its synthetic oracle, 256 words in families of 8 across five kinds, is the table `score_words` reads. The three operators depend on phiecs for their only test outside text, and the flat result is partly a fact about that oracle.

No `.rs` file in phiace or phiecs calls the three operators.

Nothing yet depends on the three operators as a result. The operator ships greedy. The hexagram table is a report with no consumer. If a later training loop exposes `better`, `worse` and `loss` as its three names, as docs/86 proposes, phinet's trainer would depend on this module and not the reverse.

## 12. The next experiment, with its controls

docs/86 names the experiment: forcing on the quest, where a word either has onward relations or does not.

The data is the synthetic oracle's held-out split. Twenty percent of the 1,280 rows are held out and the trainer sees the rest. At test the question is, given a source word and a kind, which target. Measured held-out numbers: one hop 0.011 MRR, two hops 0.095 for is-a and 0.093 for used-for, and part-of, near and unlike near chance, against a chance MRR of 0.021.

The arm is forcing. For each held-out row, take the operator's reading for the source under the kind, take the 2k nearest targets, and for each compute `worse` as `quest_ingest` already does: the fit less the candidate's strongest onward reading under any kind, against the quest's own target pool. Re-rank by `cos − λ·worse` and score the rank of the truth. The λ sweep is the one run on text, 0.05, 0.25 and 1, with k = 32, so the two results can be read side by side.

The first control is the same pipeline at λ = 0. That is the greedy one-hop reading, measured at 0.011 held out. Any gain over it at λ > 0 is a gain from the refutation term or from the lookahead that feeds it.

The second control separates those two. It is the two-hop composition, measured at 0.095 and 0.093 for is-a and used-for. Two hops apply the operator twice without the maximum over continuations and without the re-rank. If forcing beats the first control and not the second, the lookahead helped and the refutation did not. If it beats both, the refutation did something the composition does not. If it beats neither, Phient will report that.

The diagnostic is the mean `better` and mean `worse` on the rows where forcing chose differently from greedy. If forcing changes its choice mainly on rows where `worse` was already large, the refutation is doing what the thesis says.

Two conditions come first. The first is the deadband. On the trained facet the truth ties the best wrong word on almost every row, and a re-rank cannot separate ties. The run must use dithered rounding, and must say so, since dither costs corpus attribution 71% → 65% and ships off by default. The second is the pool. `worse` against a pool where every word has a continuation is the text result again. The continuations must be the quest's targets under the quest's kinds.

A second experiment waits: the hexagram table on a Phi-4 quest, with the synthetic table as control, asking whether per-hexagram better and worse come apart. No Phi-4 run has been made.

## 13. What is wrong or missing

These are stated in no order.

Forcing did not help. On text it never beat the greedy operator's perplexity of 66.9. At λ = 1 it lost 40.5 points and repeated itself across prompts more than twice as often. The one column it led was the evaluator, which is Phient's own model.

The operator that was new has never been trained. `loss` is the hinge over `better`. No loss involves `worse`. The decoder is asked to use a refutation the model never saw during training. docs/86 names exposing the three operators in the training loop as work that remains, and today `better` is implicit in the tape's hinge.

On the quest, `better` and `worse` agree to within 0.001 in every hexagram row. The two operators were meant to measure different things, one the rival and one the future. On this data they do not come apart, so the run says nothing about `worse` as distinct from `better`. The likely cause is saturation at 1.00, and that has not been checked word by word.

The quest's `worse` and phinet's `worse` are different functions with one name. phinet's compares a move's fit against the best reply after it, using the operator's prediction `after`. The quest's compares a fit against `onward`, which takes only the first reading `facet.under` returns for each kind, then the largest across kinds. There is no operator application and no `after`. They share an idea. They do not share a code path.

`nearest_forcing` does not call `forcing_choice`. It computes the same score inline. So the unit test that forcing prefers the move with a future tests a function the decoder does not use. The two share `worse`, and that is all.

The hexagram is one number from 64 channels. `hexagram_of` takes the phase of a function's centroid. Two words with very different relations can share a centroid phase. The glyph printed beside each id is the Unicode character at `0x4DC0` plus the id. Nothing in the sources ties sector order to what that glyph traditionally names.

The depth is one. Her trees went to 8. There is no leaf condition, no stage and no terminal position, so nothing Phient has built can be proved to reach anything. The thesis's correctness argument has no counterpart.

Phient's `better` is closer to the evaluation number her thesis refused than hers was. Hers was an order with no magnitude. Phient's is a difference of cosines with a margin.

Missing entirely: any Phi-4 quest; any forcing run on the quest; any λ between 0.25 and 1; any k other than 32; the decode run's wall time; any counterpart to her move memory.

## 14. How it was found

The history is short and is in four places: docs/72, docs/83, docs/86 and the first paper of the Liskov series.

The operator became a decoder before it had these names. The comment on `OperatorGuide::propose` in `guide_read.rs` records a prior-corrected proposal, scoring each word by its cosine less its usual fit. It cites docs/72 for the measurement that this over-corrects into the rare tail, and it is kept for the record while `Generator::relational_propose` ships. After that, `nearest` was written: the `k` pool words nearest the prediction by plain cosine, "decoding from the operator". The facts file records its perplexity as 66.9, down from 97.2. That is the greedy arm every later decode result is compared against.

docs/83 is the quest ingest table. It trained the facet on the synthetic oracle with attraction only and dithered rounding for 64 epochs, from kind MRR 0.023 at epoch 1 to 0.594 at epoch 64, and printed the per-kind table and the read-back rows. It is also where the byte deadband was found: nearest rounding froze every pair 8 quanta short, and the nudge fell under half a quantum below 11°. The Liskov report line and the hexagram table stand at the end of the same file, in the format `bin/quest_ingest` prints.

The Liskov series' first paper, dated 2026-01-12, read the 1968 thesis. It set out definitions 3.6 and 3.8, the forcing tree and the move memory, and it placed `phinet::liskov` against them. Its section 5 is the source of the point this paper repeats in section 13: her `better` was an order.

docs/86 is the record of the adoption. Its first section is the table of the three operators with the column "where it already was". Its second is the forcing decoder and the `decode_eval` table. Its third is the quest scored and filed by hexagram. Its fourth is what remains. It also notes that she was at MITRE and MIT, not IBM. The code followed the document. `phinet/src/liskov.rs` carries the thesis in its module comment, `guide_read.rs` gained `nearest_forcing`, `generator.rs` gained `Decode::Forcing`, and `quest_ingest` gained `liskov.rs` with `score_words` and `liskov_report`.

The order of discovery matters for reading the results. The forcing decoder was tried on text first, because text was where the operator already ran. It failed there. docs/86 then argued that text was the wrong ground and named the quest. The quest scoring was built next, as a score over the trained facet, and it came back flat for a reason docs/83 had already found. So the one experiment docs/86 points to, forcing as a decoder on the quest, is the one that has not been run. The history explains the negative numbers. It does not change them.

## 15. What remains

docs/86 closes with three items, kept here in its order.

The first is forcing at depth two on the quest. Section 12 gives the design: the held-out 20% of the synthetic oracle, λ at 0.05, 0.25 and 1, k = 32, dither on, the quest's own targets as continuations. Its controls are the greedy one-hop reading at 0.011 and the two-hop composition at 0.095 and 0.093. Until it is run, the three operators have been tested only where the idea predicts they should fail.

The second is the hexagram table on a Phi-4 quest. The synthetic table is flat, with means of −0.008, −0.008 and 0.108 and 61 of 64 hexagrams occupied. A table from a corpus with real structure is the only way to learn whether the per-hexagram means differ, and whether `better` and `worse` come apart. No Phi-4 run has been made, and nothing about the three operators as a scoring of a real quest can be claimed until one is.

The third is the training loop. Today `loss` is the tape's hinge and `better` is implicit in it. Exposing the three as the loop's own names is the first step. Giving `worse` a term in training is the second, and it would let the decoder use a refutation the model was taught. Neither has been built.

Beyond docs/86, four smaller items remain. The per-word scores in `data/quest/synthetic.scores.tsv` should be read to test the saturation reading of section 10, row by row. The quest's `worse` should be brought closer to phinet's, with an operator application and an `after`, or given its own name. `nearest_forcing` could call `forcing_choice` or a `k`-returning form of it, so that the unit tests cover the decoder's path. And the text sweep has a gap between λ = 0.25 and λ = 1 that has not been measured.

Two things remain that nothing yet addresses. The first is depth. Her trees went deeper than any lookahead Phient has built. A deeper tree over the 2k candidates has not been built, and its cost has not been measured. The second is a stage. Her `better` was lexicographic, stage first, and her `worse` had an absolute floor at stage 0. A quest with an order over its kinds could support such a `better` and such a floor. Phient does not know whether any of its corpora has such an order and has measured nothing about it.

What does not remain is the adoption. The names are in `phinet::liskov`. The tests fix what they mean. The decoder is an arm of `decode_eval`. The quest report files every word under a hexagram. docs/86 is the record.

## 16. Sources

The local files read for this paper, all in the gemphi workspace:

| path | what it gave |
|---|---|
| /home/claude/phiano/docs/papers/phient/BRIEF.md | the form of this paper |
| /home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md | the paradigm paragraph; the family; every Austen, decoding, quest, deadband, helix and Liskov number |
| /home/claude/phiano/docs/86_liskov_three_operators.md | the three-operator table; the `decode_eval` table with evaluator, distinct and shared 4-grams; the quest scoring; what remains |
| /home/claude/phinet/src/liskov.rs | `better`, `worse`, `loss`, `forcing_choice`; the module comment; the three unit tests and their seeds |
| /home/claude/phiano/src/generate/guide_read.rs | `propose` and its docs/72 note; `nearest`; `nearest_forcing`; `cosine` |
| /home/claude/phiano/src/bin/quest_ingest/liskov.rs | `MARGIN`, `WordScore`, `function_of`, `score_words`, `liskov_report`, `glyph` |
| /home/claude/phiano/docs/83_quest_ingest_table.md | the 64-epoch table; the per-kind table; the read-back rows; the report line; the hexagram table |
| /home/claude/phiano/docs/papers/liskov/01_chess_end_games.md, sections 1–5 | the thesis's abstract, definitions 3.6 and 3.8, the forcing tree, the reported depths, the move memory, and the point that her predicates define an order |

Read in part, to name the code paths correctly:

| path | what it gave |
|---|---|
| /home/claude/phinet/src/hexagram.rs, `hexagram_of` and `phase_of` | the centroid-phase sector |
| /home/claude/phiano/src/generate/generator.rs, around `Decode::Forcing` | the routing to `nearest_forcing`; `FUNCTION_STREAK_LIMIT` |
| /home/claude/phiano/src/generate/select.rs, the operator arms | `keep_content` on the candidate set |
| /home/claude/phiano/src/bin/decode_eval.rs, the arm list and loop | `data/austen_operator.fno`; the forcing arm |
| /home/claude/phiano/src/bin/quest_ingest/main.rs, the report lines | the scores path and the report line format |

A workspace search for `phinet::liskov` and `hexagram_of` across `.rs` files gave the call sites named in section 11.

Where the sources disagree, this paper says so and prefers the later document. The one disagreement found is in shared 4-grams. PHIENT_FACTS.md gives 25.7% for the sampler and 8.7% for the operator in its decoding line. docs/86 gives 26.2% and 9.9% in the run that includes the forcing arms. Section 4 uses docs/86. Every other number in this paper appears in one of the files above as quoted. The only arithmetic added is differences and ratios of those numbers, such as 107.4 less 66.9, and counts of hexagram rows by their word totals, read from the table in docs/83. The glyphs shown are those docs/83 prints.

Not read: `data/quest/synthetic.scores.tsv`, so no per-word score is quoted; docs/72, cited only through the comment in `guide_read.rs`; the thesis itself, cited only through the Liskov series' first paper; any Phi-4 output, since none exists.
