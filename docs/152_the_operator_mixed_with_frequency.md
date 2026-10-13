# 152 · The operator mixed with frequency, fitted where it never trained

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/151 found that the open stratum moved when the operator read the words two and three back. The open stratum is the 23% of held-out positions whose word was never told after the last word or pair. Before reading that as inference, one simpler explanation has to be ruled out: the further-back readings may be flatter, and so closer to frequency, which is the best reader of that stratum (3.4%). This doc mixes frequency into the operator's reading directly. It fits the mixture where the operator never trained, and corrects one fault in how the operator was read.

## 1. A fault: the operator never predicted a word after itself

`RelationModel::answers` leaves the asked word out of its answers. That is right for relations such as `cat · is-a`, where cat is not an answer. It is wrong for `next`, where a word can follow itself. In *Emma* that happens often, because the text's dashes are two tokens, `- -`. Every operator reading from docs/136 to docs/151 went through `answers`, so the word itself always scored −1.

`story/operator.rs` `cosines_under` now reads every value, the word itself included. The phase readings of docs/153 (`waves/wave.rs`) do the same. On one operator (§2's, epoch 32):

| reading of the last word | told stratum (8,147) | open stratum (2,431) |
|---|---|---|
| the word itself left out (docs/136–151) | 15.7% · 0.271 | 1.9% · 0.052 |
| **every value read** | **18.3% · 0.296** | 1.9% · 0.052 |

The told stratum gains 2.6 points; the open stratum does not move. The open-stratum results of docs/137–151 stand. On the told stratum, every operator figure since docs/136 was low by about that much.

## 2. Fitting the mixture where the operator never trained

`story/reader.rs` now makes the prior, the counts' reading with no condition, the first component of the operator's mixture. Its weight is counted with the offsets' (`relate::mixture_weights`). Weights fitted on told rows have a bias: the operator was trained on those rows, so it explains them better than it will explain unseen text.

To remove the bias, `operator_run --dev 0.10` trains on the paragraphs outside a 10% share and fits the mixture on the 5% between that share and the 5% held out. Those are paragraphs the operator never saw (174,107 training rows; 11,102 fitting rows). `story --dev 0.10` does the same for the base.

| the mixture fitted on… (same operator, dev split, epoch 32) | prior weight | told: mixed | open: mixed |
|---|---|---|---|
| the rows the operator trained on | 0.005 | 18.3% · 0.297 | 3.0% · 0.072 |
| **rows it never saw** | **0.085** | **18.4% · 0.299** | **3.2% · 0.074** |

Fitted on its own rows, the operator gives frequency almost nothing. Fitted on unseen rows, frequency gets seventeen times the weight, and both strata read better. Every mixture from here on is fitted on unseen rows.

The curve of the dev-split run (R = 64, 32 epochs, every value read):

| epoch | prior weight | told: last word | told: mixed | open: last word | open: mixed |
|---|---|---|---|---|---|
| 8 | 0.130 | 14.3% · 0.254 | 14.7% · 0.261 | 1.6% · 0.050 | 3.1% · 0.072 |
| 16 | 0.101 | 16.9% · 0.279 | 17.7% · 0.286 | 1.9% · 0.051 | 3.2% · 0.074 |
| 24 | 0.077 | 17.8% · 0.291 | 17.9% · 0.293 | 2.4% · 0.056 | **3.6% · 0.077** |
| 32 | 0.085 | 18.3% · 0.296 | 18.4% · 0.299 | 1.9% · 0.052 | 3.2% · 0.074 |
| *frequency* | | 6.9% · 0.156 | | 3.4% · 0.076 | |

- **On the open stratum the mixture is level with frequency, not past it.** It reads 3.1–3.6%. At one checkpoint of four (epoch 24) it is above frequency in both top-1 and MRR, 3.6% · 0.077 against 3.4% · 0.076. At the other three it is just below.
- **That is about what docs/151's offsets reached** (2.2–2.8%). So their gain on the open stratum is accounted for by frequency. The words before the last did not carry anything frequency does not.
- **The told stratum keeps the operator's reading**, plus up to 0.8 points.

## 3. As the base the levels move from

Flat and tree readings of held-out *Emma* (seed 7) with the operator's mixture as the base, where the path form (docs/149) mixes it with every posted condition. Perplexity · top-1 (flat):

| base | the word itself left out | every value read |
|---|---|---|
| none (the prior) | 137.0 · 20.8% | |
| next-word operator, epoch 32, no prior (docs/151) | 127.6 · 20.7% | |
| the same, prior mixed, fitted on its own rows | 126.7 · 20.8% (prior 0.059) | 128.1 · 20.8% (prior 0.002) |
| dev-split operator, epoch 32, prior mixed, fitted on unseen rows | **126.5 · 21.0%** (prior 0.145) | 127.2 · 20.9% (prior 0.085) |
| tree, the same base as the row above | 130.8 · 20.4% | 131.4 · 20.4% |

- **As a base, the corrected reading is 0.7–1.4 higher in perplexity**, though it ranks better. Reading the word itself puts probability on repetition at every position, and the fitted mixture then gives frequency less weight (0.085 against 0.145).
- **The lowest perplexity on held-out *Emma* is now 126.5**, from the dev-split operator with the word itself left out, prior mixed and fitted on unseen rows. That operator was trained on 5% less text than the one behind 127.6.
- **Top-1 stays at 20.8–21.0%.** As in docs/146–151, the base decides what is kept for the words that were not first.
- The choice between the two readings is left open here. Reading every value is the right reading of `next`; leaving the word out is what the lowest perplexity used. Neither is a setting to tune on held-out text.

## 4. What this says

The operator's reading and frequency together are as good as frequency on the never-told stratum, and better than either on the told one. Weights have to be fitted on rows the operator did not train on, or frequency gets almost nothing. Nothing yet reads the open stratum clearly better than frequency. docs/153 reads the same relations as waves, and reads them backwards.

`operator_run data/text --in text/emma --r 64 --epochs 32 --every 8 --dev 0.10 --out data/runs/operator_r64_s7_dev10` · `operator_run … --dev 0.10 --measure data/runs/operator_r64_s7_dev10/epoch_N.relate` · `story data/text --in text/emma --held-out 0.05 --seed 7 --operator 64,32 --dev 0.10 --model data/runs/operator_r64_s7_dev10/epoch_32.relate`. Outputs are in `data/runs/operator_r64_s7_dev10/log.md` (written during the run, with the word itself left out), `data/runs/waves/fno_dev10_epoch{8,16,24,32}.md` and `…_epoch32_trainfit.md` (every value read), and `data/runs/mixed_prior/`.
