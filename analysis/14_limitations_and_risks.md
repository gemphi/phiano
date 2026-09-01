# 14 — Limitations, Risks, and Honest Gaps

> This file consolidates every limitation found across the codebase audit, ranked by
> how much it costs the project's credibility or capability. Nothing here is
> speculative: each item cites its location and effect. Pair with
> [16_roadmap_to_power.md](16_roadmap_to_power.md), which converts the top items into
> concrete work.

---

## 1. Structural Limitations (physics of the architecture)

### 1.1 One-dimensional representational capacity — *the* ceiling
The semantic channel is a single phase (+ sub-band + amplitude). Distinguishable
clusters saturate around 10⁴–10⁵ words (file 12 §2). Every downstream ambition —
relations, reasoning, universal learning — is gated on dimensionality.

### 1.2 No credit assignment
Learning updates come from immediate co-occurrence only; no signal propagates through
a generation chain, a correction's *cause*, or a plan's outcome. Multi-step competence
cannot improve by design (file 12 §3.2).

### 1.3 Order-blind superposition
The sentence wave is a commutative sum; all syntax lives in external tables
(file 12 §3.1). The "sentence is a chord" metaphor is accurate — and chords don't
encode order.

### 1.4 Fixed coupling constants
0.7/0.3 semantic-syntax mix, 0.35 phase kick, 0.85/0.15 momentum EMA, π/16 default β —
all hard-coded ([constants.rs](../src/config/constants.rs)). The system's *dynamics*
never learn; only its state does (file 03 §3). The β_ij EMA is the lone exception and
proves the exception is possible.

### 1.5 Soft associative forgetting — real and unmeasured
Old associations drift as words reappear in new contexts (file 10 §3). The docs claim
"zero catastrophic forgetting"; the truth is *graceful unmeasured drift*. Until a
retention benchmark exists, the flagship superpower is scientifically unvalidated.

## 2. Engineering Gaps (fixable, ranked by leverage)

| # | Gap | Cost today | Fix scale |
|---|---|---|---|
| 1 | Memo never recalled into generation (write-only memory) | No episodic continuity; file 05 §4 | Small — recall top-k by context-wave resonance |
| 2 | 92 MB state: 98% n-gram freight, 2% model | Footprint claims false; load times; file 05 §3 | Medium — string interning + u32 ids |
| 3 | No trigram-only ablation of generation | Phase contribution to output *unquantified*; file 08 §7 | Small — one flag + one benchmark |
| 4 | Spider-net discourse prior unused by generation | No multi-sentence planning; file 08 §6 | Small-medium — wire `type_links` as planner |
| 5 | `attention_pick` dead code; Map/Reduce/Compose no-ops | Advertised modules that do nothing; files 07 §5, 09 §2 | Small — implement or re-scope |
| 6 | SSE streaming is pre-generated replay | Fake incrementalism; file 08 §5 | Medium — async decode |
| 7 | Quadratic paths: O(V²) oscillator training, O(|prompt|·V) Aboutness | Scaling walls at 10⁵ words; file 11 §3 | Medium — batching/indexing |
| 8 | ASCII-English tokenizer; suffix-rule POS tagging | Multilingual and parsing claims unsupported | Large — out of P0 scope, say so |

## 3. Evaluation Honesty Problems (the credibility tax)

### 3.1 The coherence tautology
Training *is* phase alignment (file 03); "coherence" *measures* phase alignment
([eval.rs:144](../src/eval.rs): `r = ‖ΣZ‖/N`). The benchmark headline "coherence 1.0000
on trained prompts" (docs/rust_mastery/06) is therefore close to a restatement that
training happened. It is *not* evidence of understanding — in-domain trigram models
also score perfectly on perplexity-style self-consistency. External, held-out,
adversarial metrics (perplexity on unseen text, exact-match QA, real ARC) are the
remedy (file 16, task 1).

### 3.2 The ARC benchmark grades itself
20 hand-written tasks; success = coherence + first-token substring match
(file 09 §4). As "ARC" this is a category error; as a smoke test it's fine — but the
labeling invites exactly the scrutiny this analysis applies.

### 3.3 Terminology inflation (systemic)
"Engines" for hash buckets (file 06 §1), "topology" for a permutation table,
"intentionality implemented" for keyword matching (file 07 §2), "Bloch sphere" for a
plain spherical projection, "multi-head attention" with zero learned weights,
"~2/5/12 MB" footprints that presume an unbuilt refactor (file 11 §4). Each individual
overstatement is small; their accumulation is the project's largest *external*
risk — a serious reader who catches one stops reading. **Precision of naming is now a
strategic asset.**

## 4. Operational Risks (if deployed)

| Risk | Severity | Note |
|---|---|---|
| No auth on the 31-endpoint API | High | [`src/server/`](../src/server) binds without auth/CORS policy; `/api/learn` mutates state |
| Unbounded growth of n-gram tables | Medium | Every sentence inserts rows forever; no pruning/decay of counts exists |
| Unversioned binary format | Medium | Format change silently invalidates `manifold.chroma` |
| Live Wikipedia fetch in chat path | Medium | 5 s timeout blocks the turn; offline story breaks; HTML parsing fragility |
| Determinism *requires* identical input order | Low | HashMap iteration is unordered; cross-run outputs can differ in tie-breaking unless lexicon ops are ordered |
| Correction pulse can be abused | Low | "no, X means Y" repels phases of X's tokens — a user (or prompt injection) can vandalize the manifold through natural chat; no rollback/versioning exists |

## 5. The Grand-Claim Risks (what not to claim, and what to claim instead)

| Don't claim (not supported) | Claim instead (supported) |
|---|---|
| "Solves the symbol grounding problem" | "Lexical similarity emerges from learned phase geometry — grounded *within* the lexicon" |
| "64 ways to look at anything" | "64 deterministic feature buckets per structural unit" |
| "Zero catastrophic forgetting" | "Vocabulary is never displaced; associations drift measurably (benchmark pending)" |
| "The PyTorch of oscillators" | "A complete, tested Rust testbed for oscillator-based NLP research" |
| "LLM-level capability at 1/1000 the size" | "LLM-adjacent *memory and learning* at 1/1000 the size, with trigram-class generation" |
| "Implementations of Searlean intentionality" | "A rule-based Searle *taxonomy* classifier with phase-geometric aboutness scoring" |

## 6. Scorecard

| Category | Grade |
|---|---|
| Structural limits acknowledged in docs | D (docs largely assert the opposite) |
| Engineering gaps (fixable) | B− (few root-cause-hard problems; mostly wiring) |
| Evaluation rigor | C− (rich harness, self-referential metrics, one ablation missing) |
| Operational hardening | C (works locally; not production-surface ready) |
| Claim vs. code alignment | **D+ — the single largest fixable reputational liability** |

**Bottom line:** none of the structural limitations is fatal, and almost every
engineering gap is wiring rather than invention. The one *dangerous* limitation is
narrative: the gap between what the documentation says and what the code does is
wider than the gap between the code and its real competitors. Closing the naming gap
costs nothing and raises the project's standing with exactly the audience
(researchers, engineers, investors) whose attention it deserves.
