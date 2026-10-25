# 06 — Phinum Engines, I Ching, and the Spider-Net: What They Actually Compute

> Files examined: [`src/phinum/variants.rs`](../src/phinum/variants.rs),
> [`src/phinum/config.rs`](../src/phinum/config.rs), [`src/phinum/lexicon.rs`](../src/phinum/lexicon.rs),
> [`src/phinum/iching/`](../src/phinum/iching), [`src/phinum/syntax/`](../src/phinum/syntax),
> [`src/phinum/topology/`](../src/phinum/topology), [`PLAN.md`](../PLAN.md).

The README and PLAN.md present Phinum16/32/64, the 64 hexagrams, and the spider-net as
the system's "topological engine." This file states, without metaphor, what each layer
is as code — because the honest answer is important for everything downstream.

---

## 1. Phinum16 / 32 / 64: Deterministic Hash Buckets

[`variants.rs:90–135`](../src/phinum/variants.rs):

```text
hash(s)  = fold over bytes: acc = acc·31 + byte       (config.rs:76–80)
classify = hash & (SLOTS−1) % SLOTS   →  SLOTS ∈ {16, 32, 64}
```

A word's "variation" is **which bucket its FNV-style hash lands in**. The three engines
are the same operation at three granularities. There is no training, no learned
representation, and no information beyond the hash.

**What this is good for (real):** deterministic, zero-storage, O(1) *labeling* — every
word/sentence/paragraph gets a stable bucket id at 16/32/64 granularity, useful as
feature keys, routing hints, and visualization axes.

**What it is not:** "16/32/64 harmonic perspectives on meaning." Two synonyms do not
necessarily share a bucket; two unrelated words may. The buckets inherit no semantics
from the phase manifold — the hash does not read the phasor at all.

## 2. I Ching 64-Hexagram Layer: hash % 64 + a Permutation Table

[`iching/mod.rs:47–53`](../src/phinum/iching/mod.rs): a syntax key string is FNV-1a
hashed and reduced `mod 64` to a 6-bit hexagram id; lower/upper trigrams are the id's
bit halves; `KING_WEN_MAP` ([table.rs](../src/phinum/iching/table.rs)) relabels ids into
the traditional 1–64 sequence; a rule switch maps upper trigram → Searle speech act
([iching/mod.rs:73–81](../src/phinum/iching/mod.rs)).

**Honest verdict:** an aesthetic labeling scheme over `hash % 64`. The I Ching's
conceptual system (64 archetype states) supplies *names and imagery*, not computation.
Nothing is learned, and the mapping syntax→hexagram→speech-act is a fixed hash chain.
It is charming visualization metadata — and should be presented as such.

## 3. Syntax Layer: Word Lists + Suffix Heuristics

[`syntax/parser.rs`](../src/phinum/syntax/parser.rs), [`syntax/dictionary.rs`](../src/phinum/syntax/dictionary.rs):

- POS tagging = lookup in a hard-coded closed-class dictionary (pronouns, determiners,
  auxiliaries, prepositions, ~a few hundred words) + morphological fallback:
  ends in `-ing`/`-ed`/`-s` (length > 3) ⇒ Verb, else Noun
  ([dictionary.rs:75–87](../src/phinum/syntax/dictionary.rs)).
- `SyntaxKey` = POS short codes joined by `+` (e.g., `PRON+V+PART+V+PRON`).

**Verdict:** a defensible lightweight POS guesser for English closed classes; the
suffix rule misclassifies systematically (nouns ending in -s/-ing: "business",
"building"). Unknown open-class words default to Noun. Fine as a feature extractor;
not a parser. There is no dependency grammar, no clause lattice (module names in
PLAN.md promise `clause_graph`, `dependency tree lattice` — not implemented).

## 4. The Spider-Net: Co-occurrence Transition Counters

[`topology/spider_net.rs`](../src/phinum/topology/spider_net.rs) accumulates, while
ingesting text:

- sentence-type and paragraph-type occurrence counts (per v64 bucket),
- transition counts between adjacent sentence types and adjacent hexagram ids,
- POS-bigram "shape" relation counts,
- many-to-many indexes syntax-key ↔ hexagram.

`SentenceType::classify` ([sentence.rs:34–52](../src/phinum/topology/sentence.rs)) is
punctuation/keyword rules (?, !, verb-initial, speech-act markers);
`ParagraphType` ([paragraph.rs:40–58](../src/phinum/topology/paragraph.rs)) is majority
vote over sentence types. `spin_structure` rotates a stored key's hexagram by a phase
delta and retrieves keys mapping to the rotated hexagram — a nearest-neighbor query
over the accumulated index, and the layer's most interesting operation.

**Verdict:** this *is* a legitimate "structural memory" — a learned transition model
over discrete structural types, i.e., an n-gram model **of syntax shapes instead of
words**. "Zero raw-example storage" is accurate: only type transitions are kept, and
instances are re-instantiated at generation time. As a discourse-planning prior it has
real value (file 08 §6 uses it); its weakness is that everything upstream (types,
hexagrams, POS) is rule/hash-derived, so errors compound unmeasured.

## 5. The Tier Map: Promised vs. Implemented (PLAN.md Modules 9–16)

| PLAN module | Status in code |
|---|---|
| 09 pos_tagger | **Partial** — closed-class + suffix rules |
| 10 syntax_net (key extractor) | **Working** — POS-code keys |
| 11 clause_graph (dependency lattice) | **Not implemented** |
| 12 sentence_type | **Working** — rule classifier + hash buckets |
| 13 paragraph_type | **Working** — majority vote |
| 14 structural_keys (invariant hasher) | **Working** — FNV keys |
| 15 variation_gen (16/32/64 perspectives) | **Working as hash buckets** — but see §1 |
| 16 spider_net | **Working** — transition counters |

## 6. Scorecard

| Layer | As advertised | As implemented | Useful? |
|---|---|---|---|
| Phinum16/32/64 | "Harmonic multi-resolution engines" | hash & mask into 16/32/64 buckets | As cheap deterministic feature keys — yes |
| I Ching 64 hexagrams | "Topological phase manifold" | `hash % 64` + King Wen permutation + rule switch | Visualization/naming only |
| Syntax keys | "Structural spider-net keys" | POS-list + suffix POS codes | Lightweight, brittle |
| Spider-net | "Zero-storage topological graph" | transition counts over structural types | **Genuinely promising** as a discourse prior |

**Bottom line:** one of the four layers (the spider-net's structural transition memory)
is real learned structure; the other three are deterministic labeling of varying
cleverness. The strategic risk is **terminology inflation**: naming hash buckets
"engines" and permutations "topology" invites dismissing the whole project — including
the genuinely original phase-learning core — upon inspection. Renaming and re-scoping
these layers (file 16, task 9) is a credibility fix, not merely cosmetic.
