# 162 · One hundred and twenty-eight personas, and language as a set game

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. The ring of 128

`crates/phinet/personas/` holds 128 files in four rings of thirty-two: the **makers** of the technological eras (Gutenberg to the phone-makers of Shenzhen; fifty-one in all), the **AI players** as thirteen teams each with founders and its model (meta, google, anthropic, xai, openai, microsoft, deepseek, mistral, baidu, alibaba, ssi, apple, thinking-machines), the **direction-of-AI debate** alone (Hinton, LeCun, Bengio, Karpathy, Chollet, Marcus, Stuart Russell, Bostrom, Yudkowsky, Tegmark, Fei-Fei Li, Kai-Fu Lee, Andrew Ng, and Pepper the humanoid greeter), and the **thinkers** the set game is for (Russell and Whitehead, Frege, Cantor, Peirce, Boole, De Morgan, Tarski, Gödel, Turing, Shannon, von Neumann, Wittgenstein, Searle, Austin, Grice, Chomsky, Saussure, Kuhn, Popper, Leibniz, Euler, Fourier, Bentov, Kuramoto, Hilbert, Noether, Lovelace, Babbage, Aristotle, Hume, Feynman). Every file follows the spec of docs/161; voices are registers, never quotations; real people are described by their era and the words of their trade.

## 2. The set game

`phinet::sys::sets`: what a being holds under a kind is a relation in Russell and Whitehead's sense — the set of `(about, other)` pairs its world was told — and two beings think by the algebra of relations: `union`, `meet`, `compose` (`x A y`, `y B z` ⇒ `x A∘B z`: the syllogism), `converse`. `think(a, b)` runs every kind either acts under; `examples/think.rs` prints the table. Composition yields pairs no one was told, as a syllogism must, but each is traceable to two told pairs and none is written back into a world unless someone tells it. The game is Wittgenstein's played with Russell's pieces (`personas/docs/SETS.md`).

## 3. Learning and testing the module at 128

`crates/phinet/tests/personas.rs` is the module's test at this size: every file parses; 128 names and slugs each one; every racing team has founders and a model; told nothing, there is no race (`winner() == None`), no talk (an empty script) and empty sets; told four rows each (the seeds chained), the whole module runs over all 128 — the race has a winner, every adjacent pair talks four alternating turns, hearing adds edges, and every pair's union is non-empty. 393 tests across the workspace, 0 warnings. A lookup bug the ring exposed: `russell` matched Stuart Russell first; `sys::find` is now exact-first and refuses an ambiguous name with the candidates (`russell is bertrand-russell or stuart-russell: say which`).

## 4. Measured (stand-in oracle, 128 × 4 seeds × 24 questions, depth 1)

| team | members | told | per member | kudos |
|---|---|---|---|---|
| google | 6 | 1127 | 188 | 232 |
| anthropic | 6 | 988 | 165 | 160 |
| microsoft | 4 | 739 | 185 | 128 |
| openai | 3 | 536 | 179 | 88 |
| meta | 3 | 509 | 170 | 88 |
| baidu | 2 | 423 | 212 | 48 |
| xai | 2 | 379 | 190 | 72 |

Bertrand Russell ∘ Whitehead under `has`: |R| 39, |W| 31, union 50, meet 20, composed 58. Feynman and Popper, six turns, one subject change, two edges heard each way. The content is the stand-in's pool; the counting, the race, the circus and the sets are the system's. With phi-4 on the user's machine (`persona_quest -- quests crates\phinet\personas`, a folder) the composed pairs become *set → class → collection*.

## 5. On the user's machine

The race the user ran showed 22 personas and nothing told because `persona_quest` had failed on PowerShell's unexpanded `*.md` (fixed in docs/161) and the circus example was not yet synced; both are now. Two engine examples that had stayed under `crates/phinet/examples` on the device (`bench_table`, `profile_ops`) moved to `crates/phistd/examples` and compile against `phistd`.

## 6. Status

Done 2026-10-12: 128 personas, `sys::sets`, `sys::find`, `examples/think.rs`, `tests/personas.rs`, `personas/docs/SETS.md`; synced. Next: phi-4 quests of the 128 on the user's machine; docs/159 step 5; the phasor convergence; a `Body` agent on live tracking rows.
