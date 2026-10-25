# 12 — Could It Learn Anything? A Capacity and Universality Analysis

> This file is the theoretical core of the whole analysis: whether the phase-manifold
> architecture family — not just today's code — contains, in principle, the mechanisms
> needed to learn arbitrary domains, and what specifically separates today's Phiano
> from that ceiling. Everything here is grounded either in the code or in established
> results about the referenced mechanism classes.

---

## 1. Defining "Learning Anything" — Three Levels

| Level | Meaning | Example |
|---|---|---|
| **L1 — Any vocabulary/domain** | Absorb words, entities, jargon of any field; place them in meaningful similarity geometry | Learn cardiology terms from a dictionary |
| **L2 — Any relation/structure** | Learn *compositional* structure: grammar, relational schemas, programs, procedures | Learn that "X causes Y" entails testable structure; learn a sorting procedure |
| **L3 — Any task/competence** | Improve at multi-step skills with feedback: reasoning chains, tool use, planning under objectives | Answer novel questions correctly; write working code |

"Powerful" tracks the level reached. Today's Phiano: **solid at L1 (bounded), germinating
L2, essentially absent L3.** The rest of this file explains why — and why that is not a
terminal verdict on the architecture family.

## 2. Level 1 Audit: What the Current Math Already Supports

The learning rule (file 03) is domain-agnostic: it never inspects content, only
co-occurrence. Therefore, *by construction*, it can absorb any domain's vocabulary —
Rust jargon, Webster's English, Phi-4 tiktoken merges, Wikipedia proper nouns — and it
demonstrably does ([`src/sources/`](../src/sources)). Three structural properties make
L1 robust:

1. **Open vocabulary** — new words allocate new state; nothing is displaced.
2. **Domain-agnostic clustering** — circular-mean pull works identically on any corpus.
3. **Definition grounding** — dictionary self-study bootstraps placement of unknowns.

**The L1 ceiling is capacity, and it is quantifiable:**

- The semantic channel is one phase on a circle (+16 sub-bands + amplitude).
- Learning noise floor: per-step pull ≤ lr = 0.05 rad; convergence threshold 5·10⁻⁴ rad.
- A word's resting phase under diverse usage is a *distribution* (circular variance σ),
  not a point. Two words are reliably distinguishable only if their cluster means
  exceed ~2σ of drift.
- Empirically the benchmark run holds **6,993 words** with usable geometry in 27 MB.
  Extrapolating, with band separation ×16 and the 32-harmonic torus for *measurement*
  (not storage), the architecture as coded saturates in the **10⁴–10⁵ words** range —
  after which new words land in occupied phase neighborhoods and similarity degrades
  toward hash noise. This is a *representational* bound, not an engineering one.

## 3. The Three Things That Block L2 and L3

### 3.1 Compositionality: superposition is order-blind
The sentence wave `Ψ = Σ λᵐ Z_m` ([docs/45 §2.2](../docs/45_native_learning_vs_bloated_llms.md))
is a **commutative sum**: "dog bites man" and "man bites dog" produce identical waves.
All word-order knowledge therefore lives outside the manifold — in n-gram counts and
β_ij lags. The manifold encodes *what* a sentence is about; the tables encode *how*
words order. L2 requires order inside the representation itself.

### 3.2 Credit assignment: learning is single-step, generation is multi-step
`train_sentence` updates each token from its *immediate* sentence context. When the
system generates 20 tokens and the user corrects the *claim*, the correction
([`correct_mistake`](../src/trainer/mod.rs)) repels the wrong words' phases — but there
is no mechanism to propagate blame through the *chain* of choices that produced the
wrong sentence. No loss function exists over generated sequences, so multi-step
behavior can only improve by luck of retraining, never by signal. **This — not
parameter count — is the deepest gap between Phiano and learning anything at L3.**

### 3.3 Verification: the system cannot check itself against the world
Coherence/novelty/resonance (file 04 §2.3) measure *internal consistency*. A
confidently wrong statement coheres perfectly with wrong phases. L3 requires feedback
signals that are (at least partly) **external**: execution results, user acceptance,
held-out prediction accuracy. The compose tournament (file 08 §4) is the only external-
ish loop in the codebase — and it is the one place behavior measurably improves by
selection. That is not a coincidence; it is the architecture's only existing credit-
assignment channel.

## 4. Why the Ceiling Is Not Terminal: The Established Relatives That Broke Through

The decisive fact for this architecture family: **every one of the three blockers has
been solved in a sibling mechanism class, without abandoning phase/complex geometry.**

| Blocker | Solved by | Known result |
|---|---|---|
| Capacity (1-D phases) | **Multi-dimensional complex embeddings** | ComplEx (2016) / **RotatE (2019)** model relations as rotations in ℂ^d; they learn millions of triples (FB15k-237, WordNet) at near-SOTA link-prediction accuracy. Phiano is RotatE with d = 1 and online updates instead of SGD |
| Order-blindness | **Binding operations** in vector symbolic architectures | Plate's HRR circular-convolution binding, Kanerva's HDC permutation binding — encode *structured, ordered* representations in superposed vectors. Phiano's β_ij lag is a primitive scalar binding of exactly this type |
| Credit assignment | **Contrastive negative sampling + readout training** | RotatE/ComplEx train against corrupted negatives; reservoir computing (fixed dynamics + *trained readout*) gets universal approximation properties from exactly Phiano's "fixed dynamics, adaptive state" shape — by adding a thin learned layer on top |
| Verification | **Execution feedback** | Program synthesis (DreamCoder-class systems) verifies induced programs by running them |

**Synthesis of §4:** Phiano's family is one dimensionality increase, one binding
operator, one contrastive signal, and one trained readout away from mechanisms with
published, million-scale results. None of these requires inventing new physics; all
four have precise, small-surface code landing zones in this repository (file 16 maps
them).

## 5. The Formalist's Summary

- Phiano today = **a fixed dynamical system with adaptive state** (file 03 §3).
  Such systems are memory-complete (they can store unboundedly many distinct states)
  but **computationally rigid**: their input→output function cannot improve, only
  their state can. That is why L1 is strong and L3 is absent.
- The universal-learning threshold requires an **adaptive dynamical system**: some
  parameters of the *coupling itself* must be state-updated. The repository already
  contains the first true instance — the learned β_ij lags — proving the codebase can
  host adaptive coupling; they are simply the only learned dynamical parameters so far.
- Coupled-oscillator networks are known to be computationally expressive (oscillator
  phase logic can implement switching and gating; Kuramoto lattices support universal
  dynamical behaviors). The physics does not forbid power; the current *fixed* wiring
  forbids it.

## 6. What "Powerful" Would Look Like, Stage by Stage

| Stage | Upgrade (from file 16) | Predictable capability |
|---|---|---|
| P0 (today) | — | 10⁴-word online semantic memory, µs learning, 20-token n-gram+phase generation |
| P1 | d = 32–256 phase dims per word (rotor vectors) | 10⁵–10⁶ words with clean geometry — vocabulary of a fluent speaker; knowledge-graph-scale relation storage |
| P2 | binding via phase rotation (β generalized to per-relation lags); negative sampling | ordered structure in the manifold; true relational learning (kinship, causality schemas); generation weaned off n-grams |
| P3 | trained readout + execution feedback loop | calibrated correctness; multi-step skill improvement; honest ARC-class results |
| P4 | coupling constants learned (meta-plasticity) | the system tunes its own learner — the actual "self-improving" threshold |

## 7. Verdict

**Can Phiano-as-coded learn anything? No — and it was never going to: a one-dimensional
phase circle with a fixed update rule is a lexical memory, not a universal learner.**
The honest statement of its achievement is: *a complete, working, tested implementation
of the online-learning regime that transformers lack, with a representation whose
sibling classes (complex embeddings, VSA binding, reservoir readouts) have each
independently demonstrated the missing capabilities at scale.*

The family's path to "learn anything" is therefore **not speculative — it is
compositional**: four known mechanisms, each with a small landing zone in this
codebase, each converting one blocker. Whether Phiano walks that path is now purely an
engineering-program question, which file 16 turns into a task list.
