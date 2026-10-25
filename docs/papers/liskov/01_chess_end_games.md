# Better and worse before the number: the forcing tree as a comparison a pair makes

*Phient interpreted papers · Liskov series, 1 of 16 · 2026-01-12 · on: "A Program to Play Chess End Games" (Barbara J. Huberman, Ph.D. thesis, Stanford University, Computer Science Department Technical Report CS 106 / Stanford Artificial Intelligence Project Memo AI-65, August 19, 1968; DTIC AD0673971)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the first of sixteen papers in which Phient reads one work by Barbara Liskov through its own paradigm. The work is her doctoral thesis, written under the name Barbara Huberman and submitted at Stanford in 1968 with John McCarthy as advisor. It describes a program that plays three chess endgames, king and rook against king, king and two bishops against king, and king, bishop and knight against king, and it plays them without an evaluation function. In place of a number that a position has, the program is given two predicates on pairs of positions, *better* and *worse*, and a search shape she calls a forcing tree. The program looks for a position that is better than the one it is in and that it can force against every reply.

Phient reads this thesis first among her works for one reason. Its central move is the same move the paradigm sentence at the top of this page makes. The thesis refuses to store a value in a position. It stores a comparison a pair of positions makes under a condition, the condition being which stage of the endgame the pair is in. Phical refuses to store a point in a word. It stores relations, and a relation is what a pair of words does under a condition. Phient has since taken the names of her two predicates, and a third derived from them, as the names of its own three operators, `better`, `worse` and `loss` in `phinet::liskov`, and has built a decoding mode, `Decode::Forcing`, on the shape of her tree.

Sections 2 to 5 say what the thesis says, in its own terms, with the depths it reports and without the timings it does not report. Sections 6 to 10 place its objects inside the Phient family: a position as a word's function, a stage as a channel, *better* as X ⇒ Y under c, the forcing tree as decoding, and the move memory she describes, later called the killer heuristic, as a count on a relation. Section 11 lists what Phient has measured that bears on any of this. Section 12 names the experiment that would test her idea inside phinet, with its control. Section 13 says where the analogy breaks. Section 14 is the person and the year. Section 15 is what remains. Section 16 is the sources reached and not reached.

One limit is stated up front. Phient has never played chess. Its forcing decoder is a one-step lookahead over words, and on text it did not beat the greedy operator. Every number below about Phient is one already measured for another purpose.

## 2. What the work claims, in its terms

The abstract, in full: "A program to play chess end games is described. The model used in the program is very close to the model assumed in chess books. Embedded in the model are two predicates, better and worse, which contain the heuristics of play, different for each end game. The definitions of better and worse were obtained by programmer translation from the chess books."

The claim has three parts. The first is about the model. Chess books such as Fine's and Capablanca's, which the thesis draws on, do not give the reader a scoring function. They give principles, an example, and the sentence "the principle is to drive the opposing King to the last line on any side of the board." Huberman's position is that the program should assume the same model as the book, so that the book's methods can be carried across with as little change as possible. In her words, "In this way we hope to eliminate making changes in the methods to account for a difference between the program's model and the abstract model assumed in the book."

The second part is about what the program is given. It is given two functions. "The program is supplied with two functions better and worse (containing the methods) which compare positions." Everything specific to an endgame lives in these two functions; the search is the same for all three.

The third part is about what the search does with them. "From a given starting position p, in which the program has the move, it uses tree search to find positions q which are better than p. It will search until such a position q is found for every sequence of moves by the opposition." The premise is stated as a claim about forcing. "The basic premise of this method of play is that from p white is able to force a position q better than p. 'Force' means that white must be able to answer every black move with an eventual better position."

The thesis is organised in eight chapters: an introduction, the program organisation, the definition of *better* and *worse*, one chapter for each of the three endgames, a chapter on program correctness, and a chapter of evaluations and conclusions, with one appendix and a bibliography. It was written in LISP with ARPA support. The thesis does not, in what we read, compare itself to Samuel's checkers work or to the chess programs of the day. The related work it discusses is the Logic Theory Machine and the General Problem Solver, as alternative ways of organising a problem-solving program, and the point of the comparison is the model, not the evaluation.

## 3. The mechanism, plainly

The two predicates are defined on pairs (p, q), where p is a position with white to move and q is a position with black to move that white has reached from p by some sequence. They are built from two things the programmer supplies for each endgame. The first is a stage function st, which assigns to each position an integer. The stages are ordered "from the lowest (zero) stage containing stalemate positions... to the highest stage containing checkmate positions." The second is a family of measures, one per stage, m_i, which orders the positions within stage i; a smaller measure is a more advanced position within the stage.

Her definitions 3.6 and 3.8 are then:

| predicate | definition |
|---|---|
| better(p, q) | st(q) > st(p) ∨ [st(q) = st(p) ∧ m_st(q)(q) < m_st(q)(p)] |
| worse(p, q) | st(q) = 0 ∨ [st(p) = st(q) ∧ m_st(p)(p) < m_st(p)(q)] |

So q is better than p if it reaches a higher stage, or stays in the stage and improves the measure. And q is worse than p if it is a stalemate-class position, or stays in the stage and lets the measure slip back. Within a stage the two are near-complements; across stages a drop is neither, and the consistency conditions she numbers 3.7 and 3.9 ensure black cannot force the program back down a stage once reached.

The search is the forcing tree. From p, white's moves are generated, and for each, every black reply. The tree grows breadth-first until, along every black line, a position q better than p and not worse has been reached. Then the first white move of the tree is played and the process starts again. A tree is thus a proof, over all of black's replies, that white can force an improvement, without a number for how much. For rook and king, "maximum depth of tree search in stage 2 can be as large as 8, although a depth of 5 is average; in stage 3 there is a maximum depth of 5." For bishop and knight, the deepest tree is five moves. Node counts, running times and a count of test positions are not reported, and we state none.

The rest is translation. For each endgame the stages and measures are read out of the books. The rook endgame confines the black king in a shrinking region controlled by the rook; the measure is the size of that region. The two bishops endgame confines the king to a quadrant that must contain a corner, which the thesis enforces with a function it names hascorner. The bishop and knight endgame has seven stages and is the hardest.

## 4. What it demonstrated, as reported

The thesis reports the program playing all three endgames from positions taken from Fine and Capablanca, and the trees it built for them. It reports no tournament, large test set, or timings, and Phient invents none. What it does report, in chapter 7, is an argument that the program is correct. The argument is informal. It says that if the stages and measures for an endgame satisfy the consistency conditions of chapter 3, then from any position in which the endgame is won the program's repeated construction of forcing trees reaches checkmate. It is a proof about the search given the heuristics, and no proof of the heuristics.

The thesis also reports a device for making the search cheaper, and this is the part that outlived it. When a tree finds a position that is better and not worse, the program records the last white move w that reached it. "Then every time after this, when it forms a set Q* to be used as an argument to BW, it checks to see if w was the last move made to form some q*∈Q*. If it was, then q* is made the first position in Q*, so that it will be examined first." The reason given is plain. "The theory is that in a tree search the positions are all similar, so a move which led to a better position at one point is likely to do so again." This is the idea that the Chess Programming Wiki and the Wikipedia article on the killer heuristic both cite her thesis for. Later programs applied it inside alpha-beta, remembering the move that produced a cutoff at a ply and trying it first at siblings. Her version remembers the move that produced a *better* position. The shape is the same.

The reception we can verify is thin and secondhand. Alex Bell's 1972 book, as quoted on the Chess Programming Wiki, summarised the method in one sentence: her "model for the problem was a forcing tree co-ordinated with two functions (better and worse) which were able to compare positions." The wiki lists the endgames it covers as king and queen, king and rook, king and two bishops, king, bishop and knight, and king and three knights, the last of theoretical interest. The chapters we read cover three; we did not read what the wiki takes the other two from.

Nothing in the thesis was measured against another program, and Phient has no basis to say it was stronger or weaker than any. What it demonstrated is that a search shaped by two comparison predicates, with the predicates read out of a textbook, could finish these endgames.

## 5. The one idea, restated

Strip the chess away and one idea is left. A position has no value. A pair of positions has a relation, and the relation is conditional on the stage the pair is in. The search does not maximise anything. It looks for a move after which every reply still leaves the relation *better* true, and it refuses any move after which some reply makes *worse* true.

Three consequences follow, and each lands differently in Phient.

First, the value lives in the comparison, and the comparison is conditional. *Better* is a family of functions with a stage in the subscript; the measure by which q beats p is chosen by which stage q is in. If one tried to collapse this to a single number one would get a lexicographic order, stage first and measure second, and that order is an ordinal, not a magnitude. There is no sense in which one position is better than another by seven. This is why Phient says the thesis plays "with no evaluation number"; that it has no ordering would be false, and section 13 returns to this.

Second, *worse* is not the negation of *better*. Within a stage they are near-complements, but across stages a fall is neither, and a stalemate is worse absolutely. The two predicates have different jobs. *Better* is the target of the search: what the program is looking for at the leaves. *Worse* is the pruning rule: what the program refuses along the way, because it knows that whatever else a line does, if it passes through a worse position it is not a line that forces. A search with two asymmetric predicates is a different object from a search with one score, because the score conflates what one seeks with what one avoids.

Third, forcing is a quantifier structure, not a sum. A forcing tree says: there exists a white move such that for all black replies there exists a white move such that for all black replies... a better position is reached. The structure is exists-forall-exists. An expected value averages over the replies; a forcing tree takes the worst of them. This is what gives chapter 7 its proof. One cannot prove an expectation reaches mate. One can prove a forcing tree does, if its leaves are all better and its predicates are consistent.

Phient's own sentence for the idea is the one that opens docs/86. "A move is kept if the position it reaches is better; it is refuted if some reply makes it worse. Phient prefers that shape to a score a position *has*, because it is the paradigm: a comparison a pair makes under a condition."

## 6. The position as a word's function

In Phical a word stores no point. It stores 32 to 64 relations, each a complex phasor on a closed circle with a count. What a word is, is what it does with other words under conditions. The question section 6 asks is whether a chess position in the thesis is the same kind of object.

Read the definitions in section 3 again. A position p enters the program through two functions, st(p) and m_st(p)(p). Neither is a stored property of p. Both are computed from the board when p is examined, and both are defined only for the purpose of comparing p to some q. Stages and measures exist to make *better* and *worse* definable, and in chapter 7 it is their consistency under comparison that carries the proof. So a position in Huberman's program is, operationally, the set of comparisons it can enter: which stage it shares with which other positions, and how it orders against them within that stage. That is a function, and its arguments are other positions.

The analogy to a Phical word runs like this. A word w in phinet has up to 64 channels. Each channel carries a phasor, a point on the circle, and the phasor's meaning is the phase difference it makes with the phasor of another word on the same channel. The reader that turns a pair into a relation is `cos(pred, a)`, the cosine of a phase difference. In the thesis the reader that turns a pair into a relation is m_st(q)(q) < m_st(q)(p), the order of two measures in one stage. In both cases the stored thing is not the value; in both cases the value is produced only when a pair is formed and a condition is named.

Where the analogy is stronger than it first looks is in the stage subscript. Her measure has no meaning across stages. The rook endgame's measure is a region size; the bishop-and-knight endgame's seven stages each carry their own. To compare two positions one must first find the stage they share. A Phical channel is exactly this. Two words' phasors on the is-a channel say nothing about their phasors on the part-of channel, and the quest's measured numbers say the same: within-kind MRR 0.747, wrong channel 0.015. The measure is conditional on the stage as the cosine is conditional on the channel.

Where the analogy is weaker is in what the function returns. Her st and m return integers, and the comparison is an order. Phical's channel returns a phase, and the comparison is a cosine, which is a continuous quantity between −1 and 1. Section 8 uses this difference to say what Phient's *better* had to add to hers.

## 7. The stage as a relation channel, the channel as a coordinate

A Phical word's channels are its coordinates. They are not axes of a vector space in which the word is a point. They are the conditions under which the word relates. The quest names five, is-a, part-of, near, used-for, unlike; the engine gives 64. This section reads her stage as a channel.

A stage is a condition. Under stage 2 of the rook endgame, the meaning of a position is the size of the region the black king is confined to, and nothing else. Under stage 3 the meaning is different, and the thesis defines a different measure for it. Positions in stages 2 and 3 are comparable only by stage, which is coarse. This is how a channel works in phinet. Under is-a, the meaning of a word pair is one phase difference; under part-of, another; and the two phase differences do not add. A word that is close to another under is-a and far under unlike has two relations, each with its own condition, and no mixed score.

What the reading buys is a way to say what Huberman's translation from the books was doing. A chess book says "confine the king" and "drive it to the edge" and "bring your king up." Each of these is a relation between two positions under a condition. Translating them into a program meant deciding which condition each belongs to, and in what order the conditions come. Her consistency conditions, 3.7 and 3.9, say that once a stage is reached black cannot force a lower one; in Phient's terms, once a pair has been compared under a higher channel, comparison under a lower channel no longer applies. The quest's ingest scores every about word per kind for the same reason: each row names its channel, and a truth is compared to the best wrong word on that channel and not on any other.

Two differences remain. The first is that her stages are ordered and Phical's channels are not. Stage 4 beats stage 2 in the rook endgame. The is-a channel does not beat the part-of channel. The nearest thing Phient has to an order over channels is the hexagram, the centroid phase of a word's 64-channel function in 64 sectors, and that is a filing key, not a rank. On the synthetic quest 256 words occupy 61 of 64 hexagrams and the table is flat. The second difference is that her stages are few and disjoint, and every position is in exactly one. A Phical word is on every channel at once, with a count on each. Her position has one coordinate that matters at a time; Phient's word has 64 that matter together.

## 8. The operator: better, worse, loss as X ⇒ Y under c

The Phical paradigm is written X ⇒ Y under c: a word X, a condition c, and what X goes to. An operator takes a context, produces a prediction, and compares candidates to it on the channel the condition names. The thesis's two predicates read directly as operators of this kind, and Phient has adopted their names.

Set them side by side. Her *better(p, q)* asks whether q, reached from p, is an advance. Phient's `better(pred, a, b)` asks whether a is the better next word than b under the prediction pred, and computes `cos(pred, a) − cos(pred, b)`. The arity differs, and section 13 returns to it. The shape is the same. Neither returns a value of a word. Both return a comparison a pair makes, and the condition, stage in one case and prediction in the other, is what makes the comparison well-defined.

Her *worse(p, q)* asks whether q lets the measure slip or falls into stage 0. Phient's `worse(pred, a, after, continuations)` computes `cos(pred, a) − max_t cos(after, t)`: how far the best continuation after a falls below a's own fit. This is the refutation. A word that fits the prediction but leads nowhere is a move that looks good and is refuted by the reply. That is what her *worse* is for: it rejects a line rather than grading a move. docs/86 records that *worse* existed nowhere in phinet before; *better* was implicit in the hinge's two cosines and the ranking MRR.

The third operator, `loss(pred, target, decoy, margin) = max(0, margin − better)`, is Phient's and not hers. It is the hinge that the tape already computed, renamed so the training loop's three names are the three the thesis suggests. Softmax is −log of *better* exponentiated over the table. The thesis has no learning in it, and the loss is where Phient adds a gradient.

`forcing_choice` is the composition. Among candidates it picks the largest `cos − λ·worse`; at λ = 0 it is the greedy nearest word. Three unit tests fix what the operators mean: antisymmetry of *better*, a dead-end move is refuted, and forcing prefers the move with a future. The last two are the thesis's two claims in miniature. A move after which every continuation is poor is worse. A move with a strong continuation is preferred to one with a slightly better immediate fit and no continuation.

The one thing the paradigm adds to her predicates is that the comparison is continuous. Her *better* is a boolean over an order. Phient's is a difference of cosines. That lets a margin be defined and a loss be taken; it also lets the operator be wrong by a little, which hers cannot be.

## 9. The forcing tree as decoding

The greedy operator in phiano chooses the nearest word to the prediction. The forcing decoder, `Decode::Forcing { k, lambda }`, was built on the thesis's tree, and this section says how far it follows it.

`OperatorGuide::nearest_forcing` takes the 2k nearest words to the prediction. For each candidate it appends the candidate to the context, applies the operator once more to get a prediction for the word after, and computes the best cosine any word in the pool achieves against that second prediction. That is `max_t cos(after, t)`. The candidate's *worse* is its own cosine minus that maximum. The decoder then re-ranks the 2k candidates by `cos − λ·worse` and takes the top. In her terms it is a forcing tree of depth two, with the pool as black's move generator.

What it keeps from the thesis is the asymmetry of the two operators. The candidate is sought by *better*, its nearest-ness to the prediction, and rejected by *worse*, the emptiness of what follows it. It also keeps the quantifier: the replies are taken at their best, not averaged, because the max is over continuations. What it drops is everything that makes her tree a proof. Her tree grows until every black line has reached a better position, and only then is a move played. Phient's decoder looks one step ahead and then plays regardless. There is no leaf condition, no stage, and no argument that the sequence terminates anywhere.

The measured result, from `bin/decode_eval` on 40 held-out prompts of 30 tokens each with k = 32, is in the facts file and repeated here without softening. The greedy operator's Kneser-Ney perplexity is 66.9. Forcing at λ = 0.05 gives 69.7, at λ = 0.25 gives 68.5, and at λ = 1 gives 107.4. Forcing does not beat greedy on text at any λ tried. At λ = 1 the evaluator's own quality rises to 0.712, distinctness falls to 0.777, and shared 4-grams across prompts rise to 22.3%. The reading in docs/86 is the one Phient stands by. Text is not an endgame. In an endgame most moves lead nowhere, which is why a refutation term prunes so well. In text every word has some continuation, and the words with the strongest continuations are the frequent function words. A refutation term on text pulls toward "the" and "of." That is what the numbers show.

The consequence is that the operator ships greedy and forcing is an arm. The thesis's shape is in the code. The place where the shape should bite, docs/86 says, is the quest, where a word either has onward relations under some kind or does not, and that is section 12's experiment.

## 10. The killer move as a count on a relation

The device in section 4, remembering the move w that produced a better position and trying it first in the next set Q*, is the part of the thesis the field kept. Phient already has it in a form the thesis would recognise.

A Phical relation is a phasor with a count. The count is how many times the pair has been seen to relate under that channel. It is a memory across occurrences, and it is exactly a memory of a move that worked. When the quest ingests a row, the truth's relation on the kind's channel is strengthened; the next time a similar prediction is made, that relation is what the nearest-word reader finds first. The killer heuristic says: a move that led to a better position at one point is likely to do so again, because the positions in a tree are all similar. The count says: a relation that held under this channel is likely to hold again, because the contexts in a corpus are all similar. Both are about where to look first.

Two things in the thesis's version are sharper than Phient's. First, her memory is scoped to a tree. It is reset when a move is played and a new tree begins. Phient's count is global and is never reset, so it carries the corpus's frequency as well as the last tree's success. This is the same defect section 9 found in forcing on text: a memory that does not forget prefers the common. The trigram sampler, KN perplexity 13.3 and shared 4-grams 25.7%, is a count-only decoder and shows what pure memory produces. Second, her memory stores a move, which is an action, and Phient's stores a relation, which is a pair. The move w is tried first regardless of the position it is tried in. The relation is found first only when the prediction lands near it. Hers is a bias on the search order; Phient's is a bias on the space.

The helix is where Phient's memory is filed rather than counted. The lexicon as a DNA-coded tree gives each word a codon per channel, 64 codons as base-4 ACGT, with the hexagram and the sphere degree and order beside it, and the whole lexicon of 18,496 words round-trips byte-exact in 1.9 s and 75 MB. A filing is a way to find the killer move: the words that share a codon on a channel are the sibling positions, and the move that worked for one is the first to try for the next. Nothing has been measured about using it that way. Her heuristic was cheap and mostly right; Phient's counts are cheap and their rightness is the corpus's.

## 11. What Phient has measured that bears on it

Every number here is from the facts file and none is a chess result.

| what | result |
|---|---|
| decode_eval, 40 prompts × 30 tokens, k = 32: greedy nearest-k KN perplexity | 66.9 (was 97.2) |
| forcing λ = 0.05 / 0.25 / 1 | 69.7 / 68.5 / 107.4 |
| forcing λ = 1: evaluator, distinct, shared 4-grams | 0.712, 0.777, 22.3% |
| trigram sampler KN perplexity, shared 4-grams | 13.3, 25.7% |
| greedy operator shared 4-grams | 8.7% |
| quest scored by the three operators: mean better / worse / loss | −0.008 / −0.008 / 0.108 (margin 0.1) |
| quest hexagram occupancy | 61 of 64 |
| quest, attraction-only + dithered rounding: MRR, hit@1, hit@5 | 0.594, 34.8%, 97.4% |
| quest within-kind / wrong channel / offset | 0.747 / 0.015 / 0.005 |
| quest held-out one hop / two hops is-a / used-for | 0.011 / 0.095 / 0.093 |

The first block is the forcing decoder on text, discussed in section 9. Forcing does not beat greedy at any λ; at λ = 1 it repeats itself. It is the only direct test of the thesis's shape Phient has run.

The second block is the three operators applied to the quest as scores rather than as a decoder. `bin/quest_ingest` computes, for every about word, *better* as the truth's margin over the best wrong word on the kind's channel, *worse* as the gap between that fit and the truth's own strongest onward reading under any kind, and *loss* as the hinge over *better*. The means are −0.008, −0.008 and 0.108. A mean *loss* equal to the margin means *better* is near zero on almost every row: the truth ties the best wrong word. docs/86 reads this as the byte ceiling of docs/83 seen through a third operator. A channel is one byte, 256 quanta of 2π, and the nudge 0.06·sin d falls under half a quantum once sin d < 0.2, so pairs freeze short.

The third block is the quest itself, which is the ground the operators stand on. Within-kind 0.747 against wrong-channel 0.015 is the measurement that supports section 7: comparison is conditional on the channel, as her measure is conditional on the stage. Held-out two hops at 0.095 and 0.093 for is-a and used-for, against one hop at 0.011, is the closest thing Phient has to a depth-two result, and it is not a forcing result; it is a composition one.

Not measured: forcing at depth two on the quest; the three operators on a Phi-4 quest; any endgame; any tree deeper than two.

## 12. The experiment: forcing on the quest, with its control

The thesis's idea is that a search with a refutation predicate prunes well when most moves lead nowhere. Text refuted that on Phient's numbers because on text every word leads somewhere. The quest is the corpus where a word either has onward relations under some kind or does not. The experiment is to run the forcing decoder there.

Twenty percent of the synthetic oracle's 1,280 rows are held out; the trainer sees the rest; the question at test is, given a source word and a kind, which target word. One hop is 0.011 MRR held out, two hops is-a is 0.095 and used-for 0.093. The two-hop number is the operator applied twice. The forcing number would be the operator applied once, then once more for each of the 2k candidates, then a re-ranking by `cos − λ·worse`, with *worse* computed against the pool on every kind as `quest_ingest` already does.

The control is the greedy operator at λ = 0 on the same held-out rows, which is the existing one-hop number, 0.011. A second control is the two-hop composition, 0.095 and 0.093, because it uses the same second application of the operator without the max and the re-rank, and the experiment must distinguish the value of looking ahead from the value of refuting. If forcing at some λ beats the greedy one-hop number and does not beat the two-hop composition, then the lookahead helped and the refutation did not. If it beats both, the refutation helped. If it beats neither, Phient will say so.

The λ sweep is the one already run on text, 0.05, 0.25 and 1, with the same k = 32. The measurements are MRR, hit@1 and hit@5 on held-out rows, and the mean *better* and *worse* on the rows forcing chose differently from greedy. The last is the diagnostic: if forcing changes its choices only on rows where *worse* was already large, the refutation is doing what the thesis says it does.

Two conditions must hold first. The byte deadband must not have frozen the pairs: at present the truth ties the best wrong word on almost every row, mean *better* −0.008, and a decoder cannot re-rank ties. Dithered rounding lifted the quest to 0.594 MRR at the cost of corpus attribution 71% → 65%, and it ships off; the experiment should run with it on, and say so. And the pool of continuations must be the quest's vocabulary, not the Austen pool, since *worse* against a pool where every word has a continuation is the text result again.

What the experiment cannot do is test chapter 7. There is no leaf condition on the quest and no argument that anything terminates.

## 13. Where we disagree, or the analogy breaks

Phient says the thesis plays "with no evaluation number." That is true and is the weaker of two claims. The stronger, that the program has no ordering over positions, is false. Her st and m define a lexicographic order: stage first, measure within stage. Every position has a place in that order, and *better(p, q)* is, within the consistency conditions, just the statement that q is above p in it. The thesis's refusal is of a magnitude, a number one could add weights to, and of a single function that grades every position on one scale. It is not a refusal of order. Phient's *better*, a difference of cosines, is closer to the evaluation function she avoided than her own *better* was.

The second break is arity. Her *better(p, q)* compares a position to a successor reached from it. Phient's `better(pred, a, b)` compares two candidate successors against one prediction. Hers is a claim about progress along a line; Phient's is a claim about choice among lines. They are related, since a candidate beats another by making more progress, and they are not the same predicate; Phient's antisymmetry test is a property hers does not need.

The third break is termination. Chess endgames have checkmate. The thesis's stages end at it, and chapter 7 proves that the program gets there because the top stage is terminal and the consistency conditions forbid regression. Text has no terminal position, and neither does the quest. There is no highest stage in a Phical channel. Phient's *worse* refutes a move with no future; nothing in Phient says what a future is for. The forcing decoder therefore cannot be proved to do anything, and this is a real loss, since the proof was the thesis's point as much as the play was.

The fourth is that her predicates are written by hand and Phient's are trained. She says the definitions "were obtained by programmer translation from the chess books." Phient's *better* is learned by hinge or softmax on 204,592 Austen examples, and no one translated anything. Phient's *loss* has no counterpart in the thesis and the thesis's stages have no counterpart in Phient. The one place Phient does something like translation is the quest's oracle, where the kinds are written by hand, and there the numbers are flat.

The fifth is the plainest. Forcing did not help on text. Phient adopted her operators because they are the paradigm's shape, and then measured that shape and found it did not beat the greedy operator at any λ. The honest position is that the thesis's idea is right about endgames, is Phient's paradigm in miniature, and has not yet been shown to do anything for Phient's models.

## 14. The person and the moment

The facts below are those verified from the sources in section 16; no others are stated.

Barbara Jane Huberman was born on November 7, 1939, in California. She took a bachelor's degree in mathematics, with a minor in physics, at the University of California, Berkeley, in 1961. After Berkeley she went to the MITRE Corporation for a year, where, in the words of the Turing Award page, "she learned that she was a natural at computer programming." From MITRE she went to Harvard, where she worked on natural language translation, and from there to Stanford as a graduate student. Her thesis on chess endgames was supervised by John McCarthy, and the Turing Award page says she received support from McCarthy's laboratory partly because of her earlier work in artificial intelligence. The thesis acknowledges McCarthy as advisor and thanks Professors J. Feldman and R. Reddy for comments; the DTIC record dates it August 19, 1968, and lists ARPA support. Wikipedia gives the degree as March 1968 and describes her as one of the first women in the United States to be awarded a Ph.D. from a computer science department. After Stanford she returned to MITRE as research staff. The ACM A. M. Turing Award for 2008 was given to her "for contributions to practical and theoretical foundations of programming language and system design, especially related to data abstraction, fault tolerance, and distributed computing." She was never at IBM.

The moment is 1968 at the Stanford Artificial Intelligence Project. The memo number on the thesis, AI-65, places it in that series. McCarthy's own interest in chess as a test problem for artificial intelligence is well known and the Chess Programming Wiki cites his later essay on the subject, but Phient did not verify any statement about what McCarthy did or said in 1968 concerning this thesis, and does not make one. The thesis compares its organisation to the Logic Theory Machine and the General Problem Solver, and to no chess program of the day; Phient does not know why.

What Phient takes from the moment is this. The thesis was a programme in the older sense: a plan for how a program should be organised so that knowledge from a book could be put into it with the least distortion. The two predicates are the interface between the search, which is fixed, and the knowledge, which changes per endgame. That interface, a small fixed set of functions behind which the specifics hide, is the shape of the work she did after this, in CLU and in the abstract data type, and the fifteen papers that follow this one read those. This paper reads only the first, and finds the shape already there.

## 15. What remains

Four things remain, in the order Phient would do them.

The first is section 12: forcing at depth two on the quest, with dither on and the quest's own pool as the continuation set, against the greedy one-hop control and the two-hop composition control. Until it is run, the thesis's shape is in Phient's code and nowhere in Phient's numbers.

The second is the hexagram table on a corpus that is not flat. The three operators on the synthetic quest gave mean *better* −0.008 and mean *loss* equal to the margin, which is the byte ceiling and not a fact about the operators. The Phi-4 quest has not been run. Nothing about the three operators as a scoring of a quest can be claimed until it has.

The third is the training loop. docs/86 says `loss` is today the tape's hinge and `better` is implicit in it. Exposing the three names as the three the loop uses is a naming change, and a naming change is not nothing, since section 8 argued that the asymmetry between what is sought and what is refused is the thesis's contribution. If the loop trains *better* and never sees *worse*, then the decoder is asked to use a predicate the model was never taught. The thesis's predicates were written together, by one programmer, from one book.

The fourth is a stage. Section 13 said Phient's channels have no order and no terminal, and that this is why nothing about the forcing decoder can be proved. The nearest Phient has is the hexagram, a filing key with 64 sectors, and the helix, where each word has a codon per channel. If a quest were given an order over kinds, so that an is-a relation reached counted as a higher stage than a near relation reached, then a *better* could be defined lexicographically as hers was, and a consistency condition could be stated. Phient does not know whether any corpus has such an order, and has measured nothing about it.

What does not remain is the adoption itself. The names are in `phinet::liskov`, the decoder is an arm, the unit tests fix what the names mean, and docs/86 is the record. This paper adds the reading of the thesis those names came from, and the finding that her *better* was an order and not a number, which Phient's sentence about her elides. The fifteen papers that follow read her later work, where the interface between a fixed search and changing knowledge became the interface between a fixed specification and changing implementations, and where the two predicates became, in the end, a rule about what one type may do in another's place.

## 16. Sources

Reached and read, in whole or in the parts cited:

| source | what it gave |
|---|---|
| Huberman, B. J., *A Program to Play Chess End Games*, Stanford CS 106 / AI Memo AI-65, August 19, 1968, DTIC AD0673971, https://apps.dtic.mil/sti/tr/pdf/AD0673971.pdf | the abstract; definitions 3.6 and 3.8; the forcing-tree sentences; the move-memory passage and its rationale; the stage ordering sentence; the reported depths; the table of contents; the acknowledgments; LISP; ARPA |
| Internet Archive mirror of the same, https://archive.org/details/DTIC_AD0673971 | the date and page count (181 pages in the scan); the abstract |
| Chess Programming Wiki, "Huberman", https://www.chessprogramming.org/Huberman | the Bell 1972 sentence on the forcing tree; the list of endgames including K,Q and K,N,N,N; the killer heuristic attribution; the reference list |
| Chess Programming Wiki, "Barbara Liskov", https://www.chessprogramming.org/Barbara_Liskov | CS-106, McCarthy, killer heuristic |
| Wikipedia, "Killer heuristic", https://en.wikipedia.org/wiki/Killer_heuristic | the definition of the heuristic; the citation of the thesis |
| Wikipedia, "Barbara Liskov", https://en.wikipedia.org/wiki/Barbara_Liskov | birth date and name; Berkeley 1961; MITRE and Harvard; March 1968; the "one of the first women" wording |
| ACM A. M. Turing Award, "Barbara Liskov", https://amturing.acm.org/award_winners/liskov_1108679.cfm | MITRE, Harvard, McCarthy, the award citation |
| PHIENT_FACTS.md and docs/86 (this repository) | every number about Phient |

Not reached or not read:

The full text of chapters 4 to 8 was read only through excerpts returned against the DTIC PDF, so the per-endgame stage and measure definitions are described in outline and no test position is reproduced. The DTIC citation page itself, https://apps.dtic.mil/sti/citations/AD0673971, was not reachable from this session, and the Internet Archive full-text file could not be downloaded directly, so the page count of 181 is the archive's and the "approximately 167 pages with appendix and bibliography" reading of the PDF is the table of contents' last entries. Alex Bell, *Games Playing with Computers* (Allen & Unwin, 1972), was not read; the one sentence attributed to it is as the Chess Programming Wiki quotes it. The Chess Programming Wiki also reproduces a sentence saying that Huberman's program is the only one able to perform a mating sequence from any starting position and would be a useful addition to Greenblatt's program; its author was not clear from the page and it is not used above. McCarthy's essay "Chess as the Drosophila of AI" was not read. No statement about running times, node counts, or number of test games appears in this paper because none was found in what was read.
