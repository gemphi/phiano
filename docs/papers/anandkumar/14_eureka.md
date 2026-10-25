# The reward is a sentence about the state: Eureka, read as an operator over code

*Phient interpreted papers, 14 of 16 · 2026-07-13 · on: "Eureka: Human-Level Reward Design via Coding Large Language Models" (Ma, Liang, Wang, Huang, Bastani, Jayaraman, Zhu, Fan, Anandkumar, 2024, ICLR 2024; arXiv:2310.12931)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the fourteenth of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. The paper is not a neural operator paper and not a physics paper. It is a paper about reinforcement learning, and about one part of reinforcement learning that is usually done by hand: writing the reward function. Its claim is that a large language model that can read code and write code can write reward functions for robots, that it can improve them by looking at what the trained policy did, and that the rewards it arrives at beat the rewards written by the people who built the benchmark. The paper reports this on 29 tasks and says the model wins on 83% of them.

We read it for a reason that is narrower than the paper's ambitions. Eureka never sees the robot. It sees the source code of the environment, which is a text, and it writes a function, which is another text. It then reads a short table of numbers, which is a third text, and writes again. The whole loop lives in language. The thing that moves the loop is a summary of how a policy behaved under a reward, written as words and numbers, and fed back as the condition for the next attempt. In Phical's terms this is a word moving under a condition, where the word is a reward function and the condition is the reflection. The paper is, for us, a demonstration that the condition can be a compressed record of what happened, and that such a record is enough to steer a search.

Sections 2 to 5 say what the paper claims, its setting and its numbers, taken from the arXiv abstract and HTML text. Sections 6 to 10 place its objects in the Phient family: the reward function as a word's function, the reflection as a coordinate on a relation channel, the sampling step as an operator X ⇒ Y under c, the environment code as the closed set the quest oracle stands for, and the evolutionary loop as the online half of the helix. Section 11 lists what Phient has measured that bears on this. Section 12 names one experiment inside phinet with its control. Section 13 says where the analogy breaks, and it breaks on the fitness function, which Eureka has and Phient does not. Section 14 is about the person and the moment. Section 15 is what remains.

One limit is stated up front. Phient has no reinforcement learning, no simulator, no policy, and no access to a coding language model inside gemphi. Every number about Phient below was measured for another purpose. Where the paper's method would need code that does not exist, this document says so.

## 2. What the paper claims, in its own terms

The paper opens by conceding what language models are good at and what they are not. They have, the abstract says, "excelled as high-level semantic planners for sequential decision-making tasks." But "harnessing them to learn complex low-level manipulation tasks, such as dexterous pen spinning, remains an open problem." The proposal is to keep the language model out of the control loop and put it one level up, where the reward is designed.

Eureka is described as "a human-level reward design algorithm powered by LLMs." It "exploits the remarkable zero-shot generation, code-writing, and in-context improvement capabilities of state-of-the-art LLMs, such as GPT-4, to perform evolutionary optimization over reward code." The rewards it produces are then used by ordinary reinforcement learning to train a policy. The abstract makes a point of what is not supplied: "Without any task-specific prompting or pre-defined reward templates, Eureka generates reward functions that outperform expert human-engineered rewards."

The scale of the claim is given in one sentence. "In a diverse suite of 29 open-source RL environments that include 10 distinct robot morphologies, Eureka outperforms human experts on 83% of the tasks, leading to an average normalized improvement of 52%." The 29 tasks are 9 Isaac Gym environments and 20 tasks from the Bidexterous Manipulation benchmark, which the paper calls Dexterity.

Two further claims follow. The first is that the same loop admits a person. Because the model improves rewards by reading text, a human can write that text. The paper calls this "a new gradient-free in-context learning approach to reinforcement learning from human feedback (RLHF), readily incorporating human inputs to improve the quality and the safety of the generated rewards without model updating." The second is a demonstration. "Using Eureka rewards in a curriculum learning setting, we demonstrate for the first time, a simulated Shadow Hand capable of performing pen spinning tricks, adeptly manipulating a pen in circles at rapid speed."

The method has three named parts. Environment as context: the environment's source code is put into the prompt so the model can "zero-shot generate executable reward functions." Evolutionary search: the model "samples several independent outputs from the LLM" per iteration, so that "the probability that all reward functions from an iteration are buggy exponentially decreases," and the best is carried forward. Reward reflection: the training run is summarised by "scalar values of all reward components and the task fitness function at intermediate policy checkpoints," and that summary is put back into the prompt.

One more claim is made about what the rewards look like. The paper says Eureka "mostly generates weakly correlated reward functions that outperform the human ones," and that "in a few cases, Eureka rewards are even negatively correlated with human rewards but perform significantly better." That is the whole claim. A model that reads code, a loop that samples and selects, a summary that closes the loop, and a benchmark on which it beats its authors.

## 3. The mathematics, plainly

There is not much mathematics in the paper, and what there is sits in the problem statement. The paper distinguishes two problems. The reward design problem is the classic one: given a world model, a fitness function F, and a learning algorithm, find a reward R so that the policy the algorithm returns under R scores well under F. The paper writes the target as the policy 𝒜_M(R) and asks that F(𝒜_M(R)) be maximised. F is the ground-truth task metric. For a hand reorienting a cube it is the number of successful reorientations; for a running robot it is speed. R is a shaped, dense proxy that the algorithm can follow. The gap between the two is the reason reward design is a job.

The reward generation problem is the paper's addition. Instead of a reward function, the input is a text description of the task and the source of the environment, and the output is a program. The search space is code. That is the move. A program is not a vector of weights, so the objective cannot be descended. It has to be searched.

The search is what the paper calls evolutionary, and it is simple. At iteration i the model is given the environment code, the task description, and the reflection from iteration i minus one. It returns K programs. Each program is a reward function. Each is compiled and trained with PPO for a fixed budget. Each trained policy is scored under F. The best program is kept as the parent for iteration i plus one. The standard configuration is K equal to 16 samples per iteration and N equal to 5 iterations, with the whole procedure repeated in 5 independent runs per environment.

The reflection is the only part of the algorithm that is not a sample or a maximum. During training the scalar value of every term in the reward function, and the value of F, are logged at a fixed set of checkpoints. These are written into the prompt as a short table. The model is asked to read the table and change the reward. The paper's argument for why this works is that a single fitness number is a credit-assignment problem, since it does not say which term of the reward helped or hurt, while the per-term trace does.

The human-normalised score for the Isaac tasks is (Method − Sparse) / |Human − Sparse|. A score of 1 means matching the human reward and 0 means matching the sparse reward. On Dexterity the metric is success rate. Nothing in the method is differentiable end to end, and nothing about the language model is updated. The only learned object is the PPO policy, and it is thrown away at the end of each iteration except for its score and its trace.

## 4. Its experiments and numbers as reported

The benchmark is 29 tasks over 10 robot morphologies. The 9 Isaac Gym environments, with observation and action dimensions as the paper gives them, are Cartpole (4, 1), Quadcopter (21, 12), FrankaCabinet (23, 9), Anymal (48, 12), BallBalance (48, 12), Ant (60, 8), AllegroHand (88, 16), Humanoid (108, 21), and ShadowHand (211, 20). The remaining 20 tasks are from the Dexterity benchmark. The backbone language model is GPT-4, which the paper identifies as gpt-4-0314.

Three baselines are used. Human is the shaped reward written by the benchmark designers. Sparse is the binary success function on Dexterity or the task metric on Isaac. L2R is a prior method that has the language model fill a template of pre-defined reward primitives. The paper notes that L2R was given "access to the original reward functions," which is an advantage Eureka does not have.

The training protocol is fixed. "For each final reward function obtained from each method, we run 5 independent PPO training runs and report the average of the maximum task metric values achieved from 10 policy checkpoints sampled at fixed intervals."

The headline numbers are the ones in the abstract: 83% of tasks above human, 52% average normalised improvement. The text gives the split. "Eureka exceeds or performs on par to human level on all Isaac tasks and 15 out of 20 tasks on Dexterity." Against L2R, the paper says L2R is "comparable on low-dimensional tasks (e.g., CartPole, BallBalance)" but "lags significantly behind on high-dimensional tasks."

The ablations are two. Without reward reflection, the paper says the average normalised score on Isaac drops by 28.6%. Without evolution, in a variant that draws 32 samples in one shot so that the budget is matched, the paper reports that Eureka's iterated rewards "steadily improve and eventually surpass human rewards in performance despite sub-par initial performances," while the one-shot variant's "performances are lower than Eureka after 2 iterations."

The human-in-the-loop results are also two. Starting the search from the human reward, which the paper calls Eureka with human initialisation, "is uniformly better than both Eureka and Human on all tasks" on Dexterity. Replacing the automated reflection with text written by a person, for a Humanoid running task, produced an agent that "despite running a bit slower" was "preferred by a large majority" of 20 unfamiliar users who watched policy videos.

The pen spinning result is a curriculum. First Eureka is used to write a reward for reorienting a pen to random target poses, and a policy is pre-trained on that. Then the same reward is used to fine-tune the policy toward the sequence of poses that make up a spin. The paper reports that neither the pre-trained policy nor a policy trained from scratch "can complete even a single cycle of pen spinning," while the fine-tuned policy spins the pen for many cycles.

The paper does not give, in its text, a count of negatively correlated rewards or an aggregate Dexterity success rate. Those are in figures.

## 5. The one idea, restated

The one idea is that a reward function is a small program, that a model which writes programs can write it, and that the model can be made to write it better by showing it what happened. Everything else in the paper is an arrangement for making that loop reliable: sample many so that at least one compiles, keep the best, and summarise training as a trace of the reward's own terms rather than as a single score.

Restated in Phient's vocabulary, the idea is this. There is an object, the reward, that was thought to be a matter of expert taste. Eureka shows that it is a matter of relation. The reward is defined by what it does to the policy, and what it does to the policy can be read off as a handful of scalars over time. Give a model those scalars and it can move the reward toward a better one. The reward has no meaning apart from the trace it produces under training, and the trace is enough to steer it.

The paper's own finding that its rewards are weakly correlated with the human ones is the part we take most seriously. If the search had recovered the human rewards, the result would be a compression story. It did not. It found rewards that score higher under F while looking different in the space of reward values. That means the human rewards were one point in a large set of rewards that solve the task, and not a special point. The set is defined by F and the training algorithm, not by any intuition about what a good reward should contain.

Phical's founding sentence is that a word is nothing but what it relates to. Eureka's operational version of that sentence is that a reward is nothing but what it does to training. Both refuse to store the thing and store its effect instead. The rest of this document takes that parallel as far as it goes and stops where it stops.

## 6. The reward function as a word's function

In phinet a word is a function, not a point. It is 64 relation channels, each a complex phasor on a closed circle with a count, and the count is the number of times the relation has been observed. The word has no coordinates of its own. It is the set of its relations.

The reward function in Eureka is also a function in the ordinary sense, since it maps state to scalar, but that is not the sense that matters here. What matters is that Eureka never treats the reward as an object with intrinsic content. The prompt does not ask the model what the reward means. The evaluation does not inspect the program. The reward is judged entirely by the trace it produces when a policy trains under it: the scalar values of its own terms at ten checkpoints, and the fitness of the policy at each. That trace is the reward's set of relations. The reward is what it does to a policy under a training budget. It is the word.

The analogy can be pushed one step. A Eureka reward is a sum of terms, each a named component: a distance penalty, a velocity bonus, a term for the angle of a joint. The reflection logs each term separately. So the reward, as the model sees it in the reflection, is a small number of channels, each carrying a trajectory over checkpoints. The model reads that and rewrites the program. In Phical the channels are fixed at 64 and are relation kinds; in Eureka they are however many terms the model chose to write, and they are reward components. The count of channels differs, and what they mean differs, but the structure is a word as a bundle of channels, each read by its motion.

Where the analogy is weak is at the circle. A Phical channel is a phasor on a closed circle with 256 quanta. A reward term is an unbounded scalar. The paper's reflection table gives raw values and the model is left to judge scale. There is no closed geometry in which a reward term lives, and no notion of a term wrapping around. Phient's byte deadband, where a nudge of 0.06·sin d falls under half a quantum once sin d < 0.2, has no counterpart in Eureka and nothing in the paper suggests one is wanted. The reward does not need to be small to be stored, because it is stored as source.

So the mapping is partial. The reward is a word in that it is defined by its relations. It is not a word in that its relations are not phasors and are not counted. Phient would say Eureka has found the first half of Phical, the refusal to store a point, without the second half, the closed circle that makes movement bounded and additive.

## 7. The reflection as a coordinate on the relation channel

Phical's condition c is a coordinate. It selects which relation channel is read when two words are moved, and it is the reason a word can mean one thing under one condition and another under another. The quest oracle in phiecs is built on this: five kinds, is-a, part-of, near, used-for, unlike, each a channel, and a query names the kind.

Eureka's condition is the reflection. It is a text, composed of the previous reward program, its per-term trace at checkpoints, and the fitness values. This is what the model is given at iteration i that it was not given at iteration i minus one. The environment code and the task description are constant. The reflection is the only thing that moves the prompt, and so it is the only thing that selects what the model writes next. It plays the role of c.

Phient can be precise about what kind of coordinate it is. In the quest, c is discrete and small: one of five kinds. The oracle measured that reading the right channel gives 0.594 MRR with attraction-only training and dithered rounding, and reading the wrong channel gives 0.015, near the 0.021 chance floor. The condition is a switch, and the whole result depends on throwing it correctly. In Eureka the condition is not a switch. It is a rich record with many degrees of freedom, and the paper's ablation says it matters by 28.6% of normalised score on Isaac. That is the paper's version of Phient's wrong-channel number: remove the coordinate and the search falls back toward the fitness-only signal, which the paper argues suffers from credit assignment.

The paper's human-feedback experiment is a second reading of the same idea. The reflection can be written by a person instead of generated from the trace. When a person wrote it for the Humanoid running task, the resulting gait was slower but preferred by 20 viewers. That is a condition supplied from outside the system changing which reward is arrived at, which is what Phical says a condition should do. The word did not change. The environment did not change. c changed, and Y changed with it.

The limit is that Phical has one c per query and Eureka has one c per iteration. In phinet, c indexes a channel and the channel is read. In Eureka, c is a paragraph, and the model interprets it in a way the paper does not open up. The paper reports what the reflection contains and what removing it costs, and not how the model uses it. Phient can say what its channel is, since it is a byte at a known index. Eureka cannot say what part of the reflection moved the reward. The coordinate is there, but it is not a coordinate anyone can point to.

## 8. The sampling step as X ⇒ Y under c

Phient's operator is X ⇒ Y under c: given a word's function X and a condition c, produce Y, the function the next word should have. On Austen the softmax wide-and-deep operator with 50,016 parameters ranks the held-out next word at 30.7× chance on the validation set and 30.3× on test after 8 epochs, hit@10 0.348. The operator is trained by gradient on 204,592 examples.

Eureka's operator is the language model's forward pass. X is the parent reward program. c is the reflection. Y is a new reward program. The paper does not train this operator. It takes it as given, in the form of gpt-4-0314, and uses it 16 times per iteration. The operator is the same at iteration 5 as at iteration 1. All the improvement comes from selection, from keeping the best of 16 and feeding its trace back as c.

That is the structural difference from phinet and it is worth stating exactly. In phinet the operator is learned and the condition is fixed by the data. In Eureka the operator is fixed and the condition is what learns, in the sense that the reflection accumulates what the search has found. The paper's ablation, in which 32 samples drawn at once do worse than 16 samples drawn twice with a reflection in between, is a measurement that the condition carries information the operator cannot generate on its own. Phient has an analogous measurement from the opposite side. The quest oracle with the shipped trainer reaches only 0.082 MRR, and with attraction-only training and dithered rounding reaches 0.594. Same data, same channels, same conditions; the operator changed. Phient's gains have come from the operator. Eureka's gains come from c.

There is a second way to see the sampling step. Sixteen samples from a fixed model under a fixed prompt are sixteen draws from a distribution over programs. Phient's decoding experiment measured such a spread in a smaller way: 40 held-out prompts, 30 tokens each, operator nearest-k at Kneser-Ney perplexity 66.9 against the trigram sampler's 13.3, with shared 4-grams across prompts at 8.7% for the operator against 25.7% for the sampler. The operator was more varied and less fluent. Eureka's loop is a case where variety is the point, since a wider spread of 16 programs makes a better parent more likely. The paper does not report the spread. It reports that the probability of all 16 being buggy falls exponentially with K.

The mapping is therefore: Eureka's operator is X ⇒ Y under c with a frozen operator and a learned c; phinet's is X ⇒ Y under c with a learned operator and a fixed c. Both are the same shape. They put the learning in different places.

## 9. Where the environment code lands: the quest oracle as a closed world

The paper's first component, environment as context, is the one Phient recognises most directly. Eureka gives the model the source of the environment, minus the reward, so that the model knows what state variables exist and what they are called. The paper says this is what lets the model "zero-shot generate executable reward functions," and the reason is plain. A reward is a function of state, so a reward writer must know the state. The source is a closed, complete description of that state. There is nothing about the environment the model needs that is not in the file.

Phient's quest is also a closed world. The phiecs oracle is 256 words in families of 8, with five relation kinds, 1,280 rows, and 293 target words. It exists so that phinet can be trained and tested against a ground truth that is fully known, with chance MRR 0.021 computed exactly. The reason it is synthetic is the reason Eureka gives the model the source: when the world is closed, every failure is the learner's, and every score is a score against the truth.

The correspondence is closer than it looks. Eureka's environment file lists observation dimensions, and the paper gives them: 4 for Cartpole, 211 for ShadowHand. The reward the model writes can touch any of those dimensions by name. The oracle's 256 words each have 64 channels, and a query can touch any channel by kind. In both, the reward writer, or the operator, is not asked to discover what the state is. It is asked to say how the state should move. The reward says which combination of state variables should increase. The operator says which channel should rotate toward which phase.

What Phient has measured in that closed world bears on Eureka's ablation. The oracle's held-out 20% shows one-hop generalisation at 0.011 MRR, below chance of 0.021, and two-hop generalisation at 0.095 for is-a and 0.093 for used-for, with part-of, near and unlike at about chance. So phinet, given the closed world, learns the training rows well and generalises across families poorly. Eureka, given the closed environment, writes rewards that generalise across 5 PPO seeds, since that is what the protocol averages over, but the paper does not test generalisation of a reward to a different environment. It could not, since the reward names the environment's own variables. Both systems are strong inside the closed world they were shown. Neither paper, and neither measurement, says anything about the open one.

On Austen the world is not closed and the operator is judged by rank against a pool of 1,228 words seen 20 or more times. There is no analogue of that in Eureka, because there is no analogue of a corpus. The environment is the whole world, and the model reads all of it.

## 10. The evolutionary loop as the online half of the helix

Phient's helix is a lexicon written as a DNA-coded tree: 64 codons, base-4 ACGT of the channel index, a hexagram per codon, a spherical-harmonic degree and order, B-DNA coordinates at 10.5 codons per turn and 0.34 nm rise, and a PhiTon wavelength, band and hex per codon. It serialises 18,496 words in 1.9 seconds to 75 MB with a byte-exact round trip. It is a storage form. It is not a learning loop. The helix has no online half.

Eureka's loop is nothing but an online half. Each iteration takes a parent, produces 16 children, trains each, scores each, and keeps one. The parent for the next iteration is that one plus its trace. Over 5 iterations the lineage is a chain of 5 rewards, each descended from the last, each better under F or the search would have kept the parent. The paper's figure showing rewards that "steadily improve and eventually surpass human rewards in performance despite sub-par initial performances" is a picture of that chain.

The reason Phient reads this as the missing half of the helix is that the helix already encodes the object Eureka evolves. A codon is a channel index. A reward term is a channel of the reward's trace. If a reward were written as a helix, each of its terms would be a codon, its value over checkpoints would be a phasor with a count, and the reflection would be a diff between two helices. Eureka's loop would then be a rule for producing a child helix from a parent helix and its trace. Phient has the encoding and not the rule. Eureka has the rule and not the encoding, since it keeps the reward as source text and the trace as a table.

Phient can say why it does not yet have the rule. There is no outer loop. Nothing in gemphi samples 16 operators, trains each, and keeps the best. The trainer is a single gradient descent over 204,592 examples on Austen and 1,280 rows on the oracle, and the engine at these sizes is allocation and bandwidth bound, with a 1 MB operation taking 2 to 5 ms. An outer loop of 16 by 5 would multiply that by 80. That is affordable in wall clock. It is not built.

The pen spinning curriculum is the part of the paper that speaks most directly to phiano's Trainer. Eureka wrote one reward for reorientation, pre-trained on it, then fine-tuned toward a sequence of target poses with the same reward. Neither the pre-trained policy nor a policy trained from scratch completed a single spin cycle; the fine-tuned policy completed many. That is a claim that the order of training rows matters, and that the same objective reaches a different place when the rows are staged. Phient's trainer has never been run with a staged curriculum. The Austen numbers at 3 epochs, 27.6× chance, and at 8 epochs, 30.7×, are the same rows repeated. Whether a staging would help is not measured.

## 11. What Phient has already measured that bears on it

Every number in this section is from the facts file and was measured for a purpose other than this paper.

The paper's argument for reward reflection is that a single fitness score is a poor signal because it does not say which term of the reward is responsible. Phient has one measurement of the same shape. In the quest oracle, reading the right relation channel gives 0.594 MRR and reading the wrong channel gives 0.015. The whole signal is in knowing which channel to read. A single aggregate over all channels would hide that. Phient's per-channel reading is its version of the per-term trace.

The paper's ablation that iterated search beats one-shot sampling at matched budget has no direct counterpart in Phient, since there is no outer loop. The nearest thing is the epoch curve on Austen: softmax wide-and-deep at 3 epochs reaches 27.6× chance and at 8 epochs reaches 30.7× valid and 30.3× test, hit@10 0.348. That is a single descent, not a selection, and it says only that more passes help.

The paper's finding that its rewards are weakly correlated with the human ones has an analogue in the tie experiments. The sphere tie, with one complex weight per spherical-harmonic degree and 1,192 parameters, reaches 12.8× valid and 13.1× test on Austen hinge at 3 epochs, against the circle tie's 14.7× and 14.6× at 8,360 parameters. Two parameterisations of the operator, one seven times smaller, land within 2× of each other. On the synthetic degree-filter task, 200 items at 40 epochs, the circle reaches 0.662 MRR against identity 0.646 and the sphere reaches 1.000. The two operators do not agree on what they have learned and both score. That is Phient's version of two rewards that are uncorrelated and both beat the baseline.

The paper's human-feedback result, in which a person's paragraph changed the policy, bears on Phient's persona work. Persona attribution over three writers, Chesterton, Austen and Carroll, on held-out four-sentence paragraphs is 71% overall, 74, 65 and 86 by writer, against 33% chance. Dialogue replies were judged as their persona 44% of the time by a sector reader and 72% by a Kneser-Ney panel. Those are measurements of whether a condition is visible in an output. Eureka's 20-user study is a measurement of whether a condition is preferred. Phient has the first and not the second.

The paper's environment-as-context component depends on the state being fully listed. Phient's helix serialises 18,496 words in 1.9 seconds to 75 MB, byte-exact, and is the closest Phient has to a complete listing of its own state that could be put in front of a reader.

Phient has run no reinforcement learning, no policy, no simulator, no language model call, and no reward function of any kind. None of the above is a replication.

## 12. The experiment that would test the paper's idea inside phinet

The paper's idea, reduced to what phinet could test, is that an outer loop of sample, train, score, keep, with the trace fed back, beats a single descent of the same total budget. The test needs an outer loop that does not exist and an object for the loop to vary. The object should be the thing that plays the role of the reward: the training objective of the operator.

The proposal is this. Take the phiecs quest oracle, 256 words, five kinds, 1,280 rows, held-out 20%, chance MRR 0.021. Define a family of training objectives for the operator, each a weighted sum of a small number of named terms: the attraction term, a repulsion term, a rounding penalty, a within-kind consistency term. The facts file records that attraction-only with dithered rounding reaches 0.594 MRR and the shipped trainer reaches 0.082, so at least two points in this family are already measured and differ by 7×. That is the reward space. Each objective is a program with a handful of scalar weights.

The outer loop: at each iteration draw K equal to 8 objectives by perturbing the parent's weights, train the operator under each for a fixed short budget, score each on the held-out set by MRR, keep the best, and record the per-term trace, meaning the value of each objective term at 10 checkpoints. Run N equal to 5 iterations. Total budget 40 short trainings.

The control is named. Run the same 40 trainings as one-shot: draw 40 objectives from the same perturbation distribution around the shipped trainer's weights, train each for the same budget, and take the best. This is the paper's Eureka without evolution ablation at matched samples. A second control, the paper's other ablation, runs the loop but feeds back only the held-out MRR and not the per-term trace. Since phinet does not have a language model to read the trace, the trace would be used by a fixed rule: increase the weight of the term whose value was still moving at the last checkpoint. That is a poor reader, and it is stated as such.

The measurement is held-out MRR after the loop, against the two controls, and against the fixed 0.594 that a person found by hand. If the loop beats one-shot at matched budget, the paper's evolution claim holds on this object. If the loop with trace beats the loop without, the reflection claim holds at least for a fixed reader. If neither, then the two hand-found points, 0.082 and 0.594, were reached by a route the loop cannot find, and that would be the result.

What this test does not do is use a language model. Phinet's reader would be a rule. The experiment tests the loop, not the reader. That is the honest scope. The held-out one-hop result on the oracle, 0.011, is the hard case, and any objective the loop finds should be scored there too.

## 13. Where we disagree or where the analogy breaks

The analogy breaks on F. Eureka has a fitness function. It is the task metric, given by the benchmark, and it is the ground truth the search maximises. The reward is a proxy for F and is judged only by F. Everything in the loop is anchored to it.

Phient has no F for a word. On the quest oracle it has MRR against a synthetic truth, and on Austen it has rank against the next word the author actually wrote. Both are fitness functions in the narrow sense, but neither is a task. A word has no task. There is no state of the world that a word's function is supposed to bring about. When Phient says a word is what it relates to, the relations are the whole of it; there is no further criterion by which one set of relations is correct and another is wrong, except agreement with the corpus. Eureka's rewards are free to be uncorrelated with the human rewards because F decides. Phient's operators are free to differ from one another, sphere and circle, but there is no F above the corpus to say which is right.

The analogy breaks a second time on the reader. Eureka's reflection works because a language model reads it and writes code. The paper's ablation says the reflection is worth 28.6% of normalised score on Isaac, and the paper attributes that to credit assignment across reward terms. But the mechanism by which the model reads the table and decides which term to change is not opened. It is inside gpt-4-0314. Phient does not have that reader and cannot borrow it. Section 12 replaces it with a fixed rule, which is not the same experiment.

The analogy breaks a third time on the closed circle. Every Phical channel is a phasor on 256 quanta with a count. The byte deadband is a consequence: nearest rounding froze every pair 8 quanta short, and dither fixes it at the cost of persona attribution falling from 71% to 65%, so dither ships off by default. That whole story is about what happens when a relation is forced onto a small closed set. Eureka's rewards are unbounded scalars in source code, and nothing in the paper concerns quantisation. The paper's difficulty is different: programs that do not compile, which it handles by sampling 16.

There is one place we disagree, mildly. The paper says its method requires no task-specific prompting. It does require the environment's source, which is task-specific in the strongest sense, and a task description in words. The claim is true in that no template is written per task. It is less true than the sentence suggests, since the source code is the most specific prompt there is.

Finally, the paper is about a simulator. The paper says "simulated Shadow Hand" and does not claim more. Phient's numbers are also from a synthetic oracle and a small corpus, and no Phi-4 run exists. Neither should be read past that.

## 14. The person and the moment

Everything in this section is from the profile.

Anima Anandkumar's first decade was tensor methods, the recognition that the moments of a latent-variable model form a tensor whose decomposition recovers the model, with guarantees. Her second was operators, the Fourier Neural Operator and then FourCastNet, which put an adaptive FNO on the whole planet and produced a week's forecast in under two seconds. Between those two, from 2016 to 2018, she was a principal scientist at Amazon Web Services working on multi-modal algorithms, and from 2018 she has been senior director of machine-learning research at NVIDIA, where she opened the core AI/ML lab in Santa Clara, alongside the Bren Professorship at Caltech.

Eureka is a paper from the NVIDIA side of that life. The author list has nine names, and hers is last, in the senior position. The profile does not describe the paper and makes no claim about her role in it beyond authorship, so nothing more is said here.

What the profile does record is her stated method: "All of the things that work with deep learning, let's take them, but make them a bit more principled." Eureka is an instance of the first half of that sentence. It takes a thing that works, a large language model that writes code, and applies it to a job that was done by hand. The second half, the principle, is thinner in this paper than in the operator papers. There is no theorem. There is a benchmark, two ablations, and a demonstration. That is not a criticism. It is a description of which of her two modes the paper is in.

The profile also records her view that "we have foundation models for language, not for physics," and that transformers cannot be the substrate for physics at scale. Eureka is consistent with that view in an exact way. The language model is kept out of the physics. It never controls the hand. It writes the reward, which is text, and PPO does the control. The paper's own first sentence, that language models excel as high-level planners but that low-level manipulation "remains an open problem," is that view stated as a starting point.

The moment, as the profile gives it, is the year of her UN Scientific Advisory Board appointment on 16 March 2026, the Time100 AI listing in 2026, and the co-founding of Accelerated Understanding in 2026. The paper is from October 2023, published at ICLR 2024, with a second arXiv version in April 2024. It predates all of that. The profile says nothing about her intentions toward Phient and this document claims nothing. No partnership is agreed with anyone.

## 15. What remains

What remains for Phient, from this paper, is an outer loop. Section 12 describes it: sample objectives, train short, score held out, keep the best, feed the trace forward. It does not exist in gemphi. The inner engine is cheap enough that the loop is not ruled out by cost, at 2 to 5 ms per 1 MB operation. It is ruled out only by not having been written.

What remains for the test is a reader. Eureka's loop works because a language model turns a per-term trace into a change to source. Phient's version would use a fixed rule and would therefore test the loop and not the reflection. A fair test of the reflection claim needs a reader that can do credit assignment across terms from a table, and Phient has none.

What remains for the analogy is F. The paper's whole method is anchored to a task metric. Phient's closest anchors are MRR on a synthetic oracle, with chance at 0.021, and rank on Austen, with chance MRR at 0.0063. Neither is a task, and neither says what a word is for. Until Phient can say what it would mean for a word's function to be correct beyond agreement with a corpus, the reward-design framing is a loan and not a fit.

What remains for the curriculum is one run. The paper's pen spinning result is that the same reward, applied in two stages, reaches a policy that neither stage alone reaches. Phient's trainer has never been staged. The Austen epoch numbers, 27.6× at 3 and 30.7× at 8, are repeats of the same rows. A staged run on the oracle, families first and then cross-family rows, would be the smallest test, and its held-out one-hop number, currently 0.011, is where it would show.

What remains for the byte deadband is unchanged by this paper. Eureka does not quantise, and its difficulties are about compilation and credit, not about a nudge falling under half a quantum. The dither cost, 71% to 65% on attribution, is Phient's own problem.

What remains that is not measured: no Phi-4 run exists, sphere at softmax and wide-and-deep is unrun, and no reinforcement learning of any kind has been done in gemphi. This document has proposed one experiment and has run none of it.

What remains of the paper's claim, read plainly, is the part we believe most and can use least: a reward is nothing but what it does to training, and a record of that is enough to improve it. Phical says the same of a word. Neither has yet been shown to hold outside a closed world.

## 16. Sources

Fetched and read for this document.

| what | where |
|---|---|
| arXiv abstract page, title, authors, abstract, versions (v1 19 Oct 2023, v2 30 Apr 2024), comment line "ICLR 2024" | https://arxiv.org/abs/2310.12931 |
| arXiv HTML full text (v2): method components, K=16, N=5, 5 runs, gpt-4-0314, task list with dimensions, normalised score, baselines, ablations, human feedback, pen spinning curriculum | https://arxiv.org/html/2310.12931v2 |
| arXiv HTML full text (latest) | https://arxiv.org/html/2310.12931 |
| Phient measured numbers | PHIENT_FACTS.md, same folder |
| The person | ../17_ANIMA_ANANDKUMAR_PROFILE.md |

The paper's project website and code repository, referenced on the arXiv page, were not fetched for this document and are not cited. No count of negatively correlated tasks and no aggregate Dexterity success rate are given in the paper's text; both are shown in figures we did not read as numbers, and neither is stated above.
