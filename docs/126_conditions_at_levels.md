# 126 · Conditions at levels: the scope tree, the cast, and what it measured

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 0. The correction to paper 24

Paper 24 set Context Language Models beside Phient as *data kept* against *relations kept*. That is true of what is stored and misses the match: their context is our condition. A CLM reads every word under its context and is allowed to rewrite that context (c_{t+1} = f(c_t)); Phient reads every word under conditions and the conditions come at levels — a token in a function in a file in a crate, an order in a scene in a session, a word in a sentence in a paragraph in a chapter. What had been missing on our side was that a row carried one condition and the reading composed none. The CLM keeps the levels as one flat text and edits it by judgment; what follows keeps each level as a condition word with counts, and the hierarchy as a path. The paradigm is recursive: X ⇒ Y under c, where c is itself a word read under its own c — a tree of X ⇒ Y — and the tree on disk (`data/phi4_code/<family>/<task>.rs`, `uses/<word>/<fn>.rs`) is that tree.

## 1. What was added to the base

- **`within` rows** (`phiano::code::files`): every function is told `within` every scope on its path, and each scope `within` its parent (`count_above within scope:above`, `scope:above within scope:data`). A scope is a word prefixed `scope:` so a directory named `above` and the word `above` are two conditions. Nothing new in phiecs: a level is a kind, a scope is a word, the quest can write it and ask about it.
- **`Counts::read_levels`** (`phiano::relate::levels`): D4 nested. The outermost level's conditions move the prior; what they arrive at is the base the next level's counts are smoothed toward and moved from; the innermost moves last. A level never told moves nothing, so an unseen path reads as its nearest told ancestor. This is the recursion unrolled: the reading under a path is the reading under the parent path, moved by this node.
- **`ScopeTree::cast`** (`phiano::relate::tree`): a word is not aware of where it belongs; it is drawn into scopes by what it says. Each function also says its conditions under a kind `says`, so every scope has a distribution over words — what its subtree said. A cast starts at the roots and enters, best-first with a beam, the children whose functions said the asked conditions more than their parent did (`ln dist_child / dist_parent`, summed over the conditions), the way a ray is drawn through a bounding-volume hierarchy, pruning the rest; the leaves reached are where the word is read. The tree is Rust vectors in memory, built from `within` rows, and the file system is its persistent form.
- **`code_student`** reads under the path a cast finds (`--flat` for docs/124's reading); **`code_relations`** adds a "levels" line to the board, reading each held-out function under its own path's scopes then its words and types.

Unit tests: `relate::levels` (an inner level moves what the outer arrived at), `relate::tree` (a word is drawn to the scope whose functions said it).

## 2. Measured

**The student, Phi-4's 253 answers, 248 tasks, audited checks** — with the cast against flat (docs/124):

| test | flat (docs/124) | under the scopes a cast finds |
|---|---|---|
| a new type of a known task | 187 of 248 (75%) | 187 of 248 (75%) |
| a task never seen | 36 of 248 (15%) | 36 of 248 (15%) |
| one pointing | 152 of 248 (61%) | 152 of 248 (61%) |

Identical, to the task. The cast finds a family (`sum_below` is drawn into `scope:sum`) and moves all 248 answers, but on this corpus the family's name *is* the function's words: whatever the path says, the words had already said, and a rare word (`below`) moves an answer by ~3 nats where a family scope moves it by ~1.5. The path adds nothing the name did not carry.

**The relation board.** Phi-4 by family (R = 64): levels 0.390 = flat 0.390, because a held-out family's scope was never told and reads as its parent, the root, which is the flat prior. Our own crates by file (713 files, 25.9k rows): prior 0.169, counts flat 0.149, **counts under the file's directory then its words 0.138** — the levels line is the worst.

**Why the directory hurts.** `dist` smooths a condition's counts toward its base with weight one — right for a word seen a few times (one sighting is never certainty) and wrong for a directory with five hundred rows: a call the directory never made is read at about `base / 500`, a penalty of six nats, as if the directory were certain of what it never said. A new file in a known directory calls things its neighbours did not, more often than the global prior does, and the sharp directory reading punishes every one of them. This is not the levels failing; it is the post's trust not scaling with the condition's sightings — the two-speed question of docs/113 again, now at the scope level, and it has to be answered the same way (a trust that grows with sightings, learned, not set) before a wide scope can be read as a condition.

## 3. Reading

The structure the paradigm asks for is now in the base: conditions as a tree, read recursively, searched by attraction, persisted as a path. On the only data in hand it changes no number, for two honest reasons: the one corpus's hierarchy repeats its names, and the other's wide scopes are read too confidently. Neither is an argument against the tree. The first says the measurement needs a corpus whose levels say more than its names — prose, where a chapter's conditions are not in the sentence's words; or the builders' game with scene and session, where the sepia pointing lives in its session and the needle test is read as a path. The second says what to build next: the smoothing weight as a count of its own.

The comparison with a CLM is now stated correctly: both read words under conditions; the CLM keeps its conditions as one text and edits it; Phient keeps them as a tree of words with counts, where keeping and dropping are not edits but posts — a scope that no longer moves the reading holds none.

`code_student --phi4 data\phi4_code [--flat]` · `code_relations <dir> --held-out family|file` (board line "levels") · `cargo test --lib relate::`
