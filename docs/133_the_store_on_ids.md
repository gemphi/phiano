# 133 · The store on phiecs's words: a third of the memory, the same readings

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. The wall docs/132 named first

Teaching for a night was possible; a week was not, because the counts kept every name as a string, once per row: *Emma*'s 158,000 words took 1.3 GB, about 8 KB a word. docs/132 §2 said the cure was in a crate we were not using here. It was.

## 2. What changed

- **Every name is a phiecs `Word`.** `Counts` interns every `about`, `kind`, `other` in a phiecs `Lexicon` as it learns; the tables are maps of ids (`(Word, Word) ⇒ Table`), and a string lives once. The interface is unchanged — a caller still asks in words — so nothing above it moved.
- **A table is a short list until it is wide.** Most conditions have said one or two things; `Table::Few` is a list of up to twelve `(Word, count)` pairs and becomes a map only past that.
- **The rows are counted and dropped.** `Knowing` no longer keeps the rows it learned from; the scope tree is built as the rows pass, and what `unknown` needs — each paragraph's words — is kept as words. The tree is read one paragraph at a time (`prose::for_each_paragraph`), so the whole corpus is never in memory as rows.
- **A wide table's distribution is a pass over its entries.** With an index of each value's place in the kind's sorted values (built once per learning), `dist` fills the base everywhere and adds the table's entries by index, instead of looking every value up.

## 3. Measured

| | before (docs/128–129) | now |
|---|---|---|
| *Emma* + the story: peak memory | 1,308 MB | **444 MB** (2.8 KB a word) |
| read and count 2.97 million rows | ~8 s | ~8 s |
| the story said back under `--in text/story` | exactly | exactly |
| context tests, 25–5,000 turns | 10/10 · 20/20 · 20/20 | 10/10 · 20/20 · 20/20 |
| one ask at 2,000 / 5,000 turns | 22 / 33 ms | **10 / 16 ms** |
| code student by task | 75% | 75% |

A night of teaching at 100–150k words an hour is a million words, about 3 GB now; a week is still not a laptop's job, but the next factor is no longer the strings. What remains is the ~1.2 million small tables of `scope|word` and `scope|pair` conditions (three scopes for each occurrence), which is the price of reading a word under every level of its path; a `Dense` store keyed by (scope, word) rather than a map, and counts as `u32`, would halve it again.

`story data\text --in text\story --go "She suddenly"` · `context --turns 2000` · code: `src/relate/{counts,dist,table}.rs`, `src/prose/{mod,knowing}.rs`
