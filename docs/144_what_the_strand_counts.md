# 144 · What the strand counts: the criterion is calibrated; the labels are coarse

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/143 found the strand's boundaries clearly better than chance, but its count above the true number on joined *Emma* chapters. Before changing the criterion, this doc asks *why*. Either the cut is too cheap and over-divides any text, or the text has more structure than the chapter labels mark. Two experiments decide it, and one control explains it.

## 1. Text whose only structure is known (`strand --synthetic`)

Each trial draws `k` segments (1 to 8), each 60–180 words, every word drawn independently. A word comes from English (*Emma*'s unigram) with probability `1 − λ`, and from a topic of thirty words of the segment's own with probability `λ`. Sentence marks fall every 10–20 words regardless of topic, and every segment ends on one. Inside a segment there is nothing to find. A calibrated criterion should therefore support `k` units when the topics are strong enough to see, fewer when they are not, and never more.

| λ (topic strength) | seed | exact | **over** | under | mean \|u − k\| | boundary F1 at k (±10) |
|---|---|---|---|---|---|---|
| 0.05 | 5 · 9 | 8 · 6 | **0 · 0** | 52 · 54 | 3.70 · 3.62 | 0.150 · 0.138 |
| 0.10 | 5 · 9 | 6 · 9 | **0 · 0** | 54 · 51 | 3.20 · 3.20 | 0.177 · 0.165 |
| 0.20 | 5 · 9 | 22 · 16 | **1 · 0** | 37 · 44 | 1.13 · 1.20 | 0.573 · 0.548 |
| 0.40 | 5 · 9 | **53 · 54** | **6 · 4** | 1 · 2 | 0.13 · 0.10 | **0.956 · 0.953** |

(60 trials per cell.)

**The criterion is calibrated, and conservative.** Over-counts: 11 in 480 trials (2.3%), all at the strongest topics. Where the topics are faint it stays silent rather than guess. Where they are clear it finds `k` exactly in 89% of trials, with boundaries at F1 0.95. The MDL rule (a division must pay for itself, its subtree included) does what it should: no structure is invented.

## 2. Real text with clear shifts (`strand --cross`)

Each trial joins runs of paragraphs, about 120 tokens each, from documents of four different kinds, with no two neighbours of the same kind:
- *Emma* chapters;
- our own docs (`crates/phiano/docs/*.md`);
- the talk;
- Rust source (`crates/phiplm/src`).

There are 196 documents in all. The prior is *Emma* without any chapter used.

| seed | trials | units vs k | at true k: Pk · boundary F1 (±10) |
|---|---|---|---|
| 7 | 60 | exact 1, over 58, under 1; mean excess +9.5 | **0.138 · 0.692** |
| 11 | 60 | exact 0, over 60; mean excess +9.0 | **0.174 · 0.564** |

The boundaries are the best yet. Given the count, the strand puts the cuts where the documents change (F1 0.56–0.69, Pk 0.14–0.17, against 0.31 on same-novel joins). But the count is nine units high. Section 1 rules out an over-dividing criterion, so the units must be inside the documents.

## 3. The control: one document at a time

Twenty runs of the same length were taken from a single document of each kind, and each run was given to the strand on its own:

| kind | tokens a run | units supported: mean (range) |
|---|---|---|
| *Emma* | 213 | **1.30** (1–4) |
| our docs | 223 | **4.35** (1–10) |
| Rust | 340 | **4.50** (1–14) |

That is the whole of the excess. A novel's paragraphs flow into one another: about one unit per two hundred words, and a little more over long runs (docs/143 §2). A technical doc runs prose, a table, a command and a code fragment in turn, each with a vocabulary of its own. A source file is one function after another, each with its own identifiers. The strand counts **topical units, where a text's words change**, and how many of those a document holds depends on what kind of document it is. Chapter and file labels are coarser than that.

## 4. What the strand's number now means

| on | count | boundaries |
|---|---|---|
| text with exactly `k` topics | `k` when visible, fewer when faint; never invented (2.3% over) | F1 0.95 at λ = 0.4 |
| one novel's chapters | about one extra unit per 500–800 words, from real sub-topics | F1 0.35–0.39 (same style throughout: the hard case) |
| documents of different kinds | the topical units inside each document: about 4 per 220–340 tokens of docs or code | F1 0.56–0.69 |

So "the talk supports seven units" means seven places where its vocabulary changes enough to pay for saying so. That is a measurement of the talk, read against English, and not an over-estimate of it. docs/143 called it an upper estimate relative to labels. With this doc, the honest wording is the one in §3: the strand measures topical units, and a label (a chapter, a file, a speaker's turn) may hold several.

Next, in order:
1. **The strand's units as scopes for reading.** Does a text read with the strand's levels as conditions predict its own held-out words better than the same text with no levels, or with its chapter labels only? That is the test of whether the levels it finds are the ones a reading should use.
2. **Pair-aware code (D2).** It applies only if the reading test says the units are too fine.

**Done in docs/145.** Read with weights the text counts itself, the strand's units read held-out words better than equal parts everywhere there is structure to find, and better than the true labels on mixed text (72 of 80 texts). The level the strand supports is within 0.01 nats a word of the best of its sixty-four levels, so the units are not too fine and D2 is not called for. The same test showed that Witten–Bell, used as a reading rule, over-trusts a small scope.

`strand --synthetic --trials 60 --seed 5` · `--seed 9` · `strand --cross --trials 60 --seed 7 --out <dir>` · `--seed 11`. Outputs are in `crates/phiano/data/talks/strand_bench/`.
