# 199 · The architecture of BCPL, and how each bit is starting to serve the model

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Read from the distribution in `gemphi/ext/bcpl/cintcode` — file and line cited for every claim — not from memory of BCPL. The user's ruling that framed the reading: a group selected by a band is a KIND in a hierarchy; within a kind, defined metrics still differentiate; a kind defines its enums so one can drill down; at every level there are types; and it is a TYPE in play, not a thing. BCPL already carries all of it.

## The architecture, from the files

```mermaid
graph LR
    src["source .b"] --> LEX
    subgraph bcplfe.b — the front end, 4,085 lines
        LEX["LEX  lex() @616\nchars → tokens\nname table: lookupword, declsyswords"]
        SYN["SYN  rdblockbody() @1620\ntokens → tree\nnodes mk1..mk7, selectors h1..h7"]
        TRN["TRN  trans() @2477, load() @3090\ntree → OCODE\nthe single-type stack language"]
        LEX --> SYN --> TRN
    end
    TRN --> CG["bcplcgcin.b — CG, 2,547 lines\nOCODE → Cintcode"]
    CG --> M["sysc/cinterp.c — the machine, 1,005 lines\none word type · stack · P-frames · global vector"]
    H["g/bcplfecg.h — THE ONE MANIFEST\ns_number=1 … s_putbyte · h1..h7 · sf_* · intg/feg/trng/cgg"] -.shared by every phase.-> LEX
    H -.-> SYN
    H -.-> TRN
    H -.-> CG
    L["g/libhdr.h — ug = 200\nthe system's band of the global vector"] -.-> M
    B["sysb/blib.b 1,887 · boot.b 1,118 · cli.b 208\nthe library, the boot, the REPL"] -.-> M
```

`com/bcpl.b` is twelve lines: it GETs `bcplfe.b` then `bcplcgcin.b`. Two sections, one compiler, one header between them.

## Bit by bit — what it is, and what it is doing for us

| BCPL, where | what it is | ours today | measured / status |
|---|---|---|---|
| **the one manifest** `bcplfecg.h:13–121` — `s_number=1; s_name; …` serves lex tokens, tree nodes AND ocode ops | one numbering at every level; a kind IS its number | `phiclm::manifest::S` — ported in the header's order; `Op → S` mapped; phiclm's six Rust enums to be measured against it rung by rung | ported this round, tests green; the language model's kinds (channels) are to climb the same ladder |
| **groups as ranges** — `s_mul; s_div; s_rem; s_add; s_sub` is "arithmetic" by position | a group is a contiguous range; the hierarchy is the order | `manifest::GROUPS` (12 ranges, contiguous and disjoint by test); the **band** on the circle | the band was measured to be exactly a rank cutoff on agreement (+0.0 vs truncation at every width): a band is *where the range ends*, not a second mechanism |
| **selectors** `h1=0; h2 … h7` | a kind's defined metrics: which fields mean what is fixed by the kind | `Row{about, kind, other, weight}` is h1..h4 fixed for all kinds | NOT YET per-kind; the next rung after the ladder |
| **sub-enums** `sf_none=0 … sf_neqv` under FLTOP and the `ass*` kinds | a kind defines its own enum to drill down | `manifest::Sf` ported; the answer SLOTS `kind#s` are a hash of the word, not an ordered enum | the slots to become a kind's ordered sub-enum — the drill-down the user named |
| **the global vector in bands** `intg; feg=intg+50; trng=feg+60; cgg=trng+60`; `libhdr.h:216 ug=200` | each phase owns a RANGE of globals: restriction of influence at the system level | **domain and range channels** (`<kind>` for the thing reaching, `<kind>>` for the thing reached) | **+15.1 points** containment at matched size over the shared-channel read — the one real gain of rounds 198–199 |
| **LEX** `lex()` owns every character | tokens are decided in one place | `phiecs::speak::tokens` | the typographic-apostrophe bug (8,199 shattered contractions) was a LEX bug; fixed with tests |
| **SYN** recursive descent into vectors | structure discovered from the token stream | the construction families / the DAG of knowing (found, not given); phiclm's parser for programs | built |
| **TRN** tree → OCODE | COMPRESSION: the high language into the single-type one | phiclm's `trn/` for programs; for prose, the **template** — the type sequence a sentence compresses to | the named rung: ≤1024 templates; measured today that the TYPE is what the phasors get right |
| **CG** OCODE → Cintcode | EXPANSION: the single-type language said in the machine's words | within a type, which THING: association by preference | measured today: at the thing level association wins at every k (top-1 14.1 vs 8.7, top-5 26.1 vs 20.0); no re-ordering by the phasors beats it (`sure`, `typed` ≤ ±0.5) |
| **the machine** `cinterp.c`: one word type, `!` and `@`, P-frames, step count | nothing is typed; everything is a word | `phiclm::machine`, the judge (`Verdict`, never a panic) | built; runs real BCPL on every workshop page |
| **blib / boot / cli** | the library, the boot, the REPL (208 lines) | **phient**, the brain: the arena, the protocol — `phient serve` is its cli.b | the named rung after the templates |
| **opname / sfname** | the readout per level | `talk --read` dumps a read with its scores | built this round |

## The steps, taped (every number from a run)

1. Tokenizer: U+2019 made an apostrophe only between alphanumerics. Counts fell 21.4% → 15.2%, phasors 13.9% → 12.7%: the counts' lead was partly the free win `don → t`. The gap is 2.5, not 7.5.
2. Amplitude (`ln(1+n)`, a count) multiplied agreement in the read: frequency swamped phase; `the` answered `a, i, is, you`. Agreement alone flips it: once-told words tie at 1.0.
3. Domain and range: one angle held two roles and the pulls cancelled on words like `the`. Split onto two channels: **holds the truth 72.5% vs 57.4% at matched size, +15.1.**
4. The band: `the` → wide: `you, in, is`; 0.25: `more, job, lot, good`; 0.0625: `morning, children, evening, future, noise, books`. A grammatical class appears as influence is restricted. Measured against the split's own read truncated to the same size: **+0.0 at every width** — the band IS the cutoff.
5. Top-1 under the split: 2.3% (pure agreement), 8.0% (`sure`, agreement × n/(n+1)) — the type is right, the thing is wrong. Containment +15.1 either way.
6. Two layers (`typed:1.0`: membership by band, order by association): top-1 14.0, top-5 26.2, top-10 31.0 vs count 14.1 / 26.1 / 31.2 — **no effect**. Not tuned further.
7. The within-type test (groups after nouns vs after adjectives) was contaminated by the tie defect under pure agreement and is re-run under `sure` next round, taped.

What the tape says, in the user's words: it is a type in play, not a thing. The phasors know the type (+15.1); association knows the thing; and the generator must therefore be two steps — the template first (TRN: compression to the type sequence), the words second (CG: expansion by preference) — which is the architecture above, applied to prose.

## Status

2026-10-27. `phiclm::manifest` ported from `g/bcplfecg.h` with tests; read modes `count` (default, unchanged), `agree`, `band`, `domain`, `bound`, `sure`, `typed:<turns>` behind `PHI_READ`, each measured above; `examples/band.rs` measures a read as a template (containment at matched size). Nothing switched on by a qualitative win. Next: the template — a sentence compressed to its type sequence on the manifest ladder, ≤1024 kept, measured by containment; then the slots as a kind's ordered sub-enum.
