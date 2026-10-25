# The word as a guardian: stable relations, handler calls that happen once or not at all, and the nudge as a nested action

*Phient interpreted papers · Liskov series, 10 of 16 · 2026-05-18 · on: "Distributed Programming in Argus" (Barbara Liskov, Communications of the ACM 31(3), pages 300–312, March 1988)*

"In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition."

## 1. Our abstract

The 1988 paper is Liskov's account of Argus as a whole, written for the general readership of the Communications after the language had been running for about two years. It is the tenth work in this series and the first in which the unit of programming is no longer a module in one address space. Argus is an extension of CLU, and the paper says so in one parenthesis. What it adds is two things. A guardian is an abstract object that encapsulates a resource and can be reached only through handlers. An action is an atomic computation that either commits everywhere or has no visible effect anywhere. Actions nest. A handler call is itself a subaction, and so a remote call happens once or not at all. State inside a guardian is divided into stable objects, which survive a crash, and volatile objects, which are rebuilt by a recovery process.

This paper reads Argus through Phient. A word in Phical is 32–64 relations, each a byte on a circle with a count, and nothing else touches those relations except the operators better, worse and loss. That is a guardian with three handlers. The byte on the circle is the stable state; the helix writes it to a codon and reads it back byte-exact. The tape's gradients and the cached cosines are volatile. A training nudge is an action: it makes a copy, modifies the copy, and commits the copy to the base version. The byte deadband is what happens when that commit is refused, and nearest rounding froze every pair 8 quanta short. Forcing, which appends each candidate and applies the operator once more, is a set of subactions run under coenter, each of which can abort without aborting the parent.

The reading also says where it breaks. Argus exists to make concurrent activities not interfere. Phient's training is 204,592 examples interfering on purpose, because meaning is a side effect of pairs moving. Argus has deadlock and Phient has nothing like it. Argus's stable storage loses nothing with high probability; Phient's stable byte loses every nudge under half a quantum. The measured numbers in section 11 are from the facts file only. Section 12 proposes an experiment in which the rounding step is treated as a subaction with checkpoint and retry, against the shipped dithered rounding as control. Section 16 says which copies of the paper were reached and which were not.

## 2. What the work claims, in its terms

The paper opens with a definition and a problem. "Argus--a programming language and system-was developed to support the implementation and execution of distributed programs. Distribution gives rise to some problems that do not exist in a centralized system, or that exist in a less complex form." The problem is stated as a contrast: "a centralized system is either running or crashed, but a distributed system may be partly running and partly crashed." Nodes crash. The network loses messages, delays them, reorders them, and partitions. The goal is "to provide mechanisms that make it easier for programmers to cope with these problems."

The programs the paper has in mind keep online data for a long time: file systems, mail, inventory control. For these it lists what is wanted. Consistency under failures and concurrent use. Service while some components are down. Access without knowing where the data is. Reconfiguration while running.

The mechanisms are two. "An Argus guardian is a special kind of abstract object whose purpose is to encapsulate a resource or resources." A guardian lives at one node, chosen at creation: "The creator specifies the node at which the new guardian is to reside." It is reached through "special procedures, called handlers, that can be called from other guardians." Calls carry values, not references: "Arguments are passed by value, which ensures that a guardian's objects cannot be accessed directly by any other guardian." Inside, state is stable or volatile. Stable objects are written to stable storage, and "With high probability, stable storage devices avoid loss of information in spite of failures."

The second mechanism is the action. An action is serializable, meaning "the effect of running a group of actions is the same as if they were run sequentially in some order," and total, meaning "an action either completes entirely or it is guaranteed to have no visible effect." An action commits or aborts. "Argus allows actions to be nested; thus an action can have one or more subactions," and nesting is justified twice: "They allow concurrency within an action," and "They can be used to establish checkpoints within an action." The paper's own statement of the whole is in its conclusion: "Argus is unique because it provides atomic actions within a programming language."

The claim about experience is modest and specific. "Argus has been running for about two years, although early in this period the implementation was quite incomplete." Several programs had been built, and "it is relatively easy to write distributed programs, even sophisticated ones that replicate information." The paper does not claim that Argus solves concurrency. "The programmer must think about deadlocks and starvation and implement the code to avoid them when possible."

## 3. The mechanism, plainly

Take the parts in the order a program meets them.

A guardian is created at a node. It has stable objects, volatile objects, handlers, and a recovery routine. "Whenever a handler call arrives at a guardian, a process is created to run the call." When the node crashes, everything volatile is gone. "After the crash, the Argus system restores the guardian's code and recovers the stable objects from stable storage. Then it creates a special recovery process" that rebuilds the volatile objects.

An action starts with enter topaction. A topaction has no parent. Inside it, enter action starts a subaction, and coenter starts several subactions that run in parallel. "Argus does not permit a parent to run concurrently with its children, nor does it permit any concurrency within an action except by creating subactions." Every handler call is a subaction: "Argus runs every handler call as a subaction; we refer to this subaction as the call action. This extra action ensures that calls have a zero or one semantics." If the guardian replies, the call happened exactly once. "If it is not possible to complete the call, we abort the call action, thus guaranteeing that the call (effectively) did not happen at all."

Atomicity is carried by atomic objects. Argus provides built-in atomic arrays and records "which have the same kinds of operations as ordinary arrays and records." Their operations are readers or writers. Readers take read locks, writers write locks, held until the action completes. Recovery is by versions: "The state of an unlocked object is stored in a base version. Modifications to an object are not done to the base version directly. Instead a copy is made (in volatile memory), and modifications are done to the copy."

Commit moves up the tree. When a subaction commits, "its locks and versions for objects belonging to its guardian are propagated to its parent." If it aborts, "its locks and versions are discarded and its parent action can continue from the state at which the subaction started." Starting an action costs only a unique identifier and some data structures.

A topaction commit is the expensive step. "When a topaction commits, the system carries out the two-phase commit protocol to ensure the action either commits everywhere or aborts everywhere." In Phase One the coordinator sends prepare messages; each participant records the versions written under the action on stable storage, writes a prepare record, and answers ok. Then the coordinator writes a committed record and tells the participants to commit. The cost is stated exactly: "one network round-trip (the 'prepare' and 'ok' messages) plus two writes to stable storage (the prepare and committed records)."

## 4. What it demonstrated, as reported

The paper demonstrates with one running example and one table.

The example is a bank. Figure 1 is the configuration: branches at different locations, each with its own accounts; clerks who deposit and withdraw; transfers between branches; auditors who total across them. The two problems shown are interference, where an audit sees a transfer half done, and failure, where a crash leaves one half done. Figure 6 is the branch guardian. Its stable state is three objects: a hash table ht from account numbers to buckets, a string code naming the branch, and an atomic record seed for generating account numbers. The types are atomic: htable is atomic-array[bucket], bucket is atomic-array[pair], pair is atomic-record[num: account-number, acct: acct-info]. The five handlers are open, close, deposit, withdraw and total. Figure 7 is a portion of the front-end guardian. Transfer is a topaction whose body is a coenter with two subactions, f.withdraw(from, amt) and t.deposit(to, amt); both commit or both abort. Audit is a topaction that "communicates with all branches in parallel, using a separate process and subaction for each call. As each call returns, the total is incremented." Figure 3 is a printer guardian and Figure 4 shows atomic objects in use.

The example is also where the paper is honest about locking. "Most of the handlers can deadlock with other concurrent operations. For example, deposit can deadlock with other deposits or withdrawals on its account," and "a concurrent transfer and audit could deadlock." The remedy offered is ordering: obtain the write lock first, as open does. It also notes that "A user-defined atomic type would permit us to keep a running total without having conflicts between deposits and withdrawals."

The table is Table II, action performance in milliseconds, on MicroVAX II machines under Ultrix 1.2, a 10 megabit per second Ethernet, 9 or 13 megabytes of memory, two RD53 disks of 70 megabytes each with one disk given to stable storage.

| action | read-only | update |
|---|---|---|
| subaction commit | 0.60 | 0.81 |
| subaction abort | 0.65 | 0.85 |
| local topaction commit | 0.63 | 17.50 |
| topaction commit, one participant | 36.50 | 82.00 |

The paper's verdict: "we consider our performance quite satisfactory and believe it indicates the practicality of systems like ours." The applications reported are a program library with stable storage, a catalog for finding guardians and handlers at run time, "a distributed editor that allows users at different locations to collaborate on the same document," a replicated mail repository, a Hailstone number program, and a distributed multi-user game. No sizes, user counts or uptimes are given for any of them.

## 5. The one idea restated

A computation that touches several places either happens everywhere or happens nowhere, and the language, not the programmer, keeps that promise.

Everything in the paper serves that sentence. The guardian exists so that a place has a boundary: its objects cannot be reached except through handlers, and handlers take values, so nothing outside can hold a reference into it. The division into stable and volatile exists so that the promise survives a crash: what was committed is on stable storage, and what was not is rebuilt or discarded by the recovery process. The action exists so that "happens" has a definition: serializable, so concurrent activities do not see each other's middles, and total, so a failure leaves nothing half done. The subaction exists so that the promise can be kept at more than one grain. A handler call is a subaction because a remote call is the one thing in a distributed program that most often half happens, and making it a subaction gives it zero-or-one semantics for free. Coenter exists so that concurrency within an action is itself made of actions, each of which can abort alone. Two-phase commit exists because the promise crosses machines, and the paper prices it: one round trip and two stable writes.

The paper also says what the idea does not do. It does not choose the grain. The programmer decides what is a topaction and what is a subaction, and the conclusion's third paragraph insists that both weak and strong consistency are expressible, a delayed deposit and an immediate one. It does not prevent deadlock, and the implementation did not yet detect it. It does not make representation cheap: the programmer "must think about" the efficiency of stable writes and the degree of concurrency, and user-defined atomic types are the answer but are hard to write.

Restated for Phient: a word's relations are stable state behind a boundary, the three operators are its handlers, and a nudge to a relation is an action that is either committed to the byte or has no effect. Sections 6 through 10 take this apart. The guardian is the word. The node a guardian resides at is the channel. The handler call with zero-or-one semantics is X ⇒ Y under c. The subaction under coenter is forcing. Two-phase commit across guardians is the two-hop quest.

## 6. The word as a function, and the guardian

Phical says a word is 32–64 relations and nothing else, each a phasor with a count, and that meaning is what pairs of words do under a condition. Argus has a name for an object that is nothing but what it can be asked. It is a guardian.

A guardian encapsulates a resource. Its objects "cannot be accessed directly by any other guardian." Access is by handlers, and arguments go by value. For a Phient word the resource is the 64 channels of bytes, 256 quanta of 2π each, and the count on each. The handlers are three: better(pred, a, b), worse(pred, a, after, continuations) and loss(pred, target, decoy, margin). Each takes predictions and words by value, in the sense that the caller receives a comparison and never a reference into the channels. That is the guardian reading of the word: a function from (condition, pair) to a comparison, stored as phasors behind a boundary that only three procedures cross.

Argus then divides the state, and the division fits Phient exactly. Stable objects survive a crash. Volatile objects do not, and a recovery process rebuilds them. A Phient word's stable state is the byte on each channel with its count. The helix is the stable storage: 64 codons, base-4 ACGT of the channel, 18,496 words written in 1.9 s to 75 MB and read back byte-exact. The round trip is the recovery process: restore the stable objects, then rebuild what is volatile. What is volatile in phinet is the Tape's gradients, the FFT buffers, and the cosines the decoder caches against a pool. None of it is written by the helix, and none of it need be, because it is what the paper calls "redundant information" that the guardian's own code can recompute.

The paper's process model lands too. "Whenever a handler call arrives at a guardian, a process is created to run the call," and a guardian may hold many at once. The 204,592 Austen training examples are handler calls arriving at 1,228 words. Each is a process inside the word it addresses, and a word with a high count is a guardian with a long queue. Joint training in phinet is many guardians receiving calls in parallel.

One clause needs care. Argus's guardian "encapsulates a resource," and the resource is usually something outside the program: a printer, a set of accounts. A Phient word's resource is itself. There is no printer behind the handlers. Section 13 comes back to this, because it is where the guardian reading is weakest and where it says the most.

## 7. The relation channel as a coordinate, and the node

In Argus, where a guardian lives is chosen and matters. "The creator specifies the node at which the new guardian is to reside; in this way individual guardians can be placed at the most advantageous locations." A handler call is location independent for the caller, but the system must route it, and the node is the address. Objects at one node are not objects at another; a guardian at node A holds no reference into a guardian at node B.

A Phient channel is the node. The condition c places a relation on one of 64 circles. Under c = is-a a relation resides on one channel; under c = part-of it resides on another. The quest oracle's five kinds, is-a, part-of, near, used-for and unlike, are five nodes, and the coordinate of a channel is which node it is. A relation written on the is-a channel is a guardian created at the is-a node, and asking for it elsewhere is a call to a guardian that is not there.

The quest measures what such a call returns. Within-kind MRR is 0.747. The wrong-channel reading is 0.015. The offset reading is 0.005. Chance is 0.021. Reading a relation from the channel it was not written on gives less than chance, which is the Argus routing rule seen through a ranking: the call went to a node that holds no such guardian, and what came back was noise. Phient's operator knows the condition and routes to the channel; the wrong-channel row is what happens when the routing is bypassed.

The second half of the node reading is serializability across nodes. Actions on guardians at different nodes commute unless a topaction ties them together. Two relations on different channels of the same word commute in the same sense: a nudge under is-a does not move the part-of byte. This is why the 64 channels can be trained jointly without a lock table. The paper's serializability is enforced by the language; Phient's is given by the geometry, one circle per condition, and no two conditions share a byte.

The sphere tie changes the coordinate system without changing the reading. phinet::Tie::Sphere makes the 64 channels Y_l^m with l < 8, one complex weight per degree, 1,192 params. The anisotropic tie gives one weight per (l, |m|), 36 slots, 4,776 params. The nodes are the same 64; what changes is how a weight is shared across them. The guardians stay put and the network between them is rewired. The Austen hinge numbers, circle 14.7×/14.6× at 8,360 params, sphere 12.8×/13.1×, anisotropic 16.0×/15.6×, are what the rewiring cost and gained. Argus has no rewiring that changes results, because its network carries messages and not meaning.

## 8. The operator as X ⇒ Y under c, and the call action

The paper's central sentence about calls: "Argus runs every handler call as a subaction; we refer to this subaction as the call action. This extra action ensures that calls have a zero or one semantics." A call either returns a reply, and then it happened exactly once, or "we abort the call action, thus guaranteeing that the call (effectively) did not happen at all."

X ⇒ Y under c is a handler call. The caller holds X and c. The guardian is the word, the node is the channel c selects, and the reply is a ranking over Y. Zero-or-one semantics is what the operator already has, unnamed. In the quest held-out runs, one hop returns 0.011 MRR and two-hop part-of, near and unlike return chance. Those are calls that aborted: no usable reply, no effect on the word. The operator's reply on Austen, 30.7×/30.3× at 8 epochs for the wide+deep softmax with hit@10 0.348, is the call that happened once.

The reading is sharpest on training, because a training step is a call that writes. Argus writes through a version: "Modifications to an object are not done to the base version directly. Instead a copy is made (in volatile memory), and modifications are done to the copy." At commit the copy becomes the base. The trainer's nudge is that copy. The Tape computes 0.06·sin d in floating point, volatile memory; the byte on the circle is the base version. Commit is rounding to the byte. The byte deadband is a commit that does nothing: once sin d < 0.2, about 11 degrees, the nudge is under half a quantum, and nearest rounding returns the base version unchanged. Nearest rounding froze every pair 8 quanta short. In Argus's terms the copy was made, modified, and discarded at commit, which is an abort. Nothing in the trainer said so, and the shipped quest trainer's 0.082 MRR is what silent aborts produce.

Dithered rounding reads as a change in commit policy: commit the copy with a probability tied to how far it moved, so that many small calls together move the byte. Attraction-only training with dithered rounding took the quest to 0.594 MRR, hit@1 34.8%, hit@5 97.4%. It costs corpus attribution, 71% to 65%, so it ships off by default. Argus would leave that choice to the programmer. That is the paper's third conclusion paragraph, that actions "support a spectrum of requirements."

The last piece: a call action that aborts lets the parent continue "from the state at which the subaction started." The byte deadband gives that by accident and Argus by design. A refused nudge leaves the word exactly as it was. Section 12 proposes making the refusal visible.

## 9. Nested actions, coenter, and forcing

Argus gives two reasons for nesting. "They allow concurrency within an action." "They can be used to establish checkpoints within an action." Coenter is the construct for the first: several subactions started together, each a process, each able to abort alone. Figure 7's audit is the example: one topaction, a coenter over every branch, one subaction per total, and the parent sums as each reply arrives.

Forcing in phiano is coenter. OperatorGuide::nearest_forcing takes the 2k nearest words, appends each to the context, applies the operator once more, and re-ranks by cos − λ·worse. Each appended candidate is a subaction of the decoding step. Its call to the operator is a call action. worse is what the subaction returns: how far the best continuation after the candidate falls below the candidate's own fit. A dead end is a subaction that aborts, in the sense the paper gives it, and its abort does not abort the parent, which simply ranks it low. forcing_choice is the parent summing as replies arrive. The three unit tests in phinet::liskov, antisymmetry, a dead-end move is refuted, and forcing prefers the move with a future, are tests of the subaction discipline.

The measured result is that on text the parent should not have asked. decode_eval, 40 held-out prompts × 30 tokens, k = 32: nearest-k KN perplexity 66.9; forcing λ = 0.05 69.7; λ = 0.25 68.5; λ = 1 107.4. At λ = 1 the evaluator's own score rises to 0.712, distinct falls to 0.777, and shared 4-grams across prompts rise to 22.3%. Forcing does not beat greedy on text. docs/86 gives the reason: in text every word has some continuation, so no subaction aborts, and the ranking drifts toward the words with the most continuations, which are function words. Liskov's endgame is a collection where most subactions abort. Text is not.

Argus's second reason for nesting, the checkpoint, is what Phient lacks and section 12 asks for. A subaction that aborts lets the parent "continue from the state at which the subaction started" and try something else. The trainer today has no such retry. When rounding refuses a nudge the step is over. A checkpointed trainer would notice the refusal and take a second, differently rounded step from the same state. Dither is the version of this that never looks: it commits some fraction of small nudges without knowing which were refused.

The paper's restriction on concurrency also lands. "Argus does not permit a parent to run concurrently with its children." Decoding in phiano obeys this: the parent step waits for every candidate's worse before it ranks. Joint training does not, and section 13 says why that is a break and not a bug.

## 10. Two-phase commit, the two-hop quest, and the audit

A transfer between branches is the paper's reason for two-phase commit. Withdraw at one guardian and deposit at another must both happen or neither, and the guardians are at different nodes. The coordinator sends prepare; each participant writes its versions and a prepare record to stable storage and answers ok; the coordinator writes committed and tells the participants. The paper prices it at "one network round-trip plus two writes to stable storage," and Table II gives 82.00 ms for an update topaction with one participant against 17.50 ms for a local one.

The quest's two-hop reading is a transfer. A two-hop is-a query asks for a relation held by one word to be followed by a relation held by another, on the is-a channel of each. Two guardians, two nodes, one topaction. Held-out two-hop is-a scores 0.095 MRR and used-for 0.093, against chance 0.021; part-of, near and unlike are at chance; one hop is 0.011. Something commits on is-a and used-for and not on the others. Argus would ask which participant failed to prepare. Phient cannot yet say, because the operator returns one ranking and not a record per hop. Section 15 lists the per-hop record as work.

The audit is the other pattern, and the hexagram table is Phient's audit. quest_ingest scores every word by the three operators and files it under the hexagram its 64-channel function is, the centroid phase in 64 sectors. This is Figure 7's audit: one topaction, a coenter over every word, one subaction each, and a table summed as replies arrive. The result on the synthetic quest is that 256 words occupy 61 of 64 hexagrams with mean better −0.008, mean worse −0.008, and mean loss 0.108 at margin 0.1. The audit is flat. The paper's bank audit is interesting only when transfers are in flight, and its deadlock warning is that "a concurrent transfer and audit could deadlock." Phient's audit ran on a corpus with no transfers, a flat oracle, and the flat table is what a flat corpus audits to. The Phi-4 quest, not yet run, is the corpus with transfers.

The paper's own gloss on why the audit sees a consistent total is serializability. Phient's version is that within-kind reading is 0.747 while wrong-channel is 0.015: the audit reads each word on its own channel and never sees a half-written byte, because a byte is written whole or not at all. That is totality at the grain of one quantum. It is the smallest atomic object Phient has, and section 13 says it is also the reason the stable-storage analogy fails.

## 11. What Phient has measured that bears on it

Everything here is from the facts file. Nothing else is claimed.

| Argus construct | what Phient measured | number |
|---|---|---|
| stable objects recovered from stable storage | helix round trip, 18,496 words | 1.9 s, 75 MB, byte-exact |
| guardian at a node; call to the wrong node | quest within-kind / wrong channel / offset | 0.747 / 0.015 / 0.005 (chance 0.021) |
| call action with zero-or-one semantics | quest held-out one hop | 0.011 MRR |
| topaction across two guardians | quest held-out two hops | is-a 0.095, used-for 0.093, part-of/near/unlike ≈ chance |
| commit refused, base version unchanged | nearest rounding, deadband | froze every pair 8 quanta short; sin d < 0.2 (11°) |
| optimistic commit policy | attraction-only + dithered rounding | 0.082 → 0.594 MRR; hit@1 34.8%, hit@5 97.4%; attribution 71% → 65% |
| coenter over candidates (forcing) | KN at λ = 0.05 / 0.25 / 1 vs nearest-k | 69.7 / 68.5 / 107.4 vs 66.9 |
| audit over all guardians | quest scored by three operators, filed by hexagram | 61 of 64 hexagrams; better −0.008, worse −0.008, loss 0.108 |
| the same interface on three reps | Austen hinge 3 epochs, circle / sphere / anisotropic | 14.7×/14.6× (8,360 params, 142 s) / 12.8×/13.1× (1,192, 89 s) / 16.0×/15.6× (4,776, 85 s) |
| handler agreement with a reference | phinet vs PyTorch | 62/62 parity tests |

A few rows need a sentence. The round-trip row is the only direct analogue: stable state written and recovered without loss. The wrong-node row is the routing rule measured. The one-hop and two-hop rows are the call-action and topaction rows, and two hops beat one on two kinds, which no reading of Argus predicts and section 13 flags. The deadband and dither rows are commit policy, where the Argus reading changed how Phient describes something it already had. The forcing row is coenter, measured, and negative on text. The audit row is flat and section 10 said why.

Not measured: no Phi-4 quest exists, so no audit over a corpus with structure. No per-hop record exists for the two-hop queries. The sphere at softmax and at wide+deep is unrun. No timing in Phient corresponds to Table II; the engine is allocation and bandwidth bound at these sizes, 1 MB op 2–5 ms, and nothing in that number is a commit cost. Persona attribution at 71% overall (74/65/86) and dialogue at 44% / 72% against 33% do not bear on Argus. No refused nudge has ever been counted as a refusal.

## 12. The experiment

The paper's claim, reduced to one testable sentence: a modification made on a copy and committed as a unit leaves the base either fully changed or unchanged, and a caller who learns which can retry. Phient can test the second half inside phinet, because the trainer's nudge is exactly such a copy and the byte is exactly such a base.

The experiment is a checkpointed trainer on the quest. Setup as the facts file states it: phiecs synthetic oracle, 256 words, families of 8, five kinds, 1,280 rows, 293 target words, chance MRR 0.021. Attraction-only training. Three arms.

Arm one is the control: nearest rounding, as shipped, which gave 0.082 MRR. Arm two is dithered rounding, which gave 0.594 MRR, hit@1 34.8%, hit@5 97.4%. Both numbers are known and the arms are run again only so that the third is comparable.

Arm three is the Argus arm. Each nudge is a subaction. Compute the copy in floating point, round to the byte, and compare to the base. If the byte changed, commit. If it did not, abort, record the refusal against the pair and channel, and retry once from the same state with the nudge accumulated: add the refused fraction to the next nudge for that pair rather than discarding it. This is the paper's checkpoint, and it is deterministic where dither is random. Report MRR, hit@1, hit@5, and the count of refusals per epoch.

The prediction that would confirm the reading: arm three reaches at least arm two's 0.594 without dither's cost to attribution, since it commits only nudges that actually crossed a quantum. The prediction that would refute it: arm three stays near 0.082, which would mean the deadband is not a commit problem at all but a geometry problem, and the byte ceiling is where the meaning stops rather than where the write stops.

A second arm tests coenter where docs/86 says it should bite. Run forcing at depth two on the quest oracle with the three operators scoring, control attraction-only plus dither at 0.594. The paper's own justification for nesting is concurrency within an action, and a quest word either has onward relations or does not, so a fraction of the subactions should abort. If the flat hexagram table stays flat, mean better −0.008 and mean worse −0.008, the aborts are not being seen, and the second thing to build is a refusal counter for worse: a dead end reported as a dead end, not as a small negative number.

Neither arm has been run. The first is priced by nothing in the facts file; the quest ingest's timing is not recorded there. The second is unpriced.

## 13. Where we disagree or the analogy breaks

Argus's purpose is to make concurrent activities not interfere. Phient's purpose is interference. That is the break, and everything else follows from it.

The paper defines serializability so that the audit never sees a transfer half done. Phient's training is 204,592 handler calls arriving at 1,228 guardians, and every one of them is meant to be seen by the others while half done. A nudge to "Emma" under one condition changes what the next call to "woman" under that condition compares against, and that is the mechanism. Argus would call a side effect that one action has on another a serializability violation. Phient calls it learning. Joint training also breaks the rule that "Argus does not permit a parent to run concurrently with its children": the epoch does not wait for each word's queue to drain. The guardian reading in section 6 is right about the boundary and wrong about the discipline behind it.

The second break is stable storage. The paper's stable storage "avoids loss of information in spite of failures" with high probability. Phient's stable byte is exact on recovery, 18,496 words round-tripped, but lossy on write: every nudge under half a quantum, once sin d < 0.2, is lost. Argus loses at crash and never at commit. Phient loses at commit and never at crash. Section 8 called the refused nudge an abort, which is right about the effect; the cause is the resolution of the store.

The third break is deadlock. The paper spends its honesty on it: handlers deadlock, transfer and audit can deadlock, and the implementation did not detect it. Phient has no locks and no deadlock. Two nudges to the same byte both apply, in the order the tape runs them. What Phient has instead is the flat table, 61 of 64 hexagrams with means at −0.008, a system in which every transaction commits and nothing distinguishes them.

The fourth point is an anomaly the reading exposes. Two hops beat one hop in the held-out quest, 0.095 and 0.093 against 0.011, and no Argus topaction is more likely to commit than its own subactions. Either the one-hop number measures something other than a call action, or the two-hop is not composed of two one-hops. The facts file does not say which, and this paper does not guess.

The last disagreement is with the paper's closing self-description. "Argus is unique because it provides atomic actions within a programming language." Phient has no language in that sense. Its atomicity is one quantum of one byte, given by the store. What the paper calls a design decision, Phient gets as a physical fact, and cannot change without changing the byte.

## 14. The person and the moment

Barbara Liskov was born Barbara Huberman. Her Stanford PhD, 1968, under John McCarthy, played chess endgames with a forcing tree and two comparison functions, better and worse. She worked at MITRE and then joined MIT. The Turing Award came in 2008, twenty years after this paper.

The paper's moment is March 1988. Communications of the ACM, volume 31, number 3, pages 300 to 312, in a special section. Her affiliation on the paper is the MIT Laboratory for Computer Science at 545 Technology Square. The acknowledgement names the collaborators: "Argus is the result of a collaboration with a number of other researchers, including Maurice Herlihy, Paul Johnson, Robert Scheifler, and William Weihl." Scheifler is the co-author of the 1982 TOPLAS paper "Guardians and Actions" that the paper cites as reference 14, and of the CLU papers read earlier in this series. Weihl is co-author of reference 20 on resilient atomic data types, the mechanism behind the user-defined atomic types the paper wants and finds hard. The reference list has 20 entries, of which seven are by Liskov or her group, including the Argus reference manual, "Implementation of Argus" with Curtis, Johnson and Scheifler, and "Abstraction Mechanisms in CLU."

The paper is written from inside a running system, and says so plainly: "Argus has been running for about two years, although early in this period the implementation was quite incomplete." The machines are named, MicroVAX II under Ultrix 1.2, on a 10 megabit Ethernet, with one of two RD53 disks given to stable storage. Table II is measured on them. The applications are listed without numbers. The future work is stated without a date: deadlock detection, better support for reconfiguration, cheaper communication through special kernel calls, and a fuller evaluation once the applications have users. The team intended to leave the language "essentially unchanged" while doing it.

The position the paper takes against the field is short. Languages such as SR, Ada and Mesa support distribution but not transactions. Databases provide transactions but not for general objects. Systems such as Camelot give operating-system support without language integration. Argus's claim is the intersection, and the paper does not claim more than that.

One continuity with the earlier papers deserves saying. The 1977 CLU paper argued that conventions are no substitute for enforced constraints. The 1979 manual enforced them for one address space. The 1988 paper enforces one more: that a call to another place happens once or not at all. Same reason, larger machine. Nothing in the sources reached says what Liskov thought of the two years beyond what the conclusion states, and this paper does not add to it.

## 15. What remains

For Phient, five things, in the order they could be done.

Count the refusals. The byte deadband is a commit that does nothing, and today nothing records it. A counter per pair and channel of nudges that rounded to the base version is the cheapest measurement in this paper and the one the Argus reading most directly asks for. It turns "froze every pair 8 quanta short" from a post-mortem into a per-step signal.

Run the checkpointed trainer. Section 12's third arm: accumulate a refused nudge into the next one for the same pair rather than discarding it, and compare against nearest rounding at 0.082 and dither at 0.594. If it reaches dither's number without dither's cost to attribution, the shipped default changes.

Record each hop. The two-hop quest returns one ranking for two calls. Argus would write a prepare record per participant. A per-hop record, which guardian answered and with what margin, is the only way to say why is-a and used-for commit at two hops and part-of, near and unlike do not, and why two hops beat one.

Run forcing at depth two on the quest. docs/86 asked for it and section 9 said why the coenter reading predicts it should matter on a collection where subactions abort. Report worse as a refusal count and not only as a mean.

Name the flat table. Argus has no word for a system in which every action commits and the audit distinguishes nothing. Phient has one such system, the synthetic quest, and its table has means of −0.008. The Phi-4 quest is the corpus with transfers in flight. Until it is run, the hexagram table is an audit of a bank with no customers.

Two things remain for the reading itself. The break in section 13 between serializability and interference is stated and not measured. A measurement would be an epoch run with the words' queues drained in sequence, one word at a time, against the shipped joint run, on the Austen setup with the circle tie as control at 14.7×/14.6×. If the sequential run matches, interference is not the mechanism and the guardian reading holds in full. If it falls, interference is the mechanism and the reading holds only at the boundary. And the paper's own list of applications, the distributed editor and the replicated mail repository in particular, would be worth reading in their own reports, since the CACM paper gives no numbers for them and Phient's persona and dialogue work has no analogue yet of a system that several users touch at once.

## 16. Sources

Reached and read, through WebFetch on 2026-05-18.

Barbara Liskov, "Distributed Programming in Argus," Communications of the ACM 31(3), March 1988, pages 300–312, read from a course copy at MIT (https://people.csail.mit.edu/alinush/6.824-spring-2015/papers/argus88.pdf). Every quotation in sections 2, 3, 4, 5, 8, 9, 10, 13 and 14 is from this document: the opening definition, the guardian and handler sentences, the call action and zero-or-one semantics, the two reasons for nesting, the locking and version rules, two-phase commit and its cost, the branch guardian's types and handlers, the front-end's transfer and audit, the deadlock sentences, Table II and its hardware, the application list, the conclusion, and the acknowledgement. The fetcher returned the text in pieces across several calls and did not return the conclusions verbatim in full; where section 14 paraphrases, it says so. The hardware names as returned read "MicroVAX-11" and "RD.53"; this paper writes MicroVAX II and RD53.

Wikipedia, "Argus (programming language)" (https://en.wikipedia.org/wiki/Argus_(programming_language)), used only to confirm the collaborators named in the acknowledgement and the citation. Its dating of the project to 1982–1988 is not used.

Christopher Meiklejohn, "ARGUS" (https://christophermeiklejohn.com/pl/2016/08/08/argus.html), reads the 1982 TOPLAS paper, the 1987 implementation paper and this one together. Its quotations could not be attributed to one of the three with confidence, and none is used.

Lecture slides, "Distributed Programming in Argus, Barbara Liskov, CACM 1988," University of British Columbia CPSC 538B (https://www.cs.ubc.ca/~bestchai/teaching/cs538b_2020w1/slides/Argus.pdf). Lecture notes with no quotations from the paper. Nothing here rests on them.

Not reached. The ACM Digital Library page (https://dl.acm.org/doi/10.1145/42392.42399) returned 403. The Semantic Scholar PDF link returned content that was not the paper, and its landing page returned nothing readable. The Gale record returned nothing readable. The PMG publications page at pmg.csail.mit.edu was not approved for fetching. The pdos.csail.mit.edu copy at papers/liskov-argus.pdf served the 1983 "Guardians and Actions" paper instead, and nothing from it is used. Direct download of any copy failed at the proxy with 403 on CONNECT, so no PDF was converted locally and no page-level check of quotations was possible. The Argus reference manual was not attempted.

Consequences. The names SR, Ada, Mesa and Camelot as the paper's points of comparison, and the count of 20 references with seven by Liskov's group, come from the fetcher's summary and not from a verbatim passage. The word orphan does not appear in the paper as returned, and orphan detection is not discussed here. No claim is made about the applications beyond the one sentence each the paper gives them.

All Phient numbers are from PHIENT_FACTS.md and docs/86_liskov_three_operators.md in this repository. No number about Phient was taken from anywhere else, and none was computed for this paper.
