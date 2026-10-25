# 151 · The operator trained on offsets: the words two and three back

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/150 trained the operator for 96 epochs on `word next word` alone. The told stratum improved, but the open stratum did not move: the 23% of held-out positions whose word was never told after the last word or pair stayed at 1.1–1.9% top-1, below frequency's 3.4%. The conclusion was that the input has to change, not the length. This doc changes the input in the smallest way the 8M plan names: the words two and three places on become kinds of their own, `next2` and `next3`, beside `next`. The operator learns one function for each word and one for each kind, as before. What changes is that a word's function is now shaped by three positions of what followed it, not one.

*Later (docs/152):* every operator reading here left the asked word out of its answers, so a word could never be read as following itself (`- -`). The told-stratum figures are about 2.6 points low; the open-stratum figures stand.

## 1. The run (`operator_run --ahead 3`)

- **Rows:** 548,992 told rows: the 185,209 `next` rows of docs/150, plus `next2` and `next3`. Same held-out paragraphs (5%, seed 7), same model (R = 64, width 16, two layers, hinge loss, Adam).
- **Length:** 32 epochs at about 10 minutes each, about 5½ hours in the cloud workspace. One restart, after epoch 9, put the run on a build that also logs the new measurement. Nothing was lost.
- **Measurement:** `strata` still reads `last · next → ?`. A new reading (`operator_run/ahead.rs`) also reads the second-last word under `next2` and the third-last under `next3`, each alone and with the three cosines summed: `Σ_d cos(F(w₋d + next_d), v)`. `--measure FILE` runs it on any saved operator.

## 2. The curve

Top-1 · MRR on the 10,578 held-out positions:

| epoch | told (8,147): `last·next` | told: offsets summed | open (2,431): `last·next` | open: `w₋₂·next2` | open: `w₋₃·next3` | open: offsets summed |
|---|---|---|---|---|---|---|
| 4 | 7.2% · 0.169 | 7.5% · 0.161 | 0.9% · 0.043 | 1.7% · 0.054 | 2.1% · 0.058 | 2.2% · 0.063 |
| 8 | 8.8% · 0.192 | 8.8% · 0.180 | 0.9% · 0.042 | 2.5% · 0.062 | 2.4% · 0.061 | **2.8% · 0.069** |
| 16 | 11.8% · 0.220 | 10.8% · 0.199 | 1.0% · 0.042 | 1.7% · 0.055 | 1.8% · 0.055 | 2.5% · 0.067 |
| 24 | 9.6% · 0.214 | 11.6% · 0.212 | 1.2% · 0.042 | 1.8% · 0.054 | 1.5% · 0.049 | 2.3% · 0.065 |
| 32 | 10.6% · 0.223 | 11.1% · 0.211 | 1.2% · 0.045 | 2.3% · 0.059 | 1.6% · 0.055 | **2.8% · 0.070** |
| next word only, epoch 32 (docs/150) | 15.1% · 0.264 | — | 1.5% · 0.050 | — | — | — |
| *references* | counts 26.7% · frequency 6.9% | | counts 1.2% · **frequency 3.4% · 0.076** | | | |

![the operator trained on offsets](papers/fig/f151_offsets.png)

- **The open stratum moves.** Reading the words further back takes it from the 0.9–1.9% that every `last·next` reading has held (here and across docs/150's 96 epochs) to 2.2–2.8%. The words two and three back alone each read it better than the last word does. Where the last word was never followed by the word that came, the words before it carry something the operator can use. That is the first reading of this stratum above the counts' 1.2%.
- **It does not reach frequency.** The best, 2.8% · 0.070, is still below 3.4% · 0.076. Nothing has beaten frequency on the open stratum yet.
- **It is there from the start.** The open-stratum reading is 2.2% at epoch 4 and 2.8% at epoch 32, with swings of ±0.3 points between. As in docs/150, epochs fit what is told; the gain on the open stratum comes from the offsets themselves.
- **The last word's reading pays for the room.** At equal epochs `last·next` reads the told stratum 3–5 points below the next-word-only operator (10.6% against 15.1% at epoch 32). The same R = 64 table now holds three kinds of what follows a word. On the told stratum the summed offsets recover about half a point of it.

## 3. The operator as the base

The base the levels move from (docs/137, the path's first component since docs/149) now reads every offset the operator was trained on. Each is softened at its own temperature, the one that gives the told rows the most likelihood, and they are mixed with weights counted on told positions by `relate::mixture_weights` (`story/reader.rs`). An operator trained on `next` alone reads as before: one component, weight 1, T = 0.1. It reproduces docs/150's 128.0 and 132.3 exactly.

| base (held-out *Emma*, seed 7; perplexity) | flat | tree | weights counted (next · next2 · next3) |
|---|---|---|---|
| none (the prior) | 137.0 | 141.5 | — |
| offsets, epoch 8, cosines averaged | 144.7 | 149.4 | — |
| offsets, epoch 8, mixture | 137.2 | 141.8 | 0.81 · 0.17 · 0.01 |
| offsets, epoch 32, mixture | 129.4 | 133.7 | 0.85 · 0.13 · 0.01 |
| next word only, epoch 8 | 130.4 | 134.6 | 1 |
| **next word only, epoch 32** | **127.6** | **131.9** | 1 |
| next word only, epoch 96 (docs/150) | 128.0 | 132.3 | 1 |

- **As a base, the offsets operator is behind at equal epochs**: 129.4 against 127.6 at epoch 32. The base is read over every position, and 77% of positions are told, where its `next` reading is weaker. The counted weights give `next2` 13–17% and `next3` almost nothing, so the open stratum's gain is too small a share of the positions to show here.
- **Averaging the offsets' cosines is worse than no base at all** (144.7). Mixing the softened offsets with counted weights is what makes them safe.
- **The next-word-only operator is best at epoch 32, not 96**: 127.6 flat and 131.9 in the tree, slightly below docs/150's epoch-96 figures. The 64 epochs after 32 bought nothing as a base. This is now the lowest perplexity on held-out *Emma*. Top-1 stays at 20.7% throughout.

## 4. What this says

docs/150 asked for a change of input, and the smallest one moved the stratum that 96 epochs could not: the open stratum's best operator reading doubled, from about 1.3% to 2.8%. The words before the last carry what the last word lacks when it was never followed by the word that came. That is the paradigm's sense of a relation reached through what else is known, here measured only for position. The cost is the last word's reading, because one table at R = 64 now holds three kinds. As a base, a reading weighted by how often each offset helps on told rows does not yet beat the plain operator.

Next, in the order this suggests:

1. **Room for the offsets.** The same run at R = 128 on the user's machine (`operator_run.ps1 -Ahead 3`), to see whether the told stratum's cost is the table's size.
2. **The open stratum against frequency.** The offsets summed (2.8%) and frequency (3.4%) read different words. The question is whether a mixture of the two, weights counted on told positions as above, passes frequency on the open stratum.
3. **The default base.** The epoch-32 next-word-only operator (127.6) is the one to load, if a base is loaded by default. That is still a decision about what the shipped model carries.

`operator_run data/text --in text/emma --r 64 --epochs 32 --every 4 --ahead 3 --out data/runs/operator_r64_s7_ahead3` · `operator_run data/text --in text/emma --ahead 3 --measure FILE.relate` · `story data/text --in text/emma --held-out 0.05 --seed 7 --operator 64,32 --model FILE.relate` · `python3 docs/papers/figs151.py`. Outputs are in `data/runs/operator_r64_s7_ahead3/`: `log.md`, `measured/epoch_{4,8}.md` and `as_base_epoch{8,8_mean,32}.txt`. The next-word-only epoch 32 is in `data/runs/operator_r64_s7/as_base_epoch32.txt`.
