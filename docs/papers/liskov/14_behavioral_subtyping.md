# A stand-in is what still holds: substitution read as a claim about words

*Phient interpreted papers · Liskov series, 14 of 16 · 2026-07-13 · on: "A Behavioral Notion of Subtyping" (Barbara H. Liskov and Jeannette M. Wing, 1994, ACM Transactions on Programming Languages and Systems 16(6):1811–1841)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the fourteenth of sixteen papers in which Phient reads one work by Barbara Liskov through its own paradigm. The work is the 1994 TOPLAS paper with Jeannette Wing that made the substitution principle formal. Liskov stated the principle informally in her OOPSLA keynote of 1987, printed in 1988. The 1994 paper says what the property means when objects are mutable and shared, gives two definitions of the subtype relation that satisfy it, and adds a kind of property that earlier definitions had missed: the history property, a fact true of every pair of states an object passes through, and not merely of each state alone.

We read it here because the principle is already in Phient's code under its author's name. `Facet::relational_fit`, the Liskov reading, asks how well a candidate word stands in at a position in a sentence when read against every neighbour on both sides, each under the condition the sentence puts between them. A `bin/liskov` runs that reading on Austen. The facts file gives no number for either, so this paper describes them as existing and measures nothing by them. The paper is sharper than the informal principle the code paraphrases. It says substitution is a relation between specifications, not between objects. It says the relation is checked one method at a time by weakening what is required and strengthening what is ensured. And it says that with mutable objects a stand-in must also preserve what the original could never do, which is the history property, and which has no counterpart in the code today.

Sections 2 to 5 state what the paper says, in its terms, from the full text, which we were able to read. Sections 6 to 10 place it inside the Phient family: the word as a specification with an invariant and a constraint, the condition as the renaming map, the operator as the method rule, the history property as the count that only grows, and the stand-in test as a test the quest already fails in one direction. Section 11 lists what Phient has measured that bears on any of this, all from the facts file. Section 12 names an experiment with its control. Section 13 says where the analogy breaks. Section 14 is about the two authors and the year. Section 15 is what is left. Section 16 lists what we read and what we did not.

One limit up front. The paper's relation is proved from specifications a person writes. Phient has no written specification of a word, only a trained table and an oracle of rows. Everything below that calls a word a subtype is a reading and not a proof.

## 2. What the work claims, in its terms

The abstract states the position in one sentence: "This paper takes the position that the relationship should ensure that any property proved about supertype objects also holds for its subtype objects." Then: "It presents two ways of defining the subtype relation, each of which meets this criterion, and each of which is easy for programmers to use. The subtype relation is based on the specifications of the sub- and supertypes; the paper presents a way of specifying types that makes it convenient to define the subtype relation."

The introduction turns this into a requirement. "Subtype Requirement: Let φ(x) be a property provable about objects x of type T. Then φ(y) should be true for objects y of type S where S is a subtype of T." The informal version: "the objects of the subtype ought to behave the same as those of the supertype as far as anyone or any program using supertype objects can tell." "We are interested only in safety properties ('nothing bad happens')." They are divided in two. "We focus on two kinds of such properties: invariants, which are properties true of all states, and history properties, which are properties true of all sequences of states." The example of a history property is that a bounded bag's bound does not change. "We formulate invariants as predicates over single states and history properties, over pairs of states."

The claim the paper makes for itself, in its conclusions, is priority on the second. "This paper defines a new notion of the subtype relation based on the semantic properties of the subtype and supertype. We require that a subtype preserve the behavior of the supertype methods and also all invariant and history properties of its supertype." Then: "Ours is the first work to deal with history properties, and to provide a way of determining the acceptability of the 'extra' methods in the presence of mutability."

The extra methods are the difficulty. The running example is a stack, read as a subtype of a bag, with a method swap_top that the bag does not have. Earlier definitions ignored such methods. The paper says why that will not do: "we cannot ignore extra methods in the presence of aliasing, and also in a general computational environment that allows sharing of mutable objects by multiple users." If a program holds a bag and another name for the same object is a stack, swap_top through the second name changes what the first sees. The first program reasoned about a bag, and its reasoning must survive. Substitution is a promise about reasoning, and it must hold across every state the object will ever be in, including states reached by methods the reasoner never heard of.

## 3. The mechanism, plainly

The paper builds a small model of computation and then a relation on specifications. "A type defines a set of values for an object and a set of methods that provide the only means to manipulate that object." A computation is "a sequence of alternating states and transitions starting in some initial state." A history "is the subsequence of states of a computation," and p and q range over states where p precedes q. Methods are creators, observers or mutators.

A type specification, in the Larch style, has a name, a value space, and for each method a signature with exceptions, a requires clause and an ensures clause. Two clauses are added. The invariant "defines the legal values of its type T"; creators establish it and methods preserve it. The constraint is a predicate over a pair of states, and holds "for all computations, c, and all states p and q in c such that p precedes q." The bounded bag's constraint is that its bound in p equals its bound in q.

The subtype relation is then a table of rules. A type σ is a subtype of τ when there is an abstraction function A from σ's values to τ's and a renaming map R from τ's methods to σ's, such that the following hold. The invariant rule: for every s of type σ, σ's invariant at s implies τ's invariant at A(s). The methods rule: corresponding methods have the same number of arguments, each argument type of τ's method is a subtype of σ's (contravariance), the result type of σ's method is a subtype of τ's (covariance), and σ's method signals a subset of τ's exceptions. The pre-condition rule: τ's pre-condition, with A applied, implies σ's. The post-condition rule: σ's post-condition implies τ's, with A applied before and after. The constraint rule: σ's constraint over any two states implies τ's over their images under A.

That is the constraint approach. The extension approach handles extra methods differently. Each extra mutator is given an explanation, "The extension map, E," a program written in the supertype's methods. The extra method and its explanation, run from the same state, must reach states with the same image under A. The price is that pre-conditions of inherited methods must then be equal, not merely weaker.

Stack under bag passes both. "The get method for bags does not specify precisely what element is removed; the pop method for stack is more constrained, but what it does is one of the permitted behaviors for bag's get method." Swap_top is a pop then a push. The bound never changes.

## 4. What it demonstrated, as reported

The paper is a definition and its consequences. What it demonstrates is that the definition sorts examples the way a designer's judgement would, and sorts some the other way.

The first is the pair the paper is built on. A stack is a subtype of a bounded bag: pop's behaviour is one of get's permitted behaviours, push does what put does, swap_top can be explained by pop and push, and the bound is preserved. The nondeterminism in the supertype is what makes the relation possible: "the specification of get is nondeterministic because it does not constrain which element of the bag is removed. This nondeterminism allows stack to be a subtype of bag: the specification of pop constrains the nondeterminism." A supertype that pinned down which element get removed would have no subtypes worth the name.

The second example fails, and fails on history alone. A fatset is a set of integers that only grows. An intset grows and shrinks; it has a delete method. Every state of an intset is a legal state of a fatset, and every method the fatset has, the intset has with the same behaviour. And yet "intset is not a subtype of fatset because fatsets only grow while intsets grow and shrink, i.e., it does not preserve various history properties of fatset." Intset's trivial constraint cannot imply fatset's.

The third is integers. "Smaller integers cannot be a subtype of larger integers because of observable differences in behavior; for example, an overflow exception that would occur when adding two 32-bit integers would not occur if they were 64-bit integers." The paper's remedy is the virtual type: "Sometimes supertypes are not intended to be implemented; instead they are virtual types that let us define the properties all subtypes have in common." Integer becomes a virtual supertype with the sized integers beneath it, and bags of different bounds are treated the same way.

The two definitions are compared as a trade. "The constraint approach is appealing because it is simple and direct. The specification visually highlights a type's history properties that must be preserved by its subtypes." Its cost is that the specifier must write constraints strong enough. The extension approach keeps the derivation of history properties from mutators, but "requires that the pre-conditions of the inherited methods be identical to those of the corresponding supertype methods; with the constraint approach, a subtype's method's pre-condition can be weaker than that of the supertype." The paper prefers the constraint approach. Against America, Meyer, Leavens and Cusack, its reported difference is one sentence: "none of these authors considers the problems introduced by extra mutators nor the preservation of history properties. Therefore, they allow certain subtype relations that we forbid."

## 5. The one idea restated

A thing may stand in for another wherever everything anyone could prove about the original still holds of the stand-in. That is the requirement. Everything else in the paper is the discovery of what "everything anyone could prove" contains, and the answer is more than a reader of the 1987 keynote would have guessed.

It contains what each method does, which is the part every earlier definition had. The stand-in may ask less before it acts and promise more after, never the reverse. It may take broader arguments and return narrower results. It may raise fewer exceptions. Each of these is a direction, and the directions are all the same direction: the stand-in must be at least as usable and at least as predictable as the original, method by method.

It contains what is always true of the object, the invariant, which the stand-in must keep within the original's legal values under the abstraction function.

And it contains what is true across time. This is the paper's addition and the reason it was written. A user of a fatset may prove that an element, once in, stays in. Nothing about any single state of an intset contradicts that. Only the passage between states does, and only when some other holder of the same object calls delete. The paper's insight is that a stand-in is judged on the transitions it can make, including transitions the original's user never asked for, and never on its states alone. A property over pairs of states is a property of the object's whole life, and a subtype inherits the supertype's whole life.

Restated for Phient, the sentence reads: a word may stand in for another under a condition wherever every relation the context demands still holds. That is what `relational_fit` computes, and it is the 1987 sentence. The 1994 paper says three things the code does not. First, the demands are read from a specification and not from the sentence at hand, so a stand-in that passes one sentence has not been shown to pass the type. Second, the check has a direction: the stand-in may relate to more and promise more, not less. Third, a word has a history, and a stand-in must preserve it. In Phient a word's channel carries a count as well as a phase, and the count only grows. That is a constraint in the paper's exact sense, and section 9 makes it the center of the reading.

## 6. The word as a specification

The paper says a type is a value space and a set of methods, each with a pre-condition and a post-condition, together with an invariant over states and a constraint over pairs of states. Take that shape and ask what a Phient word has. Its value space is a table of 32 to 64 channels, each a complex phasor on a closed circle with a count. Its methods are the ways the rest of the system may interrogate it: the operator X ⇒ Y under a condition, and the three named comparisons better, worse and loss. Its invariant is that every phase lies on the circle, in 256 quanta of 2π, and every count is a non-negative integer. Its constraint is that a count never decreases. Nothing in the paradigm says a count decreases, and the trainer only ever adds evidence.

That is a specification in the paper's sense. Earlier papers in this series gave the word operations. This one gives it a pre-condition, a post-condition and a history.

The pre-condition of the operator is that both words carry evidence on the channel the condition selects. `relational_fit` writes this down: a pair is counted only when `evidence_count(k) > 0` on both sides. Where the pre-condition fails the operator is not called, and the reading returns the mean over the pairs that remain, with their number. The paper would say the operator's requires clause is evidence on both sides, and that any stand-in for a word must accept at least those cases. A candidate with evidence on fewer channels asks more before it can be read, and by the pre-condition rule it is not a subtype of the word it replaces.

The post-condition is what the operator ensures: a phasor moved toward the target under the condition, by an amount the count weighs. The paper's post-condition rule says the stand-in must ensure at least this. Under the softmax operator on Austen the shipped table ensures a rank against the pool, 30.7× chance at 8 epochs, hit@10 0.348. A stand-in table that ensured less would fail the rule.

The invariant is where the specification is strongest and least interesting. The circle closes, so every phase is legal.

The constraint is the count. It is also the only clause a stand-in could break without any single reading noticing, which is the paper's point exactly. Two tables with identical phases and one with counts that had been reduced would read alike today and diverge under the next epoch of training. The paper says the second is not a subtype of the first. Phient has never checked.

## 7. The condition as the renaming map

The paper's subtype relation needs two maps before any rule is checked: an abstraction function A from the subtype's values to the supertype's, and a renaming map R from the supertype's methods to the subtype's. Without R there is no method to compare, and without A there is no state to compare it at.

Phient's condition c does the work of R. Under is-a, the operator reads one channel of the word; under part-of, another. When the quest asks whether a word may stand in for another under is-a, it is asking about the method R selects for that condition and no other. Wrong channel reads 0.015 MRR against 0.594 within the right one. That gap is the renaming map failing to find a method: the question was asked of the wrong clause, and the specification, correctly, says nothing.

The abstraction function is subtler, and Phient has more of it than it has admitted. The paper's rules never compare a subtype value to a supertype value directly. They compare A(s) to t. In the helix, the same 18,496 words are written as circle sectors, as 64 hexagrams, as sphere degrees and orders, and as base-4 codons, and read back byte-exact. Each of those writings is an abstraction function from the byte table to a coarser value space, and each is used to file the word under a supertype: a hexagram is a type that 256 quest words fall into 61 of 64 ways. When docs/86 files a word by hexagram and reports mean better, worse and loss per hexagram, it is reporting the subtype's methods at the supertype's values, through A. That is the paper's procedure, done by hand.

The paper also says A must respect the invariant: a legal subtype value maps to a legal supertype value. The helix passes this by construction, since a byte-exact round trip cannot produce an illegal value. The sphere tie passes it another way. Tie::Sphere maps 64 channels to spherical-harmonic degrees l < 8 and back, and agrees with scipy sph_harm_y to 2.4e-15, so the abstraction function is the same function under two implementations.

What Phient lacks is the second half of R. The paper's renaming map is partial: methods of the subtype not in its range are the extra methods, and those get the constraint rule or the extension map. Phient's conditions are a fixed set of channels, and no word has a channel another word lacks. There are no extra methods, and so the paper's hardest case does not arise. Section 13 says why that is a weakness of the reading and not a strength of the design.

## 8. The operator as the method rule

The paper's method rule has five parts, and each names a direction. Arguments may widen. Results may narrow. Exceptions may shrink. Pre-conditions may weaken. Post-conditions may strengthen. A subtype is safer to call in every one of these five senses.

The operator X ⇒ Y under c is one method, and the three named operators of docs/86 are three more. Each takes words and returns a comparison. Read them against the five parts.

Arguments. better(pred, a, b) takes a prediction and two candidates. A stand-in for the method could accept a broader pool of candidates and still be a subtype. The decoder's forcing_choice already does this in one direction: nearest_forcing widens the pool to 2k words before re-ranking. The paper would call that contravariance of arguments, and permit it.

Results. The operator returns a phasor, and better returns a difference of two cosines. A stand-in that returned a narrower thing, say a sign rather than a magnitude, would be covariant in the result and permitted. A stand-in that returned a broader thing, a complex number where a real was promised, would not.

Exceptions. Phient's operators do not signal. Where the paper's pre-condition fails, `relational_fit` returns zero pairs, and the caller reads the count. This is the paper's exception rule met by having no exceptions, which the paper allows, since the empty set is contained in every set.

Pre-condition. Evidence on the channel, both sides. A stand-in must accept at least that.

Post-condition. Here the operator's contract is a rank. The hinge ensures a margin of better between target and decoy; loss(pred, target, decoy, margin) is the amount by which the margin was missed. The paper's post-condition rule, applied, says that a subtype operator must ensure at least the supertype's margin. The identity operator, 7.9× chance, ensures the least. The hinge at 8,360 params ensures 14.7× valid. Softmax wide and deep at 50,016 params ensures 30.7×. These are not subtypes of one another in the paper's sense, since they are implementations of one method rather than methods of two types. But the ordering is the ordering the post-condition rule imposes, and it says which implementation could stand in for which. Any program written against the identity operator's guarantee is satisfied by the softmax. No program written against the softmax's guarantee is satisfied by the identity.

The paper says a rule of this kind is checked from specifications, once, and not from runs. Phient checks it from runs, on a held-out split, with a chance MRR of 0.0063 as the floor. That is a statistical post-condition, and section 13 says what it costs.

## 9. The count as a history property

The paper's contribution is the history property, and Phient has exactly one. A channel is a phase and a count. The phase moves. The count only grows. In the paper's form, for states p before q, count_p(k) ≤ count_q(k) for every channel k. That is a constraint clause, and it holds for every word in every table Phient has built.

The paper's fatset is the closest published example. A fatset only grows. An intset, which grows and shrinks, is not its subtype even though every state of one is a legal state of the other and every shared method behaves alike. The difference lives entirely in the transitions. A table whose counts could be reduced would stand to Phient's word as intset to fatset. It would read identically in any single state and could not be substituted, because the trainer, which weighs a nudge by evidence, would reason wrongly after a reduction it never saw.

Two places in Phient already lean on the constraint without naming it. The first is forgetting. There is a `bin/forgetting` in phiano, and the facts file gives no number for it, so nothing is claimed about what it does. But any forgetting that lowers a count breaks the constraint, and a table produced by it is a different type from the table it came from. The paper would ask that forgetting be declared, or that the constraint be weakened to admit it.

The second is the helix. The round trip of 18,496 words is byte-exact, which is a statement about a single state. Is the helix a subtype of the table under the constraint? It is, trivially, if nothing is written to it but a fresh dump. It is not shown to be if the helix is read, trained on and written back, because no test checks that counts in the codon form only grow.

The extension map has a reading too. Dithered rounding is Phient's extra mutator. The byte deadband froze every pair 8 quanta short because a nudge of 0.06·sin d falls below half a quantum once sin d < 0.2. Dither adds noise before rounding so that small nudges land. Its explanation, in the paper's sense, is that many small nudges under dither reach the abstract phase one large nudge would have reached without it. Whether that holds is a diamond the code has not drawn. What is known is the cost: dither takes corpus attribution from 71% to 65%, so it ships off. Under the extension approach, that loss says the explanation does not close, and dither is not a subtype of the plain trainer. It is a different type that scores better on one task and worse on another.

## 10. The stand-in test in the quest and the sentence

The paper's requirement is a test. Take a program that reasons about T. Substitute an S. If the program is still right, S may be a subtype of T. Phient runs two things that look like this test, and each is some distance from it.

The first is `relational_fit`. Given a sentence, a position and a candidate word, it puts the candidate in the position and reads it against every neighbour within the relation window, on both sides, each under the channel the sentence's geometry assigns to that pair. For a neighbour before, the candidate's target side is read against the neighbour's source side; for one after, the reverse. It returns the mean agreement and the number of evidenced pairs. `bin/liskov` runs this as a cloze on held-out Austen sentences and as substitution proper, placing the best stand-in in her sentence and letting a Kneser-Ney model of her say how much it minds. The facts file gives no number for this.

Read against the paper, `relational_fit` is the requirement applied to one program: this sentence. A candidate that passes has been shown to stand in here. It has not been shown to stand in for the type. The paper's φ ranges over every property provable of T, and the sentence is one property. The Kneser-Ney judge is a second program, and the paper's point is that the stand-in must satisfy both, and every other.

The second is the quest. The synthetic oracle gives 1,280 rows over 256 words in families of 8, under five kinds. Held out 20%, the trained table recovers one hop at 0.011 MRR, chance 0.021, and two hops at 0.095 on is-a, 0.093 on used-for, and about chance on part-of, near and unlike. This is a substitution failure in the paper's exact shape. A row the trainer saw is a property φ proven of the training words. A held-out row asks whether φ holds of a word that was supposed to stand in for them, a sibling in the same family under the same kind. It does not. Whatever the table learned about the family, it learned as facts about particular words. In the paper's language, the family is not a virtual type with properties the members inherit. It is a name.

The paper says why. It happens when the supertype is specified by enumeration rather than by contract. A bag whose get was specified to remove a particular element would admit no stack. A family whose is-a was specified as a list of pairs admits no unseen sibling. The paper's remedy is nondeterminism in the supertype: say less, so that more can stand in. Phient's oracle says exactly what it says, and nothing stands in.

## 11. What Phient has measured that bears on it

Nothing in gemphi proves a subtype relation from specifications, so nothing measured is a measurement of the paper's mechanism. What has been measured is whether substitution holds where the reading above says it should, and how far it fails. All numbers are from PHIENT_FACTS.md.

The Liskov reading itself. `Facet::relational_fit` and `bin/liskov` exist. The facts file gives no number for either. Not measured.

Post-condition ordering of the operator. On Austen, pool 1,228 words seen 20 or more times, 204,592 training examples, chance MRR 0.0063: identity 7.9× chance; hinge at 8,360 params 14.7× valid and 14.6× test; softmax β=8 18.0× and 18.8×; softmax wide and deep at 50,016 params 27.6× at 3 epochs and 30.7× and 30.3× at 8, hit@10 0.348. Each ensures at least what the one before it ensures.

Abstraction functions that preserve the invariant. Sphere tie, 1,192 params, hinge 3 epochs 12.8× valid and 13.1× test, hit@10 0.182, 89 s, against the circle's 14.7× and 14.6× at 8,360 params in 142 s. Anisotropic tie, 36 slots and 4,776 params, 16.0× and 15.6×, hit@10 0.254, 85 s. Sphere softmax β=8 13.2× and 13.5×, hit@10 0.189, 408 s. Synthetic degree-filter, 200 items, 40 epochs: circle 0.662, identity 0.646, sphere 1.000. phinet::sphere against scipy sph_harm_y: 2.4e-15. Helix: 18,496 words in 1.9 s, 75 MB, byte-exact.

The renaming map. Quest within-kind 0.747; wrong channel 0.015; offset reading 0.005.

Substitution to unseen members. Quest, chance 0.021: shipped trainer 0.082; attraction-only with dithered rounding 0.594, hit@1 34.8%, hit@5 97.4%. Held out 20%: one hop 0.011; two hops is-a 0.095, used-for 0.093; part-of, near and unlike about chance.

The extra mutator. Deadband: nudge 0.06·sin d falls under half a quantum once sin d < 0.2, at 11°; nearest rounding froze every pair 8 quanta short. Dither costs attribution 71% to 65% and ships off by default.

Programs reasoning about a persona. Attribution across Chesterton, Austen and Carroll on held-out 4-sentence paragraphs, chance 33%: 71% overall, 74, 65 and 86. Dialogue: 18 replies judged as their persona 44% by the sector reader and 72% by the Kneser-Ney panel.

The three operators. decode_eval k=32: nearest-k KN 66.9; forcing λ=0.05 69.7, λ=0.25 68.5, λ=1 107.4 with evaluator 0.712, distinct 0.777, shared 4-grams 22.3%. Quest by hexagram: 61 of 64 occupied; mean better −0.008, worse −0.008, loss 0.108 at margin 0.1.

Decoding. 40 prompts by 30 tokens: sampler KN 13.3, operator nearest-k 66.9, union 66.8; shared 4-grams 25.7% and 8.7%. Parity with the PyTorch reference: 62 of 62. Engine allocation and bandwidth bound, 1 MB op 2 to 5 ms.

Not measured: any number from `bin/liskov`; whether counts survive a helix round trip through training; anything about forgetting; the stand-in table of section 12; any Phi-4 run.

## 12. The experiment, with its control

Phient cannot enumerate every property provable of a word, but it can name several programs that reason about one and ask whether a stand-in satisfies all of them or only the one it was chosen by. That is the gap section 10 found in `relational_fit`.

Setup. Take the Austen Facet as trained and the pool of 1,228 words. For each of the 40 held-out prompts, and for each content-word position in them, rank the pool by `relational_fit` at that position and take the top stand-in that is not the original word. By construction the stand-in passes this program. Now ask three others whether they agree. The first is the operator, wide and deep at 50,016 params: run the prompt with the stand-in in place and record the rank of the true next word, against the original. The second is persona attribution: feed the altered paragraph to the three-writer classifier and record whether it still says Austen, against the baseline 71% overall and 65% for Austen. The third is the Kneser-Ney model of Austen: record perplexity of the altered sentence against the original.

Claim under test. If `relational_fit` reads the type and not the sentence, a stand-in it chooses should leave the three other programs' verdicts within their run-to-run spread. If it reads only the sentence, the operator's rank, the attribution and the perplexity will each degrade, and the size of each degradation is the size of the property the reading missed.

Control. The first control is a stand-in drawn at random from the pool with the same corpus count as the chosen one, which separates frequency from fit. The second is the stand-in ranked by `relational_score`, the left-context-only reading, which separates the both-sides reading from the one-sided one. The third is the original word, which sets the ceiling.

A second arm, the constraint. Repeat the substitution, then run one further epoch of training on the altered corpus and compare every channel count in the stand-in's row before and after. The counts must not decrease. Any decrease is a broken history property.

What would count. Stand-ins within the spread of the original on all three programs, and above both controls by the margin `relational_fit` is above chance on the cloze. Anything else says the reading is a sentence test and not a type test, which the paper would have predicted. Not run. The cost is one pass of the operator per altered prompt, one classifier pass per paragraph, and one epoch of training on the second arm, which at Austen scale the facts file puts near 142 s on the circle and 85 s on the anisotropic sphere.

## 13. Where we disagree, or the analogy breaks

The first break is that the paper's relation is proved and Phient's is scored. A subtype in the paper either satisfies the pre-condition rule or it does not, from two texts, before any program runs. Phient's post-condition is a rank on a held-out split, and its stand-in test is a mean over evidenced pairs. A word that stands in 59% of the way is not a subtype. We accept this.

The second break is the absence of specifications. The paper's whole method is to relate two written texts. Phient has one oracle of rows, no requires clause, no ensures clause and no constraint written anywhere. Section 6 wrote the word's specification in prose. It is a reading of the code, and the code is not checked against it. The paper would say that until it is, nothing has been specified.

The third break is extra methods, which is the paper's own subject. Every Phient word has the same channels. No word has a method another lacks, so the paper's hardest case, the extra mutator seen through an alias, cannot occur at the level of words. It occurs one level up, between trainers, and section 9 read dither and forgetting that way. But the reading is a stretch. The paper's extra method belongs to the subtype and is called by a second user; Phient's is a change of trainer applied to every word. Aliasing, which the paper says is why the problem exists, has no counterpart in a table where every word has one name.

The fourth is direction. The paper's rules are asymmetric. A subtype may require less and ensure more. `relational_fit` is symmetric: it scores how well a candidate agrees with its neighbours, and a candidate that agrees more is not thereby one that requires less. The code paraphrases the 1987 sentence, which is symmetric too. The 1994 paper's advance over it was to say which way the arrows point, and the code has not taken that advance.

The fifth is the history property. Phient has one, the count, and has never tested it. That is not a break in the analogy. It is the analogy pointing at an omission.

Last, the paper is about a person who holds a supertype and proves things. Phient has no such person. The trainer, the decoder, the classifier and the quest scorer reason about a word, and none of them proves anything. They degrade. The paper's requirement, that every proof survives, becomes in Phient the requirement that no program's score falls, and that is a weaker thing, since a score can fall a little without anyone being wrong. Section 12 is built on the weaker thing.

## 14. The person and the moment

Barbara Liskov, born Barbara Huberman, took her PhD at Stanford in 1968 under John McCarthy. She worked at MITRE, joined the MIT faculty in 1972, and by 1994 had behind her the Venus operating system, CLU, Argus and the work on distributed systems that the earlier papers in this series read. The paper lists Jeannette M. Wing's affiliation as Carnegie Mellon University and Liskov's as the MIT Laboratory for Computer Science. The funding note records Liskov's support from ARPA through the Office of Naval Research and from the National Science Foundation, and Wing's from ARPA through Wright Laboratory.

The principle is older than the paper. Liskov gave the keynote at OOPSLA in 1987 under the title "Data Abstraction and Hierarchy", and it was printed in SIGPLAN Notices 23(5) in May 1988. The sentence in it that the field took up reads: "What is wanted here is something like the following substitution property: If for each object o1 of type S there is an object o2 of type T such that for all programs P defined in terms of T, the behavior of P is unchanged when o1 is substituted for o2, then S is a subtype of T." By the account on the Wikipedia page for behavioral subtyping, Liskov said in a 2016 interview that the keynote gave only an informal rule, and that it was Wing who proposed they try to work out precisely what it meant. The paper is the result of that proposal. Its footer records receipt in July 1993, revision in April 1994 and acceptance in May 1994. It appeared in TOPLAS 16(6), November 1994, pages 1811 to 1841.

The moment was the arrival of object-oriented languages in wide use, with inheritance as their central mechanism and no agreed account of what a subclass owed its parent. The paper's related-work section names America, Meyer, Leavens, Dhara and Cusack as the contemporaries who had proposed specification-based definitions, and says that none of them handled extra mutators or history properties. The other pressure is the paper's own setting: shared mutable objects in object-oriented databases, with Thor, Liskov's group's system, cited as one. The bag that one user holds and another holds as a stack is a database object.

The name "Liskov substitution principle" is not in the paper, and the paper does not use it. The page for behavioral subtyping says the characterization from the keynote "has since been widely known" by that name and quotes Liskov's own remark that technically it is called behavioral subtyping. We did not find a verified account of who first used the shorter name, and state none. We have not read the 2008 Turing Award citation and quote nothing from it.

## 15. What remains

The paper leaves Phient one specification to write, one property to test, one asymmetry to add to the code, and one experiment.

The specification is the one section 6 wrote in prose. A word's value space, its invariant, its constraint on counts, and for each of the four operators a requires clause and an ensures clause. Docs/86 lists as remaining work that better, worse and loss become the only names by which the system touches a word. This paper adds that each name needs its two clauses, and the table its constraint. Until then, in the paper's terms, the word is unspecified and no stand-in for it can be checked.

The property to test is the count. It only grows, the trainer relies on that, and no test says so. A test that runs one epoch, compares every count before and after, and fails on any decrease is small. The same test run across a helix round trip, with training between the write and the read, would say whether the helix is a subtype of the table under the constraint or only equal to it in one state. Neither has been done.

The asymmetry is in `relational_fit`. It scores agreement and returns a mean. The paper says a stand-in must require less and ensure more, and a mean of agreements does not distinguish a candidate that fits because it is broader from one that fits because it is narrower. A second reading that counts, for each neighbour pair, whether the candidate carries evidence on at least the channels the original carried, would be the pre-condition rule. A third, whether the candidate's agreement on each of those channels is at least the original's, would be the post-condition rule. Each is a few lines beside the existing function and each returns a direction rather than a score.

The experiment is section 12's. It asks whether the Liskov reading reads the type or the sentence, and it has not been run. `bin/liskov` exists and its numbers are not in the facts file, so the first step is to run it and file what it says.

Behind all four is the paper's larger instruction, which is the one the quest already exposes. A family in the oracle is a list, and nothing stands in for a member of a list. The paper says a supertype admits subtypes to the extent that it leaves things open. A quest oracle written as a contract, with nondeterminism where the family does not care, would be the first specification in Phient that a held-out word could satisfy. The Phi-4 quest, which the facts file says has not been run, is where that would be tried.

## 16. Sources

The paper's full text was read from the copy at Carnegie Mellon, https://www.cs.cmu.edu/~wing/publications/LiskovWing94.pdf, which is the TOPLAS printing with page numbers 1811 to 1841. Every quotation from the paper above is taken from that copy: the abstract, the Subtype Requirement, the sentences on safety properties, invariants and history properties, the model of computation in section 3, the invariant and constraint clauses, the rules of the subtype relation in Figure 4, the extension map, the stack and bag passage, the swap_top discussion, intset and fatset, the 32-bit and 64-bit integers, virtual types, the comparison of the two approaches, the sentences on America, Meyer, Leavens, Dhara and Cusack, the conclusions, the funding note and the received and accepted dates. The copy is an OCR scan and some words in it are broken; where a quotation above differs from the scan it is by restoring a word the scan split or misread, and nothing more.

Also reached. The CMU Venari group page for the paper, https://www.cs.cmu.edu/Groups/venari/subtype-toplas.html, for the abstract as clean text and the authors' affiliations. The MIT Programming Methodology Group abstract page, https://pmg.csail.mit.edu/pubs/liskov94behavioral-abstract.html, for the citation. The Wikipedia page "Behavioral subtyping", for the quotation from the 1987 keynote as printed in SIGPLAN Notices 23(5), May 1988, for the account of the 2016 interview, and for the remark that the informal characterization has since been known as the Liskov substitution principle. The Wikipedia page "Liskov substitution principle", for the list of requirements and the 1994 citation; as fetched it gave neither the keynote quotation nor the origin of the name. The ACM Digital Library entry, doi 10.1145/197320.197383, was not fetched.

Not reached or not consulted. The 1987 keynote itself, "Data Abstraction and Hierarchy", which we quote only as the Wikipedia page quotes it. The 2016 interview, which we report only as that page reports it. Liskov and Wing's earlier versions of the work, including any 1993 technical report, and the 1999 CMU technical report "Behavioral Subtyping Using Invariants and Constraints", which are mentioned nowhere above as sources. The works by America, Meyer, Leavens, Dhara and Cusack, which are described only as the paper describes them. The Turing Award citation, which we have not read. Any biography of Wing beyond her affiliation as printed in the paper.

Phient sources. PHIENT_FACTS.md in this folder, for every number. docs/86_liskov_three_operators.md for the three operators, forcing and the hexagram filing. The source files phiano/src/facet/facet_relational_fit.rs, phiano/src/bin/liskov.rs and phiano/src/bin/quest_ingest/liskov.rs, read for what the Liskov reading computes and how `bin/liskov` uses it; no output of theirs is in the facts file, and none is cited.
