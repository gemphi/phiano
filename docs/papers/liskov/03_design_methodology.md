# Reliability by structure: the partition as a word that owns its resources

*Phient interpreted papers · Liskov series, 3 of 16 · 2026-02-09 · on: "A Design Methodology for Reliable Software Systems" (B. H. Liskov, 1972, Proceedings of the AFIPS Fall Joint Computer Conference, Vol. 41, Part 1)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the third of sixteen papers in which Phient reads one work by Barbara Liskov through its own paradigm. The work is a short conference paper from December 1972. It has no theorem and no experiment. It has an argument, and the argument is this: a system is reliable when its design makes the set of relevant test cases small enough to enumerate and clear enough to identify, and the way to get such a design is to divide the system into partitions, each of which is one level of abstraction, owns its resources, and talks to the others only through explicit arguments. The paper draws on Dijkstra's levels of abstraction and on her own Venus operating system, and it is the paper she later named as the origin of her idea of partitions, which became data abstraction and then CLU.

We read it here because the paper is a claim about what a unit of a system should be. Phical also has a claim about what a unit is. A word is a set of 32–64 relations, each on its own channel, and nothing else. That is a partition in her sense: a thing defined by what it owns and by what it exposes. Her rule that data passes between partitions only as explicit arguments is, in Phical, the rule that a pair of words moves only under a condition c that is named. Her rule that lower levels do not know about higher ones is a statement about the direction of X ⇒ Y.

Sections 2 to 5 state what the paper says, in its terms and with the words that the sources we reached preserve. Sections 6 to 10 place her objects inside the Phient family: the partition as a word, the owned resource as a channel, the explicit argument as the condition, the hierarchy of levels as the quest's hops and the helix's tree, and her account of reliability as a design property as a claim about how a language engine should be tested. Section 11 lists what Phient has measured that bears on any of this, all of it from the facts file. Section 12 names an experiment with its control. Section 13 says where the analogy breaks. Section 14 is about the person and the year. Section 15 is what remains.

One limit is stated at the top. We did not reach the paper's own text. The ACM Digital Library refused the fetch. What we quote of the 1972 paper comes from four secondary summaries and from Liskov quoting her own page 195 in her 1992 history of CLU. Where a detail is not in those sources, this page does not state it.

## 2. What the work claims, in its terms

The paper begins with reliability. Reliability, in its words as relayed by the summaries, "implies that a system works as expected under a given set of conditions." The standard way of getting there was to build the system and then debug it. Liskov says plainly that this has not worked: "The unfortunate fact is that the standard approach to building systems, involving extensive debugging, has not proved successful in producing reliable software." She cites Dijkstra's line that program testing can show the presence of bugs but never their absence.

Her move is to relocate the problem. Testing cannot be made to guarantee reliability by testing harder. It can be made to guarantee reliability only if two things hold: there is a minimal set of relevant test cases, and every case in that set can be generated. Whether those two things hold is decided before any test is run. In her sentence, "It is the system design which determines how many test cases there are and how easily they can be identified." That is the claim of the paper. Reliability is a property of structure, and the structure is chosen at design time.

She then asks what kind of structure. Modularity is the usual answer, and she does not reject it, but she says it is not enough on its own. If modularity is viewed only as an aid to management, "then any ad hoc modularization of the system is acceptable," and she lists the ways an ad hoc modularization fails: a module that does too much, a function that is spread across several modules, and modules that behave unpredictably because they share data. Her conclusion is that "the success of modularity depends directly on how well the modules are chosen."

The rest of the paper says how to choose them. Two ideas do the work. The first is levels of abstraction, which she credits to Dijkstra. The second is structured programming, which restricts control flow to concatenation, conditional selection and iteration, with no goto. Her unit is the partition. A partition is one level of abstraction, made of one or more functions that share resources the partition owns. Partitions connect only through explicit arguments passed to external functions. The paper ends with a two-phase design process and a statement of when a design is finished: when every abstraction has a partition, the hierarchy and resource ownership are fixed, the interfaces and control flow are clear enough that the test cases can be identified, and a user guide could be written.

## 3. The mechanism, plainly

There is no mathematics in this paper. There are rules, exact enough to state one by one.

A level of abstraction is a group of related functions. The lowest level is the one closest to the machine. Each function is either external, meaning it may be called from other levels, or internal, meaning it serves only its own level. Two rules govern the levels. As the summaries relay them: "Each level has resources which it owns exclusively and which other levels are not permitted to access," and "Lower levels are not aware of the existence of higher levels and therefore may not refer to them in any way." A higher level asks a lower level for service by calling one of its external functions. A lower level never calls up.

A partition is a level of abstraction with its resources attached. Liskov quoted her own definition in 1992: "The system is divided into a hierarchy of partitions, where each partition represents one level of abstraction, and consists of one or more functions which share common resources." The definition continues with the rule about data: "The connections in data between partitions are limited to the explicit arguments passed from the functions of one partition to the (external) functions of another partition. Implicit interaction on common data may only occur among functions within a partition." She gave the page as 195.

The connection rules follow. A connection between partitions is valid when it respects the level rules and the structured programming rules, when data crosses only as explicit arguments, and when the partitions are logically independent, which she glosses as "The combined activity of the functions in a partition support its abstraction and nothing more." Structured programming enters as a discipline on control flow, and she gives it a second job. It is a test of understanding. "The best rule is probably to keep trying to write structured programs; failure will indicate that the system abstractions are not yet sufficiently understood."

The design process has two phases that run together. In the first, the designer identifies the abstractions that describe the system's eventual behaviour. In the second, the designer checks that they are practical and fixes the data connections and control flow among the partitions. One summary lists five kinds of abstraction the paper offers as guides: of a resource, of a data characteristic, simplification by limiting information, generalization by grouping, and isolation of functions likely to change. We could not check that list against the paper and give it as relayed.

The whole mechanism is one constraint. Every effect a partition has goes through a named door, and every door is on the partition's own wall.

## 4. What it demonstrated, as reported

The paper reports no experiment and no measurement. What it offers as evidence is experience, and the experience was Venus. Venus was a small time-sharing system built at MITRE on an Interdata 3, with microprogramming, and Liskov published its design in Communications of the ACM in March 1972, nine months before this paper. The Turing Award biography describes Venus as "a small timesharing system for the Venus machine used to experiment with how different architectures helped or hindered this process," supporting sixteen teletypes with a virtual machine per user. Her 1992 history of CLU says Venus "was organized as a collection of partitions, each with externally accessible functions and hidden state information, which communicated by calling one another's functions."

So the demonstration, such as it is, is that one operating system had been built this way and had worked. The 1972 methodology paper generalizes the practice. Its examples, if it contains worked examples, are not preserved in the sources we reached, and we do not describe them. The Turing Award biography adds one detail about where the discipline came from: it says she describes the influence of Algol on her partitioning of variable scope for the Venus operating system. Scope is the language-level form of resource ownership, and the paper's rule that a partition owns its resources exclusively is that scope rule lifted to the level of a system.

What the paper demonstrated in the longer run is on record. In "A History of CLU" Liskov writes: "This notion of partitions was based on Dijkstra's ideas about levels of abstraction and my own work on the Venus operating system." She groups the paper with Parnas's 1971 and 1972 papers and Randell's 1969 paper as "the work on modularity," which "was concerned with what program components should be like." Then she says something that matters for how we read it. "By the fall of 1972, I had become dissatisfied with the papers on programming methodology," including her own, "because I believed it was hard for readers to apply the notions to their own programs." The methodology paper was the last thing she wrote before deciding that a methodology was not enough and that the unit had to be put into a language. That decision, made in the latter part of 1972 and the summer of 1973, became abstract data types and CLU.

The paper was reprinted in 1977 in the IEEE Tutorial on Software Design Techniques, edited by Freeman and Wasserman, at pages 53 to 61. That is the extent of what we can say it demonstrated. It was read, it was reprinted, and its author outgrew it within a year.

## 5. The one idea, restated

The idea is that reliability is decided by what the parts are and by what can pass between them, and that this is fixed before any code runs.

Put it as three sentences. A part is defined by what it owns. A part touches another part only through a named argument. A part below never refers to a part above. If those three hold, the number of ways the system can go wrong is countable, because every path from one part to another goes through a door that has a name. The test cases are the doors.

The idea is easy to mistake for a rule about tidiness. It is a rule about counting. A designer who follows it is not making the system neater for its own sake. The designer is making the set of things that could go wrong small enough to write down, and then writing it down. Liskov's opening argument is that testing can only guarantee reliability if the set of relevant cases is both minimal and enumerable. Sharing data between modules destroys enumerability, because an effect can travel by a path no one wrote down. Explicit arguments restore it. The hierarchy restores something else: it makes the count finite in the vertical direction, since a level's behaviour depends on levels below it and nothing above it, so a level can be tested once and then relied on.

Restated for Phient, the idea says: the unit of a language engine should be a thing that owns its state, exposes a fixed interface, and depends only on what is beneath it. Phical's word already meets the first two. A word owns its 32–64 relations. It exposes them as phasors on a circle with a count. It does not own a point in a space that other words also occupy. Whether it meets the third, dependence only on what is beneath, is the question sections 6 to 10 pursue, and it is not obviously answered yes.

There is a second restatement that Liskov herself supplies. Structured programming is a test of understanding. If you cannot write the level without a goto, you do not yet know what the level is. Phient's equivalent is a training run. If a tie cannot learn the relation without leaking across channels, the channels are not yet the right partition. The facts file holds one case of this, the byte deadband, and section 11 says what it showed.

## 6. The partition as a word: what a unit owns

Liskov's partition is a set of functions that share resources the partition owns exclusively. Nothing outside may touch those resources. What the outside sees is the external functions.

Phical's word is the same shape. A word is 32–64 relations, one per channel. Each relation is a phasor on a closed circle with a count. Those are the word's resources, and Phical holds them for that word alone. There is no shared embedding space in which two words are points and the distance between them belongs to neither. The paradigm sentence says so: Phical stores no point. What another word sees of this one is what happens when the pair moves under a condition. That is the external function.

The correspondence can be laid out.

| Liskov 1972 | Phical |
|---|---|
| partition | word w |
| resources owned exclusively | the 32–64 relations, each phasor and count |
| external function | the pair's motion under a condition c |
| internal function | the update of one channel's phasor and count |
| explicit argument | the condition c named in X ⇒ Y under c |
| implicit interaction on common data | any effect that crosses channels unnamed |

The last row is where the analogy earns its keep. Liskov permits implicit interaction on common data only within a partition. In Phical, the within-partition case is the interaction among a single word's channels, which the tie is free to couple. The forbidden case is an effect that reaches from one word's relations to another's by a path that no condition names. The measured failure mode in the facts file, the byte deadband, is exactly an effect of this kind: a nudge of 0.06·sin d falls below half a quantum once sin d < 0.2 and the rounding freezes every pair 8 quanta short. The quantization is a shared resource that every word's channels depend on and that no argument names. In her terms it is a global variable. Section 11 gives the numbers.

There is one place the shape does not match. Liskov's partition has a fixed, designer-chosen interface. Phical's word has a fixed count of channels, but which relation lives on which channel is learned, and the interface is the same for every word. Her methodology would call this a partition whose abstraction has not yet been named. That is a fair criticism and section 13 returns to it.

## 7. The owned resource as a channel, and the hierarchy as a coordinate

Liskov's second rule is that each level owns resources exclusively. In Phical, the thing owned is a channel. A channel is one byte, 256 quanta of 2π, holding one phasor, and each of a word's 32–64 channels is a relation the word has. So the resource a word owns is a set of coordinates, and the coordinate is the channel index.

Her hierarchy gives those coordinates an order. Lower levels are closer to the machine. Higher levels ask lower levels for service. Phinet's Tie::Sphere reads the 64 channels as spherical harmonics Y_l^m with l < 8, and the anisotropic tie gives one weight per (l, |m|), 36 slots. A degree l is a level. Low degrees are broad, high degrees are fine. That is an order among the coordinates that the circle tie did not have, and it is the kind of order Liskov's levels impose: a low-degree coordinate says something about the word that a high-degree coordinate refines and does not contradict.

Whether the sphere tie's degrees behave like her levels is partly measured. The synthetic degree-filter task in the facts file, 200 items over 40 epochs, gave the circle tie an MRR of 0.662 against an identity baseline of 0.646, and the sphere tie 1.000. A task defined by degree is solved by a tie that knows degree. That is consistent with degree being a real coordinate. It is not a test of her rule that lower levels do not refer to higher ones, because the sphere tie trains all degrees jointly, and gradient flows in both directions. Section 13 takes that up.

The helix reads the same coordinates as a tree. Each of the 64 channels is a codon, base-4 ACGT of the channel index, with a hexagram, a sphere degree and order, a B-DNA position at 10.5 codons per turn and 0.34 nm rise, and a PhiTon wavelength, band and hex. A tree is a hierarchy. In Liskov's paper the hierarchy is among partitions; in the helix it is among a single word's channels, since a codon's first base is a coarser division than its third. The lexicon of 18,496 words was written this way in 1.9 s, 75 MB, with a byte-exact round trip. That round trip is a reliability claim of her kind: the structure was designed so that the test could be identified and run.

What Liskov would ask, and what we cannot yet answer, is which channels are external. Her external functions are the only ones another level may call. Phical's channels are all read by the operator at once. No channel is kept private. Section 15 records the question.

## 8. The operator as X ⇒ Y under c: the explicit argument is the condition

Liskov's third rule is the one about data. Connections in data between partitions are limited to explicit arguments passed from the functions of one partition to the external functions of another. Nothing crosses without a name.

Phical's operator is X ⇒ Y under c. Two words, X and Y, move, and the motion happens under a condition c. The condition is the argument. If c is not stated, there is no motion. That is the same discipline as hers, applied to language. A word does not silently affect another word. It affects it under a named condition, and the effect is the pair's motion on the relation circle.

The paper's rule about direction reads onto the arrow. Lower levels do not refer to higher levels. In X ⇒ Y, X is the level that is asked and Y is what comes back. The operator reads X's channels and produces a distribution over Y. Y is not consulted in forming the request. In the Austen operator, X is the prompt and Y is the next word, and the measured lift over chance is a measure of how well the request is answered: hinge 14.7× valid and 14.6× test at 8,360 params, softmax wide and deep 30.7×/30.3× at 8 epochs. Those numbers are the external function's behaviour under test, which is the thing Liskov says a design should let you measure.

Her structured programming rule is a rule about control flow: concatenation, conditional selection and iteration, and no goto. The operator has the same three. Concatenation is composition of operators, which is how the paradigm sentence says the family is built. Conditional selection is the condition c. Iteration is generation: apply the operator, take the result as the next X, apply it again. The forcing_choice in phinet::liskov is a conditional selection with three outcomes, better, worse or loss, and the decode_eval numbers show what happens when the selection is weighted too heavily: forcing λ=0.05 gives perplexity 69.7, λ=0.25 gives 68.5, and λ=1 gives 107.4. A goto is a jump that the structure did not sanction. A forcing weight of 1 behaves like one.

The point of her rule is that each connection can be tested on its own. The operator's connections are the conditions, and the quest's oracle tests them one by one: within-kind 0.747, wrong channel 0.015, offset reading 0.005. A wrong-channel score at chance is her rule holding. The connection that was not named carries nothing.

## 9. Levels of abstraction in the quest and the helix

Liskov's levels form a hierarchy in which a higher level is built from calls on lower levels. The quest is Phient's hierarchy of that kind. Its synthetic oracle has 256 words in families of 8, related by five kinds: is-a, part-of, near, used-for, unlike. A one-hop relation is a level. A two-hop relation, X is-a Y and Y is-a Z, is a level built on the level below.

The measured numbers read as a test of her hierarchy. On the held-out 20%, one hop scored 0.011 MRR, at chance. Two hops scored 0.095 for is-a and 0.093 for used-for, and roughly chance for part-of, near and unlike. So the level above learned something the level below did not carry alone, but only for two of the five kinds. In her terms, the is-a and used-for abstractions were understood well enough that a partition could be built on them; the other three were not. Her rule for that case is exact: failure to structure indicates that the abstractions are not yet sufficiently understood. Phient should read the three chance-level kinds that way.

The three operators give the quest a second reading. The 256 words were scored by better, worse and loss and filed by hexagram, and they occupy 61 of 64 hexagrams. Mean better was −0.008, mean worse −0.008, loss 0.108 at margin 0.1. The facts file calls this flat and names the byte ceiling as the cause. The hexagram filing is a partition of the lexicon into 64 cells, and the scores are flat across them. Her hierarchy has an order; this filing has not yet shown one.

The helix is the other place the hierarchy lands. Its codon tree is base 4 over three positions, which gives 64 leaves. That is a three-level hierarchy, and the levels are nested in her sense: the first base partitions the lexicon into four, the second into sixteen, the third into sixty-four. A lower level in the tree does not refer to a higher one. The round trip is byte-exact, which is to say the structure was designed so the test could be enumerated: 18,496 words in, 18,496 words out, compared byte for byte.

Where the quest and the helix differ from her design is that she chose her abstractions by hand, in the first phase of her process, and only then investigated whether they were practical. The quest's five kinds and the helix's four bases were also chosen by hand. The channels that carry them were not. Phical lets training decide which channel holds which relation. That is the part Liskov's method would say is not yet designed, and it is the part section 12 proposes to test.

## 10. Reliability by structure as a claim about language

Liskov's deepest claim is that reliability is a property the structure either has or lacks before testing begins. Read as a claim about a language engine, it says this: a model is reliable when its design tells you which cases to test and lets you generate them all.

Phient has one instance of this that is clean. The phinet engine was tested against a PyTorch reference and passed 62 of 62 parity tests. That number is only meaningful because the engine was partitioned into operations each of which has a reference, and the reference could be run for each. The design determined how many test cases there were. That is her sentence made concrete, and it is why the parity number is in the facts file at all.

The rest of Phient's measurements are statistical, and here her claim strains. Reliability, in her definition, means the system works as expected under a given set of conditions. A language operator does not work or fail. It scores 14.7× chance or 30.7× chance. Her methodology has no place for a score. What it has is a way to make a score attributable. If the operator is partitioned, a change in the score can be traced to the partition that changed. The facts file has one example. Moving from the circle tie to the sphere tie changed one thing, the tie, and the Austen hinge result moved from 14.7×/14.6× at 8,360 params to 12.8×/13.1× at 1,192 params, and then to 16.0×/15.6× at 4,776 params with the anisotropic tie. Because only the tie changed, the difference is the tie's. That is reliability by structure applied to a measurement.

Her claim lands on language a second way, through the rule that abstractions are only understood when they can be written without a goto. Phient's operator on text does not beat greedy decoding when forcing is added: nearest-k KN gives perplexity 66.9 and every forcing weight gives more. Her reading would be that the abstraction the three operators name, better, worse and loss, is not yet understood well enough on text to be structured into the decoder. That is a hard reading, and the facts file gives the same one.

Her user-guide test is also a test for Phient. A design is complete when a basic user guide can be written. A guide to the operator would have to say what a channel means. It can say what a channel is, one byte, 256 quanta, a phasor and a count, and its degree on the sphere and codon on the helix. It cannot say what relation it holds, because that is learned. By her criterion the design is not complete, and section 15 says so.

## 11. What Phient has measured that bears on it

Every number here is from the facts file. Nothing else is claimed.

The parity test is the one measurement that is reliability in her sense. phinet against the PyTorch reference: 62 of 62 parity tests pass. The sphere module against scipy's sph_harm_y: maximum difference 2.4e-15. Both are tests whose cases were determined by the design, since each operation has a reference to be compared against.

The byte deadband is the one measurement that is a violation in her sense. A channel is one byte, 256 quanta of 2π. The nudge 0.06·sin d falls under half a quantum once sin d < 0.2, which is 11°. Nearest rounding froze every pair 8 quanta short. Dithered rounding fixed the freeze in the quest, taking the attraction-only trainer to 0.594 MRR, hit@1 34.8%, hit@5 97.4%, against 0.082 for the shipped trainer and chance 0.021. But dither cost corpus attribution, 71% down to 65%, so it ships off by default. The quantum is a resource that every word's channels share and that no condition names. Its effect crossed partitions, and the cost of fixing it in one place appeared in another. That is what she said would happen with implicit interaction on common data.

The channel-independence measurements bear on her explicit-argument rule. In the quest, within-kind reading scored 0.747, wrong channel 0.015, offset reading 0.005, chance 0.021. Reading the named channel works. Reading an unnamed one carries nothing. Held out: one hop 0.011; two hops is-a 0.095, used-for 0.093, part-of, near and unlike at about chance.

The tie comparisons bear on attribution by structure. Austen hinge, 3 epochs: circle 14.7× valid / 14.6× test, 8,360 params, 142 s; sphere 12.8×/13.1×, hit@10 0.182, 1,192 params, 89 s; anisotropic 16.0×/15.6×, hit@10 0.254, 4,776 params, 85 s. Softmax β=8: circle 18.0×/18.8×; sphere 13.2×/13.5×, hit@10 0.189, 408 s. Softmax wide and deep, 50,016 params: 27.6× at 3 epochs, 30.7×/30.3× at 8 epochs, hit@10 0.348. The synthetic degree-filter task: circle 0.662 (identity 0.646), sphere 1.000.

The three operators bear on structured control. decode_eval at k=32: nearest-k KN 66.9; forcing λ=0.05 69.7, λ=0.25 68.5, λ=1 107.4, evaluator 0.712, distinct 0.777, shared 4-grams 22.3%. Forcing does not beat greedy on text. Quest scored by the three operators and filed by hexagram: 256 words occupy 61 of 64 hexagrams; mean better −0.008, worse −0.008, loss 0.108 at margin 0.1.

The helix bears on enumerable testing: 18,496 words in 1.9 s, 75 MB, byte-exact round trip.

Not measured: any run in which channels were assigned by design rather than learned; any run with private channels; any test of one level trained with the levels above frozen. Not done: no Phi-4 run, only the synthetic oracle; sphere at softmax wide and deep is unrun.

## 12. The experiment: a partitioned tie, with the control named

Liskov's method has two phases. First name the abstractions. Then check that they are practical by fixing the data connections and seeing whether the structure can be built. The experiment that tests her idea inside phinet follows the same two phases.

Phase one names the abstractions. The quest has five kinds: is-a, part-of, near, used-for, unlike. Assign each kind a fixed block of channels by design, so that the 64 channels are partitioned into five named blocks plus a remainder. This is what the paper means by identifying the abstractions before building. Today the trainer is free to put any relation on any channel.

Phase two fixes the connections. Build a tie in which the weight for a pair of channels is zero unless both are in the same block. That is her rule that implicit interaction on common data may occur only within a partition. Between blocks, the only coupling is the condition c that names the kind. Train it on the same 1,280 rows and 293 targets, attraction-only with dithered rounding.

The control is the shipped attraction-only trainer with dithered rounding and the unpartitioned tie, at 0.594 MRR, hit@1 34.8%, hit@5 97.4%. The experiment passes if the partitioned tie reaches a within-kind score at or above 0.747 and a wrong-channel score at or below 0.015 with fewer parameters, and if the held-out two-hop scores for part-of, near and unlike rise above chance. It fails if partitioning costs MRR on the in-sample rows, which would mean the kinds were not the right abstractions and, by her own rule, that they are not yet understood.

The second half tests her direction rule. Train the lowest block first, freeze it, then train the next block with the first frozen, and so on. This checks whether a level can be built on the level below without the level below changing. The control is joint training of all blocks at once, which is what phinet does now. If frozen-level training matches joint training, her hierarchy holds in phinet. If it falls short, the levels depend on each other in both directions and her rule does not apply.

Two numbers come out of this that the facts file does not have: what channel independence costs or buys in MRR, and how much of the operator's lift comes from cross-level coupling. Neither has been run. Nothing here predicts the result.

The cost is small. The partitioned tie is a mask on the existing tie and the frozen-level schedule is a loop over blocks. The persona attribution test, 71% against 33% chance, should be rerun with the partitioned tie as a check that partitioning does not do to attribution what dither did.

## 13. Where we disagree, and where the analogy breaks

The analogy breaks in four places, and the first is the largest.

Liskov chooses her abstractions. The designer names them in phase one and the system is built from them. Phical does not choose. It fixes the number of channels and lets training decide what each one carries. In her method this is a design that has not been designed. She said of her own methodology paper within a year that a structure the reader cannot name is hard to use. We accept the criticism. We do not accept that it decides the matter, because no one designed a language, and a model of one may have to find the partitions rather than impose them. Section 12 is how we would find out which is right for the quest.

The second break is direction. Her lower levels do not refer to higher levels. Phinet trains its ties jointly, and gradient flows from the loss at the top through every degree of the sphere tie to the bottom. During training, the lower level is shaped by the higher. Her rule is a rule about a finished system. Phinet's finished operator does obey it, since X ⇒ Y reads X and not Y. But the process that made the operator does not, and she would count the process.

The third break is the meaning of reliability. Her definition is that the system works as expected under a given set of conditions. An operator that scores 30.7× chance is not working as expected, because there is no expectation to work to. Her methodology assumes a specification. Phient has measurements and no specification. What we take from her is the weaker claim that a change in a measurement should be attributable to a change in one partition, and section 10 gave the case where it was.

The fourth break is testing. Her paper says the design should make the relevant test cases enumerable. The parity tests are enumerable. The persona test, dialogue test and Austen operator are samples, and no design makes the set of English sentences enumerable. Her method applies to the engine and stops at the corpus. Phient should be clear about which is which.

Where we disagree outright is on the goto. She forbids it. The three operators, better, worse and loss, were built as a forcing choice that jumps the decoder out of its greedy path, and on text the jump costs perplexity at every weight tried. That looks like her being right. We keep the operators anyway, because the chess program they come from used them to reach checkmate, and text has no checkmate. Whether that is a disagreement or an unfinished experiment is not settled.

## 14. The person and the moment

Barbara Liskov was born Barbara Huberman in Los Angeles on November 7, 1939. She took a bachelor's degree in mathematics with a minor in physics at the University of California, Berkeley, in 1961. She then worked for a year at the MITRE Corporation and for a time at Harvard on language translation before going to Stanford, where she completed her PhD in March 1968 under John McCarthy. The thesis was a program to play chess endgames, and it is the source of the two comparison functions, better and worse, and of the killer heuristic.

After the PhD she returned to MITRE as research staff. There she built the Venus computer, a microprogrammed Interdata 3, and the Venus operating system, a small time-sharing system with sixteen teletypes and a virtual machine for each user. The design of Venus appeared in Communications of the ACM in March 1972. The Turing Award biography says that in 1971, shortly after finishing her experiments with Venus and presenting a conference paper on the topic, another attendee urged her to consider a position at MIT. She joined the MIT faculty in what became the Laboratory for Computer Science.

The methodology paper was presented at the AFIPS Fall Joint Computer Conference in December 1972 and printed in volume 41, part 1 of its proceedings. So it sits at the seam. It was written out of MITRE experience and read from an MIT chair. Her own account, in "A History of CLU," is that by the fall of 1972 she had become dissatisfied with programming methodology papers, her own included, because readers could not apply them, and that from the latter part of 1972 through the summer of 1973 she worked out data abstraction instead. The paper on abstract data types followed in 1974 and CLU after it. The 1972 paper was the last word of one approach and the first premise of the next.

What the paper carries of the moment is the software crisis. Its opening sentence, as relayed, says that the standard approach of extensive debugging has not produced reliable software. That was the diagnosis of the 1968 and 1969 NATO conferences, and Dijkstra's levels of abstraction, Parnas's information hiding and Randell's work on structure were the responses she cites and groups her own with.

 She received the IEEE John von Neumann Medal in 2004 and the ACM A.M. Turing Award for 2008, cited for contributions to the practical and theoretical foundations of programming language and system design, especially related to data abstraction, fault tolerance and distributed computing. She is an Institute Professor at MIT.

## 15. What remains

What remains is the design she would say we have not done.

The channels have no names. Every word has 32–64 of them, each a byte, and training decides what each carries. Her first phase asks the designer to name the abstractions. Phient has named five kinds in the quest and four bases in the helix, and has not named a single channel. The partitioned-tie experiment in section 12 is the first step, and it has not been run.

There are no private channels. Every channel of every word is read by the operator. Her partitions have internal functions that no other level may call. Whether a word should hold relations that no pair-motion exposes is a question the paradigm sentence does not answer, since it defines a word as what it relates to, and a private relation relates to nothing outside. We record the question and take no position.

The direction rule has not been tested. Joint training shapes low degrees by high ones. Frozen-level training would tell whether the hierarchy the sphere tie appears to have is a hierarchy in her sense. Not run.

The byte deadband is a global variable. It was fixed in one partition, the quest, by dither, and the fix cost another partition, attribution, six points. Her method says the resource should be owned by one partition. Which partition should own the quantum, and whether it should be a per-channel or per-word resource rather than a global one, is not decided.

The user guide cannot be written. Her completion test is that a basic user guide could be written. A guide to the operator can say how to run it and what it scores. It cannot say what channel 17 means. Until it can, by her test, the design is not complete. We think that is the right test and that Phient does not pass it.

The three operators do not beat greedy on text. Her structured-programming rule says that failure to structure means the abstraction is not understood. The forcing results at λ = 0.05, 0.25 and 1 say the same. What better and worse mean for a sentence, as opposed to a chess position, is the open question of this whole series, and this paper does not close it.

Finally, the paper itself. We did not reach it. Every quotation above is as relayed by a secondary source or by Liskov's own later citation of page 195. The page range in the proceedings is not confirmed here. A reading of the original should be done before the series is finished, and if it changes anything in sections 2 to 4, those sections should be corrected.

## 16. Sources

Reached and read.

Liskov, B., "A History of CLU," MIT Laboratory for Computer Science, Technical Report MIT-LCS-TR-561, at publications.csail.mit.edu/lcs/pubs/pdf/MIT-LCS-TR-561.pdf. Source for her own quotation of the 1972 paper at page 195, for partitions coming from Dijkstra's levels of abstraction and from Venus, for Venus as a collection of partitions, for the bibliography entries for Liskov 1972a and 1972b, for the grouping with Parnas and Randell, and for her dissatisfaction with methodology papers by fall 1972.

ACM A.M. Turing Award, Barbara Liskov bibliography, amturing.acm.org/bib/liskov_1108679.cfm. Source for the venue, Proceedings of the AFIPS Fall Joint Computer Conference, Vol. 41, Part 1, December 1972, and for the 1977 reprint in the IEEE Tutorial on Software Design Techniques, Freeman and Wasserman, pp. 53–61, and for the Venus paper in CACM 15:3, 1972, pp. 144–149.

ACM A.M. Turing Award, Barbara Liskov biography, amturing.acm.org/award_winners/liskov_1108679.cfm. Source for MITRE from 1968, the Interdata 3, the Venus description, the sixteen teletypes, the 1971 conversation that led to MIT, CLU, Argus, and the award citation.

Wikipedia, "Barbara Liskov." Source for birth date and place, Berkeley 1961, Stanford PhD March 1968 under McCarthy, the chess endgame thesis, the Harvard job, and the award list.

Adrian Colyer, "A design methodology for reliable software systems," the morning paper, 21 October 2016. Source for the quotations on extensive debugging, system design determining test cases, ad hoc modularization, the two level rules, "nothing more," and the structured-programming rule.

Shubheksha, "A Design Methodology For Reliable Software Systems: An Academic Article Summary," freeCodeCamp and readings.shubheksha.com. Source for the reliability definition, the two testing conditions, the modularity failures, the connection guidelines, the design phases, the completion criteria, and the list of five kinds of abstraction.

ryotaro.dev, "A design methodology for reliable software systems (1972)." Source for the two rules of structured programming and the explicit-argument rule.

About Computing Systems, "Paper Pursuit #7." Confirmed the two-phase process.

Not reached. The paper itself at dl.acm.org/doi/10.1145/1479992.1480018, both the abstract page and the PDF, returned 403. The Semantic Scholar entry returned no content. The Wayback Machine copy of a former valbonne-consulting.com PDF and dblp could not be fetched from this session. The Venus paper's text at dl.acm.org was not attempted after the first refusal. No DTIC copy was found.

A note on the quotations. The secondary sources quote the paper in fragments and sometimes paraphrase it. Where two give the same sentence we treat it as verbatim. Where only one gives it, we have said "as relayed." The one quotation whose wording and page we trust fully is the one Liskov gives of herself in the CLU history. The page range of the paper in the proceedings is not stated in any source we reached, and this page does not state it either.
