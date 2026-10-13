# A type is its operations: clusters, iterators and where clauses, read as words

*Phient interpreted papers · Liskov series, 6 of 16 · 2026-03-23 · on: "Abstraction Mechanisms in CLU" (Barbara Liskov, Alan Snyder, Russell Atkinson, Craig Schaffert, 1977, Communications of the ACM 20(8), pages 564–576)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the sixth of sixteen papers in which Phient reads one work by Barbara Liskov through its own paradigm. This one reads the paper that gave abstract data types a language. "Abstraction Mechanisms in CLU" appeared in the August 1977 issue of Communications of the ACM, volume 20, number 8, pages 564 to 576. Its four authors were all at the MIT Laboratory for Computer Science. It had been presented earlier that year, on 28 to 30 March 1977, at the SIGPLAN, SIGOPS and SIGSOFT Conference on Language Design for Reliable Software in Raleigh, North Carolina.

The paper's claim is that three kinds of abstraction are useful in building programs, that ordinary languages support only one of them, and that CLU supports all three. The three are procedural, control and data abstraction. The mechanisms are the procedure, the iterator and the cluster. A fourth mechanism, the parameterised cluster with its where clause, lets one cluster stand for a family of types while the compiler still checks every use. The paper demonstrates all of this on one small program, a word counter, and closes with the CLU library, in which an abstraction is entered as an interface before any implementation exists.

We read it here because its central sentence is Phient's. The paper says that if a data abstraction is to be understood at an abstract level, "the behavior of the data objects must be completely characterized by the set of operations." Phient's paradigm says a word is nothing but what it relates to, under what conditions. A CLU type has no meaning beyond its operations. A Phient word has no meaning beyond its relations. Both refuse the stored point: the representation exists, and it is hidden, and nothing outside may read it.

Sections 2 to 5 state what the paper says, in its own terms. Sections 6 to 10 place its four mechanisms inside the Phient family: the cluster as the word, the where clause as the condition, the procedure as X ⇒ Y, the iterator as decoding and the quest, the library as the helix and the channel discipline. Section 11 lists what Phient has measured that bears on it. Section 12 names one experiment and its control. Section 13 says where the analogy breaks.

One limit is stated at the top. We read the paper from a PDF mirror at the University of Michigan and the abstract from the ACM pages. The paper contains no performance table, and none is invented here.

## 2. What the paper claims, in its own terms

The abstract states the whole position in nine sentences. "CLU is a new programming language designed to support the use of abstractions in program construction. Work in programming methodology has led to the realization that three kinds of abstractions—procedural, control, and especially data abstractions—are useful in the programming process. Of these, only the procedural abstraction is supported well by conventional languages, through the procedure or subroutine. CLU provides, in addition to procedures, novel linguistic mechanisms that support the use of data and control abstractions."

The paper is an introduction, and says so: "This paper provides an introduction to the abstraction mechanisms in CLU. By means of programming examples, the utility of the three kinds of abstractions in program construction is illustrated, and it is shown how CLU programs may be written to use and implement abstractions. The CLU library, which permits incremental program development with complete type checking performed at compile time, is also discussed."

The introduction then defines the term the title turns on. "An abstraction isolates use from implementation: an abstraction can be used without knowledge of its implementation and implemented without knowledge of its use." That sentence is the paper's thesis, and each mechanism is judged by how well it holds the two sides apart. A procedural abstraction performs a computation on input objects and produces output objects. A data abstraction, in the paper's words, "is used to introduce a new type of data object that is deemed useful in the domain of the problem being solved." A control abstraction "defines a method for sequencing arbitrary actions."

The claim about data abstraction is the sharpest. The user of a type sees only its behaviour and its operations, never its representation or the algorithms inside the operations. For that to work the language must enforce it: "The language must limit access to the representation to just the operations." CLU does this with the cluster and with type checking. The claim about control abstraction is that most loops exist to act on the members of a collection, and that a collection type should be able to say how its members are visited without exposing how they are stored. CLU does this with the iterator.

The paper also claims that this discipline pays off in proof. The compiler, checking types module by module, performs a small part of the proof of correctness. The rest, the specifications and assertions, the authors say "should be expressed in a separate 'specification' language", which the paper defers. It names Alphard as "the language closest to CLU" and declares one omission: the exception handling mechanism is described elsewhere.

## 3. The mechanism, plainly

The paper's semantics rest on one distinction. "Objects are the data entities that are created and manipulated by CLU programs. Variables are just the names used in a program to refer to objects." Variables are not objects. An object may be mutable, in which case it "has a state which may be modified by certain operations", or immutable. Objects live in a dynamic storage area and are reclaimed when nothing can reach them. Assignment does not copy: after x := y the one object is shared by x and y. Argument passing works the same way. Every variable is declared with the type of object it may denote.

A type, in the paper's words, "defines a set of operations that create and manipulate objects of that type." A cluster implements one. Inside a cluster the reserved identifier rep names the representing type, and the paper's rule is exact: "The reserved type identifier rep indicates that the type specification to the right of the equal sign is the representing type for the cluster." Outside the cluster no code can see the rep. Inside it the keyword cvt marks the points where a value changes viewpoint, from the abstract type to the representation on entry and back on return. "Cvt can be used only within a cluster, and conversion can be done only between the single abstract type being defined and the (single) representation type."

The iterator is a cluster operation that yields rather than returns. It is invoked from a for statement. "Each time a yield statement is executed in the iterator, the objects yielded are assigned to the variables declared in the for statement." The loop body runs, and then "an iterator is resumed at the statement following the yield statement, in the same environment as when the objects were yielded." The collection decides the order. The loop decides the action. Neither sees the other's insides.

The parameterised cluster takes a type as an argument and states, in a where clause, what that type must provide. The paper's example is a sorted bag: `sorted_bag = cluster [t: type] is create, insert, ... where t has lt, equal: proctype (t, t) returns (bool)`. The item type must supply a less-than and an equality, each taking two items and returning a boolean. The compiler checks every instantiation such as sorted_bag[string] against that clause. "Type checking is performed on a module by module basis at compile time (it could also be done at run time)." Separate compilation and complete checking are both kept, because the compiler checks against the interface held in the library, never the implementation.

## 4. How it was used, as reported

The paper carries one worked program through all four mechanisms. It is called count_words. The task is to read a document, count how often each word occurs, and print the words in alphabetical order with their frequency as a percentage. The paper develops it from the top. At the first level count_words needs two things it does not yet have: a procedure next_word that scans the document and returns the next word, and a type wordbag whose operations are create, insert and print. Those are the procedural and data abstractions. The program is written against them before either exists, which is the point.

The second level implements wordbag as a cluster. The paper chooses a binary search tree of words, each carrying a count, together with a running total: "Thus the main part of the wordbag representation consists of a binary tree." The tree itself is another data abstraction, wordtree, with its own cluster. So the example shows nesting: a cluster whose rep is built from another cluster, each hiding its own representation from the one above.

The third level, in section 4 of the paper, replaces the print operation with an iterator, so that the caller decides what to do with each word and its count, and the bag decides only the order of visiting. The example is then generalised: wordbag becomes sorted_bag[t], parameterised by the item type, with the where clause requiring lt and equal. The word counter instantiates sorted_bag[string]. The paper reports that "the compiler will check each use of sorted_bag to ensure that the item type provides the required operations."

The library section describes how such a program is built piece by piece. "For each abstraction, there is a description unit which contains all system-maintained information about that abstraction." The description unit holds the interface: the number and types of parameters, the operations and their signatures, the where clause requirements. "An abstraction is entered in the library by submitting the interface specification; no implementations are required." A module that uses the abstraction is compiled and checked against that interface. Implementations come later, and there may be several.

The implementation section reports that a CLU compiler exists. Objects are represented as fixed-size values that carry a type code, with mutable state allocated separately. "Procedure invocation is relatively efficient. A single program stack is used, and argument passing is as efficient as assignment." Iterators are implemented as coroutines constrained to that stack, and the paper says using one is only slightly more expensive than calling a procedure. No timing, code size or benchmark number appears in the paper, and none is stated here.

## 5. The one idea, restated

Strip the syntax away and the paper says one thing. A type is the set of operations on it, and nothing else. The representation is real, it occupies storage, and it is invisible. The only way to learn anything about an object is to apply one of its operations. The only way to make an object is to call one. The language, and not the programmer's good manners, enforces this, by checking types at compile time and by letting cvt appear only inside the cluster that owns the rep.

Each of the four mechanisms is this idea seen from a different side. The procedure hides how a computation is done and shows only what goes in and what comes out. The cluster hides how a value is stored and shows only what can be done to it. The iterator hides how a collection is walked and shows only the members, one at a time. The where clause hides which type is meant and shows only which operations it must have. In every case the interface is a list of operations and the implementation is a secret.

The paper's own sentence for this is that "an abstraction isolates use from implementation." Its consequence is stated in the discussion: "Each module has a well-defined purpose—to implement an abstraction—and no other module can interfere with that purpose." A module may be rewritten and nothing that uses it need change, so long as the operations behave as they did.

There is a second idea folded in, and it matters for what follows. The where clause is a specification by operations too. Sorted_bag does not ask that t be a number or a string. It asks that t have lt and equal. Any type that can answer "is a less than b" and "is a equal to b" is a fit item, and the compiler will hold the type to it. The type parameter is defined by the comparisons it supports. This is Liskov's chess thesis again, nine years on and in a different domain: the position was never scored, it was compared, by better and worse; the item type is never named, it is required, by lt and equal.

Phient's paradigm makes the same refusal for words. A word is not a point. It is its relations, and a relation is what two words do under a condition. Read this paper with "word" for "type" and "relation" for "operation" and it does not change. That reading is the work of the next five sections.

## 6. The word as a cluster

In Phient a word is a function: 32 to 64 relations, each a complex phasor on a closed circle with a count, indexed by channel. That is a representation. The paradigm sentence says the word "is nothing but what it relates to, under what conditions." The phasor table is the rep. The relations are the operations. Which is to say a Phient word is a CLU cluster, and the paradigm sentence is the paper's rule that behaviour is "completely characterized by the set of operations", with the set now being relations under conditions.

The paper's cvt is the point where the abstract word meets its rep. Phient has that point, and has measured it. In the helix the lexicon is stored as a DNA-coded tree, 64 codons as base-4 ACGT of the channel, with the sphere degree and order and a PhiTon wavelength, band and hex per codon. The facts file records that 18,496 words go through this in 1.9 seconds, 75 MB, and the round trip is byte-exact. Byte-exact is the property cvt promises: down then up returns the same object, and nothing that went through the abstract side can tell that a rep exists. The rep in the byte deadband is one byte per channel, 256 quanta of 2π. That is the representing type in the paper's sense, one per abstraction, named in one place.

The cluster's other rule is that only the operations touch the rep. Phient's quest ingest has an entry that reads as a violation of that rule and a measurement of what the violation costs. The offset reading, which reads the rep's phase offsets directly rather than applying the relation on the kind's channel, scores 0.005 MRR on the synthetic oracle, against 0.021 chance and 0.594 for the attraction-only trainer reading through the channel. A read that bypasses the operations returns nothing. That is what the paper predicts when the representation is exposed: whatever the reader does with it is undefined by the abstraction, and here it is worse than chance.

The paper's semantics also separates object from variable. Assignment shares; it never copies. Phient's tables are shared objects in the same sense. A word in phinet's Tensor [b,w,s] is one object under many contexts; a context is a variable that names it. Nothing in the paradigm makes a fresh copy of a word per use, and the byte ceiling in docs/83, where nearest rounding froze every pair 8 quanta short, is a fact about the one shared rep.

What the cluster does not give the word is a specification. Section 15 returns to this.

## 7. The where clause as the condition c

The relation channel is Phient's coordinate. A word's 64 channels are 64 places on which a relation may be read, and a condition c selects the channel. In the quest, the kinds is-a, part-of, near, used-for and unlike are conditions, and each kind reads its own channel. The paper's construct for this shape is the where clause.

A parameterised cluster fixes the operations and leaves the type open. The where clause states what the open type must provide. Sorted_bag[t] requires that t have lt and equal, with the signature proctype (t, t) returns (bool). In Phient's terms the pair (t, t) is a pair of words. The proctype is a relation between them. The boolean is the answer under the condition the operation names. Lt is one condition; equal is another. The channel is the where clause: a word may be read on channel k if it carries a relation of kind k, and the kind is the condition.

The paper's compiler checks every instantiation against the clause. Phient's quest scores what happens when the check is skipped. The facts file records within-kind MRR of 0.747 and wrong-channel MRR of 0.015 on the synthetic oracle, where chance is 0.021. Reading a word on the channel its kind names works. Reading it on another kind's channel returns less than chance. A CLU compiler would refuse sorted_bag[t] where t lacks lt. Phinet has no compiler for kinds, so it reads the wrong channel and gets nothing. The number 0.015 is the runtime cost of a missing where clause.

The paper says type checking "could also be done at run time" and chooses compile time. Phient does neither: a wrong-channel read is a silent bad answer. The held-out quest numbers show the same thing from the other direction. One hop on held-out words scores 0.011, below chance; two hops score 0.095 on is-a and 0.093 on used-for, and near chance on part-of, near and unlike. The kinds where two hops work are the ones whose channel composes with itself. In the paper's terms those are the type parameters whose where clause is satisfied by the result of their own operations, and the others are not.

The three operators sharpen this. Liskov's chess program required of a position only that it could be compared, by better and worse. Her CLU bag requires of an item only that it be compared, by lt and equal. Phinet::liskov requires of a word only better, worse and loss. A word that supports those three on channel k is an instance of the kind. That is the where clause Phient has not yet written. Section 12 makes it the experiment.

## 8. The operator as X ⇒ Y under c

The paper's first abstraction is the procedure, and it is the one it spends least time on because, as the abstract says, conventional languages already have it. A procedure performs a computation on input objects and produces output objects. Its interface is its signature. Its body is hidden. Phient's operator is a procedure in this sense: X ⇒ Y, a map from a word or a context to a prediction, with a config `FnoConfig { space, width, layers, modes, margin, lr }` as its signature and nothing else visible.

What the paper adds is that the three abstractions are not independent. A cluster's operations are procedures. An iterator is an operation of a cluster. A where clause requires procedures of a type parameter. Phient's operator has the same shape. The condition c selects the channel, which is the where clause. The word it reads is a cluster. The decoder that calls it repeatedly is an iterator. There is one map and three ways to hold it.

The three operators in phinet::liskov make this explicit. `better(pred, a, b)` is the procedure that compares two candidate words under a prediction: cos(pred, a) − cos(pred, b). `worse(pred, a, after, continuations)` compares a candidate with the best continuation it leads to. `loss(pred, target, decoy, margin)` is the hinge over better. These are proctypes in the paper's sense: each takes words, returns a real, and says nothing about how the cosines are computed or on what tie.

The tie is where the paper's isolation of use from implementation has been tested inside phinet without anyone planning the test. The circle tie, the sphere tie (phinet::Tie::Sphere, one complex weight per spherical-harmonic degree, 64 = 8² channels as Y_l^m with l < 8, 1,192 params) and the anisotropic tie (Tie::Anisotropic, one weight per (l, |m|), 36 slots, 4,776 params) are three implementations of one operator interface. Nothing above the operator changed when they were swapped. The results did: on Austen hinge at 3 epochs, circle 14.7× valid and 14.6× test at 8,360 params in 142 seconds; sphere 12.8× and 13.1× at 1,192 params in 89 seconds; anisotropic 16.0× and 15.6× at 4,776 params in 85 seconds. That is the paper's promise, that an implementation may be replaced without the user's code changing, and its price, that the user cannot see why the answers moved.

Phinet's parity suite, 62 of 62 tests against the PyTorch reference, is a check of the operator against another implementation of the same interface. It is a second implementation of the same description unit, used as an oracle. It is not a specification.

## 9. The iterator as decoding and the quest

The iterator is the paper's control abstraction, and the paper's reason for it is that "the purpose of many loops is to perform an action on some or all of the objects in a collection." The collection should say how its members are visited. The loop should say what is done with each. The iterator yields one member, the loop body runs, and the iterator resumes "in the same environment as when the objects were yielded."

Phient has two loops of this shape. The first is decoding. The nearest-k decoder yields one word at a time: apply the operator to the context, take the nearest words, yield one, append it, resume. The context is the environment the iterator resumes in. On 40 held-out prompts × 30 tokens: the trigram sampler at KN perplexity 13.3, the operator nearest-k at 66.9, the union at 66.8. Shared 4-grams across prompts were 25.7% for the sampler and 8.7% for the operator. The operator repeats itself less across prompts and fits the held-out text worse. Both are properties of the iterator, since the loop body is the same.

Forcing is a change to the iterator's yield. At k = 32, forcing gave 69.7 at λ = 0.05, 68.5 at λ = 0.25 and 107.4 at λ = 1, with the evaluator at 0.712 and shared 4-grams 22.3% at λ = 1. Forcing does not beat greedy on text at any λ tried. Docs/86 gives the reason: on text every word has some continuation, so the refutation term has nothing to refute and drifts toward function words. A text collection contains no dead ends.

The second loop is the quest. quest_ingest walks 1,280 rows over 256 words under five kinds and, in docs/86, scores each about word by better, worse and loss and files it by hexagram. Here the collection does have dead ends: a word either has onward relations of a kind or it does not. What the walk found is flat: 256 words occupy 61 of 64 hexagrams, mean better −0.008, mean worse −0.008, mean loss 0.108 with the margin at 0.1. The truth ties the best wrong word on nearly every row. That is the byte ceiling, seen through the third operator.

The paper's iterator hides the representation of the collection. Phient's quest iterator does not, yet. The dither that lifted the quest from 0.082 to 0.594 MRR is a change to the rep's rounding, and it costs attribution 71% to 65%. A CLU iterator would not know about rounding. That the quest's iterator has to is a sign the cluster boundary is drawn in the wrong place; section 13 says so.

## 10. The library as the helix and the lexicon

The paper's last mechanism is not in the language. It is the library, and the paper gives it a section because without it the others do not scale. Every abstraction has a description unit that holds "all system-maintained information about that abstraction": the interface, the where clause, the implementations. An abstraction is entered by its interface alone, "no implementations are required", and modules that use it are checked against the interface. The library is what makes separate compilation and complete type checking compatible.

Phient's lexicon is this library, and the helix is its description unit. It files each word as a DNA-coded tree: 64 codons, each a base-4 ACGT coding of the channel, with the hexagram, the sphere degree and order, the B-DNA xyz position at 10.5 codons per turn and 0.34 nm rise, and a PhiTon wavelength, band and hex. The facts file records the size: 18,496 words in 1.9 seconds, 75 MB, byte-exact. That is a description unit per word, holding the channels it carries and the codes by which each channel may be read. The interface is the set of channels; the implementation is the phasor table behind them.

Phient's family is arranged the same way. Phinet is the parent engine and the interface. Phifno 1.0.0 implements the spherical operator. Phical implements the geometry of the relation circle. PhiTon implements light as units. Phiano applies words: Facet, Trainer, generation, personas, quest ingest, helix. Phiecs holds the entity-component world and the quest. In CLU terms phinet is the description unit and each child is an implementation entered under it. Phiqlm is entered with no implementation, which the paper permits.

The paper's compile-time check is the part Phient lacks. Nothing in phinet refuses a wrong-channel read. Nothing refuses a word without the operations a kind needs. The hexagram filing in docs/86 is the closest thing: each word is filed under the hexagram its 64-channel function is, by centroid phase in 64 sectors, and the report is per hexagram. That is a classification of words by what they carry, which is what a library index is. It is not a check; a check would have refused rows the flat table cannot support.

One more of the paper's properties holds. Incremental development: a module may be compiled against an interface before the implementation exists. Phient's sphere work was done this way. The synthetic degree-filter task, 200 items over 40 epochs, was written against the operator interface and run on circle and sphere: circle MRR 0.662 with identity at 0.646, sphere 1.000. The task did not change between runs. The implementation did. That is the library working as the paper says it should.

## 11. What Phient has measured that bears on it

Every number here is from the facts file. Nothing else is claimed.

| paper's mechanism | Phient measurement | number |
|---|---|---|
| cluster, one rep behind an interface | helix round trip | 18,496 words, 1.9 s, 75 MB, byte-exact |
| cluster, rep read directly | quest offset reading | 0.005 MRR (chance 0.021) |
| where clause satisfied | quest within-kind | 0.747 MRR |
| where clause violated | quest wrong channel | 0.015 MRR |
| implementation swapped behind one interface | Austen hinge 3 epochs | circle 14.7×/14.6×, 8,360 params, 142 s; sphere 12.8×/13.1×, 1,192 params, 89 s; anisotropic 16.0×/15.6×, 4,776 params, 85 s |
| second implementation as oracle | phinet vs PyTorch | 62/62 parity; sphere vs scipy sph_harm_y max difference 2.4e-15 |
| iterator on text | decode_eval, 40 prompts × 30 tokens | sampler KN 13.3; operator nearest-k 66.9; union 66.8; shared 4-grams 25.7% vs 8.7% |
| iterator with refutation | forcing k=32 | λ=0.05 69.7; λ=0.25 68.5; λ=1 107.4 (evaluator 0.712, distinct 0.777, shared 22.3%) |
| iterator on the quest | quest scored by three operators | 61 of 64 hexagrams; mean better −0.008, worse −0.008, loss 0.108 (margin 0.1) |
| rep-level change leaking upward | dither | quest 0.082 → 0.594 MRR; attribution 71% → 65% |

The first two rows are the cluster rule and its breach. The next two are the where clause and its breach. These four numbers are the paper's argument for enforced encapsulation, measured in a system that does not enforce it.

The fifth row is the paper's promise that implementations may be swapped, kept. Three ties ran under one interface and the harness did not change. The sixth is two implementations agreeing. That is agreement, not proof, and the paper would say so.

The iterator rows say what the paper cannot: how a control abstraction behaves when the collection has no dead ends. Forcing does not beat greedy on text. The quest, where dead ends exist, is flat at the byte ceiling. The last row is the leak. Dither is a change to how the rep is rounded and it moved the quest by a factor of seven and the persona attribution by six points. In CLU that change would be invisible outside the cluster. In Phient it is the most visible change in the file.

Not measured: any quest on a real corpus (no Phi-4 run yet, synthetic oracle only); sphere at softmax wide+deep; forcing at depth two on the quest; any typed-channel check. Anything about those is a proposal, and section 12 makes one.

## 12. The experiment: a where clause for the quest

The paper's claim, restated for Phient, is that a word should be readable on a channel only if it carries the operations the channel requires, and that the check should happen before the read. The experiment tests whether that check improves the quest, and whether it costs anything on text.

The proposal is a parameterised relation, in the paper's syntax, `relation[k] where word has better, worse, loss on k`. For each of the five kinds in the phiecs synthetic oracle, a word is admitted to the kind's channel only if its scores under the three operators on that channel pass a margin. Words that fail are not read on that channel; their rows are refused, as the CLU compiler would refuse an instantiation whose type lacks lt. The quest is then scored exactly as docs/86 scores it, and filed by hexagram.

The control is the shipped quest as it stands, with every channel open to every word. Its numbers are on file: attraction-only with dithered rounding 0.594 MRR, within-kind 0.747, wrong channel 0.015; held-out 20% one hop 0.011, two hops is-a 0.095 and used-for 0.093; and 61 of 64 hexagrams, mean better −0.008, worse −0.008, loss 0.108.

Three outcomes are distinguishable. If the typed read lifts the within-kind and held-out numbers and the hexagram table stops being flat, the paper's claim holds for words: a check before the read removes the rows that were tying the truth with the best wrong word. If the typed read refuses most rows and the survivors score as before, the byte ceiling is the whole story and no check can help until the rep changes. If the typed read scores worse, the where clause is drawn on the wrong coordinate and the kinds are not channels, which would be the most useful negative on file.

Two further arms are required for honesty. The first runs the same typed read with dither on and off, since dither is the rep-level change that moved the quest most, and the experiment must show whether a check at the interface does the same work or different work. The second runs the typed decoder on text, k = 32, the 40 held-out prompts, with the shipped nearest-k at 66.9 as control, to see whether it does what forcing at λ = 1 did and pulls toward function words.

The measurement that decides the experiment is the wrong-channel MRR. It is 0.015 in the control. In the typed arm it must be undefined, because no wrong-channel read is permitted. If the code still produces a number there, the where clause is not enforced and the experiment has not been run.

## 13. Where we disagree, or the analogy breaks

The paper's discipline is that the representation is hidden and that nothing above the cluster can tell what it is. Phient's best result in the facts file violates that. Dither, a change to how a phasor's byte is rounded, lifted the quest from 0.082 to 0.594 MRR and lowered persona attribution from 71% to 65%. In CLU that change would be inside wordbag's cluster and count_words would not know. In Phient the whole system knows. A channel is one byte, 256 quanta of 2π, and the nudge 0.06·sin d falls under half a quantum once sin d < 0.2. The representation is the ceiling. The paper assumes the rep can be made good enough that its grain does not show. Phient has not reached that.

The second break is in the iterator. The paper's iterator yields members and the loop cannot tell how they were found. Phient's decoder yields words at perplexity 66.9 against a trigram sampler at 13.3, with the same loop body. The difference is the iterator. The control abstraction is where the quality lives, and hiding it does not make it neutral. Forcing at λ = 1 raised the evaluator's score to 0.712 while raising perplexity to 107.4. Two loop bodies disagree about the same iterator. In CLU an iterator's contract is an order, and the paper has no account of a quality.

The third is the where clause. In the paper it is a static condition on a type parameter. Phient's condition c is a coordinate on which a relation is read, and the same word may satisfy it on one channel and fail on another. The held-out quest makes this concrete: two hops compose on is-a and used-for at 0.095 and 0.093, and not on part-of, near or unlike. A CLU where clause is either satisfied or not. Phient's is satisfied to a degree, per channel, and the degree is what training moves. The paper's boolean has become a real. There is a real disagreement here about whether "has the operation" can be graded.

The fourth is what the paper omits. The authors defer specifications and say the compiler performs "a small part of the actual proof." Phient has less than the compiler's small part: no check refuses a wrong-channel read. The claim of "complete type checking" is true of CLU and not of phinet.

Last, CLU has no operation that changes a type's operations. Phient's training changes which relations a word carries, which in the paper's terms is a cluster whose interface changes at run time. The paper has no such thing and would call it an error.

## 14. The person and the moment

The facts here are from Liskov's own history of CLU, MIT LCS Technical Report 561, and from the paper itself.

The CLU design began, in her words, "in the fall of 1973" at MIT. The name was chosen the same autumn: "The name was selected because it is the first three letters of 'cluster.'" The core group was Liskov, Russ Atkinson, Craig Schaffert and Alan Snyder, who were, in her words, "implementers as well as designers." Steve Zilles was deeply involved in the early work; by 1974 he was concentrating on specifications. Liskov and Zilles had presented their approach to abstraction at the ACM SIGPLAN Conference on Very High Level Languages in April 1974.

The iterator has a date and a place. Liskov visited the Alphard group at Carnegie Mellon in the summer of 1975 and saw their generator construct. Her history records: "Russ Atkinson designed iterators on the airplane going back to Boston after this meeting and described them in a design note in September 1975." The 1977 paper names Alphard as "the language closest to CLU, which represents a concurrent design effort with goals similar to our own." 

The compiler was first written for the PDP-10. In 1977 the group reimplemented it to generate macro-assembler rather than MDL, and later produced machine code for the DECsystem-20, the VAX and the 68000. Liskov's history counts "fourteen person-years" from the start of design in 1973 to the production compiler in 1980.

The paper was presented in Raleigh in March 1977 and printed in August. Its acknowledgments credit the Advanced Research Projects Agency, through Office of Naval Research contract N00014-75-C-0661, and the National Science Foundation, grant DCR74-21892. The four authors were at 545 Technology Square, the Laboratory for Computer Science.

On inheritance, which CLU lacks, her history is direct: "we felt that Simula's inheritance mechanism was a distraction from what we were trying to do." What they were trying to do is in one sentence of the same document: "We believe the best approach to developing a methodology that will serve as a practical tool for program construction is through the design of a programming language such that problem solutions developed using the methodology are programs in the language." The distinction she kept throughout, and which this paper enforces, she states as "An abstract data type is a concept whose meaning is captured in a set of specifications, while a cluster provides an implementation of a data type."

Liskov was born Barbara Huberman, took her Stanford PhD in 1968 under John McCarthy, worked at MITRE, and joined MIT. She received the Turing Award for 2008.

## 15. What remains

For the paper, what remained in 1977 was stated by its authors: specifications in a separate language, formal verification on top of them, and an exception mechanism described elsewhere. CLU acquired the exceptions. The specification language stayed a research programme.

For Phient, the paper leaves five things undone, and the facts file says which are not started.

The where clause is the first. Section 12 proposes `relation[k] where word has better, worse, loss on k` and a quest run in which wrong-channel reads are refused rather than scored. The control numbers exist. The typed arm does not. Until it runs, Phient's channels are conditions without a check.

The second is the rep. The paper's discipline works only when the representation's grain does not show through the interface. Phient's does: one byte per channel, and a nudge that falls under half a quantum at 11 degrees. Dither moves the quest by a factor of seven and costs attribution six points. The cluster boundary is drawn around a rep too coarse to hide, and a change to it is one the paper would call private and Phient must call public until it is done.

The third is the corpus. Every quest number is from the phiecs synthetic oracle. There has been no Phi-4 run. The hexagram table in docs/86 was built for a corpus that is not flat, and the synthetic oracle is flat. The library index exists and has nothing yet to index.

The fourth is forcing at depth two on the quest. The paper's iterator resumes in the environment it left. Forcing on text at depth one did not beat greedy; docs/86 says the place worse should bite is the quest, where a word either has onward relations or does not. That run is named in docs/86 as remaining and it still is.

The fifth is the specification the paper deferred. Phinet's 62 of 62 parity tests and the sphere's 2.4e-15 agreement with scipy are two implementations agreeing. The paper is precise that this is the compiler's small part of a proof and that the rest needs a specification language. Phient has no statement, outside the code, of what a relation on channel k under kind k must satisfy. Writing one would be the first specification in the family, and the three operators are the vocabulary: a word is an instance of a kind when better, worse and loss on the kind's channel meet a stated margin. That sentence is a where clause. The paper says a where clause is enough for the compiler to check. Phient should find out whether it is enough for the quest.

## 16. Sources

Sources reached and read.

| source | what it gave |
|---|---|
| Liskov, Snyder, Atkinson, Schaffert, "Abstraction Mechanisms in CLU", Communications of the ACM 20(8), August 1977, pp. 564–576, doi 10.1145/359763.359789; PDF mirror at web.eecs.umich.edu/~weimerw/2008-615/reading/liskov-clu-abstraction.pdf | full text: section headings, definitions, the count_words and wordbag example, the sorted_bag where clause, rep and cvt, the library's description units, the implementation section, related work, acknowledgments, the Raleigh conference note, key words |
| cacm.acm.org, "Abstraction mechanisms in CLU" (research article page) and cacmb4.acm.org/magazines/1977/8/11316 | the abstract, verbatim |
| Liskov, "A History of CLU", MIT LCS TR-561, publications.csail.mit.edu/lcs/pubs/pdf/MIT-LCS-TR-561.pdf | start of the project in fall 1973, the name, the group and their roles, the airplane origin of iterators in September 1975, the PDP-10 compiler and the 1977 reimplementation, fourteen person-years to 1980, the inheritance decision, the sentences on methodology and on abstract type versus cluster |
| rebelsky.cs.grinnell.edu, CS302 reading notes on the paper | a few quoted sentences, used only where the PDF confirmed them |
| en.wikipedia.org, CLU (programming language) | the page numbers 564–576 and the 1981 date of the reference manual; used for nothing else |

Sources tried and not reached.

The ACM Digital Library page for the paper, dl.acm.org/doi/10.1145/359763.359789, and the MIT PMG copy of the CLU history, pmg.csail.mit.edu/papers/history-clu.pdf, could not be fetched from this session. The dblp record could not be fetched. The Semantic Scholar page returned no content. The University of Michigan 2011 mirror of the PDF failed on certificate verification; the 2008 mirror of the same file succeeded and is the one read. The SIGPLAN Notices and SIGOPS Operating Systems Review reprints of the conference version were not read; the CACM version is the one cited.

What was verified and what was not. Every quoted sentence in sections 2 to 5 and 14 was taken from the PDF or the CACM abstract page through a fetch that summarised the page; paraphrases are given without quotation marks. The paper contains no timing or size numbers and none are stated. The date of the Raleigh conference, the contract and grant numbers, and the authors' address are as printed in the paper. Liskov's biography beyond the CLU history is limited to the facts the brief supplies: Stanford PhD 1968 under McCarthy, MITRE, MIT, Turing Award for 2008. No citation count is given because none was read.

Phient sources: PHIENT_FACTS.md in this folder, cited exactly; docs/86 for the three operators and the hexagram table; docs/83 for the quest and the byte ceiling; docs/84 for the sphere and the helix; docs/85 for the PML structure. No number in this paper about Phient comes from anywhere else.
