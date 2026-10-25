# The reward is a relation: MineDojo read as a quest with a learned judge

*Phient interpreted papers, 11 of 16 · 2026-06-01 · on: "MineDojo: Building Open-Ended Embodied Agents with Internet-Scale Knowledge" (Fan, Wang, Jiang, Mandlekar, Yang, Zhu, Tang, Huang, Zhu, Anandkumar, 2022, NeurIPS 2022 Datasets and Benchmarks, Outstanding Paper; arXiv:2206.08853)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the eleventh of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. It is the first in the series that is not about operators over functions, tensors over moments, or flight over turbulence. It is about an agent in a game. The paper is MineDojo, a NeurIPS 2022 Datasets and Benchmarks paper that won an Outstanding Paper award. Its senior author is Anandkumar; its first author, Linxi Fan, was at NVIDIA with her. The paper builds three things on top of Minecraft: a simulator with thousands of language-specified tasks, a knowledge base scraped from YouTube, the Minecraft Wiki and Reddit, and a video-language model, MineCLIP, that is used as a reward function for a reinforcement learning agent so that no one has to hand-write a dense reward for each task.

We read it for one reason. Phient's quest is an agent loop. In phiecs a synthetic oracle poses 1,280 rows of relations between 256 words in five kinds, and a trainer moves the phasors of a word toward the phasors of its related word under the kind's channel. The quest has a judge, the oracle. MineDojo's central move is to replace a hand-written judge with a learned one, trained on the relation between what people say and what they show. That is a claim about where a reward comes from, and in Phical terms it is a claim that the reward is a relation between two words under a condition, learned from pairs. This paper takes that claim seriously and asks what it would mean inside phinet.

Sections 2 to 5 restate the paper in its own terms, with the numbers it reports and no others. Sections 6 to 10 place its objects in the Phient family: the language goal as a word, the video snippet as a second word, MineCLIP's score as the phase agreement of two words on a shared channel, the PPO loop as the quest with a soft oracle, and the knowledge base as a corpus the helix could code. Section 11 lists what Phient has measured that bears on any of this, all from the facts file. Section 12 names one experiment, with its control. Section 13 says where the analogy breaks, which it does sharply at the point where the paper's agent has a body and Phient's does not. Section 14 is about the person and the year. Section 15 is what remains.

One limit is stated up front. Phient has no simulator, no video, no pixels and no reinforcement learning. Nothing in MineDojo's method exists in gemphi. Every number about Phient below was measured for another purpose. Where the paper's idea has not been run inside phinet, this page says so.

## 2. What the paper claims, in its own terms

The paper opens with a diagnosis. Agents have done well in specialist domains like Atari and Go, but they "typically learn tabula rasa in isolated environments with limited and manually conceived objectives," and so they fail to generalise. The authors then state a prescription they call a trinity: "1) an environment that supports a multitude of tasks and goals, 2) a large-scale database of multimodal knowledge, and 3) a flexible and scalable agent architecture." The paper is the delivery of the first two and a first attempt at the third.

The environment is Minecraft. The paper says it builds on the open-source MineRL codebase and unifies the observation and action spaces across all tasks, where MineRL had task-specific ones. It unlocks all three Minecraft worlds, the Overworld, the Nether and the End, and gives APIs to set initial conditions and world settings. On this it defines thousands of tasks in two families. Programmatic tasks have a checkable success condition; the paper reports 1,581 of them generated from templates into natural language goals. Creative tasks have no simple success check; the paper reports 1,560, of which 216 were written by hand, 1,042 were mined from YouTube video titles and 302 were produced by GPT-3 after deduplication. A starter set of 64 core tasks, 32 of each kind, is marked for benchmarking.

The knowledge base is three corpora. YouTube: more than 730,000 videos, "33 years of duration and 2.2B words in English transcripts." Wiki: 6,735 pages with 2.2 million bounding boxes of visual elements. Reddit: more than 340,000 posts and 6.6 million comments from r/Minecraft. The paper says it filters low-quality and toxic content.

The agent architecture is the part the paper calls novel. It trains MineCLIP, a video-language model, on pairs of 16-second YouTube clips and their time-aligned transcripts, and then uses it as a reward function. The claim is stated plainly in the abstract: the agent "is able to solve a variety of open-ended tasks specified in free-form language without any manually designed dense shaping reward." Everything, simulator, knowledge bases, algorithm and pretrained models, is open-sourced.

Read as a scientific claim, the paper says two things. First, that an agent's objective can be a learned function of language and observation rather than a hand-coded predicate, and that this learned function is good enough to train against. Second, that the internet already holds enough Minecraft to teach that function. The experiments in section 4 test the first. The second is asserted by construction and left to the community.

## 3. The mathematics, plainly

There is little mathematics in the paper, and what there is belongs to two standard pieces: contrastive learning and policy-gradient reinforcement learning. We state both plainly because the interpretation in later sections depends on their shape.

MineCLIP has two encoders. A text encoder φ_G takes a language goal G and returns a vector. A video encoder φ_V takes a snippet V and returns a vector of the same size. The paper says the text encoder reuses OpenAI CLIP's pretrained text encoder and that the video encoder is "factorized into a frame-wise image encoder φ_I and a temporal aggregator φ_a," with two extra residual adapter layers after the aggregator. The aggregator has two variants. MineCLIP[avg] averages the 16 frame features, which the paper says is fast but blind to temporal order. MineCLIP[attn] runs two transformer layers over them, which is slower and sees order. Only the last two layers of the image encoder are fine-tuned.

Training is the InfoNCE objective on 640,000 pairs of 16-second clips and transcripts. In a batch of N pairs, the score of pair (i, j) is the cosine similarity of φ_G(G_i) and φ_V(V_j), scaled by a temperature. The loss pushes the score of the true pair (i, i) up and the scores of the N−1 mismatched pairs down, in both directions. What the model learns is not a classifier of clips. It learns a geometry in which a sentence and a clip that go together sit close, and ones that do not sit far. That geometry is the whole product.

The reward is that same score. The paper defines the reward function as a map Φ_R : (G, V) → R, where V is the agent's latest 16 egocentric RGB frames at 160 × 256. The text feature is computed once per task and cached. The agent's own visual encoder reuses φ_I's pretrained weights without fine-tuning, and because the video encoder is factorised, features of frames that overlap between consecutive windows are cached. The paper does not print a further transform of the score into a reward in the parts we could read, and we do not state one.

The policy is trained with PPO, which we do not restate. The one addition is self-imitation: trajectories with high MineCLIP reward are stored in a buffer and the training alternates PPO steps with imitation steps on that buffer. Evaluation is over 3 seeds and 200 episodes per task. That is the mathematics. A contrastive geometry, a cosine, and a policy gradient against the cosine.

## 4. Its experiments and numbers as reported

The paper evaluates on 12 tasks in three groups. Animal-Zoo has four programmatic tasks: Milk Cow, Hunt Cow, Shear Sheep, Hunt Sheep. Mob-Combat has four: Combat Spider, Combat Zombie, Combat Pigman, Combat Enderman. Creative has four with no programmatic success check: Find Nether Portal, Find Ocean, Dig Hole, Lay Carpet. Baselines are a hand-engineered dense reward ("Manual Reward"), a sparse binary success reward, and OpenAI's CLIP used unfinetuned as a reward. Numbers are success rates in percent, mean and standard deviation over 3 seeds, 200 episodes each. Table 1 of the paper, as we read it:

| task | Ours (attn) | Ours (avg) | Manual | Sparse | CLIP (OpenAI) |
|---|---|---|---|---|---|
| Milk Cow | 64.5 ± 37.1 | 6.5 ± 3.5 | 62.8 ± 40.1 | 0.0 ± 0.0 | 0.0 ± 0.0 |
| Hunt Cow | 83.5 ± 7.1 | 0.0 ± 0.0 | 48.3 ± 35.9 | 0.3 ± 0.4 | 0.0 ± 0.0 |
| Shear Sheep | 12.1 ± 9.1 | 0.6 ± 0.2 | 52.3 ± 33.2 | 0.0 ± 0.0 | 0.0 ± 0.0 |
| Hunt Sheep | 8.1 ± 4.1 | 0.0 ± 0.0 | 41.9 ± 33.0 | 0.3 ± 0.4 | 0.0 ± 0.0 |
| Combat Spider | 80.5 ± 13.0 | 60.1 ± 42.5 | 87.5 ± 4.6 | 47.8 ± 33.8 | 0.0 ± 0.0 |
| Combat Zombie | 47.3 ± 10.6 | 72.3 ± 6.4 | 49.8 ± 26.9 | 8.8 ± 12.4 | 0.0 ± 0.0 |
| Combat Pigman | 1.6 ± 2.3 | 0.0 ± 0.0 | 13.6 ± 9.8 | 0.0 ± 0.0 | 0.0 ± 0.0 |
| Combat Enderman | 0.0 ± 0.0 | 0.0 ± 0.0 | 0.3 ± 0.2 | 0.0 ± 0.0 | 0.0 ± 0.0 |
| Find Nether Portal | 37.4 ± 40.8 | 89.8 ± 5.7 | N/A | N/A | 26.3 ± 32.6 |
| Find Ocean | 33.4 ± 45.6 | 54.3 ± 40.7 | N/A | N/A | 9.9 ± 14.1 |
| Dig Hole | 91.6 ± 5.9 | 88.1 ± 13.3 | N/A | N/A | 0.0 ± 0.0 |
| Lay Carpet | 97.6 ± 1.9 | 98.8 ± 1.0 | N/A | N/A | 0.0 ± 0.0 |

Three things stand out in the paper's own numbers. First, the learned reward is competitive with the hand-written one on some tasks (Milk Cow, Hunt Cow, Combat Spider, Combat Zombie) and well below it on others (Shear Sheep, Hunt Sheep, Combat Pigman). Second, unfinetuned CLIP is near zero almost everywhere, so the Minecraft fine-tuning is what makes the reward usable. Third, the variances are large. Milk Cow at 64.5 ± 37.1 means the seeds disagree a great deal. The paper does not hide this.

A second experiment tests MineCLIP as a judge rather than a reward. On 100 successful and 100 failed trajectories per creative task, the paper reports F1 agreement with human labels: Find Nether Portal 98.7 (attn) and 100.0 (avg), Find Ocean 100.0 and 100.0, Dig Hole 99.4 and 100.0, Lay Carpet 97.4 and 98.4, against OpenAI CLIP at 48.7, 98.4, 80.6 and 54.1. The paper also reports that a single agent trained on all 12 tasks shows negative transfer between tasks, and that zero-shot transfer to an unseen task (Hunt Pig) is 1.3 ± 0.6 before fine-tuning. It states that MineCLIP has 150 million parameters and is far more expensive to query than a hand-coded reward, and that the Wiki and Reddit corpora are not used in the experiments.

## 5. The one idea, restated

Strip the paper to one sentence and it is this: the judge of an action can be learned from the relation between what people say and what they show, and once learned it can stand in for a hand-written rule. Everything else, the simulator, the 730,000 videos, the 3,141 tasks, the PPO details, is the apparatus that makes that sentence testable.

The sentence has two halves and they carry different weight. The first half is that a reward is a relation. In the paper a reward is not a property of a state. It is a score between a goal G and a window V, and it exists only for the pair. The same 16 frames score high under "milk a cow" and low under "dig a hole." The state has no value on its own. This is the same move Phical makes about words: a word has no point, only relations under conditions. The paper makes it about rewards.

The second half is that the relation can be learned from aligned pairs without labels. The 640,000 clip-transcript pairs were never labelled as good or bad. The only supervision is co-occurrence: this sentence was said while this clip was shown. InfoNCE turns co-occurrence into a geometry. That is exactly what a language model does with adjacent words, and what Phient's operator does with word pairs in Austen. The paper extends it across a modality boundary, from text to pixels, and then uses the result to steer.

There is a third thing the paper does that is easy to miss. It uses the same learned relation twice, once as a reward during training and once as a judge during evaluation of creative tasks. The F1 numbers in Table 2 are the second use. This matters because it says the relation is good enough to trust as a measure, not only as a gradient. In Phient's vocabulary the oracle and the trainer become one object. The quest currently has a synthetic oracle that knows the answer. MineDojo's oracle does not know the answer; it knows what an answer looks like.

We restate the idea in Phical form, as a hypothesis and not a result. A goal is a word. A window of experience is a word. The reward is the agreement of the two under the condition "this goal was being pursued." Learn the agreement from pairs. Then move the agent toward agreement. The sections that follow ask whether each clause of that hypothesis has a home in phinet.

## 6. The goal as a word, the window as a word

Phical's first claim is that a word is a function: 32–64 relation channels, each a phasor on a closed circle with a count. Nothing in that definition says the token must be a string of letters. A word is anything that can be placed on the circle by its relations. MineDojo's language goal "milk a cow" is a string, and it is already a word in the ordinary sense, or a short phrase that phiano's ingest would treat as a few words. The video window is the harder case, and it is the case that decides whether this paper has anything to say to phinet.

MineCLIP's video encoder maps 16 frames to one vector. The paper is explicit that the encoder is factorised: an image encoder per frame and an aggregator over the 16. The output is a single point in a shared space with the text encoder. Phical does not store points. So the first question is whether the window's vector can be read as relations instead of a point. The answer is that the contrastive objective already makes it so. A CLIP-style vector has no meaning except through its cosine with other vectors. Its norm is discarded. What remains is direction, and direction on a sphere is the closest thing to a phase that a real vector space has. When the paper writes that a window "matches" a goal, it means the two directions agree.

In Phient's family the sphere has a concrete home. phinet::Tie::Sphere puts one complex weight per spherical-harmonic degree and reads the 64 channels as Y_l^m with l < 8. That tie was built for word functions, not pixels. But the structure is the right one for the paper's object: a direction on a sphere, compared to another direction. MineCLIP's shared embedding is a sphere of a few hundred dimensions with a learned metric. Phical's is a fixed set of 64 harmonics with a byte per channel. The two are not the same size and we do not pretend they are.

The condition c is where the two words meet. In MineDojo the condition is implicit: the pair (G, V) is scored as if G was being attempted during V. In Phical it would be a named channel. The quest already has five kinds, is-a, part-of, near, used-for and unlike, and each is a channel offset. A sixth kind, call it "shows," would be the channel on which a goal-word and a window-word are read against each other. Whether such a channel can be learned from aligned pairs the way InfoNCE learns its metric is the experiment in section 12. Here we only note that the paper's two encoders are two words and its cosine is a reading on one channel.

## 7. The relation channel as a coordinate

Every Phient paper in this series comes to the same point: her group's objects have coordinates, and Phical's coordinates are channels. In the FNO papers the coordinates were Fourier modes. In the tensor papers they were eigenvectors. In MineDojo the coordinate is the axis of the contrastive embedding, and the paper is unusual because it says almost nothing about that axis. It inherits it from CLIP and fine-tunes only the last two layers. The coordinate is a given.

That silence is the interesting part. The paper's method works because CLIP's space already separated "cow" from "hole" from "portal" before any Minecraft video was seen. The fine-tuning taught the space what a Minecraft cow looks like at 160 × 256, not what a cow is. The Table 1 column for unfinetuned CLIP, near zero on all but the two "Find" tasks, says the visual side needed the domain but the language side came free.

Phical does not inherit its coordinates. It fixes them: 64 channels, one byte each, 256 quanta of 2π. The paper's success suggests a question that Phient has not answered. Which of the 64 channels would the "shows" relation live on, and would it be learned or assigned? In the quest the kinds are assigned: is-a is a channel offset chosen by the oracle's construction, and the trainer learns the phases but not the offset. The measured result that the wrong channel reads at 0.015 MRR against 0.594 on the right one says the assignment is doing real work. MineDojo's coordinate is not assigned. It is whatever direction InfoNCE found.

There is a middle position. The paper's factorised encoder computes a per-frame feature and then aggregates. The avg variant is a plain mean; the attn variant is two transformer layers. On Find Nether Portal the avg variant reaches 89.8 ± 5.7 and the attn variant 37.4 ± 40.8. On Hunt Cow it is the other way, 0.0 against 83.5 ± 7.1. So the choice of how frames are combined along time is a choice of coordinate, and the paper reports that no one choice wins. In Phical the closest object is the count on each channel. A phasor with a count is a mean over the pairs that placed it there, and it is order-blind, which is the avg variant's property. An order-aware aggregate has no home in the present Phical channel. That is a limit of ours, stated plainly. The circle remembers how many and where, not when.

## 8. The operator as X ⇒ Y under c

Phical's paradigm is X ⇒ Y under c: an operator moves one word toward another under a condition. In phinet that operator is trained on Austen to move a word's function toward the next word's function, and it is measured by how well the moved function ranks the true next word: 30.7× chance valid at 8 epochs for the wide and deep softmax model, hit@10 0.348. The operator is the whole of what phinet does.

MineDojo has two operators and it helps to name both. The first is MineCLIP itself, which is not a mover but a judge. It does not turn G into V. It says how far apart they are. In Phical terms this is not X ⇒ Y. It is the reading of two words on one channel, which is what the quest's oracle does when it scores a pair as is-a or not. The second operator is the policy. The policy takes a window and a goal and returns an action, and the action changes the world so that the next window sits closer to the goal. That is X ⇒ Y under c exactly: move the current window-word toward the goal-word under the condition of the task.

The difference between the two is the difference between a metric and a dynamic. Phinet's operator is a dynamic on the circle. It moves phases. MineCLIP is a metric on the sphere. It measures. The paper's contribution is to make the metric learned and then to run a dynamic, PPO, against it. The metric does not move anything. It only says whether the move was good.

Here the analogy with the quest becomes exact enough to be useful. The quest trainer is a dynamic: attraction on a channel, moving the phasor of word A toward the phasor of word B, with dithered rounding to get past the byte deadband. The oracle is a metric: the 1,280 rows say which pairs should attract. What MineDojo suggests is that the oracle could itself be a phasor reading rather than a table. If "shows" were a channel and the goal-word and window-word had phasors on it, the agreement of those phasors would be the reward, and the trainer would move the window-word toward the goal-word on that channel. The paper's PPO loop would collapse to the quest's attraction step.

We are careful about what is and is not claimed. The collapse is a design. Nothing has been built. The quest's attraction step reaches 0.594 MRR on the synthetic oracle with a table of answers. Whether it would reach anything with a learned judge is unmeasured. The point of this section is only that the paper's two operators both have names in Phical and that the names are different: one is a read, one is a move.

## 9. Where the agent loop lands: the quest with a soft oracle

phiecs is an entity-component world with a quest. The quest is the closest thing Phient has to MineDojo's agent loop, so it is worth being precise about how far the match goes. The quest has 256 words in families of 8, five kinds of relation, 1,280 rows, 293 target words, and chance MRR of 0.021. An episode is a row: given word A and a kind, find word B. The trainer moves phasors; the oracle checks. The shipped trainer reaches 0.082 MRR; attraction-only with dithered rounding reaches 0.594, hit@1 34.8%, hit@5 97.4%.

Map MineDojo onto that. A task is a row. The goal G is the kind together with word A. Success is finding B. In MineDojo, B is not a word but a condition on the world, and the programmatic tasks check it directly, which is what the quest's oracle does. The creative tasks have no B to check, and that is where MineCLIP comes in as a judge. The quest has no creative tasks. Every row has an answer.

The paper's programmatic and creative split is the sharpest lesson for the quest. The quest is entirely programmatic. Its held-out numbers say that within a kind the trainer generalises to two hops on is-a (0.095), used-for (0.093) and only to chance on part-of, near and unlike, and that one hop held out reads 0.011, near chance. The oracle is a hard judge with a fixed table, and the trainer has learned the table more than the relation. MineDojo's soft judge, trained on 640,000 pairs it never saw again, was able to score 200 unseen trajectories per task with F1 above 97 on all four creative tasks. That is what a learned relation looks like when it generalises, and the quest does not have it.

The loop structure also differs in tempo. The quest is offline: all rows are known, epochs pass over them. MineDojo is online: the policy acts, a new window appears, the judge scores it, and the gradient follows. Phient has no online loop. phiano's generation is the nearest thing, a sampler that walks the operator forward token by token, and the decoding measurement (operator nearest-k perplexity 66.9 against the trigram sampler's 13.3) says that walk is still far from good. An online quest, where the trainer acts and a judge scores the result without a table, would be a new component of phiecs. It is not planned and nothing in the facts file describes it.

What the quest has that MineDojo does not is the byte. Each channel is one byte, and the measured deadband says the nudge 0.06·sin d falls under half a quantum once sin d < 0.2, so nearest rounding froze every pair 8 quanta short. MineDojo's rewards are floats and its policy is a float network. The quest's problem of getting a discrete phasor to move under a small reward is one the paper does not have and does not solve.

## 10. The knowledge base and the helix

MineDojo's second pillar is the knowledge base: 730,000 videos, 6,735 wiki pages, 340,000 Reddit posts. The paper is honest that only the videos are used in its experiments. The wiki and Reddit corpora are shipped for the community. This section asks what Phient would do with them, since Phient has only text and no video path at all.

The wiki is the natural fit. Phiano's quest ingest reads text into relations, and the facts file records that the helix codes a lexicon of 18,496 words as a DNA-coded tree in 1.9 s and 75 MB with a byte-exact round trip. The Minecraft Wiki's 6,735 pages are a closed vocabulary of items, mobs, blocks and recipes with explicit relations: crafting is part-of and used-for, biomes are near, mob classes are is-a. It is a real-world version of the quest's synthetic oracle, larger and noisier but with the same five kinds visible in its structure. The 2.2 million bounding boxes the paper reports are the wiki's images labelled with the words they show, which is the same "shows" relation MineCLIP learns from video, only static and already aligned.

The helix would code that lexicon the way it codes any other: 64 codons as base-4 ACGT of the channel, the hexagram, the sphere degree and order, the B-DNA geometry at 10.5 codons per turn and 0.34 nm rise, a PhiTon wavelength, band and hex per codon. None of that requires video. It requires words and relations, and the wiki has both. Whether a wiki-derived quest would train better than the synthetic one is unmeasured. The paper's own observation that its agents show negative transfer when trained across all 12 tasks is a warning: a larger vocabulary of tasks does not automatically make a better learner.

The Reddit corpus is different. The paper says it is unstructured and not used. For Phient it would be a corpus for persona work, the same way Chesterton, Austen and Carroll are. The measured attribution number, 71% over three writers on held-out four-sentence paragraphs against 33% chance, is a text-only result and says nothing about a forum. We mention it only to place the corpus, not to claim a result.

The video corpus has no home in Phient. There is no image encoder, no frame, no pixel. The paper's central data, the 640,000 clip-transcript pairs, cannot be ingested. This is the largest gap between the paper and the project and section 13 returns to it. The one thing we can say is that the transcripts alone, 2.2 billion words the paper reports, are text, and text is what phinet trains on. A transcript-only MineDojo would be a language model of people describing Minecraft, and it would have the goal-words but not the window-words. Half of the paper's relation, and the half Phient already knows how to handle.

## 11. What Phient has already measured that bears on it

Nothing in gemphi is a reinforcement learning result, a video result, or a Minecraft result. The measurements below are the ones that touch the paper's idea, each from the facts file and each made for another purpose.

| what | number | bears on |
|---|---|---|
| phinet vs PyTorch reference | 62/62 parity tests | the engine is a correct base for any new operator |
| Austen operator, softmax wide+deep, 8 epochs | 30.7× valid / 30.3× test, hit@10 0.348 | a learned X ⇒ Y from pairs, the same supervision as InfoNCE |
| Quest, attraction-only + dithered rounding | 0.594 MRR, hit@1 34.8%, hit@5 97.4% | the agent loop with a hard oracle |
| Quest, wrong channel / offset reading | 0.015 / 0.005 | the condition is doing the work |
| Quest held-out 20%, one hop | 0.011 | the hard oracle does not generalise |
| Quest held-out, two hops is-a / used-for | 0.095 / 0.093 | some kinds carry a relation, some carry a table |
| Sphere tie, Austen hinge 3 epochs | 12.8× valid / 13.1× test, 1,192 params | a direction-on-a-sphere reading exists in phinet |
| Synthetic degree-filter | circle 0.662, sphere 1.000 | the sphere separates degrees the circle cannot |
| Byte deadband | nudge under half a quantum once sin d < 0.2 | small rewards do not move a byte |
| Dither cost | attribution 71% → 65% | the fix for the deadband has a price |
| Persona attribution, three writers | 71% (74/65/86), chance 33% | a corpus can be read as a judge of style |
| Dialogue judged as persona | 44% sector / 72% KN panel | a learned judge and a rule judge disagree |
| Helix | 18,496 words, 1.9 s, 75 MB, byte-exact | a lexicon codes at wiki scale |
| Engine | allocation/bandwidth-bound, 1 MB op 2–5 ms | the cost is memory, as the paper says of MineCLIP's 150M params |

Three of these are worth a sentence. The quest's held-out numbers are the closest Phient has to the paper's Table 2. Where MineCLIP judges unseen trajectories at F1 above 97, the quest's trainer on unseen one-hop rows reads at 0.011, half of chance. The hard oracle has taught a table. That is the measured fact that makes the paper's soft judge interesting to us.

The dialogue number is the second. Eighteen replies were judged as their persona by two judges, a sector reader at 44% and a Kneser-Ney panel at 72%, against 33% chance. Two judges of the same output disagree by 28 points. MineDojo reports that its two judge variants, avg and attn, agree closely with humans on creative tasks but disagree strongly with each other as rewards on some programmatic tasks. The lesson is the same: the judge is a model, and the choice of judge is a result.

The third is the sphere. Phinet has a sphere tie and it reaches 1.000 on the synthetic degree filter where the circle reaches 0.662. That is the only Phient object with the geometry of the paper's embedding, and it has been run only at hinge, only on Austen and the synthetic filter. Softmax and wide+deep on the sphere are unrun.

## 12. The experiment that would test the paper's idea inside phinet

The paper's idea, reduced to what phinet can hold, is that a judge can be learned from aligned pairs and then used as the oracle of a quest, and that such a judge generalises where a table does not. The experiment tests exactly that on the synthetic oracle, with no video and no pixels.

Setup. Take the phiecs oracle as it is: 256 words, five kinds, 1,280 rows. Hold out 20% of rows, the same split already measured. Train a judge, not a trainer. The judge is a phinet operator that takes a pair of word functions and a kind and returns a score, trained contrastively: for each true row (A, kind, B) in the training 80%, the score of (A, B) on the kind's channel is pushed up and the scores of (A, B') for the other seven words in B's family are pushed down. This is InfoNCE with a family as the batch. The judge sees no test rows.

Then run the quest's attraction-only trainer with dithered rounding exactly as shipped, on the training rows, but replace the oracle's table with the judge's score for the attraction target. The trainer moves A toward whichever B the judge scores highest on the kind's channel.

Control. The shipped configuration: the same trainer with the table oracle, which reaches 0.594 MRR on all rows and 0.011 on held-out one-hop rows. A second control is the judge alone with no trainer, ranking B for each held-out A by score, which measures whether the judge generalises before anything is moved.

Measurement. MRR, hit@1 and hit@5 on the held-out 20%, reported per kind, against chance 0.021. The paper's claim survives in phinet if the judge-driven trainer reads above 0.011 on held-out one-hop rows and the judge-alone control reads above chance on the same. The claim fails if the judge learns the table too, which would show as judge-alone at chance on held-out rows. The paper's second finding, that the choice of aggregator changes the result, has a Phical analogue: run the judge once on the circle tie and once on the sphere tie, and report both.

What this does not test. It does not test the cross-modal half of the paper, since both words are lexical. It does not test online learning, since the rows are fixed. It does not test the 150M-parameter cost, since the judge would be a phinet operator of the sizes already measured (8,360 to 50,016 parameters). It tests one thing: whether a learned relation makes a better oracle than a table. That is the thing the quest's held-out numbers say Phient needs.

The byte deadband is a known hazard. If the judge's score differences are small, the nudge will fall under half a quantum and nothing will move. Dither is already in the trainer, at its measured cost to attribution (71% to 65%). The experiment inherits that trade and should report the frozen fraction.

## 13. Where we disagree or where the analogy breaks

The analogy breaks in four places, and the first is total. MineDojo's agent has a body. It sees 160 × 256 pixels, it has an inventory of 36 slots, a 3 × 3 × 3 voxel neighbourhood, a compass and GPS, and it acts with forward, backward, camera, attack, craft, equip, place and destroy. Phient has none of this. The quest's agent is a phasor being nudged on a circle. There is no world it changes, no window it sees after acting. The paper's idea that a window of experience is a word is a hypothesis Phient cannot test without a world, and phiecs, an entity-component system, is a world in name only until something acts in it. We do not claim it is more.

The second break is the modality. MineCLIP's value is that it crosses from text to pixels. Phical crosses from word to word. Everything in section 6 about reading a video window as a word rests on the observation that a contrastive embedding is a direction, and directions are phase-like. That is true and it is also thin. A direction in a few hundred real dimensions is not a phasor on 64 byte-quantised circles, and the sphere tie with 1,192 parameters is not a fine-tuned CLIP with 150 million. The geometries rhyme. They are not the same object.

The third is where we disagree with the paper, mildly. The paper's abstract says the agent solves tasks "without any manually designed dense shaping reward." Table 1 says the learned reward beats the manual one on four of eight programmatic tasks and loses on the other four, sometimes by a lot (Shear Sheep 12.1 against 52.3). The variances are wide enough that some of those comparisons are not settled at 3 seeds. The claim in the abstract is true as stated and the numbers say the learned reward is a substitute of uneven quality, not a replacement. The paper's own tables are more careful than its abstract, and we side with the tables.

The fourth is about the knowledge base. The paper's second pillar, internet-scale knowledge, is presented as one of three ingredients of a generalist agent, and the experiments use one of the three corpora. The wiki and Reddit data are released and not tested. So the paper's evidence that internet-scale knowledge helps is the evidence that 640,000 YouTube pairs make a usable reward. That is real evidence. It is narrower than the trinity framing, and the paper's later sentence that the agent trained on all 12 tasks shows negative transfer is a reminder that scale in tasks is not the same as scale in capability.

One place where we do not disagree at all: the paper's finding that the unfinetuned CLIP is near zero as a reward. The condition c had to be learned in the domain. The quest's wrong-channel reading of 0.015 says the same thing in Phical's terms. A relation off its channel is noise.

## 14. The person and the moment

By 2022 Anandkumar had been senior director of machine-learning research at NVIDIA for four years and Bren Professor at Caltech for the same. The profile records the career in four moves: tensor methods at UC Irvine from 2010, AWS from 2016, then NVIDIA and Caltech together from 2018. MineDojo belongs to the NVIDIA years and to a group of collaborators, Linxi Fan, Yuke Zhu and others, who were building agents rather than operators. Its first author later led the work on Voyager, but that is outside this paper and we do not describe it.

The profile places 2022 as a dense year for her. FourCastNet, the global weather model, was posted in February 2022. The ACM Gordon Bell Special Prize for HPC-based COVID-19 research came the same year, as did her election as an ACM Fellow. And the profile records "a NeurIPS Outstanding Paper (2022)." That is this paper. In the same twelve months her name sits on a paper that forecasts the whole atmosphere in under two seconds and on one that teaches an agent to milk a cow in Minecraft from YouTube. The distance between the two is a measure of how wide the group's interests ran, and of the method the profile quotes her stating: "All of the things that work with deep learning, let's take them, but make them a bit more principled."

MineDojo is the less principled of the two, on its own terms. There is no theorem, no operator, no resolution invariance. There is a large dataset, a fine-tuned CLIP and a PPO loop. What it shares with the operator papers is the bet that the right substrate matters more than the right architecture: for weather, function space instead of grid; for agents, an open world with internet-scale supervision instead of a closed one with a hand-written reward. The profile's line that "we have foundation models for language, not for physics" was said about the other bet. MineDojo is her group's contribution to the language-and-vision one, made at the moment when large pretrained models were becoming the default tool.

We note, as the profile does, that nothing here is her statement about Phient. The paper's authors have not seen phinet and the reading in this document is ours. What we take from her in this paper is not a method to copy but a discipline: report the variance, print the baseline that fails, say which corpora went unused, and let the abstract claim less than the tables show. The tables in section 4 are the paper's own and they are honest to a fault.

## 15. What remains

Three things remain, in order of size.

The first is the experiment in section 12. It is the only piece of MineDojo that phinet could run this year, and it is small: a contrastive judge on the synthetic oracle, a trainer driven by the judge, and the held-out 20% as the measure. The quest's measured held-out numbers, one hop at 0.011, are the reason to run it. Until it is run, the claim that a learned judge would generalise where the table does not is a hypothesis with the paper's Table 2 as its only evidence, and Table 2 is about Minecraft.

The second is the sphere. The paper's embedding is a direction on a sphere and phinet's sphere tie exists, but only at hinge and only on Austen and the synthetic filter. Softmax and wide+deep on the sphere are unrun. If a judge is to be built as a reading of two words on a sphere, the sphere needs the same measurements the circle already has. That is a listed gap in the facts file and this paper adds a reason to close it.

The third is the one that will not be closed soon. Phient has no world and no body. phiecs is an entity-component system without an actor, the quest is offline, and there is no window of experience for a judge to score after an action. Everything in sections 6 to 9 that treats a video window as a word is a reading, not a build. A Phient agent that acts, sees, and is judged would be a new thing, and nothing in the facts file or the plans describes it. We say this so that no reader takes the interpretation for a roadmap.

Two smaller items. The Minecraft Wiki, 6,735 pages with the five quest kinds visible in its structure, is text and could be ingested by phiano and coded by the helix. Whether it makes a better quest than the synthetic oracle is unmeasured and a wiki-derived oracle would need the same held-out discipline. And the paper's observation that two aggregators, avg and attn, make different rewards has a cheap Phical echo: the count on a channel is order-blind, and if order matters a new reading is needed. That is a design note, not a result.

Nothing in this document is a measurement of MineDojo's method inside phinet. Every Phient number is from the facts file and was measured for another purpose. The paper's numbers are its own, from its Tables 1 and 2, and we have not rerun any of them.

## 16. Sources

The paper, abstract page, fetched 2026-06-01: Fan, Wang, Jiang, Mandlekar, Yang, Zhu, Tang, Huang, Zhu, Anandkumar, "MineDojo: Building Open-Ended Embodied Agents with Internet-Scale Knowledge," arXiv:2206.08853, v1 17 June 2022, v2 22 November 2022, comments field "Outstanding Paper Award at NeurIPS 2022." https://arxiv.org/abs/2206.08853

The paper, full text, fetched 2026-06-01, from which the task counts, corpus sizes, MineCLIP architecture and training data, the reward definition, Table 1 success rates, Table 2 F1 agreement, the 3 seeds and 200 episodes evaluation protocol, the 150M parameter count, the negative-transfer and zero-shot (Hunt Pig 1.3 ± 0.6) observations, and the statement that Wiki and Reddit data are unused were taken. https://arxiv.org/pdf/2206.08853

The profile of Anima Anandkumar used for section 14, in this repository: `docs/papers/17_ANIMA_ANANDKUMAR_PROFILE.md`, and the sources it lists, in particular the Caltech News item of 16 March 2026 (https://www.caltech.edu/about/news/anima-anandkumar-appointed-to-un-scientific-advisory-board) and the Latent Space interview (https://www.latent.space/p/anima). We did not fetch those two pages for this paper; the quotations attributed to her here are as the profile records them.

Phient facts used for every Phient number: `docs/papers/anandkumar/PHIENT_FACTS.md`. No number about Phient in this document comes from any other place.

Not fetched. The MineDojo project site (minedojo.org) and the GitHub repository (github.com/MineDojo/MineDojo) were not reachable from the environment in which this was written, so no claim here rests on them. The paper's exact PPO training budget, episode lengths, and any post-processing of the MineCLIP score into a scalar reward beyond the map Φ_R : (G, V) → R were not found in the text we could read and are not stated above.
