---
id: jane-austen-persona
title: "A Persona Called Jane Austen"
description: What the engine learns from three novels, and what it can compose in her voice — short and long, measured.
sidebar_position: 15
badge: report
---

# A Persona Called Jane Austen

**Phiano Project** · `bin/austen` · generated 2026-09-14

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. A persona is which conditions a writer puts words under, aggregated — and Jane Austen is three novels' worth of those conditions.

## 1. The corpus

| text | sentences (4–60 words) | words |
|---|---|---|
| Austen — emma | 8782 | 142611 |
| Austen — persuasion | 3256 | 66000 |
| Austen — sense and sensibility | 5076 | 100771 |
| contrast — carroll-alice | 1378 | 23208 |
| contrast — chesterton-ball | 4350 | 78433 |
| contrast — chesterton-brown | 3517 | 69155 |
| contrast — chesterton-thursday | 3352 | 56588 |

Project Gutenberg editions via the NLTK `gutenberg` package (`data/corpora/austen/SOURCES.md`). Austen split 80/10/10 by sentence hash (seed 7); the lexicon is trained on the Austen training split plus every contrast author, three passes, two-sided (format v6): **19649 words, 19649 with relation evidence, 24 marker channels**, saved and reloaded as `data/austen.chroma`. Kneser-Ney trigram on the Austen training split — the judge for everything below — reads **187.5** on Austen's held-out sentences.

The markers the corpus chose as relation channels 8–31: *the*, *and*, *of*, *to*, *a*, *in*, *was*, *that*, *i*, *it*, *he*, *his*, *you*, *not*, *as*, *her*, *with*, *had*, *be*, *for*, *she*, *is*, *have*, *at*.

## 2. The persona

`Persona::from_examples` on 600 sampled Austen sentences (and 600 each for the rival personas). A persona is a histogram over the 64 sectors of the relation circle; its entropy is its breadth; `PersonalityMapper` reads temperament off the colour bands; its Q64 name is the sector of its centroid.

| persona | samples | mean length | entropy | Q64 (King Wen) | temperament | dominant sectors |
|---|---|---|---|---|---|---|
| G. K. Chesterton | 600 | 17.7 | 2.965 | 47 (13) | passionate, direct, eclectic | 47:0.170 48:0.157 46:0.136 45:0.094 44:0.055 |
| Jane Austen | 600 | 17.6 | 2.836 | 47 (13) | passionate, direct, eclectic | 47:0.200 46:0.150 48:0.116 45:0.100 44:0.074 |
| Lewis Carroll | 600 | 16.6 | 2.873 | 47 (13) | passionate, direct, eclectic | 47:0.169 48:0.157 46:0.130 45:0.103 49:0.070 |

Austen vs Chesterton: similarity 0.980; the sectors that most separate them: 48 (-0.040), 47 (+0.030), 44 (+0.019), 52 (+0.017), 46 (+0.014), 43 (+0.014).
Overlap after heat flow t = 0.05: Austen–Chesterton 0.998, Austen–Carroll 0.999.
Overlap after heat flow t = 0.2: Austen–Chesterton 0.999, Austen–Carroll 1.000.
Overlap after heat flow t = 0.5: Austen–Chesterton 1.000, Austen–Carroll 1.000.

## 3. Who wrote this? Attribution on held-out paragraphs

Held-out sentences grouped into paragraphs of four; `World::match_text` picks the persona with the highest likelihood. Chance is one in three.

| true author | paragraphs | picked Austen | picked Chesterton | picked Carroll | accuracy | mean confidence |
|---|---|---|---|---|---|---|
| Jane Austen | 150 | 101 | 34 | 15 | 67% | 0.00 |
| G. K. Chesterton | 150 | 20 | 118 | 12 | 79% | 0.00 |
| Lewis Carroll | 35 | 7 | 14 | 14 | 40% | 0.00 |

Overall 70% (233 of 335).

> From here on the machine reads a second lexicon, **`data/austen_voice.chroma`**, trained on Austen's training split alone (10501 words, 10501 with relation evidence) — the attribution lexicon above holds every author, and a composer reading its tables would borrow from Carroll and Chesterton.

## 4. The operator, on Austen

A directed Fourier operator (8,360 parameters, hinge, 4 epochs, early-stopped) trained on Austen's training split, ranking the next word among the 1228 words seen ≥ 20 times. Chance MRR 0.0063. The lexicon's own directed reading, no operator: **0.0492 (7.9× chance)**; with the operator: **0.1005 (16.1× chance)**. Saved as `data/austen_operator.fno`.

## 5. The learning machine, on Austen

The generator's candidate scorer fit to Austen (validation split), evaluated on her test split: perplexity among the n-gram's candidates — counts only **12.94**, learned **10.95**. The weights are the value audit for this author:

| feature | weight |
|---|---|
| ln_count | +1.445 |
| phase_align | +0.103 |
| resonance | -0.009 |
| function_word | +0.215 |
| repeat | +0.337 |
| operator_cos | +1.749 |
| relational | +0.671 |
| relational_pairs | +1.642 |

## 6. What she composes

Continuations of Austen-shaped prompts (none of which is a sentence of hers): **short** (12 tokens) and **long** (60 tokens), temperature 0.3, one fixed seed per prompt across arms. Arms: the learned scorer alone; the learned scorer with the operator; and the operator with **I Ching novelty** — every fourth token its prediction passes through hexagram ䷘ (id 0b101010, King Wen 25 *Wú Wàng*, "Innocence"), which keeps octaves 1, 3 and 5 of the prediction's spectrum and drops the rest. Each arm is judged by the Kneser-Ney model of Austen (lower = more like her), by the engine's evaluator, and by its distinct-token ratio.

### Short compositions (12 tokens)

**learned scorer**

- *It is a truth universally* → Mistaken and she was the very person for you to feel that.  
  <small>KN 27 · eval 0.66 · distinct 1.00</small>
- *Emma Woodhouse handsome clever and rich* → With me and in all the rest of the house for three.  
  <small>KN 17 · eval 0.71 · distinct 0.92</small>
- *Captain Wentworth had* → Been so much of her own heart was in the way to.  
  <small>KN 14 · eval 0.61 · distinct 1.00</small>
- *Elinor and Marianne* → Who had been the death of my own way and only wanted.  
  <small>KN 12 · eval 0.66 · distinct 1.00</small>
- *Mr Knightley said* → Emma as she could not help saying what may happen to be.  
  <small>KN 26 · eval 0.74 · distinct 1.00</small>
- *The evening passed* → Off in such an idea of the room and elinor was obliged.  
  <small>KN 19 · eval 0.67 · distinct 1.00</small>
- *She could not help* → Laughing as she could not help thinking it the least mind in.  
  <small>KN 12 · eval 0.72 · distinct 1.00</small>
- *Her mother was* → So very much to be in town and at the time of.  
  <small>KN 14 · eval 0.71 · distinct 1.00</small>

**learned + operator**

- *It is a truth universally* → And they were all three in so large an amount as to.  
  <small>KN 29 · eval 0.66 · distinct 1.00</small>
- *Emma Woodhouse handsome clever and rich* → With all the rest of the party to be in such and.  
  <small>KN 17 · eval 0.71 · distinct 0.92</small>
- *Captain Wentworth had* → Been so much the other day as they were all at once.  
  <small>KN 11 · eval 0.67 · distinct 1.00</small>
- *Elinor and Marianne* → She could not have believed it to be in town and is.  
  <small>KN 11 · eval 0.74 · distinct 1.00</small>
- *Mr Knightley said* → Emma as to the other day you know that he was not.  
  <small>KN 36 · eval 0.70 · distinct 1.00</small>
- *The evening passed* → Off in the world and that he had seen so much of.  
  <small>KN 16 · eval 0.71 · distinct 1.00</small>
- *She could not help* → Laughing as she could not help thinking it the change of subject.  
  <small>KN 9 · eval 0.72 · distinct 1.00</small>
- *Her mother was* → So very much to be the case you know and all that.  
  <small>KN 15 · eval 0.67 · distinct 1.00</small>

**learned + operator + I Ching novelty**

- *It is a truth universally* → And they were all three in so respectable and happy was she.  
  <small>KN 25 · eval 0.72 · distinct 0.92</small>
- *Emma Woodhouse handsome clever and rich* → With all the rest of the party to be in such and.  
  <small>KN 17 · eval 0.71 · distinct 0.92</small>
- *Captain Wentworth had* → Been so much the other day as they were all at once.  
  <small>KN 11 · eval 0.67 · distinct 1.00</small>
- *Elinor and Marianne* → She could not have believed it to be in town and is.  
  <small>KN 11 · eval 0.74 · distinct 1.00</small>
- *Mr Knightley said* → Emma as to the other day you know that he was not.  
  <small>KN 36 · eval 0.70 · distinct 1.00</small>
- *The evening passed* → Off in the world and that he had seen so much of.  
  <small>KN 16 · eval 0.71 · distinct 1.00</small>
- *She could not help* → Laughing as she could not help thinking it the change of subject.  
  <small>KN 9 · eval 0.72 · distinct 1.00</small>
- *Her mother was* → So very much to be the case you know that she had.  
  <small>KN 16 · eval 0.65 · distinct 1.00</small>

### Long compositions (60 tokens)

**learned scorer**

- *It is a truth universally* → Mistaken and she was the very person for you to feel that he should be very glad indeed but his mind was in the same time which yet kept them in an instant but anne could not be very glad to see her and she had never been in the world of edwards mother and so you are very kind.  
  <small>KN 11 · eval 0.66 · distinct 0.73</small>
- *Emma Woodhouse handsome clever and rich* → With me and in all the rest of the house for three months but now she could have been more amiable woman he had not the smallest degree wavering dubious or she would be very glad to see her and made every body was in the least mind for it is so very much to be done and as to.  
  <small>KN 10 · eval 0.69 · distinct 0.80</small>
- *Captain Wentworth had* → Been so much of her own heart was in the way to as maple grove and by that shyness which too often for emmas attention it was to be done and as soon however as you may not have supposed any thing to be in town and at the time they were all cousins or something of consequence so shocked.  
  <small>KN 11 · eval 0.70 · distinct 0.80</small>
- *Elinor and Marianne* → Who had been the death of my own way and only wanted to know what you are not very likely that the world for in spite of all her former style of living at hartfield and miss bates was quiet she found herself by this time of life and from us for we know nothing of the other day as.  
  <small>KN 11 · eval 0.69 · distinct 0.83</small>
- *Mr Knightley said* → Emma as she could not help saying what may happen to be the very next morning still to lower herself in spite of his own house and with this poor sick child is always so attentive to her sister and the first time these two she could not be able to give her the very first and second edtions volume.  
  <small>KN 12 · eval 0.60 · distinct 0.77</small>
- *The evening passed* → Off in such an idea of the room and elinor was obliged to you for any one who had been the death of mrs clay was in his own house too there was no longer what they are not going to be sure you will find that she was at last with some of the party to box hill when.  
  <small>KN 11 · eval 0.65 · distinct 0.82</small>
- *She could not help* → Laughing as she could not help thinking it the least mind in mrs smith should have been so very much obliged to you for the sake of your sister is worse than when she saw it all to her and the young man had made him happy in their way to her and she was not very likely you should.  
  <small>KN 10 · eval 0.69 · distinct 0.77</small>
- *Her mother was* → So very much to be in town and at the time of life which must have been very much to do with it never can be no doubt of its being very much to do with it never was in the world and mr musgrove has been to me as having any such thing replied emma and harriet smith the.  
  <small>KN 10 · eval 0.69 · distinct 0.68</small>

**learned + operator**

- *It is a truth universally* → And they were all three in so large an amount as to the other day as it was all that could be no doubt of his own way but she was in her hand to you for the world of your sister is an excellent young man who had been so long since you were to be sure of being.  
  <small>KN 13 · eval 0.65 · distinct 0.77</small>
- *Emma Woodhouse handsome clever and rich* → With all the rest of the party to be in such and your sister is to be of any thing but the feelings of others and it was to her sister had seen you only as the most natural thing in the same time and it was all that she had no other answer than an allowable use of the.  
  <small>KN 11 · eval 0.65 · distinct 0.67</small>
- *Captain Wentworth had* → Been so much the other day as they were all at once and his wife or of any consequence in the world to be sure said she that you had better not have believed it to be so very great on his side for the present case it took up ten minutes had its awkwardness and its sorrows yet to.  
  <small>KN 10 · eval 0.63 · distinct 0.82</small>
- *Elinor and Marianne* → She could not have believed it to be in town and is he like what one has always so much of the room and elinor was very far from it for her to be the same time related and they had been used to think of it in the same time and there was no longer than mr elliot had.  
  <small>KN 11 · eval 0.71 · distinct 0.73</small>
- *Mr Knightley said* → Emma as to the other day you know that he was not only been angry and drawing him as to the point of harriets happiness with any other and therefore it was to be sure he must have been so very much in the world and as to be made for him was it not at all in the world.  
  <small>KN 14 · eval 0.71 · distinct 0.65</small>
- *The evening passed* → Off in the world and that he had seen so much of her own room again this evening for miss marianne said not another word to say that he had been used to be the last two minutes after her to be sure said she that you should think of it and as soon the carriage with her father had.  
  <small>KN 12 · eval 0.67 · distinct 0.75</small>
- *She could not help* → Laughing as she could not help thinking it the change of subject or animation and her own heart was in these words he almost immediately they think the very first evening of this day at hartfield and to be in town the whole story long ago for her own heart was not to be sure it is very natural for.  
  <small>KN 11 · eval 0.70 · distinct 0.75</small>
- *Her mother was* → So very much to be the case you know and all that was not at all to her sister with uneasiness of her own heart was in the same time as she could not help saying what may happen to be in the course of my own and her mother was an old friend to another woman at the time.  
  <small>KN 11 · eval 0.63 · distinct 0.68</small>

**learned + operator + I Ching novelty**

- *It is a truth universally* → And they were all three in so respectable and happy was she to be sure of it with the most extraordinary sort of thing that is all this and such as it was so very much to be the same time in such an idea of that sort of thing which her friend and to be the very first subject.  
  <small>KN 14 · eval 0.72 · distinct 0.63</small>
- *Emma Woodhouse handsome clever and rich* → With all the rest of the party to be in such and your sister is to be of any thing but the feelings of others and she had no idea of her own sister very well of the day before and was in the world to be done for them the better of it and as her own heart was.  
  <small>KN 12 · eval 0.67 · distinct 0.63</small>
- *Captain Wentworth had* → Been so much the other day as they were all at once and he had not thought of any thing to be sure said she and mr elliot was as much in the evening with her for such an opportunity of being so very much in the world and never parted with it at all to her for she was.  
  <small>KN 11 · eval 0.69 · distinct 0.70</small>
- *Elinor and Marianne* → She could not have believed it to be in town and is he like what one has left behind the house was never taken good care of yourself as well have it from the first to all that was what made her no longer than mr elliot and be as you know the summer in that respect how unlike dear.  
  <small>KN 12 · eval 0.72 · distinct 0.80</small>
- *Mr Knightley said* → Emma as to the other day you know that he was not only been angry and drawing him as to the point of harriets happiness with any other and therefore it was to be sure he must have been so very much in the world and as to be made for him was no longer than yourself can shew her.  
  <small>KN 15 · eval 0.70 · distinct 0.73</small>
- *The evening passed* → Off in the world and that he had seen so much of her own room again this evening for miss marianne and in the same time that he was not only been angry with her at the same time that she was not only been angry and how did you ever see the difference of her own heart was in.  
  <small>KN 12 · eval 0.66 · distinct 0.63</small>
- *She could not help* → Laughing as she could not help thinking it the change of subject or animation and her own heart was in company with captain wentworth had been so long as she could not help saying what may comparatively be the very first subject on which you will find me as having any such thing replied mr elliot was not the smallest.  
  <small>KN 10 · eval 0.68 · distinct 0.82</small>
- *Her mother was* → So very much to be the case you know that she had been used to think of it and as far from being her friend was not the smallest doubt of its being very much to be in town and she was not only more pleasing it became for the sake of her own heart she is not to give.  
  <small>KN 11 · eval 0.68 · distinct 0.68</small>

### Summary

| arm | n | KN perplexity (geo. mean) | evaluator | distinct |
|---|---|---|---|---|
| short · learned scorer | 8 | 16.9 | 0.685 | 0.990 |
| short · learned + operator | 8 | 16.2 | 0.697 | 0.990 |
| short · learned + operator + I Ching novelty | 8 | 16.0 | 0.701 | 0.979 |
| long · learned scorer | 8 | 10.9 | 0.672 | 0.775 |
| long · learned + operator | 8 | 11.6 | 0.669 | 0.727 |
| long · learned + operator + I Ching novelty | 8 | 12.2 | 0.689 | 0.704 |

For scale: Austen's own held-out sentences read 187.5 under the same judge; the Rust Book generator read 12–14 on its corpus. A number far below the corpus's own is a copy; far above it is not her.

## 7. The sixty-four operations

`phinet::hexagram`: a hexagram's six lines gate the six octaves of a function's spectrum — line *j* yang passes octave *j*. ䷀ (111111) is the identity, ䷁ (000000) keeps only the mean; `changing_lines` walks between them. The hexagram a word *is* is its centroid sector (Q64). Readings of five of Austen's words (octave energy, bottom line first):

| word | Q64 id | King Wen | lines (octaves 0–5) | novelty of ䷘ |
|---|---|---|---|---|
| elinor | 38 | 17 | 0.06 0.10 0.08 0.19 0.56 0.02 | 0.434 |
| emma | 39 | 25 | 0.09 0.14 0.07 0.21 0.49 0.02 | 0.380 |
| love | 33 | 27 | 0.06 0.06 0.18 0.18 0.51 0.01 | 0.468 |
| marriage | 15 | 33 | 0.02 0.02 0.23 0.36 0.35 0.03 | 0.360 |
| said | 43 | 37 | 0.05 0.07 0.17 0.21 0.49 0.01 | 0.444 |
| the | 42 | 63 | 0.03 0.09 0.16 0.21 0.49 0.01 | 0.414 |

What the gates are for is §6's third arm: novelty *on demand* in composition, exact, reversible, and named. What they are not (measured on the Rust Book, `bin/hexagram_eval`): a way to predict the next word's hexagram — the operator's predicted hexagram matches the true next word's 0.9% of the time, the King Wen successor 1.1%, both at or under chance (1.6%), while "the most frequent hexagram, always" gets 12.8% because centroid sectors are skewed. The sixty-four operations are a keyboard, not an oracle.

## 8. Honest notes

- Three of the six novels; gutenberg.org is not reachable from the build machine, so *Pride and Prejudice*, *Mansfield Park* and *Northanger Abbey* are absent (`SOURCES.md`).
- The generator is a trigram sampler with a learned, relationally-featured scorer; the judge is a trigram model of the same author. Numbers near the corpus's own perplexity mean "as trigram-like as she is", not "as good as she is".
- One seed per prompt; eight prompts; treat differences under ~10% of perplexity as noise.
- Long compositions read ~11 under the judge while her own sentences read ~190: they are her trigrams stitched end to end, not her sentences. The distinct-token ratio (0.70–0.78 at 60 tokens) shows the stitching — "to be sure", "her own heart" — and is the number to move next.
- Until this run a run of four function words ended a composition outright (a hard stop in `generate.rs`), which is why earlier long arms came back three words long; the run now demands a content word instead (`FUNCTION_STREAK_LIMIT`).
- Total run 249s.


## 9. The Liskov readings on her

Barbara Liskov (as Barbara Huberman) wrote her 1968 Stanford thesis on a program to play chess end games — the killer heuristic comes from it — then built the Venus operating system at MITRE (1972) as levels of abstraction each testable against the one below, and in 1987 stated the substitution principle: a subtype may stand in wherever its supertype is expected without changing what the program does (record and mapping in `data/corpora/liskov/notes.md`). Two of these are now readings in the machine, measured on the Austen voice lexicon (`bin/liskov`).

**Substitution.** A word *b* may stand in for *a* wherever every relation the context demands of *a* still holds of *b*. `Facet::relational_fit(toks, i, cand)` reads a candidate at a position against every neighbour in the relation window, each pair under the condition the sentence puts between them, on the side the direction demands. This is not similarity — `similarity` asks whether two words present the same sides, a point-like reading — and the measurement separates them: put the best stand-in (never the original) into her sentence and a Kneser-Ney model of Austen reads 238 for the Liskov stand-in against 254 for the most similar word, which is what a random pool word reads (254). The point-like reading is indistinguishable from chance at substitution; the relational one is not. On cloze the relational fit alone (4.1× chance) trails word frequency (5.8×), and the learned scorer's own trade between the two (its `ln_count` and `relational` weights, nothing refit) reads 6.1× — the store and the Difference Engine again, and the cards deciding the mix.

**The end game.** A move is judged by the mate it moves toward. `Generator::with_goal(word, weight)` adds `weight · calls_for(cand, goal)` to every candidate's logit until the goal is said, where `calls_for` is the candidate's source side read against the goal's target side over the evidenced offset channels. Forty held-out prompts, each with a goal word its sentence reaches later, thirty tokens: no pull reaches the goal 2/40 in 23.5 tokens; pull 2 reaches 6/40; pull 4 reaches 5/40 in 8.8 tokens at a perplexity cost of 12.4 → 13.3. The pull works and is small — a trigram sampler can only be steered among the continuations it has — and the honest reading is that goal-directed composition needs the sampler replaced, not the pull raised. The killer heuristic (try first the candidate that won a sibling hypothesis) belongs in the beam and is not implemented.

**Cloze** — recover a masked content word from a pool of 622 (seen ≥ 40 times), 200 held-out positions, chance MRR 0.0113:

| reading | MRR | × chance |
|---|---|---|
| most frequent word | 0.0655 | 5.8× |
| relational, left context only (`relational_score`) | 0.0221 | 2.0× |
| relational, both sides (`relational_fit`) | 0.0463 | 4.1× |
| `relational_fit` × ln(1 + pairs) | 0.0527 | 4.7× |
| learned trade: w_count·ln count + w_relational·fit (weights from `learned_austen.json`) | 0.0691 | 6.1× |

**Substitution** — the best stand-in (never the original) put into her sentence, judged by Kneser-Ney (geo. mean over 200 sentences; lower = she minds less):

| sentence | perplexity |
|---|---|
| as written | 173.0 |
| `relational_fit` stand-in (Liskov: every demanded relation holds) | 237.9 |
| most similar word (`similarity`: the same sides, a point-like reading) | 253.9 |
| random pool word | 254.3 |

- time → **benwick** (fit) / impossible (similar) — *chapter 11 the time now approached for lady russells return the day was even fixed and anne being engaged to join her as soon as she was resettled was looking forward to an early removal to kellynch and beginning to think how her own comfort was likely to be affected by it*
- quite → **before** (fit) / like (similar) — *i was quite surprizedvery glad i am sure but really quite surprized*
- seeing → **anything** (fit) / imagine (similar) — *the expectation of seeing her however was enough to make her interested in the engagement for though she could now meet edwards mother without that strong anxiety which had once promised to attend such an introduction though she could now see her with perfect indifference as to her opinion of herself her desire of being in company with mrs*
- hardly → **might** (fit) / elinors (similar) — *she had hardly been able to speak a word and every look and action had shewn how deeply she was suffering from consciousness*
- must → **feel** (fit) / shall (similar) — *weston laughing affectedly i must protest against that*
- probably → **lived** (fit) / point (similar) — *you have probably entirely forgotten a conversation it is not to be supposed that it could make any impression on youa conversation between us one evening at barton parkit was the evening of a dancein which i alluded to a lady i had once known as resembling in some measure your sister marianne*
- good → **mrs** (fit) / thousand (similar) — *her recent good offices by anne had been enough in themselves and their marriage instead of depriving her of one friend secured her two*
- two → **each** (fit) / power (similar) — *two maids and two men indeed*

**Endgame** — 40 prompts (three words of a held-out sentence) with a goal word that sentence reaches later; 30 tokens, temperature 0.3, learned scorer; the pull is `weight · calls_for(cand, goal)` added to every logit until the goal is said:

| pull weight | reached | tokens to goal (mean, when reached) | KN perplexity | distinct |
|---|---|---|---|---|
| 0 | 2/40 | 23.5 | 12.4 | 0.896 |
| 0.5 | 5/40 | 17.0 | 12.7 | 0.910 |
| 1 | 4/40 | 13.0 | 12.6 | 0.907 |
| 2 | 6/40 | 16.2 | 12.9 | 0.908 |
| 4 | 5/40 | 8.8 | 13.3 | 0.893 |

- pull 0 · *chapter 11 the* → lady: Party to the house and they were all that she had been long enough to make the best of it and as soon have done for himself in at the.
- pull 0 · *i was quite* → sure: Of your sister is an excellent young man and she was not only so much to be in the same time and he had not elinor appear to have been.
- pull 0 · *the expectation of* → enough: What had passed away in such an idea of the house and he was not very likely to be in town but she had never been able to give her.
- pull 2 · *chapter 11 the* → lady: Party to the other day you know and when he was gone this was too much to be in the same time it is so very kind and he was.
- pull 2 · *i was quite* → sure: Of your sister is to be in love with him and his wife but not at all to go away again it was the first place been so much in.
- pull 2 · *the expectation of* → enough: Mr elliot was in the world and as to be done without any affection and almost made me quite by chance they have no doubt of its being very much.


## 10. Decoding from the operator

Every composition above came from the trigram sampler with the operator and the relational reading re-weighting its candidates — the ceiling named in docs/74–75. `Generator::with_decode(Decode::Operator { k })` lifts that: the candidate set is the `k` words of the function space nearest the operator's prediction, and the learned scorer chooses among them with the n-gram count as a feature rather than the gate; `Decode::Union` offers both sets. Forty held-out prompts, thirty tokens, `bin/decode_eval`:

| arm | n | KN perplexity (geo. mean) | evaluator | distinct | mean tokens | novel bigrams |
|---|---|---|---|---|---|---|
| trigram sampler (k=32) | 40 | 13.6 | 0.679 | 0.867 | 30.0 | 0.0% |
| operator nearest-k (k=32) | 40 | 97.2 | 0.682 | 0.802 | 30.0 | 0.2% |
| union (k=32) | 40 | 86.2 | 0.678 | 0.818 | 30.0 | 0.2% |

Read against her own sentences (187.5 under this judge): the trigram sampler at 13.6 is stitching her trigrams; decoding from the operator at 97 is composing — the text is no longer a copy — and it is not yet her: the samples read as function-word runs, the evaluator is level, and almost every bigram is still one the tables have seen (0.2% novel), because the scorer's `ln_count` weight keeps choosing table-legal words among the operator's nearest. The ceiling has moved from the sampler to the operator's own ranking (16.1× chance, MRR ≈ 0.10): with a better operator the same decoder writes better text, which is the measurement to run next.

## 11. Persona sectors, centred

This run reads every word's sector on its target-side function *minus the lexicon mean* (`Fingerprint::lexicon_mean`): attribution rose from 64% to **70%** (Austen 61 → 67%, Chesterton 71 → 79%, Carroll 46 → 40%) and the Austen–Chesterton similarity fell from 0.991 to 0.980. The dominant sectors still sit at 44–48 for all three authors: centring removes the shared mean but not the shared shape, and a sector that separates authors by more than a few percent is still open.
