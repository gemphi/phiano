# Results — non-linear readout, score saturation, order, and definition composition

Sections to append to `docs/how/RESULTS.md`. Every number here is held-out, on a
deterministic 80/10/10 split, and reproducible from a named binary.

---

## §4a — A measurement bug that invalidated every earlier γ sweep

**Found while wiring the non-linear readout. It changes how §3's headline should
be read.**

`PhianoLM` scored a candidate by summing the per-channel cosine against the
context across all 16 language-model channels. A sum over 16 terms each in
[-1, 1] spans 32 units. Exponentiating a spread that wide saturates the softmax:
the top candidate takes essentially all the mass and every other candidate
underflows the probability floor (`1 / (V·100)`).

Measured on the toy corpus: the score spread was ~20 units at a typical
position, and the phase probability was **pinned at the floor at essentially
every position**. A back-off distribution that returns the same constant
everywhere cannot beat a unigram at any mixing weight — so γ\* = 0 was partly a
statement about the score *scale*, not only about the manifold.

The fix is one line: score with the **mean** cosine across channels rather than
the sum, which puts scores in [-1, 1] and gives β back its role as a
temperature. `PHASE_BETA` moves 1.0 → 8.0 to compensate.

What it changed, on `rust_book_corpus.txt`, 1 epoch:

| quantity | before fix | after fix |
|---|---|---|
| phase recovers, of unigram's signal (co-occurrence) | 0.9% | **4.7%** |
| phase recovers, of unigram's signal (ranking-only) | 24.3% | **25.3%** |
| best γ | 0 | 0 |
| best perplexity | 124.66 | 124.66 |

So the manifold carries about **five times more signal** than §3 reported under
the co-occurrence objective, and the ranking result was largely unaffected. The
conclusion does not flip — γ\* is still 0, phase still loses to word frequency —
but the size of the gap was overstated.

Guarded by `test_phase_distribution_is_not_saturated`, which asserts the score
spread stays within [-1, 1].

**This is the fourth claim this harness has falsified, and the second one that
was the harness's own.**

---

## §4b — The non-linear readout: HOW 16's open question, closed

HOW 16 argued that every scoring path in the engine is linear — a weighted sum
of unit vectors, then argmax — and that a non-linear readout is the fourth
missing requirement. `SectorReadout` implements one: a table keyed on the
discretised context cell holding **a bias per target sector**. The conditioning
on both sides is what matters; a bias depending on the context alone adds the
same constant to every candidate and cannot reorder anything. (The first
implementation had exactly that bug; `test_bias_is_conditional_not_constant`
now documents it.)

Fitted on the training split only, scored on validation. `cargo run --release
--bin readout`:

| training regime | context | cells | held-out coverage | phase alone | best γ off → on |
|---|---|---|---|---|---|
| co-occurrence + ranking | 2-word | 3981 | 99.8% | 192.55 → 191.99 (−0.29%) | 0 → 0 |
| co-occurrence + ranking | recurrent | 3593 | 99.4% | 189.10 → 185.64 (−1.83%) | 0 → 0 |
| ranking only | 2-word | 3735 | 99.7% | 184.70 → 183.10 (−0.86%) | 0 → 0 |
| ranking only | recurrent | 2033 | 99.4% | 180.40 → **174.22 (−3.42%)** | 0 → 0 |

**Answer: no.** The readout improves the phase distribution measured on its own
— up to 3.4% — but never enough to move γ\* off zero. Linearity was not the
binding constraint on this corpus.

Two things make that a real answer rather than a null result:

* **Coverage is ~99.5%.** The table is consulted at essentially every held-out
  position, so "no effect on γ\*" is not a table that never fires. Getting there
  required a design change: the key grid is deliberately coarser (8 buckets per
  channel) than the 64-sector target grid, because 64⁴ is 16.7M cells and a table
  that fine is a cache miss on every unseen context. `test_finer_keys_cover_less`
  pins that trade-off.
* **The γ = 0 column is unchanged to 1e-6** in every row. γ = 0 removes the phase
  term, so the readout cannot legally touch it; a move there would mean the
  comparison was leaking. It is asserted in the run and in
  `test_readout_changes_phase_scoring_only`.

---

## §4c — Where word order belongs

Two experiments, opposite answers, and the difference is the point.

### In the sequence: order is real, but the golden-angle rotation is the wrong carrier

`cargo run --release --bin order`. Three context constructions, same facet,
same split. "swap cos" is the cosine between `ctx(a,b)` and `ctx(b,a)`: **1.0
means order is not represented at all.**

| regime | context | swap cos | phase alone | best γ |
|---|---|---|---|---|
| co-occurrence | 2-word | 0.509 | 192.57 | 0 |
| co-occurrence | bound | −0.428 | 193.56 | 0 |
| co-occurrence | recurrent | n/a | **188.63** | 0 |
| ranking only | 2-word | 0.617 | 182.69 | 0 |
| ranking only | bound | −0.249 | 194.30 | 0 |
| ranking only | recurrent | n/a | **173.08** | 0 |

* The existing 2-word context **barely encodes order at all** (swap cos 0.51 and
  0.62 — the swapped context is still more similar to the original than not).
  The 0.4/1.0 recency weighting is a magnitude weight, not an encoding.
* Positional binding genuinely fixes that (swap cos goes *negative*: the swapped
  context is now anti-correlated). **And it makes prediction worse** — 194.30 vs
  182.69 under the ranking objective.
* The recurrent state, which carries order through its per-channel rotation
  kernel, is the best of the three in both regimes, by a wide margin under
  ranking (173.08 vs 182.69).

So order matters, and a learned/tuned rotation carries it; a fixed golden-angle
rotation encodes order faithfully and predicts worse. Faithful ≠ useful.

### In a definition: order actively hurts

See §4d. Binding a dictionary entry by word position was the **worst** condition
tested, 34 points below baseline.

The reconciliation: a sentence *is* a sequence and its order is signal. A
definition is a set of constraints that happens to have been written down in a
line — the rank of *animal* in "a small domestic feline animal" is a fact about
the lexicographer, not about *cat*. Rotating by that rank scatters the same
concept word to a different angle in every entry it appears in.

---

## §4d — Definitions as compositions, and mutual reinforcement

The existing `DefinitionGrounder` was measured earlier: it halves phase
dispersion and improves no relation metric. Three properties explain why — it
writes **one channel** of 64, it uses a **symmetric centroid**, and the pull is
**one-way** (the headword moves toward its definers; the definers never move
toward the headword, so a family of mutually-defining words never converges).

`Conception` changes all three, each switchable, so each is measurable.

`cargo run --release --bin conception`, on a corpus built from the Webster's
entries themselves (the Rust-book corpus contains no kinship vocabulary, which
made the first run vacuous — 0 usable relation pairs). Ranking-only training,
3 composition rounds, 23 usable relation pairs:

| condition | pair/random | nbr@10 | analogy@1 | analogy MRR | dispersion |
|---|---|---|---|---|---|
| baseline | 71.7% | 4.8% | 0.00% | 0.0045 | 0.985 |
| grounder (1 channel, centroid) | 71.9% | 4.8% | 0.00% | 0.0047 | 0.452 |
| compose: bag, no rotation | 82.6% | 0.0% | 1.85% | 0.0296 | 0.504 |
| compose: bound, as written | 37.4% | 0.0% | 0.00% | 0.0044 | 0.964 |
| compose: bound, canonical | 57.1% | 14.3% | 0.00% | 0.0063 | 0.981 |
| **+ reinforce (bag)** | **85.0%** | 0.0% | **4.32%** | **0.0644** | 0.446 |
| + reinforce (canonical) | 59.6% | 14.3% | 0.00% | 0.0025 | 0.964 |

Three findings, in order of size:

1. **Multi-channel composition works where single-channel grounding did not.**
   The grounder moves the relation metrics by +0.2pp and +0.0002 MRR — noise.
   Bag composition across all 64 channels moves pair/random +10.9pp and MRR ×6.6.
   The limitation was never the idea; it was writing one channel of sixty-four.

2. **Mutual reinforcement is the largest single effect measured on this
   benchmark to date.** Adding the back-pull on top of bag composition takes MRR
   from 0.0296 to 0.0644 — it more than doubles what composition alone
   achieves, and lifts analogy@1 from 1.85% to 4.32%. Letting the definers move
   toward the headword is what turns a definition graph into concept regions
   instead of independent placements.

3. **Positional binding inside a definition is destructive.** −34.3pp on
   pair/random, the worst condition in the table. Canonicalising position by
   sorting the definer set recovers about half of it and is the only condition
   that improves neighbour@10 (+9.5pp), which is worth noting but does not
   recover the loss.

The no-phase perplexity is 437.68 in every row, as it must be: composition
touches phases only, and the n-gram path cannot see it. A row where that number
moved would mean the experiment was contaminated.

**Caveat, stated up front: 23 usable relation pairs.** These effects are large
and consistent in direction across metrics, but the sample is small, and the
analogy figures rest on few analogies. Before any of this is quoted as a
headline it needs a vocabulary with more relation coverage.

---

## §4e — What the prior literature already settled, and what it implies next

Three of these results have direct precedent, and the precedent is useful
because it says which of them is likely to generalise.

* **Dict2vec** (Tissier, Gravier & Habrard, EMNLP 2017) learns embeddings by
  pulling together words that co-occur in dictionary definitions — "strong
  pairs" when two words appear in each other's definitions, "weak pairs" when the
  relation is one-way — with negative sampling for the rest. That is
  §4d's reinforcement, and its strong/weak asymmetry is a sharper version of
  Phiano's flat `REINFORCE = 0.15`. **Suggested next change:** make the back-pull
  reciprocal-aware — a full step when *A* defines *B* and *B* defines *A*, a
  fraction otherwise.
* **Retrofitting** (Faruqui et al., NAACL 2015) and **counter-fitting** (Mrkšić
  et al., NAACL 2016) post-process trained vectors toward a lexicon by
  alternating "stay near where you were" with "move toward your neighbours".
  `Conception::compose_all` is structurally the same algorithm on a torus, and
  it is missing retrofitting's first term: there is currently **no anchor to the
  pre-composition position**, which is why dispersion falls from 0.985 to 0.446.
  **Suggested next change:** add a β-weighted pull back toward the trained phase,
  and sweep β. Retrofitting's whole point is that the balance is tunable.
* **The grounding kernel** (Vincent-Lamarre et al., *Topics in Cognitive
  Science*, 2016) is the most directly actionable. A dictionary's definitional
  graph has a **Kernel** (~10% of entries — what remains after recursively
  removing words that are defined but define nothing), a **Core** (6–9%, the
  largest strongly connected component), and **MinSets** (~1%, the smallest sets
  from which everything else is definable). Words nearer the MinSets are more
  concrete, more frequent, and acquired earlier.

  `compose_all` currently treats all 39,925 entries as equal and iterates
  Jacobi rounds over the lot. The literature says that is the wrong schedule:
  meaning propagates *outward from the kernel*. **Suggested next experiment,
  and the one I would run first:** compute the Core by SCC on the definition
  graph, hold those words fixed (or move them at a much smaller step), and
  compose the periphery against them. It is a few hours of work, it is testable
  on exactly the benchmark above, and it is the difference between "definitions
  average out" and "definitions ground".

* **Searle and Harnad on why this has a ceiling.** The Chinese Room and the
  symbol grounding problem both say the same thing about this architecture: a
  dictionary is symbols defined by symbols, so composing definitions can arrange
  meaning but cannot originate it — the "dictionary go-round". The MinSet result
  is the empirical form of that argument: ~1% of the vocabulary has to be
  grounded some other way (sensorimotor, in the original framing; in a text-only
  system, by corpus statistics or by an external signal). This is a real ceiling
  on §4d, and it predicts where the returns stop: composition should keep helping
  until the kernel is saturated, then plateau. **That prediction is testable** —
  measure relation accuracy as a function of how much of the Core is covered.

* **Φ-ML framing.** Physics-informed machine learning is the family Phiano
  belongs to: a physical model (Kuramoto–Sakaguchi coupling) used as a structural
  prior instead of a learned one. The honest lesson from that literature is that
  a physics prior pays when the physics is the true generative process and costs
  when it is not — and §3 and §4b are the local version of that. Phase coupling
  is not how text is generated, and every measurement so far says the *objective*
  (ranking, 27× better than co-occurrence) matters more than the representation.
  The prior earns its place on latency, editability and forgetting resistance —
  the 98.1% retention figure — not on perplexity.

### Sources

- [Dict2vec: Learning Word Embeddings using Lexical Dictionaries — ACL Anthology](https://aclanthology.org/D17-1024/)
- [Retrofitting Word Vectors to Semantic Lexicons — ACL Anthology](https://aclanthology.org/N15-1184/)
- [Counter-fitting Word Vectors to Linguistic Constraints](https://ar5iv.labs.arxiv.org/html/1603.00892)
- [The Latent Structure of Dictionaries — Topics in Cognitive Science](https://onlinelibrary.wiley.com/doi/10.1111/tops.12211) ([open PDF](https://eprints.soton.ac.uk/366805/1/PhilV-L.dict.webscimind.pdf))
- [Physics-informed machine learning meets engineering — Alan Turing Institute](https://www.turing.ac.uk/events/phi-ml-meets-engineering)

---

## §4f — Porting dict2vec's mechanics, and what the control said

The user supplied the dict2vec paper, which specifies three things `Conception`
had approximated with a single flat constant:

1. **Strong vs weak pairs.** Strong = each word occurs in the other's
   definition (*car*/*vehicle*); weak = one-way (*car*/*road*). Their grid
   search selected β_s = 0.8, β_w = 0.45.
2. **Controlled negative sampling.** Never draw a definitional relative as a
   negative. They measure it discarding ~2% of drawn negatives.
3. Their gains are **largest on small corpora** — +30% at 50M tokens vs +12.5%
   at the full dump — and dictionary pairs beat WordNet pairs (569 vs 564 at
   50M, Table 5). That is the strongest argument this direction suits Phiano,
   which trains on small corpora by design.

Implemented as `DefinitionGraph` (reciprocity test, relatedness test, pair
counts) and `Facet::sample_negative_controlled`. Then measured — with the
control that isolates the split from the rate:

| condition | pair/random | analogy@1 | analogy MRR | dispersion |
|---|---|---|---|---|
| baseline | 68.4% | 0.00% | 0.0010 | 0.986 |
| + reinforce, flat at 0.15 | 85.9% | 4.32% | 0.0698 | 0.480 |
| + strong/weak split (0.8 / 0.45) | 89.2% | 6.17% | 0.0972 | 0.406 |
| **control: flat at 0.8, no split** | **92.5%** | **6.79%** | **0.1066** | 0.327 |

**The split does not beat its own control.** Flat reinforcement at 0.8 beats the
graded rule on every relation metric. What helped was **raising the
reinforcement rate**, not distinguishing reciprocal pairs — and without the
control this would have been reported as "dict2vec's strong/weak split gives
+0.096 MRR", which is true and attributes it to the wrong mechanism.

The diagnosis is in the pair counts the run prints: **22,179 strong to 1,053,528
weak — 47.5:1**, against dict2vec's ~9:1 (417K:3.9M). Webster's entries are long
and discursive, carrying citations and quotations, so nearly every definitional
pair comes out one-way and downweighting weak pairs mostly just reduces total
pull. The split is not refuted; it is untested, because the graph it was given
is not the graph the method assumes. `clean_definition` strips brackets and
apparatus but not quoted usage examples, and that is the fix to make before
re-running this row.

**A warning that belongs beside the winning row:** dispersion falls 0.986 → 0.327
in the best condition. That is the manifold concentrating, and it is the exact
failure mode the earlier collapse work was built to detect. The relation metrics
are up ~100× on MRR and the representation is a third of the way to collapsed.
Retrofitting (Faruqui et al. 2015) has the standard remedy — an anchor term
pulling each word back toward its pre-composition position, weighted against the
neighbour pull — and `compose_all` currently has no such term. **That is the next
change**, and the sweep over its weight is what makes "how much composition"
a tunable rather than a guess.

### Corrected ranking of what to do next

1. Add retrofitting's anchor term and sweep it. The current best result is
   partly collapse.
2. Clean quotations out of Webster's entries, re-measure the strong/weak split
   against a graph with a defensible strong:weak ratio.
3. Grounding-kernel scheduling (§4e) — compose the periphery against a held
   Core rather than treating all 39,938 entries as equal.
4. Wire `sample_negative_controlled` into `Trainer::apply_negatives` and measure
   it. It is built and tested but not yet on the training path.

---

## §4g — Reproducibility, the anchor, and three imported mechanisms that did not pay

### The finding that came first: the experiments were not reproducible

`Facet::rebuild_sample_pool` built the negative-sample pool by iterating
`lexicon`, a `HashMap`. Rust seeds `HashMap` hashing randomly **per process**,
so every run of the same binary on the same data drew a different negative
sequence, trained a different model, and measured different numbers.

Two runs of the identical composition experiment:

| condition | run A | run B |
|---|---|---|
| control: flat at 0.8 — analogy MRR | 0.1066 | 0.0521 |
| control: flat at 0.8 — pair/random | 92.5% | 92.3% |

Same ranking, **magnitudes a factor of two apart**. Every effect smaller than
that gap was unfalsifiable, including several reported in §4d and §4f. The pool
and the LM's vocabulary index are now sorted; `test_training_is_reproducible`
pins it, and two consecutive runs now produce byte-identical grounding traces.

**Everything below is from a deterministic run. §4d and §4f magnitudes should be
read as directions, not sizes.**

### Deterministic results

39,938 definitions, ranking-only training, 3 composition rounds, 23 usable pairs:

| condition | pair/random | analogy@1 | analogy MRR | dispersion |
|---|---|---|---|---|
| baseline | 69.2% | 0.62% | 0.0107 | 0.994 |
| grounder (1 channel, centroid) | 69.5% | 0.62% | 0.0092 | 0.467 |
| compose: bag, no rotation | 81.9% | 5.56% | 0.0654 | 0.528 |
| compose: bound, as written | 37.7% | 0.00% | 0.0108 | 0.974 |
| + reinforce, flat at 0.15 | 83.0% | 5.56% | 0.0643 | 0.486 |
| + strong/weak split (dict2vec) | 87.4% | 7.41% | 0.0935 | 0.411 |
| **control: flat at 0.8** | 91.3% | **10.49%** | **0.1177** | 0.334 |
| anchor α=0.25 | 93.6% | 5.56% | 0.0833 | 0.369 |
| anchor α=0.50 | 94.4% | 3.70% | 0.0574 | 0.413 |
| anchor α=1.00 | **94.7%** | 1.85% | 0.0311 | 0.507 |
| anchor α=2.00 | 91.3% | 1.23% | 0.0197 | 0.663 |
| anchor + held kernel | 86.4% | 0.62% | 0.0126 | 0.766 |
| controlled negatives (retrained) | 70.8% | 0.62% | 0.0107 | 0.994 |

### What holds

**Definition composition is a large, real effect.** Analogy@1 goes 0.62% → 10.49%
and MRR ×11. The existing single-channel grounder moves neither. The limitation
was writing one channel of sixty-four, not the idea.

**The anchor is a clean dial, and there is no free lunch on it.** As α rises
0.25 → 2.0, dispersion recovers monotonically (0.369 → 0.663) and analogy MRR
falls monotonically (0.0833 → 0.0197). Best relation accuracy and best manifold
spread are at opposite ends. Retrofitting's trade-off is real here and the
project has to choose a point on it rather than hope for one that dominates.

### Three imported mechanisms that measured null or negative

* **Strong/weak split** (dict2vec): 0.0935 MRR versus 0.1177 for its own flat
  control. Loses. The gain in §4f was the reinforcement *rate*, not reciprocity.
* **Grounding-kernel scheduling** (Vincent-Lamarre): 0.0126 MRR, near baseline.
* **Controlled negative sampling** (dict2vec): pair/random +1.6pp, MRR ±0.0000,
  dispersion ±0.000. An honest null on this corpus.

The first two share one diagnosable cause. The definition graph comes out at
**47.5:1 weak:strong** against dict2vec's ~9:1, and the kernel at **49.6% of
entries** against the paper's ~10%. Both numbers say the same thing: Webster's
definer sets are inflated, because `clean_definition` strips brackets and
apparatus but not quoted usage examples and citations. Neither mechanism has
been given the graph it assumes, so neither is refuted — both are untested.

**Cleaning the source is therefore the highest-value next change**, and it
unblocks two mechanisms at once.

---

## §5 — Workstream A: the measurements, made trustworthy

### §5a — A1: the definitional core

`clean_definition` removed *apparatus*. It never removed *illustration*, which
is most of a Webster's entry. `definition_core` takes the first sentence of each
of the first three senses, cutting at Webster's own structural markers (`Note:`,
`as,`, `See`, `Cf.`, `Syn.`).

| quantity | before | after | target |
|---|---|---|---|
| mean definers per entry | 32.4 | **10.1** | — |
| grounding kernel | 30.5% | **9.9%** | ≤ 20% (literature ~10%) |
| entries surviving | — | **100%** | ≥ 90% |
| weak∶strong, raw | 50.5∶1 | 505∶1 | — |
| weak∶strong, after promotion | — | **6.1∶1** | ≤ 15∶1 (dict2vec ~9∶1) |

The kernel landing on the published ~10% is the strongest evidence the cleaning
is right rather than merely aggressive.

Raw reciprocity gets *worse*, and that is not a regression: a 10-word gloss is
far less likely to point back than a 32-word essay. Dict2vec never reached 9∶1
on reciprocity either — it concatenated four modern dictionaries and promoted
weak pairs whose words are among each other's K nearest (§3.1, K = 5).
`promote_neighbours` implements that, and takes 727 strong pairs to 38,157.

**Its first version promoted 0 of 271,300 pairs and passed its test.** It ranked
each word only among the words it defines, which makes "a is in b's top-k"
equivalent to "b already defines a" — logically incapable of promoting anything.
The neighbourhood is now undirected, and the test asserts promotion *can* create
a strong pair rather than only that it breaks nothing.

### §5b — A2: the probe set, and what it did to the headline

23 usable pairs across 3 families → **305 pairs across 10 families**, split
deliberately between semantic (gender, antonym, hypernym, nationality) and
morphological (number, comparative, past tense, agent, quality, negation),
because a manifold can learn the second from spelling while learning nothing
about meaning and an aggregate would hide that.

**The §4g headline did not survive the larger benchmark at its reported size.**

| metric | on 23 pairs | on 296 pairs |
|---|---|---|
| analogy@1, best condition | 10.49% | **1.55%** |
| analogy MRR, best condition | 0.1177 | **0.0267** |
| pair/random, best condition | 92.3% | **73.7%** |

The direction survives — MRR still rises ~100× from baseline, and every
composition condition beats the grounder — but the *magnitude* was inflated
roughly sevenfold by a benchmark too small to measure it. This is the stop
condition from the build order firing exactly as written, and the answer is
"survives, much smaller", not "refuted".

Per family, best condition:

| family | usable | pair/random | nbr@10 | analogy MRR |
|---|---|---|---|---|
| hypernym | 35 | 90.8% | 17.1% | 0.0285 |
| antonym | 35 | 87.4% | 11.4% | 0.0095 |
| quality | 25 | 85.4% | 4.0% | 0.0513 |
| negation | 23 | 83.9% | 26.1% | **0.0740** |
| gender | 25 | 78.3% | 4.0% | 0.0147 |
| nationality | 25 | 69.0% | **32.0%** | 0.0423 |
| past tense | 34 | 67.7% | 8.8% | 0.0136 |
| agent | 29 | 64.5% | 3.4% | 0.0214 |
| number | 35 | 63.1% | 5.7% | 0.0108 |
| comparative | 30 | 48.1% | 0.0% | 0.0011 |

The breakdown is the point. **Comparative is at chance** (48.1% pair/random is
a coin flip), while hypernym and antonym are strong — so the manifold is not
merely learning spelling. Negation and nationality lead on analogy, which is
what a phase-offset representation should be best at: both are near-uniform
transformations applied to a stem.

### §5c — A4: the latency claim, measured

69,786-word vocabulary, 1.06M n-gram entries, held-out words never seen in
training. p50 / p99, in microseconds:

| path | p50 | p99 | what it is |
|---|---|---|---|
| learn | **49 µs** | 301 µs | unseen word + definition composed into 64 channels |
| correct | **22 µs** | 95 µs | one fact overridden, logged and applied |
| unlearn | **0.05 µs** | 0.09 µs | prior phase restored — one struct write |
| recall | 5.9 ms | 7.3 ms | resonance against all 69,786 words, linear scan |

Unlearning is a single struct write because nothing else in the model encoded
the fact. That is the architectural claim, and it is now a number.

Recall is the outlier and the honest one: this is the full linear scan, which is
what is wired today. The sector index exists and is not on this path — making it
so is C1's job, and 5.9 ms is the baseline it has to beat.

**The comparison is stated, not measured.** Installing one fact by gradient
descent is a forward pass, a backward pass and an optimiser step over every
adapted parameter — conventionally hundreds of milliseconds to seconds even for
one LoRA step on a GPU, and one step rarely installs a fact. No such run was
performed here. Quoting a ratio against a number nobody ran is exactly the kind
of unmeasured assertion this harness exists to catch.

### §5d — A side effect worth more than the task that produced it

Expanding the probe set wedged the experiment, and the cause was `resonance`
calling `cos` **64 times per comparison**. Resonance is the innermost operation
in every retrieval, relation probe and analogy — one pass of the new benchmark
makes ~190 million of these calls.

Channels are quantised to one byte (`CHANNEL_QUANTA` = 256), so the angular
difference between two channels is always one of 256 values and its cosine is a
**table lookup on the byte difference**. Exact, not approximate: the
quantisation already happened when the phase was stored.
`test_resonance_matches_the_transcendental_form` asserts agreement to 1e-12, and
`test_cos_table_matches_cos` recomputes every entry so a generated table cannot
rot.

This makes every retrieval path in the engine faster, not just the benchmark.

### §5e — A3: the effect, with an error bar

Training became reproducible when the sample pool stopped depending on HashMap
order, but reproducible is only half of what a measurement needs. One
deterministic number has no error bar, and after A2 shrank the headline
sevenfold the remaining effects were small enough that the interval decides
them.

`Trainer::with_seed` mixes a seed into every stochastic decision. Two failure
modes are easy here and both are tested against: a seed that is accepted but
never mixed in gives identical runs and a *fake* error bar of zero, and a seed
that leaks into the corpus split varies the data, so the spread would measure
the split rather than the model.
`test_seed_varies_training_without_varying_the_data` asserts same-seed
reproduction, different-seed variation, and an unchanged vocabulary.

Five seeds, full retrain each, 296 usable pairs:

| seed | baseline MRR | composed MRR | pair/random |
|---|---|---|---|
| 100 | 0.0002 | 0.0278 | 64.9% → 74.5% |
| 101 | 0.0002 | 0.0216 | 63.6% → 74.2% |
| 102 | 0.0002 | 0.0279 | 63.5% → 74.1% |
| 103 | 0.0002 | 0.0280 | 64.7% → 73.9% |
| 104 | 0.0004 | 0.0294 | 63.4% → 73.6% |

| metric | baseline | composed |
|---|---|---|
| analogy MRR | 0.0002 ± 0.0001 | **0.0270 ± 0.0031** |
| pair/random | 64.0% ± 0.7 | **74.1% ± 0.3** |

**Composition clears its own noise by a wide margin on both metrics.** The
spread is tight — a standard deviation of 0.0031 on a mean of 0.0270, about 11%
— so the ~135× lift over baseline is not a seed artefact.

The separation test used is the crude one (means further apart than the two
spreads combined) and is labelled as crude in the output: it is not a t-test,
and n = 5 is small. It is enough to settle the question A2 raised.

### Workstream A, closed

| task | acceptance criterion | result |
|---|---|---|
| A1 | kernel ≤ 20%, weak∶strong ≤ 15∶1, ≥ 90% entries survive | 9.9%, 6.1∶1, 100% — **pass** |
| A2 | ≥ 300 pairs, ≥ 8 families, per-family reported | 305 pairs, 10 families, 296 usable — **pass** |
| A3 | `--seed`/`--runs`, all figures carry ± | 5 seeds, mean ± sd — **pass** |
| A4 | p50 and p99 for learn / unlearn / answer, stated baseline | 49 µs / 0.05 µs / 5.9 ms — **pass** |

The build order's stop condition on composition — *kill it if the analogy gain
does not survive a 300-pair benchmark with error bars* — is resolved:
**it survives**, at roughly a seventh of the magnitude first reported, and
clears its interval on five seeds.

B and C are now unblocked.

---

## §6 — Workstream B: moving the measured wins into the product

### §6a — B1: composition on the startup path, behind a guard

`model.rs` grounded startup phases with `DefinitionGrounder::ground_phases`,
which writes `theta(0)` and nothing else. `ground_best` replaces it with
`Conception::compose_anchored` across all 64 channels, at
`COMPOSITION_ANCHOR = 0.5` and `GROUND_BY_COMPOSITION = true` (both paths remain
reachable for one release).

**The guard is the load-bearing part.** Composition concentrates the manifold —
that is how it creates concept regions — and past a point concentration *is*
collapse, which is the failure the whole harness exists to detect. So the
composed facet is checked before it is kept: if phase dispersion falls below
`DISPERSION_FLOOR = 0.40`, it is discarded and the caller keeps what it had.

A guard that has never been observed to reject is a comment, not a guard, so
both paths are driven from tests:

* `test_dispersion_floor_rejects_and_preserves` composes every word toward one
  shared definer, confirms the fixture genuinely collapses, then asserts the
  guarded path rejects it **and leaves every channel of the caller's facet
  untouched** — not partially composed.
* `test_healthy_composition_is_accepted` confirms a spread-preserving
  composition is accepted and written through.

The floor of 0.40 is set from the anchor sweep, which ran from 0.54 at no anchor
to 0.81 at a strong one, and from the earlier unguarded rule that reached 0.305.

The second test needed its fixture rebuilt, which is worth recording: generated
names (`head00`…`head39`) share a long prefix and `SpectralPhasor::seeded` lands
them in a narrow band of channel 0 — the fixture started at dispersion 0.250 and
stayed there, so it was testing the seeding rather than the guard. Real words
fixed it.

### §6b — B3: the recurrent context, actually made default

The recurrent construction has been measured best of three since §4c and was
still not the default, because `PhianoLM::probability` takes a trigram and
structurally cannot carry a state across a sentence. `perplexity` now walks each
sentence maintaining the recurrent state; `perplexity_two_word` keeps the old
path as the comparison point.

Re-measured on `rust_book_corpus.txt` after the A-workstream changes:

| regime | context | swap cos | phase alone |
|---|---|---|---|
| co-occurrence | 2-word | 0.505 | 192.46 |
| co-occurrence | bound | −0.410 | 193.65 |
| co-occurrence | **recurrent** | n/a | **188.60** |
| ranking only | 2-word | 0.613 | 183.24 |
| ranking only | bound | −0.258 | 194.32 |
| ranking only | **recurrent** | n/a | **175.10** |

Unchanged in direction and magnitude from §4c, so the A-workstream changes did
not disturb it. γ\* is still 0 everywhere, and the γ = 0 column is identical
across all three constructions — asserted in
`test_default_context_is_recurrent_and_gamma_zero_is_untouched`, because if
context construction moved the no-phase baseline, every no-phase number in these
results would have silently shifted when B3 landed.

**What B3 does not do:** it does not move γ\*. The recurrent context makes the
phase distribution meaningfully better on its own (175.10 against 183.24) and
still loses to word frequency. That is the same conclusion §4b reached about the
non-linear readout, from a different direction.

---

## §7 — Is the sentence the unit of meaning?

Every measurement in this project so far has scored a **word**. Analogy is
`word:word::word:word`. Pair-versus-random is word against word. Perplexity is
the next word. The relation set is 305 word pairs. And γ\* = 0 has held across
seven independent attempts — all seven of them next-word prediction.

That is a gap in the measurement, not a settled result. If meaning is carried by
sentences and a word is a hair on the coat, a representation that compresses
*what comes next at the sentence level* could be doing its job while losing
every word-level benchmark ever run against it.

`cargo run --release --bin sentence`. Given 3 sentences of context, rank the
true continuation against 49 distractors drawn from the held-out half. Trained
on the first 80% in corpus order (the word-level harness shuffles, and a
next-sentence task needs contiguity). 970 items.

| scorer | top-1 | top-5 | MRR |
|---|---|---|---|
| chance | 2.0% | – | 0.0900 |
| phase (bag) | **8.7%** | **21.5%** | **0.1710** |
| phase (recurrent) | 5.2% | 17.3% | 0.1351 |
| phase (bound) | 2.7% | 11.4% | 0.1007 |
| **lexical overlap** | **27.9%** | **54.3%** | **0.4106** |

**Not supported, as tested.** Phase reaches 1.9× chance and loses to word
overlap by a wide margin. A representation that only partially recovers word
repetition has re-derived a bag of words in complex arithmetic; the gap to
*lexical*, not the gap to chance, was the claim.

### The finding underneath it

The first version of this benchmark had only the unordered encoder, and
reporting "phase loses to lexical" from that alone would have been a result
about the encoder rather than about the hypothesis: a bag of words in complex
arithmetic cannot beat a bag of words, and asking it to is asking nothing. Three
encodings were added so the question could actually be put.

The answer is the interesting part: **order makes it worse, monotonically.**
Bag 0.1710 → recurrent 0.1351 → bound 0.1007. And this is now the *third*
independent place the same thing has happened:

| where | unordered | ordered |
|---|---|---|
| inside a definition (§4d) | 81.9% pair/random | 37.7% bound |
| two-word LM context (§4c) | 183.24 ppl | 194.32 bound |
| sentence composition (§7) | 0.1710 MRR | 0.1007 bound |

Three separate tasks, three separate scales, same direction. Rotating by
position times the golden angle is a **faithful** order encoding — the
swap-cosine goes negative, so the representation genuinely distinguishes
"dog bites man" from "man bites dog" — and it is a **destructive** one, because
it scatters the same word to a different angle in every context it appears in,
which is precisely what a shared representation must not do.

That is a finding about the binding operator, not about order. It is also the
sharpest argument yet for a *learned* rotation: §4c already noted the recurrent
kernel (λ, ω tuned rather than fixed) beats the golden angle at word level, and
every rotation in this codebase is currently a compile-time constant.

### What this does not test

Two things, and the second is the one worth building.

* **Role structure, as opposed to position.** *Money is a form of currency that
  enables transactions; usually a paper or coin representation* carries
  `genus(money, currency)`, `function(money, transaction)`,
  `form(money, paper)`. Those are **typed relations**, and the type is not the
  token's position in the sentence — *currency* is the genus whether it appears
  third or thirtieth. Everything measured above encodes position. Role-filler
  binding is already implemented (`SpectralPhasor::bind`/`unbind`,
  `Wave::proposition`/`query_role`) and is not on any measured path.
* **A learned kernel.** Fixed constants cannot adapt the rotation to the
  relation it is meant to carry. Making the rotation differentiable is what
  would let the manifold discover its own binding operator rather than being
  handed one.
