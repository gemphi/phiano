# 74 · One pair key, two engines, sixty-four gates, and a persona called Jane Austen

> In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models — Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle (sphere) with a count of how often it has been played out; meaning is a side effect of pairs of words moving under a condition.

## 1. The pair key is one definition

`RelationVocab::channel(tokens, i, j)` — the rarest marker word between *i* and *j* (channels 8–31), else the offset *d − 1* (channels 0–7) — was the trainer's key (`couple_relations`) and the relational scorer's key (`Facet::relational_score`), but `Composition` keyed pairs by offset alone. Now `Composition::from_corpus_keyed(space, sentences, pool, PairKey::Channel(&vocab))` uses the trainer's key; `PairKey::Offset` keeps the old bench. Test: `pair_key_is_one_definition`.

Measured (`bin/pair_key`, Rust Book, 704 words, chance MRR 0.0101):

| key | fillers/word | phase-only L1–4 | cleanup K=3 L1–4 |
|---|---|---|---|
| offset | 4.0 | 59× 19× 8× 3× | 80× 61× 31× 1× |
| channel, all fillers | 19.9 | 6× 5× 2× 1× | 19× 7× 31× 1× |
| channel, 4 best-evidenced (`MAX_FILLERS = 4`) | 4.0 | 52× 36× 13× 8× | 81× 66× 57× 3× |

A word bound to twenty conditions does not fit on a 64-sample circle. Four does, and the trainer's key beats the offset key at every depth past the first.

## 2. Difference Engine vs Analytical Engine, in Lovelace's words

Verbatim excerpts saved in `data/corpora/lovelace/notes_excerpts.md` (Notes A, D, G; fourmilab and MacTutor transcriptions; gutenberg.org is unreachable from the build machine).

- "The Analytical Engine weaves algebraical patterns just as the Jacquard-loom weaves flowers and leaves."
- "While the Difference Engine can merely tabulate, and is incapable of developing, the Analytical Engine can either tabulate or develope."
- "The Analytical Engine has no pretensions whatever to originate anything. It can do whatever we know how to order it to perform."

The mapping: `relational_score` is the Difference Engine — fixed gears (channels), each context wheel turning the candidate's wheel through the channel its condition selects, and it can only tabulate what the corpus set into the wheels. The Analytical Engine is store (two-sided lexicon) + mill (Fourier operator) + cards (objectives and `learn::Tuner` weights). The relational reading is the Difference Engine inside the Analytical Engine, and the learned weight on it (+0.70 Rust Book, +0.67 Austen) is the card that says how much to trust it. Her limit is our reporting rule: every number is how well the machine followed the order.

Merits, honestly: the Difference Engine reading is exact, evidence-bounded, and cheap (a few cosines per context word); it cannot generalise past what was coupled. The Analytical reading (operator + learned scorer) generalises — 16× chance on Austen's next word vs 7.9× for the lexicon alone — and pays for it in parameters (8,360) and in the ways it can be wrong (docs/72's proposal salad). Neither replaces the other; the scorer weights them.

## 2b. Liskov: substitution and the end game (`bin/liskov`)

The "chess-playing machine from another lady" is Barbara Liskov's 1968 thesis (as Huberman) *A Program to Play Chess End Games* — Stanford, not IBM; she then built Venus at MITRE and CLU at MIT. Record: `data/corpora/liskov/notes.md`. In the machine:

- `Facet::relational_fit(toks, i, cand)` — the substitution principle for words: fit under every demanded relation, both sides. Substitution test (Austen, KN judge): as written 173; Liskov stand-in **238**; most-similar word 254; random 254. The point-like reading is chance; the relational one is not. Cloze: fit 4.1×, frequency 5.8×, learned trade 6.1× chance.
- `Facet::calls_for(word, goal)` and `Generator::with_goal(goal, weight)` — the end game: 40 prompts, 30 tokens; reach 2/40 → 6/40 (pull 2), tokens-to-goal 23.5 → 8.8 (pull 4), perplexity 12.4 → 13.3. Works, small; the sampler is the ceiling.
- Killer heuristic → beam ordering: not implemented.

## 3. The I Ching as sixty-four operations (`phinet::hexagram`)

Six lines gate six octaves of a 64-sample spectrum: line *j* yang passes octave *j*. `apply` (fft → gate → ifft, exact), `gate`, `changing_lines`, `hexagram_of` (Q64 sector of the centroid), `phase_of`, `novelty = 1 − cos(f, H f)`, `lines` (octave energies). 15 phinet tests. In `generate.rs`, `OperatorGuide.novelty: Option<(id, every)>` passes the operator's prediction through a named gate every *n*-th token — novelty on demand in composition.

Prediction was measured and refuted (`bin/hexagram_eval`, Rust Book): operator's predicted hexagram = next word's hexagram 0.9%; identity 5.8%; previous word 3.6%; King Wen successor 1.1%; chance 1.6%; "most frequent, always" 12.8% (sector skew). A keyboard, not an oracle. The operations stay.

## 4. A persona called Jane Austen (`bin/austen`, papers/15)

Corpus: *Emma*, *Persuasion*, *Sense and Sensibility* (NLTK Gutenberg mirror; P&P, Mansfield Park, Northanger Abbey, Lady Susan missing — gutenberg.org blocked, see `SOURCES.md`); contrast: Chesterton ×3, Carroll. Attribution on held-out paragraphs 64% (Austen 61%, Chesterton 71%, Carroll 46%). Operator on Austen 16.1× chance. Learned scorer 12.94 → 10.95.

Two defects found and fixed on the way:

1. **Hard stop.** `generate.rs` ended a composition at four consecutive function words. "With me and" was a whole 60-token composition. Replaced by `FUNCTION_STREAK_LIMIT`: the run demands a content word (candidates filtered), and only ends if none can follow. Long compositions now reach 60 tokens.
2. **Leakage.** The attribution lexicon holds every author, so the composer borrowed Carroll's "white rabbit with pink eyes". Composition now reads `data/austen_voice.chroma`, trained on Austen alone.

After both: long compositions read KN ≈ 11 against her own ≈ 190 — stitched trigrams, distinct-token ratio 0.70–0.78. That ratio is the next number to move. The I Ching arm raises the evaluator slightly (0.689 vs 0.669) and perplexity slightly (12.2 vs 11.6): novelty costs exactly what it should.

Persona fingerprints now use the target-side function centroid (`relational_sector`) instead of channel-0 phase; sectors still bunch at 45–49 (histogram similarity 0.991 across authors) — the centroid skew of §3 again, open.

## 5. Open

- Persona sector skew; a channel-balanced sector would separate authors better than 64%.
- Distinct-token ratio of long compositions (stitching).
- The three missing novels: drop the Gutenberg texts into `data/corpora/austen/` and re-run `bin/austen`.
- Killer heuristic in the beam; replace the trigram sampler so the end-game pull has room.
