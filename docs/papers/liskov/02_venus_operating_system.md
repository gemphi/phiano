# A level is a condition: the Venus machine and the layered word

*Phient interpreted papers · Liskov series, 2 of 16 · 2026-01-26 · on: "The Design of the Venus Operating System" (Barbara H. Liskov, 1972, Communications of the ACM 15(3):144–149; earlier version at the Third ACM Symposium on Operating Systems Principles, 1971)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the second of sixteen papers in which Phient reads one work by Barbara Liskov through its own paradigm. The first read her 1968 thesis, the chess endgame program with a forcing tree, better and worse, and no evaluation number. This one reads the work she did next, at the MITRE Corporation, between the thesis and MIT. Venus was a small time-sharing system. Its paper is six pages long. Its claim is that the shape of the machine underneath an operating system decides how complicated the operating system has to be, and that a machine can be shaped on purpose, by microprogram, so that the software above it becomes simple.

We read it here for one reason. Venus is built as levels of abstraction, and in each level a name means something only because of what the level below it provides. A segment name at one level is a number the paging system hands out. The same segment, one level up, has a name a programmer chose, and a dictionary relates the two. That is the paradigm sentence at the top of this page written as an operating system. A word is what it relates to under a condition, and the condition is the level.

Sections 2 to 5 state what the paper says, in its own terms, using only what we could verify. The full text sits behind the ACM paywall and we could not read it. We read its abstract, the Turing Award biography's account of the system, Liskov's own account in her history of CLU, and two secondary descriptions; Section 16 says which is which. Sections 6 to 10 place Venus inside the Phient family: the level as the condition c, the microprogram as phinet's fixed lower layer, the virtual machine per user as a condition that confines error, the semaphore as an operator with a count, and the partition with hidden state as the word-as-function. Section 11 lists what Phient has measured that bears on any of this, all from the facts file, none of it an operating system. Section 12 names one experiment that would test her claim inside phinet, with its control. Section 13 says where the analogy breaks. Section 14 is about the person and the years around 1971. Section 15 is what is left.

One limit is stated up front. Phient has no operating system, no microprogram, and no measurement of software complexity. What it has is a stack of layers, phinet under phiano under phiecs, and a fixed byte-per-channel floor that behaves like a microprogrammed machine: it sets what the layers above can say. Every number below was measured for another purpose.

## 2. What the paper claims, in its own terms

The abstract, as the ACM Digital Library and the 2001 reprint give it, is short and we quote it in full. "The Venus Operating System is an experimental multiprogramming system which supports five or six concurrent users on a small computer. The system was produced to test the effect of machine architecture on complexity of software. The system is defined by a combination of microprograms and software. The microprogram defines a machine with some unusual architectural features; the software exploits these features to define the operating system as simply as possible."

Four claims are packed into those sentences, and the paper's title names the fifth. First, Venus is an experiment. It was produced to test something, and the thing tested is the effect of machine architecture on the complexity of software. This is not the usual operating systems claim of the period, which was about throughput, users served, or memory used. Second, the object of the experiment is a machine that does not exist in hardware. The Interdata 3 computer Venus ran on could have its instruction set changed by microcode, and the Turing Award biography says she used this to create what it calls the "Venus Computer", a machine "tailored to supporting the construction of complex software". Third, the operating system is the software that sits on that machine and uses its features. The abstract's phrase is exact: the software "exploits these features to define the operating system as simply as possible". The measure of success is simplicity, not speed. Fourth, the scale is small. Five or six concurrent users. The biography says the system supported 16 teletypes. Fifth, the paper is about design, and the design is layered.

Liskov's own later description, in her history of CLU, says what the layers were made of. Venus "was organized as a collection of partitions, each with externally accessible functions and hidden state information, which communicated by calling one another's functions", and each partition "represents one level of abstraction". She credits Dijkstra's ideas about levels of abstraction as the source, and credits her own work on Venus as the basis for the design methodology paper she wrote the same year.

Per Brinch Hansen, who reprinted the paper in his 2001 collection of classic operating systems, gives the outside view. "The Venus system was a small timesharing system serving five or six users at a time." And: "Although it broke no new ground, the Venus system was another convincing demonstration of Dijkstra's concepts of semaphores and layers of abstraction." We take both halves of that sentence seriously. The paper is not a first. It is a demonstration, and its interest is in what it demonstrated and how carefully.

## 3. The mechanism, plainly

The full text was not available to us, so the description here is assembled from the abstract, the Turing biography, Liskov's CLU history, Andrew Herbert's 2015 history of virtualisation, and one secondary summary of the paper's contents whose provenance we cannot fully vouch for. Where a detail rests only on that summary we say so.

The mechanism has two halves. The lower half is microprogram. On the Interdata 3 the instruction set was defined by microcode, and Herbert's account is that in Venus "the lowest levels of abstraction were implemented by microprogramming". The secondary summary lists what those lowest levels provided: segments, which are the unit of virtual memory; virtual machines, of which there were sixteen, each with its own address space; input and output channels; and semaphores in Dijkstra's sense, with the P and V operations and the process queues they manage implemented in microcode. The summary gives segment sizes of up to 64,000 bytes, 15-bit segment names, and 256-byte pages. It also describes a procedure call mechanism in which parameters and results are passed through a special segment used as a pushdown stack, with call and return moving execution between program segments. We have not confirmed those figures against the paper and we do not lean on them.

The upper half is software, and it is here that the level structure shows. The summary describes a dictionary level, which maps external segment names, the ones a programmer chooses, to internal names, the ones the paging system allocates. Above that it describes a queue level for communication between processes, and a set of device levels that it names controllers, drivers and requesters. Each of these is a partition in Liskov's sense: a set of externally accessible functions over state the level keeps to itself. A level uses only the levels below it.

The part of the mechanism that the Turing biography states directly, and that matters most for our reading, is the virtual machine. "Each user was connected to a virtual machine so that major errors would not compromise the entire system, only the virtual machine for that user." The virtual machine is a boundary the microprogram draws. A process that fails, fails inside it.

Put together, the mechanism is this. The microprogram makes a machine whose primitive objects are already the objects an operating system needs: segments, processes on virtual machines, semaphores. The software then has less to build, and what it builds it builds as levels, each of which is a small vocabulary defined in terms of the vocabulary below. The test the paper proposes is whether the software comes out simpler as a result.

## 4. What it demonstrated, as reported

The abstract says the system "supports five or six concurrent users on a small computer", and that is the one result we can quote from the paper in the paper's words. The ACM bibliography entry for the Turing Award adds only that the paper "describes the Venus operating system and some results of experiments with it". What those experiments were, and what they measured, we could not read.

The secondary summary reports an effort figure of about six person-years for the system's design and application, and a separate figure of two to three person-years on the machine side. It reports a conclusion that levels of abstraction proved a valuable tool for designing with clarity and precision, and a second conclusion that most errors in the system's interactions were minimised by rules about which level owned which resource, rules that were, in the summary's phrase, faithfully followed. We report these because they are the kind of result a paper about software complexity would contain, and because they fit the abstract, but we did not verify them against the text and a reader should treat them as unconfirmed.

What is confirmed is the shape of the demonstration rather than its numbers. Venus ran. It served users through teletypes. It was built on a microprogrammed machine whose instruction set had been chosen for the software above it. And it was built in levels, each a partition with hidden state and exported functions. Brinch Hansen's judgement, that it was "another convincing demonstration" of Dijkstra's semaphores and layers, is the judgement of someone who had built the RC 4000 system on similar ideas and was not easily impressed. He does not call it new. He calls it convincing.

The demonstration that mattered in Liskov's own account was not about Venus at all. In the history of CLU she says the Venus design was the basis for her 1972 paper "A Design Methodology for Reliable Software Systems", presented at the Fall Joint Computer Conference in December 1972. And she says that by fall 1972 she had come to see that methodology papers, her own included, were too abstract, and that the way forward was to notice that "many of the modules discussed in the papers on modularity were defining data types". That noticing became abstract data types and CLU. So the Venus paper demonstrated, to its author, that a partition with hidden state and exported functions is a thing worth naming, and the name she gave it two years later was a type.

The scale should be kept in view. Five or six users. Sixteen teletypes. The claim about complexity is a claim about a system of that size.

## 5. The one idea, restated

The one idea is that a level of abstraction is a machine. Not a metaphor for one. Each level defines a set of objects and a set of operations on them, and the level above it programs against those objects and operations exactly as software programs against hardware. The bottom level in Venus happened to be a real microprogram, so the idea was literal there. The point of the paper is that the same relation holds all the way up. The dictionary level is a machine whose instructions take external names. The queue level is a machine whose instructions send and receive. Nothing above a level knows how that level is built.

Restated once more, with the direction reversed: the meaning of an object depends on which level you are standing on. A segment name at the paging level is a 15-bit number that identifies a region of virtual memory. The same segment at the dictionary level is a name a person typed. There is no level-free fact about what the segment "is". There is what it relates to, and the relations differ by level.

That is the sentence Phient opens every paper with. A word is nothing but what it relates to, under what conditions. Venus says: an object is nothing but the operations available on it, at the level you are on. Liskov's condition is the level. Phical's condition is c. The next five sections make that identification carefully and then see how far it carries, because the identification is easy to state and the place where it breaks, the strictness of the level ordering, is the most interesting thing about it.

The idea has a consequence she drew later and we draw now. If an object is only its operations at a level, then two levels built differently but exporting the same operations are the same level to everything above. The paging system could be rewritten and the dictionary would not know. That is what makes the levels an experiment rather than a description: you can vary one and hold the rest fixed.

There is a second, smaller idea inside the first. The microprogram is the level that decides the vocabulary of every level above it, and it was chosen on purpose. The paper's experiment is whether choosing it well makes everything above it simple. Phient has a layer like that. It is the byte. Every channel of every word is one byte, 256 quanta of 2π, and that choice was made once, at the bottom, and every layer above lives with it. Section 9 returns to what it costs.

## 6. A word as a function, and the partition as a word

Phical's word is a function. It takes a relation channel and returns a complex phasor and a count, 32–64 times over. It has no hidden point. It has hidden state: the phasor array is what the word is made of and nobody outside reads it directly. What is read is the result of applying the word: where it sits on a channel, relative to another word, under a condition.

Liskov's partition, in her own words, has "externally accessible functions and hidden state information" and communicates with other partitions "by calling one another's functions". That is the shape of a Phical word. The hidden state is the phasor array. The exported functions are the relations. When phiano's operator asks what word follows this context on this channel, it calls the function a word exports and never touches the array by name.

The identification says what "hidden" means for a word. In Venus, a level hides its state so the level above cannot depend on how it is built, and so the level can be rebuilt without disturbing what is above. In Phical the phasor array is hidden in a weaker sense: nothing forbids reading it, but nothing above the word is written in terms of it. The trainer moves phasors, the operator reads cosines. The circle and sphere ties in the facts file are two different arrays under the same exported function. A tie that changes the representation from one weight per channel to one per spherical-harmonic degree, 8,360 parameters to 1,192, leaves the operator's question unchanged. That is a rebuilt level under a stable interface, and the facts file has its cost: 14.7× valid on the circle at 3 epochs, 12.8× on the sphere.

There is a difference to keep. A Venus partition is a level, and there are a handful. A Phical word is one of 18,496 in the helix, and all export the same function. Venus levels differ in what they export; words differ only in the values behind a shared signature. So the word is the object a level is made of. The level, in Phical, is the condition, and the next section takes that up.

What the partition view adds is a name for what the trainer must respect. If a word is a function with hidden state, a training step may change its state only if it preserves what the word exports, its relations. That is a specification, and Liskov's next decade was about writing such specifications down. Phient has none written. The nearest thing is the parity suite, 62 of 62 tests against the PyTorch reference, which fixes what phinet's tensor operations export. Nothing fixes what a word exports.

## 7. The relation channel as a coordinate, and the level as the condition c

A relation channel in Phical is a coordinate. A word has 32–64 of them, each a phasor on a closed circle. The quest names five: is-a, part-of, near, used-for, unlike. A word's position on the is-a channel is what it relates to under the condition "is-a". The condition selects the coordinate.

A Venus level is the same move with names instead of angles. The dictionary relates an external segment name to an internal one. The internal name is what the segment is under the paging condition. The external name is what it is under the programmer's condition. Two coordinates, one object, and a function, the dictionary, between them. In Phical the analogue of the dictionary is a channel, with no table: the is-a channel is the coordinate on which "dog" and "animal" are close, and the near channel is the one on which "dog" and "kennel" are close. One reads the angle on the right coordinate.

So a level of abstraction, in Phical terms, is a coordinate on which some relations are visible and others are not. At the paging level, a segment's only relation is to its pages. At the dictionary level, it has a name and the pages are invisible. The level does not hide by fiat. It hides because a coordinate reads one relation at a time.

The quest measured how well this holds. On the synthetic oracle, 256 words in families of 8 across five kinds, the attraction-only trainer with dithered rounding reaches 0.594 MRR against chance 0.021, within-kind 0.747. The number that bears on Venus is the wrong-channel reading: 0.015. Read a word on the wrong coordinate and you get chance. The channels are levels in Liskov's sense. The offset reading, 0.005, is the same fact from the other side.

Where the identification strains is in ordering. The dictionary level calls down into the paging level; the paging level does not call up. Phical's channels are not ordered. Is-a is not above part-of. They are peers on a circle. So a level is a coordinate, but Venus adds a partial order on coordinates that Phical does not have. The sphere tie is the closest Phient has come to giving channels an order: 64 channels as spherical harmonics Y_l^m with l < 8, so that degree ranks them coarse to fine. The anisotropic tie, one weight per (l, |m|), 36 slots, 4,776 parameters, treats degree as a level, and it beat the flat circle: 16.0× valid, 15.6× test, hit@10 0.254, against 14.7×, 14.6× at 8,360 parameters. That is one measurement in favour of ordering the coordinates. It is not a measurement of Venus's order, which is ownership, not resolution.

## 8. The operator as X ⇒ Y under c, and the semaphore as an operator with a count

The Phical operator takes X to Y under c. In phiano it maps a context to a next word on a channel, and the hinge, the softmax, and Liskov's three operators score how well Y follows X under c. A semaphore is an operator with a count.

Dijkstra's semaphore, which Venus implemented in microcode according to the secondary summary, is a pair: a count and a queue. P decrements the count, and if it goes negative the process waits. V increments it and wakes a waiter. The count is the whole state. A process does not know who else is waiting; it knows the number. Phical's relation is a phasor on a closed circle with a count. The count records how often a pair has been seen under a condition; the phasor records where. The two counts are different objects with the same role: one integer summarising the history of a shared thing.

The reading that carries is this. P and V are X ⇒ Y under c where c is the semaphore. A process X applies P and becomes either the running process Y or the waiting process Y′, and which depends only on the count. In phiano, a context X applies the operator on channel c and becomes the next word Y, and which word depends on the phasors and their counts. In both cases the operator is deterministic given the state, and the state is small.

What Venus adds, and Phient does not have, is blocking. A semaphore can say no, not yet. The operator in phiano always answers; if nothing fits well it returns the least bad. The nearest thing in the facts file is the forcing decoder, which scores each candidate by cos − λ·worse, where worse asks how far the best continuation after a word falls below the word's own fit. That operator can refuse a word because it leads nowhere. It did not beat greedy on text: nearest-k KN perplexity 66.9 against 69.7 at λ = 0.05, 68.5 at λ = 0.25, and 107.4 at λ = 1. The honest reading there was that text is not an endgame; every word has some continuation. In Venus terms, the count never goes negative on text, so P never blocks.

The semaphore's other role, mutual exclusion, has a Phical reading too. A level that owns a resource takes the semaphore before touching it, and the summary reports that ownership rules kept errors down. In the quest, ownership is the channel: each kind owns its coordinate, and the wrong-channel reading of 0.015 says the rule was followed. No semaphore enforces it. The geometry does, because a coordinate is only readable as itself.

## 9. Where the microprogram lands: phinet, the byte, and the helix

Venus is two halves. The microprogram defines the machine; the software defines the operating system on it. The experiment is whether a well-chosen lower half makes the upper half simple. Phient has the same two halves, and the join is the byte.

phinet is the parent engine: the Tensor [b, w, s] complex, the Tape autograd, the unitary FFT, the Fno, HRR, the 64 hexagram gates, and the sphere. That is the microprogram. phiano, the word application, is written against it and never below it. Facet, Trainer, generation, personas, quest ingest, and the helix are software on the phinet machine. phiecs, the entity-component world and the quest, is a further level up. When phiano wants a cosine on a channel it calls phinet. When phiecs wants a word scored it calls phiano. The calls go down, not up, and that is Liskov's structure.

The feature of the phinet machine that most resembles a microprogrammed choice is that a channel is one byte. 256 quanta of 2π. That is a decision at the bottom that every level above inherits. The facts file has its cost measured. The trainer's nudge is 0.06·sin d, and it falls under half a quantum once sin d < 0.2, about 11°. Nearest rounding froze every pair 8 quanta short of where the trainer wanted them. Dither fixes the quest, 0.082 MRR to 0.594, and costs corpus attribution, 71% to 65%, so dither ships off by default. This is the Venus experiment run in reverse. Liskov chose the machine to make the software simple. Phient chose the byte to make the helix small and exact, and the software above has had to grow a dither step, a deadband analysis, and a default switch. The machine's architecture set the complexity of the software. Her claim held. It held against us.

The helix is where the choice pays. The lexicon as a DNA-coded tree, 64 codons as base-4 ACGT of channel, hexagram, sphere degree and order, B-DNA geometry at 10.5 codons per turn and 0.34 nm rise, PhiTon wavelength, band and hex per codon: 18,496 words in 1.9 s, 75 MB, byte-exact round trip. Byte-exact is the phrase. The helix is possible because the machine is a byte machine, in the way that Venus's software was simple because its machine had segments and semaphores. And the engine is allocation and bandwidth bound at these sizes, a 1 MB operation in 2–5 ms, which is the other face of a small fixed word.

Venus had sixteen virtual machines. The helix has 64 codons. The numbers are unrelated. The role is the same: a small, fixed vocabulary at the bottom, chosen so that everything above can be said in it.

## 10. Where the virtual machine lands: the quest, the persona, and a condition that confines error

The Turing biography's sentence about Venus is the one we keep returning to. Each user "was connected to a virtual machine so that major errors would not compromise the entire system, only the virtual machine for that user". The virtual machine is a condition that confines error. In Phical a condition is c, and the question is whether Phient's conditions confine error the way hers did.

The quest is the cleanest case. Five kinds, five channels, and the wrong-channel reading of 0.015 says that an error on one kind stays on that kind. A word mistrained on part-of does not become wrong on is-a, because is-a is another coordinate. That is a virtual machine per kind. The held-out numbers show where the confinement stops helping: one hop 0.011, which is chance, and two hops is-a 0.095, used-for 0.093, with part-of, near and unlike at chance. A held-out word that the trainer never saw on a channel has no position there, and no level above can supply one. In Venus a virtual machine that was never loaded runs nothing. Same thing.

Personas are the second case, and here the confinement is softer. Three writers, Chesterton, Austen, Carroll, and held-out four-sentence paragraphs are attributed at 71% overall, 74, 65 and 86 by writer, against chance 33%. Dialogue replies are judged as their persona 44% by the sector reader and 72% by the Kneser-Ney panel. A persona is a condition on the whole operator, not a channel. It is more like a Venus user than a Venus level: a virtual machine with its own address space, running the same instruction set as the others. Its errors are not confined by geometry, and Austen's 65% measures that.

The three operators are the third case, and the one where Venus and the thesis meet. The quest scored by better, worse and loss and filed by hexagram gives 256 words on 61 of 64 hexagrams, mean better −0.008, mean worse −0.008, mean loss 0.108 at margin 0.1. Flat. The truth ties the best wrong word on almost every row, and the byte ceiling is the reason. In Venus terms, every virtual machine reports the same load. The hexagram table is a set of 64 conditions and on this quest nothing distinguishes them. That is what a flat structure scores, and the corpus that is not flat, Phi-4's, has not been run.

What Venus adds to all three is the word "compromise". A virtual machine's job is to make an error local. Phient's conditions do that for channels, partly for personas, and for the hexagrams have not yet had anything to confine.

## 11. What Phient has already measured that bears on it

Nothing in gemphi is an operating system, and nothing was measured for Venus. The table lists what has been measured that a reader of Venus would want to know, with the facts file as the only source.

| Venus notion | Phient measurement | Value |
|---|---|---|
| The machine's exported behaviour is fixed | phinet vs PyTorch parity tests | 62/62 |
| Same interface, rebuilt level | circle hinge vs sphere hinge, Austen, 3 epochs | 14.7×/14.6× (8,360 params, 142 s) vs 12.8×/13.1× (1,192 params, 89 s) |
| Ordered coordinates | anisotropic tie, 36 slots, 4,776 params | 16.0×/15.6×, hit@10 0.254, 85 s |
| Error confined to a level | quest wrong-channel reading | 0.015 (chance 0.021) |
| Level with no content | quest held-out one hop | 0.011 |
| Cost of the fixed machine word | byte deadband | nudge under half a quantum below 11°; 8 quanta short |
| Software grown to cope | dither on / off | quest 0.082 → 0.594 MRR; attribution 71% → 65% |
| Small fixed vocabulary at the bottom | helix | 64 codons; 18,496 words, 1.9 s, 75 MB, byte-exact |
| Operator that can refuse | forcing decode, k = 32 | KN 66.9 greedy; 69.7 / 68.5 / 107.4 at λ = 0.05 / 0.25 / 1 |
| Conditions all reporting alike | quest by hexagram | 61 of 64 occupied; better −0.008, worse −0.008, loss 0.108 |

Three of these bear directly. The parity count is Phient's only written specification of what a level exports, and it covers phinet's tensor operations and nothing above them. The wrong-channel reading is Phient's only measurement of error confinement, and it is on a synthetic oracle of 256 words. The byte deadband is Phient's only measurement of what a machine-level choice costs the software above it, and it was found by accident, in the course of asking why the quest trainer had stalled.

Two things are absent and should be named. Phient has no measure of software complexity. Liskov's experiment was about how hard the operating system was to write, and Phient has never counted lines, modules, or person-hours for any layer, and the facts file forbids inventing them. And Phient has no measurement of what happens when a level is rebuilt while the system runs. The sphere tie replaced the circle tie between runs, not during one.

The Austen operator numbers, identity 7.9× chance, hinge 14.7×, softmax β=8 18.0×/18.8×, wide and deep 30.7×/30.3× at 8 epochs with hit@10 0.348, are the state of the software layer. They say nothing about Venus except that the layer works well enough to be worth confining.

## 12. The experiment: one level, two machines, with a control

Her experiment was to build the same software on two machines and see which was simpler. Phient cannot measure simpler, but it can measure the thing the software was simple for, which is being right. So the experiment is this. Take one level of the software, the quest trainer, and run it on two phinet machines that differ only in a bottom-level choice. Hold the software fixed. Measure what the choice costs above.

The choice is the byte. The facts file says a channel is one byte, 256 quanta, and that the deadband under 11° is what froze the trainer. The experiment builds a second phinet machine in which a channel is two bytes, 65,536 quanta, and everything else, the tie, the hinge, the nudge, the rounding, is identical. Then it runs the shipped quest trainer, attraction-only with dithered rounding, on both, on the same synthetic oracle: 256 words, families of 8, five kinds, 1,280 rows, 293 targets.

The control is the shipped machine with dither on, at its recorded 0.594 MRR, hit@1 34.8%, hit@5 97.4%, within-kind 0.747, wrong channel 0.015. The arm is the two-byte machine with dither off. Her claim predicts that the arm matches or beats the control without dither, because the software step that dither is, the grown coping step, exists only because of the machine. If the arm needs dither too, the byte was not the reason and her claim does not hold here.

The second measurement is attribution. Dither cost 71% to 65% on the three-writer test. Run persona attribution on the two-byte machine with dither off. If it recovers 71% while the quest holds 0.594, the machine-level change has removed a software-level tradeoff, which is her claim exactly.

The third measurement is the helix. It is 75 MB and byte-exact at one byte per channel. At two bytes it is presumably 150 MB and the round trip must be re-verified; the 64 codons as base-4 of a byte become something else. This is the part of her experiment that measures complexity, in the only unit Phient has: does the layer above still work unchanged, or does it need rewriting. If the helix must be redesigned, the two-byte machine has moved complexity upward, not removed it.

What would count against the idea: the arm needs dither, or attribution stays at 65%, or the helix breaks. Any one says the byte is not the lowest level that matters, and that the thing shaping the software is elsewhere, most likely the trainer's nudge rule, which would then be the real microprogram. The experiment says nothing about users, semaphores, or a system running while used. It is a static test. Section 15 has the dynamic one.

## 13. Where we disagree, and where the analogy breaks

The analogy breaks in four places. The first is the largest.

Venus levels are ordered and Phical channels are not. A level calls down and never up. The dictionary uses the paging system; the paging system does not know the dictionary exists. That order is what makes a level a level and what let her say a partition can be replaced without disturbing what is above. Phical's channels are 32–64 coordinates on a closed circle, and is-a does not call part-of. They are peers. The sphere tie gives them a degree, and the anisotropic tie showed a trainer that respects degree does better, but degree is resolution, not ownership. So when Section 7 says a level is a coordinate, it says half of what she said. The other half, that levels form a hierarchy with a direction, Phient has in its crate structure, phiecs over phiano over phinet, and nowhere in its words.

The second break is that Venus is deterministic and Phical is not. A semaphore's count is exact. P either blocks or does not. A Phical phasor is trained, and its position is a statistic over a corpus. The wrong-channel reading of 0.015 is a mean on a synthetic oracle built to be separable. Venus's confinement was a guarantee the microprogram enforced. Phient's is a measurement that could go the other way on Phi-4.

The third break is the measure of success. She measured complexity of software. Phient measures MRR, perplexity, hit@k and attribution. Section 12 substitutes "does the layer above still work" for complexity. That substitution is ours. A system can be simple and wrong. Her experiment could have found that a well-chosen machine made a wrong operating system easy to write, and nothing in Phient's numbers would detect that case.

The fourth is scale and purpose. Venus served five or six users in 1971, and Brinch Hansen's view is that it broke no new ground. Dijkstra's THE system, which she credits, is where the level structure came from. What Venus added was the microprogram. Phient's byte is a fair analogue, but Phient did not choose the byte to make the software simple. It chose it to make the helix small. That her claim then held is a finding about her claim, not a design that followed it.

Where we disagree is small. Her paper treats the machine as a means to simple software. Phient treats the lowest layer as a fact the software must read, more like a corpus than a design. A byte per channel is not a good machine for training; it is the machine the helix needs. She would have redesigned it.

## 14. The person and the moment

The facts here are the verified ones only. Barbara Liskov was born Barbara Jane Huberman in Los Angeles on 7 November 1939. She took a bachelor's degree in mathematics with a minor in physics at Berkeley in 1961. After Berkeley she moved to Boston and worked for a year at the MITRE Corporation, where she became interested in computers and programming, then worked at Harvard on language translation, then went to Stanford. In March 1968 she received her PhD from Stanford's computer science department, one of the first women in the United States to do so, supervised by John McCarthy. The thesis, "A Program to Play Chess End Games", Stanford technical report CS-106, is the work the first paper in this series read. The Chess Programming Wiki records that the program could solve KQK, KRK, KBBK, KBNK and KNNNK, and that it introduced the killer heuristic.

After the PhD she married Nathan Liskov and returned to the Boston area, to MITRE in Bedford, Massachusetts, to work on computer design and operating systems. The Turing Award biography gives her MITRE years as 1968 to 1972. Venus is the work of those years. The biography's account is that she used an Interdata 3, whose instruction set could be changed by microcode, to create a machine tailored to the software she wanted to build on it, and that the resulting system supported sixteen teletypes with each user on a virtual machine.

The moment is 1971. The paper was presented at the Third ACM Symposium on Operating Systems Principles that year, and the journal version appeared in Communications of the ACM in March 1972. The biography says that at the 1971 conference, shortly after finishing her Venus experiments, another attendee urged her to consider a position at MIT, and that she left MITRE and joined the MIT faculty in the Laboratory for Computer Science. December 1972 saw her methodology paper at the Fall Joint Computer Conference, which by her own account was built on Venus. By fall 1972, again by her own account, she had decided that methodology papers were too abstract and had noticed that the modules everyone was describing were types. CLU followed.

What we take from the moment is the sequence. A thesis with no evaluation number, only better and worse. Then a system whose test was whether the machine could be chosen to make the software simple. Then a methodology paper, then the recognition that the units of the methodology were data types, then a language. Each step abstracts the previous one. Venus is the step where the abstraction was still made of microcode and segments, and where the question was still about a machine.

## 15. What remains

The first thing that remains is to read the paper. Six pages in Communications of the ACM, March 1972, pages 144 to 149, reprinted in Brinch Hansen's 2001 collection at pages 282 to 294. Everything in Sections 3 and 4 that rests on the secondary summary should be checked against it and confirmed or struck, and the revision should say what changed.

The second is the experiment of Section 12, which is static. The dynamic version is closer to what she did. Venus ran while it was used, and its levels were exercised by five or six people at once. Phient's nearest thing to a running system is quest ingest feeding phiecs while phiano decodes, and nothing has been measured about what happens when the helix is read by one process while the trainer writes it. A byte-exact round trip is a guarantee about a file at rest. Whether a level can be rebuilt under a running level is the question Venus was built to answer and Phient has not asked.

The third is specification. Section 6 said that a word is a partition with hidden state and exported functions, and that Phient has no written statement of what a word exports. The parity suite fixes phinet's tensor operations at 62 of 62. Nothing fixes phiano's word. Writing that down, as tests a tie must pass to be a tie regardless of representation, is what her next paper in this series is about. Without it the circle and sphere ties are two things; with it they are one level rebuilt.

The fourth is ordering. Channels are peers and levels are not. The anisotropic tie's 16.0× against the circle's 14.7× at fewer than half the parameters is one number in favour of giving channels an order. Whether that order should be ownership, as in Venus, so that some channels are defined in terms of others, is open. Nothing has been built.

The fifth is the Phi-4 run. The hexagram table is flat on the synthetic quest and the facts file says the corpus that is not flat has not been run. Every confinement claim in Section 10 is a claim about 256 words built to be separable. Venus served real users. Until the quest has read a real corpus, the virtual machine reading is a reading of a test bench.

The last is the honest one. Phient has no measure of software complexity, and her paper is about nothing else. Until it has one, every claim here that her idea "held" is a claim that a machine-level choice shaped the accuracy of the software above it. That is related to her claim. It is not her claim.

## 16. Sources

Sources reached and read, with what each supplied.

The ACM Digital Library entry for the paper (dl.acm.org/doi/10.1145/361268.361272) and the Communications of the ACM issue page for March 1972 (cacm.acm.org, vol. 15 no. 3) supplied the abstract, which Section 2 quotes in full, and the bibliographic details. The full text returned a 403 error at every attempt, through both the article page and the PDF link, and we did not read it. The SIGOPS Operating Systems Review and Third SOSP entries (dl.acm.org/doi/10.1145/850614.850617 and 10.1145/800212.806493) confirmed the 1971 conference version exists; their pages were also not readable.

The Springer entry for the reprint in Per Brinch Hansen, editor, Classic Operating Systems: From Batch Processing to Distributed Systems, 2001, pages 282–294, supplied the abstract again and the reprint's page numbers.

The ACM A.M. Turing Award page for Barbara Liskov (amturing.acm.org/award_winners/liskov_1108679.cfm) supplied the Interdata 3, the microcoded instruction set, the "Venus Computer" phrase, the sixteen teletypes, the virtual machine per user, the MITRE years, the marriage, and the 1971 conference and move to MIT. Its bibliography page supplied the journal citation and the FJCC 1972 citation.

Barbara Liskov, "A History of CLU", MIT Laboratory for Computer Science technical report MIT-LCS-TR-561 (publications.csail.mit.edu), supplied her own description of Venus as partitions with hidden state and exported functions, the credit to Dijkstra, the link to the 1972 methodology paper, and the fall 1972 turn to data types.

Per Brinch Hansen, "The Evolution of Operating Systems", 2000 (the cs.ucf.edu copy), supplied the "five or six users" and "broke no new ground" judgements.

Andrew Herbert, "Notes on Virtualization", version 9.0, 2015 (sigops.org SOSP history), supplied the statement that Venus's lowest levels were implemented by microprogramming.

The Chess Programming Wiki page on Barbara Liskov and the Wikipedia article supplied the biographical dates in Section 14 and the thesis title and report number.

One secondary source, a Chinese-language blog summary of the paper (cnblogs.com/icecri), supplied the segment sizes, page size, sixteen virtual machines, the microcoded semaphores, the pushdown-stack segment, the dictionary and queue levels, the device levels, the person-year figures and the conclusions about ownership rules. We could not verify any of it against the paper, and every use of it above is marked as unverified.

Sources not reached: Semantic Scholar and dblp returned no content or were disallowed; the Crossref and Semantic Scholar APIs were refused by the proxy; the ACM full text was refused. No DTIC or MITRE document on Venus was found.

Phient numbers: PHIENT_FACTS.md in this folder, and docs/86_liskov_three_operators.md for the three operators.
