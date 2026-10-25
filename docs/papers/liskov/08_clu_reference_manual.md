# The hidden rep: a word as a cluster, a channel as a type generator, decoding as an iterator

*Phient interpreted papers · Liskov series, 8 of 16 · 2026-04-20 · on: "CLU Reference Manual" (Barbara Liskov, Russell Atkinson, Toby Bloom, Eliot Moss, J. Craig Schaffert, Robert Scheifler, Alan Snyder; Lecture Notes in Computer Science 114, Springer, 1981; first issued as MIT LCS TR-225, October 1979)*

"In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition."

## 1. Our abstract

The CLU Reference Manual is the document that fixed a language. Seven authors, an MIT technical report in October 1979, then a Springer volume of 193 pages in 1981. It is a manual, so it makes few arguments. It defines. It defines what an object is and what a variable is, and keeps them apart. It defines three kinds of module: the procedure, the iterator, and the cluster. It defines how a type is made from another type by a type generator, how a module keeps state of its own between calls, how a call ends in one of several named ways. Everything CLU is known for is in it, stated as a rule rather than as a claim.

This paper reads the manual through Phient. The reading is direct. A CLU object lives in a universe of objects and a variable only names it. In Phical a word is a name and the thing named is 32–64 phasors on a circle. A CLU cluster hides its representation, and only the operations in its header can touch it. In phinet the relations of a word are the rep, and the only things that touch them are the operators better, worse and loss. A CLU type generator makes a type from a type, array[int] from array and int. In Phical a channel is a generator of that kind: the same relation circle, instantiated for a condition c. A CLU iterator yields one object at a time and the loop body runs between yields. Decoding in phiano is an iterator over the operator, thirty tokens for each of forty prompts. Own variables are the manual's name for state that belongs to the module and survives the call. The count on each relation is Phient's own variable.

What the manual gives Phient is a discipline for naming the parts. Phient has a rep, an interface, generators and an iterator, and had not said so. The paper also says where the analogy breaks. CLU's rep is hidden to keep it changeable; Phient's rep is the meaning, and it must be read. CLU's type checking is static; Phient's channels are bytes and their errors are measured, 256 quanta of 2π, a deadband at 11 degrees. The measured numbers in section 11 are all from the facts file. Section 12 proposes one experiment: hold the operator interface fixed and swap the rep, with the circle tie as the control. Section 16 lists what could and could not be reached, and the manual's own full text was not among the reachable.

## 2. What the work claims, in its terms

The manual's own description of itself is one sentence, given in the MIT DSpace record: "This document serves both as an introduction to CLU and as a language reference manual. Sections 1 through 4 present an overview of the language." The Springer table of contents shows the shape. Four short chapters first: Modules (pages 1 to 4), Data types (5 to 7), Semantics (8 to 12), The library (13 to 15). Then the reference proper: Notation, Lexical considerations, Types, type generation, and type specifications (19 to 30), Scopes, declarations, and equates, Assignment and invocation, Expressions, Statements, Exception handling and exits (60 to 67), and Modules again (68 to 90), the long chapter where procedures, iterators, clusters and their parameters are defined.

The manual claims little in the way a paper claims. It sets rules. Where it does state a position, the position is the one the group had argued in print since 1974. Liskov's later history of the language puts it plainly: "The primary goal of the project was to do research on programming methodology," and "We believed that our main 'product' was concepts rather than the language." The manual is the point where the concepts became rules that a compiler enforced.

The rules that matter here are these. Objects are the things a program creates and uses. Variables are names. "CLU objects reside in an object universe (or heap), and a variable just identifies (or refers to) an object." Some objects never change state; integers and strings are "immutable." Others, records and arrays, "can have a succession of states." Assignment does not copy: "CLU assignment causes sharing: after executing 'x := y,' variables x and y both refer to the same object." A procedure's formals are local variables initialised by that same assignment, which the 1977 CACM paper called call by sharing.

A data abstraction is implemented by a cluster: a header listing operations, a definition of the rep, and the procedures and iterators that implement the operations. "Only procedures inside the cluster can access the representations of objects of the type. This restriction is enforced by type checking." An array is not a type. "Instead it is a 'type generator' that, given a type as a parameter, produces a type." User clusters may take type parameters too, constrained by a where clause that "describes the names and signatures of any operations the parameter type must have." An iterator "is a single operation that yields its results incrementally." A call ends normally or in one of several named exceptions, and an unhandled one becomes "failure." Late in 1979 the group added two things users had asked for, "the 'resignal' statement" and "'own' data."

## 3. The mechanism, plainly

Take the parts in the order the manual takes them.

An object has a type, an identity and, if mutable, a state. It lives in the heap and is garbage collected; Eliot Moss wrote the collector. A variable is a cell holding a reference. Assignment writes a reference into a cell. Two cells can hold the same reference, and then a mutation through one is seen through the other. There is no pointer syntax. Liskov later credited the absence of explicit pointers for much of the language's safety: "Pointer manipulation is clearly both a nuisance and a source of errors in other languages."

A procedure takes objects and returns objects. Its arguments arrive by sharing. It ends by return or by signal. A signal names an exception and may carry objects. The caller handles it in an except clause or it propagates as failure. This is the termination model: the call is over when it signals. There is no resumption.

An iterator is a procedure that yields instead of returning. The for statement invokes it. "Each time the iterator yields a result, the loop body is executed; when the body finishes, control resumes in the iterator, and when the iterator returns, the loop terminates." The body sees one object at a time and never sees the collection. The iterator's local state survives across yields. Liskov's history says Russ Atkinson designed iterators on a flight home after seeing Alphard's generators.

A cluster names a type and lists its operations. Inside, a rep equate names the representation type. The operations are ordinary procedures and iterators, except that they may convert between the abstract type and the rep. The CACM paper gives the mechanism: cvt in a signature means the argument arrives as rep or the result leaves as the abstract type, and "this change of viewpoint applies only within clusters." Outside the cluster the rep does not exist. The type checker enforces this at compile time, and the group "ruled out coercions."

A parameterised cluster or procedure takes types as parameters. array[t] is one; a user's sorted_bag[t] is another. The where clause lists the operations t must have. "When the type is instantiated, the compiler checks that the actual type parameter has the operations (and signatures) listed in the where clause," and inside the body "the only operations of a type parameter that can be used are those listed."

Own variables are the state a module keeps between invocations. The history gives only the fact of their addition. The manual's Modules chapter, which I could not read, holds the definition. The plain sense of the term is used here, and section 16 marks it as unverified.

## 4. What it demonstrated, as reported

A reference manual reports nothing. What was demonstrated is in the history that Liskov wrote in 1992 and in the record of what CLU was used for.

The timeline is short and dense. The name was chosen in the fall of 1973. Design notes ran from December 6, 1973 to July 30, 1979, seventy-eight of them. A compiler was started in the summer of 1974 for the PDP-10, first generating MDL, then in 1977 reimplemented to generate macro-assembler for the DEC System 20. "All the features of CLU were designed and integrated into the language by the end of 1976." The manual came out in July 1978 and the final version in October 1979. "By 1980 we had a high quality compiler that could be exported." It was later retargeted to VAX and to the 68000, and later still changed to generate C.

The people are known. Four main designers: Liskov and the graduate students Atkinson, Schaffert and Snyder. Steve Zilles was in the early work and then concentrated on specification. Scheifler and Moss joined later. On the implementation, Atkinson did the runtime and debugger, Moss the garbage collector, Scheifler the compiler front end, Snyder the parser and Schaffert the type checker of the preliminary version. Toby Bloom is on the manual as an author. The history does not assign her a component and I do not assign one.

The uses the history lists: a text editor called TED, "still in use today" in 1992; ETUDE, a WYSIWYG editor; a browser for database conceptual schemas; a circuit design system; a gate-array layout system; and the LP theorem prover. Teaching was "probably its main use today both at M.I.T. and elsewhere," in the software engineering course and the compiler course.

Two evaluative statements from the history stand as the group's own verdict. On the safety of the object model: the belief that CLU programs were reliable "in large part because we do not manipulate pointers explicitly." On what was left out: "CLU lacks an inheritance mechanism," a lack Liskov defended because subclass implementations that depend on a superclass rep must be rewritten when it changes, and "this use of inheritance is not desirable in production programs or in programs developed by many people."

What was not demonstrated belongs in the same tone. The manual carries no benchmarks. The history gives no counts of users or programs. The CACM paper says the group intended "eventually to add formal specifications to the CLU system," and the manual's library chapter is where that was to go. Whether it went there is not something I can report from the sources reached.

## 5. The one idea restated

A thing is its operations. Its representation is nobody's business but its own.

That is the cluster, and every other construct in the manual serves it. Objects and variables are separated so that identity is a fact about the object and naming is a fact about the program; the same object under two names is one thing, and the cluster's operations act on the thing. Sharing is the default because copying would make the rep visible: a copy has to know what it is copying. The type generator exists so that the rule scales: array is not a type with a rep, it is a rule for making types, each with the rep the rule gives it, and the rep of array[int] is as hidden as the rep of any user cluster. The where clause exists so that a parameterised module can be checked without seeing the parameter's rep; it sees a list of operation names and signatures and nothing else. The iterator exists so that a loop over a collection does not have to know how the collection is stored. The body gets one element at a time and the collection's rep stays inside the cluster that yields it. Exceptions exist so that an operation can fail without the caller reading the rep to find out why; the failure has a name and carries what the operation chose to give. Own variables exist so that an operation can keep state that is not the state of any one object, and keep it hidden in the same way.

The manual also says what the idea costs. If the rep is hidden, every question about the object must be an operation. There is no reading a field. If a caller needs something the header does not give, the header must grow, and the cluster must implement it. This is slower to write than reaching in. The group accepted the cost for the reason the CACM paper gives: conventions do not hold, and "conventions are no substitute for enforced constraints." The manual is the constraint.

Restated for Phient: a word is its relations, and the phasors are the rep. The manual's discipline says the rest of the system should reach the phasors only through named operations. Phient has three: better, worse and loss. Whether three are enough is a question the manual would ask first, and section 15 lists it. Sections 6 through 10 take each construct in turn and say what it names in phinet: the cluster and the word, the generator and the channel, the procedure and the operator, own variables and the count, the iterator and decoding.

## 6. The word as a function, and the cluster

In Phical a word is 32–64 relations, each a phasor with a count, and the word means nothing except through those relations under a condition. The manual has a name for this shape. It is a cluster.

A cluster's header lists what can be done with the type. Its rep is the storage. Its operations are the only path from one to the other. For a Phient word the header is short: better(pred, a, b) compares two candidates under a prediction; worse(pred, a, after, continuations) asks how far the best continuation after a falls below a's own fit; loss(pred, target, decoy, margin) is the hinge over better. Those are the operations. The rep is the 64 channels of bytes, 256 quanta of 2π each, and the count on each.

The manual insists on one thing that Phient had not said in its own documents. The rep is reached only from inside. In phinet the trainer reads phasors, the hexagram filer reads the centroid phase, the helix writer reads the byte of each channel and encodes it as a codon. Each of these is a procedure inside the cluster. None of them is the meaning. The meaning is what better, worse and loss return. That is the cluster reading: a word is a function from (condition, pair) to a comparison, and the phasors are how the function is stored.

The object and variable distinction lands cleanly. The word "Emma" in the pool of 1,228 Austen words seen 20 or more times is a variable: a name in a program. The object is the 64-channel function. Two spellings that name the same function share it, and a nudge on one channel is seen through both names. This is call by sharing, and it is how the operator trains: the 204,592 examples do not copy words, they pass references to the same objects and mutate their state through the tape.

Mutability lands too. The manual says integers and strings are immutable and records and arrays are not. A Phient word during training is a mutable record: each epoch is a new state. After the helix writes it, it is an immutable string of codons, 18,496 words in 1.9 s, byte-exact on the round trip. The same object in two phases of its life, one mutable and one not, is exactly the manual's division of types, and the round trip is the test that the immutable form is the same object.

The one place the cluster reading needs care is the count. A count is per relation, and it is state that survives every call. The manual's word for that is own. Section 9 takes it.

## 7. The relation channel as a coordinate, and the type generator

The manual says array is not a type. It is a type generator: give it int and it produces array[int], a type with its own rep and its own operations. Give it string and it produces another. The generator is one rule; the types are its instances.

A Phient channel has this form. The relation circle is one rule: a phasor on a closed circle, a byte of 256 quanta, a count. The condition c instantiates it. Under c = is-a the circle is one type; under c = part-of it is another; the quest oracle has five kinds, is-a, part-of, near, used-for and unlike, and each is the circle instantiated for that kind. The 64 channels of a word are 64 instances of one generator, and the coordinate of a channel is which instance it is.

This is more than a naming convenience, because the manual's rule for generators fixes what may cross between instances. Nothing. array[int] and array[string] share no objects. Their reps are separate and their operations are separate. In the quest measurements this is the wrong-channel reading: within-kind MRR 0.747, wrong channel 0.015, offset reading 0.005, with chance at 0.021. Reading a relation from a channel it was not written on gives less than chance. The generator reading says why: the instances are distinct types, and a value of one has no meaning as a value of another. The measurement is the compiler's type error, seen through a ranking.

The where clause is the second half. A parameterised module may use only the operations of its parameter that the where clause names. In Phient the trainer is a parameterised module. It is written once and runs over the circle tie, the sphere tie and the anisotropic tie. What it may use of the tie is the three operators. Its where clause, written in the manual's syntax, would read: where tie has better, worse, loss. The compiler's check at instantiation is the parity test: 62 of 62 against the PyTorch reference.

The sphere tie makes the generator reading concrete. phinet::Tie::Sphere gives one complex weight per spherical-harmonic degree, and 64 = 8² channels become Y_l^m with l < 8. The anisotropic tie gives one weight per (l, |m|), 36 slots. These are three reps for one abstract type. The trainer does not change. On Austen hinge at 3 epochs the circle gives 14.7× valid / 14.6× test at 8,360 params, the sphere 12.8×/13.1× at 1,192 params, the anisotropic 16.0×/15.6× at 4,776 params. The rep changed, the interface did not, and the numbers moved. That is the manual's promise kept and its warning in one run. Section 13 returns to it.

## 8. The operator as X ⇒ Y under c, and the procedure

The manual's procedure "performs a computation on a set of input objects and produces a set of output objects." It takes its arguments by sharing, keeps no state unless it owns some, and ends by return or by signal.

X ⇒ Y under c is a procedure in that sense. Its inputs are the word X and the condition c. Its output is a ranking over Y. It shares its arguments: the operator does not copy the word's channels, it reads them, and a nudge on the channel from training is visible to every later call. It ends one of two ways: a prediction, or nothing usable. In the quest held-out runs, one hop is 0.011 MRR and two-hop part-of, near and unlike are at chance. Those are the calls that end in failure, in the manual's sense: an exceptional termination with a name, and no value.

The termination model is the useful part of the reading. CLU does not resume after an exception. The call is over, the caller decides. Phient's decoder does the same thing at each token: the operator either returns a nearest word or it does not, and the choice is made and not revisited. Forcing, the arm that applies the operator once more to each candidate and re-ranks by cos − λ·worse, is the one place Phient reaches back inside a call before it ends. The manual would say that is two procedures, one of which calls the other, and it would be right. worse is a second operator invoked on the continuation, and its result is combined with the first before the first returns. That is a resignal: an exception caught and re-raised with more information. The manual added resignal late, at users' request. Phient added worse late, at Liskov's.

The exception names are the second thing the reading gives. In the manual an operation's signals are part of its header. A caller can see which failures are possible without seeing the rep. Phient's three operators have a natural signal each. better signals when the two candidates tie within the byte deadband, which happens once sin d < 0.2, about 11 degrees, where a nudge of 0.06·sin d falls under half a quantum. worse signals when no continuation is above the candidate's own fit, the dead-end case its unit test checks. loss signals when the margin is not met. Today all three return numbers. The quest scoring shows what a signal would have said sooner: mean better −0.008, mean worse −0.008, mean loss 0.108 at margin 0.1 is one signal raised on nearly every row, a tie within the deadband, and the hexagram table is flat because a table of exceptions is flat.

## 9. Own variables, and the count

The manual's own variables were added in the final design pass of 1979, at users' request, alongside resignal. The history says only that. Taking the term in its plain sense, an own variable belongs to the module and not to any one invocation. It is initialised once and keeps its value across calls. It is hidden with the rest of the rep. It is the state a cluster has that no object of the type has.

Phient has exactly one such thing and it is the count. Each relation is a phasor and a count. The phasor is the object's state, the byte on the circle. The count is not part of any word's meaning. It is how many times the relation has been seen, and it belongs to the training module. It is written by the trainer and read by the trainer. It survives across epochs, across corpora, across the helix round trip. It is an own variable of the operator cluster, and the manual gives Phient the name.

Two more candidates deserve a check against the definition. The persona in phiano is state that survives calls: three writers, Chesterton, Austen and Carroll, attributed at 71% overall (74/65/86) on held-out 4-sentence paragraphs against a chance of 33%. But the persona is an object with its own relations, passed as an argument. The dither flag is closer: a module-level setting that changes how every rounding call behaves, shipped off by default because it costs corpus attribution, 71% to 65%. But it is set once and never written by a call. The manual would make it an equate. So the count stands alone, and it is the right one, because the count is where the byte ceiling lives. Nearest rounding froze every pair 8 quanta short, and dithered rounding took the quest from a shipped 0.082 MRR to 0.594. The count is what decides whether a nudge is applied or lost, and a variable that decides that for every call is the definition of own.

The manual's caution about own state is implicit in why it was added last. A module with own variables is not a pure function of its arguments. Two calls with the same arguments can return different results. Phient accepts this because training is exactly that: the same pair under the same condition returns a different comparison after the count moves. But it means the operator's header is incomplete. better(pred, a, b) does not say that its answer depends on a count it does not receive. The manual would have it say so.

## 10. The iterator, and decoding

An iterator "is a single operation that yields its results incrementally." The body of the for statement runs between yields and never sees the collection. The iterator keeps its own local state across yields. Atkinson designed it after seeing Alphard's generators, on an airplane.

Decoding in phiano is an iterator. The collection is the set of continuations of a prompt, which does not exist as an object: it is produced one token at a time by applying the operator to the context. Each token is a yield. The body is whatever the caller does with the token, which in decode_eval is to append it and go on for 30 tokens. The iterator's local state is the context, which grows by one word at each yield and is never handed to the body whole.

The manual's separation is the useful thing. The body does not know how the iterator makes tokens. The nearest-k operator, the trigram sampler and the union are three iterators with the same header, and decode_eval is one body that runs against all three. That is why the arms are comparable: KN perplexity 13.3 for the trigram sampler, 66.9 for operator nearest-k (was 97.2), 66.8 for the union, on 40 held-out prompts × 30 tokens. Shared 4-grams across prompts, 25.7% for the sampler and 8.7% for the operator, are a property of the yields and not of the body.

Forcing is where the iterator reading is sharpest. Forcing at λ = 1 gives KN 107.4, distinct 0.777 and shared 4-grams 22.3%, while the evaluator's own quality rises to 0.712. The iterator, told to prefer a word with a future, yields the frequent function words, because in text every word has some continuation. Liskov's endgame is different. Most moves lead nowhere. worse was made for a collection where most yields are dead ends, and text is not such a collection. The manual's discipline gives the fix its shape: change the iterator, not the body. A quest iterator over the oracle's 256 words, where a word either has onward relations or does not, is a different collection with the same header, and docs/86 names it as the place worse should bite. It has not been run at depth two.

One more thing the manual gives. An iterator's yields can be consumed by a for over any type, so long as the types agree. The quest ingest, the helix writer and the decoder are three fors over the same operator. In the manual they would share a where clause. In Phient they share phinet and nothing forces them to share the header, and section 15 lists that as work.

## 11. What Phient has measured that bears on it

Everything here is from the facts file. Nothing else is claimed.

| construct in the manual | what Phient measured | number |
|---|---|---|
| where clause, checked at instantiation | phinet vs PyTorch reference | 62/62 parity tests |
| one interface, three reps (circle, sphere, anisotropic) | Austen hinge, 3 epochs, valid / test | 14.7×/14.6× at 8,360 params; 12.8×/13.1× at 1,192; 16.0×/15.6× at 4,776 |
| generator instances share no values | quest, within-kind vs wrong channel vs offset | 0.747 / 0.015 / 0.005 (chance 0.021) |
| iterator, three arms, one body | KN perplexity, 40 prompts × 30 tokens | sampler 13.3; nearest-k 66.9; union 66.8 |
| resignal (forcing, cos − λ·worse) | KN at λ = 0.05 / 0.25 / 1 | 69.7 / 68.5 / 107.4 |
| own variable (count) and rounding | quest MRR, shipped vs attraction + dither | 0.082 → 0.594; hit@1 34.8%, hit@5 97.4% |
| immutable form, round trip | helix, 18,496 words | 1.9 s, 75 MB, byte-exact |
| the same signal on every row | quest scored by three operators | better −0.008, worse −0.008, loss 0.108 (margin 0.1); 61 of 64 hexagrams |

A few of these deserve a sentence. The parity count is the closest thing Phient has to a compile-time check. It says the abstract operations agree with a reference on 62 cases. It says nothing about the rep. The three-rep row is the manual's central promise measured: the rep changed, the trainer did not, and the anisotropic rep at 4,776 params beat the circle at 8,360. The sphere itself agrees with scipy sph_harm_y to a max difference of 2.4e-15, which is the rep's own unit test and not a claim about the interface. The channel row is the type error made visible. The iterator rows show three iterators under one body, and the resignal row shows that the one-step lookahead costs perplexity at every λ tried. The count row is the own variable doing its work. The round-trip row is the immutable form.

What is not measured is longer. No Phi-4 run exists; the quest is a synthetic oracle only. The sphere at softmax and at wide+deep is unrun. Nothing has been measured about whether a rep swap preserves any particular ranking, only that the aggregate numbers move. No signal has been raised as a signal; every operator returns a number. The engine is allocation and bandwidth bound at these sizes, 1 MB op in 2–5 ms, so no timing here says anything about the cost of abstraction. Dialogue at 44% (sector reader) / 72% (Kneser-Ney panel) against 33% is a real number and does not bear on the manual.

## 12. The experiment

The manual's claim, reduced to one testable sentence: a program written against the header does not change when the rep changes. Phient can test that inside phinet, because it has three reps and one trainer.

The experiment is a rep swap with the interface held. Take the Austen setup as the facts file states it: Emma, Persuasion and Sense and Sensibility; 1,228 words seen 20 or more times; 204,592 training examples; hinge; 3 epochs. Fix the trainer's source. Run it against the circle, sphere and anisotropic ties, and record per run the aggregate MRR against chance, already known; the per-pair ranking on the held-out set, not known; and the pairs on which better ties within the byte deadband.

The control is the circle tie at 14.7× valid / 14.6× test, 8,360 params, 142 s. It is the shipped rep and the numbers are in the facts file.

The manual predicts two things and the second is the interesting one. First, the program compiles: the trainer runs unchanged against all three. Phient already knows this. Second, and this is what the where clause promises, the behaviour the caller can observe is the behaviour the header specifies. The header specifies better, worse and loss. So a caller that uses only those should see the same result from every rep that implements them correctly. Phient already knows this is false in aggregate: 14.7×, 12.8×, 16.0×. The experiment measures how false, per pair. If the three reps agree on the ordering of most held-out pairs and disagree on a small set, the disagreement is the rep leaking through the header, and the set is where. If they disagree broadly, the header is no specification, and the operators are three functions with shared names.

A second arm tests the own variable. Run the circle tie twice: once with the count retained across epochs as shipped, and once with the count reset at the start of each epoch. If the reset run gives a different MRR from 14.7×/14.6×, the count is doing what the manual says own variables do, and the size of the difference is how much of the operator lives outside its header.

A third arm is the one docs/86 asks for. Run forcing at depth two on the quest oracle: 256 words, 1,280 rows, chance MRR 0.021. The control is attraction-only with dithered rounding at 0.594. The iterator reading says worse should bite where most yields are dead ends, and the quest is that collection. If forcing at depth two does not beat 0.594, the reading is wrong.

None of these has been run. The first two are cheap by the facts file's timings, 85 to 142 s per Austen run. The third is unpriced.

## 13. Where we disagree or the analogy breaks

The manual hides the rep so that it can be changed. Phient's rep is the meaning, and it must be read. This is the break, and it is not small.

In CLU a caller who reads a cluster's rep is wrong. The type checker stops it. The point of the rule is that the rep is an implementation detail, and the abstraction is what the type is. In Phient the 64 channels of a word are not a detail. The helix writes them as codons because they are the word. The hexagram filer reads the centroid phase because the phase is where the word sits among 64 sectors. PhiTon maps each codon to a wavelength and band because the byte is a colour. Every one of these is a procedure that reads the rep from outside the operator cluster, and every one is the kind of thing the manual forbids. Phient does not think they are wrong. It thinks they are the point. A word's phasors are its relations, and relations are what a word is. The manual's discipline can name the parts, and it did in sections 6 to 10. It cannot forbid Phient from looking at them.

The second break is static checking. The manual's where clause is checked once, at instantiation, and thereafter the compiler guarantees the parameter has the operations. Phient's checks are measurements, taken after the fact: 62 of 62 parity tests, a wrong-channel reading of 0.015, a deadband at 11 degrees. Nothing in phinet stops a call from reading a channel under the wrong condition. The number says it was a mistake, later. The manual would call this runtime checking and say the group "preferred compile-time checking to runtime checking because it enabled better runtime performance and allowed us to find errors early." Phient has no compile-time and finds its errors late, in a table.

The third point is inheritance, and here Phient agrees with the manual. Liskov left inheritance out because subclasses that depend on a superclass rep must be rewritten when it changes. Phient has no hierarchy of words. "Emma" does not inherit from "woman." Under c = is-a there is a relation, on one channel, with a count, and that is all. The quest's two-hop is-a at 0.095 against chance 0.021 is a relation composed twice. The manual's reason for the omission is Phient's reason.

The fourth break is the one this reading kept meeting. The count is own, and the manual added own last, at users' request. An operator whose answer depends on state it does not receive is hard to specify. Phient's whole training loop is that. The header is incomplete, and Phient has not written the complete one.

## 14. The person and the moment

Barbara Liskov was born Barbara Huberman. She took her PhD at Stanford in 1968 under John McCarthy, with a thesis on chess endgames played by a forcing tree and two comparison functions, better and worse. She worked at MITRE and then joined MIT. The Turing Award came in 2008.

The manual's moment is 1979. The history she wrote in 1992 dates the project from 1972, when she and Steve Zilles began work on data abstraction; from the fall of 1973, when the name CLU was chosen and design began in earnest; and from December 6, 1973, the date of the first design note. The seventy-eighth and last note is dated July 30, 1979. The manual that this paper reads was issued three months later, in October 1979, as MIT LCS TR-225. The Springer volume followed in 1981 as Lecture Notes in Computer Science 114, VIII plus 193 pages. The history also cites a Springer printing dated 1984; I have not resolved whether that is a reprint or a second edition, and I do not claim either.

The seven authors are the four designers, Liskov, Atkinson, Schaffert and Snyder, plus Bloom, Moss and Scheifler. The history says Scheifler and Moss joined as the work went on, and assigns Moss the garbage collector and Scheifler the compiler front end. Atkinson has the runtime and debugger and, in the story the history tells, the iterator, designed on a flight back to Boston. Snyder and Schaffert built the parser and type checker of the preliminary compiler. The history does not say what Toby Bloom did and I do not.

What the moment was for the group is in one sentence from the history: "We thought of our primary output as being publications, and that success would be measured by our influence on programming methodology and practice and on future programming languages." The manual was the least publication-like thing they produced. It was the rules. The CACM paper of 1977, "Abstraction Mechanisms in CLU," had made the argument two years earlier and closed with the position that "conventions are no substitute for enforced constraints." The manual is the enforcement written down.

Two things about the moment are true and easy to get wrong. CLU had an exception mechanism from the start, and the 1977 paper deliberately left it out of its account. And the language was complete by the end of 1976; the two years to the final manual were spent on the compiler, on user requests such as resignal and own, and on writing it all down. Nothing in the sources reached says what Liskov herself thought of own variables, and this paper does not guess.

## 15. What remains

For Phient, five things, in the order they could be done.

Write the header. phinet::liskov has better, worse and loss, and forcing_choice. Nothing forces the trainer, the decoder, the quest ingest and the hexagram filer to reach the rep through them. The manual's first rule is that they should, and the first job is to list, for each module in phiano and phiecs, which reads the phasors directly and which does not. docs/82's inventory of the missing and unused is the place to record it.

Name the signals. Each operator has a natural exceptional termination, and section 8 named them: a tie within the deadband, a dead end, a missed margin. Today all three are numbers the caller must interpret. Making them named outcomes would let the quest report count signals rather than average small numbers, and the flat hexagram table would become a count of ties instead of a mean of −0.008.

Run the rep swap per pair. Section 12's first arm. The aggregate numbers are known; the per-pair agreement of circle, sphere and anisotropic on the held-out set is not, and it is the direct measure of whether the three operators are an abstraction of the rep or three functions with shared names.

Run the count reset. Section 12's second arm. It is the cheapest measurement of how much of the operator lives in its own variable.

Run forcing at depth two on the quest. docs/86 asked for it and section 10 explained why the manual's iterator reading predicts it should matter there and not on text.

A sixth is a question rather than a run. Are three operations enough? The manual's clusters list every operation a caller may need, and the list is usually longer than three. Phient reads phasors directly for the helix, the hexagram and PhiTon because no operator gives what those readers need. Either those are operations the header should list, with names, or they are reads of the rep that the manual would forbid and Phient allows. Section 13 said Phient allows them. Saying which they are, one by one, is the work.

For the reading itself, one thing remains. The manual's full text was not reached. The definitions of own, cvt, up and down, and the exact wording of the Modules chapter are taken from the 1977 paper and the 1992 history, and section 16 says so. A reading of the manual's own pages 68 to 90 would settle whether own variables in CLU are what section 9 assumed, and might change what section 9 says. It would also settle what the library chapter, pages 13 to 15, says about the formal specifications the CACM paper promised.

## 16. Sources

Reached and read, through WebFetch on 2026-04-20.

MIT DSpace record for MIT-LCS-TR-225, "CLU Reference Manual," authors Liskov, Atkinson, Bloom, Moss, Schaffert, Scheifler, Snyder, date issued October 1979, with the self-description quoted in section 2 (https://dspace.mit.edu/handle/1721.1/149509, persistent link https://hdl.handle.net/1721.1/149509). The record's PDF bitstream returned 405 to the fetcher and the CONNECT was refused to curl, so the manual's body was not read.

Springer Nature Link record for LNCS 114, "CLU: Reference Manual," 1981, ISBN 978-3-540-10836-8, VIII plus 193 pages, with the chapter list and page ranges used in section 2 (https://link.springer.com/10.1007/BFb0035014). Chapter pages for Semantics (10.1007/BFb0035017) and Modules (10.1007/BFb0035027) showed no preview text.

Barbara Liskov, "A History of CLU," MIT-LCS-TR-561, April 1992 (https://publications.csail.mit.edu/lcs/pubs/pdf/MIT-LCS-TR-561.pdf). Every quotation in sections 2, 3, 4 and 14 not attributed to the CACM paper is from this document, including the object universe, the sharing rule, the cluster's three parts, the iterator, the type generator, the where clause, the termination model and failure, the design-note dates, the team, the platforms and the uses.

Liskov, Snyder, Atkinson, Schaffert, "Abstraction Mechanisms in CLU," Communications of the ACM 20(8), August 1977, read from a course copy (https://web.eecs.umich.edu/~weimerw/2008-615/reading/liskov-clu-abstraction.pdf). Source for the three kinds of abstraction, the objects and variables sentences, cvt, the where clause example, the built-in types listed in section 3, and the closing sentences quoted in sections 4 and 14.

Wikipedia, "CLU (programming language)," used only to confirm the reference list and the signal/except vocabulary (https://en.wikipedia.org/wiki/CLU_(programming_language)).

Not reached. The ACM Digital Library guide-book records (10.5555/889832, 10.5555/538622) returned 403. DTIC's copy ADA077018 returned 403 and its citation page was not approved. The csail publications PDF of TR-225 (https://publications.csail.mit.edu/lcs/pubs/pdf/MIT-LCS-TR-225.pdf) returned no machine-readable text. The PMG ftp copy of the CLU syntax document was blocked by robots. dblp and Semantic Scholar returned nothing usable.

Consequences. The definition of own variables in section 9 rests on one sentence in the history and on the plain meaning of the word; the manual's Modules chapter was not read. The up and down operations were not verified in any reached source and are not described. The Springer 1984 date in the history's bibliography is reported and not resolved. Toby Bloom's role is not stated because no reached source states it. The manual's list of built-in types is given only as the CACM paper gives it, and the manual's own chapter on types, pages 19 to 30, may list more. Nothing in this paper about garbage collection goes beyond the history's one clause that Moss wrote the collector. Every quotation is attributed to the document it was read in.

All Phient numbers are from PHIENT_FACTS.md and docs/86_liskov_three_operators.md in this repository. No number about Phient was taken from anywhere else, and none was computed for this paper.
