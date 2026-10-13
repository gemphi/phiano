# The typed placeholder: a call that returns before it answers

*Phient interpreted papers · Liskov series, 11 of 16 · 2026-06-01 · on: "Promises: Linguistic Support for Efficient Asynchronous Procedure Calls in Distributed Systems" (Barbara Liskov and Liuba Shrira, 1988, Proceedings of the ACM SIGPLAN 1988 Conference on Programming Language Design and Implementation, Atlanta, pp. 260–267)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the eleventh of sixteen papers in which Phient reads one work by Barbara Liskov through its own paradigm. The series has followed her from the chess endgame with its two comparison functions, through Venus, CLU and the abstract data type, into Argus and the guardian. This paper takes the small paper she wrote with Liuba Shrira in 1988, eight pages at PLDI in Atlanta, which gave the word "promise" its present meaning in programming.

The paper's claim is narrow and the mechanism is simple. A remote procedure call makes the caller wait. A message-passing program does not wait, and runs faster, and is harder to write. Liskov and Shrira wanted the speed of the second with the shape of the first. Their answer has two parts. A call-stream is a connection between a sender and a receiver over which calls are delivered in order and replies come back in order. A promise is a typed placeholder, created at the moment a stream call is made, which is blocked until the reply arrives and ready after. The caller goes on. When it needs the result it claims the promise, and the claim either returns the value or signals the exception the call raised. That is the design. The rest of the paper is about composing streams so that the output of one becomes the input of the next.

We read it here because Phient's operator is a call that returns before it answers. Applying the operator to a word does not give the next word. It gives a prediction, a phasor on each of 32 or 64 channels, and the next word is found later by comparing that prediction against candidates. The prediction is a placeholder. The comparison is the claim. Sections 6 to 10 argue that a Phical word is a set of typed promises, one per channel, that the decoder is a call-stream, that the quest's two-hop reading is a composition of streams in the paper's sense, and that the byte deadband is a promise that becomes ready with the wrong exception.

Sections 2 to 5 state what the paper says in its own terms. Section 11 lists what Phient has measured, from the facts file only. Section 12 names one experiment and its control. Section 13 says where the analogy breaks: a promise holds a value and Phical holds no point.

One limit is stated at the top. Phient runs in one process on one machine. It has no streams, no remote calls and no distribution. Nothing here is a result. It is a reading.

## 2. What the work claims, in its terms

The abstract is four sentences and it is worth having whole. "This paper deals with the integration of an efficient asynchronous remote procedure call mechanism into a programming language. It describes a new data type called a promise that was designed to support asynchronous calls. Promises allow a caller to run in parallel with a call and to pick up the results of the call, including any exceptions it raises, in a convenient and type-safe manner. The paper also discusses efficient composition of sequences of asynchronous calls to different locations in a network."

The introduction states the problem as a cost. "Remote calls require the caller to wait for a reply before continuing, and therefore can lead to lower performance than explicit message exchange." The alternative, which the paper calls a stream, lets a caller make several calls without waiting for replies. The paper then asks the questions that a language must answer if it is to let a program do this. "How can this be indicated? How can the results of the two calls be picked up without error or confusion? What happens if one of the calls signals an exception? Finally, suppose a communication problem makes it impossible to complete one of the calls; how is this indicated?" And it says what the answer is for. "Promises were invented to answer these questions in a way that preserves the merits of organizing programs using procedures and procedure calls without sacrificing the performance benefits of streams."

The paper has five sections: introduction, call-streams, promises, composing streams, and conclusions. The call-stream section defines the stream and its guarantees, and places the work inside Argus. "Argus provides active entities called guardians, each of which resides entirely at a single node of a network. Each guardian provides operations called handlers that can be called by other guardians." The promises section is the language part. It defines the type, its two states and its two operations, gives an example that records grades and prints averages, and defines a local fork. Its discussion compares promises with the futures of MultiLisp. "The design of promises was influenced by the future mechanism of MultiLisp."

The composing section takes up what happens when a program uses more than one stream, and the results of calls on one are the arguments of calls on the next. It offers forks and then a coenter statement, and it prefers the coenter. The conclusion opens with the purpose. "Streams were invented to improve the performance of distributed programs." It closes on structure. "Although forks can permit composition of streams, better structured programs result from a mechanism like the coenter, which also handles process termination properly."

## 3. The mechanism, plainly

A stream connects two parties. "Two entities, a sender and a receiver, can be connected by a stream. The sender can make calls to the receiver over the stream." Streams "guarantee exactly-once, ordered delivery of requests: Each call request or reply is delivered to the user code exactly once, the request for call n + 1 is delivered to the user code at the receiver only after the request for call n has been delivered to it, and the reply to call n + 1 is delivered to the user code at the sender only after the reply to call n has been delivered to it." The receiver's code must run the calls so that they appear to happen in call order.

Three kinds of call run over a stream. An ordinary remote call, where the caller waits. A stream call, where the caller may make more calls before the reply arrives. And a send, "like stream calls except that the sender is interested in the reply only when the call terminates abnormally."

A stream can break. "When the system at the sending side breaks a stream, this means that any calls whose replies have not yet been received will never have replies." The language must then "cause the calls to terminate with an exception." In Argus these are the two standard ones. "Both exceptions have a string argument that explains the reason for the failure, e.g., failure("handler does not exist"), or unavailable("cannot communicate")."

The promise is the handle on a stream call. "A promise is a place holder for a value that will exist in the future. It is created at the time a call is made." The example type is `pt = promise returns (real) signals (foo)`, and a stream call has the form `x: pt := stream h(3)` where h is a handler. "A promise object is in one of two states: blocked or ready." It is created blocked and becomes ready when the reply arrives. If the stream breaks first, "the system changes the promise to the ready state with an appropriate value, e.g., unavailable("could not communicate")."

Two operations read it. "The claim operation waits until the promise is ready. Then it returns normally if the call terminated normally, and otherwise it signals the appropriate exception." The ready operation "returns true if the promise is ready and false if it is blocked." "A promise can be claimed multiple times; the same outcome will occur each time." Promises stay local: "promises are not legal as arguments or results" of handler calls. A local fork gives the same type to a local call, whose "results are stored in the promise, which then becomes claimable."

## 4. What it demonstrated, as reported

The paper is a design paper. It reports no timings and no message counts, and we state none. It demonstrates by example and by argument.

The example in section 3.1 is a grades program. Grades are recorded by stream calls to a database. Each call returns the student's updated average, and the program stores each promise in an array. When all the calls have been issued, it claims the promises and streams the averages to a printer in alphabetical order. The program issues many calls before it needs any reply, and takes the replies in an order of its own choosing. The array of promises is what lets the two orders differ.

The argument in section 3.3 is against the futures of MultiLisp, in two parts. "First, they are inefficient to implement unless specialized hardware is available, since every object must be examined each time it is accessed to determine whether or not it is a future." A promise is a distinct type, so the check is done once, by the compiler. "Promises are strongly typed and thus avoid the need for runtime checking to distinguish them from ordinary values." The second part is about exceptions. "In MultiLisp, exceptions are turned into error values automatically, and information about the error value propagates through the expression that caused the future to be claimed and then through surrounding expressions." A promise makes the claim a place in the program, and the exception is raised there. The paper concedes the point: "Promises force all programs to be structured like this, so the structure of the program using promises will be identical to one using futures when safe exception handling is a concern."

The composition section shows a three-stage pipeline, read, compute and write, whose idea is to "pipeline the requests for data from the read stream into the compute stream." It generalises. "A composition can have an arbitrary number of levels; in each case, the output of the ith stream becomes the input of the i + 1 st stream, possibly with some local computation done along the way." One way to build this is with forks: "the main program simply forks two processes, one running each of these procedures." The other is the coenter. "A coenter statement contains a number of arms, each defining a computation to be run as a process." "The process executing the coenter is halted, and remains halted until all the subprocesses complete."

The efficiency claim is stated once. "Buffering allows us to amortize the overhead of kernel calls and the transmission delays for messages over several calls." The paper places the work in a "project in heterogeneous computing," citing the Mercury memo, and reports no running measurement.

## 5. The one idea restated

Strip the streams away and one idea is left. A call has two moments, the moment it is made and the moment its result is wanted, and a language should let those be different moments. What connects them is a value of a named type, created at the first moment, filled at the second, and read only by a program that asks for it by name. The type says what the call may return and what it may signal. The read either gives the value or raises the signal. Nothing else about the call is visible.

Everything else follows. The two states follow because between the two moments there must be a way to say the result is not here yet. The explicit claim follows because a placeholder that is read implicitly, like a MultiLisp future, has to be checked on every access, and the check costs. The exceptions-on-claim rule follows because the caller has moved on, so the only place an exception can land is where the caller comes back. The stream follows because if many calls are outstanding at once, something must say which reply is which, and order is the cheapest such thing. The two standard exceptions follow because a call whose reply will never come must still be claimable, and what it yields must be a value of the type.

The idea belongs to the same family as everything else in this series. In CLU a procedure is what its heading says it returns and signals. In the 1979 exception paper a procedure has several named ways of ending. Here the heading is lifted off the procedure and made into a value that can be held. A promise is a procedure's specification, minus its arguments, waiting to be met. That is why the paper says the promise type "mirrors" the handler type without its argument list. Liskov's constant concern is that a program should say what it promises and be held to it. In 1988 she made the promise a thing with a type.

Phient reads this as a claim about what an answer is. A word in Phical is a function that relates to other words under conditions. The operator applied to it does not give a word back. It gives a phasor per channel, which is a description of the word that would satisfy the relation, and the word itself is found later by comparison. The phasor is the placeholder. The comparison is the claim. The channel is the type. That is the idea the next five sections carry across.

## 6. The word as a set of typed promises

In Phient a word stores no point. It is 32 or 64 relations, each a complex phasor on a closed circle with a count. Read as a function, the word maps a condition c to the words it relates to under c. Read through this paper, each channel is a promise. It is a placeholder of a named type, the kind of relation, created when the word enters the lexicon, and it is filled, in the sense of having a phase that means something, only as pairs are seen under that kind. A channel with a count of zero is a promise still blocked. A channel with a high count is a promise that has been claimed many times and, in the paper's words, gives "the same outcome" each time.

The quest is where the types are visible. The phiecs oracle has 256 words in families of 8 and five kinds of relation: is-a, part-of, near, used-for and unlike. Each kind is a channel, so each word has five typed promises about its family. The trainer with attraction only and dithered rounding claims them at 0.594 MRR against chance 0.021, with hit@1 34.8% and hit@5 97.4%. Within a kind, 0.747. On the wrong channel, 0.015. The wrong-channel number is the typed promise doing its job. A promise of type `returns (real)` cannot be claimed as an integer, and the compiler says so. A quest word asked under a kind it was not trained on gives chance, which is Phient's way of saying the type does not match. Phient says it with a number rather than a refusal. Section 10 takes that up.

The Austen operator has the same shape with no names. The pool is 1,228 words seen 20 or more times, trained on 204,592 examples. The wide and deep softmax operator at 50,016 params reaches 30.7× chance MRR on validation and 30.3× on test at 8 epochs, with hit@10 0.348. The channels there were never typed by a kind. They are 64 promises whose types were never declared, filled by whatever pairs the corpus supplied. That the operator reaches 30× chance says the channels hold something. That no one can say what type each holds says the heading was never written.

The helix filing is the nearest thing to the heading. Each of 64 channels is a codon, base-4 ACGT, with a hexagram, a sphere degree and order, and a PhiTon wavelength and band. The lexicon of 18,496 words is written that way in 1.9 s and 75 MB and reads back byte-exact. Read through this paper, that file is an array of promises, one word per row, sixty-four typed placeholders per word, most still blocked.

## 7. The relation channel as a coordinate, and the call-stream

The paper's stream is a guarantee about order. Call n + 1 is delivered after call n. Reply n + 1 is delivered after reply n. The paper says the receiver's code must make the calls "appear to happen in call order." What the stream gives the language is a coordinate, the position of a call in the sequence, and it is that coordinate which lets a reply be matched to its call without the program saying anything.

Phient's decoder is a stream in exactly that sense. A prompt is a context. The operator is applied, a prediction is compared to candidates, the winner is appended, and the operator is applied again. Over 40 held-out prompts × 30 tokens that is a stream of 30 calls per prompt, each delivered after the last, each reply appended in order. The nearest-k decoder scores 66.9 Kneser-Ney perplexity on that stream, down from 97.2, and the union arm 66.8. The trigram sampler scores 13.3. The position of a token in the stream is the coordinate the next call is given, and it is the only one.

The second coordinate is the channel, and it is the one the paper's stream does not have. A stream carries calls of one handler type, or of many, but each call names its handler. A Phical application does not name a channel. It returns all 64 at once and lets the comparison decide which ones matter. So the decoder is a stream of calls whose types are decided after the reply. That is not something the paper permits, and section 13 returns to it.

The quest's channel is a coordinate in the paper's sense. The kind is named in the query, and the reply is read on that channel. Within-kind 0.747, wrong channel 0.015, offset reading 0.005. The offset number is the one that speaks to the stream. Reading the reply one channel over from where the call was made gives chance. That is a reply delivered out of order, matched to the wrong call. The stream's whole purpose is to make that impossible, and on the quest it is impossible for the same reason: the coordinate is part of the call.

The byte gives the coordinate its resolution. A channel is one byte, 256 quanta of 2π. A phase is one of 256 positions on a circle, and a hexagram is one of 64 sectors of it. The quest's 256 words occupy 61 of the 64. Sixty-one positions on the stream, and on the synthetic oracle a flat table: mean better −0.008, worse −0.008, loss 0.108 with margin 0.1. The stream is ordered. The order says nothing yet, because the oracle was built flat.

## 8. The operator as X ⇒ Y under c, and the stream call

The paradigm sentence says the operator drives a pair of words under a condition. X ⇒ Y under c. In the paper's terms, X is the sender, the operator is the stream call, and Y is what the promise holds when it becomes ready. The condition c is the type of the promise, what the call returns and what it may signal.

The paper's distinction between a stream call and a claim is one Phient already makes and has not named. Applying the operator to a context is the stream call. It costs one pass through the engine, a 1 MB op at 2–5 ms. It gives back a prediction, a phasor on each channel, and no word. Comparing that prediction to the candidates by cosine is the claim. It is where the value is read and where the exception, if there is one, lands. The second can be done many times on one prediction, which is the paper's "the same outcome will occur each time."

The paper's three kinds of call are three decoders. The ordinary remote call, where the caller waits, is the greedy nearest-k decoder: apply, claim, append, apply again. The stream call is the forcing decoder of docs/86. `nearest_forcing` takes the 2k nearest words, appends each to the context, applies the operator once more for each, and only then claims, ranking by cos − λ·worse. That is 2k stream calls outstanding and one claim over all. At k = 32 it scores 69.7 at λ = 0.05, 68.5 at λ = 0.25, and 107.4 at λ = 1, against 66.9 for the ordinary call. Forcing does not beat greedy on text. The paper would say that is a stream with no latency to hide. The reply is immediate, and issuing 64 calls at once buys no overlap, only a different ranking.

The send is the third kind, where the sender wants the reply only on abnormal termination. The trainer's hinge is a send. The trainer applies the operator to a pair and is interested in the reply only through loss, `max(0, margin − better)`. A reply that clears the margin is a normal termination and the trainer moves on. A reply that does not is abnormal, and the trainer reads it. That the mean loss on the quest is 0.108, the margin itself, says almost every send came back abnormal.

The evaluator at λ = 1 is the one number that runs the other way: 0.712 with distinct 0.777 and shared 4-grams 22.3%. The stream call bought a reply the evaluator liked and the perplexity did not. The paper does not adjudicate between two judges of one reply, and neither does this one.

## 9. Composing streams, and the two-hop quest

Section 4 of the paper is about what happens when the result of a call on one stream is the argument of a call on the next. "In each case, the output of the ith stream becomes the input of the i + 1 st stream, possibly with some local computation done along the way." The paper's example is a pipeline: read, compute, write. The difficulty is that promises from the first stream must be claimed before calls on the second can be made, while both streams are kept full.

The quest holdout is a composition of two streams. A word is held out. Its own channels have no counts, so a one-hop query on it is a stream call whose promise never becomes ready with a value: 0.011 MRR, which is chance. A two-hop query issues a first call on a trained word, claims its reply, and makes that reply the argument of a second call under the same kind. Two hops on is-a score 0.095 and on used-for 0.093. Part-of, near and unlike are about chance. The two-hop numbers are small. They are also the only measurements in the facts file where a reply on one stream is the argument of a call on another.

The paper offers two ways to write the composition and prefers one. With forks, the program spawns a process for each stage and lets them run. With a coenter, the program declares each stage as an arm, and "the process executing the coenter is halted, and remains halted until all the subprocesses complete." The reason for preferring the coenter is termination: the enclosing process knows when all the arms are done, and can handle a failure in one arm by ending the others. Two-hop quest today is a fork. The first hop is applied, its reply is read, the second hop is applied. Nothing halts, and no arm can end another.

The joint training of phinet is the coenter Phient already has. The facts file lists it among phinet's parts. Where a trainer runs several arms and waits on all of them before the step ends, that is the coenter's shape. No number isolates joint training, so we say only that the shape exists.

The pipeline's cost argument is amortisation. "Buffering allows us to amortize the overhead of kernel calls and the transmission delays for messages over several calls." Phient's engine is a Tensor of shape [b, w, s], complex, and the batch dimension b is buffering in the paper's sense: several calls issued in one message. The engine is allocation-bound. Whether a bigger b amortises anything at these sizes is not measured, and section 12 asks.

## 10. Exceptions on claim, and the deadband as a promise made ready wrongly

The paper's answer to its fourth question is the part of the design Phient most needs. A call whose reply will never come must still yield something when claimed. The stream breaks, and "the system changes the promise to the ready state with an appropriate value, e.g., unavailable("could not communicate")." The claim then signals. The caller finds out where it asks, with a name and a string.

The byte deadband is a promise made ready with the wrong value. A channel is one byte, 256 quanta of 2π. The trainer's nudge is 0.06·sin d, and once sin d < 0.2, at 11°, it falls under half a quantum. Nearest rounding took it to zero and returned. Every pair froze 8 quanta short. In the paper's terms the trainer issued a send, the reply was lost, and the system marked the promise ready with the old value instead of unavailable. No claim signaled. The trainer read convergence.

Dither is the fix that shipped. It works on the quest, 0.594 MRR against the shipped trainer's 0.082. It costs corpus attribution, 71% to 65%, so it ships off by default. It is a change to the stream rather than to the promise. It fixes delivery and tells the caller nothing.

The three operators are where a claim could signal. `loss` is `max(0, margin − better)`. On the quest its mean is 0.108, the margin: the truth ties the best wrong word on almost every row. That is a string that says "unavailable: byte ceiling" for the whole table. Today the trainer reads it as a number and sums it. A trainer that treated loss at the margin as an exception on claim, per pair, would have the paper's design: the send goes out, the promise comes back, the claim returns or signals, and the trainer's next line is a handler.

The paper's other exception, failure, has a Phient reading too. Failure("handler does not exist") is a call to a handler the receiver lacks. A quest query on the wrong channel is that call. It scores 0.015, chance, because the oracle was built by kind and the experiment read the wrong kind on purpose. No decoder in phiano knows it has done this. The paper's design would have the claim say so.

Persona attribution has the same gap. Three writers, chance 33%, 71% overall, 74/65/86 by writer; dialogue replies judged as their persona 44% by the sector reader and 72% by the Kneser-Ney panel. Each reply is a claim on a promise of type persona. The sector reader says whether the claim returned normally. It does not say which sector a wrong reply landed in. That string is the paper's fourth question, and it is not printed.

## 11. What Phient has measured that bears on it

Everything in this section is from the facts file and nothing else.

| measurement | value | bears on |
|---|---|---|
| phinet vs PyTorch reference | 62/62 parity tests | the engine the calls run in |
| Austen softmax wide+deep, 50,016 params | 30.7× valid / 30.3× test at 8 epochs, hit@10 0.348; chance MRR 0.0063 | untyped promises, filled |
| Austen hinge, 8,360 params | 14.7× valid / 14.6× test, 142 s | the send with a hinge reply |
| decode, 40 prompts × 30 tokens | sampler 13.3, nearest-k 66.9 (was 97.2), union 66.8 | the ordinary call, in a stream of 30 |
| forcing, k = 32 | λ=0.05 69.7, λ=0.25 68.5, λ=1 107.4; at λ=1 evaluator 0.712, distinct 0.777, shared 4-grams 22.3% | 2k stream calls outstanding, one claim |
| shared 4-grams across prompts | sampler 25.7%, operator 8.7% | order in the stream |
| quest, attraction-only + dither | 0.594 MRR, hit@1 34.8%, hit@5 97.4%; chance 0.021 | typed promises claimed |
| quest within-kind / wrong channel / offset | 0.747 / 0.015 / 0.005 | the type check; failure("handler does not exist") |
| quest held-out 20% | one hop 0.011; two hops is-a 0.095, used-for 0.093; part-of/near/unlike ≈ chance | composing two streams |
| shipped quest trainer | 0.082 MRR | the promise made ready wrongly |
| byte deadband | nudge 0.06·sin d under half a quantum at sin d < 0.2; froze 8 quanta short | the lost reply |
| dither cost | attribution 71% → 65%; ships off | fixing the stream, not the claim |
| three operators on quest | 61 of 64 hexagrams; better −0.008, worse −0.008, loss 0.108 | loss as the exception on claim |
| helix | 18,496 words, 1.9 s, 75 MB, byte-exact | the array of promises |
| persona attribution | 71% (74/65/86), chance 33% | a claim without a string |
| dialogue | 44% sector reader / 72% KN panel | the same |
| sphere anisotropic tie | 4,776 params, 16.0×/15.6×, hit@10 0.254, 85 s | a different type for the channel |
| engine | allocation/bandwidth-bound, 1 MB op 2–5 ms | the cost the stream is meant to hide |

Not measured and not claimed. Phient has no remote call, no stream and no distribution. No batch-size sweep isolates the cost of issuing many operator applications at once against one at a time. No number isolates joint training. No trainer signals on loss at the margin; loss is summed. Forcing at depth two on the quest has not been run. No Phi-4 run has been made; the oracle is synthetic only. Sphere at softmax and wide+deep is unrun. There is no weather work and no partnership.

The sphere row is included because the paper's promise is typed by what it returns. The circle tie types a channel as one of 64 sectors. The anisotropic tie types it as a degree and an absolute order, 36 slots, and at 4,776 params it beats the circle's hinge at 8,360. The type changed and the claim got better. It is the only measurement in the facts file where the type of the placeholder was varied and the rest held.

## 12. The experiment, with its control

The paper's claim is that a call which returns before it answers, with a typed placeholder to hold the answer, is faster than a call which waits, and no less safe. The experiment tests each half inside phinet.

The first half is speed. The decoder applies the operator one context at a time, and the engine's Tensor has a batch dimension b it does not use there. The experiment runs the 40 held-out prompts × 30 tokens decode with all 40 applications issued as one batch per step, so that 40 stream calls are outstanding and claimed together, against the control, the shipped decoder issuing one application per prompt per step. The measure is two numbers. Wall time per step, which is what the paper says a stream should improve. And Kneser-Ney perplexity, which must be 66.9 in both arms, because the batch changes when the reply is read and not what it is. If perplexity moves, something else changed. If wall time does not move, the engine's cost is where the facts file says it is, in allocation, and there is nothing to amortise at 1 MB per op and 2–5 ms.

The second half is safety, and it is the deadband. The trainer's send becomes a promise in the paper's sense: the nudge is computed, and if 0.06·sin d falls under half a quantum, the promise is made ready with unavailable rather than the old value, and the claim signals. The handler is the arm. The control handler swallows the signal and rounds, which is the shipped trainer at 0.082 MRR on the synthetic quest. The second handler applies dither to that pair on that channel for that step. The third raises the margin for that pair by one quantum and reissues the send. The measure is quest MRR on the same 1,280 rows against chance 0.021, with within-kind and wrong-channel as before, and persona attribution as the cost, where global dither costs 71% → 65%. If the per-pair handler reaches near 0.594 and keeps 71%, the paper's design is confirmed inside phinet: the claim signaled where it should, the handler acted where it should, and no one else paid.

The second half shares its handlers with the experiment the exception paper named. The difference is where the signal lives. There it was a counter read after the fact. Here it is the value the promise is made ready with, read at the claim. Only the second requires the trainer to stop.

Both halves are small. A batch dimension exists. A counter and a branch are cheap. What is not cheap is a trainer that treats its own loss as something that can be signaled.

## 13. Where we disagree, or the analogy breaks

The analogy breaks first on the value. "A promise is a place holder for a value that will exist in the future." When claimed it returns that value or signals. Phical stores no point. The operator's reply is a phasor per channel, a direction rather than a word. The claim in Phient, the cosine against candidates, does not return the value the promise held. It returns the nearest thing to it in a pool. There is no moment at which a Phical promise is ready in the paper's sense. Section 8's reading of the "same outcome" line holds only for a fixed prediction and a fixed pool.

It breaks second on typing. The paper's argument against futures is that a promise is a distinct type, checked at compile time, so nothing has to be examined at runtime to see what it is. Phient's channel types are declared nowhere except in the quest, where the oracle was built by kind. In the Austen operator the 64 channels have no declared type and every claim examines all 64. That is the MultiLisp cost the paper set out to avoid, and Phient pays it on every application.

It breaks third on distribution. The stream exists because the receiver is on another node, the reply takes time, and the caller has other work. Phient is one process, and the reply takes 2–5 ms because the engine is allocation-bound. There is no latency for a stream to hide. The forcing decoder's 2k outstanding calls buy no overlap, and the numbers say so: 69.7, 68.5, 107.4 against 66.9. Section 12's first half asks whether batching buys anything, and we expect little.

It breaks fourth on locality. "Promises are not legal as arguments or results." A promise stays in the guardian that made it. Phient passes predictions everywhere. Two-hop quest passes a claimed reply, which the paper allows. The forcing decoder appends an unclaimed candidate to the context and applies the operator again, which it forbids.

We disagree with the paper at one point, and it is small. The paper prefers the coenter to the fork because the enclosing process knows when its arms finish and can handle termination. Phient's trainer is a fork in that sense and has run without an enclosing process because nothing in it terminates abnormally. That is the problem. The deadband froze every pair and no arm ended. The paper's preference is right, and it has not mattered in Phient only because Phient has no exceptions for a coenter to handle.

The last break is scale. Phient's facts are 1,228 Austen words, 256 synthetic quest words, and three personas, in one process. The paper was written for a network.

## 14. The person and the moment

The paper appeared in the Proceedings of the ACM SIGPLAN 1988 Conference on Programming Language Design and Implementation, held in Atlanta, Georgia, June 22–24, 1988, at pages 260 to 267. The copyright line reads "© 1988 ACM 0-89791-269-1/88/0006/0260 $1.50". The same text appeared in ACM SIGPLAN Notices in July 1988. The authors are given as Barbara Liskov and Liuba Shrira, MIT Laboratory for Computer Science, Cambridge, MA 02139. The work was supported by ARPA and ONR under contract N00014-83-K-0125, by NSF grant DCR-8503662, and by a Hebrew Technical Institute Postdoctoral Fellowship.

The paper's place in Liskov's work is fixed by its references. It cites Halstead's "Multilisp: A language for concurrent symbolic computation," ACM Transactions on Programming Languages and Systems 7, 4, October 1985, as the influence on the design. It cites "Communication in the Mercury System" by Liskov, Bloom, Gifford, Scheifler and Weihl, Programming Methodology Group Memo 59, MIT Laboratory for Computer Science, 1987, also in the Proceedings of the 21st Hawaii Conference on System Sciences, January 1988, as the project the streams belong to. The paper describes that project only as a "project in heterogeneous computing." The language the promises are placed in is Argus, with its guardians and handlers, and the paper says "Argus computations run as atomic transactions" and defers the rest.

Liuba Shrira was a researcher in the Programming Methodology Group at MIT from 1986 to 1997. She is a professor of computer science at Brandeis University and affiliated with MIT CSAIL, an ACM Distinguished Member, and is credited with coining the term "promise" in computing. Her later work with Liskov includes "Replication in the Harp File System" in 1991, "Providing high availability using lazy replication" in 1992 with Ladin and Ghemawat, and "Ownership Types for Object Encapsulation" in 2003 with Boyapati. We did not reach a source for her doctorate and do not state one.

Liskov in 1988 was twenty years past her Stanford thesis under John McCarthy, in which a chess program played endgames with two comparison functions, better and worse, and no evaluation number. She had come to MIT from MITRE. Between the thesis and this paper lie Venus, CLU, the exception mechanism with Snyder, and Argus. She received the Turing Award in 2008.

We note the line from CLU to this paper without claiming she drew it in these words. The promise type is written `promise returns (real) signals (foo)`. That is a CLU procedure heading with the arguments removed and the wait removed. The idea that a module is what it promises did not change between 1974 and 1988. What changed is that in 1988 the promise could be held in a variable and claimed later.

## 15. What remains

Four things remain, in the order in which they could be done.

The first is the batch. Section 12's first half runs the 40-prompt decode with all 40 operator applications issued as one batch per step, against the shipped one-at-a-time decoder. Perplexity must hold at 66.9. Wall time is the number. The engine is allocation-bound at 1 MB per op and 2–5 ms, so the expected gain is small, and the point is to have a number where today there is a sentence. It is the cheapest experiment in the series.

The second is the claim that signals. A trainer whose send comes back unavailable when the nudge falls under half a quantum, and whose claim raises at that point, with a handler per pair. Section 12's second half gives the arms and the control. This is the experiment that tests the paper's design inside phinet.

The third is the coenter. Two-hop quest is a fork today: hop, read, hop. A coenter would issue the first hop for every held-out word, halt until all replies are in, and issue the second hops as one batch. The numbers to beat are 0.095 on is-a and 0.093 on used-for, and we expect them not to move. What it adds is termination: a word whose first hop comes back at chance can be ended before its second hop is issued. That is a count Phient does not print.

The fourth is the string. Every claim in phiano today returns a number. The paper's claim returns a value or raises a name with a string. For persona, that string is which sector a wrong reply landed in, a 3×3 table. For the quest, it is which kind a wrong-channel query was read on, a 5×5 table. For the Austen operator it is nothing yet; the channels have no declared types. The helix has codons, the hexagram has sectors, the sphere has degree and order. Any could be the type. None has been declared as one.

Phient will not have streams. It has no node to send to. What it can take from this paper is the separation of the moment a call is made from the moment its answer is wanted, with a typed thing in between that either returns or signals. The operator already makes that separation. What is missing is the type on the thing in between, and a claim that can say no. The paper concludes that "promises are a good way of supporting efficient, asynchronous remote procedure calls in a programming language." Whether they support a trainer that knows when it has not moved is the question, and it has a control.

## 16. Sources

Reached and read.

The paper itself, as a PDF hosted in a course archive (heather.miller.am/teaching/cs7680/pdfs/liskov1988.pdf), read through the fetch tool in three passes. All quotations attributed to the 1988 paper come from it: the abstract, the section headings, the introduction's four questions and statement of purpose, the stream and its ordering guarantee, the three kinds of call, the broken stream and the two standard exceptions, the definition of a promise, the type syntax and the stream call form, the two states, claim and ready, the multiple-claim rule, the restriction on promises as arguments, the local fork, the grades example, the comparison with MultiLisp futures, the pipeline and composition rule, the fork and coenter, the amortisation sentence, the conclusion's opening and closing sentences, the header and copyright line, the affiliation and funding note, and the references to Halstead and the Mercury memo. The fetch tool returned summaries with short quotations; we have used only the quotations and have not reconstructed the prose around them.

The Wikipedia article on Liuba Shrira, for her years in the Programming Methodology Group, her position at Brandeis and CSAIL affiliation, her ACM Distinguished Member status, the credit for the term "promise," the July 1988 SIGPLAN Notices appearance, and the three later papers named in section 14.

A page by Christopher Meiklejohn (christophermeiklejohn.com/pl/2016/03/04/promises.html) summarising the paper, read before the PDF was reached and used for nothing the PDF did not also give.

Web search listings showing the ACM Digital Library entries for the PLDI proceedings (doi 10.1145/53990.54016) and the SIGPLAN Notices reprint (doi 10.1145/960116.54016), used to locate the sources above and to confirm the DOI.

Not reached.

The ACM Digital Library page returned 403. The DeepDyve page failed with too many redirects. The Programming Methodology Group publication pages at pmg.csail.mit.edu were tried; Liskov's redirected to plain HTTP and was not followed, and Shrira's lists nothing before 1991. The Mercury memo (PMG Memo 59) and Halstead's MultiLisp paper were not fetched; we state about Mercury and futures only what the 1988 paper says of them. No source was reached for Shrira's doctorate.

Phient sources. PHIENT_FACTS.md in this folder for every number about Phient. docs/86_liskov_three_operators.md for the three operators, the forcing decoder and the hexagram table; its numbers are cited from the facts file. The seventh paper in this series, on exception handling in CLU, for the handler arms section 12 shares. The brief in this folder for the biographical facts in section 14 not in the paper: the Stanford thesis under McCarthy, MITRE, MIT, and the 2008 Turing Award.

Nothing in this paper about Liskov's or Shrira's work is stated that was not read in one of the reached sources. Where a detail could not be verified, it was left out.
