# 137 · The operator as the base the counts move from, and what the counts never heard

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/136 ended with the operator's reading of held-out *Emma* at 9.2% against the bigram's 18.8%, and said the operator's place, if it has one, is where the count has nothing. This doc asks exactly that: of the positions a new paragraph puts to the tree, how many has the count never heard — and there, does the operator reach further than the prior the counts fall back to? It also makes the operator's distribution the base the levels move from (D4 nested with the operator at the root in place of the prior), gives the field the operator's word functions, and takes the second wall of docs/132 down: a tree told more is no longer read again.

## 1. Measured: the two strata

Same held-out set as docs/135–136 (5% of *Emma*'s paragraphs, 101 of them, 10,578 positions, seed 7). The operator is phinet's Joint FNO on the told paragraphs' `word next word` rows (185,209 rows; R = 64, width 16, two layers, eight epochs, 25 minutes on two cores, hinge loss 0.016), saved once as `data/text/emma_next.relate` (7.9 MB) so the measurements can be run again without the epochs (`--model`). Its cosines become a distribution at a temperature fitted on 2,000 *told* rows — never on what is held out — over a fixed grid; T = 0.1 won (mean ln p −5.20 against −5.63 at 0.05 and −6.17 at 0.2).

Each held-out position is put in one of two strata by what the counts know: was the word that came ever told after the last word, or after the last pair? Each stratum is then read three ways: by the counts (flat, the docs/135 reading), by the operator from the last word alone, and by the prior alone (every word by how often it came next anywhere).

| the word that came was… | positions | counts (flat) top-1 · MRR | operator (last word) | prior alone |
|---|---|---|---|---|
| told after the last word or the last pair | 8,147 (77%) | **26.5% · 0.387** | 11.5% · 0.232 | 6.9% · 0.156 |
| never told after either | 2,431 (23%) | 1.1% · 0.045 | 1.3% · 0.047 | **3.4% · 0.076** |

Three things, plainly.

Where the count has heard the pair or the word go on to what came, it is the reading: 26.5% against the operator's 11.5%, the same gap as docs/136's whole-set numbers, now seen to be entirely this stratum.

Where the count has heard nothing — nearly a quarter of a new paragraph — **the operator reaches no further than the count's own fall-back**: 1.3% against 1.1%, and both below the prior alone at 3.4%. The counts lose to the prior here because the word and pair levels, having been told other continuations, move the mass toward those; the operator loses to it because what it learned from `word next word` is, as docs/92 said of the quest table, the shape of an answer — a function word after a noun, a noun after `the` — which the unigram prior already has by frequency. The relation the paradigm wants the operator for — the word never told after this word, reached by what *this word's kin* were told — is not what eight epochs of this training yield. That is the measurement; it does not say the operator cannot, it says this one, trained this way, does not.

The two strata also say what the ceiling of docs/135 is made of: at most 77% of positions can be answered by anything that only recalls; the other 23% need inference, and nothing in the tree infers yet.

## 2. Measured: the readings together

| reading | top-1 | MRR | perplexity |
|---|---|---|---|
| flat: the last word, then the last two (docs/135) | 20.6% | 0.309 | 159.0 |
| flat, the levels moving from the operator's distribution (T = 0.1) | 20.6% | 0.308 | **155.4** |
| tree by counts, gated (docs/135) | 20.1% | 0.304 | 201.5 |
| tree by counts, moving from the operator's distribution | 20.4% | 0.307 | 162.3 |
| tree, cast by phasor, seeded words, R = 64 (docs/136) | 20.4% | 0.307 | 163.9 |
| tree, cast by phasor with the operator's word functions, R = 64 | 20.5% | 0.307 | 162.6 |

`Knowing::base` is new: the base the outermost level moves from, given the words said — here the operator's distribution for the last word (`Counts::read_levels_from`; a post is still earned against the prior). With it the flat reading keeps its top-1 and MRR and gains a little perplexity (159 → 155): the operator's distribution is a better floor than the unigram's for the mass the counts leave unposted, which is the 23% above. The count-cast tree gains more (201 → 162) for the same reason, the gated casts having been the worst-floored positions. The field with the operator's word functions (`Field::with_words`) reads the same as the seeded one to within a tenth of a point: on this test, learned functions of this kind change nothing about which scope a cast is drawn to, consistent with docs/136's finding that the cast's limit is the superposition, not the directions.

## 3. The second wall: nothing told is read again

`Knowing::learn_new(root)` tells the tree only the paragraphs whose paths it has not seen — the chapter the teacher appends to grows by a paragraph, and that paragraph alone is read, counted and added to the scope tree (`Counts::learn` and `ScopeTree::add_rows` already accumulate; a field over the scopes is built again). `story --teach` uses it: the first cycle reads the tree, every later cycle reads one paragraph. Tested (`what_is_new_is_told_without_reading_the_told_again`): learning again with nothing new reads nothing; a paragraph appended to a told chapter is told once, under `p2`. The first wall of docs/132 (the store) stands where docs/133 left it, 2.8 KB a word.

## 4. In one line

Three quarters of a new paragraph the counts recall and the operator does not; the last quarter neither recalls, and the operator trained on `word next word` infers no better than frequency does — so the operator earns its place in the tree so far only as the floor under the counts, and the reading that infers is still to be built.

Housekeeping: `story/{operator,strata}.rs` hold the measurement (`--operator R,epochs --model file`), every source file under 200 lines, nothing dangling, workspace tests and zero warnings as before.

`story data\text --in text\emma --held-out 0.05 --phasor 256 --operator 64,8 --model data\text\emma_next.relate` · `cargo test --workspace`
