# The only way to use the objects: what a word owes to an interface

*Phient interpreted papers · Liskov series, 16 of 16 · 2026-09-14 · on: "The Power of Abstraction" (Barbara Liskov, ACM A.M. Turing Award lecture for the 2008 award, delivered 2009; slides dated October 2009, given as the OOPSLA 2009 keynote and at SOSP 2009; the ACM citation of March 2009; the same title given at NSF, December 2009, and Princeton, March 2010)*

"In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition."

## 1. Our abstract

This is the last paper in the series and it reads the last work: not a paper, a lecture. Barbara Liskov received the ACM A.M. Turing Award for 2008. It was announced on March 10, 2009 and presented on June 27, 2009 in San Diego. The lecture she gave for it is titled "The Power of Abstraction." The version this paper reads is the slide deck dated October 2009, kept in the SOSP 2009 archive, fifty-eight slides. The same talk was the OOPSLA 2009 keynote, and she gave it again at NSF in December 2009 and at Princeton in March 2010 under a one-paragraph abstract that begins: "Abstraction is at the center of much work in Computer Science."

The lecture is a retrospective. It walks from the Venus machine and the Venus operating system through the programming methodology of the early 1970s, Dijkstra, Wirth and Parnas, to the 1972 partitions paper, the 1973 meeting in Savannah where she began working with Steve Zilles, the 1974 abstract data types paper, CLU, and then to type hierarchy and the substitution principle. It ends with what she thinks is missing and what she thinks is next. It cites its sources on the slides by author, title, venue and month, which is why this paper can be exact about what it claims.

The reading through Phient is the reading the whole series has been building. The 1974 definition on slide 19 is three lines: a set of operations, a set of objects, and "the operations provide the only way to use the objects." Phient's word is a set of relations, and its three operators, better, worse and loss, are the operations. The question the lecture puts to Phient is whether those three are the only way, and section 13 says plainly that today they are not. The where clause and the subtype slide, "proof happens at different times," read as the relation channel: a condition c under which a rule is instantiated. The Parnas sentence she chose to carry, "the connections between modules are the assumptions which the modules make about each other," reads as the operator X ⇒ Y under c, which is nothing but an assumption one word makes about another.

Section 11 gives the measured numbers, all from the facts file. Section 12 proposes the substitution test that phinet can run with three ties and one trainer, control named. Section 14 gives the person and the moment from verified sources. Section 16 says what was reached: the slides, the citations, the press releases, the event pages; and what was not: the ACM Digital Library record, the video page, and her own MIT copy of the slides.

## 2. What the work claims, in its terms

The lecture claims a history and a verdict. The history is that data abstraction was invented in a particular decade by particular people answering a particular question, and the verdict is that it won.

The outline on the first slide has four parts: "Inventing abstract data types," "CLU," "Type hierarchy," "What next." The prehistory is two machines and one system: "The Venus machine," "The Interdata 3," "The Venus operating system," and then "Programming methodology." The context is "The software crisis," with "Machines were getting cheaper" and "Machines were getting bigger/faster," and she cites Dijkstra's "The Humble Programmer," CACM, October 1972. The questions of the period were "How should programs be designed?" and "How should programs be structured?"

The landscape she draws is a list of papers. Dijkstra, "Go To Statement Considered Harmful," CACM, March 1968. Wirth, "Program Development by Stepwise Refinement," CACM, April 1971. Parnas, "Information Distribution Aspects of Design Methodology," IFIP Congress 1971, with the one sentence quoted on the slide: "The connections between modules are the assumptions which the modules make about each other." Parnas again, "On the Criteria to be used in Decomposing Systems into Modules," CACM, December 1972. Then her own: "A Design Methodology for Reliable Software Systems," FJCC, December 1972, the partitions paper, with a diagram of operations over a partitioned state. The slide after it asks "How can these ideas be applied to building programs?" and the answer slide is two words: "Connect partitions to data types."

The claim to invention is dated. "Meeting in Savannah," the ACM SIGPLAN-SIGOPS interface meeting of April 1973, is where she "Started to work with Steve Zilles." The result is Liskov and Zilles, "Programming with Abstract Data Types," ACM SIGPLAN Conference on Very High Level Languages, April 1974. What that paper proposed is given in her own summary: abstract data types are "A set of operations," "A set of objects," and "The operations provide the only way to use the objects"; and the paper proposed "Clusters with encapsulation," "Polymorphism," "Static type checking (we hoped)," and "Exception handling."

The verdict comes at the end. "Modularity based on abstraction is the way things are done." "Modern Programming Languages: Are good." Then the reservations: "The state of programming: Isn't great, Especially web services and browsers, Return of globals." And a concern about the field: "Performance isn't the most important issue," set against "semantics" and "simplicity," with the example "one-copy consistency" and the line "Failures should be catastrophes."

The ACM's own statement of what the work claimed is the citation: "For contributions to practical and theoretical foundations of programming language and system design, especially related to data abstraction, fault tolerance, and distributed computing."

## 3. The mechanism, plainly

The lecture rebuilds CLU in nine slides of code. This is the mechanism, and it is worth setting out because every later section leans on it.

A cluster names a type by its operations: "IntSet = cluster is create, insert, delete, isIn, … end." A caller writes "s := IntSet$create( )" and "IntSet$insert(s, 3)." Inside, "rep = array[int]" names the representation, and "create = proc ( ) returns (cvt)" builds one by "return (rep$create( ))." The cvt marks the boundary where a rep becomes an abstract object. Outside the cluster there is no rep.

Polymorphism is a cluster with a type parameter: "Set = cluster[T: type] is create, insert, … end," used as "Set[int]$create( )." The constraint is the where clause: "where T has equal: proctype(T, T) returns (bool)." Inside the body, "rep = array[T]," and insert may write "if e = x[i]" only because the where clause promised equal.

Exception handling starts from Goodenough's "Exception Handling: Issues and a Proposed Notation," CACM, December 1975, and two decisions: "Termination vs. resumption" and "How to specify handlers." The code is "choose = proc (x: T) signals (empty)" with "if rep$size() = 0 then signal empty," and the caller's "except when empty: …". The slide on handling lists "Propagating," "Shouldn't happen," and "The failure exception," and states two principles: "Accurate interfaces" and "Avoid useless code."

Iterators answer "For all x in C do S" against two worries, "Destroy the collection?" and "Complicate the abstraction?" A visit to CMU, "Bill Wulf and Mary Shaw, Alphard, Generators," is credited. The code is "for x: int in Set[int]$members(s) do sum := sum + x end," and inside the cluster "members = iter (x: cvt) yields (T)" with "yield (z)" in a loop over the rep.

The design decisions are listed without argument: "Heap-based with garbage collection," "No block structure," "Separate compilation," "Static type checking," "No concurrency," "No go tos," "No inheritance." The goals are "Ease of use, Simplicity, Expressive power, Performance" and "Minimality, Uniformity, Safety." The reason for a language at all is four lines: "Communicating to programmers," "Do ADTs work in practice?," "Getting a precise definition," "Achieving reasonable performance."

Then type hierarchy. Inheritance "was used for" two things, "Implementation" and "Type hierarchy." Implementation inheritance "Violated encapsulation." Type hierarchy "Wasn't well understood, E.g., stacks vs. queues." The substitution principle is given in one sentence: "Objects of subtypes should behave like those of supertypes if used via supertype methods," citing "Data abstraction and hierarchy," SIGPLAN Notices, May 1988. The last mechanism slide sets "where T has …" against "where T subtype of S" and says "Proof happens at different times." That sentence is the hinge of section 7.

## 4. What it demonstrated, as reported

A lecture demonstrates nothing new. What it reports is what was demonstrated over thirty-five years, and what the field said in reply.

The field's reply is the award, announced March 10, 2009, under the headline "ACM Turing Award Goes to Creator of Influential Innovations in Computer Software Design," with a prize of $250,000 supported by Intel Corporation and Google Inc., presented June 27 in San Diego. Dame Wendy Hall, then ACM president, is quoted: "Her elegant solutions have enriched the research community ... and have had a practical effect as well." Andrew Chien of Intel: "Barbara Liskov's work consistently reflects an extraordinary combination of rigorous problem formulation and sound mathematics."

The long citation is the fullest account of what was demonstrated. She "has led important developments in computing by creating and implementing programming languages, operating systems, and innovative systems designs that have advanced the state of the art of data abstraction, modularity, fault tolerance, persistence, and distributed computing systems." Venus was "an early example of principled operating system design." CLU was "one of the earliest and most complete programming languages based on modules formed from abstract data types and incorporating unique intertwining of both early and late binding mechanisms." Argus "extended many of the CLU ideas to distributed programming, and incorporated the first versions of nested transactions to maintain predictable consistencies." It adds "decentralized information flow, replicated storage and caching of persistent objects, and modular upgrading of distributed systems."

MIT's announcement of the same day quotes Provost L. Rafael Reif: "Every time you exchange e-mail with a friend, check your bank statement online or run a Google search, you are riding the momentum of her research." It says her work influenced every major language since 1975 and names Ada, C++, Java and C#. That claim is MIT's. The slides say only that modern languages "Are good."

The lecture's reception at OOPSLA is on record in a BLOG@CACM post from the conference. It reports that she traced how "the thought process changed from having complete control over the resources to giving up some of it to gain abstractions," and that her remark that inheritance is not the most interesting feature in modern languages started a discussion. InfoQ's recording describes the keynote as reprising her Turing lecture, one hour and eighteen minutes long.

What the lecture itself reports as demonstrated is on the "What next" slides. "Modularity based on abstraction is the way things are done." Systems research "Has done well": distributed hash tables, map-reduce, client/server, distributed information flow. And what was not, in her list of missing features: "Procedures are important," "Closures and iterators," "Exception handling," "Built-in types," "Can't we do better for 'serialization'?"

## 5. The one idea restated

The operations provide the only way to use the objects.

Everything else in the lecture is the history of that sentence and the consequences of it. The Venus system and the 1972 partitions paper are the sentence before it had a name: a partition is state that only its operations touch. The Savannah meeting and the 1974 paper are the sentence finding its name, data type, and its home, the cluster. CLU is the sentence enforced by a compiler. The where clause is the sentence applied to a type you have not seen yet: you may use of T only what the clause names. Exceptions are the sentence applied to failure: an operation that cannot do what it says must say so through its interface, "Accurate interfaces," and not by leaving the rep in a state the caller has to inspect. Iterators are the sentence applied to loops: the body gets elements and never the collection. The substitution principle is the sentence applied to hierarchy: a subtype is one whose objects behave as the supertype's when used through the supertype's operations, because those operations are the only way anyone was ever using them.

The lecture also states the idea in its second form, the abstract she used at NSF and Princeton: abstraction "encompasses finding the right interface for a system as well as finding an effective design for a system implementation." Two halves. The interface, which is the set of operations, and the implementation, which is the rep and the code behind cvt. The power in the title is that the two can be worked on separately, and that the second can be changed without the first knowing.

The idea has a cost, and the lecture names it in the aside on storage: "persistent storage typically violates abstraction," a file system or a database "is a big space of globals" "without support for encapsulation." She wants "an object store," "automatic translation," "type preservation." The cost of the idea is that it must hold everywhere or it does not hold. One global breaks it.

For Phient the sentence lands on the word. A word is a set of relations, and the three operators are its operations. The lecture's question is whether they are the only way to use the word. The next five sections take the idea's parts in turn: the object and the word, the where clause and the channel, the assumption and the operator, the subtype and the substitution, the object store and the helix.

## 6. The word as a function, and the set of objects

Slide 19 defines an abstract data type by three things. In Phient the three are present, and naming them by her names is the first thing the lecture does for Phient.

A set of objects. A Phient word is not a point in a space. It is 32–64 relations, each a complex phasor on a closed circle with a count. The set of objects is the set of such words: on Austen, a pool of 1,228 words seen twenty or more times; on the quest oracle, 256 words in families of 8. Each is an object in her sense, with an identity that survives its state changing during training and a state that is the bytes on its channels.

A set of operations. Phient has three, in phinet::liskov. better(pred, a, b) is cos(pred, a) minus cos(pred, b). worse(pred, a, after, continuations) is cos(pred, a) minus the best continuation's fit after a. loss(pred, target, decoy, margin) is the hinge over better. forcing_choice composes them. The lecture's word for a type with exactly these operations is a cluster, and the phasors are its rep.

The operations provide the only way to use the objects. This is the clause Phient must answer for, and the answer is section 13. Here the point is what the clause means for the paradigm. In Phient a word is nothing but what it relates to, under what conditions. That is already an interface definition: the word is defined by how it behaves toward other words under c, and by nothing inside it. Her sentence and the paradigm sentence are the same claim made from two sides. She says the inside is hidden. Phient says the inside is not the meaning. Both agree that the meaning is the operations.

Where the reading sharpens is the count. Each relation carries a count, and the count decides whether a nudge lands. The byte deadband: a channel is one byte, 256 quanta of 2π; a nudge of 0.06 times sin d falls under half a quantum once sin d is below 0.2, about 11 degrees; nearest rounding froze every pair 8 quanta short. The count is state that none of the three signatures mention. Her principle of "Accurate interfaces" says a header should say what the operation depends on. better's does not say it depends on the count. The lecture's CLU example is exact here: choose "signals (empty)" because the caller must know that outcome is possible. Phient's better should signal a tie within the deadband for the same reason. It returns a small number instead, and section 11 shows what a table of small numbers looks like.

## 7. The relation channel as a coordinate, and where the proof happens

The polymorphism slides give two ways to constrain a type parameter and one sentence about the difference. "where T has …" lists the operations T must supply. "where T subtype of S" says T must behave as S. And: "Proof happens at different times."

The relation channel is Phient's type parameter. The relation circle is one rule, a phasor with a count. The condition c instantiates it: under c = is-a the circle is one thing, under c = part-of another. The quest oracle has five kinds, is-a, part-of, near, used-for and unlike, and a word's 64 channels are 64 instances. The coordinate of a channel is which instance it is. This is Set[int] against Set[string]: one cluster, a parameter, an instance per parameter, nothing shared between instances.

The rule that instances share nothing is measured. On the quest, within-kind MRR is 0.747; the same relation on the wrong channel gives 0.015; at an offset 0.005; chance is 0.021. A value read from a channel it was not written on is worth less than a guess. In CLU that is a type error the compiler catches. In phinet it is a number the report prints afterwards.

That is where her sentence about proof bites. "where T has …" is checked when the module is instantiated: the compiler looks for equal and finds it or does not. "where T subtype of S" cannot be checked that way; behaving like S is a property of all uses, and the proof is done by whoever writes the subtype, before any instantiation. Phient's checks are later than either. The parity suite, 62 of 62 against the PyTorch reference, is the closest thing to an instantiation check: the operators compute what the reference computes. Nothing checks that a channel is used under the c it was trained under. The wrong-channel number is a proof done after the run, on the run's own output.

The three ties make the where clause concrete. The trainer is one module. It runs over phinet::Tie::Sphere, one complex weight per spherical-harmonic degree, 64 = 8² channels as Y_l^m with l < 8, 1,192 params; over Tie::Anisotropic, one weight per (l, |m|), 36 slots, 4,776 params; and over the circle, 8,360 params. What it may use of a tie is the three operators. That is its where clause. On Austen hinge at 3 epochs the circle gives 14.7× valid / 14.6× test, the sphere 12.8×/13.1×, the anisotropic 16.0×/15.6×. The module compiled against all three; the results differ. Her slide says "has" and "behaves like" are different proofs. Phient has done the first. Section 12 is the second.

## 8. The operator as X ⇒ Y under c, and the assumptions modules make

Of all the sentences she could have carried from Parnas, she carried this one: "The connections between modules are the assumptions which the modules make about each other." It is the operator.

X ⇒ Y under c is one word's assumption about another under a condition. It is the connection, and in Phical the connection is the only thing stored: a phasor on X's channel c, moved by pairs. Her lecture treats the connection as the thing to be minimised and made explicit: the interface is where the assumptions are written down. Phient treats the connection as the thing to be learned. The form is the same.

The exception slides give the operator its outcomes. A CLU operation ends by returning or by signalling, and the handler slide lists what a signal can mean: "Propagating," "Shouldn't happen," "The failure exception." The operator in phinet ends the same three ways, though it does not say so. A ranking with a clear winner. A ranking in which the truth ties the best wrong word within the byte deadband, a signal that should propagate. Or a ranking in which nothing is above chance, which is failure. The held-out quest gives the third case: one hop 0.011 MRR against chance 0.021; two hops is-a 0.095, used-for 0.093, the other kinds at chance. Those runs signalled failure and the report averaged them.

Her two principles for exceptions are "Accurate interfaces" and "Avoid useless code." The first is section 6's complaint about the count. The second is a rule Phient can apply. Useless code, in her termination model, is what a caller writes to inspect a result that should have been a signal. The quest scoring by the three operators does that: mean better −0.008, mean worse −0.008, mean loss 0.108 at margin 0.1. A mean of −0.008 is a tie on nearly every row, averaged into a number no caller can act on.

The decoding arms show the operator's termination in text. Forty held-out prompts, thirty tokens each: trigram sampler KN perplexity 13.3, operator nearest-k 66.9 (was 97.2), union 66.8. Forcing, the operator invoked once more on each candidate and re-ranked by cos − λ·worse, gives 69.7 at λ = 0.05, 68.5 at 0.25 and 107.4 at 1, where the evaluator's own score rises to 0.712 and shared 4-grams reach 22.3%. In her terms forcing is a resignal, a result re-raised with a second operator's information. It does not beat greedy on text. The assumption it adds, that a word with no future is worse, is true of endgames and weak for English.

## 9. Substitution, and the subtype that behaves

The slide that carries her name is one sentence: "Objects of subtypes should behave like those of supertypes if used via supertype methods." It follows the two uses of inheritance and the verdict that implementation inheritance "Violated encapsulation." Type hierarchy is the use worth keeping, and substitution is the rule that makes it safe.

Phient has three places where substitution is the question.

The first is the tie. The circle, the sphere and the anisotropic tie are three implementations under one interface, and the trainer uses them only through the operators. Her rule asks: do they behave alike when used through better, worse and loss? In aggregate they do not. 14.7×/14.6×, 12.8×/13.1×, 16.0×/15.6× on the same Austen hinge run at 3 epochs. The synthetic degree-filter task separates them further: 200 items, 40 epochs, circle MRR 0.662 against identity 0.646, sphere 1.000. So the three ties are not subtypes of one another in her sense. They share operation names. That is the lecture's own distinction: sharing names is "where T has," behaving alike is "where T subtype of S," and the second is the harder proof. Phient has not done it per pair.

The second is the is-a channel. The quest's is-a kind is a subtype relation stated by the oracle. Under her rule, a word that is-a another should behave like it when used through the operations that apply to the other. Phient measures something weaker: whether the relation can be read back, within-kind 0.747, and whether it composes, two-hop is-a 0.095 against chance 0.021. Neither tests that the subtype behaves like the supertype under the supertype's operations. Section 12 writes that test.

The third is the persona. Three writers, Chesterton, Austen and Carroll, attributed at 71% overall (74/65/86) on held-out four-sentence paragraphs, chance 33%; dialogue replies judged as their persona 44% by the sector reader and 72% by the Kneser-Ney panel, chance 33%. A persona is a supertype of its sentences, and attribution asks whether a sentence behaves like its writer's when used through the reader. This is the substitution principle run backwards, from behaviour to type. It is the closest Phient has to a behavioural subtype test, and it is a different test.

What the substitution reading gives Phient is the direction of proof. Her slide says the subtype relation is proved by whoever writes the subtype, ahead of use. Phient's better, worse and loss are the supertype methods. A rep that claims to implement a Phient word owes a proof that, under those three, it behaves as the circle does. No such proof exists, and the aggregate numbers say it would fail. That is what the reading is for.

## 10. The object store, the return of globals, and the helix

The lecture's last third is about now. Three slides bear directly on what Phient has built.

"Return of globals." The state of programming "Isn't great," "Especially web services and browsers," and the reason given is the return of globals. A global is state any module can read, and it breaks the sentence of section 5 because it is used without going through operations. Phient's equivalent is the direct read of the rep. The helix writer reads each channel's byte to make a codon. The hexagram filer reads the centroid phase to pick one of 64 sectors. PhiTon reads the byte as a wavelength and a band. These are globals in her sense.

"Persistent storage typically violates abstraction." She wants "an object store," with "automatic translation" and "type preservation," instead of a file system or a database that is "a big space of globals." The helix is Phient's persistent store. The lexicon is written as a DNA-coded tree: 64 codons, base-4 ACGT of the channel; each codon carries its hexagram, its sphere degree and order, its B-DNA xyz at 10.5 codons per turn and 0.34 nm rise, and its PhiTon wavelength, band and hex. 18,496 words in 1.9 s, 75 MB, byte-exact round trip. Translation is automatic: a word goes out as codons and comes back as the same bytes. Type is preserved: what comes back is a word with 64 channels. Those two of her three the helix has. The third, that the store is used only through the operations, it does not: the helix is a global. The round trip proves it loses nothing, and nothing stops a module reading it.

"Failures should be catastrophes." This is the concern slide, set against performance. A system that fails should fail loudly and rarely. Phient's failures are quiet. A wrong-channel read returns 0.015. A tie within the deadband returns −0.008. Dither, which took the quest from 0.082 MRR to 0.594, costs corpus attribution 71% to 65%, and so ships off by default: a trade of the kind she warns about, though here the semantics are on both sides and the trade was measured.

The three operators are where the lecture lands last. Better, worse and loss are Phient's attempt at the set of operations for a word. The lecture says that set is the whole of the abstraction. On the quest they scored 256 words into 61 of 64 hexagrams, flat, at the byte ceiling. The lecture would say: the operations are correct, the rep is at its limit, change the rep and keep the operations. That is the power in the title.

## 11. What Phient has measured that bears on it

Everything here is from the facts file. Nothing else is claimed.

| slide or principle | what Phient measured | number |
|---|---|---|
| "where T has …", checked at instantiation | phinet vs PyTorch reference | 62/62 parity tests |
| instances of one generator share nothing | quest, within-kind / wrong channel / offset (chance 0.021) | 0.747 / 0.015 / 0.005 |
| "where T subtype of S": same operations, three reps | Austen hinge, 3 epochs, valid / test, params, time | circle 14.7×/14.6×, 8,360, 142 s; sphere 12.8×/13.1×, 1,192, 89 s; anisotropic 16.0×/15.6×, 4,776, 85 s |
| the reps differ where the task is spherical | synthetic degree-filter, 200 items, 40 epochs, MRR | circle 0.662 (identity 0.646); sphere 1.000 |
| the rep's own unit test | phinet::sphere vs scipy sph_harm_y | max difference 2.4e-15 |
| accurate interfaces: the un-signalled tie | quest scored by better / worse / loss (margin 0.1) | means −0.008 / −0.008 / 0.108; 61 of 64 hexagrams |
| termination: failure returned as a number | held-out quest, one hop; two hops is-a / used-for | 0.011; 0.095 / 0.093; others at chance |
| resignal: forcing cos − λ·worse, k = 32 | KN perplexity at λ = 0.05 / 0.25 / 1 | 69.7 / 68.5 / 107.4 (greedy 66.9) |
| object store: translation and type preservation | helix round trip, 18,496 words | 1.9 s, 75 MB, byte-exact |
| the global that decides a nudge | quest MRR, shipped vs attraction + dither | 0.082 → 0.594; hit@1 34.8%, hit@5 97.4% |
| performance against semantics | dither's cost to corpus attribution | 71% → 65%, ships off |

The parity row is the only check run before use, and it checks the operations, not any use of them. The three-rep row is the lecture's central promise and its central risk in one run: the interface held, the numbers moved, and the smallest rep is not the worst. The degree-filter row says the reps are not substitutable when the task has structure one of them can see. The hexagram row is the flat table of section 8.

Not measured: per-pair agreement between ties; any signal raised as a signal; the sphere at softmax or wide+deep; anything on Phi-4, since the quest is a synthetic oracle only; and timing that would separate the cost of abstraction from an engine that is allocation and bandwidth bound at these sizes, 1 MB op in 2–5 ms.

## 12. The experiment

The lecture's testable sentence is the substitution principle. Phient has three reps, one interface, one trainer.

State it as she does. Let the circle tie be the supertype S. Let the sphere and anisotropic ties be candidate subtypes T. The supertype methods are better, worse and loss. The principle predicts that objects of T behave like objects of S when used only through those three. The test is whether they do, pair by pair.

Setup, from the facts file. Austen: Emma, Persuasion, Sense and Sensibility; pool of 1,228 words seen twenty or more times; 204,592 training examples; hinge; 3 epochs. Train the circle, the sphere and the anisotropic tie with the trainer's source fixed. On the held-out set, for every (pred, a, b) triple the evaluation already forms, record the sign of better under each tie. Record, for each candidate, the sign of worse against the same continuation pool. Record the rows on which loss is zero under each.

The control is the circle at 14.7× valid / 14.6× test, 8,360 params, 142 s. It is the shipped rep and its numbers are already in the file.

The measurement is agreement. For each candidate T, the fraction of held-out triples on which sign(better_T) equals sign(better_S), and the same for worse and for loss. The principle passes for T if agreement is high and the disagreements are confined to triples where S itself is within the byte deadband, where |better_S| is below one quantum's nudge. It fails if disagreement is spread across triples with clear margins under S. The aggregate numbers, 12.8×/13.1× and 16.0×/15.6× against 14.7×/14.6×, already say the ties differ; this says where, and whether the difference is at the interface or under it.

The deadband matters because of her slide on when proof happens. A triple inside the deadband is one on which S has not decided. T disagreeing there is no violation, because S made no promise. T disagreeing outside it is.

A second arm tests the is-a channel as a subtype relation, on the quest oracle: 256 words, 1,280 rows, five kinds, chance MRR 0.021, control attraction-only with dithered rounding at 0.594 MRR. For each is-a row (x is-a y), take every relation y has under the other four kinds, and ask whether x, read under the same kind, ranks the same target in its top five. That is substitution in her words: x used through y's methods. The number is the fraction of y's relations that x inherits. The two-hop is-a figure, 0.095, is the only related number Phient has.

Neither arm has been run. The first costs three Austen runs, 85 to 142 s each. The second is unpriced.

## 13. Where we disagree or the analogy breaks

The lecture's sentence is that the operations provide the only way to use the objects. Phient's honest answer is that today they do not, and that Phient is not sure they should.

Three modules read the rep directly. The helix writer reads each channel's byte to make a codon. The hexagram filer reads the centroid phase to pick a sector. PhiTon reads the byte as a wavelength. None goes through better, worse or loss. In her terms each is a global, and her diagnosis of why programming "Isn't great" is the return of globals. Phient could answer that these are operations the header should list; section 15 says so. It must also say that her rule was built so the rep could change without anyone noticing, and Phient's rep is not a detail. The helix exists because the bytes are the word. A discipline that hides them hides the meaning. The disagreement is real and it is about what the rep is for.

The second break is the time of proof. She lists "Static type checking" among CLU's decisions and "Static type checking (we hoped)" among the 1974 proposals, and her slide on subtypes is about when proof happens. Phient proves nothing before a run. Its checks are the parity suite, 62 of 62, and the tables afterwards. The wrong-channel figure, 0.015, is a type error found by measuring its consequence. Phient accepts this because a learned relation cannot be checked before it is learned, and says so.

The third is "Failures should be catastrophes." Phient's failures are gradients. The held-out quest at one hop gives 0.011 against chance 0.021, worse than guessing, and nothing stopped. The lecture would call that a silent failure. Phient would answer that a ranking model has no natural place to stop, and that the right response is to raise the signal, which section 15 asks for.

The fourth is performance. She says it "isn't the most important issue" against semantics and simplicity. Phient's engine is allocation and bandwidth bound, 1 MB op in 2–5 ms, and the anisotropic tie's case is partly that it is smaller and faster: 4,776 params and 85 s against 8,360 and 142 s. Phient does weigh that. It also weighs that the anisotropic tie scores higher, 16.0×/15.6× against 14.7×/14.6×, and the lecture's rule would take the second reason and not the first.

The fifth is inheritance, and there is no break. CLU had "No inheritance," implementation inheritance "Violated encapsulation," and Phient has no hierarchy of words. "Emma" does not inherit from anything. Under c = is-a there is a phasor with a count, and the quest measures it.

## 14. The person and the moment

The verified facts, from the ACM award pages, the ACM and MIT announcements, and the event pages.

Barbara Jane Huberman was born on November 7, 1939, in California. She took a BA in mathematics at Berkeley in 1961 and a PhD in computer science at Stanford in 1968, under John McCarthy, with a thesis on a program to play chess endgames. The ACM's biography calls her one of the first women in the United States to be awarded a computer science PhD; MIT's announcement calls her the first U.S. woman to earn a PhD from a computer science department. This paper reports both wordings. She was at the Mitre Corporation from 1968 to 1972 and joined MIT's Laboratory for Computer Science, becoming full professor in 1980, Associate Department Head from 2001 to 2004, Associate Provost for Faculty Equity from 2007, and Institute Professor in 2008.

The award is the ACM A.M. Turing Award for 2008, announced on March 10, 2009, with the short citation quoted in section 2 and the long citation in section 4. The prize was $250,000, supported by Intel and Google, presented on June 27, 2009, in San Diego. MIT's announcement notes she was the second woman to receive it. The ACM page lists the IEEE John von Neumann Medal in 2004 and election to the National Academy of Engineering in 1989. The Princeton page adds the ACM SIGPLAN Programming Language Achievement Award in 2008.

The lecture's moment is the autumn of 2009. The slide deck read here is dated October 2009 and is kept in the archive of SOSP 2009 at Big Sky. The same talk was the OOPSLA 2009 keynote that month, where a BLOG@CACM post recorded the room's reaction. She gave "The Power of Abstraction" again as an NSF CISE Distinguished Lecture on December 3, 2009, and at Princeton on March 10, 2010, under the abstract quoted in section 5. The ACM Digital Library files a video of the Turing lecture under this title; its page was not reached and its recording date is not claimed.

What the moment was, in her slides, is the last third of the deck. Thirty-five years after the 1974 paper, "Modularity based on abstraction is the way things are done," and the concerns are "Concurrency," "Multi-core machines," "Distributed systems?," "Internet Computer" with "Semantics, reliability, availability, security," and "Massively parallel machines." The person who wrote a chess program with two comparison functions and no evaluation number in 1968 closes her Turing lecture by saying performance is not the most important issue. That is verified, and it is the sentence this series has been about.

## 15. What remains

For Phient, in the order they could be done.

List the reads. For each module in phiano and phiecs, say whether it reaches a word's phasors through better, worse and loss or directly. The helix writer, the hexagram filer and PhiTon are known to read directly. docs/82's inventory is the place to record the rest. This is the lecture's first demand and it is a day's work.

Decide what they are. Each direct read is either an operation the header should name, with a signature and a stated dependence on the count, or a global that Phient chooses to keep. Section 13 said Phient keeps the helix because the bytes are the word. The answer may differ per module.

Raise the signals. better should signal a tie within the byte deadband. worse should signal a dead end. loss should signal a missed margin. Today each returns a number and the quest report averages them into −0.008, −0.008 and 0.108. A report that counted signals would say how many of 1,280 rows are ties, and the flat hexagram table would become a count.

Run the substitution test. Section 12's first arm, control the circle at 14.7×/14.6×. It is the cheapest direct measure of whether the three ties are one type or three.

Run the is-a substitution on the quest. Section 12's second arm. It turns the oracle's is-a rows into her rule and gives a number no current run gives.

Take the lecture's list of missing features as a checklist. "Procedures are important." "Closures and iterators." "Exception handling." "Built-in types." "Can't we do better for 'serialization'?" Phient has an iterator, a serialisation, and no exceptions.

For the series, one thing remains. This paper read the slides. It did not hear the lecture. The ACM's video, the OOPSLA recording and her own copy of the slides at MIT were not reached, and what she said between the slides, which in a retrospective is most of it, is not here. A reading of the recording would settle what she said about Venus, which the slides give in two bullet points, and about the Interdata 3, which they give in one.

For the three operators, the lecture's last word is the one to take. The operations are the abstraction. Better, worse and loss are Phient's operations for a word. The rep under them has changed three times and the operations have not. On the quest they report a flat table at the byte ceiling. Her lecture says that when the interface is right and the numbers stop, you change the rep. The Phi-4 quest, not yet run, is the next rep.

## 16. Sources

Reached and read, through WebFetch on 2026-09-14.

Barbara Liskov, "The Power of Abstraction," slide deck dated October 2009, fifty-eight slides, from the SOSP 2009 (Big Sky) archive at SIGOPS (https://www.sigops.org/s/conferences/sosp/2015/archive/2009-Big_Sky/01-1-liskov-slides.pdf). Every slide quotation in this paper is from this deck: the outline, the cited papers with venues and months, the Parnas sentence, the 1974 definition, the CLU code, the design decisions, the LSP sentence and its 1988 citation, "Proof happens at different times," and the "What next" slides.

ACM A.M. Turing Award, recipient page (https://awards.acm.org/award-recipients/liskov_1108679): award year 2008, the short citation, the long citation quoted in section 4.

ACM A.M. Turing Award, laureate biography (https://amturing.acm.org/award_winners/liskov_1108679.cfm): birth date and place, Berkeley 1961, Stanford 1968, Mitre 1968–1972, MIT positions, full professor 1980, Institute Professor 2008, von Neumann Medal 2004, NAE 1989, "one of the first women in the United States to be awarded a computer science PhD."

ACM press release, March 10, 2009 (https://www.acm.org/media-center/2009/march/acm-turing-award-goes-to-creator-of-influential-innovations-in-computer-software-design): headline, prize, sponsors, presentation date and place, the Hall and Chien quotations.

MIT News, March 10, 2009 (https://news.mit.edu/2009/turing-liskov-0310): the Reif quotation, "second woman," "first U.S. woman to earn a PhD from a computer science department," the languages claim attributed to MIT in section 4.

CRA Government Affairs blog, March 2009 (https://cra.org/govaffairs/blog/2009/03/turing-award-recipient-announced/): confirms the long citation's first sentence and the nested-transactions clause.

BLOG@CACM, "The Power of Abstraction: Barbara Liskov's OOPSLA Keynote" (https://cacm.acm.org/blogcacm/the-power-of-abstraction-barbara-liskovs-oopsla-keynote): the OOPSLA 2009 reception and the "complete control over the resources" sentence. Author and date not shown as fetched.

InfoQ (https://www.infoq.com/presentations/liskov-power-of-abstraction): the OOPSLA talk as reprising the Turing lecture; one hour eighteen minutes.

NSF CISE Distinguished Lecture page (https://www.nsf.gov/events/power-abstraction): December 3, 2009, and the abstract quoted in section 5. Princeton CS event page (https://www.cs.princeton.edu/events/event/power-abstraction): March 10, 2010, same abstract, SIGPLAN award 2008.

Not reached. Wikipedia was consulted only for the thesis title. The ACM Digital Library record of the Turing lecture (https://dl.acm.org/doi/10.1145/1283920.1962421) returned 403. The ACM video page (https://amturing.acm.org/vp/liskov_1108679.cfm) required a permission not granted in time. Her MIT copy of the slides (http://www.pmg.csail.mit.edu/~liskov/turing-09-5.pdf) returned 403. The YouTube recording was rate-limited. Crossref was refused by the proxy.

Consequences. No spoken words from the lecture are quoted; every quotation attributed to the lecture is slide text. The date and place at which the ACM's filed video was recorded are not claimed. The laureate page as summarised gave the award year once as 2009 and the recipient page gives 2008; this paper follows the recipient page and the press release, the 2008 award announced in 2009. Nothing is claimed about the Venus machine or the Interdata 3 beyond the slide titles.

All Phient numbers are from PHIENT_FACTS.md and docs/86_liskov_three_operators.md in this repository. No number about Phient was taken from anywhere else, and none was computed for this paper.
