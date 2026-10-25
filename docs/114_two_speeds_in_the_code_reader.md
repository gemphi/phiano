# 114 · Two speeds in the code reader: no gain (a null result)

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Trial** (`code_reader/own.rs`, `--own N`). The two-speed posts of docs/113 were carried over to the code reader.

- Every condition "`w` stood `d` back" keeps its kind's weight α[d, sightings] from docs/110. That is the fast speed.
- A condition seen at least N times in the counts may also earn its own weight, learned on the development files and pulled back toward its kind's (λ = 0.01).
- The own weights are keyed by the token's text, so they carry from the development counts to the test counts.
- 3 epochs over 10,000 development positions; base 4-gram, K = 8.

## Results (test; split 0 / split 1)

| reader | top-1 | top-5 | perplexity | identifiers top-1 | identifiers perplexity |
|---|---|---|---|---|---|
| 4-gram | 55.1 / 57.1% | 78.4 / 80.2% | 10.2 / 8.7 | 25.4 / 27.7% | 145 / 103 |
| 4-gram ⊕ movement, weights per kind (docs/110) | 59.7 / 61.9% | 81.3 / 83.1% | **7.5 / 6.6** | 34.5 / 38.2% | **62.9 / 46.1** |
| two speeds, own weight after 16 | 60.0 / 62.0% | 81.2 / 82.9% | 7.7 / 6.7 | 35.0 / 38.2% | 69.9 / 52.2 |
| two speeds, own weight after 64 | 60.0 / 62.0% | 81.2 / 82.9% | 7.6 / 6.7 | 35.0 / 38.3% | 69.6 / 51.7 |
| two speeds, own weight after 256 | 60.0 / 62.0% | 81.3 / 83.0% | 7.6 / 6.7 | 34.6 / 38.1% | 68.8 / 50.8 |

(The corpus includes the reader itself, so the baselines differ by a few tenths from docs/110.)

## Reading

- **No gain.** Top-1 rises by 0.1–0.5 points, which is within the noise of editing the corpus. Perplexity gets slightly worse, most on identifiers (63 → 69, and 46 → 52).
- **What the own weights learned.** At N = 64, 1,843 conditions earned their own weight. Those that moved most are near punctuation and keywords: `mut`@1 −11.4, `&`@1 −10.5, `pub`@1 −7.2, `(`@1 −5.1, `,`@1 −5.0. Each is switched off, because the 4-gram already carries what one token back says. They are not new tendencies. The kind weights of docs/110 had already down-weighted these, so switching them fully off gains little and overfits the development files.
- **Why the builders' game differs.** There, Phi-4's tendency lived in particular words (`slab`, `pillar` on the act), and no weight per kind could find it (docs/111–113). In code, what a condition is worth is carried by its kind: how far back it is and how often it was seen. Per condition there is too little left to learn from 10,000 positions.
- **Kept, but not the default.** `--own` is a flag, and its row stays in the report. The docs/110 reader remains the one to beat.

Raw: `114_raw/`.
