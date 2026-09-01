# MEASURED RESULTS — first harness run

> _Every number below comes from `cargo run --release --bin evaluate`, on
> `data/rust_book_corpus.txt`, 7,757 sentences, 80/10/10 deterministic split,
> vocabulary 6,016. Raw log: `data/evaluation.json`._

---

## 1. The headline

| Model | Held-out perplexity |
|:---|---:|
| **Phiano — counts + absolute discounting + unigram back-off** | **124.66** |
| Kneser-Ney trigram baseline | 131.02 |
| Phiano — phase manifold as the back-off distribution | 1,034 |

Two findings, and they point in opposite directions.

**Phiano's n-gram layer beats the Kneser-Ney baseline** — 124.66 vs 131.02, a 5%
improvement, once its tables are wrapped in absolute discounting with a proper
hierarchical back-off. Before this work the same tables were raw maximum
likelihood, an unseen n-gram had probability exactly zero, and measured
perplexity was **119,644**.

**The phase manifold does not currently contribute to prediction.** This is the
question [HOW 15](15_proving_it_works.md) was written to settle, and the answer
is unambiguous.

---

## 2. The mixing sweep — the decisive experiment

`P_base(c | ctx) = γ · P_phase(c | ctx) + (1 − γ) · P_unigram(c)`

Everything else — the counts, the discount, the interpolation — is held
identical. γ = 0 removes the manifold; γ = 1 makes it the sole back-off. So the
curve *is* the measurement of what the manifold contributes.

| γ | perplexity | γ | perplexity |
|---:|---:|---:|---:|
| **0.0** | **124.66** ← best | 0.6 | 160.04 |
| 0.1 | 127.95 | 0.7 | 172.77 |
| 0.2 | 132.24 | 0.8 | 191.98 |
| 0.3 | 137.28 | 0.9 | 228.53 |
| 0.4 | 143.29 | 1.0 | 1,034.12 |
| 0.5 | 150.66 | | |

**Monotonically increasing.** The optimal mixing weight is exactly zero. Every
unit of phase information added makes prediction worse.

This is not a tuning artefact. `P_phase` is a softmax over **mean phase
coherence across 16 independent channels** — the multi-channel similarity the
torus representation exists for, not a single angle — and the curve has no
minimum anywhere in the interior.

---

## 3. Is it the objective? — yes, and by a factor of 27

The obvious hypothesis: centroid attraction teaches the manifold *what
co-occurs*, while a language model is scored on *what follows*. `bin/experiment`
tests it across three axes — training regime × context construction × softmax
temperature — and adds the control that makes the answer quantitative.

### The control

Mixing phase against a **unigram** back-off asks it to beat word frequency, which
is a strong and trivially available opponent. Mixing it against a **uniform**
base asks only whether it beats knowing nothing. On a log scale, uniform is the
floor and unigram is a reference point, so the fraction of that gap the phase
distribution closes measures how much the representation actually knows.

| training regime | uniform base | phase base | unigram base | signal recovered |
|:---|---:|---:|---:|---:|
| co-occurrence + ranking | 193.58 | 192.77 | 124.66 | **0.9%** |
| **ranking objective only** | 193.58 | **173.92** | 124.66 | **24.3%** |

Training the manifold on next-word ranking alone, rather than alongside centroid
attraction, takes it from carrying essentially nothing (0.9%) to carrying a
quarter of the predictive signal that raw word frequency provides — a **27×
improvement from changing the objective and nothing else**.

That is the clearest confirmation available of the argument in
[HOW 02 §7](02_the_kuramoto_step.md) and [HOW 16 §4](16_learning_anything.md):
centroid attraction is a *descriptive* target whose optimum is collapse, and a
predictive target is what makes a representation encode anything worth having.
The two objectives pull the phases in different directions; running both is
worse than running the useful one.

### The recurrent context helps too

Comparing context constructions at γ = 1 (phase alone as the back-off):

| context | β = 1.0 | β = 0.25 |
|:---|---:|---:|
| two-word centroid | 359.32 | 199.98 |
| **recurrent state** `h_t = λ_k e^{iω_k} h_{t-1} + z_t` | **275.90** | **183.43** |

The recurrent state beats the two-word centroid by 23% at β = 1 and 8% at
β = 0.25. Carrying the whole prefix at per-channel timescales is worth real
perplexity — this is the one component holding information a trigram table
structurally cannot have.

### But γ\* is still 0

Across all 20 configurations of context × temperature, mixed against a unigram
back-off, the optimal mixing weight remains **exactly zero**. Phase at its best
(173.92) is still well behind unigram frequency (124.66).

Note also which temperature wins: β = 0.25, the *flattest* setting tested.
Sharpening the phase distribution monotonically hurts. A distribution that is
best when it is closest to uniform is a distribution that does not know much —
which is the same conclusion the recovery percentage reports, arrived at
independently.

## 3b. A claim of mine that measurement refuted

[HOW 04 §8](04_cooccurrence_memory.md) and [HOW 13 §4](13_persistence_and_cost.md)
both asserted that pruning singleton n-grams would roughly halve the table at
"negligible quality cost". It does not.

| | before | after |
|:---|---:|---:|
| n-gram entries | 136,807 | 26,338 (**−80.7%**) |
| held-out perplexity | 148.92 | 269.89 (**+81.2%**) |

On a corpus of 7,757 sentences with a 6,016-word vocabulary, most n-grams are
singletons *and they carry most of the coverage* — discarding them discards the
model. Pruning is a size/quality trade that pays only where repetition is heavy
enough for singletons to be genuine noise.

The lossless route to footprint is vocabulary interning, which is still
outstanding. `Facet::prune_singletons` remains available, with the measured cost
in its doc comment rather than the assumption that motivated it.

This is the harness doing the job it was built for: the claim was plausible,
widely true elsewhere, and wrong here.

## 3c. Relation accuracy — does the manifold know that `woman` goes with `man`?

Perplexity says whether the model predicts text. It says nothing about whether
related words sit in related places. `bin/relations` trains on the full
Webster's dictionary (71,809 definitions, 124,504-word vocabulary) and asks
three questions of increasing difficulty, each against its own chance baseline.

| family | pairs | pair > random | near@10 | near@50 | analogy@1 | analogy@5 |
|:---|---:|---:|---:|---:|---:|---:|
| gender (`man:woman`, `grandfather:grandmother`, …) | 10 | 80% | 0% | 0% | 0.0% | 0.0% |
| number (`man:men`, `woman:women`, …) | 7 | 100% | 0% | 0% | 0.0% | 0.0% |
| antonym (`hot:cold`, `big:small`, …) | 6 | 83% | 0% | 0% | 0.0% | 0.0% |
| **chance** | | **50%** | **0.01%** | **0.04%** | **0.0008%** | |

Three findings, and they are not the same finding.

**Grouping: weakly real.** `resonance(man, woman)` beats `resonance(man, random)`
87% of the time against a 50% baseline. There is signal. It is above chance and
it improved as the vocabulary grew from 56k to 124k.

**Localisation: absent.** `woman` is not among the **50 nearest words to `man`**,
out of 124,504. Nor is `grandmother` near `grandfather`, nor `cold` near `hot`.
An 87% pairwise win rate and a 0% top-50 rate are consistent: the signal is real
and far too weak to survive competition with 124,502 other words.

**Relations: absent.** Analogy asks whether the step from `man` to `woman` is the
*same step* as from `grandfather` to `grandmother` — computed as a per-channel
phase offset, which is exactly unbind-then-bind in the phase domain. Zero of the
tested analogies ranked first. Not "low"; zero.

### This agrees with the perplexity result

Two independent measurements now say the same thing. The mixing sweep put the
manifold at 24.3% of the predictive signal that raw word frequency provides.
Relation accuracy puts it above chance on grouping and at chance on structure.
Both describe a representation with weak global organisation and no local
organisation — which is what contrastive co-occurrence training produces, because
co-occurrence is what it is given.

### Definition grounding does not help

Grounding is the idea [HOW 05](05_definition_grounding.md) calls the best in the
codebase. Measured here it:

- drops phase dispersion from **0.94 to 0.50** — it concentrates the manifold,
- and changes **none** of the relation metrics.

Placing a word at the centre of mass of its definition is a plausible way to get
semantic positions, and on this evidence it tidies the manifold without adding
relational structure. That does not make it worthless — it is still the reason
positions are non-arbitrary — but the claim that it grounds *meaning* is not yet
supported by anything measured.

## 3e. The objective is what creates relational structure

§3c measured relation accuracy under co-occurrence training and found zero
analogy signal. That was an incomplete experiment: §3 had already shown the
ranking objective recovers 27x more predictive signal, and the relations run had
not used it. Re-run across all three regimes, on 12,000 dictionary definitions
(41,489-word vocabulary):

| objective | pair > random | near@10 | near@50 | analogy@1 | analogy@5 | MRR |
|:---|---:|---:|---:|---:|---:|---:|
| co-occurrence | **69%** | 5% | 5% | 0.00% | 0.00% | 0.0005 |
| **ranking** | 53% | 5% | **10%** | **0.62%** | **1.59%** | **0.0120** |
| both | 55% | 5% | 5% | 0.00% | 0.00% | 0.0006 |
| *chance* | *50%* | *0.02%* | *0.12%* | *0.0024%* | *0.0121%* |

A clean dissociation, and the third measurement to point the same way.

**Co-occurrence groups; ranking structures.** Centroid attraction is markedly
better at the crude question — is `woman` closer to `man` than a random word is —
at 69% against 50% chance. It produces **exactly zero** relational structure.
The ranking objective is barely above chance at grouping (53%) and is the only
regime where an analogy is ever solved: 0.62% at rank 1 against 0.0024% chance,
and a mean reciprocal rank 24x the co-occurrence figure.

**Read the analogy number carefully.** 0.62% of ~162 analogies is *one correct
answer*. At chance you would expect 0.004, so a single hit is unlikely — but it
is one hit, and n = 1 deserves no more weight than that. The trustworthy number
is **MRR: 0.0120 against 0.0005**, because it averages over all 162 trials rather
than counting a single success.

**Running both is worse than running the better one.** On every relational
metric, `both` matches co-occurrence rather than ranking. The two objectives
interfere — the third independent observation of this, after the perplexity
sweep and the dispersion trace.

**Ranking also collapses least**, holding dispersion at 0.9968 against 0.9544.

### And it is now the cheap option

`train_predictive` recomputed the prefix centroid at every position, making it
O(L²·D) per sentence — which is why it cost 157 seconds against co-occurrence's
10. Carrying the per-channel accumulators forward makes it O(L·D) for the same
result:

| | before | after |
|:---|---:|---:|
| 12,000 definitions | 157.2s | **5.4s** (29x) |

The better objective is now also the faster one, which removes the only argument
for the current default.

**Recommendation:** `Trainer::train` — record n-grams and amplitudes, then rank,
with no centroid attraction — should replace `train_sentence` as the default
learning path. The method exists and is documented; flipping the ~20 call sites
is a separate change so the perplexity effect can be measured on its own.

## 3f. Catastrophic forgetting — measured at last

`docs/45` asserts **"Catastrophic Forgetting: Zero"** in a comparison matrix
against Phi-4, GLM and GPT-4. Nothing tested it. It is also the claim this
architecture is most likely to win, which made leaving it unmeasured the biggest
missed opportunity in the project.

Domain A is the Rust Book (3,000 sentences, 750 held out); domain B is Webster's
(3,000 entries). They share function words and little else. Three models, two of
them bounds:

| trained on | co-occurrence | ranking |
|:---|---:|---:|
| A only | 5,246 | 4,970 |
| A and B **interleaved** (ceiling) | 5,533 | 5,482 |
| A then B (the measurement) | 7,192 | 5,899 |
| B only (floor) | 311,784 | 275,607 |
| **retention** | **93.5%** | **98.1%** |
| degradation on A | +37.1% | +18.7% |

`retention = (ln floor − ln sequential) / (ln floor − ln ceiling)`.

**The claim substantially holds.** Against a floor 50× worse, a model that
learned an entirely new domain kept 93–98% of what it knew. That is a genuinely
strong result and it is now measured rather than asserted.

**But "zero" is wrong, and the honest number is better anyway.** Held-out
perplexity on A still worsens by **18.7%** after learning B. "Zero catastrophic
forgetting" invites a reviewer to find the 18.7% and dismiss the whole matrix.
"93–98% retention against a measured floor, with 19% degradation" is a claim
that survives scrutiny and is more informative.

**The ranking objective forgets less** — 98.1% against 93.5%, degradation halved.
That is the fourth independent measurement favouring it, after the mixing sweep,
the relation benchmark and the dispersion trace.

### Two caveats worth stating before quoting this

**The n-gram tables cannot forget by construction.** Scored from counts alone the
sequential model reads 384.70, and it would read that whatever it learned next:
tallies are never overwritten. So a large part of the system is trivially immune,
and the measurement above is deliberately taken at γ = 1 to put the *manifold*
under test instead.

**Retention is measured on the weak component.** Those γ = 1 perplexities are in
the thousands because the phase back-off is a poor language model on its own — 
§3 established that. Retention is a valid measurement of what the manifold keeps;
it is not a claim that the manifold is good.

### The bug this benchmark shipped with

The first run reported retention of exactly 100.0%, with the ceiling and the
sequential model identical to two decimal places. They were the same model:
training all of A and then all of B *is* the sequential condition, so using it as
the joint ceiling made retention 1.0 by construction. The ceiling now interleaves
the domains round-robin, and a regression test asserts the two conditions produce
different models.

A benchmark that reports a perfect score is to be distrusted before it is
believed.

## 3d. Footprint after interning

The n-gram tables keyed on owned `String`s: each bigram follower stored a full
copy of the word, each trigram key stored two, and `phase_lags` duplicated the
bigram key set a third time. Interning to `u32` ids, with sorted
`Vec<(id, count)>` follower lists instead of nested hash maps:

| | dictionary-scale model |
|:---|---:|
| vocabulary | 124,504 |
| n-gram entries | 2,698,355 |
| **on disk, interned (measured)** | **59.2 MB** |
| string-keyed equivalent (estimated) | 154.2 MB |
| reduction | **62%** |

The 59.2 MB is measured; the 154.2 MB is an estimate from mean word length and
entry count, so treat the ratio as approximate.

**And the README's 2–12 MB target is still out of reach — for a reason that is
not encoding.** 2.7 million n-gram entries cost 8 bytes each in the best possible
layout, so the payload alone is 21.6 MB before any overhead. Hitting 12 MB
requires *fewer n-grams*, not a better encoding of them. Pruning is the obvious
lever and §3b measured what it costs. That trade is a decision, not an
optimisation.

## 4. Collapse is fixed

| epoch | valid ppl | coherence | **dispersion** | gini |
|---:|---:|---:|---:|---:|
| 0 | 1,173.00 | 0.575 | 0.959 | 0.092 |
| 1 | 1,509.08 | 0.650 | 0.959 | 0.090 |
| 2 | 1,731.88 | 0.551 | 0.944 | 0.102 |

Phase dispersion holds at **0.94–0.96** (1.0 = uniformly spread, 0.0 = every word
at one angle) and sector Gini stays near 0.09. The negative-sampling term is
doing its job: the manifold no longer synchronises toward a point.

The unit test `test_repulsion_prevents_collapse` asserts this directly, by
training the same corpus with and without repulsion and comparing dispersion.

Note also that **coherence moves independently of perplexity** — it rises from
0.575 to 0.650 between epochs 0 and 1 while held-out perplexity gets 29% worse.
That is [HOW 08](08_self_scoring.md)'s argument, now visible in data.

---

## 5. Overfitting

Held-out perplexity is **best at epoch 0** and rises monotonically thereafter,
while training perplexity falls (9.01 → 4.85). One pass over this corpus is the
right amount of training; the harness reports `best_epoch` so this is no longer
something anyone has to guess.

---

## 6. What this means

Read plainly:

- The **infrastructure claims hold**. Contrastive training prevents collapse.
  Identity seeding separates same-length words. Binding makes word order survive.
  The counts, properly smoothed, beat a standard baseline.
- The **representation carries real but insufficient signal**. It beats uniform;
  it loses to unigram frequency. At its best it recovers 24.3% of what counting
  words provides.
- The **objective was the binding constraint**, and it was measurable. Switching
  from co-occurrence to ranking moved the manifold from 0.9% to 24.3% — 27×,
  from one change.

Capacity was necessary and not sufficient. The channels are independent now
(`test_torus_reads_independent_channels` proves the torus is no longer a 1-D
curve), and what fills them usefully is the objective, not the width.

## 7. What to try next, in order

The three axes already swept say where the remaining slope is.

1. **Train on ranking only, for longer, and measure.** The 27× came from a few
   passes. `Harness::train_ranking_only` takes a pass count; sweep it. This is
   the single highest-slope direction identified so far.
2. **Wire the recurrent state into the trainer, not just the scorer.** It is
   worth 23% at scoring time while the trainer still uses a two-word window, so
   the representation is being taught against a weaker context than the one it
   is evaluated on.
3. **Widen `LM_CHANNELS` past 16** toward the full 64, now that there is a metric
   that will say whether the extra channels carry anything.
4. **Reconsider the count/phase split.** Phase is losing to unigram at the
   back-off position. It may be the wrong job for it — the manifold may be
   better used to *re-rank* the n-gram candidate set, where it only has to break
   ties among plausible continuations rather than model the whole vocabulary.

Every one of these is one command to evaluate, and the answer is a number.

---

## 8. Reproducing

```bash
cargo run --release --bin evaluate                        # per-epoch training curve
cargo run --release --bin experiment                      # the full experiment grid
cargo run --release --bin experiment -- <corpus> <epochs>
cargo test                                                # 94 tests
```

Output: `data/evaluation.json`, `data/experiment.json`.
