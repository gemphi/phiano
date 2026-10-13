# 136 · The field of phasors on the tree: the cast by cosine, measured beside the counts

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/132 said plainly that everything the tree does, it does with counts, and that the paradigm's other word — *phasor* — was nowhere in the code. This doc puts it there, in the one place docs/132 named first: the cast. Every number below is measured on the same tests as the counts, with the same gate reading the result, so the only thing that changed is what draws a word into a scope.

## 1. What was built

**A word's function.** Every word has a function of length R (phinet's `Function`, a complex vector). At R = 64 it is the word's phiecs [`Phasor`](../../phiecs/src/phasor.rs) — the sixty-four angles `Phasor::seeded(spelling)` gives it on the relation circle — read as one unit phasor per channel, `e^{iθ_k}` (`Phasor::angles` is new, four lines). At another R it is a seeded unit-phase function (`phinet::unit_phase`). Nothing is learned here; the function is what the spelling seeds, as a fresh word's phasor always was.

**A scope's phasor.** `relate::field::Field` gives every scope of the tree one function: the sum of what the scope said beyond what the whole tree says, each word weighted by its D4 post,

`g(scope) = Σ_w ln⁺( p_scope(w) / p(w) ) · f(w)`

— the same `ln(dist / prior)` the counts post (docs/124), carried into function space. A word the scope says no more often than the tree does adds nothing; a scope that said a thousand common words does not drown the one rare word it said. The scopes' phasors are built once after the counting (`Knowing::with_field(R)`), from `Counts::said`, in the time the table shows.

**The cast.** The asked words — the last six and the last three pairs, as before — are one function too, each by how much it says, `Σ_c ln(1/p(c)) · f(c)`. A cast descends from the roots, entering at each level the two children whose phasor points the asked way (phinet's `cosine` above zero) and keeping every scope entered with its path. No table is consulted in the descent; `said` and `own` on each result are the counts' locality, so `Knowing::next` reads the result through the same gate as before (docs/135 §3). The results are ranked `Order::Told` — most said, deepest, told later (phiecs's `Condition`: now over was), most attracted — or `Order::Attraction`, the cosine alone, which is the phasor with no time in it. `--phasor R` on `story` and `context` casts this way; `--order attraction` ranks by cosine alone.

## 2. Measured

**The context tests** (docs/129: ten needles, twenty keys, twenty keys updated later; filler from *Emma*; seed 7).

| cast | turns | needles | keys | updated keys (latest) | read in |
|---|---|---|---|---|---|
| counts (docs/129) | 400 · 2000 | 10/10 · 10/10 | 20/20 · 20/20 | **20/20 · 20/20** | 5 · 8 ms |
| phasor, R = 64 | 400 · 2000 | 10/10 · 10/10 | 20/20 · 20/20 | 11/20 · 14/20 | 1 · 2 ms |
| phasor, R = 256 | 400 · 2000 | 10/10 · 10/10 | 20/20 · 20/20 | 14/20 · 15/20 | 1 · 2 ms |
| phasor, R = 1024 | 400 · 2000 | 10/10 · 10/10 | 20/20 · 20/20 | 16/20 · 15/20 | 2 · 3 ms |
| phasor, R = 4096 | 400 · 2000 | 10/10 · 10/10 | 20/20 · 20/20 | 18/20 · 17/20 | 3 · 6 ms |
| phasor, R = 64, by attraction alone | 400 · 2000 | 10/10 · 10/10 | 20/20 · 20/20 | 11/20 · 13/20 | 1 · 2 ms |

Needles and keys are found at every R because the pair carries them (`vault is` was said once, in one turn; the language reading alone answers it). The *update* column is the cast's own test — two turns both say `alpha is`, and the scope decides which value — and there the phasor is behind the counts and climbs with R: 11, 14, 16, 18 of 20 as R goes 64, 256, 1024, 4096. Seed 8 gives the same shape (counts 20/20 · 20/20; phasor at R = 64, 10/20 · 10/20).

Why, exactly (`PHIANO_CAST_DEBUG=1` shows it): at 2000 turns a block of a hundred turns says some 3,500 words, 1,500 of them different, and the key's turn is one of them. The block's phasor is the superposition of 1,500 weighted phasors; asking it whether `alpha` is among them is a cosine whose signal is one word's post (about 3 nats) against the sum, and whose noise is `1/√(2R)`. At R = 64 the two are a few standard deviations apart; the block that holds the later turn is not always among the two entered, and the cast never reaches it. The count cast asks the block a different question — *does your table hold `alpha` at all?* — which is a lookup with no noise and no capacity. That is the finding, stated as the measurement states it: **a phasor per scope is a superposition and has a capacity in R; a count table per scope has none.** At R = 4096 the phasor is 64 KB a scope, larger than the block's table (about 1,500 entries), and still misses two updates in twenty.

**The story** (docs/128). Under `--in text/story` the phasor cast says the story back exactly at R = 64 and R = 256, and is drawn into the story's paragraph one word sooner than the counts — after "she suddenly" rather than "she suddenly had" — because the asked function weighs `suddenly` by its rarity where the counts need two words said.

**Going on in new text** (docs/135 §3: 5% of *Emma*'s paragraphs held out, 10,578 positions, seed 7).

| reading | top-1 | MRR | perplexity |
|---|---|---|---|
| flat: the last word, then the last two | 20.6% | 0.309 | 159 |
| tree, cast by counts, gated (docs/135) | 20.1% | 0.304 | 201 |
| tree, cast by phasor, R = 64, gated | 20.4% | 0.307 | 164 |
| tree, cast by phasor, R = 256, gated | 20.4% | 0.307 | 166 |
| counts: the last word alone (the bigram) | 18.8% | 0.288 | 160 |
| operator: the last word, Joint FNO, R = 64, 8 epochs | 9.2% | 0.189 | — |

The phasor cast reads new text a little better than the count cast and a little under the flat reading, and the gate says why: of the 10,578 positions, the count cast's result passes the gate at 1,343 and the phasor cast's at 288. A new paragraph that merely shares two words with a told one meets the count cast's `said`; its cosine against that paragraph's whole phasor is small, so the phasor cast is drawn elsewhere or nowhere, and the position is read by the language. Fewer new paragraphs read as some told paragraph, hence the perplexity. All three tree readings are within half a point of the flat one, which is the trigram ceiling docs/135 named.

The last two rows are the same question asked of the last word alone. The counts answer it as a bigram, at once, at 18.8%. phinet's Joint operator — trained on the told paragraphs' `word next word` rows (185,209 of them; R = 64, width 16, two layers, eight epochs, 31 minutes on two cores, hinge loss 0.056 → 0.016), reading each held-out position from its last word by cosine over every word that ever came next, 191 positions whose last word it never saw — answers it at 9.2%, half the bigram, after half an hour of the slow speed. That is docs/92's finding on new text: the operator learns the shape of the answer, not the answer; the count has the answer the moment it is told. No perplexity is given for the operator because a cosine ranking carries no probabilities.

## 3. What this says about the next build

The phasor is now in the code, in the cast, and measured. On the tests the week was built on it does not beat the counts: the context tests want membership, which a count table answers exactly and a superposition answers to within its capacity; going on in new text is the trigram ceiling for both. What the phasor was promised for in docs/132 — two scopes near without a shared word, the paraphrased needle — is not something a *seeded* function can do: `Phasor::seeded("combination")` and `Phasor::seeded("code")` point nowhere near each other, and nothing in a session of *Emma* with twenty keys tells the operator they should. That needs functions learned from telling — the Joint table trained on the tree's own rows — and the honest cost of that speed is in the last row of the table: hours of epochs for what the counts fill in a second, with the reading still to earn its place. So:

- the cast stays the counts' by default (`Order::Told`, D3 locality, now over was); `--phasor R` is there to measure against, not to switch to;
- the field takes a learned table as easily as a seeded one (`Field::with_words`), so the next measurement is the same cast with the Joint's functions for the words, and the paraphrase question asked of a tree told by Phi-4, where `combination` and `code` are said in each other's company;
- the capacity finding is a reason to keep scopes as tables and words as phasors: a scope is a *set* of what was said (phiecs's `Dense` store, exact), a word is a *direction* (its phasor, learned); the cast asks the set, the reading moves along the direction.

Housekeeping in the same commit, by the standing rules: the old Searle experiment whose modules were left under `bin/context` when the context tests took that name is a binary of its own again (`in_c`, nothing dangling); the three source files still over 200 lines are split along their seams (`code_student/{answer,writing}.rs`, `code_write/tally.rs`, which also folds two copies of the writers' scoreboard into one); the tree's test moved to `relate/tests.rs`. Workspace: no warnings, 353 tests.

Where the crates stand after this doc, against docs/132 §2: phiecs's `Phasor` is used (every word's seeded angles); phinet's `Function`, `cosine`, `unit_phase` and the Joint operator are used (the field; the operator row); the ECS's `Dense`, `World`, `System`, `hrr`, `sphere` and phical are still waiting.

`context --turns 400,2000 --phasor 256` · `context --turns 400,2000 --phasor 64 --order attraction` · `story data\text --in text\story --go "She suddenly" --phasor 256` · `story data\text --in text\emma --held-out 0.05 --phasor 256 --operator 64,8` · `cargo test --workspace`
