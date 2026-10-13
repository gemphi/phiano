# 65 - Phinet: The Fourier Tensor Engine

> _A word is a function on the relation circle. Phinet learns operators between such functions._

---

## 1. What it is

`src/phinet/` is four modules and no new dependencies:

| Module | What |
|---|---|
| `tensor.rs` | Complex tensors `[batch, width, space]`, a reverse-mode tape, and a unitary radix-2 FFT as a differentiable op. Ops: `add`, `add_bias`, `mul`, `scale`, `linear` (pointwise across width), `mode_mix` (one complex matrix per Fourier mode), `fft`/`ifft`, `gelu` (on Re and Im), `cosine`, `hinge`, `mean_real`. Adam on `(Re, Im)` pairs. Every gradient is checked against finite differences in `test_gradients_match_finite_differences`. |
| `space.rs` | The function space. `encode` reads a `SpectralPhasor` as `f(k) = a_k·e^{iθ_k}` over the 64 relation channels — `a_k` the evidence when the word has any, flat otherwise — normalised to unit energy. `FunctionSpace::from_facet` encodes a whole lexicon; `context` is the engine's own superposition of the preceding words as a function; `examples` builds deterministic train/held-out `(context → next word)` pairs. |
| `operator.rs` | The Fourier neural operator: `h ← σ(F⁻¹(R·F h) + W h + b)` stacked, with a lift and a projection. `R` is a learned complex `width×width` matrix per kept Fourier mode — a learned circular convolution over the relation channels. `modes = 0` removes the spectral path and is the pointwise control. Trained by hinge on cosine against a sampled decoy, the ranking objective the trainer already uses on phases. Saves and loads its parameters. |
| `reader.rs` | Opens the existing model files and reports what is in them. |

`cargo run --release --bin phinet [corpus] [sentences] [epochs]` runs the reader and the measurement below.

## 2. Why an operator, and why Fourier

Everything in the engine before this was a sum, a multiplication by a constant, or a lookup, followed by argmax. `nonlinear.rs` says so in its own header. The promotion gave every word 64 sides, but no operation could learn that side 3 and side 40 *together* mean something neither does alone. That is what a learned operator on the function space is for.

A dense 64×64 complex matrix would mix channels too. But the channels are not an arbitrary index: the relation inventory hands them out in rank order — offsets first, then markers by how often they link — and a *filter* over that ordering is a statement about neighbouring relations. Keeping only the low modes (12 each side of zero, here) is the regulariser: relations of similar rank are made to behave alike and the exceptions have to earn their high-frequency coefficients. It also makes the operator resolution-free; the same `R` applies to a 32- or 128-channel circle. This is the Fourier neural operator of Li et al. (2020) with the relation circle as its spatial domain.

## 3. What is in the existing models

`data/manifold.chroma` (92 MB) did not load at all before this. The header says format v1, but the phasor inside is the 20-byte `{phase, amplitude, band_n}` written before `count` and `packed` existed, and bincode reads positionally, so `#[serde(default)]` never applied: the reader took the next word's length prefix as part of a phasor and asked for 47 GB. `Storage::read_v1_lexicon` now parses v1 bodies by hand, trying each historical record width and accepting the one that parses the lexicon to the header's word count and the tables behind it to end of file.

```
data/manifold.chroma  (92.2 MB)
  facet, format v1: 216097 words, 0 with relation evidence, 0 marker channels
  n-grams: 194788 bigram contexts, 535124 trigram contexts, 623938 phase lags
  channel-0 dispersion 0.288 (all); other channels 0.998
  amplitude mean 1.158 max 2.000; band_n max 1845381
  note: no observation counts stored (pre-`count` layout)
  note: no relation evidence: every word is a flat function
  note: band_n reaches 1845381: incremented on every converged step, carries no information

data/memory.chroma  (3.4 KB)
  memory log: 2 entries — a context-wave log, not a lexicon
```

So the shipped model is 216,097 words with **one learned angle each**, and that angle has dispersion 0.288 over the vocabulary: most of it has collapsed toward one direction. The 63 other channels are identity seeds, learned by nothing. `memory.chroma` had the same positional-default bug (written before `count`); `Memo::load_from_file` now bounds its allocation to the file size and falls back to the older layout.

## 4. What the operator learns

`bin/phinet data/prose_corpus.txt 20000 4` — WikiText-2 prose, 20,000 sentences, held-out sentences by hash, next word ranked among the ~2,200 words the corpus uses 25+ times. Chance MRR for that pool is `H_N/N ≈ 0.0037`. The identity arm is the engine's existing method — superpose the context, rank by cosine — read in function space.

```
                                          MRR     hit@10   vs chance
shipped v1 manifold (flat functions)
  identity (superposition)              0.00993    0.020      2.6x
  pointwise control (modes=0)           0.01088    0.018      2.8x
  fourier operator (modes=12)           0.04444    0.086     11.6x

promoted lexicon (evidenced functions, trained on the same 16k sentences)
  identity (superposition)              0.02473    0.075      6.7x
  pointwise control (modes=0)           0.07010    0.211     18.9x
  fourier operator (modes=12)           0.10309    0.286     27.9x
```

8,360 parameters in every learned arm. Three readings:

* **The spectral path is what works.** On the flat space the pointwise control cannot do anything the identity does not — every channel has the same amplitude, so a map that cannot move information between channels has nothing to act on — and it measures the same. The Fourier operator, on the same input, is 4.5× better. On the evidenced space the gap is 18.9× → 27.9×.
* **The promoted lexicon gives the operator more to work with.** Identity 2.6× → 6.7×; operator 11.6× → 27.9×; hit@10 0.086 → 0.286. Evidence is where a pointwise map gets its purchase (the control goes from useless to 18.9×), and the spectral filter adds on top of it.
* **This is the first learned thing in the engine.** Two epochs on 5,000 sentences already gives 14.6× on the evidenced space; the loss is still falling at four epochs on 20,000.

The trained operators are saved as `data/phinet_shipped_v1_manifold.fno` and `data/phinet_promoted_lexicon.fno` (`Fno::from_bytes`).

## 5. What it does not claim

The task is next-word ranking against a frequent-word pool with a window of six, on 20k sentences, one seed. It is not the perplexity harness and is not comparable to the Kneser-Ney number. What it establishes is narrower and, for the architecture, more important: on the representation the engine actually stores, a learned filter in the spectral domain of the relation circle recovers structure that superposition cannot, and it recovers more of it when the sides of the polygon carry evidence.

## 6. Next

* Put the operator in the generation path: `generate.rs` ranks candidates by `resonance` against a superposed context; `Fno::apply` on the context function, then cosine, is a drop-in scorer and `bin/evaluate` will say what it is worth in perplexity.
* Train the operator and the lexicon together: gradients already reach the input functions (`cosine` back-propagates into both arguments), so the phases and evidence of the words themselves can be parameters.
* Compose: `unbind` through the operator rather than through phase subtraction — the recursion the `roles.rs` identity was missing.
