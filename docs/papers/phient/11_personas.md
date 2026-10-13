# Personas
*Phient papers, 11 of 16 · 2026-06-01 · Personas: a writer as the conditions they put words under — fingerprints, full ingestion, attribution at 71%*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

A persona in Phiano is the set of conditions one writer puts words under, gathered into counts. The lexicon is shared. Each word in it is a `SpectralPhasor` of 64 channels. A persona owns no words. It owns two kinds of count: how often its sentences put words into each of the 64 sectors of the relation circle, and which pairs of words its sentences put side by side. This paper covers three things built on that idea. The first is the fingerprint, `persona::Fingerprint`, and the reader that attributes a text to a writer, `World::match_text`. The second is full ingestion, `bin/persona_ingest`, which trains one lexicon on three writers' training splits and builds each persona from its whole training split. The third is generation under a persona, through `Generator::with_persona` and `with_persona_examples`, and the dialogue built on it, `persona::Dialogue`.

The writers are G. K. Chesterton, Jane Austen and Lewis Carroll. Chesterton gives three books, Austen three novels, Carroll one. Attribution is measured on held-out paragraphs of four sentences. Chance is one in three. With personas built from 600 sampled sentences each, attribution read 62%. With personas ingested from every training sentence it reads 71% overall: Chesterton 74%, Austen 65%, Carroll 86%. Carroll gained most. 600 sampled sentences were about half of his 1,102.

Generation under a persona is measured by reading the generated text back with the same attribution reader. With no conditioning, 24 prompts read back as the persona asked for 33% of the time, which is chance. Conditioning on the persona's own bigram evidence at weight 1 raised that to 53%. Conditioning on the sector histogram alone reached 43%. The `Impersonator` went from 39% to 56% once `Composer::compose` generated its text through `Generator` in place of four fixed frames. Its Kneser-Ney perplexity under Austen's judge fell from 4,478 to 23.6 in the same change. The dialogue's 18 replies read back as their speaker 44% of the time under the sector reader and 72% under a panel of one Kneser-Ney model per writer.

The result is plain. A writer's conditions, gathered over a shared lexicon, separate three writers well above chance. The sector histogram is the weaker half of a persona. The persona's own pair evidence moves generated text more than its histogram does. The engine's own attribution reader, the same reader that scores 71%, is now the weakest link in the chain, because it leans toward the writer with the most text.

## 2. The claim

The claim comes from the paradigm. docs/79 extends it by one sentence: "A persona is which conditions a writer puts words under, aggregated; a corpus is every condition a language has been observed in." If a word is what it relates to under conditions, then a writer is a habit of conditions. Two writers who share a vocabulary still differ in which relations they put their words under, and how often. That difference should be readable from a text alone. It should need no labelled features, no stylometric list and no second model per writer.

The claim has three parts, and each can be tested.

First, one lexicon is enough. A persona is a set of counts over the lexicon's sectors and pairs, so personas need no lexicons of their own. Before docs/79, "her voice" in `bin/austen` was a separate lexicon per author. The Austen one is `data/austen_voice.chroma`, 10,501 words trained on her training split alone. docs/79 conditioned one shared lexicon instead. docs/81 made the shared lexicon the stored form, `data/personas.chroma` with `data/personas.json`.

Second, attribution follows from the counts. A text has a fingerprint too, computed the same way over the same lexicon. The persona under which that fingerprint is most likely is taken to have written it. Nothing is trained to discriminate. The reader asks each persona how probable the text's sectors are, and the highest likelihood wins.

Third, composition follows from the same counts. To write as a persona is to generate under its conditions. docs/80 says it in one line: "A persona is a set of conditions; composing as one is generating under them." docs/81 gives the dialogue form. The question names a condition. The persona's manifold says what that condition calls for. The answer is generated toward it.

The claim has limits, and the sources state them. The judge in every measurement here is the engine's own attribution reader or a Kneser-Ney trigram model. Both read word choice and word order. Neither reads punctuation, case or clause rhythm, because `Tokenizer::tokenize` deletes punctuation, case, apostrophes and hyphens before anything is counted (docs/79 §2.2). docs/79 states the consequence: "a persona lives at the word-bag level whatever else improves."

The claim also does not need the three writers to sit far apart on the circle. They do not. Their fingerprints share the same dominant sectors, 44 to 48. Austen and Chesterton have similarity 0.980. What is claimed is smaller and exact. The small differences that remain are enough to attribute a held-out paragraph at 71% where chance is 33%, and enough to push generated text toward the writer asked for at 53% and 56% where chance is the same.

## 3. The mechanism

`Persona::from_examples` builds a persona from example sentences. At its centre is a `Fingerprint`: a histogram over the 64 sectors of the relation circle, with a sample count and a mean length. `Fingerprint::extract(facet, examples)` reads each word of each sentence, finds its sector and adds to that bin. The sector of a word is `Fingerprint::relational_sector(facet, word, mean)`. It reads the word's target-side function minus the lexicon mean, `Fingerprint::lexicon_mean(facet)`. The centring came in with docs/76 and is covered in section 7. The histogram's entropy is the persona's breadth. `PersonalityMapper` reads a temperament off the colour bands. The Q64 name, `Fingerprint::q64`, is the sector of the centroid, which is also a hexagram.

`World` holds the personas. `World::match_text(facet, text)` fingerprints the text and asks each persona for `Fingerprint::likelihood` of it. The persona with the highest likelihood comes back in a `MatchResult`. `World::compare` reports the similarity of two personas, and `difference_vector` the sectors that separate them. `overlap_after(other, t)` smooths both histograms by heat flow on the circle for time `t` and reports how much they overlap. `PersonaStore` saves a `World` with the name of its lexicon and loads it read-only. `bin/persona_ingest` writes the store. `bin/persona_chat` opens it.

Generation under a persona lives in `src/generate/persona.rs`. `PersonaPrior` holds the persona's normalised histogram, a `weight`, the lexicon mean, a memo of word sectors, the persona's own bigram table and an `ngram_weight`. For each candidate the generator adds `persona_bonus`, the sum of two terms:

```
bonus(word)             = weight · clamp(ln(p(sector) · n), −2, +2)     n = number of sectors
ngram_bonus(prev, word) = ngram_weight · ln(1 + count(prev, word))
```

The first term is the sector prior. `p(sector) · n` is the persona's probability for the word's sector over the uniform probability, and `PERSONA_CLIP = 2.0` bounds it either way. docs/79 describes it as "`weight · ln p(sector)/uniform` from the fingerprint histogram, clipped ±2". The second term is pair evidence, counted by `PersonaPrior::with_examples` over the persona's own sentences. The code comment names it: "the conditions *this* writer put pairs of words under."

`Generator::with_persona(facet, fp, weight)` sets the sector prior only. `Generator::with_persona_examples(facet, fp, examples, weight, ngram_weight)` sets both. `with_persona_prior` takes a prior already built, because one persona's bigram table is large and a dialogue reuses it across turns.

Two consumers sit on top. `Impersonator::impersonate_with` composes six candidates with `with_persona_examples` at sector weight 0 and pair weight 1. It scores each by the persona's likelihood of the candidate's own fingerprint and by the evaluator's quality, and returns the best (docs/80). `persona::Dialogue` in `src/persona/dialog.rs` answers a turn in four steps, set out in section 10. Nothing in either path writes the lexicon.

## 4. What was measured

Seven numbers carry this paper. Chance is 33% in every row.

| measurement | harness | size | result |
|---|---|---|---|
| attribution, 600 sampled, contrast text in the lexicon | `bin/austen` (docs/papers/15) | 335 paragraphs | 70% (233 of 335) |
| attribution, 600 sampled, clean split | `bin/persona_ingest` (docs/81) | paragraphs of four | 62% |
| attribution, full ingestion | `bin/persona_ingest` (docs/81) | paragraphs of four | **71%** (74 / 65 / 86) |
| generation read back, persona bigrams at 1 | `bin/persona_eval` (docs/79) | 24 prompts | 53% |
| impersonation read back, before and after | `bin/impersonate_eval` (docs/80) | 12 prompts | 39% → 56% |
| dialogue, sector reader | `bin/persona_chat` (docs/81) | 18 replies | 44% |
| dialogue, Kneser-Ney panel | `bin/persona_chat` (docs/81) | 18 replies | 72% |

The first row comes from the generated report docs/papers/15. There the true-author rows read Austen 67% on 150 paragraphs, Chesterton 79% on 150 and Carroll 40% on 35. The lexicon under that run was trained on Austen's training split plus every contrast author's whole books. The contrast authors' held-out text was therefore inside the manifold the reader used. docs/81 names that leak and removes it. The second row is the same 600-sample persona on a lexicon trained on the three training splits only. It reads 62%, lower than 70%. This paper prefers the later document and treats 62% as the clean baseline for sampled personas.

The third row is the headline. It is measured "the same way both times" as the second, on held-out paragraphs of four sentences, 150 per author, the persona with the highest likelihood winning. Section 8 returns to the phrase "150 per author", which cannot hold for Carroll.

The fourth and fifth rows judge generated text with `World::match_text` on the composed text alone. `bin/persona_eval` takes 24 prompts: 8 held-out sentences from each author, first three words, 30 tokens, trigram decoding on the shared `austen.chroma`. `bin/impersonate_eval` takes 12 prompts, 4 per author, on the shared three-author lexicon. Its fluency judge is Kneser-Ney on Austen's training split: 4,478 before and 23.6 after, with distinct-token ratio 0.53 before and 0.86 after. docs/79 cites the same change as 4,478 → 24.

Timings are small. In docs/81 the shared lexicon of 18,496 words took 12 s over three passes. Ingesting all three personas took 8 s. The whole ingest run took 53 s. `bin/persona_chat` opens the store in about a second and answers six questions for three personas in 17 s. docs/papers/15 records its whole `bin/austen` run at 249 s.

## 5. The one idea restated

A writer is the conditions they put words under.

The clearest instance in the sources is one word asked of three writers. In the dialogue, the question "and what of love" names the condition *love*. Each persona then reads what *love* calls for over its own 600 most-used content words, with `Facet::calls_for`, the directed reading X ⇒ Y. Austen answers **regard**. Chesterton answers **smiling**. Carroll answers **notice**. The lexicon is the same object for all three. The phasors of *love*, *regard*, *smiling* and *notice* are the same numbers in each case. What differs is the set of words each writer has put under conditions often enough to rank among its 600, and so the set over which the reading is taken.

The same holds for *marriage*: **former** for Austen, **sudden** for Chesterton, **melancholy** for Carroll. And for *money*: **decided** for Austen, **staring** for Chesterton. No rule says Austen's writing concerns regard. The answer falls out of which pairs she moved, and how often.

Put in the terms of docs/79, a corpus is every condition a language has been observed in, and the shared lexicon holds those conditions. A persona is a subset of those observations, gathered per writer. Three operations use the subset. Attribution asks which subset best explains a new text. Composition generates while favouring the subset. Dialogue picks the subset's answer word and generates toward it.

The subset has two grains. The coarse grain is the sector histogram: for each of 64 sectors, how often this writer's words fall there. The fine grain is the pair table: for each ordered pair of words, how often this writer put them side by side. The measurements say the fine grain carries more. Under generation the pair evidence alone gives 53% and the sectors alone 43% (docs/79). Under judgment a per-writer trigram model separates the dialogue's replies at 72% and the sector reader at 44% (docs/81). A pair is a condition with both words named. A sector is a condition with one word named and the other averaged away. The finer the condition, the more of the writer it holds.

That is the whole idea, and it is the paradigm applied once more. Meaning is a side effect of pairs of words moving under a condition. A writer's identity is a side effect of which pairs that writer moved. The paper's numbers measure how far that side effect can be read back from text with no punctuation, one shared lexicon and three writers whose sector histograms nearly coincide.

## 6. Fingerprints in depth

docs/papers/15 prints the three fingerprints as `Persona::from_examples` built them, from 600 sampled sentences each.

| persona | samples | mean length | entropy | Q64 (King Wen) | temperament | dominant sectors |
|---|---|---|---|---|---|---|
| G. K. Chesterton | 600 | 17.7 | 2.965 | 47 (13) | passionate, direct, eclectic | 47:0.170 48:0.157 46:0.136 45:0.094 44:0.055 |
| Jane Austen | 600 | 17.6 | 2.836 | 47 (13) | passionate, direct, eclectic | 47:0.200 46:0.150 48:0.116 45:0.100 44:0.074 |
| Lewis Carroll | 600 | 16.6 | 2.873 | 47 (13) | passionate, direct, eclectic | 47:0.169 48:0.157 46:0.130 45:0.103 49:0.070 |

The table is honest about how close the three are. All three have Q64 47, King Wen 13. All three get the same temperament from `PersonalityMapper`. Their mean sentence lengths differ by about one word. Their top five sectors are almost the same set, 44 to 48, with Carroll's fifth at 49. The Q64 name and the temperament therefore say nothing about who wrote what.

The differences are in the weights. Austen puts 0.200 of her mass on sector 47 against Chesterton's 0.170 and Carroll's 0.169. Chesterton leans on 48 at 0.157, where Austen has 0.116. Austen has the lowest entropy, 2.836, and so the narrowest spread. Chesterton has the highest, 2.965. `World::compare` for Austen against Chesterton gives similarity 0.980, and `difference_vector` names the sectors that separate them most: 48 (−0.040), 47 (+0.030), 44 (+0.019), 52 (+0.017), 46 (+0.014), 43 (+0.014). The largest single difference is four hundredths of the mass.

Heat flow makes the closeness plainer. `overlap_after` at t = 0.05 gives Austen–Chesterton 0.998 and Austen–Carroll 0.999. At t = 0.2 it gives 0.999 and 1.000. At t = 0.5 both are 1.000. The overlaps say that whatever separates the writers lives at the finest scale of the histogram, in bin-to-bin differences.

Two inputs shape every fingerprint in these tables and are easy to miss. First, stop-word filtering is off in every fingerprint ever computed (docs/79 §2.3). `data/stop_words.txt` is absent, so `PhiConfig::is_stop_word` is always false. Every *the*, *and* and *of* lands in a sector. The corpus's own marker channels, 8 to 31, are those same words: *the*, *and*, *of*, *to*, *a*, *in*, *was*, *that*, *i*, *it*, *he*, *his*, *you*, *not*, *as*, *her*, *with*, *had*, *be*, *for*, *she*, *is*, *have*, *at*. Second, the tokenizer has removed punctuation and case before any sector is read, and `split_sentences` splits on every full stop, so "Mr. Darcy" breaks a sentence.

Function words are strong habits of any writer. They are also shared by every writer of English. Whether fingerprints separate better without them is not measured.

## 7. Centring, and the leak

Two corrections came before full ingestion. Each changed the reader's input without changing the reader.

The first is centring. Every word's sector used to be read on its raw target-side function. From docs/76 onward it is read on that function minus the lexicon mean, `Fingerprint::lexicon_mean`. If every word's function shares a common component, every word's sector is pulled toward the sector of that component, and every writer's histogram piles up there. Subtracting the mean should leave what is particular to each word. docs/papers/15 §11 reports the effect on the `bin/austen` run.

| | before centring | after centring |
|---|---|---|
| attribution overall | 64% | **70%** |
| Austen | 61% | 67% |
| Chesterton | 71% | 79% |
| Carroll | 46% | 40% |
| Austen–Chesterton similarity | 0.991 | 0.980 |

Centring helped the two writers with the most text and cost Carroll six points. The report states the limit in its own words: "centring removes the shared mean but not the shared shape, and a sector that separates authors by more than a few percent is still open." The dominant sectors still sit at 44 to 48 for all three.

The second correction is the leak. In `bin/austen` the lexicon was trained on Austen's training split plus the contrast authors' whole books. Austen was split 80/10/10 by sentence hash with seed 7. Chesterton and Carroll were not split for the lexicon at all. So their held-out paragraphs had already moved the phasors the reader was using. docs/81 fixes this in `bin/persona_ingest`. Every author is split 80/10/10 by sentence hash, seed 7. The shared lexicon is trained on the three training splits only.

| author | books | sentences | ingested from | held out | words |
|---|---|---|---|---|---|
| G. K. Chesterton | 3 | 11,219 | 8,975 | 1,122 | 162,752 |
| Jane Austen | 3 | 17,114 | 13,691 | 1,712 | 247,252 |
| Lewis Carroll | 1 | 1,378 | 1,102 | 138 | 18,305 |

The shared lexicon has 18,496 words, all with relation evidence, 24 marker channels, three passes. Removing the leak with 600-sample personas gives 62%. That is lower than the 70% of the leaky run. The sources do not decompose the drop. A leak that lets held-out text shape the manifold would be expected to flatter the reader, and the later, clean number is the one this paper builds on.

The 71% of full ingestion is measured against 62% on the same clean split, a gain of nine points. Set against the leaky 70% it would look like one.

## 8. Full ingestion and the 71%

Before docs/81, personas were rebuilt on every run from 600 sampled sentences each. `bin/persona_ingest` ingests each persona from its whole training split instead, once, and saves it. The lexicon stays the same clean shared lexicon from section 7. Only the number of sentences behind each persona changes. docs/81 measures both on held-out paragraphs of four sentences.

| persona | sampled 600 | all sentences |
|---|---|---|
| G. K. Chesterton | 73% | 74% |
| Jane Austen | 52% | 65% |
| Lewis Carroll | 57% | **86%** |
| **all** | **62%** | **71%** |

Chance is 33%. The pattern follows the counts in section 7's table. Chesterton goes from 600 sentences to 8,975 and gains one point. His persona was already well estimated. Austen goes from 600 to 13,691 and gains thirteen points. Carroll goes from 600 to 1,102 and gains twenty-nine. docs/81 gives the reason for Carroll: "1,102 sentences is all he has, and sampling 600 of them was throwing away half his evidence." The gain is largest where the sample was the largest share of what exists. The sources offer no further account of the pattern.

The leaky run had Carroll hardest at 40%. Under full ingestion he is easiest at 86%, and Austen is lowest at 65%.

Two details of the count need stating. First, docs/81 says the paragraphs were "150 per author". Carroll has 138 held-out sentences. In fours, 138 sentences make fewer than 150 paragraphs. The reader in `src/bin/persona_ingest/measure.rs` takes at most 150 chunks of four from each author's test split, so Carroll's row rests on the chunks that exist. docs/papers/15, on a different split, counted 35 Carroll paragraphs. Second, the overall 71% is pooled over paragraphs, as the code counts it. It is not the plain mean of 74, 65 and 86, which is higher. Carroll's 86% therefore moves the overall figure less than Chesterton's or Austen's rows do.

A third detail comes from outside the persona docs. The quest work in phiecs found a byte deadband in training. PHIENT_FACTS records that dithered rounding, which frees pairs frozen short of their target, "costs corpus attribution 71% → 65%, so it ships off by default." The 71% is the persona number, and it is the one the dither was measured against.

The whole ingest run is 53 s: 12 s for the lexicon, 8 s for all three personas, the rest for splitting and measuring. The result is written to `data/personas.chroma` and `data/personas.json` through `PersonaStore`.

## 9. Composing as a persona

Until docs/79 the generator had no persona knob. `Generator::with_persona` gave it one, and `bin/persona_eval` measured it. The harness takes 24 prompts: eight held-out sentences from each author, first three words. It generates 30 tokens with trigram decoding on the shared lexicon and judges the composed text alone with `World::match_text`.

| conditioning (sector, bigram) | read as asked | Austen | Chesterton | Carroll |
|---|---|---|---|---|
| none | 33% | 58% | 25% | 17% |
| sector prior 1 | 43% | 79% | 17% | 33% |
| **persona bigrams 1** | **53%** | 92% | 42% | 25% |
| both, 1 and 1 | 44% | 75% | 17% | 42% |
| persona bigrams 2 | 49% | 88% | 25% | 33% |

The first row is the control, and it shows the judge's bias. Unconditioned text reads as Austen 14 of 24 times, whoever was asked for. docs/79 says why: "the judge and the shared tables both lean toward the author with the most sentences." The sector prior alone lifts the total to 43%, but most of the gain is Austen's own row. The persona's bigrams at weight 1 give the best total, 53%, with Austen at 92% and Chesterton at 42%. Adding the sector prior to the bigrams lowers the total to 44%.

docs/79 prints samples for the prompt *chapter 11 the* under bigram conditioning. Chesterton: "little old man with the first time in his own back garden might hear some new cannibal islands said brown". Carroll: "little golden key and hurried her into the garden wall just as well go back to the same thing with you said the queen". The prompt is Austen's, and each continuation still picks up its writer's company.

docs/80 then rebuilt `Impersonator` on the same knob. Before, `Composer::compose` was four fixed frames, "In the beginning, the narrative opens with X and Y.", filled with nouns. After, it generates one 8-token segment per sector on the river's path, each pulled toward that sector's bank word. `impersonate_with` uses sector weight 0 and pair weight 1, the setting `bin/persona_eval` found best.

| | read back as asked | KN perplexity (Austen persona) | distinct |
|---|---|---|---|
| before, four frames | 39% (Carroll every time) | 4,478 | 0.53 |
| **after, generator** | **56%** | **23.6** | 0.86 |

Per persona after: Austen 92%, Carroll 58%, Chesterton 17%. Chesterton was read as Austen 9 of 12 times. Austen's sample begins "whole of the most extraordinary conduct is felt by every thing".

docs/80 names why Chesterton is seldom found. His fingerprint sits between the other two, at similarity 0.98 with Austen, and the judge cannot see him.

## 10. Talking to a persona

`persona::Dialogue` answers a turn in four steps (docs/81 §2).

The first step finds the condition. It is the rarest content word of the question that the lexicon knows. The second step finds what that condition calls for. Over the persona's own 600 most-used content words it takes the one with the strongest directed reading from the condition, `Facet::calls_for`. This is the persona's answer word. The third step generates. Nine candidates come from `Generator`, conditioned on the persona and pulled toward the answer word with `with_goal`. They start from three places: the question's tail, the condition alone, and the condition with what it calls for. The fourth step chooses. The best candidate is ranked by the persona's likelihood of its fingerprint and by the evaluator's quality, less what the dialogue has already said. A candidate repeating more than a quarter of its 4-grams is refused. The lexicon is never written.

`cargo run --release --bin persona_chat` puts the same six questions to all three personas in 17 s. Two replies to one question, as docs/81 prints them (Chesterton answered under *marriage* ⇒ *sudden*):

> **you** · what do you think about marriage
>
> **Jane Austen** · Days that in the very first subject was so much more than she had been long enough to be very well as you know what she had been. <sub>under *marriage* ⇒ *former* · KN panel: Austen, ×4.8</sub>
>
> **Lewis Carroll** · Tone herself up and began to be said the king said alice very much like keeping so close to her sister were happy together said the king and. <sub>under *marriage* ⇒ *melancholy* · KN panel: Carroll, ×9.4</sub>

Two judges read every reply alone: the engine's sector attribution, and `Panel`, one Kneser-Ney trigram model per writer, where the least surprised model wins. The ratios are the panel's margins. Chesterton's reply on money reads ×15.2 for him, Austen's on love ×13.7 for her.

| over 18 replies | read back as the persona who spoke |
|---|---|
| the engine's sector attribution | 44% |
| Kneser-Ney panel, one model per writer | **72%** |
| chance | 33% |

Both are above chance and they disagree. The sector reader calls almost everything Austen, as docs/79 §3 predicted: she has the most text, and the three fingerprints overlap at 0.98. A per-writer language model separates the same replies at 72%. docs/81 draws the conclusion: the engine's own attribution is now the weakest link in the persona chain. The generation is ahead of it.

The replies carry each writer's company: Father Brown and Syme for Chesterton, Alice and the King for Carroll, Emma and Jane for Austen. No name was given in any question.

## 11. What it depends on, and what depends on it

phinet is the parent engine. Its complex `Tensor [b,w,s]`, `Tape` autograd, unitary FFT and `Fno` are checked against PyTorch in 62 of 62 parity tests. `phinet::joint::write_back` is one of the passes that move source angles only (docs/79 §2.2), and the fingerprint reads the target side. The operator is a phinet `Fno`. The one trained on Austen, `data/austen_operator.fno`, reaches 30.7× chance valid and 30.3× test at 50,016 parameters. It is Austen's alone. docs/81 says a shared-lexicon operator "has to be trained before the dialogue can use one." That operator is not built. The 64 hexagram gates are phinet's too, and a fingerprint's Q64 name is a hexagram: 47, King Wen 13, for all three writers.

phical is the geometry of the relation circle. A persona is a histogram over that circle's 64 sectors, and `overlap_after` is heat flow along it. Every fingerprint number in this paper is a phical reading.

phifno 1.0.0 is the spherical operator. Its sphere tie reads the same 64 channels as spherical harmonics with l < 8. No persona measurement in the sources uses it. Whether a writer's sectors separate better on the sphere than on the circle is not measured.

phiton gives light as units: `LightQuantum`, `SpectralBand`, `PhitonColor`. `PersonalityMapper` reads temperament off "the colour bands", but the sources read here do not say that it uses phiton's types.

phinum is a child of phinet in the family. docs/79 records a `src/phinum` of about 1,500 lines as a second Searle stack with no callers. Nothing in the persona path calls it. phiqlm is planned, and nothing of it is built.

phiano owns everything in this paper: `Facet`, `Trainer`, `Generator`, `Persona`, `World`, `PersonaStore`, `Dialogue` and the four binaries `persona_eval`, `impersonate_eval`, `persona_ingest` and `persona_chat`. Two other phiano pieces touch the personas. The helix encodes a lexicon as a DNA-coded tree and PHIENT_FACTS records 18,496 words in 1.9 s with a byte-exact round trip. That word count is the same as the shared persona lexicon's, though the sources read here do not name the file.

phiecs is the entity-component world and the quest. Its synthetic oracle exposed the byte deadband. On the quest, the shipped trainer reads 0.082 MRR and attraction-only training with dithered rounding reads 0.594. The dither cost corpus attribution 71% → 65%, so it ships off. The persona number is the guard that kept a quest fix out of the default path.

phiace is not described in the sources read for this paper. No dependency between it and the personas is recorded there.

What depends on the personas is the demo. docs/79 lists `bin/austen` and `bin/persona_eval` among the paths that can be shown, and docs/80 adds `compose` and `impersonate` with the caveat that they now generate.

## 12. The next experiment

docs/81 §4 names it. The next thing is a discriminative attribution reader, one whose sectors are weighted by how well they separate writers. Today they are weighted by how often each writer uses them. docs/81 names the KN panel as "the yardstick it has to beat." docs/80 names the same need in its own words: a cross-author Kneser-Ney panel, or an attribution reader trained to discriminate. The reader is not built.

The experiment has one change. Today `World::match_text` scores a text by each persona's likelihood of its sector histogram. Every sector counts in proportion to how often the persona uses it. So sectors 44 to 48, which all three writers fill, dominate every score. `difference_vector` already shows where the writers differ: sector 48 at −0.040 and sector 47 at +0.030 between Austen and Chesterton, then 44, 52, 46 and 43 at under two hundredths each. A discriminative reader would weight each sector by how well it separates the writers on the training splits, and leave the likelihood form otherwise alone.

| reader | held-out paragraphs of four | 18 dialogue replies |
|---|---|---|
| `World::match_text`, likelihood (control) | 71% (74 / 65 / 86) | 44% |
| Kneser-Ney panel, one model per writer (yardstick) | not measured | 72% |
| discriminative sector reader | not built | not built |

Everything else stays fixed. The shared lexicon of 18,496 words in `data/personas.chroma`. The personas as ingested in `data/personas.json`. The 80/10/10 split by sentence hash with seed 7. The same held-out paragraphs, and the same 18 replies from `bin/persona_chat`. Dither off, since it moved the control from 71% to 65%.

One gap in the table should be closed first. The KN panel has only been run on the 18 replies. Its score on the held-out paragraphs is not measured. Without it, the yardstick is known on 18 items and unknown on the set where the reader scores 71%. Running `Panel` on the same paragraphs closes it.

Two outcomes matter. If the discriminative reader beats 71% on paragraphs and 44% on replies, it replaces `match_text` as the judge in `bin/persona_eval` and `bin/impersonate_eval`. Those two numbers, 53% and 56%, would then be re-read with a judge that does not lean toward Austen. The conditioning could then be tuned against it, which docs/80 says must wait for a better judge. If it does not beat the control, the sector histogram has reached what it can hold, and the persona's reader should move to pairs, where the panel already works.

A second experiment follows the first: a shared-lexicon operator, so the dialogue can decode through it. Its control is the current replies, decoded through the trigram tables.

## 13. What is wrong or missing

The sources disagree in four places.

First, the attribution baseline. docs/papers/15 gives 70% overall for 600-sample personas. docs/81 gives 62% for 600-sample personas. The lexicons differ: the first held the contrast authors' whole books, the second only training splits. This paper prefers docs/81. The 70% should not be quoted as the control for the 71%.

Second, the paragraph count. docs/81 says 150 paragraphs per author. Carroll's 138 held-out sentences cannot make 150 paragraphs of four. His 86% rests on fewer paragraphs than the other two rows. The sources do not print his count.

Third, the dialogue's conditioning. docs/81 §2 says the nine candidates are conditioned on "sector histogram and its own pair evidence". `Dialogue::open` in `src/persona/dialog.rs` builds its prior with sector weight 0.0 and pair weight 1.0. So the dialogue uses pair evidence alone, the setting docs/80 gives `impersonate_with`. The replies were produced by the code, so this paper follows the code.

Fourth, the decoding numbers cited for the next step. docs/81 cites docs/78 as "10% shared 4-grams against the sampler's 26%". docs/79 gives 26% and 10%. PHIENT_FACTS gives 25.7% for the sampler and 8.7% for the operator. This paper prefers PHIENT_FACTS, the fact sheet the series is bound to.

Beyond disagreements, much is missing or weak.

The representation cannot hold style. The tokenizer deletes punctuation, case, apostrophes and hyphens, and splits on every full stop. docs/81 ends: "no persona can be told by its commas." Stop-word filtering is off in every fingerprint, because `data/stop_words.txt` is absent.

The sector reader is biased toward the writer with the most text. Unconditioned text reads as Austen 14 of 24 times. Chesterton's generated text reads back as his 17% of the time under `bin/impersonate_eval`. The judge of every generation number in this paper is that same reader.

The samples are small. 24 prompts, 12 prompts, 18 replies. One seed per prompt. No variance is reported for any persona number.

Three writers only, and one of them has one book. Austen is missing three novels because gutenberg.org was not reachable from the build machine (docs/papers/15 §8).

Generated text still carries the trigram tables' idiom. docs/81 quotes "to be sure you will not like it". The generator's sentences are stitched trigrams, and the dialogue decodes through those tables, because no shared-lexicon operator exists.

`Impersonator`'s `fit` compared two incompatible sector coordinates until docs/80 (docs/79 §2.4). Every `fit` number from before that fix means nothing.

No human has read the personas' output and judged whether it sounds like the writer. Every judge is a model.

## 14. How it was found

The history runs through five documents.

The first persona measurements in the sources are in docs/papers/15, "A Persona Called Jane Austen", the report that `bin/austen` writes. It built the three personas with `Persona::from_examples` on 600 sampled sentences each and printed their fingerprints, their similarity and their overlap under heat flow. It measured attribution on held-out paragraphs at 70%, 233 of 335. It also switched to a second lexicon, `data/austen_voice.chroma`, for composition, because "a composer reading its tables would borrow from Carroll and Chesterton." At that stage "her voice" meant a lexicon of her own.

docs/76 added centring. Reading every word's sector on its target-side function minus the lexicon mean lifted attribution from 64% to 70% and lowered the Austen–Chesterton similarity from 0.991 to 0.980. docs/papers/15 §11 records the run. docs/80 cites docs/76 for the 0.98 similarity that hides Chesterton.

docs/79 was the demo brief. Two independent read-throughs of all 524 source files produced a ranked list of what was not done, and several items bear on personas: the four-sentence template behind `compose` and `impersonate`, the tokenizer, the absent stop-word file, and `Impersonator`'s mismatched sector coordinates. The brief also made one new measurement. It built `Generator::with_persona`, `with_persona_examples` and `PersonaPrior`, and measured them with `bin/persona_eval`. That gave the 53% from pair evidence and the 43% from sectors. It was the first time one shared lexicon was conditioned on a persona.

docs/80 took the first gap on docs/79's list. `Composer::compose` stopped filling four frames and began generating one goal-driven segment per sector. `Impersonator::impersonate_with` began composing with the persona-conditioned generator at the setting docs/79 had measured best. It stopped training the lexicon on its own output. Its `fit` moved to the fingerprint's own sector coordinates. `bin/impersonate_eval` measured 39% → 56% and KN perplexity 4,478 → 23.6. docs/80 also named the next problem: the judge.

docs/81 did two things. It found and removed the leak by training the shared lexicon on training splits only, and it replaced 600-sample personas with full ingestion through `bin/persona_ingest` and `PersonaStore`. That took attribution from 62% to 71%. It then built `persona::Dialogue` and `bin/persona_chat`, and introduced the per-writer Kneser-Ney `Panel` as a second judge. The panel read the replies at 72% where the sector reader read 44%. The conclusion moved the project's attention from generation to attribution.

Each step used the previous step's weakest number as its target. docs/79 targeted the template. docs/80 found the judge. docs/81 found the leak, then found the judge again. The PHIENT_FACTS line on dither, 71% → 65%, shows the persona number then serving as a guard for work elsewhere.

## 15. What remains

The work that remains falls into four groups, in the order the sources give them.

The reader comes first. The discriminative attribution reader of section 12 is not built. The Kneser-Ney panel has not been run on held-out paragraphs, so the yardstick is known only on 18 replies. Until a reader exists that does not lean toward the writer with the most text, the generation numbers of 53% and 56% are measured by a biased judge. docs/80 says the conditioning cannot be tuned against the judge before the judge is fixed.

The representation comes second. `Tokenizer::tokenize` must keep punctuation and case before any persona can be told by its commas. `split_sentences` must stop splitting on every full stop. `data/stop_words.txt` is absent, and whether fingerprints read better with or without stop words is not measured. docs/79 §2.2 also records that only one of six objectives writes the target side of a word, `Trainer::couple_relations`. The fingerprint reads the target side. Every pass that moves only source angles leaves the persona's reading behind it.

Decoding comes third. The replies carry the trigram tables' idiom. Decoding through an operator is known to lower shared 4-grams, 8.7% against the sampler's 25.7% in PHIENT_FACTS. The operator in `data/austen_operator.fno` is Austen's. A shared-lexicon operator for three writers is not built, and without it the dialogue cannot decode through one.

Scale comes last. Three writers is a small world. Austen is missing three of her novels. Carroll has one book and 138 held-out sentences. No persona number has a variance. docs/79 names what larger data would need: a streaming corpus reader, sampled-softmax training, the target-side write in every objective, a tokenizer that keeps punctuation and case, and one lexicon format that carries several personas as conditions. The last item is the one docs/81 began. `data/personas.chroma` and `data/personas.json` hold one lexicon and three personas.

Some things are not claimed. No persona has been built on any corpus other than these three writers' seven books. No human judgment of the output exists. No persona measurement uses the sphere, phiton or phinum. No test on Phi-4 or any external model has been run for personas.

The core of the idea is in place. One shared lexicon. Personas as counts over it, stored once and loaded read-only. Attribution from likelihood at 71% on held-out paragraphs. Generation under the persona's own pairs. A dialogue that finds the persona's answer word by the directed reading X ⇒ Y and generates toward it. What remains is a judge that can see the differences the pair evidence already makes, and a representation that can hold what a writer does with punctuation.

## 16. Sources

Every number in this paper comes from the files below. The six documents first listed are the sources this paper was assigned. No source file was changed.

`/home/claude/phiano/docs/papers/phient/BRIEF.md`. The rules for this series: front matter, the paradigm paragraph, sixteen sections, the arc, and the voice.

`/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md`. The paradigm paragraph opening this paper. The family: phinet, phical, phifno 1.0.0, phiton, phinum, phiqlm (planned), phiano, phiecs. The 62/62 parity. Persona attribution at 71% (74/65/86). The dialogue at 44% and 72%. The quest's 0.082 and 0.594 MRR. The dither's cost of 71% → 65%. The helix at 18,496 words in 1.9 s. The operator at 30.7× and 30.3×, and the shared 4-gram rates 25.7% and 8.7%.

`/home/claude/phiano/docs/79_demo_brief.md`. The extended paradigm sentence on personas and corpora. The list of what is not done: the four-sentence template, the tokenizer, the single target-side writer, the absent stop-word file, `Impersonator`'s mismatched sectors, `src/phinum` without callers. §3: `Generator::with_persona`, `PersonaPrior`, the sector prior clipped ±2, the persona bigram evidence, and `bin/persona_eval` with its five arms, from 33% to 53%, the samples for *chapter 11 the*. §4: the persona's parameters and the list of what large-data learning needs.

`/home/claude/phiano/docs/80_impersonation_generated.md`. `Composer::compose` from four frames to generated segments. `Impersonator::impersonate_with` at sector weight 0 and pair weight 1. `bin/impersonate_eval`: 39% → 56%, KN perplexity 4,478 → 23.6, distinct 0.53 → 0.86, per persona 92%, 58%, 17%. The call for a better judge.

`/home/claude/phiano/docs/81_persona_dialogue.md`, sections 1 and 4, with the dialogue description, samples and judge table of sections 2 and 3 read for context. §1: the leak, `bin/persona_ingest`, the split table, the shared lexicon of 18,496 words, the timings, and attribution from 62% to 71%. §4: the discriminative reader, the KN panel as yardstick, the trigram idiom, the missing shared-lexicon operator, and the manifold without punctuation.

`/home/claude/phiano/docs/papers/15_JANE_AUSTEN_PERSONA.md`. The corpus table. The three fingerprints from 600 sampled sentences, with entropy, Q64, temperament and dominant sectors. Similarity 0.980 and the separating sectors. Overlap under heat flow. Attribution at 70%, 233 of 335. The voice lexicon of 10,501 words. §8's notes on the missing novels. §11 on centring, 64% → 70%.

Code read to name types and functions exactly, with no number taken from it except where noted in the text: `/home/claude/phiano/src/generate/persona.rs` for `PersonaPrior`, `PERSONA_CLIP`, `persona_bonus` and the two `with_persona` constructors. `/home/claude/phiano/src/persona/dialog.rs` for `Dialogue::open` and its prior's weights. `/home/claude/phiano/src/bin/persona_ingest/measure.rs` for how attribution counts paragraphs. The listings of `src/persona` for `Fingerprint`, `World`, `MatchResult` and `PersonaStore`.

The existing papers in `/home/claude/phiano/docs/papers/phient/`, numbers 1 to 8, were read for voice only.
