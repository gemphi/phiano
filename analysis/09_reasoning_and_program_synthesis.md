# 09 — Reasoning Engines and Program Synthesis

> Files examined: [`src/reasoning/`](../src/reasoning) (all 12 modules),
> [`src/synthesis/`](../src/synthesis) (4 modules), [`src/lifelong/`](../src/lifelong),
> [`data/arc_tasks.json`](../data/arc_tasks.json), [`src/metrics/arc.rs`](../src/metrics/arc.rs),
> [`specs/005_program_synthesis_lifelong_learning.md`](../specs/005_program_synthesis_lifelong_learning.md).

---

## 1. The Reasoning Family: One Algorithm, Seven Costumes

Auditing all of [`src/reasoning/`](../src/reasoning), every engine reduces to a
variation of **greedy nearest-neighbor walk on the phase circle**:

| Engine | Actual algorithm |
|---|---|
| [`pathfinding.rs`](../src/reasoning/pathfinding.rs) | From context phase, repeatedly jump to the nearest unvisited lexicon word; stop when phase shift < 0.01 |
| [`planning.rs`](../src/reasoning/planning.rs) | Same walk, but target = `current + 0.3·(goal − current)` (interpolated attraction) |
| [`multi_path.rs`](../src/reasoning/multi_path.rs) | Run the walk from 1–8 evenly-spaced phase offsets (2π/n), keep the highest-confidence chain; effort levels cap steps (1/4/16/32/64) |
| [`hybrid.rs`](../src/reasoning/hybrid.rs) | pathfinding + top-5 phase analogies per token; confidence 0.8/0.4 × analogy boost |
| [`analogy.rs`](../src/reasoning/analogy.rs) | A:B similarity = `1 − |Δφ|/π`; `find` = top-n phase neighbors |
| [`program_analogy.rs`](../src/reasoning/program_analogy.rs) | compares *sequences of phase deltas + sector transitions* — structural (relational) analogy, the most interesting variant |
| [`counterfactual.rs`](../src/reasoning/counterfactual.rs) | clone facet, overwrite phases of premise words with counterfactual phases, re-eval → coherence delta |
| [`abstraction.rs`](../src/reasoning/abstraction.rs) | circular centroid + phase deltas common to all examples (within 0.15 rad) |
| [`sorting.rs`](../src/reasoning/sorting.rs), [`comparison.rs`](../src/reasoning/comparison.rs), [`diagnostics.rs`](../src/reasoning/diagnostics.rs) | utilities: phase sort, engine bake-off, convergence mode |

**What this genuinely is:** *associative traversal over learned memory* — the phase
manifold as a semantic graph, walked by proximity. Given a question, the system
retrieves the concepts nearest the question's phase signature and orders them into a
chain. This is a legitimate, explainable retrieval-and-chain primitive (a cousin of
 spreading-activation in classic semantic networks).

**What it is not:** deduction, planning over world models, or compositional inference.
No engine performs multi-step logical operations whose *conclusion* is entailed by
premises; each step is independent proximity matching. Confidence values (0.8/0.4)
are constants, not calibrated probabilities. "Reasoning" here = "associative recall
choreographed to look like thought."

**The exception worth developing:** [`program_analogy`](../src/reasoning/program_analogy.rs)
compares *relations between relations* (phase-delta sequences), which is the correct
structural shape for analogy ( Gentner-style relational matching). It is 40 lines from
being a publishable experiment on ARC-style relational transfer (file 16, task 5).

## 2. Program Synthesis: A Truthful Micro-DSL

[`ProgramOp`](../src/synthesis/program.rs): `{Map, Filter, Reduce, Compose, Sort, Reverse, Identity}`.

- **Executed for real:** `Sort`, `Reverse`, `Identity`, `Filter(word)` (equality filter).
- **No-ops in the interpreter:** `Map`, `Reduce`, `Compose` — matched by a catch-all
  that clones tokens unchanged ([program.rs:34–55](../src/synthesis/program.rs)).
  The advertised "beam search" is actually **full enumeration** of 3^depth op sequences
  ([search.rs:11–48](../src/synthesis/search.rs)), scored by token-overlap similarity.
- [`heuristic.rs`](../src/synthesis/heuristic.rs) (phase-signature structure guessing)
  exists but is **not wired into** the synthesizer.
- [`library.rs`](../src/synthesis/library.rs): a `ComponentLibrary` storing named
  programs with phase signatures and `reuse_count` — retrieval by phase-signature
  similarity > 0.6. The germ of a real *learned-program* memory.

**Verdict:** a truthfully small program-induction scaffold — 4 real ops, enumeration
search, string-overlap fitness. As specified in specs/005 it promises ARC-scale
induction; as implemented it can induce "sort this", "reverse that", "drop word X".
The ComponentLibrary is the piece with an actual future (file 16, task 5).

## 3. Lifelong Learning Module

[`src/lifelong/`](../src/lifelong): `history.rs` (task log), `meta.rs` (skill metadata),
`monitor.rs` (regression watch), `reuse.rs` (component transfer). Integration test
[`test_lifelong_learning_and_transfer`](../tests/ch14_integration.rs) passes: a
learned component is stored, retrieved, and reused on a similar task. This is
engineering-grade transfer *infrastructure* around the synthesis library — the
scaffolding for lifelong program induction, waiting for the DSL to grow into it.

## 4. The ARC Benchmark: What "Evaluation" Means Here

[`data/arc_tasks.json`](../data/arc_tasks.json): 20 hand-written toy tasks (10 analogy,
5 pattern, 5 transform). [`ArcBenchmark::evaluate`](../src/metrics/arc.rs) trains
input→output pairs as sentences, then marks a task "correct" if
`coherence > 0.5` **and** a generic prediction string ("{input} relates to the pattern")
contains the first token of the expected output.

**Verdict, stated plainly:** this does not evaluate ARC-style reasoning. It evaluates
whether training aligned the phases (tautology risk, file 14 §3) and whether the
expected answer's first word appears in a canned string. Real ARC evaluation would
require executing an induced transformation on held-out grids. The benchmark *harness*
is fine; the *task set and success criterion* are self-graded. Fixing this (real
ARC-format grids, exact-output match) is file 16, task 1 — the cheapest credibility
upgrade available.

## 5. Scorecard

| Subsystem | As advertised | As implemented | Grade |
|---|---|---|---|
| Reasoning engines | "Phase pathfinding, planning, hybrid reasoning" | proximity walks + analogy scoring | C+ (solid retrieval theater) |
| Program analogy | structural analogy | phase-delta sequence matching | **B — the diamond** |
| Program synthesis | "AST synthesis & beam search" | 4-op enumeration, 3 no-op ops | C− |
| Component library | "Persistent ComponentLibrary" | working store + reuse + transfer test | B− |
| Lifelong learner | "LifelongLearner" | working bookkeeping around the library | B− |
| ARC benchmark | "ARC evaluation" | coherence + first-token match on 20 toy tasks | F (as ARC), C (as harness) |

**Bottom line:** the reasoning stack is associative retrieval wearing seven hats, and
that is *okay* — explainable retrieval is exactly what a phase memory is for — but two
genuine seeds deserve water: relational (program) analogy, and the component library
as a lifelong program memory. Everything else should either be re-scoped honestly or
connected to real evaluation.
