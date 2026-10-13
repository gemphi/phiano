# 157 · The open stratum and the real-world prior

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/156 found the first lever on the open stratum — the part-of-speech frame. This doc tests the data axis: the claim that the open stratum is starved of data, and that supplying data (a dictionary, real-world frequency, usage) reads it. The claim is right in one specific way and wrong in another, and the measurements say which.

## What was tried (all on held-out *Emma*, `bin/ladder`)

The sandbox cannot fetch corpora (Hugging Face, Project Gutenberg, NLTK's data server all 403), but two things install with their data bundled and work offline: **Open English WordNet** (`wn`, `oewn:2024`) and **wordfreq** (real-world frequencies from Wikipedia, subtitles, news, books — billions of words). From WordNet a full free dictionary was built: `data/lex/dict.jsonl`, 84,094 entries, each with its part of speech, supersense, up to three definitions, synonyms, is-a chain and antonyms.

Four data interventions were measured as reader rungs:

| intervention | open stratum (top-1 · MRR) | against frequency |
|---|---|---|
| frequency (Emma prior) | 0.0% · ~0.022–0.025 | — |
| WordNet synonym smoothing | 0.0% · 0.013–0.017 | **worse** |
| last word's kin (co-occurrence) | 0.0% · 0.020–0.022 | level |
| POS frame (docs/156) | 0.0–0.9% · 0.026 | +~15% |
| **real-world prior (wordfreq)** | **0.7–1.5% · 0.040–0.042** | **+~75% (≈2×)** |
| dictionary glosses added as text | — · — | **hurt the told stratum** (20%→15.5%) |
| WordNet examples added as text | — · — | neutral on open, told held ~18% |

## What this says

**The real-world frequency prior is the one lever that works.** A word *Emma* never said — `threshold`, `outcomes`, `ego` — has no rank in Emma's counts, but wordfreq knows it is common English. Re-ranking the open stratum by real-world frequency roughly **doubles** its MRR and gives the first non-zero top-1 there. This is the strongest single result on the open stratum across docs/152–157.

**Adding the dictionary as *text* does not help, and the register matters.** Feeding WordNet glosses ("*able is usually followed by…*") into the counts **hurt** the reader: definitional register is not narrative usage, and 37k dictionary headwords dilute every prediction. WordNet's real example sentences ("*she exceeded our expectations*") kept the register clean (the told stratum held at ~18% instead of collapsing) but were too few — 44k short sentences — to cover the specific continuations Emma tests, so the open stratum did not move beyond the wordfreq prior. The user's principle — meaning is use (PI §43) — is not refuted: it says the *use* that becomes meaning must be real-register prose at volume, which is exactly the data the sandbox cannot fetch (author corpora) and which a local model (phi-4) would have to generate.

**The dictionary's value is as structure, not as text.** POS and supersense (frame), is-a and synonyms (kin candidates), and — most of all — frequency are what the dictionary contributes. The glosses and examples, as flat training text, are not worth their noise at this scale.

## The working reader

Told positions are read by the levels (last word, pair — ~20%). Open positions are read by the **real-world prior re-weighted by the POS frame** (~0.042). A single reader that does both is the levels moving from the real-world, POS-framed prior; it keeps the told stratum at its best (~20%) but the levels still override the prior on open positions, so the clean open gain (0.042) is only fully realised when a position is known to be open. Routing by the levels' confidence recovers most of it. The next build is to make the real-world prior the reader's default base (`Knowing::with_real_prior`), so the shipped system reads the open stratum by real-world frequency, not one novel's counts.

## What is still missing

Volume of real-register usage. The sandbox cannot fetch it; the open paths are (a) author corpora dropped into the workspace by hand, or (b) phi-4 on the user's machine generating natural usage sentences (not definitions) per word, in persona voices — the recursive "accumulate use → meaning" loop, done in the right register. Both are preserved as apparatus: `scripts/gen_use.py` (gloss form, parked), `scripts/gen_examples.py` (example form), ready to point at a larger or model-generated source.

`ladder data/text --in text/emma --n 900 --frame pos` reproduces the table. `wn`, `wordfreq` via pip; `scripts/export_dict.py` builds `data/lex/dict.jsonl`; frequencies in `data/lex/freq.tsv`.
