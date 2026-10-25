# Agreement among liars: a word as 3f+1 replicas, a channel as a view, meaning as a quorum

*Phient interpreted papers · Liskov series, 15 of 16 · 2026-08-10 · on: "Practical Byzantine Fault Tolerance" (Miguel Castro and Barbara Liskov, 1999, Proceedings of the Third Symposium on Operating Systems Design and Implementation, OSDI 99, New Orleans, February 1999)*

"In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition."

## 1. Our abstract

The paper is short and its claim is plain. A service can be replicated on 3f+1 machines so that it keeps working, and keeps giving correct answers, while f of them do anything at all, including lying, and while the network drops, delays, duplicates and reorders messages. Earlier algorithms with that goal assumed a synchronous network or were too slow to use. This one assumed neither. It ran a file system. On the Andrew benchmark the replicated file system took 3% more time than the unreplicated NFS that came with Digital Unix. That number is the paper's argument for the word "practical" in its title.

The mechanism is a three-phase agreement among replicas about the order in which to execute requests. A primary proposes an order. Backups confirm it to each other. Each replica executes once it has seen enough confirmations that no set of f liars could have produced them. When the primary itself fails or lies, the others change view and a new primary takes over, carrying forward everything the old view had already committed. The client trusts nothing until f+1 replicas say the same thing.

This paper reads that through Phient. The reading is not forced. A Phical word is 32–64 relations, each on its own channel, and no one of them is the meaning. Meaning is what enough of them agree on. That is a quorum. A channel is a coordinate, and a coordinate under a condition is a view; the primary of a view is picked by arithmetic, p = v mod |R|, and the condition c picks a channel the same way. The operator X ⇒ Y under c is a request, and the three operators better, worse and loss are three ways of counting how many relations back the answer. The deadband of the byte is Phient's f: the number of quanta a relation can be wrong by and still be counted as agreeing.

Sections 6 to 10 make that reading in order. Section 11 lists what Phient has measured that bears on it, all from the facts file. Section 12 proposes an experiment: corrupt f of a word's channels on purpose and ask whether the quest still reads. The control is the shipped attraction-only trainer at 0.594 MRR. Section 13 says where the analogy fails, and it fails at the paper's first assumption, that replicas are deterministic copies of one state; Phient's channels are deliberately not copies. Section 14 gives what is verified about the two authors and the moment. Section 16 lists what was reached and what was not; the paper's own PDF was read, its published HTML copy was not.

## 2. What the work claims, in its terms

The abstract is the claim, and it is worth taking one clause at a time. "This paper describes a new replication algorithm that is able to tolerate Byzantine faults." Byzantine faults, after Lamport, Shostak and Pease, are the ones where "faulty nodes may behave arbitrarily." The paper adds why this matters now: "malicious attacks and software errors are increasingly common and can cause faulty nodes to exhibit arbitrary behavior." Then the contrast: "Whereas previous algorithms assumed a synchronous system or were too slow to be used in practice, the algorithm described in this paper is practical: it works in asynchronous environments like the Internet and incorporates several important optimizations that improve the response time of previous algorithms by more than an order of magnitude." Then the evidence: "We implemented a Byzantine-fault-tolerant NFS service using our algorithm and measured its performance. The results show that our service is only 3% slower than a standard unreplicated NFS."

The introduction lists four contributions in its own words. It "describes the first state-machine replication protocol that correctly survives Byzantine faults in asynchronous networks." It "describes a number of important optimizations that allow the algorithm to perform well so that it can be used in real systems." It "describes the implementation of a Byzantine-fault-tolerant distributed file system." And it "provides experimental results that quantify the cost of the replication technique."

The system model is stated before the algorithm. The network "may fail to deliver messages, delay them, duplicate them, or deliver them out of order." Faulty nodes "may behave arbitrarily." Failures are independent. Replicas are state machines: "they must be deterministic and they must start in the same state." The algorithm "provides both safety and liveness provided at most" one third of the replicas "are simultaneously faulty." Safety means that "clients eventually receive replies to their requests and those replies are correct according to linearizability." Liveness needs one more assumption: that message delay "does not grow faster than" some bound "indefinitely." The paper calls this "a rather weak synchrony assumption" and says it "enables us to circumvent the impossibility result" of Fischer, Lynch and Paterson. The division of labour is one sentence: "The algorithm does not rely on synchrony to provide safety. Therefore, it must rely on synchrony to provide liveness."

The number 3f+1 is argued, not assumed. A replica "must be able to proceed after communicating with 2f+1 replicas, since f replicas might be faulty and not responding." But the silent ones might be honest, so "of those that responded, f might be faulty." The honest responders must still outnumber the liars, "i.e., 2f+1 > f," which needs 3f+1 in all.

## 3. The mechanism, plainly

In view v the primary is "replica p such that p = v mod |R|." The rest are backups. A client sends a request to the primary, which "assigns a sequence number, n, to the request, multicasts a pre-prepare message" with the view, the number and a digest "to all the backups, and appends the message to its log."

A backup accepts the pre-prepare if it is signed, in the current view, between the water marks, and the backup "hasn't already accepted a pre-prepare message for the current view and sequence number with a different digest." That condition is the defence against a primary that tells backups different things. The backup then "enters the prepare phase by multicasting a PREPARE message to all other replicas."

The first predicate is prepared. It holds when replica i has logged "the request m, a pre-prepare for m in view v with sequence number n, and 2f prepares from different backups that match the pre-prepare." The invariant follows: "if prepared(m,v,n,i) is true then prepared(m',v,n,j) is false for any non-faulty replica" j, because two sets of 2f+1 out of 3f+1 share an honest member. "The pre-prepare and prepare phases are used to totally order requests sent in the same view even when the primary is faulty."

"Replica i multicasts a COMMIT to the other replicas when prepared becomes true." Committed-local holds when prepared holds "and i has accepted 2f+1 commits (possibly including its own) from different replicas that match the pre-prepare." Then the replica executes. "The prepare and commit phases are used to ensure that requests that commit are totally ordered across views."

The client "waits for f+1 replies from different replicas with the same result," and if none come "broadcasts the request to all replicas," which starts the backups' timers.

Every 100 requests in the implementation, replicas checkpoint their state and multicast a digest. A checkpoint is stable when a replica holds "2f+1 of them for sequence number s with the same digest signed by different replicas," and it then discards all protocol messages at or below s. The low water mark h is the last stable checkpoint and "the high water mark H = h + k."

When a backup's timer expires it multicasts a view-change message for v+1 carrying its stable checkpoint proof and, for every request prepared above it, the pre-prepare and 2f prepares. The new primary, with 2f+1 such messages, finds "min-s of the latest stable checkpoint in V and the highest sequence number max-s in a prepare message in V," re-issues a pre-prepare for each number between them, and fills gaps with "a special null request." Backups check the new-view message and proceed.

## 4. What it demonstrated, as reported

"We implemented BFS, a Byzantine-fault-tolerant NFS service, using the replication library." The hardware: "DEC 3000/400 Alpha workstations" with "a 133 MHz Alpha 21064 processor, 128 MB of memory," running "Digital Unix version 4.0," each replica with its file system "on a DEC RZ26 disk," all "connected by a 10Mbit/s switched Ethernet." "All experiments ran with one client running two relay processes, and four replicas." Four replicas is f = 1, the smallest the algorithm allows.

Two measurements were reported. The micro-benchmark timed a null operation with arguments and results of 0 or 4 kilobytes. Read-write replicated operations cost 3.35 ms at 0/0 against 0.82 unreplicated, 309%; 14.19 ms at 4/0 against 4.62, 207%; 8.01 ms at 0/4 against 4.66, 72%. Read-only operations cost 1.62, 6.98 and 5.94 ms, overheads of 98%, 51% and 27%.

The second measurement is the one the abstract rests on. The Andrew benchmark "has five phases: (1) creates subdirectories recursively; (2) copies a source tree; (3) examines file status without data; (4) examines every byte; (5) compiles and links." BFS-strict is the algorithm as described; a second variant applies the read-only optimisation to lookup; "BFS-nr, which is identical to BFS but with no replication," is the third; "NFS-std, which is the NFS V2 implementation in Digital Unix," is the reference.

Against BFS-nr the cost of replication showed: 64.48 seconds for BFS-strict against 51.07, "BFS-strict takes only 26% more time" than the unreplicated version of itself, and 61.07 for the read-only variant, 20% more. Against NFS-std, at 62.52 seconds, the picture changed. The paper says the results show BFS can be used in practice, since "BFS-strict takes only 3% more time to run the complete benchmark." The read-only variant was 2% faster than NFS-std. By phase, BFS-strict was 69% faster on phase 1, 2% faster on phases 2 and 5, and 35% and 32% slower on phases 3 and 4.

The paper's explanation of how a replicated system beats an unreplicated one is not among the sentences I could verify, and I do not supply one. Two reported things bear on the cost. "MACs can be computed three orders of magnitude faster than digital signatures," 10.3 microseconds for a 64-byte message against 43 ms for a 1024-bit RSA signature on the paper's Pentium Pro. And "all other replicas send replies containing just the digest of the result."

Against Rampart and SecureRing the paper claims to be "more than an order of magnitude" faster, and notes that "they rely on synchrony assumptions." The conclusion adds a use the title does not advertise: "it can mask errors that occur independently at different replicas, including nondeterministic software errors."

## 5. The one idea restated

Truth is what a quorum says, and a quorum is sized so that the liars cannot be a quorum.

Everything in the algorithm serves that sentence. The number 3f+1 is the smallest population in which 2f+1 responders are guaranteed to contain a majority of honest ones even when the f silent replicas were honest and f of the loud ones were not. The prepare phase makes 2f+1 replicas write down the same order before anyone acts, and two such sets must share an honest member, so no two honest replicas ever act on different orders in the same view. The commit phase makes 2f+1 replicas write down that they saw the prepare quorum, so that when the view changes the new primary, gathering 2f+1 view-change messages, is guaranteed to hear from at least one honest replica that saw any request that was committed. Checkpoints are quorums too, 2f+1 matching digests, so that a replica may forget its log without forgetting anything the group agreed on. The client's f+1 matching replies are the smallest set that cannot be all liars.

Nothing in this asks any replica to be trusted. The primary is not trusted; the backups check that it never says two things under one sequence number. The backups are not trusted; each of their votes counts only inside a set too large to be forged. The client is not trusted by the replicas either; requests are signed and timestamped. The network is not trusted to deliver anything on time, and the algorithm's correctness does not depend on it doing so. Only its progress does, and only weakly: messages must eventually arrive within some bound, and the paper argues that any network whose faults are eventually repaired satisfies this.

The quorum arithmetic was known before this paper. What Castro and Liskov added is the demonstration that it could be run at the speed of a file system: no signatures in the common case, digests instead of full replies, tentative execution once prepared, read-only requests answered without ordering at all. The idea restated in the paper's own terms is that a service does not need honest machines. It needs enough machines that honesty can be counted, and a protocol in which every step is a count and no step is a belief.

## 6. The word as a function, and the word as 3f+1 replicas

In Phient a word is a function of its condition: give it c and it returns a phasor, one of 32–64. There is no place where the word's meaning is kept. This paper gives that a second name. A word is a replicated service. Each relation is a replica holding what the word means from one angle, under one condition. A request is a prediction: given this context, what comes next. In phinet the reply is never taken from one relation. The operator scores a candidate by cos(pred, a) over the channels together, which is a count of how many replicas agree that a is close.

The paper's client "waits for f+1 replies from different replicas with the same result." Phient's decoder waits for the candidate whose cosine is highest, which is the candidate most channels agree on. The paper counts replies and Phient sums phasors, but the structure is the same. No single channel decides. A channel that is wrong, by noise, by a bad example, or by the byte's rounding, is outvoted so long as the others are right. The quest shows this working and failing. Within-kind reading gives 0.747 MRR: the right channels agree. Wrong-channel reading gives 0.015, below the chance of 0.021: a reply from the wrong replica, taken alone, is worse than a guess. Offset reading gives 0.005. The paper would say that a reply from one replica is not a result.

The 3f+1 arithmetic lands on the channel count. If a word has 64 channels and meaning must survive f of them being arbitrary, 64 = 3f+1 gives f = 21. At 32 channels, f = 10. Phient has not measured f. It has measured something adjacent: the deadband. A nudge of 0.06·sin d falls under half a quantum once sin d < 0.2, about 11 degrees, and there a channel stops moving whether or not it should. A frozen channel is no liar. It is a replica that has stopped answering, and the paper counts those against f too: 2f+1 must proceed without the silent f. Nearest rounding froze every pair 8 quanta short, every replica silent at once, and the shipped quest trainer read 0.082 MRR. Dithered rounding, which keeps channels answering, read 0.594. That is the difference between a quorum that can form and one that cannot.

The paper's replicas "must be deterministic and they must start in the same state." Phient's channels are neither; section 13 returns to that. The reading holds where it matters: a word is a group, its meaning is what the group agrees on, and the operator is the client counting replies.

## 7. The relation channel as a coordinate, and the view

A channel is a coordinate: relation k of word w is a phasor at position k on a closed circle of 256 quanta, and c selects k. The paper has an object with exactly this shape. The view is a number v, and the primary of view v is "replica p such that p = v mod |R|." Nothing is chosen; the view number picks the replica by arithmetic, and every honest replica computes the same primary. Phient's condition c picks a channel the same way. The condition is the coordinate.

The paper's view is also the unit of agreement. Requests are "totally ordered" within a view by pre-prepare and prepare; ordering across views is what commit is for. Phient's channel is the unit of comparison. better(pred, a, b) is cos(pred, a) − cos(pred, b), and the cosine is taken channel by channel and summed. Two words are compared under one condition on one channel, and the sum across channels is agreement across views. The quest's kinds, is-a, part-of, near, used-for, unlike, are five conditions, and the oracle's rows are requests in five views. Within-kind reading at 0.747 is total order within a view. Two-hop is-a at 0.095 on held-out data, used-for at 0.093, and part-of, near and unlike at about chance, is ordering across views, and it mostly does not hold. The paper's commit phase exists because ordering across views is the hard case, and the quest agrees.

The view change is the part of the paper Phient has no counterpart for. When the primary of view v fails, backups time out and move to v+1 with a message carrying everything they prepared above their last stable checkpoint. The new primary re-issues those requests under the new view and fills gaps with a null request. Nothing committed is lost; nothing the old primary was still deciding is trusted. Phient changes condition freely, from is-a to part-of, and carries nothing forward. The two-hop numbers say what that costs: a relation established under one condition does not survive the change, and the paper's answer is that it must be re-proposed under the new view by whoever holds the proof.

The hexagram is a coordinate of the same kind at a coarser grain. hexagram_of takes the centroid phase of a word's 64-channel function and files it in one of 64 sectors. On the synthetic quest 256 words occupy 61 of 64: 64 views, four words in each on average, three empty. The paper would ask which channel led when each word was filed. Phient has not asked, and section 15 lists it.

## 8. The operator as X ⇒ Y under c, and the three phases

X ⇒ Y under c is the request. The context X is the client, the next word Y the result, c the view in which the request is served. The paper serves a request in three phases and the three operators of phinet::liskov fall onto them.

Pre-prepare is the prediction. The primary assigns a number and multicasts a digest, and the backups check only that they have not accepted a different digest under that number. In phinet the trainer proposes pred, the phasor the context points at, and no comparison has happened yet. The prediction is a proposal of order.

Prepare is better. A replica is prepared once it holds "2f prepares from different backups that match." better(pred, a, b) is cos(pred, a) − cos(pred, b): the weight of channels on which a matches the proposal minus the weight on which b does. The hinge trains that difference and the MRR reads it. The Austen numbers are prepare quorums forming: identity 7.9× chance, hinge 14.7×/14.6× at 8,360 params, softmax β=8 18.0×/18.8×, wide+deep 30.7×/30.3× at 8 epochs, hit@10 0.348. The paper's invariant, that no two requests prepare under one number, is the antisymmetry test of phinet::liskov: better(pred, a, b) = −better(pred, b, a).

Commit is loss. A replica commits on 2f+1 commits, the margin by which the group knows that the group knows. loss(pred, target, decoy, margin) is max(0, margin − better): zero when the truth beats the decoy by the margin, positive otherwise. On the quest, mean loss is 0.108 at margin 0.1 and mean better is −0.008. The truth ties the best wrong word on almost every row. A replica in that state is prepared for two requests at once, which the invariant says cannot happen among honest replicas. Phient's channels are not honest in that sense; they are bytes, and the byte ceiling is where two candidates prepare under one number.

worse is the view change. worse(pred, a, after, continuations) is cos(pred, a) − max over t of cos(after, t): how far the best continuation after a falls below a's own fit. A word that leads nowhere is refuted, a backup's timer expiring. forcing_choice ranks by cos − λ·worse. On text it does not help: KN 66.9 greedy, 69.7 at λ = 0.05, 68.5 at 0.25, 107.4 at 1. Every word in text has some continuation, so the timer never expires, and a view change that fires anyway costs perplexity. docs/86 says the place worse should bite is the quest, where a word either has onward relations or does not. The paper would agree: view changes are for a silent primary, not a slow one.

## 9. Fault tolerance as a claim about language

The paper's claim, made about machines, is that correctness does not need trust. It needs numbers: 3f+1 participants, 2f+1 to proceed, f+1 to believe. As a claim about language: a word's meaning does not need any one of its uses to be right. It needs enough uses that the wrong ones, from noise, error or intent, cannot form a majority on any channel.

That is stronger than statistics smoothing noise, because the paper's faults are arbitrary. A Byzantine replica is adversarial. It may say one thing to one backup and another to a second, and the backups' answer is to refuse to prepare two things under one number. In a corpus the adversary is real. Irony says the opposite of what it means; a persona borrows another's words; a decoy is chosen because it is close. Persona attribution, 71% overall on held-out 4-sentence paragraphs against 33% chance, 74 for Chesterton, 65 for Austen and 86 for Carroll, is a quorum among three writers. With three replicas f = 0, and the paper would say the attribution is not fault tolerant: it can be right, but one lying writer would break it. Dialogue at 44% by the sector reader and 72% by the Kneser-Ney panel is the same vote with two clients counting.

The deadband is where the claim about language becomes a claim about bytes. A channel is one byte, 256 quanta of 2π. The training nudge is 0.06·sin d, and once sin d < 0.2 that is under half a quantum and rounds to nothing. The paper's votes have no grain. Phient's do, and below it a channel cannot change its vote. That is not a Byzantine fault. It is a replica that has stopped, and the paper counts stopped replicas against f as it counts liars. Nearest rounding stopped every replica 8 quanta short. Dither restarted them and took the quest from 0.082 to 0.594 MRR. But dither costs corpus attribution, 71% to 65%, so it ships off by default. That is the paper's trade between liveness and safety, with safety first. Phient chose the same order without knowing it was choosing.

The checkpoint is the count. Every 100 requests the paper's replicas agree on a digest of their state and discard the log below it. The count on each relation is the only state in Phient that survives the log of training examples. The helix is a stable checkpoint made explicit: 18,496 words as codons in 1.9 s, 75 MB, byte-exact on the round trip. Byte-exact is what a digest requires. The paper needs 2f+1 replicas to sign one digest; Phient has one writer and one reader.

## 10. Where it lands: phinet, the quest, the helix, the three operators

In phinet the landing is the tie. A Tie makes one word's channels answer together, and the paper's question is how many must agree before an answer counts. The circle tie has 8,360 params and reads Austen at 14.7×/14.6×. The sphere tie, one complex weight per spherical-harmonic degree with 64 = 8² channels as Y_l^m for l < 8, has 1,192 params and reads 12.8×/13.1× in 89 s. The anisotropic tie, one weight per (l, |m|) in 36 slots, 4,776 params, reads 16.0×/15.6× in 85 s and beats the circle. The paper's arithmetic offers one reading: a tie that makes channels vote in degree groups forms quorums more easily. Section 12 tests it.

In the quest the landing is the oracle itself: 256 words in families of 8, five kinds, 1,280 rows, 293 target words. The paper's client waits for f+1 matching replies. The quest's hit@5 at 97.4% says the truth is nearly always among the five nearest, and hit@1 at 34.8% says the quorum for first place forms about a third of the time. The gap between them is the paper's gap between prepared and committed. Enough channels agree that the truth is close; not enough agree that it is closest.

In the helix the landing is the codon. 64 codons as base-4 ACGT of the channel, the hexagram, the sphere degree and order, B-DNA xyz at 10.5 codons per turn and 0.34 nm rise, a PhiTon wavelength, band and hex per codon. This is the paper's checkpoint: the state, digested, storable and transferable. The paper's replica "can obtain a missing checkpoint state from one of the replicas whose checkpoint messages certified its correctness." Phient's helix is written by one process and read back by one. The certification step does not exist yet.

In the three operators the landing is section 8's. better is prepare, loss is commit, worse is the view change, and forcing_choice is the protocol run once per candidate. The measured facts are that forcing does not beat greedy on text at any λ, and that the quest scored by the three operators is flat, mean better −0.008, mean worse −0.008, mean loss 0.108, over 61 of 64 hexagrams. The paper's reading of flat is not that the operators are wrong. It is that the population sits at the byte ceiling, where every candidate prepares and none commits.

In the persona work, three writers is fewer than 3f+1 for any f above zero. The vote works, at 71%, without being fault tolerant. The paper does not say a three-replica service is useless. It says a fourth replica is the price of tolerating one.

## 11. What Phient has measured that bears on it

Everything here is from the facts file. Nothing else is claimed.

| in the paper | what Phient measured | number |
|---|---|---|
| f+1 matching replies | Austen hinge, 3 epochs, 8,360 params, valid / test | 14.7× / 14.6× chance (chance MRR 0.0063) |
| a larger quorum | softmax wide+deep, 50,016 params, 8 epochs | 30.7× / 30.3×, hit@10 0.348 |
| three replica groupings | circle / sphere / anisotropic tie, hinge 3 epochs | 14.7×/14.6× at 8,360; 12.8×/13.1× at 1,192; 16.0×/15.6× at 4,776 |
| reply from the wrong replica | quest within-kind / wrong channel / offset | 0.747 / 0.015 / 0.005 (chance 0.021) |
| prepared but not committed | quest hit@5 / hit@1 | 97.4% / 34.8% |
| silent replicas, restarted | quest MRR, nearest rounding vs dithered | 0.082 → 0.594 |
| safety over liveness | corpus attribution with dither | 71% → 65%, so dither ships off |
| view change on text | forcing λ = 0 / 0.05 / 0.25 / 1, KN | 66.9 / 69.7 / 68.5 / 107.4 |
| ordering across views | held-out two hops, is-a / used-for | 0.095 / 0.093; part-of, near, unlike ≈ chance |
| checkpoint, byte-exact | helix, 18,496 words | 1.9 s, 75 MB, round trip exact |
| three replicas, f = 0 | persona attribution, chance 33% | 71% (74/65/86) |
| the reference implementation | phinet vs PyTorch | 62/62 parity tests |

The three-tie row is the one section 12 tests. The quest hit rows are the closest Phient has to the paper's two predicates. The dither rows are the safety-liveness trade measured twice. The forcing row is the view change fired where the primary never goes silent. The two-hop row is the commit phase not holding.

What is not measured is longer. No run has corrupted channels on purpose, so there is no measured f. No run has counted, per candidate, how many channels voted for it; the cosine is a sum and the count has not been separated from the magnitude. No run has asked whether two trainings produce the same helix bytes, the certification the paper requires. The engine is allocation and bandwidth bound at these sizes, 1 MB op in 2–5 ms, so no timing in Phient corresponds to the paper's 3%. The sphere at softmax and wide+deep is unrun. No Phi-4 run exists; the quest is a synthetic oracle only. The decoding numbers, sampler KN 13.3, nearest-k 66.9 (was 97.2), union 66.8, shared 4-grams 25.7% and 8.7%, do not bear on the paper.

## 12. The experiment

The paper's claim reduced to one testable sentence: a service on 3f+1 replicas returns correct results while f of them return arbitrary ones. Phient can test the shape of that claim inside phinet because a word is a group of channels and the quest has a ground truth.

The experiment is deliberate corruption. Take the quest oracle as the facts file states it: 256 words, families of 8, five kinds, 1,280 rows, 293 target words, chance MRR 0.021. Train with attraction-only and dithered rounding, the shipped best. Then, at evaluation and not at training, replace f of each target word's 64 channels with arbitrary phasors, uniformly random on the circle, and read the quest. Do it for f = 0, 1, 2, 4, 8, 16, 21 and 32, recording MRR, hit@1 and hit@5. Repeat adversarially: set each corrupted channel to the phasor of the best wrong word, the paper's liar rather than its noise.

The control is the uncorrupted read at 0.594 MRR, hit@1 34.8%, hit@5 97.4%. That is f = 0 and it is in the facts file.

If a word behaves like a 3f+1 service, MRR should hold near 0.594 until f approaches 21, one third of 64, and fall sharply after. If it falls linearly from f = 1, the channels do not vote; they average, and section 6 is wrong for Phient. If it holds under random corruption and fails under adversarial corruption at a much smaller f, the channels vote but there is no prepare phase: nothing checks that the same wrong word was proposed to every channel, and a lying word can make a quorum. That last outcome would say that better is prepare without the invariant.

A second arm runs the same corruption against the three ties on Austen. The control is the circle at 14.7×/14.6×. If the anisotropic tie holds its 16.0× to a larger f than the circle holds 14.7×, section 10's guess is measured. If the sphere holds longest of the three, the grouping and not the parameter count is what tolerates faults.

A third arm tests the checkpoint. Train the same corpus twice from different seeds and write both to the helix. The control is the byte-exact round trip of one training. Count the codons that differ. The paper needs 2f+1 replicas to sign one digest; two trainings that disagree on most codons could sign nothing, and the helix would be a log, not a checkpoint.

None has been run. The first is cheap. The second costs 85 to 142 s per Austen run per f per tie. The third costs two trainings and a diff.

## 13. Where we disagree or the analogy breaks

The paper's replicas "must be deterministic and they must start in the same state." That is false of Phient on purpose. A word's 64 channels are not 64 copies of one meaning. They are 64 different relations, one per condition, and the design wants them to differ. The paper tolerates f arbitrary replicas because the other 2f+1 are identical. When a Phient channel is wrong no other channel holds what it should have held. This is the break. A quorum among channels is a quorum among witnesses to different events, and the paper's arithmetic does not cover that case.

The second break is linearizability. The paper's safety is that replies are "correct according to linearizability": the replicated service behaves as one sequential machine would. Phient has no sequential machine to compare against. There is no unreplicated Austen. The ground truth on the quest is the oracle's table, on Austen the held-out text, and neither is a specification, only a record of what the word returned once. The paper's correctness is against a spec; Phient's is against a sample. That is why the paper can say 3% and Phient can only say 14.7×.

The third is the adversary. The paper's faults are arbitrary because its motivation is attack. Nothing in Austen is attacking phinet. Hinge decoys are chosen to be hard, but the trainer chooses them, not an enemy. The persona work is closer: a writer who imitates another is a replica reporting a state not its own. But the imitation is the point, not a fault. The paper's lying replica has no place in a system whose job is sometimes to lie well.

The fourth is time. The paper's liveness rests on a bound on message delay, and its view change is triggered by a timer. Phient has no clock in the operator. worse measures a dead end in relation space, not a delay in time, and section 8's reading of worse as a view change is a reading of shape, not mechanism. Forcing on text fires the view change on every candidate, as a protocol with no timer would.

The fifth is the one I would press. The paper's 3f+1 is derived from silent and lying responders. Phient's channel count, 32–64, comes from the byte, the hexagram, the sphere with l < 8 and the codon, and from nothing in that argument. If section 12's experiment shows that meaning survives about a third of channels being arbitrary, that is a coincidence the design did not plan. If it shows something else, the arithmetic was never Phient's, and section 6 is a name and not a law.

## 14. The person and the moment

Barbara Liskov was born Barbara Huberman. She took her PhD at Stanford in 1968 under John McCarthy, with a thesis on chess endgames played by a forcing tree and two comparison functions, better and worse. She worked at MITRE and then joined MIT. The Turing Award came in 2008.

The paper's affiliation line reads "Laboratory for Computer Science, Massachusetts Institute of Technology, 545 Technology Square, Cambridge, MA 02139." The paper is dated February 1999 and was presented at the Third Symposium on Operating Systems Design and Implementation in New Orleans in that month.

Miguel Castro is the first author. The acknowledgements say he "received a PRAXIS XXI fellowship," and the research "was supported by DARPA contracts" monitored by the Army at Fort Huachuca and the Air Force Research Laboratory, and by NEC. The authors thank, among others, Atul Adya, Chandrasekhar Boyapati and Nancy Lynch, and the anonymous referees. I have not verified from any source reached what Castro's position was at the time beyond the fellowship, and I do not state it. The brief for this series places practical Byzantine fault tolerance after viewstamped replication in Liskov's work, and the lineage from one to the other is plausible on the face of the mechanism, view numbers and a primary chosen by arithmetic, but the paper's own statement of that lineage is not among the sentences I could verify and I do not claim it.

What the moment was is in the paper's first paragraph. "Malicious attacks and software errors are increasingly common. The growing reliance of industry and government on online information services makes malicious attacks more attractive and makes the consequences of successful attacks more serious." That was written when the Internet was a decade old as a public thing and the assumption of the older agreement literature, that the network is synchronous, had stopped being true of any network anyone used. The paper's phrase for its own advance is that it is "the first to work correctly in an asynchronous system like the Internet." The word "practical" in the title is a claim against Rampart and SecureRing, which the paper says were "more than an order of magnitude slower."

One thing about the moment is easy to get wrong. The 3% is against NFS-std on the Andrew benchmark in total, and it is the number the abstract chooses. The same paper reports 26% against its own unreplicated code and 309% on the zero-byte micro-benchmark. All three are in the paper and all three are true. The abstract chose the one that answers the title.

## 15. What remains

For Phient, five things, in the order they could be done.

Run the corruption curve. Section 12's first arm. It is the cheapest measurement that would say whether a word as a replica group is a law or a name, and it uses the quest oracle and the shipped trainer.

Separate the count from the sum. The cosine sums phasors across channels and the sum carries both how many channels agree and how strongly. The paper counts replies and ignores their strength. A reader that counts channels above a threshold, alongside the cosine, would give Phient its first f+1, and the quest's gap between hit@5 at 97.4% and hit@1 at 34.8% is where to look first.

Give the hexagram a primary. Section 7 noted that hexagram_of files a word by centroid phase without saying which channel led. The paper's primary is p = v mod |R|. A filing that records which channel had the largest magnitude when the sector was chosen would let the flat table of docs/86 be read by view, and 61 of 64 might stop being flat once the empty three are known to be empty for a reason.

Certify the helix. Section 12's third arm. Two trainings, one diff. Until that number exists the helix is one replica's log and not a checkpoint, and the byte-exact round trip proves only that the writer and reader agree with each other.

Run forcing at depth two on the quest. docs/86 asked for it and section 8 explained why a view change belongs where the primary goes silent. Text has no silent primaries. The quest does.

A sixth is the one section 13 ended on. The channel count of 32–64 was not chosen by the paper's arithmetic. If the corruption curve breaks near a third, the count should be reconsidered as 3f+1 for some f that Phient wants, and 64 = 3·21+1 is a fact about integers that the design has not yet claimed. If it breaks elsewhere, the paper stops being a description of Phient and stays a description of what Phient is not.

For the reading itself, two things remain. The paper's own explanation of why the read-only variant beat NFS-std, and of the phase 1 result where BFS-strict was 69% faster than the unreplicated reference, is in the paper and was not among the sentences returned to me; section 4 says so and gives no explanation. And the view-change message's two sets are described in section 3 by function rather than by letter, because the two fetches that returned them disagreed on which letter named which, and I preferred to state what the sets hold over guessing their names.

## 16. Sources

Reached and read, through WebFetch on 2026-08-10.

Miguel Castro and Barbara Liskov, "Practical Byzantine Fault Tolerance," the PDF as hosted by MIT's 6.824 course (https://css.csail.mit.edu/6.824/2014/papers/castro-practicalbft.pdf). Read in three passes. Every quotation in sections 2, 3, 4, 5 and 14 is from this document: the abstract, the four contributions, the network and failure model, the determinism sentence, the 3f+1 argument, the safety and liveness sentences, the delay assumption and the Fischer-Lynch-Paterson remark, the primary rule, the pre-prepare acceptance conditions, the prepared and committed-local predicates, the invariant, the purposes of the two phase pairs, the client's f+1 rule, the checkpoint and water-mark sentences, the min-s and max-s sentence and the null request, the optimisations, the MAC timing example, the hardware and configuration sentences, the micro-benchmark caption and numbers, the Andrew benchmark phases, the definitions of BFS-strict, BFS-nr and NFS-std, Tables 2 and 3 with their percentages, the 26% and 3% sentences, the Rampart and SecureRing sentence, the conclusion's two claims, the affiliation line, the February 1999 date, and the acknowledgements.

USENIX, conference record for OSDI 99, "Practical Byzantine Fault Tolerance" (https://www.usenix.org/conference/osdi-99/practical-byzantine-fault-tolerance). Confirms authors, affiliation, the venue as the 3rd Symposium on Operating Systems Design and Implementation, New Orleans, February 1999, and the abstract verbatim.

Adrian Colyer, "Practical Byzantine Fault Tolerance," the morning paper, 18 May 2015 (https://blog.acolyer.org/2015/05/18/practical-byzantine-fault-tolerance/). Used only to cross-check the phase purposes, the f+1 reply rule, the client's broadcast on timeout and the pre-prepare acceptance conditions against the PDF. No number was taken from it.

Not reached. The authors' HTML copy at pmg.csail.mit.edu (http://pmg.csail.mit.edu/~castro/osdi99_html/osdi99.html) and the PDF at pmg.csail.mit.edu/papers/osdi99.pdf were refused by the network egress policy. The Berkeley copy (people.eecs.berkeley.edu, BFT-osdi99x2.pdf) returned lecture slides rather than the paper. The CMU copy (cs.cmu.edu, Castro99lecture.pdf) was disallowed by robots. The ACM Digital Library and Semantic Scholar were refused at the proxy. Wikipedia was not consulted.

Consequences. The names of the two sets in the view-change message are not given because two extractions from the same PDF disagreed on them; their contents are described. The paper's explanation of the read-only variant outrunning NFS-std, and of the phase 1 speedup, was not returned and is not supplied. Castro's position in 1999 beyond the PRAXIS XXI fellowship is not stated. The relation of this paper to viewstamped replication is not claimed from the paper. The checkpoint interval of 100 requests is taken from the first extraction's paraphrase of the implementation section and is the one number in this paper about her work that was not returned as a verbatim sentence. All Phient numbers are from PHIENT_FACTS.md and docs/86_liskov_three_operators.md in this repository. No number about Phient was taken from anywhere else, and none was computed for this paper.
