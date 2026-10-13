# 90 · The 8M model: Phi-4's behaviour in 8, 16 or 64 million angles

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. This note fixes the
target the user set on 2026-09-28 and the plan to reach it. Nothing in it is
measured yet except where a number is given.

## 1. The target

One model, built in the gemphi workspace with phinet as its engine, that
**generates, chats and writes code** — Rust and Python first — in
**8, 16 or 64 million numbers** (a setting, not a constant) instead of
Phi-4's 14 billion (≈1,750×, 875× and 220× smaller). It compresses by
keeping **relations, not data**: no text is stored, only what each token
relates to, under which condition, with a count.

**Channels are not fixed at 64.** 64 is the base every token has (8 offsets
for word order, the rest for the first relation kinds). Beyond it, channels
are added as kinds and conditions appear — up to 1,024 and with no hard upper
bound — and a token pays only for the channels it actually uses.

## 2. The budget

A number is one angle on one channel (a source and a target side each). Every
token has the 64 base channels (128 angles); extension channels are sparse, so
the budget buys either more tokens or more relations per token:

| budget | tokens at the base only | or at 64 base + 64 extension channels each |
|---|---|---|
| 8M | 62,500 | 31,250 |
| 16M | 125,000 | 62,500 |
| 64M | 500,000 | 250,000 |

English's working vocabulary is 20–40k; Phi-4's tokenizer about 100k; Rust and
Python add their keywords, standard-library names and identifiers. Today's
Phi-4 model is 392 words ≈ 50,000 angles: 0.6% of 8M.

**How it is stored.** The 64 base channels stay the dense, byte-packed block
phiano already uses (`PHASE_CHANNELS`, 106 files depend on it, untouched).
Extension channels are a sparse list per token — (channel id, source byte,
target byte, count) — with ids from 64 upward handed out by one registry as
new kinds, code relations and conditions are declared. A `.chroma` without
extensions reads as before.

## 3. Teachers

| teacher | what it gives | how it is read |
|---|---|---|
| Phi-4 (quest, Ollama) | relations; short sentences per word; answers to simple questions (chat) | phiecs quest → `relations.tsv` + sentences → phiano |
| The Rust book (rust-lang/book, MIT or Apache-2.0) | English about code, Rust examples | text + code blocks |
| Python documentation (PSF licence) | English about code, Python examples | text + code blocks |
| gemphi's own Rust (phinet, phiano, phiecs, phiace …) | a large corpus of Rust the user owns | code |
| the compilers (`cargo check`, Python `ast.parse`) | whether code is valid | an oracle, like Phi-4, but exact |

Code relations need no model to extract: a parser gives them exactly —
`defines`, `calls`, `returns`, `type-of`, `field-of`, `imports`, `implements`,
`argument-of`. They become kinds on their own channels, as `is-a` and
`has` are for words.

## 4. How each capability is produced

- **Knowledge** (works today): `word · kind ⇒ answer` on the kind's channel.
  Phi-4 quest: MRR 0.762, hit@5 92.4% on what it was told (docs/89).
- **Generation**: the 8 offset channels are word order (the word 1–8 places
  ahead); the relation channels are meaning; phinet's operator predicts the
  next token from both (shipped operator 30.7× chance on Austen, docs/78).
- **Chat**: phiano's dialogue (topic ⇒ goal, docs/87), trained on Phi-4's own
  answers instead of a book.
- **Coding**: the same generator over code tokens, with the scope it is in as
  the condition *c* (open function, open block, expected type), and the
  compiler as `better`/`worse` (docs/86): a continuation that stops the code
  compiling is refuted.

## 5. What has to become true (the walls, measured today)

1. **A word holds 32–64 relations**, each (kind, answer, angle, count) — the
   paradigm as written. Today a word holds one angle per kind, so answers
   blur into other words' (cat · is-a ⇒ mammal tied with allergies; docs/89).
2. **Relations it was not told must be readable.** Held out: 0.005 MRR, chance
   0.017 (docs/89). Without this the model is a lookup table, and asking
   Phi-4 about 62,500 tokens at ~70 s each is 49 days of CPU.
3. **Conditions longer than 8 tokens.** Code and chat need state across a
   function or a conversation; the offset channels see 8 ahead. The condition
   *c* (scope, speaker, topic) has to be carried as its own channel — which
   extension channels make possible: a condition is a channel id like any
   kind.

## 6. The scoreboard

Printed together, every run, against the model's size in angles:

| capability | measure |
|---|---|
| knowledge | MRR on held-out relations (today 0.005; chance 0.017) |
| generation | perplexity on held-out Rust-book and Python-doc chapters |
| chat | agreement with Phi-4 on questions it never saw (relations match, Phi-4's answer's likelihood, judge preference) |
| coding | share of generated snippets that pass `cargo check` / `ast.parse`; share that pass a small test |
| size | angles used, of the chosen budget (8M / 16M / 64M); channels in use |

The thesis holds if every row climbs as the model grows toward its budget,
and the 8M, 16M and 64M points show what each doubling buys.

## 7. Order of work

1. **Extension channels and relation slots**: the sparse channel list beyond
   the base 64, one registry for channel ids, and several answers per
   (token, kind) — fixes wall 1 and unlocks wall 3; re-measure docs/89.
2. **Corpora**: Rust book, Python docs and gemphi's Rust ingested; code
   relations from the parser (`syn` for Rust, `ast` for Python).
3. **Quest v2**: Phi-4 asked for sentences and short answers as well as
   relations; a fifth held out for the agreement test.
4. **The scoreboard** above, with the angle count.
5. **Scale**: seeds from the Rust book and Python docs' own vocabulary;
   `phi4-mini` where speed matters more than depth.
