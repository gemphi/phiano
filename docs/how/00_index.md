# The HOW Series — How Phiano Learns

> Sixteen documents tracing every mechanism by which a word becomes a number,
> a number moves, and movement becomes knowledge. Every claim in this series is
> traced to a line of source. Every example is computed, not asserted.

---

## Why this series exists

Most of `docs/` describes **what** Phiano is: a phase manifold, a spider-net, a
32-core topology. Very little of it describes **how** a specific token's phase
angle changes when a specific sentence arrives, and **why** that change counts as
learning.

That gap matters, because the difference between a learning system and a
clustering system lives entirely in the update rule. This series closes the gap.

Each document follows the same shape:

| Section | What it gives you |
|:---|:---|
| **Mechanism** | The rule, stated as maths |
| **In the source** | Exact file and function |
| **Worked example** | Real numbers, hand-computed, reproducible |
| **What this buys** | The capability it genuinely unlocks |
| **The ceiling** | What this mechanism structurally cannot do |
| **How it generalises** | The concrete change that lifts the ceiling |

The last two sections are the point. A mechanism you understand the limits of is
a mechanism you can extend. A mechanism you only celebrate is a mechanism you are
stuck with.

---

## The sixteen

### Part I — Representation (how a word becomes a number)

| # | Document | Core question |
|:--|:---|:---|
| [01](01_word_to_number.md) | **Word → Phasor** | What exactly is stored for one word, and how much can 16 bytes hold? |
| [02](02_the_kuramoto_step.md) | **The Kuramoto Step** | What happens to five phase angles when one sentence arrives? |
| [03](03_learning_word_order.md) | **Learning Word Order** | How does an undirected circle encode "subject before verb"? |
| [04](04_cooccurrence_memory.md) | **Co-occurrence Memory** | Where does the actual fluency come from? |

### Part II — Grounding (how a number acquires meaning)

| # | Document | Core question |
|:--|:---|:---|
| [05](05_definition_grounding.md) | **Definition Grounding** | How does a word get a position that means something? |
| [06](06_sentence_superposition.md) | **Sentence Superposition** | How do many phasors become one wave — and what is lost? |
| [07](07_ray_casting.md) | **Ray Casting** | How does the model retrieve, and what is the geometry of retrieval? |
| [08](08_self_scoring.md) | **Self-Scoring** | How does the model grade itself, and does the grade mean anything? |

### Part III — The loop (how learning becomes continuous)

| # | Document | Core question |
|:--|:---|:---|
| [09](09_envision_curiosity.md) | **Envision** | How does it notice what it does not know? |
| [10](10_anti_phase_correction.md) | **Anti-Phase Correction** | How is a mistake unlearned in one step? |
| [11](11_generation.md) | **Generation** | How is a token chosen? |
| [12](12_memory_layers.md) | **Memory Layers** | How is experience stratified? |

### Part IV — Scale and generality (how it becomes powerful)

| # | Document | Core question |
|:--|:---|:---|
| [13](13_persistence_and_cost.md) | **Persistence & Cost** | What does this actually cost, per word, per second, per watt? |
| [14](14_lifelong_transfer.md) | **Lifelong Transfer** | How does knowledge move between tasks? |
| [15](15_proving_it_works.md) | **Proving It Works** | What experiment would falsify the design? |
| [16](16_learning_anything.md) | **Learning Anything** | The four missing properties, and the change each one needs. |

---

## The one-paragraph summary

Phiano stores each word as a single angle on a circle. Training rotates
co-occurring words toward their shared centre of mass, records which words follow
which, and grows an amplitude that stands for familiarity. Retrieval projects a
query onto the same circle and ranks by squared distance. This is a real,
working, unusually cheap online learner — it updates in one pass, forgets nothing
by gradient interference, costs 16 bytes per word, and can be read by a human at
any moment. It is also, in its current form, a **one-dimensional** learner with a
**collapsing** objective and an **order-blind** composition operator, which are
three separate reasons it cannot yet learn arbitrary structure.

Documents 01–14 explain the machine as built. Document 15 explains how to find
out whether it works. Document 16 explains, precisely and constructively, what to
change so that "learns anything" becomes a claim the architecture can carry.

---

## Reading paths

- **New to the codebase**: 01 → 02 → 04 → 11. Ninety minutes, and you can trace a
  sentence end to end.
- **Evaluating the design**: 08 → 15 → 16. This is the critical path.
- **Extending it**: 16 first, then 01, 03 and 06 for the three highest-leverage
  edits.

### Results

| # | Document | What it is |
|:--|:---|:---|
| [RESULTS](RESULTS.md) | **Measured Results** | The first harness run: real held-out perplexity, the mixing sweep that answers whether the phase manifold contributes, and the collapse diagnostics. |

### Appendix

| # | Document | Core question |
|:--|:---|:---|
| [A1](A1_better_worse_lineage.md) | **The `better`/`worse` Lineage** | Barbara Huberman's 1968 Stanford chess-endgame thesis invented `better`/`worse` as paired predicates. What her design has that Phiano's compose tournament dropped. |

---

## Companion

- [`../EVALUATION.md`](../EVALUATION.md) — the verdict: what is real, what is
  overstated, and the ranked list of changes.
