# 135 · One workspace, no file named after its folder, and going on measured on new text

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. The crates under one roof

Everything Phiano needs now lives in the phinet repository as one Cargo workspace:

```
C:\Users\phiac\Workspace\gemphi\phinet\
  Cargo.toml                 the workspace (members below; the release profile)
  crates\phinet\             learns, knows no words: the Fourier tensor engine and operators
  crates\phifno\             the spherical Fourier operator for words
  crates\phical\             exact geometry of the relation circle and sphere (package phi-cal)
  crates\phiecs\             the entity-component-system for language; the quest asks
  crates\phiton\             PhiTon: light as the unit system
  crates\phiano\             words and applications: relations, the tree, the students
```

Moved from `gemphi\phiano`, `gemphi\phifno`, `gemphi\phiton`, `gemphi\phiace\crates\phiecs` and `gemphi\phiace\crates\phical`; phinet's own crate went to `crates\phinet`. Dependency paths are siblings now (`../phinet`, `../phiecs`, `../phical`, `../phiton`); the `[profile.release]` lives once, in the workspace root; member lock files and the old `target` directories (11 GB under phiano) were moved to `gemphi\_to_delete\` for you to delete. phiace's workspace no longer lists phiecs or phical, and its `apple` crate points at `../../../phinet/crates/phical`. Build anywhere under `phinet` with `cargo build --release --bin story`; the binaries land in `phinet\target\release\`, and every script finds that directory through `cargo metadata`, so the old `phiano\target\release` paths are gone from them. phiano keeps its own git history (`crates\phiano\.git`); phinet on disk is not a repository, so nothing is nested. Not moved because they do not exist as crates: `phinum` is a module of phiano (`src/phinum`), `phicli` has no directory, and *photon* is `phiton`. The whole workspace builds with no warnings; 342 tests pass (phiano 237, phinet 50, phiecs 37, phi-cal 13, phiton 4, phifno 1).

## 2. No file named after its folder

Seventeen files in phiano were `src/<x>/<x>.rs`. Each is now named for what it holds: `model/run.rs`, `server/types/requests.rs`, `server/chat_intent/classify.rs`, `envision/gaps.rs`, `reasoning/multi_path/solve.rs`, `phase_flow/propagate.rs`, `metrics/harness/perplexity.rs`, `trainer/build.rs`, `facet/learn.rs`, `command/dispatch.rs`, `storage/save.rs`, `wiki_bulk/extracts.rs`, `wave/sentence.rs`, `persona/fingerprint/extract.rs`, `persona/world/personas.rs`, `roles/bind.rs`, `phical/phase.rs`. The parent modules declare the new names; nothing else changed. The other crates had none.

## 3. Going on, measured on text the tree has not seen

docs/128 said the tree says its own text back and goes on in a language; it did not say how well it predicts text it was not told. Now it does (`story data\text --in text\emma --held-out 0.05`): 5% of *Emma*'s paragraphs held out (101 paragraphs, 10,578 asked positions), the next word read at every position from the words before it, scored against the word that came.

| reading | top-1 | MRR | perplexity |
|---|---|---|---|
| flat: the last word, then the last two (the language alone) | 20.6% | 0.309 | 159 |
| tree, first version: the cast's scopes and the words under them, always | 15.8% | 0.267 | 863 |
| tree, hearing a scope only when the text is inside it | **20.1%** | 0.304 | 201 |

The first version lost badly: a new paragraph drawn into some told paragraph by a few shared words was then read as *that* paragraph — one sighting of `scope|pair`, half the probability on one word — and was wrong. What fixed it is D3 again, the condition is local, made exact: a cast's scope is heard only when the words are inside it — its last scope itself says at least two of the asked conditions *and* the last three pairs of words. The story's own paragraph says all its pairs, so the story is still said back exactly; a key's turn says `alpha is`, so the context tests are still 10/10 · 20/20 · 20/20; a new paragraph of Austen says a told paragraph's three pairs only when it repeats it, so it reads as the language reads it. The tree now predicts new text within half a point of the flat reading and recalls told text exactly; the two were one reading before and are one reading still, with the gate deciding which game the words are in (§29).

What this number also says, plainly: 20% top-1 is a trigram over 158,000 words, and that is what the counts are. Going on in new text is where the operator belongs (docs/132 §2); this table is the line it has to beat.

`story data\text --in text\emma --held-out 0.05 --seed 7` · `cargo test --workspace`
