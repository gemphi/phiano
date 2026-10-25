# 154 · A phase-preserving operator, and a pumped mirror

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/153 left two things to build. The Joint FNO reads the relation backwards only by running its filter from the other end, because its activation does not commute with conjugation or with a global phase. And the mirror was unit-modulus — RotatE — so its conjugate was its exact inverse but it had no gain to spend. This doc builds both missing pieces: an FNO that keeps phase, and a mirror that pumps.

## 1. A phase-preserving activation (`phinet::Tape::squash`)

The GELU in the FNO is applied to the real and imaginary parts separately, so it breaks both symmetries (docs/153 §5.2). The replacement is

**`σ(z) = z · tanh|z| / |z|`**,

which shrinks a sample's modulus and leaves its phase alone. It commutes with a global phase, `σ(e^{iφ}z) = e^{iφ}σ(z)`, and with conjugation, `σ(z̄) = conj σ(z)`. `squash.rs` has the forward map, the closed-form gradient (the radial `tanh|z|/|z|` plus its slope along the ray), and a test that checks the gradient against finite differences and the phase against the modulus.

**`Tie::Phase`** (`phical::operator`) is the circle operator built from it: the same FFT spectral path, but no biases and `squash` in place of GELU. Every step is then complex-linear or acts on the modulus alone, so the whole operator commutes with a global phase. `Fno::defects` measures it:

| over 1,000 word inputs | `Tie::Circle` | `Tie::Phase` |
|---|---|---|
| ‖F(e^{iφ}x) − e^{iφ}F(x)‖ / ‖F(x)‖ | 0.78–0.99 (grows with training) | **< 1e-12** |

The test `the_wave_tie_commutes_with_a_global_phase_and_the_circle_does_not` asserts both. The two operators now share one forward graph (`operator/graph.rs`), written once for `Fno` and for the second operator of a `Sequential`, which had duplicated it and silently ignored the tie.

## 2. The phase operator reads held-out *Emma* (`operator_run --tie phase`)

Same dev split as docs/152 (train outside 10%, fit the mixture on the 5% between, measure on the held-out 5%; the word itself is read, docs/152). At epoch 24, R = 64:

| reading (top-1 · MRR) | `Tie::Circle` (docs/152, ep 24) | `Tie::Phase` (ep 24) |
|---|---|---|
| forward, told stratum (8,147) | 17.8% · 0.291 | 16.9% · 0.283 |
| forward, open stratum (2,431) | 2.4% · 0.056 | 1.7% · 0.050 |
| forward, mixed with the prior | 17.9% · 0.293 | 17.1% · 0.286 |
| **backwards, the pair was told (8,236)** | 17.4% · 0.248 | **17.4% · 0.260** |
| backwards, never told | 0.0% | 0.0% |
| the mixed reading's own perplexity | — | 154.9 |

- **The phase operator reads forward a little below the circle** (16.9% against 17.8%). `squash` is a gentler nonlinearity than GELU; some of the circle's forward accuracy was bought by the activation the phase tie gives up.
- **It reads backwards as well as it reads forwards** (17.4% · 0.260), and its symmetry is now exact rather than a defect the reading has to work around. On this operator the forward and the time-reversed readings are the same quality, which is what the symmetry was for.
- The open stratum is unmoved (1.7%), as everywhere: a symmetry of the operator is not a reader of what was never told.

So `Tie::Phase` buys the exact symmetry and loses about a point of forward reading. Whether that trade is worth making for the shipped operator is a decision, not a result; the circle stays the default.

## 3. A pumped mirror (`mirror_run --pumped`)

The unit mirror of docs/153 gave each kind a phase per mode, `H_k = e^{iθ_k}`. The **pumped** mirror gives it a gain as well, `H_k = e^{g_k + iθ_k}`: unconstrained ComplEx (Trouillon et al. 2016). Its score is the cosine `Re⟨H a_w, a_v⟩ / |H a_w|`, and its conjugate `H̄` is now the **adjoint**, not the inverse — four-wave mixing with gain, which can return more than it receives in some modes and less in others (Yariv & Pepper 1977). The gradients stay closed-form (`conjugate/train.rs`); `mirrors saved before gains still load` and reproduce docs/153 exactly. A test trains the gains on a ring and checks the adjoint identity and that the gains survive a save.

Held-out *Emma*, dev split, 256 epochs:

| reading (top-1 · MRR) | unit mirror (docs/153) | pumped mirror |
|---|---|---|
| forward, told stratum | 11.0% · 0.233 | **14.5% · 0.248** |
| forward, mixed with the prior | 13.1% · 0.258 | **15.6% · 0.265** |
| forward, open stratum, mixed | 3.4% · 0.074 | 3.3% · 0.074 |
| backwards, the pair was told | ~13% | 14.5% · 0.248 |
| the mixed reading's own perplexity | 196.9 | 197.8 |

- **The gains buy the told stratum 3.5 points** (11.0% → 14.5%), because a gain per mode is capacity the unit mirror did not have — it closes about a third of the gap to the Joint FNO (18.3%, docs/152).
- **The open stratum does not move** (3.3% mixed, against frequency's 3.4%): the gains fit what was told, like everything else.
- **The conjugate still reads backwards** (14.5%), now by the adjoint rather than the inverse. The amplitude-only reading is still nothing (0.1%): phase carries the reading, pumped or not.

## 4. What this says

The two symmetries docs/153 found missing can each be supplied. `Tie::Phase` makes the FNO commute with phase and conjugation exactly, at the cost of about a point of forward reading. The pumped mirror gives the all-pass filter a gain, which buys a third of the gap to the FNO on the told stratum and turns the free inverse into the adjoint. Neither moves the open stratum — that remains the problem of docs/152, untouched by a symmetry or a gain. What they add to the paradigm is precision: a relation and its reverse are now the same object read two ways, exactly, and the pump that four-wave mixing needs is in the model.

`cargo test -p phical symmetry conjugate` · `cargo test -p phinet squash` · `operator_run data/text --in text/emma --dev 0.10 --tie phase --epochs 32 --out data/runs/operator_r64_s7_dev10_phase` · `operator_run … --tie phase --measure FILE.relate` · `mirror_run data/text --in text/emma --dev 0.10 --pumped --epochs 256 --out data/runs/mirror_r64_s7_dev10_pumped`. Outputs are in `data/runs/operator_r64_s7_dev10_phase/` and `data/runs/mirror_r64_s7_dev10_pumped/`.
