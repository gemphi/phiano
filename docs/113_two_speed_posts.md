# 113 · Two-speed posts: the counts student catches bag of words on Phi-4, and keeps the rules

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Where docs/112 left it.** Weights per aspect helped a little on Phi-4 but hurt the rule game. Bag of words still led on Phi-4, because Phi-4's act is a tendency carried by particular words (`slab` and `pillar` towards `record`, colour-words towards `bring`, docs/111).

**Trial** (`wordposts.rs`): the two speeds of paper 19, applied to the post.

- **Fast speed (D2 unchanged).** A word holds a post on an aspect if its counts move the answer by τ = 0.5 nats: weight 1 with a post, 0 without. This learns a word from one pointing.
- **Slow speed.** A word told at least 8 times under the aspect may earn its own weight there. The weight is learned from how well its move predicts answers it was not counted on (four inner folds of the training set). A penalty (λ = 0.05) pulls it back toward its D2 post.
- **The fresh-word rule** (added after the first run, see below). While a word still at the fast speed holds a post in the utterance, the whole aspect is read by D2. Habits learned where that word was never said are not trusted against it.
- The condition keeps weight 1 (D4). With an infinite threshold the learner is exactly D2: the table's last row reproduces τ = 0.5 to the digit, which checks the code.

The threshold of 8 sightings was fixed in the code before any run. The 4 / 32 / ∞ rows are the sweep around it.

## Students of Phi-4 (5 folds by call)

| student | t0: exact | t0: act | sampled: exact | sampled: act |
|---|---|---|---|---|
| bag of words + softmax | **70.5%** | 77.6% | **54.2%** | **67.5%** |
| counts, D2 (τ = 0.5) | 60.3% | 66.4% | 46.6% | 61.2% |
| counts, τ per aspect (docs/111) | 65.1% | 70.2% | 49.3% | 62.9% |
| counts, weights per aspect (docs/112) | 66.4% | 73.8% | 50.1% | 62.3% |
| two-speed posts, own weight after 4 | 70.2% | 78.1% | 46.9% | 64.0% |
| **two-speed posts, own weight after 8** | **70.2%** | **79.9%** | 47.2% | 64.0% |
| two-speed posts, own weight after 32 | 62.3% | 68.4% | 46.6% | 61.2% |
| two-speed posts, never (= D2) | 60.3% | 66.4% | 46.6% | 61.2% |

The words that moved furthest from their D2 post (temperature-0 run, last fold) include `pillar`@act +1.6 and `slab`@act +1.4. These are Phi-4's `record` tendency, found per word, which docs/111 read off by hand.

## The rule game (seed 7), same learner

| test | counts, D2 | two-speed posts |
|---|---|---|
| seen / combinations / report / no name | 100 / 100 / 100 / 100% | 100 / 100 / 100 / 100% |
| sepia: 1 / 3 pointings / 1 + `colour` / 3 + `colour` | 26 / 26 / 100 / 100% | 26 / 26 / 100 / 100% |
| training curve: 20 / 40 / 80 / 160 / 320 / 640 | 19 / 38 / 51 / 70 / 96.6 / 98.7% | 19 / 38 / 51 / 70 / **98.8 / 98.9%** |
| child / stranger, 10–80 episodes | as D2 | identical to D2 |

**What went wrong first.** Without the fresh-word rule, sepia fell to 0% (0 / 0 / 0 / 33%). The slow speed had learned that numerals and stone names mean colour `any`, which is true of every training order, because they carried no colour-word the student didn't know. Those habits outvoted `sepia`, heard once. Bag of words fails there in the same way (0%). The rule was added after seeing this. On Phi-4 it changes almost nothing: 70.2% stays 70.2%, and the sampled run goes 47.7 → 47.2%.

## Reading

- **On the steady teacher, the counts student now matches bag of words** (70.2% against 70.5% exact) and beats it on the act (79.9% against 77.6%). It gets there with a weight only for words seen often enough.
- **On the rules, it costs nothing.** It is equal to D2 everywhere, and slightly better at 320–640 episodes. It keeps the one-pointing learning that bag of words lacks.
- **On the noisy sampled teacher, the slow speed helps little** (46.6 → 47.2%), and bag of words still leads by 7 points. When the same call gets different answers, per-word weights fitted on four fifths of about 300 answers are mostly noise. The penalty keeps them near D2, which is the right side to fail on.
- **Two speeds are a condition on the post, not two models.** A word is heard by its counts until the language has used it enough to say how much it counts. That is D13 (working exactness) made per word.

## Next

- Run `.\scripts\phi4_builders.ps1 -Roles`. Its `_graded.md` now includes this learner.
- Carry the two-speed post to the code reader (docs/110): each (d, w) condition would get its own weight after N sightings, instead of one per sightings bucket.
