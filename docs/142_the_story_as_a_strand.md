# 142 · The story as a strand: sixty-four levels from the word to the whole, coded like DNA

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. The ask: how is a language abstracted?

The user brought the joint-embedding argument (paper 26): a world model must predict at a level that drops what it cannot predict, and the hard part is finding that level, done by an encoder kept from collapse by maximising an information measure. They asked how *we* abstract a language, and to rewrite the talk's close in "DNA codes": between a word and a full story lie roughly sixty-four levels of use, and the idea is information maximisation.

The answer, in one paragraph. A Phasor Language Model abstracts a language by its tree of conditions: every word is told under the scopes it was used in, each scope is a distribution over what was said there smoothed toward its parent's (D4), and a reading is made at the lowest level that has something to say, the levels above standing behind it. Each level is a model of its own. What was missing was a way to find a text's *own* levels when no directory gives them — a talk is one file. The strand does that, and it is information maximisation made exact: a text is divided where dividing shortens its description, the divisions are kept where they pay for themselves, and sixty-four levels are read off the result, each named by a codon.

## 2. The strand (`phiplm::prose::strand`)

- **The code.** A segment's description length is its tokens predicted in order with Witten–Bell escape (PPM method C) to the parent's distribution: `p(t) = (c(t) + T·p_parent(t)) / (i + T)`, `T` the types seen so far. This is the breadth smoothing the levels already use (docs/131), so no new knob enters (`strand/code.rs`).
- **The division.** Dividing a span saves its code length as one segment minus the halves' as two. It costs what it takes to say: two tree flags (`2 ln 2`) and where the cut falls. *Where* is coded with the story's own marks as the codebook: one bit for "after a mark the text wrote" (any non-word token: a full stop, a comma, a question mark), then a uniform choice within that class. A cut is therefore cheaper where the text already marked one. Nothing is assumed about which marks.
- **The tree.** Every span's best division depends only on the span and its parent, so the whole tree is built down to the word first (`strand/cut.rs`). It is then pruned bottom-up by minimum description length: a division's value is its gain plus the value of what it unlocks below, clipped at zero. **The pruned tree's leaves are the units the text's information supports.**
- **The order and the levels.** The divisions are made in an order: the pruned tree first, most information unlocked first; below it, where no division pays, the largest unit first at its best cut. Level `k` is the division at `m(k)` units, spaced geometrically from one unit (`TTT`, k = 63) to one per token (`AAA`, k = 0), one unit more per level where the geometry is slower. Codon `k` is `k` in base four over ACGT, the same code phiano's helix uses for the sixty-four relation channels, now in `phinet::codon`.
- **What a unit says.** A unit's signature is the words it says beyond its parent, by D4's `c · ln(p_unit / p_parent)`, in the order said. Nothing is generated.
- **As a tree of conditions.** `Strand::rows` tells every span as a scope `within` its parent and `says` its words. A cast can be drawn into it like any other tree (tested: "engineer constraint" lands in the engineer's half).

Tests (`cargo test -p phiplm strand`, 4): sixty-four levels from the whole to the word; the first division falls where the robot gives way to the engineer, right after the full stop; text with no repetition is not divided (gain ≤ 0, one unit supported); the strand casts.

## 3. Measured on the talk

`strand crates/phiano/data/talks/world_models_close_and_qa.txt --out crates/phiano/data/talks/world_models.strand` runs on the talk's close and its questions, as the user pasted it: 730 tokens, with *Emma* (198,124 tokens) as the prior.

| codon | units | nats over the level above |
|---|---|---|
| TTT ䷿ | 1 | — |
| TTG ䷾ | 2 | −2.6 |
| TTC ䷽ | 3 | +22.0 |
| TTA | 4 | +7.7 |
| TGT ䷻ | 5 | +1.9 |
| TGG | 6 | +5.5 |
| TGC ䷹ | 7 | +5.4 |
| TGA ䷸ | 8 | −0.1 |
| GTT ䷯ | 17 | −3.7 |
| CTC ䷝ | 35 | −5.3 |
| CCT | 66 | −23.9 |
| AGT | 231 | −52.3 |
| AAA ䷀ | 730 | −50.6 |

**The talk's information supports seven units.** Dividing it into them shortens its description by 39.9 nats. Every level below `TGC` costs more to say than it saves.

Two findings came out of building it, both kept as rules:

- **The first division alone does not pay** (−2.6 nats at TTG), but the one it unlocks pays +22.0. Stopping at the first unprofitable division would have said "one unit". Bottom-up pruning says seven.
- **Below the supported level, nothing steers the cuts.** Under the first criterion, a fixed Dirichlet with one pseudo-count, *every* level gained, because such a segment expects its own words to repeat. Under ordering by single-step gain, the ladder peeled off one-word slivers. The Witten–Bell code, the cost of saying a cut, and cutting after the text's own marks fixed both. The slivers that remain are the talk's real one-word sentences ("No," and "Right.").

Against a prior that knows almost nothing (a one-word prior), the same talk supports ten units. The count is a measurement of a text *relative to what was expected of it*; English is the reference prior.

## 4. The talk rewritten in its codons

Each line is a unit, in the words it says beyond its parent, in the order said (seven words at most). Every level, every unit, is in `story.dna`; the levels with their gains are in `levels.tsv`.

**TTT ䷿ — the whole (1 unit).** samples · learning · it's · uh · you · ai · constraint

The talk against English: its own subject words, and *uh*, the spoken register, which a novel never says.

**TTC ䷽ — three (+22.0 nats).**

1. two variables contrasting methods lot practical i've
2. of and know work you blah ai
3. to about that space constraint a right

**TGC ䷹ — seven, the level the information supports (+5.4 nats).**

1. two variables contrasting methods lot practical i've — *dimension-contrastive methods, their applications*
2. abandon reinforcement i mean minimize so horribly — *minimise RL, horribly inefficient*
3. know you rl do have blah representations — *learn by observation, RL on top of representations*
4. real ai of for physical llms um — *real, physical AI; not LLMs*
5. know questions one to and quick about — *quick questions*
6. mpc engineers in constraints space system into — *MPC engineers' constraints in state space*
7. to a that trained need right task — *a head must be trained per constraint, per task*

(The italic glosses after each dash are ours, for the reader; the strand's output is the words before the dash.)

**GTT ䷯ — seventeen, below the supported level, divided by size (−3.7 nats).**

1. two variables contrasting methods lot practical i've
2. abandon reinforcement i mean minimize so horribly
3. there people but like is desperate else
4. can have learning know blah good representations
5. uh won't require sometimes can't avoid certainly
6. if in making real progress ai grounded
7. don't work on generative so as probably
8. uh a ami labs like it's that
9. noisy completely helpless problems we're working that's
10. know questions one to and quick about
11. mpc engineers love because they
12. in constraints them space system do into
13. very small your that maps tiny projection
14. need different encoder for each constraint projector
15. right
16. task is like open door i'm talking
17. right okay think leave thank jan all

At `AAA` the strand is the talk itself, one word per unit.

## 5. Where it sits

The strand is the measurement paper 26 §8 pointed at. JEPA cannot measure the information in its representation and maximises a surrogate. The tree's levels carry their information as a number (`ln(p_level / p_parent)`), and the strand turns that number into the levels a text supports. It goes into paper 25's second edition as finding 4.4, with the infographic `fig/f25v_strand.png`.

What it is not, honestly:

- **A bag-of-words code.** Within a segment, word order counts only as the order of first occurrences. Phrase structure below a sentence is not seen; that is the territory of D2 and the pairs.
- **A quadratic division.** O(n²) per span: fine for a talk or a chapter, not for a novel in one piece (cut it by its own files first).
- **Signatures, not summaries.** A unit's signature is what it says beyond its parent, so a short unit's signature can be function words.

**Tested in docs/143.** Boundaries are clearly better than equal or random cuts; structureless text gives one unit 235 times in 240; two of the talk's six boundaries are exactly on its structural seams. The count runs above chapter labels; docs/144 shows that the criterion is calibrated on synthetic text (11 over-counts in 480 trials) and that the excess is real topical sub-structure inside the labelled units. Seven is what the talk's words support.

Housekeeping: `phinet::codon` (the codon code moved down from phiano's helix, which now re-exports it); `phiplm::prose::strand` (`mod.rs`, `cut.rs`, `code.rs`, `prior.rs`, `tests.rs`, each under 200 lines); phiano's `strand` binary; the talk's text and its strand under `data/talks/`. Workspace tests pass with zero warnings.

`strand <story.txt> [--prior <dir>] [--out <dir>] [--show TTT,TGC,…] [--words N]`
