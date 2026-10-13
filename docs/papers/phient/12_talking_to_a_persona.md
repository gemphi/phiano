# Talking to a Persona
*Phient papers, 12 of 16 · 2026-06-15 · Talking to a persona: the condition, what it calls for, nine candidates, and two judges that disagree*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

A persona in Phiano is the set of conditions one writer puts words under, gathered from that writer's text. This paper describes what happens when a user talks to one. `persona::Dialogue`, in `src/persona/dialog.rs`, answers a turn in four steps. It picks the condition the question names: the rarest content word of the question that the lexicon knows. It asks the persona's manifold what that condition calls for: over the persona's 600 most-used content words, the one with the strongest directed reading from the condition, X ⇒ Y. It draws nine candidate replies from the generator, conditioned on the persona and pulled toward that word, seeded from three places in the question. It chooses one by the persona's own likelihood and the evaluator's quality, less what the dialogue has already said.

The personas are three writers: G. K. Chesterton, Jane Austen and Lewis Carroll. `bin/persona_ingest` builds them from whole training splits, over a shared lexicon of 18,496 words trained on the three training splits only. Held-out attribution on four-sentence paragraphs, 150 per author, rose from 62% to 71% against a chance of 33% when the personas were ingested from every training sentence instead of 600 sampled ones.

`bin/persona_chat` put the same six questions to all three personas in 17 s. Two judges read each of the 18 replies alone. The engine's own sector attribution read 8 of them as the persona who spoke, 44%. A Kneser-Ney trigram panel, one model per writer, read 13, 72%. Chance is 33%. Both are above chance. They disagree, and the shape of the disagreement is the main result. The sector reader called 15 of the 18 replies Austen. The panel was right on all six Austen replies, five of six Chesterton replies, and two of six Carroll replies.

The paper also reports what the transcript shows when it is read closely. The called-for word appears in none of the 18 replies. Two Austen replies were spoken over the repeat limit, because no candidate that turn was fresh enough. Trigram idiom such as "to be sure" crosses from one persona to another. Doc 81 names the engine's own attribution as the weakest link in the persona chain. A discriminative attribution reader is the next thing. It is not built. The Kneser-Ney panel is the yardstick it has to beat.

## 2. The claim

The claim of docs/81 is narrow. A dialogue with a persona is not retrieval. The question names a condition. The persona's manifold says what that condition calls for. The answer is generated toward it. Nothing is templated. The lexicon is never written during a dialogue, so the persona does not train on its own words.

This follows from the paradigm. If a word is nothing but what it relates to under conditions, then a persona is which conditions a writer puts words under, aggregated. Asking a persona about love is asking what love calls for in that writer's aggregated relations. The sources give three answers from one run: love ⇒ regard for Austen, love ⇒ smiling for Chesterton, love ⇒ notice for Carroll. The three words differ because each is chosen from a different pool of 600 words, read through the same shared Facet.

The claim has three parts that can be checked separately. First, the condition and its called-for word are chosen by rule from the lexicon, with no hand list. The rule is in `src/persona/dialog/reading.rs` and is short enough to read in full. Second, the reply is conditioned on the persona. The generator carries the persona's sector histogram and its own pair evidence (docs/79 §3) through a `PersonaPrior`. Third, the reply can be read back as the persona who spoke. This third part is the one that was measured. It was measured twice, by two judges.

The claim does not include fluency. The replies are 28-token walks through trigram tables, and they read as such. It does not include that the reply is about the called-for word. The word is a goal with a pull of 1.5 (`DIALOG_GOAL_PULL`), and the transcript shows that the pull alone does not place it in the text. It does not include memory of the user beyond the context wave. The wave carries across turns, so the sixth answer is spoken after the first five. The set of spoken 4-grams is kept, to penalise repeats. Nothing else about the conversation is kept.

What is measured is this. Over 18 replies, the Kneser-Ney panel read 72% as the persona who spoke and the sector reader read 44%. Chance is 33%. Doc 81 draws two readings from it. The first is that the persona conditioning is choosing the company each writer keeps: Father Brown and Syme turn up for Chesterton, Alice and the King for Carroll, Emma and Jane for Austen. The second is that the engine's own attribution is now the weakest link in the persona chain. The generation separates the writers well enough for an outside model to tell them apart at 72%. The engine's reader cannot.

## 3. The mechanism

`Dialogue::open(facet, persona)` builds the state for one persona. It makes a `PersonaPrior` with `PersonaPrior::new(facet, &persona.fingerprint, 0.0).with_examples(&persona.examples, 1.0)`. It extracts a `Fingerprint` from the persona's examples. It builds the answer pool with `answer_pool`. It opens an empty `ContextWaveBuffer` of `DEFAULT_CONTEXT_WINDOW`. It starts an empty set of spoken 4-grams and a turn counter at zero.

The constants sit at the top of `dialog.rs`:

```rust
pub const DIALOG_CANDIDATES: usize = 9;
pub const DIALOG_REPEAT_MAX: f64 = 0.25;
pub const DIALOG_TOKENS: usize = 28;
pub const DIALOG_SEED: usize = 6;
pub const DIALOG_POOL: usize = 600;
pub const DIALOG_GOAL_PULL: f64 = 1.5;
pub const DIALOG_REPEAT_COST: f64 = 0.8;
```

`Dialogue::reply(facet, user)` does the turn. It tokenizes the user's text. It calls `topic`, which walks the tokens, skips function words and words of two letters or fewer, looks each remaining word up in `facet.lexicon`, and keeps the one with the smallest `evidence_total()`. It calls `calls_for`, which walks the pool, skips the topic and every word the user said, scores each word by `facet.calls_for(w, topic)`, and keeps the highest. A best score of zero or less returns no goal.

It then builds nine seeds. Index `i % 3 == 1` seeds with the topic alone. Index `i % 3 == 2` seeds with the topic and the goal, or the topic and the last two words of the question when there is no goal. Index `i % 3 == 0` seeds with the tail of the question, `DIALOG_SEED - i / 3` tokens long. Each candidate gets its own `Generator::deterministic(self.tokens, temperature, phinet::mix64(self.turn * 7919 + i))`, with `Decode::Trigram`, the persona prior, and `with_goal(w, DIALOG_GOAL_PULL)` when a goal exists. The temperature is `0.3 + 0.15 * ((i / 3) % 3)`. Each candidate generates against a swapped-in copy of the dialogue's context wave, so no candidate sees another's text.

A candidate shorter than three tokens is dropped. The rest are scored:

```rust
let fit = self.fingerprint.likelihood(&Fingerprint::extract(facet, &[text.clone()]));
let quality = eval.eval(facet, &text).overall;
let repeat = self.repeat(&text);
let combined = (fit * IMPERSONATE_FIT_WEIGHT + quality * IMPERSONATE_QUALITY_WEIGHT)
    * (1.0 - DIALOG_REPEAT_COST * repeat);
```

The two weights live in `crate::config`. Their values are not in the sources read for this paper. The best candidate with `repeat <= DIALOG_REPEAT_MAX` wins. If none qualifies, the candidate with the lowest repeat speaks. Only the spoken turn advances the context: the user's text and the reply are pushed into the buffer with `push_turn`, and the reply's 4-grams join the set of what was said. The result is a `Reply` with `text`, `repeat`, `topic`, `calls_for`, `fit` and `quality`, so every transcript line carries its own confidence.

## 4. What was measured

Ingestion was measured first. `bin/persona_ingest` splits each author 80/10/10 by sentence hash, seed 7. The shared lexicon is trained on the three training splits only, and each persona is ingested from its whole training split.

| author | books | sentences | ingested from | held out | words |
|---|---|---|---|---|---|
| G. K. Chesterton | 3 | 11,219 | 8,975 | 1,122 | 162,752 |
| Jane Austen | 3 | 17,114 | 13,691 | 1,712 | 247,252 |
| Lewis Carroll | 1 | 1,378 | 1,102 | 138 | 18,305 |

The shared lexicon has 18,496 words, all with relation evidence, 24 marker channels, three passes, 12 s. Persona ingestion took 8 s for all three. The total was 53 s, written to `data/personas.chroma` and `data/personas.json` through `PersonaStore`. `bin/persona_chat` opens the store read-only in about a second.

Attribution on held-out paragraphs of four sentences, 150 per author, the persona with the highest likelihood winning, measured the same way before and after:

| persona | sampled 600 | all sentences |
|---|---|---|
| G. K. Chesterton | 73% | 74% |
| Jane Austen | 52% | 65% |
| Lewis Carroll | 57% | 86% |
| all | 62% | 71% |

Chance is 33%. Carroll gains most. His 1,102 training sentences are all he has, and sampling 600 threw away about half his evidence.

The dialogue was measured next. `cargo run --release --bin persona_chat` ran all three personas on the same six questions in 17 s. Each reply was read alone by two judges.

| over 18 replies | read back as the persona who spoke |
|---|---|
| the engine's sector attribution | 8, 44% |
| Kneser-Ney panel, one model per writer | 13, 72% |
| chance | 33% |

Counted from the transcript, per persona:

| persona | sector reader | KN panel |
|---|---|---|
| G. K. Chesterton | 2 of 6 | 5 of 6 |
| Jane Austen | 6 of 6 | 6 of 6 |
| Lewis Carroll | 0 of 6 | 2 of 6 |

The sector reader named Austen for 15 replies, Chesterton for 2 and Carroll for 1. Its margin, printed in the transcript, was 0.00 or 0.01 every time. The panel named Austen for 11, Chesterton for 5 and Carroll for 2. On 8 replies both judges were right. On 5 only the panel was right. On 5 both were wrong. There is no reply that only the sector reader got right.

The transcript also prints per-reply numbers. Fit, the speaking persona's likelihood of the reply's fingerprint, ran from 0.754 to 0.806. Quality, the evaluator's overall score, ran from 0.641 to 0.731. Panel perplexity for the winning writer ran from 12 to 48. Repeat ran from 0% to 40%.

Not measured: human judgement of any reply; whether the called-for word changes what the judges read; any dialogue longer than six turns; any persona other than these three.

## 5. The one idea

A persona is a set of conditions, so talking to one means naming a condition and reading what it calls for.

The question is treated as a pointer into the lexicon. Most of its words are function words or words used everywhere. They say little about which condition the user means. The rarest known content word says most. So the question is reduced to one word, the topic. This is crude, and it is also exactly what the paradigm asks for. A condition is a word that other words move under. The rarest content word is the one least diluted by every other use.

The persona is treated as a pool plus a reading. The pool is the 600 content words that writer uses most. The reading is `Facet::calls_for`, the directed relation X ⇒ Y read through the shared lexicon. The shared lexicon means the three personas speak with the same relations. The pools differ, so the same condition calls for different words in each. Under *love*, Austen's pool yields *regard*, Chesterton's yields *smiling*, Carroll's yields *notice*. No list of topics or answers exists anywhere in the code.

The reply is treated as a walk toward the called-for word, taken under that writer's sector histogram and pair evidence. The walk is a trigram walk. It is conditioned, and it is pulled, and it is still a trigram walk. That is why its sentences read as they do.

The judgement is treated as a separate act. The reply is not scored by whether it is correct, since there is no correct reply. It is scored by whether it sounds like the persona, by the persona's own fingerprint, and whether it holds together, by the evaluator. It is then read, after the fact, by two judges that know nothing about how it was made. That separation is what lets the result be stated honestly. The engine generates under its own reading of the persona. An outside model checks whether the writer can be recovered.

Restated in one line: the question names a condition, the persona's manifold says what that condition calls for, the answer is generated toward it, and the answer is then read back to see whose it is. The first three clauses are the mechanism. The last clause is the measurement. The two judges disagree about the last clause, and the disagreement points at the engine's reader.

The idea holds even where the output is weak. The replies are not good English. They are, at 72% by the panel, recognisably the writer who spoke. That is the thing a persona is supposed to carry. The sources make no claim about grammar.

## 6. The condition

`Dialogue::topic` reads the user's turn and returns one word. Its rule is in four lines of `reading.rs`. Tokenize the turn. Skip every function word and every word of two letters or fewer. Look the rest up in `facet.lexicon`, skipping any word the lexicon does not know. Keep the word with the smallest `evidence_total()`, floored at 1.0. The comparison is strict, so between two equally rare words the one said first wins.

The comment in the source gives the reason: the rarest word is the most specific, "because a word the corpus uses everywhere names no condition in particular". Evidence total is the count the paradigm attaches to every relation. A word with little evidence is one the three writers used under few conditions. That makes it a sharp condition to answer under.

The six questions of the transcript show the rule at work.

| turn | the user's words | condition chosen |
|---|---|---|
| 1 | good morning how are you today | today |
| 2 | what do you think about marriage | marriage |
| 3 | tell me about money and family | money |
| 4 | and what of love | love |
| 5 | what should a young woman do with her afternoon | afternoon |
| 6 | why does any of this matter | matter |

In turn 1, *today* beat *morning* and *good*. In turn 3, *money* beat *family* and *tell*. The user asked about two things, and the rule kept one. There is no way in the current code to answer under two conditions at once. In turn 5, *afternoon* beat *young* and *woman*. The question's social content, what a young woman should do, is thrown away, and only the time of day survives. In turn 6, *matter* was kept. Here the word is used in its sense of "to be important", and the lexicon has no sense distinction to draw on. The persona is asked what *matter* calls for, across every sense the three writers used.

Every persona got the same condition for every turn. That follows from the rule. The topic is read from the shared Facet, which all three personas share, and not from the persona's pool. The persona enters only at the next step.

The rule has two limits the transcript does not test. A question made only of function words and unknown words has no topic, and then there is no goal. The seeds then fall back to the question's tail, and the reply is generated with no pull. And a misspelt or modern word the lexicon lacks is silently skipped. The user is not told which word was answered under. The `Reply` does carry `topic`, so a caller can show it, and `bin/persona_chat` prints it on every line as "under *today*".

## 7. What it calls for

Once the topic is fixed, `Dialogue::calls_for` asks the persona's manifold what that condition calls for. The candidates are the persona's pool: the 600 most frequent content words in its training examples, longer than two letters, known to the lexicon, ties broken alphabetically. `answer_pool` builds it once, in `Dialogue::open`. The topic itself is excluded. So is every word the user said in this turn. Each remaining pool word `w` is scored by `facet.calls_for(w, topic)`, the directed reading X ⇒ Y from the topic to the word. The highest score wins. If the best score is zero or less, there is no goal.

The transcript prints every result:

| condition | G. K. Chesterton | Jane Austen | Lewis Carroll |
|---|---|---|---|
| today | ball | temper | temper |
| marriage | sudden | former | melancholy |
| money | staring | decided | grow |
| love | smiling | regard | notice |
| afternoon | few | fine | hot |
| matter | broad | former | ten |

The eighteen called-for words are sixteen distinct words. Two repeat. Under *today*, Austen and Carroll both answer *temper*. This means *temper* is in both pools, and the shared lexicon reads it as the strongest answer from *today* in both. Austen answers *former* under *marriage* and again under *matter*. Nothing in `calls_for` excludes a goal already used earlier in the dialogue, so the same word can be called for twice.

Some pairs read plainly. *love* ⇒ *regard* for Austen is one. *afternoon* ⇒ *hot* and *afternoon* ⇒ *fine* both read as weather under a time of day. *marriage* ⇒ *melancholy* for Carroll and *marriage* ⇒ *former* for Austen are less clear. *matter* ⇒ *ten* for Carroll has no reading a person would recognise. The paper does not claim these pairs are good. They are the lexicon's directed relations, filtered through each writer's frequent words. The pool is what makes them the persona's. The relation is shared.

The goal word is then handed to the generator with `with_goal(w, DIALOG_GOAL_PULL)`, a pull of 1.5. It also appears in one third of the seeds, as the second word of the "topic and goal" seed. Counted from the transcript, the called-for word appears in none of the 18 spoken replies. *staring* appears in Chesterton's turn 5 reply ("strong and staring white hands"), but that turn was called for *few*, and *staring* was the goal of turn 3. The pull shapes the walk. It does not put the word on the page.

Whether the pull changes what the judges read has not been measured. No run with the goal removed exists in the sources. Section 12 names that run as a control.

## 8. Nine candidates

Doc 81 gives the reason for nine: "nine walks from one context are one answer". The trigram tables have strong attractors. Docs/78 measured 25.7% shared 4-grams across prompts for the trigram sampler. Doc 81 and the code comment round this to 26%. Nine walks from one seed at one temperature tend to fall into the same attractor. So the nine start from three different places and run at three temperatures.

The seed and temperature grid for turn 4, "and what of love", for Austen, whose goal was *regard*:

| i | seed rule | seed text | temperature |
|---|---|---|---|
| 0 | tail, 6 tokens | and what of love | 0.30 |
| 1 | topic | love | 0.30 |
| 2 | topic and goal | love regard | 0.30 |
| 3 | tail, 5 tokens | and what of love | 0.45 |
| 4 | topic | love | 0.45 |
| 5 | topic and goal | love regard | 0.45 |
| 6 | tail, 4 tokens | and what of love | 0.60 |
| 7 | topic | love | 0.60 |
| 8 | topic and goal | love regard | 0.60 |

This assumes the tokenizer keeps all four words of the question, which the sources do not state. With a four-word question, the tail seeds of 6, 5 and 4 tokens are the same text. The three tail candidates then differ only in temperature and random stream. For turn 1, "good morning how are you today", the tails differ: six words, then "morning how are you today", then "how are you today".

Each candidate has its own random stream, `phinet::mix64(self.turn * 7919 + i)`. The turn counter starts at zero, so turn 4 uses streams `mix64(23757 + i)` for i from 0 to 8. The stream depends on the turn and the index only. It does not depend on the persona. Chesterton, Austen and Carroll draw from the same nine streams on the same turn. On turn 6 both Chesterton and Austen begin "Out thoroughly before". A shared stream from a shared tail seed is a likely cause. It has not been checked.

Every candidate uses `Decode::Trigram`, the tables, and not the operator. It is conditioned by the persona prior, built with a zero weight on the fingerprint argument of `PersonaPrior::new` and a weight of 1.0 on the persona's own examples through `with_examples`. Each candidate is 28 tokens long. Each runs against the dialogue's context wave, swapped in and swapped back, so the wave holds the earlier turns and nothing from sibling candidates.

A candidate of fewer than three tokens is dropped without a score. The transcript does not report how many were dropped on any turn. The eight candidates that lost on each turn are not saved. That is a gap. The per-candidate spread of fit, quality and repeat would show how much the choice step is choosing, and it is not recorded.

## 9. The choice

Each surviving candidate gets three numbers. Fit is `self.fingerprint.likelihood` of the candidate's own fingerprint: how likely the speaking persona finds a text with this sector profile. Quality is `Evaluator::eval(facet, &text).overall`, coherence against the manifold. Repeat is the share of the candidate's 4-grams that are stale.

`Dialogue::repeat` counts a 4-gram as stale in two cases. It is stale if this dialogue already spoke it on an earlier turn. It is also stale if the candidate itself already used it a few words earlier. So a candidate that loops inside itself pays the same cost as one that repeats the previous answer. The count is over the persona's own dialogue only. Each persona has its own `Dialogue` and its own set of spoken 4-grams. A phrase Chesterton said does not count against Austen.

The combined score is the weighted sum of fit and quality, multiplied by `1 - 0.8 × repeat`. At a repeat of 12% the multiplier is 0.904. At the limit of 25% it is 0.8. Above 25% the candidate is refused outright, whatever it scores. The best candidate at or under the limit speaks. If none qualifies, the least repetitive candidate speaks instead.

The transcript shows the fallback path running twice, both times for Austen. On turn 3, "tell me about money and family", Austen's spoken reply has a repeat of 40%. On turn 6, "why does any of this matter", it has 28%. Both are over 25%. By the code, a reply over the limit can only be spoken when no candidate was under it, and the spoken one is the freshest. So on turn 3 every Austen candidate that survived repeated at least 40% of its 4-grams. On turn 6 every one repeated at least 28%.

The Austen replies show where the repeats come from. "to be sure you will soon have been so much more than she had never seen him" on turn 1. "to be sure you must have been so much more than she had never seen him" on turn 3. "were to be sure you will not like him" on turn 4. "to be very much obliged to say" on turns 5 and 6. Austen has the most text of the three, 247,252 words, and her tables have the deepest attractors.

Across the 18 replies, 10 have a repeat of 0%, five have 12%, one has 8%, one 28% and one 40%.

The fit and quality values of the spoken replies sit in narrow bands. Fit is 0.754 to 0.806. Quality is 0.641 to 0.731. The lowest quality, 0.641, is Chesterton's turn 5 reply, which the panel read as Chesterton at ×10.17. Quality and attribution are not the same thing.

## 10. Two judges that disagree

Each reply was read alone by two judges. The first is the engine's sector attribution: the persona whose fingerprint gives the reply the highest likelihood. The second is a `Panel` of Kneser-Ney trigram models, one per writer, trained on that writer's sentences. The least surprised model wins.

| turn | spoke | sectors read | KN panel read | KN ppl | factor |
|---|---|---|---|---|---|
| 1 | Chesterton | Austen | Chesterton | 30 | ×1.05 |
| 1 | Austen | Austen | Austen | 15 | ×4.43 |
| 1 | Carroll | Austen | Austen | 24 | ×1.69 |
| 2 | Chesterton | Carroll | Austen | 48 | ×1.11 |
| 2 | Austen | Austen | Austen | 18 | ×4.77 |
| 2 | Carroll | Austen | Carroll | 29 | ×9.39 |
| 3 | Chesterton | Austen | Chesterton | 19 | ×15.19 |
| 3 | Austen | Austen | Austen | 12 | ×5.59 |
| 3 | Carroll | Austen | Austen | 42 | ×1.51 |
| 4 | Chesterton | Chesterton | Chesterton | 19 | ×6.43 |
| 4 | Austen | Austen | Austen | 13 | ×13.66 |
| 4 | Carroll | Austen | Austen | 38 | ×3.25 |
| 5 | Chesterton | Chesterton | Chesterton | 28 | ×10.17 |
| 5 | Austen | Austen | Austen | 22 | ×3.56 |
| 5 | Carroll | Austen | Carroll | 42 | ×2.86 |
| 6 | Chesterton | Austen | Chesterton | 24 | ×2.84 |
| 6 | Austen | Austen | Austen | 24 | ×2.60 |
| 6 | Carroll | Austen | Austen | 34 | ×1.62 |

The factor is printed by the transcript beside each panel verdict. Its definition is not stated in the sources read for this paper.

The sector reader's failure is uniform. It names Austen for 15 of 18 replies, with a margin of 0.00 or 0.01 every time. Docs/79 §3 predicted exactly this. Austen has the most text, and the three fingerprints overlap at 0.98. A reader that weights sectors by how often each writer uses them will favour the writer who uses them most. Its 44% is Austen's six, plus two Chesterton replies.

The panel's failure is narrower. It misses one Chesterton reply, turn 2, reading it as Austen at ×1.11, close to a tie. It misses four Carroll replies, all read as Austen, at ×1.69, ×1.51, ×3.25 and ×1.62. Carroll has one book and 18,305 words. His replies carry phrases the Austen replies also carry: "to be sure" on turn 5, "you know what" on turn 6. On turn 4 his reply names "macian". The generator decodes through the shared Facet's trigram tables, and the persona prior only weights them. Words from one writer's text can enter another writer's walk, and the panel reads them where they came from.

Where the panel is sure, the text shows why. Chesterton's turn 3 reply at ×15.19 names Father Brown and Syme and "small sums of money". Austen's turn 4 at ×13.66 names Jane and Emma. Carroll's turn 2 at ×9.39 names the king twice and Alice once. Doc 81's reading is that the persona conditioning is choosing the company each writer keeps. The panel sees that company. The sector reader reads sectors, and the three writers' sectors overlap at 0.98.

## 11. Where it sits in the PML family

The dialogue is a Phiano feature. Phiano is the word application: Facet, Trainer, generation, personas, quest ingest, helix. `Dialogue` reads a `Facet`, generates with a `Generator`, scores with an `Evaluator`, and conditions on a `Persona`, a `Fingerprint` and a `PersonaPrior`. All of these are Phiano types. The only direct call into phinet in `dialog.rs` is `phinet::mix64`, for the random streams.

Beneath Phiano sits phinet, the parent engine: Tensor `[b,w,s]` complex, Tape autograd, unitary FFT, Fno, joint training, HRR, 64 hexagram gates, and the sphere. phinet matches a PyTorch reference on 62 of 62 parity tests. The lexicon the dialogue reads is made of phinet's phasors. Each word is 32–64 relations, each a complex phasor on a closed circle with a count. `evidence_total`, which picks the topic, is the sum of those counts. `Facet::calls_for`, which picks the answer word, is a directed reading of those phasors. Phical is the geometry of that circle. A channel is one byte, 256 quanta of 2π, so every relation the dialogue reads is quantised to that step.

The operator trained on Austen is the part of the family the dialogue most needs and cannot yet use. Softmax wide+deep, 50,016 params, reaches 30.7× chance valid and 30.3× test at 8 epochs. Decoding through it gives 8.7% shared 4-grams against the trigram sampler's 25.7%. Doc 81 rounds these as 10% and 26% from docs/78. The facts sheet is the later record, and this paper uses its figures. The operator in `data/austen_operator.fno` is Austen's alone. A shared-lexicon operator must be trained before the dialogue can decode through it. That operator is not built. The sphere tie and the anisotropic tie were measured on the Austen hinge and softmax settings only. Neither has been tried on a persona lexicon.

Liskov's three operators in `phinet::liskov` offer another choice rule. Docs/86 found that forcing does not beat greedy on text: nearest-k KN perplexity 66.9, forcing λ=0.05 69.7, λ=0.25 68.5, λ=1 107.4. The dialogue's choice step does not use them.

The helix codes an 18,496-word lexicon in 1.9 s, 75 MB, byte-exact round trip. That is the same word count as the shared persona lexicon. The sources do not say whether it is the same lexicon.

phifno 1.0.0, phiton, phinum and phiqlm are the other children of phinet. phiqlm is planned, and nothing is built. None of them is used by the dialogue. phiecs, the entity-component world and quest, does not appear in this paper's sources. Neither does phiace. Nothing in the sources connects the persona dialogue to either.

What depends on the dialogue is, for now, only `bin/persona_chat` and the transcript it wrote.

## 12. The next experiment

The next thing named in doc 81 is a discriminative attribution reader. It would weight sectors by how well they separate writers, rather than by how often each writer uses them. It is not built. The experiment below is how it should be judged.

The test set is fixed before the reader is written. It has two parts. The first is the held-out attribution set already used in docs/81: four-sentence paragraphs, 150 per author, from the 10% held-out split of each writer, split by sentence hash with seed 7. The second is a fresh `persona_chat` run: the same six questions, plus new ones, to reach a larger number of replies than 18. Replies are saved with all nine candidates, not just the spoken one, so that the choice step can be studied too.

There are two controls. The first control is the current sector reader, unchanged. On held-out paragraphs it scores 71% overall, 74/65/86 by author. On the 18 replies it scores 44%. The new reader has to beat both on the same text. The second control is the Kneser-Ney panel, one model per writer. On the 18 replies it scores 72%. Doc 81 names it as the yardstick the new reader has to beat. The panel is trained on each writer's sentences. For the comparison to be fair, the panel must be trained on the training split only, and the split must be stated in the run. The sources read for this paper do not say which split the panel used.

The measurement is the share read as the persona who spoke, per author and overall, against chance of 33%. The number to watch is Carroll. The sector reader got 0 of 6 of his replies. The panel got 2 of 6. His held-out paragraphs score 86% with the current reader. A discriminative reader that fixes his replies without losing his paragraphs would place the fault in how the fingerprint's sectors are weighted.

A second experiment, smaller, tests the goal. Run the same questions with `with_goal` removed, so the called-for word enters only through one seed in three. Then run with the goal seed also replaced by the tail. Keep the random streams, `mix64(turn * 7919 + i)`, identical across the three runs. The control is the current run with the goal. If the panel's 72% does not move, the called-for word is not doing work that the judges can see. That would not falsify the claim that the manifold says what a condition calls for. It would show the generator is not yet able to carry it.

Neither experiment has been run. Neither needs new data. Both need the candidates saved, which the current `Dialogue::reply` discards.

## 13. What is wrong or missing

The engine's own reader cannot tell the personas apart in the dialogue. It names Austen for 15 of 18 replies, with a margin of 0.00 or 0.01 every time. It gets no Carroll reply right. The three fingerprints overlap at 0.98. This is the weakest link in the chain, and doc 81 says so.

The same fingerprint drives the choice. Fit is the speaking persona's likelihood of the candidate's fingerprint, taken alone. It is not compared with the other personas' likelihoods. So the choice step prefers candidates the speaker finds likely, even when another persona finds them likelier. The choice optimises the same weak signal that fails as a judge.

The called-for word appears in none of the 18 replies. The pull of 1.5 and one seed in three are not enough to place it. Whether it changes what the judges see is not measured.

The replies are not grammatical English. They are 28-token trigram walks, cut at the token limit, with a full stop added. They carry the tables' idiom across personas: "to be sure", "very much obliged to say", "as you know what". Chesterton's and Austen's turn 6 replies start with the same three words. The random stream is the same for every persona on a turn, which may cause this. It is not checked.

The manifold has no punctuation or case (docs/79 §2.2). No persona can be told by its commas.

The topic rule keeps one word. "Tell me about money and family" is answered under *money* alone. "What should a young woman do with her afternoon" is answered under *afternoon* alone. A called-for word can repeat within one dialogue, as *former* does for Austen, because earlier goals are not excluded.

Two Austen replies were spoken over the repeat limit, at 40% and 28%, because every candidate on those turns was over it.

The measurement is small. Eighteen replies, six questions, three writers, one run. A change of one reply moves a judge's score by more than five points. No human read the replies as a judge.

The panel's training split is not stated in the sources. If it was trained on the same training sentences the persona prior was built from, it is reading the generator's own source text back. That is a fair test of "does this sound like the writer" and a weak test of anything more.

The two weights in the combined score, `IMPERSONATE_FIT_WEIGHT` and `IMPERSONATE_QUALITY_WEIGHT`, are not in the sources read here. The losing candidates are not saved. The transcript prints a factor beside each panel verdict without defining it.

The dialogue does not use the operator. The only operator trained is Austen's.

## 14. How it was found

The history runs through five documents.

Docs/78 measured decoding. The trigram sampler shared 25.7% of its 4-grams across prompts. The operator shared 8.7%. That number set up the dialogue's repeat rule. The source comment on `DIALOG_REPEAT_MAX` cites it: the trigram tables have strong attractors, "and a dialogue is where that shows most". The same doc is why doc 81 says operator decoding would loosen the replies' idiom.

Docs/79 built the persona conditioning. §3 describes the generator conditioned on the persona's sector histogram and its own pair evidence, which is the `PersonaPrior` the dialogue uses. §3 also predicted the failure the sector reader showed: Austen has the most text, and the three fingerprints overlap at 0.98. §2.2 records that the manifold has no punctuation or case.

Before docs/81, personas were rebuilt on every run from 600 sampled sentences each, in `bin/austen`. The lexicon under them was trained on Austen's training split plus the contrast authors' whole books. The contrast authors' held-out text leaked into the manifold. Attribution in that setting was 62% overall: Chesterton 73%, Austen 52%, Carroll 57%.

Docs/81 fixed both faults with `bin/persona_ingest`. Every author is split 80/10/10 by sentence hash, seed 7. The shared lexicon is trained on the three training splits only. Each persona is ingested from its whole training split. Attribution rose to 71%: Chesterton 74%, Austen 65%, Carroll 86%. The personas were saved to `data/personas.chroma` and `data/personas.json`, so `bin/persona_chat` could open them in about a second instead of rebuilding them.

The same doc built `persona::Dialogue`. Its four steps are the condition, what it calls for, the reply and the choice. The number nine and the three seed positions came from the observation, stated in the code, that nine walks down one trigram path are one answer. The repeat cost and refusal came from the same trigram attractors docs/78 measured.

Docs/81 then ran the dialogue and read it with two judges. The Kneser-Ney panel was added as a second judge because the engine's reader was known, from docs/79 §3, to lean toward Austen. The panel gave 72%, the sector reader 44%. Doc 81 drew the conclusion that the engine's attribution is now the weakest link in the persona chain.

Paper 16, `16_PERSONA_DIALOGUE_TRANSCRIPT.md`, is the verbatim transcript of that run. It carries the per-reply numbers that doc 81 abbreviates: turn, condition, called-for word, fit, quality, repeat, both verdicts. The counts in sections 4, 9 and 10 of this paper are taken from it.

Later, docs/86 tried Liskov's three operators as a decoding choice rule. Forcing did not beat greedy on text. That result bears on any future change to the dialogue's choice step, but docs/86 did not touch the dialogue.

## 15. What remains

A discriminative attribution reader remains to be built. It should weight sectors by how well they separate writers, and it should be judged against the current sector reader and against the Kneser-Ney panel, on held-out paragraphs and on dialogue replies. Section 12 gives the run. It is not built.

The choice step should then use the new reader. Today, fit is the speaking persona's likelihood taken alone. A fit that compares the speaker's likelihood with the other personas' would choose replies that are more the speaker's and less anyone's. That change depends on the new reader and is not built.

A shared-lexicon operator remains to be trained. The only operator in the sources is Austen's, in `data/austen_operator.fno`. Decoding through an operator shared 8.7% of 4-grams across prompts against the trigram sampler's 25.7%. A dialogue decoded through one would carry less of the tables' idiom.

Punctuation and case remain to be added to the manifold (docs/79 §2.2). Until then no persona can be told by its commas, and every name comes out in lower case.

The dialogue should save every candidate, with its seed, temperature, fit, quality and repeat. Without that, the spread the choice step is choosing from cannot be studied, and the goal cannot be ablated cleanly.

The goal needs its own test. The called-for word appears in none of the 18 replies. A run with the goal removed, with the random streams held fixed, would show whether the word changes what the judges read. Not run.

The random streams should depend on the persona. `mix64(turn * 7919 + i)` gives all three personas the same nine streams on a turn. Whether this causes shared openings, as on turn 6, is not checked.

The topic rule could keep more than one word, or keep the rarest two. A question about "money and family" names two conditions. The current code answers under one.

The measurement needs to be larger. Eighteen replies from one run is enough to show that the judges disagree and in which direction. The panel's training split should be stated in every run.

More personas remain. Carroll, with 18,305 words and one book, is the thinnest. He gained most from full ingestion, from 57% to 86%, and he is the one the dialogue's judges read worst. A fourth writer with more text than Carroll and less than Austen would show whether the fault follows text size.

Nothing here claims a persona that holds a conversation. The claim is that a condition, read through a writer's manifold, produces text an outside model can attribute to that writer at 72% against 33% chance. That claim stands on 18 replies and is ready for a larger test.

## 16. Sources

These are the local files read for this paper. Nothing else was read, and nothing on the web was used.

| path | what was taken from it |
|---|---|
| `/home/claude/phiano/docs/papers/phient/BRIEF.md` | the form of the paper: front matter, sixteen sections, the arc, the style rules |
| `/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` | the paradigm paragraph; the PML family; the numbers for phinet parity, the Austen operator, decoding, the sphere and anisotropic ties, the byte deadband, the helix, persona attribution, the dialogue judges, and Liskov's operators |
| `/home/claude/phiano/docs/81_persona_dialogue.md` | ingestion: the 80/10/10 split, seed 7, the per-author table, the lexicon of 18,496 words, 24 marker channels, three passes, 12 s, 8 s, 53 s; attribution 62% to 71%; the four steps of `Dialogue`; the sample replies; the judges' 44% and 72%; what remains |
| `/home/claude/phiano/docs/papers/16_PERSONA_DIALOGUE_TRANSCRIPT.md` | all 18 replies verbatim, with turn, condition, called-for word, fit, quality, repeat, sector verdict and margin, panel verdict, perplexity and factor; the totals 8 and 13 |
| `/home/claude/phiano/src/persona/dialog.rs` | the constants, `Reply`, `Dialogue`, `Dialogue::open`, `Dialogue::grams`, `Dialogue::repeat`, `Dialogue::reply`, the seed grid, the temperature rule, the random streams, the combined score, the fallback |
| `/home/claude/phiano/src/persona/dialog/reading.rs` | `Dialogue::answer_pool`, `Dialogue::topic`, `Dialogue::calls_for` |

Several documents are cited in this paper through those sources and were not read directly. Docs/78 is cited for decoding and shared 4-grams, through doc 81, the code comment on `DIALOG_REPEAT_MAX`, and the facts sheet. Docs/79 §2.2 and §3 are cited for the missing punctuation and case, the persona conditioning, and the 0.98 fingerprint overlap, through doc 81. Docs/86 is cited for Liskov's operators, through the facts sheet.

Where the sources disagree, this paper says so and prefers the later record. The one disagreement found is the shared 4-gram figures from docs/78. Doc 81 gives 10% for the operator and 26% for the sampler. The code comment gives 26% for the sampler. The facts sheet gives 8.7% and 25.7%. The facts sheet figures are used here.

Counts in sections 4, 9 and 10 that do not appear as totals in any source are counted directly from the transcript's 18 lines: per-persona scores for each judge, how often each judge named each writer, where the judges agreed, the spread of repeat values, and the absence of the called-for word from every reply. The seed table in section 8 is worked from the code, and it assumes the tokenizer keeps all four words of "and what of love", which the sources do not state. The multipliers 0.904 and 0.8 in section 9 are worked from `DIALOG_REPEAT_COST`. The stream offset 23757 in section 8 is worked from the code's `self.turn * 7919` at turn index 3.

No source file was modified.
