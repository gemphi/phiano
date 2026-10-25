# 128 · The language in the tree: a novel told as rows, and the story going on in it

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 0. "They have the language"

docs/127 ended: the CLM has the language, we have only what we were told. It meant one thing: a CLM runs on a model that has read most of English before the two paragraphs arrive, and the tree of docs/127 had read seventy-eight words. In Phient's terms "the language" is not a different kind of thing from the story; it is more told rows, under their own scopes, in the same tree. So it was told. *Emma* (Jane Austen, 1816; 55 chapters, 2,312 paragraphs, 158,046 words, already in `data/corpora/austen`) and the two paragraphs now lie under one root:

```
data/text/emma/ch01.txt … ch55.txt     (paragraphs separated by blank lines)
data/text/story/ch1.txt                (the two paragraphs)
```

Reading `data/text` makes 2,772,514 rows and takes about eight seconds; the counts hold them in 1.3 GB (Rust vectors and hash maps of strings; no model, no training). 7,231 distinct words.

## 1. What was added

- **The reader** (`phiano::prose::read_tree`): a directory is a scope, a `.txt` file is a scope (a chapter), each blank-line paragraph in it is a scope `p1`, `p2`, …. docs/127's one-sentence-per-file form reads the same way.
- **The word under each scope** (`in-word`, `in-pair` rows): besides `next` of a word anywhere, each occurrence is told under `scope|word` and `scope|pair` for every scope below the root. What this story goes on to is then a condition of its own, read after what the language goes on to.
- **Going on inside a scope** (`Knowing.within`, `story --in text/story`): the cast starts at that scope and stays in its subtree; the word levels still draw on the whole tree. The levels, coarse to fine: the path's scopes; the last word everywhere, then under each scope of the path; the last two everywhere, then under each scope. Nine levels for a paragraph of the story; each posts only if it says something (D2 against the prior) and moves the reading from where the levels above left it (`read_levels`).
- **`--beyond`**: where what was told ends (`⟨end⟩` is the best next word), go on with the language instead of stopping.

## 2. Run

Without `--in` (the cast free over the whole tree), from "She suddenly":

> she suddenly had an opportunity of speaking to her, and the same time, and i am sure i should not have been a great deal of the two, and she was not to be sure," said he," i am not at all."

The story's paragraph draws the first two words; `had an` is said hundreds of times in *Emma* and once in the story, and with the word levels global the novel wins — the student leaves the story at its third word. This is docs/126's finding in prose: a wide scope outweighs a narrow one unless the narrow one is a condition of its own.

With `--in text/story` and the scoped word levels:

> **She suddenly** → she suddenly had an idea: their new investment would demand them to meet, and she was going to make an irresistible move on this young ceo of an ai company.
>
> **He** → he had been struggling to reach. it suddenly happened that she was in love with her boss, but only circumstances needed to be right.
>
> **The investment** → the investment would demand them to meet, and she was going to make an irresistible move on this young ceo of an ai company.

The story's own goings-on now outrank the language's where the story has them, and the student says the story back. With `--beyond`, past where the story ends:

> she suddenly had an idea: their new investment would demand them to meet, and she was going to make an irresistible move on this young ceo of an ai company." i am sure i should not have been a great deal of the two, and the same time, and i am not at all."-- and i will not be afraid of being in love with her boss, but only circumstances needed to be right.-- i am very much to the direction he had been struggling to reach. it suddenly happened that she was in love," said he …

Austen's sentences under the story's conditions, with the story's own phrases drawn back in ("in love with her boss, but only circumstances needed to be right"; "the direction he had been struggling to reach") because the story's scopes keep posting. Inside the novel itself (`--in text/emma/ch01 --go "Emma"`) it says the first chapter back word for word, which is what counts over one telling must do.

## 3. What this is and is not

It is the whole mechanism of the paradigm at the level of a language: words as seeds, what they go on to counted under conditions at levels, a story as a path, going on as reading under the path. It accumulates: a second novel is a directory; nothing is retrained and nothing summarised. It is honest about what it has: 158,000 words of one author give the next word by what followed the last two under a scope — a conditioned trigram with a tree, and its prose reads like one. That is the same count that gave the builders' game student 89.5% and the code student 75%; it is not Phi-4's language, and the way to more of it is the same as everywhere in this project: more told, from the teacher, through the quest. The operator's job (the phasor functions, R relations per word) is still the part that reaches what the counts never saw (docs/105: 25–28% on relations never told, 0% for counts); it has not been put on the tree yet, and a tree whose nodes are phasor functions rather than count tables is the design for it.

Beside the CLM, this closes the gap docs/127 named without claiming it is closed: the tree now has a language, in rows, and the CLM's remains larger. What the tree has that the CLM does not is unchanged: no summary, no edit, every step a row that can be shown, learning at the moment of telling.

`story data\text --in text\story --go "She suddenly" --words 90 --beyond` · `story data\text --in text\emma\ch01 --go "Emma"` · code: `src/prose/{mod,knowing}.rs`, `src/relate/tree.rs` (`cast_under`), `src/bin/story/main.rs`
