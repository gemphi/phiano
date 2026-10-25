# The named exit: a procedure that ends in one of several conditions

*Phient interpreted papers · Liskov series, 7 of 16 · 2026-04-06 · on: "Exception Handling in CLU" (Barbara H. Liskov and Alan Snyder, 1979, IEEE Transactions on Software Engineering SE-5(6), pp. 546–558)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the seventh of sixteen papers in which Phient reads one work by Barbara Liskov through its own paradigm. The earlier papers in the series followed her from the chess endgame with its two comparison functions, through the Venus operating system, into CLU and the abstract data type. This paper stays inside CLU and takes one part of it, the exception mechanism, which Liskov designed with Alan Snyder and published in 1979.

The paper's subject is small and its claim is sharp. A procedure is a mapping from arguments to results. Most of the time it ends in the normal way and returns. Sometimes it cannot, and it must say so. CLU's answer is that a procedure may end in one of several named conditions, that each condition is declared in the procedure's heading, that signaling a condition terminates the procedure, and that only the immediate caller may catch it. Anything the caller does not catch becomes one universal condition called failure, carrying a string. That is the whole design. The paper spends its length defending each choice against the alternatives in PL/I, Mesa and Goodenough's 1975 proposal.

We read it here because Phient has the same shape and has not named it. A word in Phical is a function that relates to other words under conditions. The operator applied to a word does not return one answer. It returns a fit on each of 32 or 64 channels, and the reading of that fit depends on the condition. Some of those readings are the normal result. Some are the word saying it cannot continue this way. Sections 6 to 10 argue that the signals list of a CLU procedure is the closest thing in Liskov's work to the channel list of a Phical word, that the termination model is the reading Phient already takes when it applies the operator once and passes the result on, and that the failure exception is the thing the byte deadband is missing.

Sections 2 to 5 state what the paper says in its own terms. Section 11 lists what Phient has measured, from the facts file only. Section 12 names one experiment and its control. Section 13 says where the analogy breaks: CLU's exceptions are discrete names checked by a compiler, and Phient's channels are continuous phases learned from counts.

One limit is stated at the top. Phient has no exception mechanism. No trainer or decoder in gemphi declares, signals or catches a named condition. The three operators better, worse and loss are the nearest thing, and they are comparisons, not exits. Nothing here should be read as a result. It is a reading.

## 2. What the work claims, in its terms

The paper's abstract opens with the purpose. "For programs to be reliable and fault tolerant, each program module must be defined to behave reasonably under a wide variety of circumstances. An exception handling mechanism supports the construction of such modules." The mechanism it describes is the one in CLU, and the paper says it was designed with "ease of use and enhanced program readability" as goals, with major tradeoffs made between simplicity and expressive power.

The paper has six sections: introduction, the model, syntax and semantics, implementation, expressive power, and discussion. The model is the part that carries the argument. It begins by naming the thing carefully. "In referring to the condition as exceptions rather than errors we are following Goodenough. The term 'exception' is chosen because, unlike the term 'error,' it does not imply that anything is wrong." A procedure is a mapping. Its specification has "a number of clauses, one specifying the behavior for the normal case and one for each exception case." An exception is a way the mapping ends, and it belongs to the specification.

From this the paper draws its first constraint. Exceptions are handled by the immediate caller and by no one else. The paper's words are that CLU will "allow only the immediate caller of a procedure to handle exceptions signaled by that procedure," because "the caller should know nothing about the exceptions signaled by procedures used in the implementation of the invoked procedure." The caller knows the mapping, and it does not know how the mapping is implemented.

The second constraint is the termination model. "Signaling an exception terminates the signaling procedure." The paper contrasts this with resumption, in which the signaler waits for the handler to fix the condition and then continues. It gives its reasons. "The termination model requires a simpler linguistic mechanism for its support than does the resumption model." And "in the resumption model, the signaler and caller are mutually dependent," which makes both the specification and the implementation harder to write.

The third constraint is static association. Handlers are attached to statements in the source text, and the handler that catches an exception is fixed by the program's structure. The paper says of PL/I that "on-units are associated with invocations dynamically rather than statically," and counts that against it. It says Goodenough's proposal "is more constrained and better structured" than PL/I, and that CLU's is "still more constrained."

The fourth constraint is that there is no way to turn exceptions off. "We do not believe that providing a means for programmer disabling of exceptions is consistent with encouraging good programming practice."

## 3. The mechanism, plainly

A CLU procedure declares in its heading what it returns and what it may signal. The paper's first example is a division routine, `div = proc (x, y: int) returns (int) signals (zero-divide)`. Inside the body, `signal zero-divide` ends the procedure and hands control to the caller. The heading is checked. "The information in the procedure heading is used to check that the exception names actually signaled are the correct ones." An exception may carry result objects, in the same way that a normal return carries results.

The caller catches with an except statement. The form is "statement except handler list end." A handler names one or more exceptions and gives a body. The paper lists the forms. `when underflow, zero_divide: body` catches named exceptions with no arguments. `when e1, e2 (s: string, i: int): body` binds their result objects. `when neg, underflow (*): body` discards whatever they carry. `others: body` and `others (e_name: string): body` catch anything not named earlier, and the second form is given the name of the exception as a string.

The rule for which handler runs is lexical. Control goes to "the innermost handler that handles that exception and is part of an except statement containing the invocation." A handler body is ordinary code. If it returns or signals, "rather than terminating just the handler," it "will instead terminate the entire enclosing procedure."

Within one procedure there is a second statement, exit. "The signal statement signals the condition to the calling procedure activation, the exit statement directly raises the condition so that it can be handled in the same procedure activation." Exit is the same idea turned inward. It lets a handler be placed where the code reads well. The paper says the signal statement "can be viewed simply as terminating a procedure invocation and exiting to the appropriate handler in the caller."

Then there is failure. "Failure has one associated result object, a string that may contain some information about the cause of the failure. Every procedure can potentially signal failure; therefore failure is implicitly an exception of every procedure." It may not be listed in a heading. It may be signaled explicitly. But "the most common way that failure is signaled, however, is by an uncaught exception being automatically turned into a failure exception." The paper's example is a procedure that does not catch zero; its invocation "will be terminated with the exception failure ('unhandled exception: zero')."

Handlers attach to statements only. The paper says this was done "to simplify the mechanism" and to avoid providing "a mechanism for substituting new values for expressions."

## 4. What it demonstrated, as reported

The paper reports experience rather than measurement. Its evidence is of three kinds: an implementation with a cost argument, a worked example, and a statement of use.

The implementation section states three criteria in order. "Normal case execution efficiency should not be impaired at all." "Exceptions should be handled reasonably quickly, but not necessarily as fast as possible." "Use of space should be reasonably efficient." It then compares two designs. The branch table method would "follow each invocation with a branch table containing one entry for each exception that can be raised by the invocation." That is fast, and it costs space, because "every invocation must be followed by a branch table." CLU chose the handler table method. "Rather than build a branch table per invocation, the compiler builds a single table for each procedure." This "trades off some speed for space." The table is searched linearly, and "placing the entries in the table in the (linear) order in which the corresponding handlers appear in the source text guarantees that the first candidate found is the handler to use." The unhandled case is one more row, "one additional entry at the end of the handler table." No timings are given in the paper, and we state none.

The worked example is a procedure sum_stream that reads signed decimal integers from a character stream and returns their sum. It signals overflow when the sum leaves the integer range, unrepresentable_integer when a single number does, and bad_format when a field is malformed. It is built from get_number, which converts a field with s2i and translates the lower-level exceptions into the ones sum_stream promises, using the `(*)` form to discard result objects it does not need, and from get_field, which gathers characters and handles end_of_file inside its own loop so that a partial field can still be returned. The example shows exceptions crossing levels of abstraction and being renamed at each level.

The statement of use is the paper's conclusion. "We have used the mechanism in writing many CLU programs (for example, most of the CLU compiler is written in CLU). We are convinced that our programs are better structured than they would be in the absence of the mechanism." It then names the gap. "We have not written programs that attempt to handle the problem of resource constraints, a situation where resumption is most likely to be needed. Further experimentation is needed to reach a final conclusion."

Liskov's later history of CLU adds one report the 1979 paper does not make. It says that "CLU exceptions are implemented efficiently," and that this let them serve "as a general way of conveying information from a called procedure to its caller."

## 5. The one idea restated

Strip the syntax away and one idea is left. A procedure has several ways of ending, each with a name, each with its own result objects, and all of them are part of what the procedure is. The normal return is the first of these ways. The exceptions are the others. The heading lists them. The caller reads the heading and decides, for each named ending, what to do next. The procedure that signals is finished. It does not wait to be told what to do, and it does not know or care who catches.

Everything else in the paper is a consequence. Termination follows because a procedure that is finished has nothing to resume. One-level handling follows because the caller reads the heading of what it called, and only that heading. Static association follows because the heading is text, so the handler that reads it can be text too. The failure catch-all follows because a caller that has not written a handler for a condition has said, in effect, that the condition is not part of its own mapping, and the only honest thing to do is end the caller as well, with a string that says why. And the refusal to let exceptions be disabled follows because a way of ending cannot be removed from a mapping by someone who did not write it.

The idea is about specification, and Liskov says so. In the history of CLU she wrote that "if exceptions are propagated automatically, a procedure may raise an exception not described in its specification." The exception mechanism is there to keep the specification true. That is the same concern that drove the abstract data type: a module is what it promises, and only what it promises, and the language should make it hard to promise less than the module does. Exceptions are the promises about the ways a procedure can fail to keep the main promise.

Phient reads this as a claim about what a function is. A function in the everyday sense is a rule that gives one output. A CLU procedure is a rule that gives one of several named outputs, and the name is part of the output. Liskov's earlier chess program had no evaluation number, only two comparisons, better and worse. Here she again refuses a single value. A procedure's result is a condition and an object, together, and the condition is named. That is the idea the next five sections carry across.

## 6. The word as a function with a signals list

In Phient a word stores no point. It is 32 or 64 relations, each a complex phasor on a closed circle with a count. When the operator is applied, the word is presented as an input and the output is a fit on each channel. Read as a function, the word is a mapping from a condition c to the words it relates to under c. That is close to Liskov's procedure, and the closeness is exact in one respect. The procedure's heading lists the conditions in which it may end. The word's channels list the conditions under which it may relate.

Take the quest as the case where this is visible. The quest oracle in phiecs has 256 words in families of 8, and five kinds of relation: is-a, part-of, near, used-for and unlike. Each kind is a channel. A word's row in the quest says, for each kind, which target it relates to. That row is the word's heading. It says: under is-a, I end here; under part-of, I end there. The quest trainer with attraction only and dithered rounding reaches 0.594 MRR against chance 0.021, hit@1 34.8% and hit@5 97.4%. Within a kind the number is 0.747. On the wrong channel it is 0.015. The wrong-channel number is the one that matters here. A word asked to relate under a kind it was not trained on gives an answer indistinguishable from chance. In CLU terms, the caller asked for an ending that is not in the heading.

CLU's response to that is loud. An exception that is signaled and not caught becomes failure, with a string. Phient's response is a fit near chance, which is silence. The quest scores the wrong channel at 0.015 only because the experiment was designed to read it. Section 9 takes this up.

The other direction of the analogy is the signals list as a limit on the word. A CLU procedure may signal only what its heading names. A Phical word has only the channels it has, and the count on each channel says how often that relation has been seen. A channel with a count of zero is a condition the word has never ended in. The helix filing of docs/84 makes this concrete: each of 64 channels is a codon, base-4 ACGT, and a word's function is the string of codons. Read as a heading, it is the list of conditions the word may signal. The lexicon of 18,496 words, written that way, takes 1.9 s and 75 MB and round-trips byte-exact. That is a heading file for the whole vocabulary, and it has never been read as one.

## 7. The relation channel as a coordinate, and one-level handling

Liskov's second constraint is that only the immediate caller may catch. The reason is stated in the paper: the caller knows the mapping and not its implementation, so exceptions from deeper procedures are none of its business until they are renamed into the heading it can see. The sum_stream example does exactly this. get_field catches end_of_file itself. get_number translates what s2i signals into what it promised. sum_stream sees only overflow, unrepresentable_integer and bad_format.

Phient's operator is one-level in the same way. The operator maps a context to a prediction, the prediction is compared to candidates by cosine, and the winning candidate is appended. The next application starts from the new context and does not see how the prediction was formed. The channel on which a relation was read is a coordinate of the output, and it is the only coordinate the next step is given. There is one hop.

The quest holdout measures what this costs. On held-out 20% of the oracle, one hop scores 0.011, which is chance. Two hops on is-a score 0.095 and on used-for 0.093; part-of, near and unlike are about chance. The two-hop numbers are low. They are also the only place in the facts file where a relation is read through an intermediate word. In CLU terms, two hops is a caller catching an exception from a procedure it called, which caught and renamed one from lower down. The renaming is what the middle word does when it is trained. The held-out word was not trained, so it has no heading of its own, and the only way to reach its relations is through a word that has one.

This is the reading Phient takes of Liskov's one-level rule. It is a rule about who renames information. Every level of the call chain must state, in its own heading, every ending it can reach, including endings it reached only by catching and renaming something lower. The middle word in a two-hop quest does the same. Its is-a channel must carry, in its own phase, the fact that the word below it relates under is-a to something further. The 0.095 says it carries a little of that. The 0.011 says that without the middle word it carries none.

The coordinate itself is a phase on a closed circle, quantised to a byte. A CLU exception is a name in a finite list. A channel reading is a sector in a finite ring. The hexagram filing of docs/86 turns the 64-channel function into one of 64 sectors, and the quest words occupy 61 of them. Sixty-one named endings, and a flat table.

## 8. The operator as X ⇒ Y under c, and the termination model

The paradigm sentence says the operator drives a pair of words under a condition. X ⇒ Y under c. In CLU the invocation is X, the procedure is the operator, and the ending is one of several Y, each named. The name is c. The termination model is what makes the reading hold.

Under termination, the signaler is finished when it signals. Under resumption, it would wait for the handler and go on. The paper gives two cases where resumption is wanted. Numeric underflow, where "the desired action is to substitute a particular value (e.g., zero) for the result of the operation and continue." And a storage pool that is nearly full, where "it may still be possible to satisfy the request if some of the objects stored in the pool can be reorganized to use less storage." It concedes these cases and then says "unless it is shown that such cases arise frequently, they do not justify the more complex mechanism."

Phient's decoder is a termination model. The operator is applied, one candidate is chosen, and the application is over. The greedy nearest-k decoder scores 66.9 Kneser-Ney perplexity over 40 held-out prompts × 30 tokens. The forcing decoder of docs/86 is the resumption model. `nearest_forcing` takes the 2k nearest words, appends each to the context, applies the operator once more, and re-ranks by cos − λ·worse. The first application does not finish until it has seen what its result leads to. At λ = 0.05 that scores 69.7, at λ = 0.25 68.5, at λ = 1 107.4. Forcing does not beat greedy on text at any λ tried.

Docs/86 gives the reason: a one-step lookahead in a space where every word has some continuation is not the endgame Liskov had, where most moves lead nowhere. Liskov's 1979 argument is the same argument in the language of procedures. Resumption is worth its cost only where the signaler can do something useful with what the handler sends back. On text, every word has a continuation, so there is nothing to send back that the next application would not find on its own. Termination is cheaper and scores the same or better, which is what the paper predicted about its own domain.

The place forcing should bite is the quest, where a word either has onward relations or does not. Docs/86 says this and has not run it at depth two. At depth one the three operators score mean better −0.008, worse −0.008, loss 0.108 with margin 0.1, which is flat. The dead ends that would make the lookahead pay are absent from an oracle where every word has a family of 8.

## 9. Failure, and the byte deadband as an unhandled exception

This is the constraint Phient most lacks. In CLU, a condition that is signaled and not caught does not vanish. It becomes failure with a string, and the caller is terminated in turn. "We decided to turn all unhandled exceptions into a special exception called 'failure' and propagate it," Liskov wrote later. The 1979 paper says "we felt it was unrealistic to require the programmer to provide handlers in situations where no meaningful action can be taken." So the language provides the one handler that always applies, and it is loud.

The byte deadband is the case. A channel is one byte, 256 quanta of 2π. The trainer's nudge is 0.06·sin d, and it falls under half a quantum once sin d < 0.2, which is 11°. With nearest rounding the nudge is applied, rounded, and lost. Nearest rounding froze every pair 8 quanta short. Nothing signaled. The trainer asked each channel to move, the channel could not, and the request ended in a condition the trainer had no name for. It ended in silence, and the silence looked like convergence.

Dither is the fix that shipped, and it is a resumption fix. It substitutes a value and continues, the paper's underflow case. It works on the quest: 0.594 MRR with dither against 0.082 for the shipped trainer. It costs elsewhere: persona attribution falls from 71% to 65%, so dither ships off by default. That is also what the paper predicts. Resumption is more powerful and more expensive, and its cost is paid by everyone.

Liskov's alternative is a failure channel. A nudge that falls under half a quantum is an unhandled exception. The honest thing, on her design, is to terminate the step with a name and a string, and let the caller decide. The caller here is the training loop. It could count the sub-quantum nudges per channel, raise the margin, or switch to dither for that pair only. Any of these is a handler. What Phient has today is the PL/I behaviour the paper argues against, where the condition is handled by whatever is dynamically in scope, here the rounding function, which handles it by doing nothing.

The third operator, loss, is `max(0, margin − better)`, and it is the nearest thing to failure in phinet. On the quest its mean is 0.108, which is the margin. That number says the truth ties the best wrong word on almost every row. Read correctly, it is a failure string. It says "unhandled exception: byte ceiling" for the whole table at once. It terminates nothing. The trainer reads loss, rounds, and continues. Liskov's mechanism would have it stop, name the condition, and hand the pair to something that can act.

## 10. Static handlers, the others arm, and the hexagram table

Handlers in CLU are bound in the source text, and a catch-all named others may sit last. Two things in Phient look like a handler table, and one looks like others.

The hexagram table of docs/86 is a handler table. Each word's 64-channel function is reduced to one of 64 sectors by the centroid phase, and the word is filed under that sector. The sector is a name. The 256 quest words fall into 61 of 64 sectors. On the synthetic quest the table is flat, mean better −0.008, worse −0.008, loss 0.108, and docs/86 says plainly that this is what a flat structure scores, and that the table exists for a corpus that is not flat, Phi-4's, which has not been run. In CLU terms it is a handler table for a procedure whose exceptions have never been signaled. The rows are there and nothing has landed in them.

The helix is the second table, and it is closer to source text. A word is 64 codons, base-4 ACGT of channel, with a hexagram, a sphere degree and order, B-DNA xyz at 10.5 codons per turn and 0.34 nm rise, and a PhiTon wavelength, band and hex per codon. It is written once and read back byte-exact, static in the paper's sense. The handlers, if there were any, would be found by reading the codons in order, which is the paper's linear search.

The others arm is the union decoder. The facts file has three decoding arms: the trigram sampler at 13.3 perplexity, the operator nearest-k at 66.9, and union at 66.8. Union takes the operator's candidates and falls back to the sampler when the operator has nothing. That is `others: body`. It scores what the operator scores, which says the operator rarely has nothing, on text. On the quest, where wrong channel scores 0.015, an others arm would fire on every wrong-channel query and would have somewhere to send it.

The paper's second form, `others (e_name: string): body`, is the one Phient should want. It gives the handler the name of the unexpected condition. Phient's decoder does not know which channel a candidate was read on. The quest experiments know, because the oracle was built by kind. The Austen operator does not, because the corpus was ingested as pairs without a kind. The persona work has a weak version: three writers, 71% attribution, replies judged as their persona 44% by the sector reader and 72% by the Kneser-Ney panel. The persona is the condition. The reply either lands in its persona's sector or it does not. What is missing is the string that says which sector it landed in instead. That change to the sector reader has not been made.

## 11. What Phient has measured that bears on it

Everything in this section is from the facts file and nothing else.

| measurement | value | bears on |
|---|---|---|
| phinet vs PyTorch reference | 62/62 parity tests | the engine the operators run in |
| Austen hinge, 8,360 params | 14.7× valid / 14.6× test over chance MRR 0.0063 | the operator as a mapping with one normal ending |
| decode, 40 prompts × 30 tokens | sampler 13.3, nearest-k 66.9 (was 97.2), union 66.8 | termination decoder; union as others |
| forcing, k = 32 | λ=0.05 69.7, λ=0.25 68.5, λ=1 107.4; at λ=1 evaluator 0.712, distinct 0.777, shared 4-grams 22.3% | resumption decoder |
| quest, attraction-only + dither | 0.594 MRR, hit@1 34.8%, hit@5 97.4% | the kind as a signals list |
| quest within-kind / wrong channel / offset | 0.747 / 0.015 / 0.005 | the unhandled condition |
| quest held-out 20% | one hop 0.011; two hops is-a 0.095, used-for 0.093; others ≈ chance | one-level handling |
| shipped quest trainer | 0.082 MRR | before the resumption fix |
| byte deadband | nudge 0.06·sin d under half a quantum at sin d < 0.2; froze 8 quanta short | the silent unhandled exception |
| dither cost | attribution 71% → 65%; ships off | the cost of resumption |
| three operators on quest | 61 of 64 hexagrams; better −0.008, worse −0.008, loss 0.108 | the flat handler table |
| helix | 18,496 words, 1.9 s, 75 MB, byte-exact | the static heading file |
| persona attribution | 71% overall (74/65/86), chance 33% | the condition as sector |
| dialogue | 44% sector reader / 72% KN panel, chance 33% | the same |
| engine | allocation/bandwidth-bound, 1 MB op 2–5 ms | cost of any added table |

Not measured and not claimed. No trainer signals a named condition; there is no count of sub-quantum nudges. Forcing at depth two on the quest has not been run. The hexagram table on a Phi-4 quest has not been run, because no Phi-4 run has been made; the oracle is synthetic only. Sphere at softmax and wide+deep is unrun. There is no weather work and no partnership.

The sphere numbers bear on this paper only indirectly. The anisotropic tie at 4,776 params reaches 16.0×/15.6× and beats the circle's 14.7×/14.6× at 8,360. The tie changes the coordinate system, from 64 sectors on a circle to Y_l^m with l < 8. A signals list on the sphere is a list of degree and order. The helix records both. We do not choose between them here.

## 12. The experiment, with its control

The experiment gives the trainer one named condition and asks whether naming it changes what the trainer does. The condition is the deadband. A nudge on a channel is sub-quantum when 0.06·sin d is under half of one of the 256 quanta, which the facts file places at sin d < 0.2. Today that nudge is rounded to zero and nothing records it. The experiment adds a counter per word and per channel, incremented when the nudge is sub-quantum, and treats the counter as a signal: when a pair's channel has signaled for a run of steps, the step is terminated for that pair and handed to a handler.

Three handlers are the arms. The first handler does nothing, which is the control. It is the shipped trainer with nearest rounding and no dither, at 0.082 MRR on the synthetic quest. The second handler is dither for that pair only, on that channel only, for the steps in which the signal is raised. The third handler raises the margin for that pair by one quantum and retries the step. The measure is quest MRR on the same 1,280 rows against chance 0.021, with hit@1, hit@5, within-kind and wrong-channel as in docs/83. The second comparison is global dither, the attraction-only trainer at 0.594 MRR. If the per-pair handler reaches the neighbourhood of 0.594, the signal identified the pairs that needed dither. If it stays near 0.082, the signal is not what the deadband is about.

The cost side is persona attribution. Global dither costs 71% → 65% on three writers. The experiment runs the same attribution with the per-pair handler in place. Liskov's paper says that the price of resumption is paid by every invocation. If a handler that fires only on signaled pairs keeps 71%, the price is not paid, and the paper's argument for termination with a named exit is confirmed inside phinet. If it falls to 65% regardless, the cost was in the dither itself.

A second experiment follows from section 8 and is cheaper. It runs `Decode::Forcing` on the quest at depth two, which docs/86 lists as remaining. The control is the depth-one quest score by the three operators, mean loss 0.108. The measure is whether `worse` stops being flat when the candidate's onward relations are read one hop further. On a synthetic oracle where every word has a family, we expect little. It is the one place the resumption model has a stated chance, and the facts file has no number for it.

Both experiments are small. The engine is allocation-bound and a 1 MB op costs 2–5 ms, so a counter per channel is not the cost. The cost is a trainer that can stop.

## 13. Where we disagree, or the analogy breaks

The analogy breaks first on the name. A CLU exception is a discrete identifier in a finite list, chosen by the programmer and checked by the compiler against the heading. A Phical channel reading is a phase on a closed circle, quantised to a byte, learned from counts. The hexagram filing gives it 64 names, and a word near a sector boundary is filed on one side by a rounding. No compiler checks that a word signals only what it lists. The quest's wrong-channel score of 0.015 comes from no checker. It is a number that happens to be near chance. Liskov's whole design depends on the heading being enforced, and Phient's heading is a histogram.

It breaks second on termination. In CLU the activation that signals ceases to exist. In Phient nothing ceases. The operator is a unitary rotation on a closed circle, and the same word is there after the step as before, with its phases moved. A word that ends in a bad condition on one channel is still present on the other 63. What Phient can terminate is a step, or a candidate. That is the reading section 12 takes, and it is narrower than the paper's.

It breaks third on who the caller is. In CLU the caller is another procedure with its own heading and its own handlers, written by someone. In Phient the caller of the operator is the decoder, or the trainer, and neither has a specification in Liskov's sense. No document says which endings the decoder handles. The union arm is an others handler by accident of construction. The paper's argument against PL/I is that dynamic association makes it impossible to read a program and know which handler will run. Phient's decoder is dynamic in exactly that sense.

We disagree with the paper at one point, and it is the point the paper itself flagged. The paper concedes that resumption is right for the storage-pool case and the underflow case and argues that those cases are rare. On the quest they were common. The deadband froze every pair, and the fix that worked was dither, which is resumption. The paper's prediction holds on text, where forcing does not beat greedy. It does not hold on the byte-quantised trainer, where the resumption fix is the only one measured to work. Whether a named-exit fix would also work is section 12's question. Until it is run, the honest position is that on this system the paper's minority case was the common case.

The last break is scale. Phient's facts are 1,228 Austen words seen 20 or more times, 256 synthetic quest words, and three personas. Nothing here is at the scale where the paper's cost arguments were made.

## 14. The person and the moment

The paper was received on March 8, 1979, revised on June 25, 1979, and appeared in IEEE Transactions on Software Engineering, volume SE-5, number 6, pages 546 to 558. The author note describes Barbara H. Liskov as Associate Professor of Electrical Engineering and Computer Science at the Massachusetts Institute of Technology, Cambridge, with research interests in "programming methodology, distributed systems, and the design of languages and systems to support structured programming." Alan Snyder is described as a Member of Technical Staff at the Computer Research Laboratory of Hewlett-Packard Laboratories, Palo Alto. The work was supported by the Advanced Research Projects Agency of the Department of Defense, monitored by the Office of Naval Research under contract N00014-75-C-0661, and by National Science Foundation grants.

Liskov's own history of CLU places the design earlier than the paper. She writes that "the main decisions about our exception mechanism had been made by June 1975," and that "we considered and rejected the resumption model present in both PL/I and Mesa because it was complex and also because we believed that most of the time, termination was what was wanted." The CLU Reference Manual was published in July 1978, before the paper, and updated in October 1979, after it. The history says a resignal statement was added late, in 1979. The 1979 paper as we read it does not describe resignal, and we do not attribute it to the paper.

The history names the design group as Liskov, Russ Atkinson, Craig Schaffert, Alan Snyder, Bob Scheifler and Eliot Moss. It credits iterators to Atkinson and the exception mechanism to no one person.

The moment, in Liskov's own account, was a belief that "some kind of exception mechanism was needed" for programs that "are prepared to cope with the presence of errors by attempting various error recovery techniques." The same history says the hardest part "is to provide good human engineering for catching exceptions," which is where the paper spends most of its pages: the forms of the handler, the placement of the except statement, the exit statement that lets a loop's handler sit where the reader expects.

Liskov received the Turing Award in 2008. She was at MITRE before MIT. Her 1968 Stanford thesis, under John McCarthy, played chess endgames with two comparison functions and no evaluation number. We note the line from that thesis to this paper without claiming she drew it: in both, the result of a computation is a named condition.

The bibliographic details above come from a secondary index and from the paper's own front matter. We did not reach a source for Alan Snyder's later career and do not state one.

## 15. What remains

Four things remain, in the order in which they could be done.

The first is the counter. A trainer that records, per word and per channel, how many steps ended with a sub-quantum nudge, and reports it alongside loss. This is a table and a print statement. It measures nothing new on its own, and it is the signals list of the deadband; without it section 12 cannot be run.

The second is the handler. The per-pair dither and the per-pair margin raise, run against the shipped trainer and against global dither on the synthetic quest, with persona attribution as the cost measure. Section 12 gives the arms and the control. This is the experiment that tests the paper's claim inside phinet: that a named exit with a local handler is cheaper than resumption everywhere, and as good where it fires.

The third is depth two on the quest, which docs/86 already lists. The `worse` operator has been flat on the synthetic oracle at depth one because every word has a family. Whether it is flat at depth two is unknown. If it is, the synthetic oracle cannot test the forcing tree at all, and the test waits for Phi-4. If it is not, then the quest is the one place in Phient where the resumption model has a measured advantage, and the paper's argument should be read again with that number in hand.

The fourth is the string. CLU's failure carries a string that says what was not handled. Phient's sector reader says whether a reply landed in its persona's sector, and nothing about where else it landed. The `others (e_name: string)` form is the small change: report the sector a wrong reply landed in, and count the table. On three personas this is a 3×3 table. On the quest it is a 5×5 table of kinds, and wrong channel at 0.015 says the off-diagonal is flat, and it has not been printed.

None of these is a language feature. Phient will not have a signals clause or an except statement. What it can have is a trainer and a decoder that know the names of the conditions they end in, that stop when they end in one they have no handler for, and that say which one it was. That is the paper's design with the syntax removed. The paper's own conclusion applies to it: the mechanism was used in writing the compiler that compiled it, and the authors were "convinced that our programs are better structured than they would be in the absence of the mechanism." Whether the same is true of a trainer is the question, and it is a question with a control.

## 16. Sources

Reached and read.

The paper itself, as a PDF hosted in a course archive at Tufts (cs.tufts.edu/~nr/cs257/archive/barbara-liskov/exceptions.pdf), read through the fetch tool in three passes. All quotations attributed to the 1979 paper come from it: the abstract, the section headings, the termination and resumption argument, the handler forms, the exit statement, the failure exception and its automatic conversion, the three implementation criteria and the two table methods, the sum_stream example, the underflow and storage-pool cases, the conclusion, the author note, the received and revised dates, and the contract number. The fetch tool declined to reproduce whole sections and returned summaries with short quotations; we have used only the quotations and have not reconstructed the prose around them.

The researchr bibliographic record for LiskovS79, which gives the journal, volume 5, issue 6, pages 546 to 558, and the year.

Barbara Liskov, "A History of CLU," MIT Laboratory for Computer Science Technical Report MIT-LCS-TR-561, read from publications.csail.mit.edu. Quotations in sections 4, 5, 9 and 14 attributed to the history come from it: the June 1975 date for the main decisions, the rejection of resumption, the decision to turn unhandled exceptions into failure, the sentence on specifications, the sentence on efficient implementation, the design group, the resignal addition, and the reference manual dates of July 1978 and October 1979.

A web search listing that showed the Semantic Scholar entry for the paper and the ACM entries for "A History of CLU" and for the SIGSOFT impact paper "Influences on the design of exception handling," used only to locate the sources above.

Not reached.

The ACM Digital Library page for the SIGSOFT impact paper returned 403. The ADS abstract page for the paper (1979ITSEn...5..546L) was blocked by robots. The Semantic Scholar page required an approval that was not given in time. A direct download of the Tufts PDF from the shell was refused by the egress proxy; the fetch tool reached it. The IEEE Xplore record was not attempted. No DOI is stated because none of the reached sources gave one.

Phient sources. PHIENT_FACTS.md in this folder for every number about Phient. docs/86_liskov_three_operators.md for the three operators, the forcing decoder and the hexagram table; its numbers are cited from the facts file. The brief in this folder for the biographical facts in section 14 that are not in the paper: the Stanford thesis under McCarthy, MITRE, MIT, and the 2008 Turing Award.

Nothing in this paper about Liskov's or Snyder's work is stated that was not read in one of the reached sources. Where the fetch tool reported that the paper does not discuss something, we have not claimed that it does. Where a detail could not be verified, it was left out.
