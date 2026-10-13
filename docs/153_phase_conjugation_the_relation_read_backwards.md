# 153 · Phase conjugation: the relation read backwards, the matched filter, and the mirror

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

The user asked what pumped conjugate waves, phase and amplitude, phase conjugation and time-reversed waves are in Phinet and Phiano, and how our Fourier operators would work with them. They also asked what radar and Tesla's magnifying transmitter offer. The physics carries over as mathematics, not hardware.

- **Phase conjugation:** a phase-conjugate mirror returns z\* for every z = A·e^{iθ} that reaches it. For one frequency that is the wave run backwards in time.
- **Four-wave mixing:** the mirror is made with two strong pump waves and the probe. The conjugate is proportional to pump₁ · pump₂ · probe\*.

This doc maps those pieces onto the model, measures what our operators already do, and builds the one that is exact.

## 1. What the readout already is

Both learned readers answer `w ⇒ ?` the same way. They produce an output function q and correlate it with every word's function: `Re ⟨q, a_v⟩ / (|q| |a_v|)`. The code is `phinet::cosine`, which is Σ x·conj(y). That is a **matched filter**: the received wave times the conjugate of each template, the core of radar detection. The product of a wave with a conjugate was in the model from the start (docs/113).

A matched filter can be read two ways. Coherent detection keeps the real part, which needs the phase to line up. Envelope detection keeps only the modulus |⟨q, a_v⟩|, the amplitude. `waves/strata.rs` reads every held-out position both ways (table in §4). **Amplitude alone reads 0.1% top-1 on the told stratum; the phase-coherent reading reads 18.3% with the same operator.** Everything the operator has learned is carried by phase.

## 2. Our FNO has no conjugate

For time reversal to come for free, the operator would have to commute with conjugation: F(x̄) = conj F(x). Reading a phase-shifted wave would also require it to commute with a global phase: F(e^{iφ}x) = e^{iφ}F(x). `FnoWave::defects` measures both over 1,000 of the trained operator's inputs:

| Joint FNO, R = 64 (docs/152's dev split) | epoch 8 | epoch 16 | epoch 32 |
|---|---|---|---|
| ‖F(e^{iφ}x) − e^{iφ}F(x)‖ / ‖F(x)‖, φ = 1 rad | 0.78 | 0.90 | 0.99 |
| ‖F(x̄) − conj F(x)‖ / ‖F(x)‖ | 1.85 | 1.85 | 1.87 |

These are as large as the outputs themselves. Three things in the code cause it:

- the GELU is applied to the real and imaginary parts separately, and GELU(−x) ≠ −GELU(x);
- the lift and every layer add a bias;
- the weights are complex.

Training makes the phase defect larger. The FNO can still be read backwards (§4), but only by running the filter from the other end: F(a_u + a_next) for every word u, correlated with the word that came after. That is one forward pass per word, 6,877 in all, cached.

## 3. The mirror (`phical::conjugate`)

The exact case is the FNO's spectral path cut to one channel and to |R_k| = 1. **A word is a unit spectrum a_w (R = 64 Fourier coefficients). A kind is one phase per mode, H_k = e^{iθ_k}: an all-pass filter.** The score is

`s(w, kind, v) = Re ⟨H a_w, a_v⟩ = Re Σ_k e^{iθ_k} a_w(k) conj(a_v(k))`,

which is the cosine, since |H a_w| = 1. Because H is unitary, `Re ⟨H a_w, a_v⟩ = Re ⟨a_w, H̄ a_v⟩`. So:

- **H̄ a_v is the time-reversed wave.** It reads which word v came after, with nothing trained for it.
- **Hⁿ** carries a word n steps on.
- **H⁻¹ = H̄.**

This is RotatE (Sun et al. 2019), or ComplEx with |r| = 1 (Trouillon et al. 2016), written as a Fourier operator.

**Training.** The rows, decoys, hinge and margin are those of the Joint FNO. Every gradient is closed-form, because the score is bilinear: ∂s/∂a_w = H̄ a_v, ∂s/∂a_v = H a_w, ∂s/∂θ_k = −Im(H_k a_w(k) conj a_v(k)). Updates use lazy Adam, with the touched words renormalised after each step.

**Tests.** `conjugate::tests` trains a ring of twelve words told only `w_i next w_{i+1}`. Read forward, it gives w_{i+1}. Read by H̄, it gives w_{i−1}, which was never told. Read by H², it gives w_{i+2} for most of the ring.

**Cost.** An epoch over 174,107 rows takes about 1 second; the FNO's takes about 200.

## 4. Held-out *Emma*: the FNO and the mirror on the same rows

**Setup.** Both readers are trained on the told paragraphs outside a 10% held-out share (174,107 `next` rows, seed 7). The mixture with the prior is fitted on the 5% between the two shares, paragraphs neither reader saw (docs/152). Both are measured on the same 10,578 held-out positions. The FNO is read without excluding the word itself (docs/152 §1). Top-1 · MRR:

| reading | Joint FNO, 32 epochs (about 2 h) | mirror, 128 epochs (2 min of training) |
|---|---|---|
| **forward, told stratum (8,147)**: last word | **18.3% · 0.296** | 14.1% · 0.245 |
| forward, told: amplitude only | 0.1% · 0.005 | 0.1% · 0.003 |
| forward, told: mixed with the prior | **18.4% · 0.299** | 16.7% · 0.272 |
| **forward, open stratum (2,431)**: last word | 1.9% · 0.052 | 0.7% · 0.032 |
| forward, open: mixed with the prior | 3.2% · 0.074 | 3.2% · 0.074 |
| the mixed reading's own perplexity (no counts) | **146.6** | 196.9 |
| prior weight counted on unseen rows | 0.085 | 0.257 |
| **backwards, the pair was told (8,236)** | **17.4% · 0.248** | 14.1% · 0.227 |
| backwards, told: amplitude only | 0.0% · 0.003 | 0.0% · 0.004 |
| backwards, the pair was never told (2,443) | 0.0% · 0.008 | 0.0% · 0.004 |
| what reading backwards costs | one FNO pass per word (6,877), cached | the conjugate filter H̄, nothing more |
| *frequency* | told 6.9% · open 3.4% · backwards 7.4% / 1.1% | |

![the relation read backwards](papers/fig/f27_back.png)

- **The relation read backwards works, untrained, for both.** Where the pair was told, the word before is found 14–17% of the time: twice frequency's 7.4%, with no `prev` row ever learned. In the mirror this is exact: H̄ is H's inverse. In the FNO it comes from correlating the forward filter from the other end, at one forward pass per word.
- **Neither reaches a pair that was never told** (0.0%). The conjugate returns what the forward wave carried; it does not invent.
- **The FNO reads forward better** (18.3% against 14.1%; perplexity 147 against 197). The mirror has no width and no nonlinearity: one phase per mode is all a kind can do. In exchange it trains about 200 times faster per epoch, and its reverse reading costs nothing.
- **Powers do not compose in language.** The mirror's H² read from the word two back gets 1.2% on the told stratum, and H³ gets 0.1%. In the ring test, H² worked. Two steps on in *Emma* are not the same relation applied twice. When the mirror was trained on `next2` and `next3` as kinds of their own (`mirror_run --ahead 3`), its `next` reading fell from 14% to 4.7%. One spectrum per word could not hold three phase relations; the FNO, with width, fell from 15.1% to 10.6% (docs/151, read then with the word itself left out).

## 5. What radar and resonance give, and what they do not

- **Matched filtering:** in the model, and the reason phase carries everything (§1).
- **Coherent integration:** radar sums N echoes as complex values after aligning their phases. docs/151's "offsets summed" was a coherent sum of real parts. It helped only where the offsets were trained as separate kinds. Under the mirror's powers it lowers the told stratum (5.9% against 14.1%), because H² and H³ do not carry the next word (§4).
- **Envelope detection:** throws the phase away, and with it the reading (§1).
- **Beamforming:** the counted mixture weights (`relate::mixture_weights`) play the part of array weights, fitted on data.
- **CFAR (a detection threshold set against the local noise):** the D2 post plays the same part, KL ≥ τ against the prior.
- **Resonance (Tesla's magnifying transmitter is a high-Q resonant transformer):** what carries over is a driven oscillator storing energy where the drive matches. Here a relation told many times is a stored count, the amplitude, and the softening temperature sets the reading's bandwidth. It was fitted at T = 0.1 for every reader here.

Nothing in this doc depends on any claim about wireless power, which was never demonstrated at distance.

**The pumps.** In the model the pump is the counts: they hold amplitude, how often a relation was told. The operators hold phase, which relation it is. The reading that uses both is the mixture of docs/152, with weights counted on rows neither saw.

## 6. What this says, and next

The conjugate exists in Phient's reading in three places:

1. **The readout:** a correlation with the conjugate, where phase is all that is read.
2. **The mirror:** H̄ reads the relation backwards exactly, with nothing trained.
3. **The FNO:** reads backwards only by running the filter from the other end, because its activation breaks the symmetry.

Reading backwards finds the word before twice as often as frequency where the pair was told, and never where it was not. Phase conjugation inverts what was told; it does not reach what was not, so the open stratum still needs something else.

Next:

1. **An FNO that commutes with phase.** A modReLU activation (z · σ(|z| + b)/|z|) and no biases would give the FNO's width with the mirror's free inverse. It can be measured against both, on the same split.
2. **A pumped mirror:** a gain |H_k| ≠ 1 per mode, which is ComplEx unconstrained. The conjugate is then the adjoint, not the inverse.

`cargo test -p phical conjugate` · `mirror_run data/text --in text/emma --epochs 256 --every 32 --dev 0.10 --out data/runs/mirror_r64_s7_dev10` (`--ahead 3`, `--measure FILE`) · `operator_run data/text --in text/emma --dev 0.10 --measure data/runs/operator_r64_s7_dev10/epoch_32.relate` · `python3 docs/papers/figs27.py`. Outputs are in `data/runs/waves/` and `data/runs/mirror_r64_s7_dev10{,_ahead3}/`.
