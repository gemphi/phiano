# The stable byte: a word as a guardian, a comparison as an action that commits or aborts

*Phient interpreted papers · Liskov series, 9 of 16 · 2026-05-04 · on: "Guardians and Actions: Linguistic Support for Robust, Distributed Programs" (Barbara Liskov and Robert Scheifler, ACM Transactions on Programming Languages and Systems 5(3), pp. 381–404, July 1983)*

"In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition."

## 1. Our abstract

This is the ninth of sixteen papers in which Phient reads one work by Barbara Liskov through its own paradigm. The first eight followed her from the chess endgame to the CLU Reference Manual. This one leaves the single machine. In 1983 Liskov and Robert Scheifler published, in the ACM Transactions on Programming Languages and Systems, an overview of a language and system called Argus, built on CLU, for programs whose modules run at distinct nodes of a network. The paper has two concepts and it says so in its summary: guardians and actions. A guardian is a module that lives at one node, owns some data, and can be reached only through operations called handlers. It has stable state, written to stable storage, and volatile state, which a crash destroys and a recover section rebuilds. An action is an atomic activity: indivisible with respect to other actions, and recoverable, so that its effect is all or nothing. Actions nest. A handler call is a subaction. A subaction that fails aborts without aborting its parent.

This paper reads those two concepts into Phient. The reading is this. A word is a guardian. Its stable state is the byte on each of its 64 channels and the count beside it, which the helix writes to disk and reads back byte-exact. Its volatile state is everything recomputed from the bytes: the operator's cosines, the hexagram sector, the tape. Its handlers are better, worse and loss. A comparison under a condition is an action. It reads two words, holds them, and either commits or aborts. The byte deadband is where the reading becomes a measurement. A nudge below half a quantum is a version that never commits. Nearest rounding froze every pair 8 quanta short because the volatile version was thrown away every time. Dithered rounding is a commit rule, and it took the quest from 0.082 MRR to 0.594.

The reading also gives forcing a name. forcing_choice appends each candidate to the context, applies the operator once more, and re-ranks by cos − λ·worse. In Argus terms that is a coenter over 2k subactions, and worse is the abort test. On text it does not beat greedy at any λ tried. The interpretation says why in the paper's own terms, and section 12 says where the test should be run instead. Section 13 says where the analogy breaks, and it breaks in the middle: Argus wants serializability, and Phient's meaning is a side effect of pairs moving at once. Every number here is from the facts file. Section 16 lists what was reached, which includes the paper itself, and what was not.

## 2. What the work claims, in its terms

The paper opens with a need. "Technological advances have made it cost effective to construct large systems from collections of computers connected via networks. To support such systems, there is a growing need for effective ways to organize and maintain distributed programs: programs in which modules reside and execute at communicating, but geographically distinct, locations. In this paper we present an overview of an integrated programming language and system, called ARGUS, that was designed for this purpose." The abstract says the same in one sentence.

The claims are about what such a language must provide. "A major concern is to provide continuous service of the system as a whole in the face of node and network failures." Distribution also serves efficiency, "in terms of both responsiveness and cost-effective use of hardware." And the requirement the whole design turns on: "consistency is a crucial requirement: an adequate language must provide a modular, reasonably automatic method for achieving consistency."

The answer to consistency is atomicity. "Two properties distinguish an activity as being atomic: indivisibility and recoverability." "By indivisibility we mean that the execution of one activity never appears to overlap (or contain) the execution of any other activity." "By recoverability we mean that the overall effect of the activity is all-or-nothing: either all of the objects remain in their initial state, or all change to their final state." Atomic activities are called actions.

The answer to modularity is the guardian. "A guardian encapsulates and controls access to one or more resources, for example, databases or devices." "A guardian makes these resources available to its users by providing a set of operations called handlers." "A guardian's state consists of stable and volatile objects."

The paper is explicit that atomicity is a restriction. "The objects that do provide these properties are called atomic objects, and we restrict our notion of atomicity to cover only access to atomic objects." And it is explicit about what atomicity cannot do. "Not all effects of an action can be undone by aborting that action, since a change to the external environment, for example, printing a check, cannot be undone by program control alone." "The commit of a top-level action is irrevocable."

The summary restates the two concepts: "ARGUS has two main concepts: guardians and actions. Guardians maintain local control over their local data." "While guardians are the unit of modularity, actions are the means by which distributed computation takes place." The last position is a claim about a class: "atomic actions are necessary and are a natural model for a large class of applications."

## 3. The mechanism, plainly

Take the parts in the paper's order.

An atomic object is implemented with locks and versions. "There are two kinds of locks: read locks and write locks." "All locks acquired by an action are held until the completion of that action, a simplification of standard two-phase locking." Writing makes a copy. "When a write lock is obtained, a version of the object is made, and the action operates on this version. If, ultimately, the action commits, this version will be retained, and the old version discarded. If the action aborts, this version will be discarded, and the old version retained." Versions and locks "can be kept in volatile memory." Durability comes later: "we do not actually copy information to stable storage until actions commit." Commit across guardians uses "a standard two-phase commit protocol."

Argus provides atomic arrays, records and variants, CLU's with atomicity added, and "a mechanism for user-defined atomic data types."

Actions nest. "Nested actions, or subactions, are a mechanism for coping with failures, as well as for introducing concurrency within an action." "Subactions can commit and abort independently, and a subaction can abort without forcing its parent action to abort." "When a subaction commits, its locks are inherited by its parent; when a subaction aborts, its locks are discarded."

A remote call is a subaction. The paper holds that "it seems natural to implement a remote procedure call as a subaction: communication failures will force the subaction to abort, and the caller has the ability to abort the subaction on demand." A handler call runs in seven steps: a new subaction of the caller is created; a message with the arguments is built; the caller is suspended and the message sent; the system "makes a reasonable attempt to deliver the message"; a process and a subaction are created at the receiving guardian to run the handler; a results message is built; the caller continues. If delivery fails, "the subaction aborts, and the call terminates with a failure exception."

A guardian has stable variables, a recover section and a background section. "After a crash of the guardian's node, the language support system recreates the guardian with the stable objects as they were when last written to stable storage." "The recover section runs after a crash. Its job is to recreate a volatile state that is consistent with the stable state." Concurrency inside a guardian is the coenter statement: "Each coarm instance runs in a separate process, and each process executes within a new top-level action or subaction." Deadlock is left to the user: "we rely on the user to time out and abort top-level actions."

## 4. What it demonstrated, as reported

The paper demonstrates by example and reports on an implementation in three sentences.

The example is a mail system built from three kinds of guardian. "Mailers act as the front end of the mail system: all use of the system occurs through calls of mailer handlers." "A maildrop contains the mailboxes for some subset of users." "The mapping from user_id to maildrop is provided by the registries." "send_mail appends a message to the end of the message array." Sending to several users uses coenter: "The message is sent to all users simultaneously. A nonexistent user or a failure to send a message transfers control outside the coenter, forcing termination of all active coarms; the outer action is then aborted, guaranteeing that none of the messages is actually delivered."

The remarks that follow the example are frank and they are the paper's real report. A mailer depends on one registry, so "if the best registry for a mailer goes down, the mailer effectively goes down as well." Atomic arrays give less concurrency than wanted: since send_mail and read_mail both modify the message array, "either operation will lock out all other operations on the same mailbox until the executing action commits to the top level," and "an add user operation will lock out all operations on all mailboxes at that maildrop." The authors say what is needed: "data types that allow more concurrency than do atomic arrays." They report deadlocks: "any two concurrent add user or add registry requests will almost always deadlock." And they concede that "the particular granularity of actions we have chosen may not be the best," and that eventual consistency among registries might suffice, by "a background process in that registry to (eventually) inform all other registries."

On implementation: "We have completed a preliminary, centralized implementation of the language, ignoring difficult problems such as lock propagation and orphan detection. We are working on a real, distributed implementation. At this point it is unclear how efficient such an implementation can be." Orphans are named and deferred: aborts "can result in orphan handler processes that continue to run at the called guardians and elsewhere. We have developed algorithms for dealing with orphans, but they are beyond the scope of this paper."

So what the paper demonstrated in 1983 is a design, one worked example, and a list of the example's faults written by its own authors. No timing, no throughput, no distributed run. Wikipedia dates the Argus effort 1982 to 1988 and cites Liskov's 1988 CACM article "Distributed Programming in Argus," 31(3), 300–312, which I did not reach.

## 5. The one idea restated

A module owns its data, lives in one place, and can be reached only through named operations. A computation that touches several modules either happens entirely or not at all, and it may be built from smaller computations that each have the same property.

That is the pair, and the paper is careful that neither half works alone. Guardians without actions give locality and encapsulation but no consistency: two handler calls could each succeed and leave the pair of guardians in a state neither meant. Actions without guardians give consistency but no locality: nothing says where data lives or who may touch it. Together they say that every piece of distributed state has one home, that the home is reached through handlers, and that a handler call is a subaction of whatever called it. Consistency follows from composition. "A top-level action starts at some guardian. This action can perform a distributed computation by making handler calls to other guardians; those handler calls can make calls to still more guardians; and so on." "Since the entire computation is an atomic action, it is guaranteed that the computation is based on a consistent distributed state and that, when the computation finishes, the state is still consistent, assuming in both cases that user programs are correct."

The versions are the mechanism that makes the idea cheap. A write does not touch the object. It makes a copy, works on the copy, and only at commit does the copy become the object. Until then the copy lives in volatile memory. Stable storage is written once, at commit, and never for an aborted action. This is why a guardian can crash and recover: the stable state is only ever a committed state, and the recover section rebuilds the rest. And it is why an action can abort for free: the versions are dropped and nothing else moved.

The paper adds one qualification and it matters for Phient. Atomicity is enforced only on atomic objects, and the authors say plainly that atomic arrays serialize more than a mail system wants. The idea is therefore a choice of granularity, and the right granularity is a property of the data type, which the user may define.

Restated for Phient: a word is a guardian whose stable state is its bytes and counts; a comparison under a condition is an action that reads two words and either commits a change to a byte or leaves it; a decode or a training epoch is a top-level action made of subactions, each of which may abort without aborting the whole. Sections 6 to 10 take this apart.

## 6. The word as a function, and the guardian

In Phical a word is 32–64 relations, each a phasor with a count, and it means nothing except through those relations under a condition. Argus has a name for this shape and it is the guardian.

Take the definition in pieces. "A guardian encapsulates and controls access to one or more resources." The resource is the word's 64 channels. "A guardian makes these resources available to its users by providing a set of operations called handlers." The handlers are three: better(pred, a, b), worse(pred, a, after, continuations), loss(pred, target, decoy, margin). "Guardians must communicate by calling handlers, and the arguments to handlers are passed by value." The operator does not hand a word's phasors to another word. It computes a cosine and returns a number. That is by-value passing: no word's state is shared through the call.

"A guardian's state consists of stable and volatile objects." This is the part of the reading that does work. The stable state of a word is the byte on each channel, 256 quanta of 2π, and the count. The helix writes exactly that: 64 codons, base-4 ACGT of the channel byte, with a hexagram, a sphere degree and order, and a PhiTon wavelength per codon; 18,496 words in 1.9 s, 75 MB, byte-exact round trip. The volatile state is everything else: the floating phasor the tape holds during an epoch, the cosines the operator computes, the centroid phase that files a word under one of 64 hexagrams, the spherical-harmonic weights of a tie. Every one of these is recomputed from the bytes.

"After a crash of the guardian's node, the language support system recreates the guardian with the stable objects as they were when last written to stable storage." "The recover section runs after a crash. Its job is to recreate a volatile state that is consistent with the stable state." The helix round trip is the recover section. It reads the codons back and rebuilds the word, and the test that it is the same guardian is that the bytes match. Whether the rebuilt volatile state is consistent, in the paper's sense, the round trip does not say, because it checks the stable half only. Section 15 lists it.

"A guardian runs at a single node but can survive crashes of this node." A word in phinet lives in one Tensor [b,w,s], at one index w. It is not replicated.

The reading gives Phient one thing it had not said. A word has two states, and only one of them is the word. The byte is the word. The float is a version.

## 7. The relation channel as a coordinate, and the node

"Guardians are created dynamically. The programmer specifies the node at which a guardian is to be created." "Guardians allow a programmer to decompose a problem into units of tightly coupled processing and data." A node in Argus is a place. Data there is local, and anything from elsewhere arrives as a value through a handler call.

A Phient channel is a place in this sense. The condition c picks a channel, and the relation of X to Y under c lives on that channel and nowhere else. The quest oracle has five kinds, is-a, part-of, near, used-for and unlike, and each kind is a node. A relation written on the is-a channel is data local to that node. Reading it from another channel is a handler call to the wrong guardian, and the paper says what happens then: "If that guardian no longer exists, the subaction aborts, and the call terminates with a failure exception."

The quest measures the failure exception. Within-kind MRR is 0.747. Wrong channel is 0.015. Offset reading is 0.005. Chance is 0.021. A read from the wrong node returns less than chance, which is the ranking's way of saying that the call did not find the guardian. In Argus the caller would get a named exception. In phinet it gets a number below chance and has to notice.

The coordinate reading also bears on the paper's phrase "tightly coupled processing and data." A channel is a place where a relation sits and where a specific operator acts on it. The circle tie has one weight per channel, 8,360 params on Austen. The sphere tie treats 64 = 8² channels as Y_l^m with l < 8 and has one complex weight per degree, 1,192 params. The anisotropic tie has one weight per (l, |m|), 36 slots, 4,776 params. These are three ways of grouping the nodes. The circle gives every node its own handler; the sphere makes nodes of the same degree share one; the anisotropic, nodes of the same degree and |m|. On Austen hinge at 3 epochs the circle gives 14.7× valid / 14.6× test, the sphere 12.8×/13.1×, the anisotropic 16.0×/15.6× at 4,776 params, beating the circle at 8,360. The grouping that shared processing among the right nodes did better with fewer resources.

There is a limit to the node reading. Argus nodes are geographically distinct, and the point of distribution in the paper is that a node can crash while others continue. Phient's 64 channels live in one tensor in one process. No channel has ever crashed alone, and none has been recovered while the others ran. The node reading names a structure and no fault model. Section 13 returns to this.

## 8. The operator as X ⇒ Y under c, and the action

X ⇒ Y under c is an action in the paper's sense. It examines the phasors of X and of candidate Ys on channel c, and attempts to transform the bytes that training moves. It is indivisible if no other comparison appears to overlap it, and it is recoverable if its effect is all or nothing.

The second property is the one Phient has measured without naming it. Recoverability in Argus is implemented with versions: a write makes a copy, the action works on the copy, commit retains the copy and abort discards it. Phient's training does this and the copy is the floating phasor. The tape nudges a channel by 0.06·sin d, where d is the phase difference under the condition. That nudge lives in volatile memory. The stable object is the byte. Commit is rounding to the byte. Under nearest rounding, a channel being one byte of 256 quanta, the nudge falls under half a quantum once sin d < 0.2, about 11 degrees, and below that the version is discarded every time. In Argus terms the action aborts, silently, on every such pair. Nearest rounding froze every pair 8 quanta short.

Dithered rounding changes the commit rule. It commits a version below half a quantum sometimes instead of never. On the quest, attraction-only with dithered rounding took the shipped trainer from 0.082 MRR to 0.594, hit@1 34.8%, hit@5 97.4%, against chance 0.021. What changed is when a version becomes the object. The paper would recognise this as a change in the atomic type: "What is needed are data types that allow more concurrency than do atomic arrays." Phient's byte is an atomic array of the strictest kind, and dither is the user-defined type the paper says should be allowed.

The first property, indivisibility, is where Phient departs, and section 13 takes it up. But one part lands. "All locks acquired by an action are held until the completion of that action." In the hinge, better(pred, target, decoy) reads target and decoy and holds both until loss, max(0, margin − better), is computed. The two reads are not interleaved with a write to either word. That is a read lock held to completion. Whether the order of pairs across batches matters has not been measured.

The paper's word for what the operator returns when it cannot commit is failure. Phient returns a number. The quest, scored by the three operators, gives mean better −0.008, mean worse −0.008, mean loss 0.108 at margin 0.1, across 256 words in 61 of 64 hexagrams. That is the failure exception raised on nearly every row and reported as a mean. Section 15 asks for it as a count.

## 9. Nested actions, and forcing

"Nested actions, or subactions, are a mechanism for coping with failures, as well as for introducing concurrency within an action." "A subaction can abort without forcing its parent action to abort."

forcing_choice is a coenter. The parent action is the choice of the next token given a context. It takes the 2k nearest words, appends each to the context, applies the operator once more, and re-ranks by cos − λ·worse. Each candidate is a coarm, and each coarm is a subaction of the choice. worse is how far the best continuation after a falls below a's own fit, and a candidate with no continuation is a subaction that would abort. At λ = 0 the abort is ignored and the choice is greedy.

In the mail example, sending to several users, "a nonexistent user or a failure to send a message transfers control outside the coenter, forcing termination of all active coarms; the outer action is then aborted." This is the endgame shape. In Liskov's 1968 program a move is refuted if some reply makes it worse, and most moves lead nowhere. Forcing is a coenter over candidate moves with the refutation as the abort.

The measurement says the shape does not fit text. decode_eval, 40 held-out prompts × 30 tokens, k = 32: operator nearest-k KN perplexity 66.9; forcing λ = 0.05 gives 69.7, λ = 0.25 gives 68.5, λ = 1 gives 107.4, with the evaluator rising to 0.712, distinct falling to 0.777 and shared 4-grams rising to 22.3% from the operator's 8.7%. Forcing does not beat greedy on text at any λ. In the paper's vocabulary, no coarm ever aborts. Every word in text has some continuation. So worse never returns a dead end, only a graded preference for words with strong continuations, and those are the frequent function words. A coenter in which no subaction can fail is a slower loop.

Section 4.1 of the paper has the sentence for this: "the particular granularity of actions we have chosen may not be the best." The place where subactions can abort is the quest, where a word either has onward relations under some kind or it does not. docs/86 names that as the place worse should bite. Section 12 makes it the experiment.

One more part of the nesting rule lands. "When a subaction commits, its locks are inherited by its parent; when a subaction aborts, its locks are discarded." When forcing accepts a candidate, the context grows and the next choice inherits it. When it rejects one, nothing of the rejected continuation remains. The decoder already does this.

## 10. The quest and the helix as a guardian system

The paper's worked example is three kinds of guardian in a mail system. Phient's quest is the nearest thing it has to such a system, and the helix is its stable storage.

The quest oracle in phiecs has 256 words in families of 8, related by five kinds, 1,280 rows, 293 target words. Each word is a guardian in the section 6 sense. Each kind is a node in the section 7 sense. A one-hop query is a handler call. Two hops is a handler that calls another, which is what the paper's summary describes: "those handler calls can make calls to still more guardians; and so on."

The held-out numbers read as the paper's failure semantics. Held out 20%: one hop 0.011 MRR against chance 0.021; two hops is-a 0.095, used-for 0.093, part-of, near and unlike about chance. A one-hop call to a relation the trainer never saw fails. A two-hop call through is-a or used-for succeeds a little. A two-hop call through the other three kinds is a subaction that aborts. In the mail system, that is the registry lookup failing before the maildrop is reached. Argus would report a failure at the registry; the quest reports a rank near chance. That the two composable kinds are is-a and used-for is a fact about this oracle and nothing more.

The hexagram table is the quest's registry. docs/86 files each word under the hexagram its 64-channel function is, the centroid phase in 64 sectors, and scores each about word by the three operators. The result is 256 words in 61 of 64 hexagrams, mean better −0.008, mean worse −0.008, mean loss 0.108 at margin 0.1. Flat. The paper's remarks have the diagnosis: an atomic array serializes everything. The byte is Phient's atomic array. Within the deadband every truth ties its best wrong word, so the registry has 61 entries that all say the same thing.

The helix is stable storage and it meets the paper's requirement without qualification. "We do not actually copy information to stable storage until actions commit." The helix writes bytes, and a byte is by construction a committed value: the float version is gone by the time the codon is written. 18,496 words in 1.9 s, 75 MB, byte-exact. The B-DNA geometry, 10.5 codons per turn and 0.34 nm rise, and the PhiTon wavelength per codon are volatile state a reader recomputes. The recover section, in the paper's phrase, "recreates a volatile state that is consistent with the stable state." The helix reader does the first half. Whether a word lands in the same hexagram after a round trip has not been checked and is not claimed.

## 11. What Phient has measured that bears on it

Everything here is from the facts file and docs/86.

| construct in the paper | what Phient measured | number |
|---|---|---|
| stable state, written at commit, recovered after a crash | helix round trip, 18,496 words | 1.9 s, 75 MB, byte-exact |
| versions that never commit (nearest rounding) | byte deadband | nudge under half a quantum once sin d < 0.2 (11°); every pair frozen 8 quanta short |
| a changed commit rule (dithered rounding) | quest MRR, shipped vs attraction-only + dither | 0.082 → 0.594; hit@1 34.8%, hit@5 97.4% (chance 0.021) |
| cost of the changed rule | corpus attribution with dither | 71% → 65%, so dither ships off |
| handler call to the wrong node | quest within-kind / wrong channel / offset | 0.747 / 0.015 / 0.005 |
| handler that calls a handler | quest held-out two hops | is-a 0.095, used-for 0.093, others ≈ chance; one hop 0.011 |
| coenter with worse as the abort test | decode_eval forcing, k = 32 | nearest-k 66.9; λ = 0.05 69.7; λ = 0.25 68.5; λ = 1 107.4 |
| failure reported as a mean | quest scored by three operators | better −0.008, worse −0.008, loss 0.108 (margin 0.1); 61 of 64 hexagrams |
| grouping nodes under one handler | Austen hinge, 3 epochs, valid / test | circle 14.7×/14.6× (8,360 params); sphere 12.8×/13.1× (1,192); anisotropic 16.0×/15.6× (4,776) |
| handlers held against a reference | phinet vs PyTorch | 62/62 parity tests |

The first three rows are the paper's version mechanism read straight: the helix writes the stable byte, the float nudge is the version, nearest rounding discards it below half a quantum, and dither commits it sometimes. The fourth row is the cost: dither takes persona attribution from 71% overall (74/65/86 for Chesterton, Austen and Carroll) to 65%, which is why it ships off. The wrong-node and two-hop rows are the failure semantics. The forcing row is the coenter that never aborts. The flat row is failure reported as a mean, and it is the measurement this reading most wants changed.

What is not measured is longer. Nothing about concurrency in phinet has been measured: no two training steps have run at once against the same word, so nothing is known about indivisibility, locks or serializability in Phient. No channel has been recovered while the others ran. No count of aborts has been taken, because no operator raises one. The engine is allocation and bandwidth bound at these sizes, 1 MB op in 2–5 ms, so no timing here says anything about the cost of atomicity. There is no Phi-4 run; the quest is a synthetic oracle only. The sphere at wide+deep is unrun.

## 12. The experiment

The paper's claim, reduced to one testable sentence: a computation built from subactions that can abort independently is more robust than the same computation run flat, and the gain appears where subactions actually fail. Phient can test the second half, because it has a collection where subactions fail and one where they do not.

The experiment is forcing at depth two on the quest. Setup, as the facts file states it: the phiecs synthetic oracle, 256 words in families of 8, five kinds, 1,280 rows, 293 targets, chance MRR 0.021. Trainer: attraction-only with dithered rounding, the arm at 0.594 MRR, hit@1 34.8%, hit@5 97.4%. That run is the control. Nothing in the trainer changes.

The treatment changes the reader. For each query (X, kind), take the 2k nearest candidates on the kind's channel, k = 32 as on text. For each candidate a, open a subaction: from a, under every kind, read the best onward relation, and compute worse(pred, a, after, continuations) as docs/86 defines it for the quest: the gap between a's fit and a's own strongest onward reading under any kind. A candidate with no onward relation under any kind is an aborted subaction. Re-rank by cos − λ·worse at λ in {0.05, 0.25, 1}, the three values already run on text, at depth two, so that the onward reading is itself checked once more.

Record MRR, hit@1 and hit@5 against 0.594, 34.8% and 97.4%, and record separately the fraction of candidates whose subaction aborted. On text that fraction was, in effect, zero. If it is also near zero on the quest, the experiment says nothing about forcing and something about the oracle.

The prediction is specific. If aborts are common and forcing at some λ beats 0.594, the nested-action reading holds: worse bites where subactions can fail. If aborts are common and forcing does not beat 0.594 at any λ, the reading is wrong, and the text result was no artefact of text. If aborts are rare, the oracle is too dense for the test.

A second arm tests the commit rule. Run the quest trainer with three rules for turning a float nudge into a byte: nearest rounding, dithered rounding, and a third rule that commits only when better on the training pair increased and otherwise discards the version, in the paper's exact sense of abort. The first two are known, 0.082 and 0.594. The control is dither. The reading predicts the third rule will not freeze 8 quanta short, because the abort is on outcome rather than on size, and says nothing about whether it will beat dither.

Neither arm has been run or priced.

## 13. Where we disagree or the analogy breaks

The break is at indivisibility, and it is the middle of the paper.

"By indivisibility we mean that the execution of one activity never appears to overlap (or contain) the execution of any other activity." Actions "are scheduled in such a way that their overall effect is as if they had been run sequentially in some order." Argus wants this because two overlapping updates to a mailbox can leave it in a state neither writer meant. Phient's paradigm says the opposite about its objects. Meaning is a side effect of pairs of words moving under a condition. Joint training is many pairs moving at once, and a word's phasor on a channel is where it is because of every pair that touched it in the same epoch. Serialize the pairs and the phasor still ends somewhere, but nothing measured says it ends in the same place, and the paradigm gives no reason to expect it. Argus forbids overlap to protect the state. Phient's state is the overlap. Section 12 does not test this because Phient has no concurrent trainer.

The second break is the fault model. A guardian "runs at a single node but can survive crashes of this node." The failure the paper guards against is one node going down while the rest continue. Phient's 64 channels are one tensor. When the process dies they all die, and the helix restores them all. Nothing in Phient has ever failed partially. The paper's term for this is a centralized implementation, which is what the authors had in 1983.

The third break is irrevocability. "The commit of a top-level action is irrevocable." Phient has no top-level action that cannot be revisited. The nearest thing to an irrevocable commit is emitting a token in decode_eval, and the paper's example fits: "printing a check, cannot be undone by program control alone." The decoder prints thirty checks per prompt and never aborts one.

The fourth point is where Phient agrees. The authors wrote a mail system, then a page saying its atomic arrays serialized too much, its registries could deadlock, and its granularity might be wrong. Phient's byte serialized too much and needed a loophole. "Such types can be built as user-defined atomic types, although we do not present implementations here" is where Phient is: dither is the type, and there is no second one.

Last, a break in the handler reading. better and worse return values, but the trainer does not call them. It calls the tape's hinge, which reads and writes phasors directly. docs/86 says so: today loss is the tape's hinge and better is implicit in it. The guardian has three handlers and one caller that bypasses them.

## 14. The person and the moment

Barbara Liskov was born Barbara Huberman. She took her PhD at Stanford in 1968 under John McCarthy, with a thesis on chess endgames played by a forcing tree and two comparison functions, better and worse. She worked at MITRE and then joined MIT. The Turing Award came in 2008.

The paper's moment is fixed by its own header. It was received in November 1981, revised in September 1982, accepted in November 1982, and published in July 1983 in ACM Transactions on Programming Languages and Systems, volume 5, number 3, pages 381 to 404. Both authors were at MIT's Laboratory for Computer Science. The work was supported by the Advanced Research Projects Agency under contract N00014-75-C-0661 and the National Science Foundation under grant MCS 79-23769. The acknowledgment thanks "members of the ARGUS design group, especially Maurice Herlihy, Paul Johnson, and Bill Weihl."

The paper stands on CLU. The atomic arrays, records and variants are CLU's with atomicity added. The 1977 CACM paper "Abstraction Mechanisms in CLU" is in the reference list, and CLU "was chosen because it supports the construction of well-structured programs through abstraction mechanisms." Robert Scheifler was one of the seven authors of the CLU Reference Manual, read in the eighth paper of this series, where Liskov's history assigns him the compiler front end. The nested actions rest on two MIT theses the paper cites: D. P. Reed's 1978 dissertation, MIT/LCS/TR-205, and Eliot Moss's 1981 dissertation "Nested Transactions: An Approach to Reliable Distributed Computing," MIT/LCS/TR-260. Moss was also an author of the CLU manual. Stable storage is cited to Lampson and Sturgis at Xerox PARC, 1979, and the recovery background to J. N. Gray's 1978 notes.

So the moment is this. The CLU project finished its manual in October 1979. Within two years the same laboratory had submitted a paper on what to do when the modules of a CLU-shaped program are on different machines. The answer kept the cluster's discipline, a rep reached only through its operations, and added two things CLU did not have: state that survives a crash, and a call that can fail and be undone. In 1983 this was a design with a centralized prototype. "We are working on a real, distributed implementation. At this point it is unclear how efficient such an implementation can be."

What came after is stated here only as far as it was reached. Wikipedia dates Argus to 1982 through 1988 and names Herlihy, Johnson, Scheifler and Weihl alongside Liskov. Liskov's 1988 article "Distributed Programming in Argus" appeared in Communications of the ACM 31(3), pages 300 to 312. I did not read it.

## 15. What remains

For Phient, five things, in the order they could be done.

Count the aborts. Section 8 read the quest's flat table as a failure exception reported as a mean: better −0.008, worse −0.008, loss 0.108 at margin 0.1, on nearly every row. The paper's design reports failure by name and the caller counts. quest_ingest should report, per hexagram, how many rows tie within the deadband, how many have no onward relation, and how many miss the margin, alongside the means it reports now.

Run forcing at depth two on the quest. Section 12's first arm. docs/86 asked for it, and the fraction of aborted subactions is the number that decides whether the run tested anything.

Run the outcome-commit rule. Section 12's second arm: commit a nudge when better on the training pair increased and discard it otherwise. Two of the three rules are already measured at 0.082 and 0.594. The third is the paper's action applied to one step.

Check the recover section. The helix round trip is byte-exact on the stable state. The paper's recover section must also rebuild a volatile state consistent with it. The check is small: file every word by hexagram before the round trip and after, and count the words that moved sector. If none moved, the volatile state is a function of the stable state, as it should be. If some moved, the hexagram depends on something the helix does not store.

Route the trainer through the handlers. Section 13 noted that the tape's hinge reads and writes phasors directly and that better is implicit in it. docs/86 lists exposing better, worse and loss as the three names of the training loop. Until then the paper's first rule, that guardians communicate by calling handlers, is not met.

A sixth is a question. Argus answers deadlock by relying "on the user to time out and abort top-level actions." Phient has no concurrent trainer, so it has no deadlock, and section 13 said its paradigm wants the overlap Argus forbids. But the day two trainers touch one word at once, the question of which nudge commits will arrive, and Phient has no rule. Whether both nudges landing is a rule or an accident is not something the facts file can say.

For the reading itself, two things remain. The 1988 CACM paper was not read, and it is where the distributed implementation the 1983 paper promised would be reported. And the orphan algorithms the paper defers were not reached; Wikipedia cites Walker's MIT/LCS/TR-326, which I did not open.

## 16. Sources

Reached and read, through WebFetch on 2026-05-04.

Barbara Liskov and Robert Scheifler, "Guardians and Actions: Linguistic Support for Robust, Distributed Programs," ACM Transactions on Programming Languages and Systems 5(3), July 1983, pages 381–404, read from the copy served for MIT's 6.824 course (https://pdos.csail.mit.edu/6.824/papers/liskov-argus.pdf). Every quotation from the paper in this document is from this copy: the abstract, the opening paragraph, the goals, the definitions of indivisibility and recoverability, the lock and version rules, the nested action rules, the remote procedure call argument, the handler call steps, the guardian structure, the coenter statement, the deadlock policy, the mail system and its remarks, the implementation status, the orphan sentence, the summary, the acknowledgment, the dates, the funding, and the reference entries for CLU, Lampson and Sturgis, Reed, Moss and Gray. Section headings were taken from the same copy.

MIT OpenCourseWare, 6.824 Spring 2006, the course's reading notes on the paper (https://ocw.mit.edu/courses/6-824-distributed-computer-systems-engineering-spring-2006/9a4259c1b7785210c883f9821e15ec2d_argus.pdf). These are study questions, and nothing in this paper rests on them.

Christopher Meiklejohn, "ARGUS," 8 August 2016 (https://christophermeiklejohn.com/pl/2016/08/08/argus.html), a reading summary used only to cross-check the concepts.

Wikipedia, "Argus (programming language)" (https://en.wikipedia.org/wiki/Argus_(programming_language)), used only for the 1982 to 1988 dates, the names of the design group, and the citations to Liskov's 1988 CACM article and Walker's TR-326, marked as such in sections 4, 14 and 15.

Not reached. The ACM Digital Library records of the paper and of the 1988 CACM article (10.1145/42392.42399) were not fetched; the fetcher required an approval that did not arrive. The dblp TOPLAS volume 5 page was disallowed by robots. The Semantic Scholar record returned no content. The PMG copy of the 1988 paper (pmg.csail.mit.edu/papers/argus88.pdf) was not fetched for the same reason as the ACM records. Copies of the 1983 paper at pdos.csail.mit.edu, courses.mpi-sws.org and research.cs.queensu.ca could not be downloaded by curl through the proxy; the pdos copy was read through the fetcher instead. Liskov's 1988 "Distributed Programming in Argus" was not read. Walker's TR-326 was not read.

Consequences. The paper was read in extracts returned by a fetcher against specific prompts rather than as a continuous text, so the handler-call steps are given as the fetcher returned them, with the middle steps abbreviated. Nothing about the distributed implementation after 1983, its performance, or its uses is stated. Scheifler's and Moss's authorship of the CLU manual is taken from the eighth paper of this series, which verified it against the MIT DSpace record and Liskov's history of CLU. Every quotation is attributed to the document it was read in.

All Phient numbers are from PHIENT_FACTS.md and docs/86_liskov_three_operators.md in this repository. No number about Phient was taken from anywhere else, and none was computed for this paper.
