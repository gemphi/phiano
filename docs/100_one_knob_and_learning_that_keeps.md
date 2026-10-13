# 100 · One knob, and learning that keeps

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Two claims, measured by `learn_demo` (phiano) on the quest's own tables,
3,679 rows from Phi-4: the cat laws run and the night's eight seeds, split
into three persona files (`phiace/personas/demo`). A fifth of every
persona's rows is held out by a hash of the row. Scores are MRR over every
answer the personas were told; the prior ranks by how often an answer
occurs (the type-only baseline).

## 1. One knob: relations per word

Only R, the number of relations a word holds, is turned. Width 16, 2
layers, 30 epochs, margin, learning rate and the data stay fixed; the
operator's Fourier modes follow R (R/4, at most 16). Three seeds (7, 8, 9),
mean ± spread:

| R | numbers | told MRR | held-out MRR | held-out within kind | held-out hit@5 |
|---|---|---|---|---|---|
| 8 | 30,112 | 0.152 ± 0.010 | 0.065 ± 0.005 | 0.083 ± 0.009 | 9.4% |
| 16 | 59,632 | 0.335 ± 0.017 | 0.087 ± 0.012 | 0.099 ± 0.015 | 12.5% |
| 32 | 118,672 | 0.430 ± 0.051 | 0.105 ± 0.010 | 0.121 ± 0.020 | 17.3% |
| 64 | 236,752 | 0.442 ± 0.097 | 0.112 ± 0.015 | 0.146 ± 0.009 | 19.5% |
| 128 | 472,912 | 0.442 ± 0.077 | 0.113 ± 0.010 | 0.146 ± 0.015 | 18.2% |
| prior | — | — | 0.055 | 0.056 | — |

What it shows:

- **Size is R.** Numbers double with R, exactly: the table is words × R ×
  2, and the operator grows with its modes.
- **Accuracy follows R until the data runs out.** Held out rises from
  0.065 to 0.112 (1.7×) and hit@5 from 9% to 20% between R = 8 and R = 64.
  Within kind it goes from 1.5× the prior to 2.6×.
- **Here it stops at 64.** R = 128 costs twice the numbers and reads the
  same. With 3,679 rows there is nothing more for the extra relations to
  hold.
- **Less data, same knee (measured after, `--share`).** Trained on a share
  of the told rows, with the same held-out rows read (2 seeds each):

  | R | 25% of rows | 50% | 100% (above) |
  |---|---|---|---|
  | 8 | 0.015 | 0.024 | 0.065 |
  | 16 | 0.025 | 0.040 | 0.087 |
  | 32 | 0.038 | 0.068 | 0.105 |
  | 64 | 0.049 | 0.070 | 0.112 |
  | 128 | 0.054 | 0.071 | 0.113 |

  (held-out MRR; the prior is 0.055.) Data lifts every R; the knee sits at
  32–64 at every share. The guess that more rows would move the knee up is
  **not** supported in this range: between 900 and 3,700 rows, the
  number of rows sets the accuracy, and R stops paying at about the same
  place. At 25% the model is below the prior on held-out rows while its
  told MRR is 0.80: it memorises.
- **Told MRR varies with the seed at R ≥ 32** (0.33–0.50 at R = 64). Held
  out varies less, so the table does not depend on it, but it is
  training noise, not settled.

This matches the rule the project set: the number of relations per word is
the size knob, and the size worth paying for is set by what the quest has
told the model. At about 4% of Phi-4, as planned, that means many more rows
than 3,679.

## 2. Learning one persona after another

Outdoors (cat, tree, river; 1,777 rows), then Workshop (hammer, car,
computer; 958), then Town (bread, doctor, music; 944), all learned into
**one** model at R = 64. Each stage grows the model's table with
`RelationModel::extend` (new words get new rows; old rows and the operator
are kept), trains 30 epochs, and saves `stepN_<persona>.relate`. After each
stage every persona learned so far is read again. Seed 7:

| after | read | naive: told / held out | keep: told / held out / within |
|---|---|---|---|
| Outdoors | Outdoors | 0.542 / 0.098 | 0.542 / 0.098 / 0.115 |
| Workshop | Outdoors | 0.444 / 0.101 | 0.524 / 0.114 / 0.143 |
| Town | Outdoors | **0.387** / 0.079 | **0.505** / 0.115 / 0.160 |
| Town | Workshop | 0.480 / 0.094 | 0.523 / 0.122 / 0.160 |
| Town | Town | 0.573 / 0.126 | 0.524 / 0.125 / 0.164 |

*Naive* trains each stage on the new persona only; *keep* trains it on the
new persona plus every earlier told row (replay: the relations are the
memory, so they are kept, not the text).

- **Naive forgets.** By stage 3, Outdoors' told MRR has fallen from 0.542
  to 0.387 and its held out to 0.079. In the naive model, cat · is-a gives
  *food* first, and tree · made-of gives *iron, cells, linen*.
- **Keep keeps, and each persona helps the others.** Outdoors' told MRR
  stays at 0.505. Its held out *rises* from 0.098 to 0.115, and within
  kind from 0.115 to 0.160, after Workshop and Town were learned. Words
  from other files add to what the model can infer for this one. The
  newest persona costs a little (0.573 → 0.524 told) for holding all three.
- The kept model after stage 3 (`relate_ask`):
  `cat · is-a ⇒ mammal, animal, feline, pet` ·
  `hammer · used-for ⇒ toolkit, carpentry, toolbox` ·
  `bread · made-of ⇒ flour, baked-good, grain-product`.

Repeated on seeds 8 and 9 (`--only stages`): Outdoors after Town,

| seed | naive told / held out | keep told / held out |
|---|---|---|
| 7 | 0.387 / 0.079 | 0.505 / 0.115 |
| 8 | 0.390 / 0.102 | 0.480 / 0.127 |
| 9 | 0.277 / 0.088 | 0.339 / 0.098 |

Keep beats naive on every seed, told and held out, and on every seed the
earlier persona's held-out score is higher after the later personas were
learned than before (0.098 → 0.115, 0.100 → 0.127, 0.085 → 0.098).

## The demo on the GPU machine

`scripts/demo.ps1` runs it end to end:

1. Checks the GPU (nvidia-smi) and `ollama ps`.
2. Copies the three personas' tables and asks a fourth persona live:
   Kitchen (knife, soup, oven), words no earlier quest asked. It runs one
   quest per seed, all three at once, so the GPU answers them in parallel.
3. Runs `learn_demo` on all four, with the knob sweep and the four stages.
4. Asks the last model about a stage-1 word and a stage-4 word.

The GPU makes Phi-4 answer faster, and nothing else. phinet trains on the
CPU: one thread, about 2 minutes per stage at R = 64.

Reproduce here: `cargo run --release --bin learn_demo -- data/demo
../phiace/personas/demo/outdoors.md …/workshop.md …/town.md --sweep
8,16,32,64,128 --epochs 30 [--seed 8 --only knob]`.
