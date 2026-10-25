# 132 · Where we are: teaching for hours, and which of our own powers the work is using

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Two questions, answered plainly: when can the system be taught for hours and hours, and where the Fourier operators, the phasors, phical and the phiace crates are actually doing the work in what was built this week — and where they are not.

## 1. Teaching for hours: where we are

**What runs unattended today.** Three scripts, each one run of bounded length: `phi4_code.ps1` (253 tasks, three rounds of asking for unknown words, the three tests), `phi4_sepia.ps1` (the needle), `phi4_story.ps1` (three rounds of asking for the story's unknown words). Each ends when its fixed list of asks ends — an hour or two — and none of them has been run yet on the machine with Phi-4. That is the honest state: the system can be taught in bounded batches; it has not been taught for hours because nothing kept asking.

**What changes with this doc.** `scripts\teach.ps1 -Hours 8` is a loop on a time budget. Each loop the teacher goes on with the story (five paragraphs; before each, the student's own going-on is held against the teacher's and logged — agreement in judgments, D20, the first generation measure the project has had), the story's and the code student's unknown words are asked for and told, any task Phi-4 has not answered is asked, and every fourth loop the measurements are appended to `data\teach_runs.md` with the time: the context tests and the code student by task. Everything learned is a file under `data\`; a stopped run resumes where it was; nothing is retrained because nothing is trained. Verified with a stub teacher in the container; the first real run is the user's.

**What stands in the way of days.** Three walls, in order of nearness.

1. *The store.* The tree keeps a row per occurrence as strings (`suddenly@text/story/ch1/p2#1`), seven to nine rows each: 1.3 GB for *Emma*'s 158,000 words, so about 8 KB a word. Phi-4 on the GTX 1660 Ti gives in the order of 50–150k tokens an hour; eight hours is roughly a million words, roughly 8 GB. That holds for a night on a machine with the memory, and not for a week. The cure is in a crate we already have and are not using here (§2): phiecs interns words (`Lexicon`, `Word` ids) and keeps aspects in id-indexed dense stores; a tree on ids instead of strings is a tenth of the size and the rows need not be kept at all once counted.
2. *Re-reading.* Each cycle reads the whole tree again (8 s for 2.8 million rows). Fine for hours; a day's tree would take minutes a cycle. `Counts::learn` already accumulates; the loop should tell it only the new files.
3. *What hours buy.* Counts. More words known, more pairs, more scopes — and the measured ceilings of a counts student stay: the teacher's own inconsistency (docs/107), and the relations never told, which counts cannot reach (docs/105: 0% for counts, 25–28% for the phasor operator). Hours of teaching make the student know more of what Phi-4 said; they do not make it infer. That is the operator's job, and §2 says where it is.

## 2. Which of our powers the week's work used

| | used in docs/124–131 | not used, though it exists |
|---|---|---|
| **phiecs** (the ECS) | `quest::Row` (every row of every table); `quest::Spawned`, `Oracle` (every ask of Phi-4); `builders` (the game's prompts and parser); `Condition`'s rule *now over was* (docs/129's recency), as a sort order, not as the type | `World`, `Word`, `Lexicon` (interning), `Dense<A>` (id-indexed stores), `Phasor` (64 channels on every entity), `Relation`/`Edges` (the multigraph), `System`/`Stage`, `Quest::run` (the open-ended quest with depth and budget) |
| **phinet** (the Fourier engine) | `relate` (the Joint FNO operator, `Inherit`, `two_hop`, `score`) — only as the comparison line on the relation board, where the counts beat it (docs/124–125); `mix64`/`fnv1a` for folds | the FNO as the student's reading; `hrr` (holographic bind/unbind); `sphere`; the R knob on anything but that board |
| **phical** (exact geometry of the circle and sphere: Möbius, Q64, winding, heat) | nothing | all of it |
| **phiano** | `relate::{Counts, levels, tree}` (new, counts only), `code`, `prose` (new); `relate::RelationModel` on the board | `phasor` (the chroma facets), `generate`, `dialogue`, `persona`, `phical` module (manifold, topology), everything the earlier papers built |
| **phiray / phiphx** (BVH, ray casting, GPU) | the *idea* of the cast (docs/126) | the code |

**Where the progress is real** — all of it measured, all of it counts: a word from one pointing (sepia 100%; one answer of a code family lifts the student from 15% to 61%); the context tests at five thousand turns with nothing dropped (docs/129); the story said back exactly under its scope and gone on with a language (docs/128); the code student on the base at 75% by task (docs/124). These are the paradigm's *count* — "each a complex phasor on a closed circle **with a count**" — doing the work alone.

**Where the paradigm's other half is missing.** The same sentence says *phasor*. In the week's code there is no phasor: a scope is a count table, a word is a count table, the cast is a ratio of counts, the levels are nested counts. The user's own description of the cast — word uses as Fouriers and phasors drawn into a context — is not what the code does yet; it does the count version of it. This is the one place where the project's existing power is plainly unused where it should be used, and it is the same place where counts have a measured ceiling:

- a scope should be a *phasor*: phiecs's `Phasor` (64 channels) on every scope entity, the sum of what its subtree said, so a cast is a cosine between the asked words' function and the scope's — phinet's `cosine` and `normalise` exist — and two scopes that never shared a word can still be near;
- a level should be an *operator reading*: phinet's `Inherit` already reads a word as its own reading plus its parents' along a kind (docs/97 §1) — `within` is that kind, and `inherit_epoch` trains it; the levels of docs/126 are that structure with counts in place of functions;
- the paraphrased needle of docs/129 ("the vault's combination") is exactly a relation never told, the thing the operator reached in docs/105 and the counts did not.

That is the next build, and it is not a new idea; it is docs/97 on the tree: the same `within` rows, read by `RelationModel` with `Inherit` along `within`, beside the counts on the same scoreboard — the context tests of docs/129 and the one-hearing strata of docs/125 — with R as the knob. It will be slower to learn (epochs) and the counts will stay the fast speed; the question the measurement answers is whether the phasor reaches what the count cannot, on the tree, as it did on the night table.

## 3. In one line

We can teach for a night now, and the loop to do it is `teach.ps1`; we cannot teach for a week until the tree is on phiecs's ids; and what the week's work proved, it proved with counts alone — the phasors, the operator, phical and the ECS are built and waiting, and the tree is where they go next.

`scripts\teach.ps1 -Hours 8` → `data\text\story\teach.log`, `data\teach_runs.md` · `story data\text --in text\story --teach 5 ollama run phi4-builder`
