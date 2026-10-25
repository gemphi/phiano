# Phinet, the Parent Engine
*Phient papers, 4 of 16 · 2026-02-23 · Phinet, the parent: complex tensors, the tape, the unitary FFT, and gradients checked by finite differences*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

Phinet is the parent crate of the PML family. It holds the machinery that lets anything be learned across the samples of a function on a closed circle. This paper covers the four parts at its floor: the complex tensor, the tape that records and replays operations, the unitary FFT, and the tests that check every analytic gradient against a finite difference. The operator, the joint table, holographic binding, the sixty-four hexagram gates and the sphere all stand on these four parts. They are covered elsewhere in the series and appear here only where they depend on the floor.

A function in Phinet is a `Vec<Complex64>` sampled at a power-of-two number of points. A `Tensor` is always rank three, `[batch, width, space]`, and always `f64` complex. The `Tape` is a vector of nodes. Each node holds a value, an optional gradient, the op that made it and a flag for whether gradient flows into it. The FFT is radix-2 Cooley–Tukey and divides by `√N` in both directions. So the inverse is the adjoint, and the backward of `fft` is one line: `g_x = ifft(g)`.

Correctness is checked twice. In the crate, `test_gradients_match_finite_differences` pushes a small network through every op and compares each partial with a finite difference. Outside the crate, a PyTorch twin replays the same tape with `torch.autograd`. The parity page reports 62 checks, all passing, with the largest difference 9.9e-16. PHIENT_FACTS states the same result as 62/62 parity tests.

Speed was measured on one two-core Linux x86-64 machine. `Fno::apply` fell from 916 µs to 88 µs per function once `apply` ran in chunks of 256. A 64-point FFT pair costs about 1.2 µs. The engine at these sizes is allocation- and bandwidth-bound: every op on a 1 MB tensor costs 2–5 ms forward, `add` included, because each op pays for a fresh allocation.

The paper also reports what is wrong. The finite-difference test in the code is one-sided, while the autograd guide describes a central difference. The docs disagree on how many tests and parity checks exist. Determinism across machines has been verified on one machine only. There is no GPU path, no second derivative and no `f32` compute. Each of these is stated with its source below. Section 12 proposes the next engine experiment, a buffer pool for the tape, with the current engine as its control.

## 2. The claim

The claim of this paper is narrow. Phinet's floor is small, complete for what the paradigm needs, and checked. Small means the whole crate was split, in the last changelog round, into files of under 200 lines each, with `tensor/` alone holding the tape, the FFT, the optimiser and their tests. Complete means every operation the operator, the joint table and the phasor ranges use has a forward rule, a backward rule and a test. Checked means each backward rule has been compared two ways: against a finite difference of the loss in Rust, and against `torch.autograd` in Python on the same numbers.

The claim rests on one design choice about complex gradients. For a real loss `L` of a complex value `z = x + iy`, Phinet stores `g = ∂L/∂x + i·∂L/∂y`. The autograd guide calls this the direction of steepest ascent in the plane. In Wirtinger terms it is `2·∂L/∂z̄`, and it is what torch returns as `.grad` for a complex leaf. Every parameter update is then a real-vector update on the pair `(x, y)`, and `Adam` runs on those pairs. No Wirtinger calculus appears anywhere in the code.

Two consequences follow, and both are claims the tests check. A product `y = x·w` back-propagates as `g_x = g_y·conj(w)`. A unitary transform `y = U x` back-propagates as `g_x = U⁻¹ g_y`. Because Phinet's FFT is unitary, its backward is its inverse, and the FFT costs nothing extra to differentiate. The spectral layer of a Fourier neural operator becomes a forward FFT, a complex matrix per mode, and an inverse FFT, with a backward that is the same three steps run in reverse.

Phinet does not claim speed parity with PyTorch or TensorFlow. The performance guide says the remaining gap to a BLAS-backed batched complex GEMM is about 10×. It does not claim generality. There is no broadcasting, no tensor of rank other than three, no FFT of a length other than a power of two, no device other than the CPU. The what-is-missing page lists each of these, marks some as deliberate and some as gaps, and this paper keeps those labels.

What the claim does mean for the rest of the family is this. When Phiano reports that an operator ranks Austen's next words at 14.7× chance, or that the sphere tie reaches 13.1× on the test split, those numbers come out of this tape. If the tape were wrong, every one of them would be wrong in a way no ranking metric would reveal. The finite-difference tests and the torch parity checks are what stand between the measured results of papers elsewhere in this series and that failure.

## 3. The mechanism, plainly

The crate root, `src/lib.rs`, defines a function as `pub type Function = Vec<c64>`, where `c64` is `num_complex::Complex64`. Three helpers make functions. `normalise` scales a slice to unit energy in place and returns false if the energy is below 1e-24. `unit_random(space, seed)` draws a unit-energy function with random phases and magnitudes. `unit_phase(space, seed)` draws a function with magnitude one at every sample and random phases, which the source calls "the shape a role has". All randomness passes through `mix64`, the finaliser of splitmix64.

A `Tensor` has a shape `[batch, width, space]` and a flat `data: Vec<c64>`. `space` is the circle. `width` is how many functions are stacked as features. `batch` is how many are in flight. Every op asserts its shape.

The `Tape` lives in `src/tensor/tape.rs`. It is `nodes: Vec<Node>`, and a `Node` has four fields: `value: Tensor`, `grad: Option<Tensor>`, `op: Op` and `requires_grad: bool`. A `Var` is `Var(pub usize)`, an index into that vector. `tape.param(t)` pushes a leaf with `requires_grad` true. `tape.constant(t)` pushes a leaf with it false. Every forward op computes its value, pushes a node carrying an `Op` variant that names its inputs, and returns the new `Var`.

The `Op` enum is the whole vocabulary: `Leaf`, `Add`, `AddBias`, `Mul`, `Scale`, `Linear`, `ModeMix`, `Fft`, `Ifft`, `Fft2`, `Ifft2`, `Gelu`, `Cosine`, `Hinge`, `MeanReal`, `Gather`, `Unit`, `CosineTable`, `MixtureNll`, `Mobius`, `CrossEntropy` and `Tie`. `ModeMix` is `y[b,o,m] = Σ_i x[b,i,m]·r[i,o,m]`, one complex matrix per mode. `Tie` is `y[i,o,k] = r[i,o,group[k]]`, one weight shared by a group of modes; the source comment says a spherical operator ties one weight per degree.

`backward(out)` zeroes every gradient, seeds `out` with 1, and walks the nodes in reverse, adding into each input that requires grad. `truncate(keep)` drops every node after the first `keep` and clears all gradients, so one tape serves many steps without re-creating the parameters. `Fno::reset` uses it.

The FFT lives in `src/tensor/fft.rs` as `fft_inplace(x: &mut [c64], inverse: bool)`. It asserts a power-of-two length, does a bit-reversal permutation, runs butterflies with twiddle `e^{−2πi/len}` (conjugated for the inverse), and multiplies every sample by `1/√n`. `fft2_inplace` reads a row as an `s1 × s2` torus and applies the 1-D transform to rows, then to columns.

`Adam` treats each complex parameter as two reals, with `eps = 1e-8`, `β₁ = 0.9`, `β₂ = 0.999` and bias correction. `with_decay` gives AdamW. `Schedule` offers constant, warm-up plus cosine, and step.

The finite-difference tests live in `src/tensor/tests_grad.rs`. The FFT tests live in `src/tensor/tests_fft.rs`. Section 9 walks through the first of these line by line.

## 4. What was measured, and the numbers

Four kinds of measurement bear on the floor. Each is quoted as it stands in its source.

Parity with PyTorch. `docs/interop/01-pytorch-parity.md` reports, on torch 2.14 at complex128, 62 checks in fifteen groups, all passing. The largest differences by group:

| group | checks | max \|Δ\| |
|---|---|---|
| FFT / iFFT, unitary both ways | 4 | 3.3e-16 |
| linear, add_bias, gelu, mode_mix, unit, proj | 6 | 3.5e-16 |
| gradients of the tape (∂/∂lift, ∂/∂R, ∂/∂proj, ∂/∂bias, ∂/∂x) | 5 | 0.0 |
| mixture_nll and its gradient | 2 | 2.2e-16 |
| one training step: loss, nine parameter gradients | 10 | 9.9e-16 |
| one Adam step on all nine parameters, then `mask_modes` | 9 | 1.1e-16 |
| `fft2` on a 4×4 torus and its gradient | 2 | 1.1e-16 |

The remaining eight groups (cosine and hinge, Möbius, bind and unbind, hexagram, zeta weights, `Fno::apply` on a 321-parameter operator, cross-entropy with AdamW, resample and quantise) all sit at or below 2.2e-16. PHIENT_FACTS records the result as 62/62.

Finite differences. The changelog's latest entry reports 29 tests passing, among them the gradient check, the mixture gradient check, the Möbius check on both arguments, the 2-D torus check, Parseval, and a pure tone landing in one bin.

Wall-clock. `docs/measurements/02-benchmarks.md`, run with `cargo run --release --example bench` on two cores at `opt-level = 3`, one run, ±10% treated as noise:

| path | size | before | after |
|---|---|---|---|
| `fft_inplace` forward + inverse | 64 points × 100,000 pairs | 1.85 µs per pair | 1.2 µs |
| `Fno::apply` | 4,096 functions; space 64, width 8, 2 layers, 12 modes (8,360 params) | 916 µs per function | 88 µs |
| `Fno::train_epoch` (hinge) | 10,000 examples, pool 2,000, batch 64 | 2.98 s | 1.76 s |
| `Fno::train_epoch_mixture` | 2,000 examples, table 2,000, batch 64 | 2.41 s | 1.45 s |
| `Tape::cosine_table` | 256 × 6,000 × 64 | 79 ms | 69 ms |

Where a batch goes. For the wide operator (width 16, 3 layers, 16 modes; 50,016 params) against a 1,228-row table, one softmax batch splits into stacking 0.3 ms, forward 30 ms, `cosine_table` 4–7 ms, `cross_entropy` 0.7 ms, backward 54–75 ms, and `Adam::step` plus `mask_modes` 0.8 ms. `Tensor::zeros` of 1 MB takes 0.7 ms and a clone 0.66 ms, at about 1.5 GB/s.

A training check. The Colab notebook trains the rotate-by-one task from MRR 0.073 untrained to 1.000 in 60 epochs. The Rust example in `lib.rs` asserts only that MRR exceeds 0.9.

## 5. The one idea, restated

Phinet computes relations on a closed circle, and it computes their gradients honestly. Everything else in this paper is a consequence of taking that sentence at face value.

A relation in Phient is a complex phasor. A word is 32–64 of them. When Phiano hands a word to Phinet, it hands over a function: complex samples around a circle. The circle is closed, so the natural basis for it is the Fourier basis, and the natural operation on it is the FFT. A Fourier neural operator acts on the circle by lifting a function into several channels, transforming to frequency, mixing each frequency with its own complex matrix, transforming back, and adding a pointwise path. Every one of those steps is an op on the tape.

The unitary normalisation is what makes this cheap to learn. If the forward FFT divides by `√N` and the inverse does the same, then the two are each other's adjoint. Under Phinet's gradient convention, the adjoint of a linear map is exactly its backward. So the backward of `fft` is `ifft`, and the backward of `ifft` is `fft`. The circle keeps its energy under both, which is Parseval's identity, and the tests check it.

The gradient convention is what makes this cheap to write. Storing `∂L/∂Re + i·∂L/∂Im` turns every complex backward into a conjugate multiply. `mul(a, b)` gives `g_a = g·conj(b)`. `linear` gives `g_x = Σ_o g_o·conj(w_io)`. `mode_mix` is the same rule, per mode. A real non-linearity applied to the real and imaginary parts separately, which is how `gelu` works here, back-propagates through each part on its own.

The finite-difference check is what makes it honest. An analytic backward rule can be wrong in a way that still trains. A sign error in one term, a missing conjugate, a factor of two: each would bend the loss surface slightly, and a ranking objective would still climb. The operator would report some multiple of chance, and the number would be wrong. The only defence is to perturb each parameter by a small real or imaginary step, recompute the loss from scratch, and compare the slope with what the tape said. Phinet does this for every op it has.

Restated in the paradigm's own terms: the word is its relations, the relations are samples on a circle, the circle's own transform is its own adjoint, and the slope of every relation's loss has been measured against a perturbation of it. That is the parent. The children inherit the circle, the transform and the checked slopes, and add their own geometry on top.

## 6. Complex tensors in depth

A `Tensor` in Phinet carries a shape of three `usize` and a flat vector of `c64`. There is no stride, no view and no in-place op. Every op allocates its output. The what-is-missing page marks rank three and the absence of broadcasting as deliberate: every op asserts its shape, and a shape mismatch fails loudly.

The three axes have fixed meanings. `space` is the circle, sampled at a power of two: 32, 64 or 128 in practice. `width` holds stacked features. `batch` holds examples in flight. The op table in the tensor-engine guide names each op by what it does to those three axes. `add_bias` adds `[1,w,1]` to `[b,w,s]`. `linear` maps `[b,i,s] × [i,o,1]` to `[b,o,s]`, pointwise across width. `mode_mix` maps `[b,i,m] × [i,o,m]` to `[b,o,m]`, one complex matrix per Fourier mode. `gather` pulls rows of a `[V,1,s]` table by index into `[b,1,s]`. `cosine` reduces `[b,1,s]` against `[b,1,s]` to `[b,1,1]`, and `cosine_table` reduces `[b,1,s]` against every row of `[V,1,s]` to `[b,V,1]`.

Precision is `f64` throughout. The numerical-conventions guide says there is no `f32` path and no mixed precision. The later what-is-missing page adds that `to_bytes_compact` stores parameters as `f32`, halving the file with drift under 1e-10, but compute stays in complex128.

The epsilons are fixed and written down. `cosine` and `cosine_table` clamp their denominators at `max(‖p‖‖t‖, 1e-12)`. `unit` and `normalise` leave rows with energy under 1e-24 at zero. `mixture_nll` takes `log(max(p, 1e-12))`.

The worked example in `lib.rs` shows the tensor shape serving a task. Sixty items on a 64-point circle; the target of each is the item itself rotated by one sample.

```rust
let inputs: Vec<_> = (0..60).map(|i| unit_random(64, i)).collect();
let targets: Vec<_> = inputs.iter().map(|f| { let mut g = f.clone(); g.rotate_right(1); g }).collect();
let cfg = FnoConfig { space: 64, width: 4, layers: 1, modes: 32, margin: 0.5, lr: 1e-2, tie: Tie::Circle };
let mut fno = Fno::new(cfg, 1);
for epoch in 0..60 {
    fno.train_epoch(&targets, &examples, &pool, 20, &mut adam, epoch);
}
```

The comment in the source says the identity cannot express this relation and a spectral filter can. A shift by one sample is, in frequency, a phase ramp: each mode multiplied by its own unit phasor. `mode_mix` with one complex weight per mode can represent that exactly. The example asserts MRR above 0.9; the PyTorch twin reaches 1.000 in 60 epochs from 0.073.

Seeds enter as explicit `u64` values. There is no global random number generator. `Tensor::randn(shape, scale, seed)`, `unit_random`, `unit_phase`, `hrr::role` and `Fno::new(cfg, seed)` all take one. Decoys during training are drawn from `mix64` of the seed and the example index. The conventions guide states that two runs with the same seeds produce identical bytes from `to_bytes()` on the same machine.

## 7. The tape in depth

The tape is a flat record. Forward ops push nodes in the order they run. There is no graph object, no pruning, no retain or release; the tape is replayed as recorded. Nothing is reused across steps, and the tape is rebuilt for every batch. In return the backward pass is a single reverse loop over a vector, easy to read and easy to check.

`requires_grad` propagates forward. A `param` sets it true. Any node computed from a node that requires grad also requires it. `constant` sets it false, and gradients stop there. `any_grad(&[Var])` asks whether any input needs a gradient. This matters for speed: the `cosine_table` backward now skips the gradient pass for a side that needs none. While the operator trains, the table is a constant. Before that change, a full 204k-example softmax epoch took over 14 minutes.

The backward rules, with `g` the incoming gradient, are these for the core ops:

| op | backward |
|---|---|
| `add(a, b)` | `g_a = g`, `g_b = g` |
| `mul(a, b)` | `g_a = g·conj(b)`, `g_b = g·conj(a)` |
| `linear(x, w)` | `g_x = Σ_o g_o·conj(w_io)`; `g_w[i,o] = Σ_{b,s} g[b,o,s]·conj(x[b,i,s])` |
| `fft(x)` | `g_x = ifft(g)` |
| `gelu(x)` | `g_x = gelu'(Re x)·Re g + i·gelu'(Im x)·Im g` |
| `unit(x)` | projection of `g` orthogonal to `x`, divided by `‖x‖` |
| `gather(T, idx)` | scatter-add `g` rows into `T` |
| `hinge(pos, neg, m)` | `−1/b` into `pos`, `+1/b` into `neg` where the margin is violated |

The `gather` rule carries weight beyond its size. Because it scatter-adds, the table it reads from can itself be a `param`. That is how `Joint` trains the operator and the table of word functions together: the table is a parameter on the operator's tape, held near its starting point by a proximal tether. Without a differentiable gather there would be no joint training.

Normalising a row with `unit` removes the radial direction, so the gradient loses its radial component too: Phinet projects `g` orthogonal to `x` and divides by `‖x‖`.

`mobius(z, abcd)` has the richest rule. For `w = (a z + b)/(c z + d)`, the guide gives `∂w/∂z = (ad − bc)/(cz + d)²`, `∂w/∂a = z/(cz+d)`, `∂w/∂b = 1/(cz+d)`, `∂w/∂c = −z w/(cz+d)` and `∂w/∂d = −w/(cz+d)`, each conjugated into the convention. A separate test confirms that identity coefficients leave the input unchanged to 1e-12, and that coefficients `(e^{0.7i}, 0, 0, 1)` multiply every sample by `e^{0.7i}`, which is binding by rotation.

The guide lists what the tape does not do: higher-order gradients, in-place ops, checkpointing, graph reuse across steps, gradient clipping.

## 8. The unitary FFT

The whole transform is under fifty lines of `src/tensor/fft.rs`. It asserts `n.is_power_of_two()`. It permutes the input into bit-reversed order with a running index `j`. It then runs the butterflies, doubling `len` from 2 to `n`:

```rust
let sign = if inverse { 1.0 } else { -1.0 };
let ang = sign * 2.0 * PI / len as f64;
let wlen = c64::new(ang.cos(), ang.sin());
// for each block of length len, for k in 0..len/2:
let u = x[i + k];
let v = x[i + k + len / 2] * w;
x[i + k] = u + v;
x[i + k + len / 2] = u - v;
w *= wlen;
```

At the end every sample is multiplied by `1.0 / (n as f64).sqrt()`. Both directions carry that factor. The source comment states the consequence: `ifft(fft(x)) == x` and each is the adjoint of the other.

Phinet's FFT equals `torch.fft.fft(x, norm="ortho")`. The numerical-conventions guide says NumPy's default and torch's default, `"backward"`, differ from it by a factor of `√N` on the forward and `1/√N` on the inverse. The parity page says the default would put a factor of `N` between the two engines. The twin therefore uses `norm="ortho"` throughout.

Two tests check the transform directly. `test_fft_is_unitary_and_invertible` draws a 64-sample tensor from seed 7, transforms it, and checks that the energy of the output equals the energy of the input to within 1e-9. That is Parseval's identity. It then inverts and checks each sample returns to within 1e-9. `test_fft_of_a_pure_tone_is_one_bin` builds `e^{2πi·k·t/n}` with `n = 64` and `k = 5`, transforms it, and checks that bin 5 has magnitude `√n` and every other bin has magnitude below 1e-9. A pure tone lands in one bin, with the unitary scale.

The torus test extends both checks to two dimensions. `test_fft2_is_unitary_and_its_backward_is_the_inverse` uses `s1 = 4`, `s2 = 8`. It checks the round trip to 1e-12, Parseval on the torus to 1e-9, and that a tone with frequencies 1 and 3 on the two axes lands in bin `1·s2 + 3` with magnitude `√(s1·s2)`. It then takes `mean_real` of the transform, runs `backward`, and compares the gradient at every seventh sample with a finite difference, to 1e-5.

The FFT is not where the time goes. A forward and inverse pair at 64 points costs 1.2 µs, and the benchmark page says the FFT and the mode mix are a small share of a softmax batch.

The what-is-missing page states what the transform does not do: no arbitrary lengths, no real-input FFT, nothing beyond two dimensions. Arbitrary lengths and real FFTs are marked not planned. The phasor ranges P16 to P128 are all powers of two.

## 9. Gradients checked by finite differences

`test_gradients_match_finite_differences` in `src/tensor/tests_grad.rs` is the test that the rest of the family leans on. It builds a small operator on a circle of `s = 8` samples with `width = 3`, and runs every core op in one chain:

```rust
let table = tape.param(table_t);                 // [5,1,8], seed 1
let raw = tape.gather(table, &[4, 0, 2]);
let x = tape.unit(raw);
let target = tape.gather(table, &[1, 3, 0]);
let decoy = tape.constant(Tensor::randn([3, 1, s], 1.0, 3));
let h = tape.linear(x, lift);
let h = tape.gelu(h);
let hf = tape.fft(h);
let hm = tape.mode_mix(hf, r);
let hs = tape.ifft(hm);
let hl = tape.linear(h, w);
let hl = tape.add_bias(hl, bias);
let h2 = tape.add(hs, hl);
let h2 = tape.gelu(h2);
let h2 = tape.scale(h2, 0.7);
let y = tape.linear(h2, proj);
let sp = tape.cosine(y, target);
let sn = tape.cosine(y, decoy);
let loss = tape.hinge(sp, sn, 1.5);
```

This is one Fourier layer with its ranking loss, fed from a table that is itself a parameter. Six parameter tensors are checked: `lift`, `r`, `w`, `bias`, `proj` and `table`.

The test first asserts that the hinge loss is positive. The comment says the hinge must be active for the check to mean anything. A hinge below its margin has zero gradient everywhere, and a zero gradient would match a zero finite difference trivially.

It then runs `backward` once and keeps each analytic gradient. For every third element of every parameter, and for both the real and the imaginary part, it rebuilds the network from the same seeds with that one part raised by `eps = 1e-6`, computes the loss on a fresh tape, and forms `fd = (lp − base)/eps`. The assertion is `|fd − an| < 1e-4·(1 + |an|)`. The table is rebuilt from seed 1, bumped, and passed in through a thread-local override. A final assertion requires more than 40 partials to have been checked.

The Möbius test does the same for `mobius` on both its arguments, over every sample and every coefficient, with the same `eps` and tolerance. The mixture test does it for `cosine_table` and `mixture_nll`. The torus test does it for `fft2`.

A reader should know three limits of this check. The difference in the code is one-sided, `(L(θ+ε) − L(θ))/ε`, so its truncation error is of order `ε`. The autograd guide describes a central difference, `(L(θ+ε) − L(θ−ε))/2ε`. The code is what runs; the guide is out of date. Second, only every third element of each parameter is sampled. Third, the tolerance of 1e-4 relative is loose enough to pass a small error. The torch parity checks close the second and third gaps: they compare every gradient element with `torch.autograd`, and the tape-gradient group agrees to 0.0.

## 10. Parity, seeds and reproducibility

The finite-difference test asks whether each backward rule is the derivative of its forward rule. The torch parity check asks whether the forward rules compute what a reader expects. `examples/torch_reference.rs` runs Phinet on fixed inputs and writes every intermediate, every gradient and one Adam step to JSON, and `torch/phinet_torch.py` replays the same computation in PyTorch and compares. The commands are:

```
cargo run --release --example torch_reference > torch/reference.json
python torch/phinet_torch.py torch/reference.json
```

Four conventions had to match for the two engines to agree. The FFT is unitary in both directions, `norm="ortho"`. GELU is the tanh approximation, `0.5·x·(1 + tanh(√(2/π)(x + 0.044715 x³)))`, applied to the real and imaginary parts separately; `torch.nn.GELU` on a complex tensor is undefined. The complex gradient is torch's conjugate-Wirtinger derivative, so `Mul` backward is `g·conj(b)` on both sides. Adam treats a complex parameter as two reals, which `torch.optim.Adam` does through the real view.

With those matched, the parity page reports 62 checks passing. The one disagreement found while writing the twin was on the Python side: `lines()` had been normalised by total energy instead of octave energy. The Rust definition, in which mode 0 has no line, stands. The groups that matter most for this paper are the tape gradients, the training step with nine parameter gradients, and the Adam step. Their largest differences are 0.0, 9.9e-16 and 1.1e-16.

Reproducibility inside one machine is by construction. There is no global RNG. Every constructor and every `train_epoch*` takes a `u64` seed. `rayon` appears only in `cosine_table`, where rows are independent and each row's reduction is sequential, so the parallel schedule cannot change the sum. The conventions guide states that two runs with the same seeds produce identical bytes from `to_bytes()` on the same machine. The changelog's last entry, which split the crate into files under 200 lines, reports the reference dump byte-identical before and after.

Reproducibility across machines is stated more carefully. The code uses IEEE `f64` and no fused multiply-add intrinsics, so results are expected to match between x86-64 and aarch64. The guide says this has been verified only on one Linux x86-64 machine, and against torch's CPU kernels, which are not bit-identical and agree to 1e-15.

A saved operator is also a measured artefact. `Fno::to_bytes` for the 8,360-parameter operator was 134,104 bytes in the first draft and is 134,113 bytes with the version-2 header. Format 2 carries a `PHINET` magic and a version. `to_bytes_with` adds the optimiser state. Legacy version-1 files still load, and `from_bytes` reads formats 1, 2 and 3.

The Colab notebook, `torch/phinet_colab.ipynb`, trains the rotate-by-one task in torch from MRR 0.073 to 1.000 in 60 epochs.

## 11. What it depends on, and what depends on it

Phinet depends on very little. The crate root imports `num_complex` for `Complex64`. `rayon` runs the one parallel path. `serde_json` is a dev-dependency for the torch reference dump. The PyTorch twin is a port kept for checking. There is no BLAS, no GPU runtime and no Python at run time.

Everything else in the PML family depends on Phinet. PHIENT_FACTS lists the parent's contents as `Tensor` `[b,w,s]` complex, `Tape` autograd, the unitary FFT, `Fno`, joint training, HRR, 64 hexagram gates and the sphere. This paper covers the first three. The rest are built from them.

`Fno` is the operator. Its spectral layer is `fft`, `mode_mix`, `ifft`. Its pointwise path is `linear` and `add_bias`. Its activation is `gelu`. Its losses are `hinge`, `cross_entropy` and `mixture_nll` over `cosine` and `cosine_table`. `Joint` needs `gather` to scatter-add. The hexagram gates are masks on Fourier modes. `hrr::bind` is a pointwise complex product, and the Möbius test confirms that rotation is bind.

The sphere shows the floor extending without new machinery. `phinet::Tie::Sphere` gives one complex weight per spherical-harmonic degree over 64 = 8² channels read as `Y_l^m` with `l < 8`. In the tape it is the `Tie` op: `y[i,o,k] = r[i,o,group[k]]`. PHIENT_FACTS reports the sphere tie at 1,192 params, Austen hinge 3 epochs 12.8× valid / 13.1× test, hit@10 0.182, 89 s, against the circle's 14.7×/14.6× at 8,360 params in 142 s. `Tie::Anisotropic`, one weight per `(l,|m|)` over 36 slots, 4,776 params, reached 16.0× valid / 15.6× test, hit@10 0.254, 85 s. `phinet::sphere` agrees with scipy's `sph_harm_y` to a maximum difference of 2.4e-15.

The children named in PHIENT_FACTS are these. phifno 1.0.0 is the spherical operator. phical is the geometry of the relation circle. phiton carries light as units: `LightQuantum`, `SpectralBand`, `PhitonColor`. phinum is listed as a child. phiqlm is planned, and nothing of it is built.

phiano is the word application: `Facet`, `Trainer`, generation, personas, quest ingest, helix. It hands Phinet a function on the circle and receives ranked words back. Every Austen number in PHIENT_FACTS passes through this tape: identity 7.9× chance, hinge 14.7× valid / 14.6× test, softmax β=8 18.0×/18.8×, and the wide-and-deep operator at 50,016 params 30.7×/30.3× at 8 epochs, hit@10 0.348, against a chance MRR of 0.0063. The benchmark page says the operator's share of a Phiano run is seconds; `bin/austen` takes about 400 s and `bin/liskov` about 11 min, almost all of it Phiano's own `relational_fit`.

phiecs is the entity-component world and the quest. phiace hosts the crates. Neither adds engine code; both inherit it.

## 12. The next experiment, with the control named

The benchmark page names its own next engine item. Every op on a 1 MB tensor costs 2–5 ms forward and 1–5 ms backward, `add` included. `Tensor::zeros` of the same 1 MB takes 0.7 ms and a clone 0.66 ms. The page concludes that each op pays for a fresh allocation and its first touch, and that a buffer pool or an `f32` path is the next engine item, before a faster FFT. This has not been built. The experiment below tests the allocation explanation directly.

The treatment is a buffer pool for the tape. When `truncate(keep)` drops the nodes after the parameters, their `value` and `grad` tensors return to a pool keyed by shape instead of being freed. The next batch's forward ops draw their outputs from the pool. Nothing else changes: same ops, same loops, same `f64`.

The control is the current engine at the current commit, built with the same `opt-level = 3`, run on the same two-core Linux x86-64 machine, with the same seeds. Two workloads are timed on both. The first is `examples/profile_ops.rs` on a `[64,16,64]` tensor, per op, forward and backward. The second is `examples/profile_softmax.rs`: one softmax batch of the wide operator (width 16, 3 layers, 16 modes, 50,016 params) against the 1,228-row table, split into stack, forward, `cosine_table`, `cross_entropy`, backward and `Adam::step`. The benchmark page's current figures for the second are forward 30 ms and backward 54–75 ms. Each workload should be run more than once, since the page treats a single run as ±10% noise.

If `add` on the pooled tape falls toward the cost of one pass over 1 MB, the allocation explanation holds, and the pool ships. If `add` stays near 2–5 ms, the cost lies elsewhere, perhaps in first touch of pages the pool does not avoid, and the explanation is wrong. If the pool speeds some ops and not others, the per-op table says which.

Correctness is the hard constraint, since reused memory is where stale values hide. The pooled engine must produce the same bytes from `to_bytes()` as the control after an identical training run, since the conventions guide promises identical bytes for identical seeds on one machine. It must pass the 29 tests, including every finite-difference check. It must pass all 62 parity checks with the reference dump byte-identical. Any difference in any of these ends the experiment regardless of speed.

A smaller experiment belongs beside it: replace the one-sided difference with a central one and tighten the tolerance below 1e-4 relative, with the present test on the same seeds as control. Neither experiment has been run, and neither result is known.

## 13. What is wrong or missing

The finite-difference test and its documentation disagree. `docs/guide/04-autograd.md` says each partial is compared with `(L(θ+ε) − L(θ−ε))/2ε`. The code in `tests_grad.rs` computes `(lp − base)/eps`, a one-sided difference. The code runs; the guide describes something that does not. The one-sided form has error of order `ε`, which is part of why the tolerance is `1e-4·(1 + |an|)`. A central difference would allow a tighter bound. This is a documentation bug, and the check itself is weaker than the guide implies.

The test samples every third element of each parameter. A backward bug confined to the other two thirds would pass it. The torch parity checks cover every element, so the gap is closed in practice, but the in-crate test alone does not close it.

The docs disagree on counts. The what-is-missing table says "15 tests" and "55 torch parity checks", its closing section says "Sixty parity checks pass", and the Colab section of the parity page says the notebook "runs the 55 checks". The parity table itself lists 62, and the latest changelog entry says 29 tests and 62 parity checks. The later documents are preferred here: 29 tests and 62 checks. The what-is-missing table also says complex autograd is "checked to 1e-16", while the autograd guide says ≤ 1e-15 and the parity table's largest difference is 9.9e-16. The guide's bound is the accurate one.

The autograd guide's table of backward rules omits ops the tape has. `tape.rs` defines `Fft2`, `Ifft2`, `CrossEntropy` and `Tie`. None appears in the guide's table. Each has a test elsewhere, but a reader of the guide would not know they exist.

Determinism across machines has been verified on one Linux x86-64 machine only. The aarch64 claim is an expectation.

The engine is slow by the standard of the frameworks it is measured against. The performance guide puts `apply` about 10× off a BLAS matmul, with no SIMD, no blocking and one output row at a time. There is no GPU. There is no `f32` compute.

The tape has no second derivatives, no hooks, no in-place ops, no checkpointing and no gradient clipping. The tape is rebuilt every batch.

GELU is applied to real and imaginary parts separately. It is not holomorphic. The conventions guide lists modReLU and cardioid alternatives on the roadmap. Neither is built, and whether either would change any ranking result is not measured.

The benchmarks are one run on one two-core machine. The page itself says to treat ±10% as noise. No confidence intervals exist for any timing in this paper.

## 14. How it was found

The history is in `docs/about/03-changelog.md`, newest first. Read oldest first, it is the record of how the floor was built and then checked.

Commit `b78269d`, "the engine", brought in `Tensor`, `Tape` with add, add_bias, mul, scale, linear, mode_mix, fft, ifft, gelu, unit, cosine, hinge, mean_real and gather, `Adam`, `Fno` with `train_epoch`, `to_bytes`, `rank`, `Joint` with the tether, and `hrr`. The finite-difference gradient tests, the Parseval and pure-tone FFT tests, and the rotate-by-one operator test arrived in the same commit. The engine and its checks were written together.

`a344607` added decoy sampling through `FrequencySampler`. `90385d5` added `cosine_table`, `mixture_nll` and the mixture objective, with a finite-difference test for both new ops. `4ca1014` put `cosine_table` on rayon, added `Tape::mobius` with a finite-difference check on both arguments, and added `zeta_weights` as a spectral prior. `08fd280` added the sixty-four hexagram gates.

`bea5720` is where the second check began. `examples/torch_reference.rs` dumped every op, gradient and one Adam step, and `torch/phinet_torch.py` replayed them: 55 checks, all passing, largest difference 9.9e-16. This round found the `lines()` normalisation mismatch on the Python side.

The next entry, called the impediments round, answered the what-is-missing page. It added `cross_entropy` with its test and parity check, AdamW and saved optimiser state, the versioned `to_bytes`, slice-based `linear` and `mode_mix`, and chunked `apply`. The speed-up came from measurement. `examples/profile_apply.rs` showed a plain `add` taking 136 ms on one 4,096-function batch, because that batch kept a dozen 32 MB tensors alive and paid for the memory in page faults. Chunking `apply` at 256 took it from 916 to 88 µs per function. That round closed at 26 tests and 60 parity checks.

The completion round added `fft2` and `ifft2` on the torus, `Schedule` on `Adam`, and `to_bytes_compact`: 29 tests, 62 parity checks. The last entry split the crate into files under 200 lines with behaviour unchanged and the reference dump byte-identical.

The allocation finding came from `examples/profile_softmax.rs` and `examples/profile_ops.rs`, recorded in `docs/measurements/02-benchmarks.md`. That page also records that a full 204k-example softmax epoch had been over 14 minutes before the `cosine_table` backward learned to skip the constant side.

On the Phiano side, the what-is-missing page cites Phiano docs/75 for the sampler as the ceiling and docs/76 for centring persona sectors on the lexicon mean. PHIENT_FACTS cites docs/86 for Liskov's three operators, which are Phinet's `phinet::liskov` module. The one-sided difference and the count disagreements in section 13 were found while writing this paper, by reading `tests_grad.rs` beside the guides.

## 15. What remains

The performance guide gives the engine work in order of payoff: a real batched GEMM for `linear` and `mode_mix`, reuse of tape allocations between steps, `f32`, SIMD butterflies in the FFT, and a GPU. The what-is-missing page repeats the first two as its first open item and `f32` compute as its second. None is built. The benchmark page's allocation finding moves the buffer pool ahead of the rest, and section 12 describes how to test it.

The documentation needs three corrections that follow from section 13. The autograd guide should describe the one-sided difference the code uses, or the code should move to a central difference and the guide can stay. The guide's backward table should gain `fft2`, `ifft2`, `cross_entropy` and `tie`. The what-is-missing page and the Colab section should carry the current counts, 29 tests and 62 parity checks, and the 1e-15 bound.

The finite-difference test could sample every element instead of every third, and with a central difference its tolerance could tighten. That would strengthen the claim of section 2 without leaning on the torch twin.

Cross-machine determinism needs a second machine. The conventions guide expects agreement between x86-64 and aarch64 and has checked only the first. A run of the reference dump on an aarch64 machine, compared byte for byte, would settle it.

The activation question is open. GELU on parts is not holomorphic, and modReLU and cardioid are on the roadmap. Any replacement would need its own backward rule, its own finite-difference test and its own parity group, and then a comparison on the Austen operator against the present GELU as control. Not built, not measured.

The family work that rests on this floor has its own open items, listed in PHIENT_FACTS. The sphere tie has not been run at the softmax or wide-and-deep settings. There has been no Phi-4 run; the quest uses a synthetic oracle only. phiqlm is planned and nothing of it is built. There is no weather work of any kind. No partnership has been agreed with anyone.

The engine is pre-1.0 at `0.1.0`. The changelog says the format of `to_bytes` and every signature may change between commits. A 1.0 would need a frozen format and frozen signatures. Nothing in the sources sets a date for that, and this paper does not.

What does not remain is the core question this paper set out to answer. The tensor, the tape, the unitary FFT and the gradient checks exist, each backward rule has been compared with a finite difference and with torch, and the numbers the rest of the series reports were computed on them.

## 16. Sources

Every number and name in this paper was taken from the local files below. Nothing was taken from the web. Where two sources disagree, the paper says so and prefers the later one; the order of the changelog fixes which is later.

| path | what was taken from it |
|---|---|
| `/home/claude/phiano/docs/papers/phient/BRIEF.md` | the rules and the sixteen-section arc of this series |
| `/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` | the paradigm paragraph; the family; 62/62 parity; the Austen, sphere, anisotropic and scipy figures; the allocation-bound line; the list of things not done |
| `/home/claude/phinet/src/lib.rs` | `Function`, `mix64`, `normalise`, `unit_random`, `unit_phase`; the module table; the rotate-by-one example and its `FnoConfig` |
| `/home/claude/phinet/docs/guide/01-tensor-engine.md` | the shape `[batch, width, space]`; the op table; the gradient convention; the names of the correctness tests |
| `/home/claude/phinet/docs/guide/04-autograd.md` | the tape's node structure, `backward`, `truncate`; the backward rule of each op; the two checks; the central-difference formula it states; what the tape does not do |
| `/home/claude/phinet/docs/guide/05-numerical-conventions.md` | `f64` throughout; the unitary FFT and its relation to NumPy and torch; GELU; the epsilons; Adam's constants; seeds; determinism and its one-machine verification |
| `/home/claude/phinet/docs/guide/07-performance.md` | before and after timings; `APPLY_CHUNK = 256`; the 136 ms `add`; the ~10× gap; the order of payoff |
| `/home/claude/phinet/docs/measurements/02-benchmarks.md` | the benchmark table and machine; `to_bytes` sizes; the softmax batch breakdown; the 1 MB allocation figures; the 14-minute epoch; Phiano run times |
| `/home/claude/phinet/src/tensor/tape.rs` | `Var`, `Op`, `Node`, `Tape`, `param`, `constant`, `truncate`, `any_grad` |
| `/home/claude/phinet/src/tensor/fft.rs` | `fft_inplace`, `fft2_inplace`, the `1/√n` scale, GELU and its derivative |
| `/home/claude/phinet/src/tensor/tests_grad.rs` | the gradient test's network, `eps`, tolerance, sampling and the one-sided difference; the Möbius tests |
| `/home/claude/phinet/src/tensor/tests_fft.rs` | the Parseval, pure-tone and torus tests and their tolerances |
| `/home/claude/phinet/docs/about/01-what-is-missing.md` | the comparison with PyTorch and TensorFlow; the older counts; the open items |
| `/home/claude/phinet/docs/about/03-changelog.md` | the commit history, the version `0.1.0`, and the test and parity counts by round |
| `/home/claude/phinet/docs/interop/01-pytorch-parity.md` | the 62 checks by group; the conventions that had to match; the `lines()` fix; the Colab result |

The first six phinet paths, with PHIENT_FACTS, are the sources assigned to this paper. The rest were read to confirm names and to state the history and the disagreements exactly. No source file was modified. Phiano docs/75, docs/76 and docs/86 were not opened; they are named only as the what-is-missing page and PHIENT_FACTS name them.

Timings are from a single run on one two-core Linux x86-64 machine. Parity figures are from torch 2.14 at complex128. Tolerances and step sizes are read from the test source. To reproduce, run the two commands in section 10, or `cargo run --release --example bench`.
