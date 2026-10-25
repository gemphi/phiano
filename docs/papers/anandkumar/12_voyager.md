# The Quest That Keeps What It Learns: Voyager's Skill Library Read Against Phient's Relation Loop

*Phient interpreted papers, 12 of 16 · 2026-06-15 · on: "Voyager: An Open-Ended Embodied Agent with Large Language Models" (Wang, Xie, Jiang, Mandlekar, Xiao, Zhu, Fan, Anandkumar; 2023; arXiv 2305.16291)*

"In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition."

## 1. Our abstract

Voyager is the paper in Anandkumar's list that is least about operators and most about loops. It puts a frozen GPT-4 inside Minecraft and asks it, again and again, for three things: what to try next, code that does it, and a verdict on whether the code worked. Nothing in the model is trained. What grows is a library of small programs, each with a text description, and the library is what the agent reuses. The paper reports 63 unique items in 160 prompting iterations, 3.3× more than the best prior method, distances 2.3× longer, and the wooden tool milestone reached 15.3× faster in iterations than AutoGPT. It also reports that the library transfers: in a fresh world, tasks the agent had never been given were solved in every trial, while baselines mostly failed.

This paper matters to Phient for a specific reason. Phient's quest, the loop in phiecs, has the same shape at a smaller scale. It asks a model one word at a time, under a named relation kind, and it keeps what it learns as a growing store of relations. The store is not code. It is 32–64 phasors per word on a closed circle with a count. But the loop is the same loop: propose, act, verify, keep. Voyager's three parts, the automatic curriculum, the skill library and the iterative prompting, map onto the quest's three parts: which word to ask about next, which relations are already held, and whether the answer moved a pair of words the right way.

We read the paper on its own terms in sections 2 to 5. We then interpret its parts in Phical terms. The skill library becomes a relation store; the retrieval by embedding becomes the relation channel as a coordinate; self-verification becomes the count on a phasor. We report what Phient has measured that bears on the same question, which is the quest's own numbers: 0.594 MRR on a synthetic oracle with attraction-only training and dithered rounding, hit@5 97.4%, and a held-out score that is near chance for one hop and better for two hops in some kinds. We name an experiment that would test Voyager's central claim inside phinet, with a control. We say where the analogy breaks: Voyager's skills execute and Phient's relations only rank, and Voyager's growth is measured in items while ours is measured in retrieval. Nothing here is a claim that Phient has done what Voyager did. Phient has run the quest on a synthetic oracle only. No Phi-4 run exists.

## 2. What the paper claims, in its own terms

The abstract is exact about its claim, and we keep to it. Voyager is called "the first LLM-powered embodied lifelong learning agent in Minecraft that continuously explores the world, acquires diverse skills, and makes novel discoveries without human intervention." It has three components. The first is "an automatic curriculum that maximizes exploration." The second is "an ever-growing skill library of executable code for storing and retrieving complex behaviors." The third is "a new iterative prompting mechanism that incorporates environment feedback, execution errors, and self-verification for program improvement."

The model is not fine-tuned. The paper says Voyager "interacts with GPT-4 via blackbox queries, which bypasses the need for model parameter fine-tuning." That is a design choice with a consequence the authors name: the skills "are temporally extended, interpretable, and compositional, which compounds the agent's abilities rapidly and alleviates catastrophic forgetting." Forgetting is avoided because nothing in the weights changes; what accumulates is outside the model.

The empirical claim is stated in three ratios. Voyager "obtains 3.3x more unique items, travels 2.3x longer distances, and unlocks key tech tree milestones up to 15.3x faster than prior SOTA." The prior methods compared are ReAct, Reflexion and AutoGPT, each adapted to the same Minecraft interface. A fourth claim is generalization: Voyager "is able to utilize the learned skill library in a new Minecraft world to solve novel tasks from scratch, while other techniques struggle to generalize."

The paper was submitted on 25 May 2023 and revised on 19 October 2023. The version we read is v2. The arXiv comment gives a project website and open-source codebase at voyager.minedojo.org; we did not fetch that site and cite only the arXiv pages.

What the paper does not claim is also plain. It does not claim the agent sees. The limitations section says Voyager "does not currently support visual perception, because the available version of GPT-4 API is text-only." It does not claim the loop is cheap: "GPT-4 API incurs significant costs. It is 15× more expensive than GPT-3.5." It does not claim the verifier is reliable; it gives a failure where the verifier did not recognize spider string as success. It does not claim the curriculum is grounded; it gives a hallucinated item, a copper sword, that the curriculum proposed and that does not exist in the game. These admissions are part of what the paper says, and we treat them as such.

## 3. The mathematics, plainly

There is very little mathematics in Voyager, and that is a fact about the paper rather than a gap in our reading. The paper's formal content is the structure of three prompts and one retrieval rule.

The curriculum prompt contains a directive toward diversity, the agent's current state, the list of completed and failed tasks, and questions generated by GPT-3.5 with answers drawn from wiki knowledge. The paper uses a warm-up schedule that adds state information to the prompt only after some tasks are done. The output of this prompt is one task in natural language. Formally, this is a function from a state and a history to a task, and the function is the frozen model.

The skill library is the nearest thing to a mathematical object. Each skill is a program with a description. The description is embedded with text-embedding-ada-002 into a vector. Retrieval embeds the current task together with environment feedback into the same space and takes the nearest neighbours. The paper uses the top five. Formally, retrieval is a nearest-neighbour query in a fixed embedding space; the library is a set of points in that space with programs attached.

The iterative prompting mechanism is a loop with a bound. The model writes code; the code runs; three signals come back: environment feedback through the game's chat channel, execution errors from the JavaScript interpreter, and a verdict from a separate GPT-4 call that reads the state and says whether the task is done, with a critique if not. The paper says, "If the agent gets stuck after 4 rounds of code generation, then we query the curriculum for another task." Formally, this is a fixed-point search on code under three feedback signals with a step limit of four.

The temperatures are stated: "We set all temperatures to 0 except for the automatic curriculum, which uses temperature = 0.1." So two of the three functions are deterministic given their prompts, and only the proposer has any randomness. That is worth stating in plain terms. The whole loop's diversity comes from the environment and from a small temperature on one call.

In Phical's language, we would write the loop as an operator. Let X be the state of the world plus the library, and let c be the current task. Voyager's step is X ⇒ Y under c, where Y is the state after the code ran and the library after a skill was added. The paper does not write it this way, and we do not claim the authors would. But it is a step operator with a condition, and the library is what the operator carries forward. That is the shape we interpret in sections 6 to 10.

## 4. Its experiments and numbers as reported

The environment is Minecraft through MineDojo, with Mineflayer JavaScript APIs for control. The models are gpt-4-0314 for the curriculum and code generation, gpt-3.5-turbo-0301 for question answering, and text-embedding-ada-002 for embeddings. Each condition is run three times and results are given as mean and standard deviation. The main evaluation allows 160 prompting iterations.

Exploration is measured by unique items obtained. The paper states that Voyager keeps "discovering 63 unique items within 160 prompting iterations." AutoGPT reached 19. ReAct and Reflexion made little progress on this measure. The ratio 3.3× is the abstract's summary.

The tech tree is measured by iterations to the first tool of each level, as Table 1 reports.

| Milestone | Voyager | AutoGPT | ReAct | Reflexion |
|---|---|---|---|---|
| Wooden tool | 6 ± 2 | 92 ± 72 | 0/3 | 0/3 |
| Stone tool | 11 ± 2 | 94 ± 72 | 0/3 | 0/3 |
| Iron tool | 21 ± 7 | 135 ± 103 | 0/3 | 0/3 |
| Diamond tool | 102 (1/3) | 0/3 | 0/3 | 0/3 |

The paper gives the speedups as 15.3× for wooden, 8.5× for stone and 6.4× for iron. Diamond was reached in one trial of three at iteration 102. Map coverage is a figure, not a table; the paper's sentence is that Voyager "is able to navigate distances 2.3× longer compared to baselines."

Generalization is measured in a fresh world on tasks never given during the main run, with a cap of 50 iterations, as Table 2 reports.

| Task | Voyager | AutoGPT with library | AutoGPT | ReAct | Reflexion |
|---|---|---|---|---|---|
| Diamond pickaxe | 19 ± 3 (3/3) | 39 (1/3) | 0/3 | 0/3 | 0/3 |
| Golden sword | 18 ± 7 (3/3) | 30 (1/3) | 0/3 | 0/3 | 0/3 |
| Lava bucket | 21 ± 5 (3/3) | 0/3 | 0/3 | 0/3 | 0/3 |
| Compass | 18 ± 2 (3/3) | 30 (2/3) | 0/3 | 0/3 | 0/3 |

The ablations are the part we care about most. Replacing the curriculum with a random one: "The discovered item count drops by 93%." Removing the library: Voyager "exhibits a tendency to plateau in the later stages." Removing self-verification: "a significant drop (−73%) in the discovered item count." Replacing GPT-4 with GPT-3.5: GPT-4 "obtains 5.7× more unique items." So by the paper's own numbers, the curriculum and the verifier are each worth more than the library within 160 iterations, and the library's value shows late and in transfer.

## 5. The one idea, restated

Voyager's one idea is that a frozen model can learn for a lifetime if what it learns is kept outside the model in a form the model can read back. The model is a function that does not change. The environment changes. The library changes. Learning is the change in the library. Nothing else moves.

That idea has three parts that the paper keeps separate, and the ablations show why. First, someone has to choose what to try. If the choice is random, most of the gain is gone. Second, someone has to say whether it worked. If nobody checks, most of the gain is gone too. Third, what worked has to be kept in a way that can be found again. Without that, the agent still climbs, but it stops climbing sooner and cannot carry what it climbed to a new world.

The restatement in our terms is short. A skill is a name, a description and a body. The name and description are the part the model can find; the body is the part that runs. The library is a map from descriptions to bodies, and the map is queried by nearness in an embedding. The agent's competence is the size and reach of that map. That is a very old idea, a case base, made new by a model that can write the bodies and write the descriptions and judge the outcomes.

It is also the idea that a curriculum is a policy over questions, not answers. Voyager's curriculum does not know how to make a pickaxe. It knows what the agent has, what it has failed at, and what a diverse next step would be. The step is a task in words. The words go to the coder. The coder's output goes to the world. The world's reply goes to the verifier. The verifier's reply goes back to the coder, four times at most, and then either to the library or to the curriculum again.

We note one more thing the paper says that its headline ratios do not: the diamond tool was reached in one run of three. Lifelong learning, in this paper, means a strong slope for 160 iterations and a library that transfers. It does not mean convergence to mastery, and the authors do not say it does. The idea is the loop and the store, not the endpoint.

## 6. A word as a function, and a skill as one too

In Phient a word is a function. It is not a point in a space. It is 32–64 relations, each a complex phasor on a closed circle with a count. The word "hammer" holds a phase toward "nail" on the used-for channel, a phase toward "tool" on the is-a channel, and a count on each that says how often that phase has been confirmed. Meaning is what happens when two such functions move under a condition.

Voyager's skill is also a function, in the ordinary programming sense. `mineBlock`, `craftItem`, `killMob` are the primitives the paper names. A learned skill composes them. Its description says what it does. Its embedding is where it sits.

The interpretation we make is that Voyager's skill and Phient's word occupy the same slot in their respective loops. Both are the unit that is stored, retrieved and composed. Both have a public face that is used for finding and a private body that is used for acting. In Voyager the face is a text description embedded into a vector; the body is JavaScript. In Phical the face and the body are the same object: the relations are both what the word means and what is used to find it. There is no separate embedding. The 32–64 phasors are the address.

This collapse is the difference we want to draw out. Voyager needs a second model to embed descriptions because its bodies are code and code does not sit anywhere. Phical does not, because the body of a word is already a set of positions on circles. A phase is a coordinate. A count is a confidence. Retrieval is reading the phase on the right channel and looking for the word whose phase is nearest. That is what the quest's trainer does when it scores a target, and the fact that a word is its own index is why the quest can be as small as it is: 256 words, families of 8, five kinds, 1,280 rows.

We do not claim this is better. Voyager's skills do something in the world. Phient's words only rank other words. A skill that mines iron and a word that points at "iron" under is-a are functions of very different power. What we claim is narrower: the storage form is analogous, and in Phical the storage form is also the retrieval form. That is the fact that the rest of the interpretation rests on.

## 7. The relation channel as a coordinate

Voyager retrieves with one embedding space. Every skill is a point; every query is a point; nearness is one number. The paper asks for the top five. There is no notion in the library of what kind of nearness is meant. A skill for smelting and a skill for fighting a spider are near or far by whatever the embedding model decided, and the query cannot say "near in purpose" as opposed to "near in materials."

Phical has that notion built in. A word has channels, and each channel is a kind of relation. The quest names five: is-a, part-of, near, used-for, unlike. When the loop asks about a word, it asks under a kind, and the answer is written to the phase on that kind's channel. Retrieval under a kind reads that channel and no other. The relation channel is a coordinate in the literal sense: which axis of the word you are reading.

This is where the quest's measurements say something sharp. On the synthetic oracle, the shipped trainer scored 0.082 MRR and the attraction-only trainer with dithered rounding scored 0.594 MRR, hit@1 34.8%, hit@5 97.4%, against chance 0.021. Within-kind retrieval scored 0.747. Reading the wrong channel scored 0.015, and reading with an offset scored 0.005. Both of those are below chance. The channel is not a decoration. Read the right one and the word is found; read the wrong one and you do worse than guessing.

Voyager cannot report a "wrong channel" number because it has no channels. Its retrieval is either close enough or not. If we were to add channels to a Voyager-style library, each skill would carry several embeddings, one per kind of use, and the query would name a kind. The paper does not do this, and we do not know whether it would help. What we can say is that in phinet the kind is the thing that makes retrieval work at all, and that the cost of ignoring it is measured: 0.594 to 0.015.

There is a second reading of Voyager's "environment feedback" that fits here. When the query to the library includes what the world just said, the query is being conditioned. That is c in X ⇒ Y under c. Voyager conditions by concatenating text. Phical conditions by choosing a channel. The two are the same move at different resolutions, and Phical's is the coarser and more legible one.

## 8. The operator as X ⇒ Y under c, and the step of the loop

Voyager's loop is a step operator, and we can write it out. X is the pair of world state and library. c is the task the curriculum produced. The operator runs code, reads feedback, revises up to four times, and returns Y, the new world state and the library with one more skill or none. The paper's own description is that the process "terminates when verification succeeds," and then the skill is added and the curriculum is asked again.

Phient's quest is a step operator of the same form. X is the current relation store. c is a word and a kind. The loop asks the model for one word under that kind. The answer is a target. The trainer moves the phase of the asked word on that channel toward the target's phase, by an amount that depends on the distance, and updates the count. Y is the store with that pair moved.

Two things about the two operators are the same. The model is frozen in both. And the state that accumulates is outside the model in both. Voyager says this bypasses fine-tuning and alleviates catastrophic forgetting. The quest says the same in its own way: the model is asked, not trained, and the store is what learns.

Two things are different, and the differences are instructive. First, Voyager's step is expensive and rare: 160 iterations is the whole run, and each iteration may call GPT-4 several times. The quest's step is one word, and the phiecs oracle gives 1,280 rows over 256 words. The scale of the loop is different by orders of magnitude, in opposite directions: Voyager takes few large steps, the quest takes many tiny ones. Second, Voyager's step either adds a skill or does not; the library grows by whole units. The quest's step moves a phase by a fraction of a circle. Growth in phinet is continuous, and it has a floor. The facts file records that a channel is one byte, 256 quanta of 2π, and a nudge of 0.06·sin d falls under half a quantum once sin d < 0.2, about 11°. Nearest rounding froze every pair 8 quanta short. Dither fixes that in the quest and is what makes 0.594 possible, but it costs corpus attribution, 71% to 65%, so it ships off by default.

Voyager has no such floor because its step is discrete. Its analogue of the deadband is the four-round limit: a skill that cannot be made to verify in four tries is dropped and the curriculum moves on. Both are places where the loop decides that small progress is no progress. Both are set by hand.

## 9. Where the skill library lands: phiecs, the quest, and the store

The skill library lands in phiecs. phiecs is the entity-component world plus the quest. Its entities are words and its components are the channels. The quest is the loop that fills them. This is not a loose analogy; it is the same data structure with a different payload. Voyager's library is a set of records, each with an embedding key and a code body. The quest's store is a set of words, each with 32–64 phasor-and-count pairs. Both are appended to by the loop and read by the loop.

Where they differ is what a record can do. A Voyager skill can be executed. It changes the game world. A Phical relation can only be read and compared. The quest's output is a ranking, and its quality is MRR and hit@k. The library's output is items and distance. There is no run of the quest that changes anything outside the store.

The automatic curriculum lands in the quest as the choice of which word and which kind to ask next. Voyager's curriculum asks GPT-4 for a task given history. The quest's curriculum, as run so far, is the oracle's row order: 1,280 rows over 293 target words. That is a fixed curriculum, not an automatic one. This is a real gap and we do not paper over it. The quest has no proposer. It has a list.

The iterative prompting with self-verification lands in the count. When a pair is moved, its count goes up. A relation that has been confirmed many times has a high count and a phase that has stopped moving. A relation that was asserted once has a count of one and a phase that may be anywhere. Voyager's verifier says yes or no once, and a skill is kept or not. The quest's verifier is the accumulation of agreement over many asks. Neither is a proof. Voyager reports a verifier that missed spider string. The quest reports wrong-channel reading at 0.015 and offset reading at 0.005; if the loop wrote to the wrong channel, the count would rise on a relation that does not help.

The helix is the shape the store takes when it is written out. The facts file records a lexicon as a DNA-coded tree: 64 codons as base-4 ACGT of channel, with hexagram, sphere degree and order, B-DNA geometry at 10.5 codons per turn and 0.34 nm rise, and a PhiTon wavelength, band and hex per codon. 18,496 words round-trip byte-exact in 1.9 s and 75 MB. Voyager's library is a vector database and a directory of JavaScript files. Both are the serialised memory of a frozen-model loop. The helix carries no code and executes nothing. It carries relations and counts. That is the entire difference between the two stores, and it is the difference between an agent and a lexicon.

## 10. Where the loop lands: phiano, the sphere, and the graph

phiano is where the quest's output is used. Facet, Trainer, generation, personas and quest ingest are all in phiano. Quest ingest is the path by which a store built in phiecs becomes usable for ranking and generation. So the analogue of Voyager's transfer experiment, where the library built in one world is used in a fresh one, is quest ingest followed by a task the quest never saw. The facts file has one such measurement: held-out 20% of the oracle, one hop 0.011 MRR, two hops is-a 0.095 and used-for 0.093, part-of, near and unlike roughly at chance. One hop is below chance of 0.021. Two hops in two kinds is about four times chance. That is the honest state of transfer in the quest. It is not the 3 of 3 Voyager reports on unseen tasks, and we do not pretend otherwise.

The graph is where Voyager's tech tree lives. A tech tree is a directed graph of prerequisites: wood before stone, stone before iron, iron before diamond. Voyager climbs it and reports iterations per level. Phical's graph is the relation graph: word to word under a kind. The quest's synthetic oracle is families of 8 words with five kinds, which is a small graph with a known structure. The tech tree has depth; the oracle has breadth. Voyager's two-hop result, iron after stone after wood, is the same kind of quantity as the quest's two-hop held-out score, and both are the first place where composition is tested. Voyager reaches iron in 21 ± 7 iterations. The quest reaches two-hop is-a at 0.095 MRR. These numbers are not comparable and we will not put them in one table.

The sphere is not in Voyager. Voyager's embedding space is a flat vector space of whatever dimension ada-002 gives; the paper does not use its geometry beyond nearness. phinet's sphere tie, one complex weight per spherical-harmonic degree with 64 channels as Y_l^m for l < 8, is a different kind of index: 1,192 params against the circle's 8,360, Austen hinge 12.8× valid and 13.1× test at 3 epochs against 14.7× and 14.6×, in 89 s against 142 s, and on the synthetic degree-filter task MRR 1.000 against the circle's 0.662. If a Voyager-style library were indexed on a sphere by degree, low-degree nearness would mean coarse purpose and high-degree nearness would mean fine detail. That is speculation and we mark it as such. No such library exists in gemphi, and the sphere at softmax and wide+deep is unrun.

The agent loop itself lands only partly. The quest asks and stores. It does not act. There is no world in phiecs that pushes back the way Minecraft pushes back; the oracle answers, it does not fail. That is why Voyager's execution errors have no analogue here. The nearest thing is the deadband: the place where the world, in this case the byte, refuses a step.

## 11. What Phient has already measured that bears on it

Everything in this section is from the facts file, and there is nothing else to cite.

The quest is the loop that bears directly. On the phiecs synthetic oracle, 256 words in families of 8, kinds is-a, part-of, near, used-for and unlike, 1,280 rows, 293 target words, chance MRR 0.021: the shipped trainer scored 0.082 MRR. Attraction-only training with dithered rounding scored 0.594 MRR, hit@1 34.8%, hit@5 97.4%. Within-kind retrieval scored 0.747. Wrong-channel reading scored 0.015 and offset reading 0.005. Held out 20%: one hop 0.011; two hops is-a 0.095, used-for 0.093, part-of, near and unlike about chance.

Three of those bear on Voyager's three ablations. Voyager says removing the curriculum costs 93% of items. The quest has no automatic curriculum, so there is no matching number; the oracle's row order is fixed. Voyager says removing self-verification costs 73% of items. The quest's nearest measurement is the trainer comparison: 0.082 to 0.594 is the gain from changing how a confirmed answer is written into the store, which is the closest thing the quest has to changing how verification is applied. Voyager says removing the library makes the agent plateau late. The quest's nearest measurement is held-out transfer, one hop 0.011, which says that the store as it stands does not yet carry to words it was not asked about.

The byte deadband bears on the loop's step. A channel is one byte, 256 quanta of 2π. The nudge 0.06·sin d falls under half a quantum once sin d < 0.2, about 11°. Nearest rounding froze every pair 8 quanta short. Dither fixes the freeze in the quest and costs attribution 71% to 65%, so it is off by default. This is the quest's equivalent of Voyager's four-round limit: the point at which a small step is not taken.

The operator results bear on what a learned relation is worth on real text rather than an oracle. On Austen, pool 1,228 words seen 20+ times, 204,592 training examples, chance MRR 0.0063: identity 7.9× chance, hinge with 8,360 params 14.7× valid and 14.6× test, softmax β=8 18.0× and 18.8×, softmax wide+deep with 50,016 params 27.6× at 3 epochs and 30.7× valid, 30.3× test at 8 epochs, hit@10 0.348. Decoding on 40 held-out prompts times 30 tokens: trigram sampler KN perplexity 13.3, operator nearest-k 66.9, union 66.8; shared 4-grams sampler 25.7%, operator 8.7%. Persona attribution across Chesterton, Austen and Carroll on held-out 4-sentence paragraphs: 71% overall, 74, 65 and 86, chance 33%. Dialogue: 18 replies judged as their persona 44% by sector reader and 72% by Kneser-Ney panel.

What is not measured: no Phi-4 run, so the quest has only asked an oracle and never a model; no weather work; no sphere at softmax or wide+deep; no partnership with anyone. phinet matches PyTorch on 62 of 62 parity tests, and the engine is allocation and bandwidth bound at these sizes, 1 MB op in 2–5 ms.

## 12. The experiment that would test the paper's idea inside phinet

Voyager's central claim, stripped of Minecraft, is this: a frozen model plus an external store that is retrieved by nearness and appended by a verified loop does better over time than the same model without the store, and the store transfers to tasks the loop never saw. The quest already has the model, the store and the loop. What it lacks is the ablation and the automatic curriculum. The experiment is to add both, and to measure what Voyager measured.

The setup is the phiecs oracle as it stands: 256 words, families of 8, five kinds, 1,280 rows, held-out 20%. The trainer is attraction-only with dithered rounding, since that is the configuration that reached 0.594 MRR. The metric is MRR and hit@k on the held-out split, both one hop and two hops, by kind. These are the numbers the facts file already reports, so the experiment extends a table rather than inventing one.

The treatment is a curriculum. Instead of walking the oracle's rows in order, the loop chooses the next word and kind from the store's own state: prefer words with low counts, prefer kinds with low within-kind MRR so far, and prefer pairs that would extend a two-hop chain. That is a hand-written curriculum, not a model-written one, and it is the honest analogue of Voyager's, since the quest has no Phi-4 run and cannot ask a model to propose.

The control is the same loop with a random row order over the same budget of asks. Voyager's control was exactly this: "if the curriculum is replaced with a random one." If the curriculum treatment does not beat random on held-out two-hop MRR, the paper's first ablation does not carry to phinet.

The second treatment is the library ablation. Run the loop with the store reset every k asks, for k in a small set, and measure held-out MRR at the end. Voyager reports plateau without the library. The prediction, if the analogy holds, is that a store reset often enough gives held-out one-hop near chance, which is where the quest already is, and that a store never reset gives higher two-hop scores. If the never-reset store does not beat the reset store on two hops, the store is not doing what a library does.

The third treatment is verification. The quest's count is its verifier. Run the loop with counts frozen at one, so every ask moves the phase by the same amount regardless of history, and compare held-out MRR. Voyager's −73% is the number to compare shape against, not value.

Every number from this experiment would be new and would need to be measured before it is cited. We predict nothing beyond the direction, and we would report the direction either way.

## 13. Where we disagree, or where the analogy breaks

The analogy breaks in four places, and the first is the biggest. Voyager's skills execute. A skill is a program that changes the world, and its value is the world it produces. Phient's relations rank. A relation moves a phase and its value is whether the right word comes up first. The word "compositional" in Voyager's abstract means that a skill can call another skill. The nearest thing in Phical is a two-hop chain, and the quest's two-hop numbers are 0.095 and 0.093 in two kinds and chance in three. Composition in Voyager is what makes a diamond pickaxe. Composition in the quest is barely above noise. We will not use the one word for both without saying this.

The second break is the curriculum. Voyager's is its most valuable part by the paper's own ablation, 93% of items. The quest has none. Its curriculum is the oracle's row order. Anything we say about interpreting the automatic curriculum is an interpretation of something we have not built.

The third break is the verifier. Voyager's verifier is a second call to the same model, and the paper is candid that it errs. The quest's verifier is the oracle, which does not err, because it is the ground truth. That makes the quest's 0.594 an easier number than any Voyager reports. When a Phi-4 run exists, the verifier will be a model, and every number in section 11 that came from the oracle will need to be measured again. The facts file says this in one line: no Phi-4 run yet, synthetic oracle only.

The fourth break is the direction of scale. Voyager takes few large steps and reports items. The quest takes many small steps and reports ranks. Voyager's 160 iterations produce 63 items; the quest's 1,280 rows produce 0.594 MRR. There is no exchange rate between an item and an MRR point, and we have not proposed one.

Where we disagree with the paper is narrower. We think the paper's own ablations argue against its abstract's emphasis. The abstract leads with the skill library as the mechanism of lifelong learning. The ablations show the curriculum and the verifier each cost more within the run, and the library shows its value mainly in transfer and in the late plateau. That is not a contradiction, but it is a different story from the one the title tells, and a reader interested in why the loop works should read Figure 9 before the abstract. In Phical terms the same point is: the store is what learns, but the choosing and the checking are what make the store worth reading. The quest's 0.082 to 0.594, achieved by changing how answers are written and not what is stored, is our version of the same lesson.

## 14. The person and the moment

Everything in this section is from the profile, and we add nothing to it.

Voyager is a 2023 paper with Anandkumar as senior author and Linxi Fan beside her; both are at NVIDIA, where she has been senior director of machine-learning research since 2018 and opened the core AI/ML lab in Santa Clara. The paper sits between the two strands the profile describes. Her first decade was tensor methods, the recognition that moments of a latent-variable model form a tensor whose decomposition recovers the model with guarantees. Her second was operators: the Fourier Neural Operator in 2020 and FourCastNet in 2022, which does a week's forecast in under two seconds and is roughly 45,000× faster than numerical weather prediction on a node-hour basis. Voyager is neither. It has no tensor decomposition and no operator. It is a paper about what a frozen language model can do with a store and a loop.

The profile records her stated view: "We have foundation models for language, not for physics." Voyager is a paper that uses the language foundation model as it is and asks what can be built around it. The method she describes for her own work, "All of the things that work with deep learning, let's take them, but make them a bit more principled," is visible in Voyager's ablations, where each part is removed and its cost reported. The paper does not claim more than those ablations support.

The moment in which we write is stated in the profile. On 16 March 2026 the United Nations appointed her to its Scientific Advisory Board. She was named to Time100 AI in 2026 and received the Time100 Impact Award in 2025 for using AI to accelerate scientific discovery. She co-founded Accelerated Understanding in 2026. The profile also records that she campaigned to rename NIPS to NeurIPS and petitioned IIT Madras on gender segregation in admissions.

The profile is explicit about what is not claimed, and we repeat it. Nothing here is her endorsement of Phient. No partnership has been agreed with anyone. The profile says a partnership is something to earn with a result she would recognise, measured the way she measures. Voyager gives us the shape of such a result for the quest: an ablation table with the curriculum, the store and the verifier each removed, and a transfer table on tasks the loop never saw. Sections 11 and 12 are what we have and what we would need.

## 15. What remains

What remains is mostly on our side. Voyager is a finished paper with its code released, and we have taken from it a shape. The shape is the loop that proposes, acts, checks and keeps, and the store that is retrieved by nearness. The quest has the loop and the store. It does not have the proposer, and its checker is an oracle rather than a model.

The first thing that remains is the Phi-4 run. The facts file says there is none. Until there is, every quest number is a number about an oracle, and the analogy to Voyager, which asks a real model, is an analogy in form only. We say this because the last section of every interpreted paper should say what would make the interpretation real, and this is it.

The second is a curriculum. Section 12 describes a hand-written one. Voyager's is model-written, and the paper's own numbers say it is the most valuable single part. A quest without a proposer is a quest that reads a list. When a model is asked to choose the next word and kind, the store's own state, low counts and low within-kind MRR, is the natural prompt.

The third is transfer. Held-out one hop at 0.011 is below chance and is the number that must move before any claim about a store that carries. Two hops at 0.095 and 0.093 in two kinds is the seed of such a claim and no more.

The fourth is the deadband. Dither is what got the quest to 0.594 and is off by default because it costs attribution. Voyager has a four-round limit that plays the same role, set by hand and reported as a limitation. A loop that knows when a small step is worth taking, in bytes, without dither, would remove the trade.

The fifth is the sphere. Nothing in Voyager touches it. The sphere tie at 1,192 params against the circle's 8,360, and 1.000 against 0.662 on the degree-filter task, is a result about indexing that a library could use. Whether it helps a store that is retrieved by kind is unmeasured, and the sphere at softmax and wide+deep is unrun.

What does not remain is any claim that Phient has built an embodied agent. It has not. There is no world in phiecs that pushes back, no code that executes, no items. There is a store of relations, a loop that fills it from an oracle, and a set of numbers, some good and some at chance. That is the honest inventory, and Voyager's own honesty about spider string and copper swords is the standard we hold ourselves to in reporting it.

## 16. Sources

Fetched and read for this paper.

Wang, G., Xie, Y., Jiang, Y., Mandlekar, A., Xiao, C., Zhu, Y., Fan, L., Anandkumar, A. *Voyager: An Open-Ended Embodied Agent with Large Language Models.* arXiv:2305.16291, v1 25 May 2023, v2 19 October 2023. Abstract page: https://arxiv.org/abs/2305.16291

Full text, v2, as read for sections 2 to 5: https://arxiv.org/pdf/2305.16291v2

The paper's arXiv comment names a project website and open-source codebase at https://voyager.minedojo.org/. We did not fetch that site and cite it only as the paper cites it.

Phient facts: `PHIENT_FACTS.md` in this folder. Every Phient number above is from that file and no other.

Profile: `../17_ANIMA_ANANDKUMAR_PROFILE.md`. Every fact about the person in section 14 is from that file. The profile's own sources are the Caltech News item of 16 March 2026, the Latent Space interview, the Wikipedia article, the FourCastNet arXiv page 2202.11214, the Blavatnik Awards profile and the Caltech CMS faculty page; we did not refetch those for this paper and make no claim from them beyond what the profile states.

Numbers quoted from the paper and their location in it: three key components and headline ratios, abstract; models and temperatures, section 3.1; four-round limit, section 2.3; 63 items in 160 iterations, Figure 1 discussion; Table 1 tech tree and 15.3×, 8.5×, 6.4×, section 3.3; 2.3× distance, section 3.3; Table 2 unseen tasks, section 3.3; ablations 93%, plateau, −73%, 5.7×, section 3.4; limitations on cost, verifier, hallucination and vision, the limitations section. Section numbers follow v2 as fetched.
