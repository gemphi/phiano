# The Sphere
*Phient papers, 6 of 16 · 2026-03-23 · The sphere: 64 = 8², one weight per degree, the anisotropic tie that beats the circle, and phifno 1.0.0*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

A word in Phient has sixty-four channels. Each channel is a complex phasor with a count. Phinet has always read those sixty-four numbers as samples on a circle, and its operator transforms them with a unitary FFT and learns a complex weight matrix per kept Fourier mode. This paper is about a second reading of the same sixty-four numbers.

The real spherical harmonics `Y_l^m` with degree `0 ≤ l < 8` and order `−l ≤ m ≤ l` number exactly `Σ(2l+1) = 8² = 64`. So a word's sixty-four channels are also the coefficients of a function on the sphere, band-limited at degree 7. Channel `k` is the pair `(l, m)` by `k = l² + l + m`. The degree `l` is the scale of the relation. The order `m` is its orientation.

The spherical Fourier neural operator, the engine of FourCastNet 3 in NVIDIA's `torch-harmonics` and `makani`, forecasts the atmosphere in this representation. It learns one complex weight per degree, shared by the `2l+1` orders of that degree. Such an operator commutes with every rotation of the globe. Phinet now has that tie. It is `phinet::Tie::Sphere`, a field of `FnoConfig`, backed by `Tape::tie` and by the transform module `phinet::sphere`.

Three results carry the paper. On a synthetic task where the relation is per degree, the sphere tie reaches MRR 1.000 where the circle reaches 0.662 and identity 0.646. On Austen next-word ranking with a hinge objective over three epochs, the sphere tie reaches 12.8× chance on validation and 13.1× on test with 1,192 parameters, against the circle's 14.7× and 14.6× with 8,360, in 89 s against 142 s. Under softmax β=8 the sphere tie reaches only 13.2× and 13.5×, where the circle climbs to 18.0× and 18.8×.

The third result is the finding. The anisotropic tie, `Tie::Anisotropic`, gives one weight per `(l, |m|)`. The cosine and sine orders of one `|m|` share a weight. It has 36 slots and 4,776 parameters. On Austen, hinge, three epochs, it reaches 16.0× valid and 15.6× test, hit@10 0.254, in 85 s. That beats the circle's 14.7× and 14.6× at 8,360 parameters.

`phinet::sphere` agrees with `scipy.special.sph_harm_y` to 2.4e-15 on all sixty-four harmonics over the 8 × 16 grid. The child crate that names all of this is phifno 1.0.0. The shipped operator in phiano is still the circle, at softmax, wide and deep, 30.7× valid and 30.3× test. The anisotropic tie at that width and objective is not measured.

## 2. The claim

The claim is narrow. Sixty-four is the count of real spherical harmonics below degree 8, and a word already has sixty-four channels. Therefore the operator that phinet trains on the circle has a sibling on the sphere that costs almost nothing to add. The sibling ties weights across orders. The more it ties, the fewer parameters it has and the more symmetry it imposes. The question is which symmetry a word can afford.

Doc 84 answers in one line: "Reflection symmetry (`φ → −φ`) is worth keeping on a word; full rotation symmetry is not." The evidence for the first half is the anisotropic tie. It keeps the reflection and drops the rotation, and on Austen it ranks the right next word higher than the free circle does, with 57% of the circle's parameters. The evidence for the second half is the isotropic sphere tie. It keeps full rotation symmetry. It holds 88–90% of the circle's ranking with a seventh of the parameters, and then stops. Softmax lifts the circle from 14.7× to 18.0× and lifts the sphere only to 13.2×. Doc 84 calls eight weights per channel pair "a ceiling, not a start".

The claim has a second, stricter half. Where the relation between input and target is itself per scale, the sphere is the exact operator and the circle is not. The synthetic degree filter is built so. There the sphere tie reaches MRR 1.000 and the circle 0.662, which is barely above identity at 0.646. The circle has more parameters and still cannot represent the map, because it does not group the channels by degree.

The claim does not reach further. The channel-to-harmonic assignment is by channel index. Nothing in Phical places a relation at degree 3 because it is a relation of a certain scale. Channel 10 is degree 3 because `⌊√10⌋ = 3`. So when the anisotropic tie wins on Austen, it wins with an arbitrary map from relations to harmonics. Doc 84 lists "placing a word's phasors on the sphere by relation kind" as remaining work. It is not built.

Nor is the claim that the sphere replaces the circle. The shipped operator is the circle. The circle at softmax, wide and deep, with 50,016 parameters, reaches 30.7× and 30.3× at eight epochs. No sphere arm has run at that width. The paper therefore claims four things only. Sixty-four is eight squared. One weight per degree is exact where the task is per degree. One weight per degree is a ceiling on Austen. One weight per `(l, |m|)` beats the circle on Austen at the hinge setting that was measured.

## 3. The mechanism

The operator is the Fourier neural operator of Li et al. (2020) in its one-dimensional periodic form, as `phinet::operator` states it. A stack of layers computes

```text
  h ← σ( F⁻¹( R · F(h) )  +  W h + b )
```

`F` is the unitary FFT along the sample axis. `R` is a learned complex `width × width` matrix per Fourier mode. `W` is a pointwise linear map across the width, `b` a bias, and `σ` is GELU on real and imaginary parts. A lift takes one input function to `width` features and a projection brings it back. Training is a hinge on cosine similarity against a sampled decoy, or cross-entropy over the whole table in the softmax arms.

`FnoConfig` holds `space`, `width`, `layers`, `modes`, `margin`, `lr` and, since doc 84, `tie: Tie`. The field carries `#[serde(default)]`, and `Tie::Circle` is the default. Every operator saved before the sphere still loads as a circle.

`Tie` has three variants. `Tie::Circle` is the FFT path, with `modes` low frequencies kept on each side of zero. `Tie::Sphere` does no FFT. Sample `k` is taken to be the real harmonic `Y_l^m` with `l = ⌊√k⌋`, and one complex weight per degree is shared by the `2l+1` orders. `space` must be a perfect square, and `modes` becomes the number of degrees kept. `Tie::Anisotropic` shares one weight per `(l, |m|)`.

`Tie::groups(space)` returns the slot of each sample, or `None` for the circle. For the sphere the slot is `⌊√k⌋`. For the anisotropic tie it is `l(l+1)/2 + |m|`, where `|m| = |k − (l² + l)|`. `Tie::degrees(space)` is the number of slots: 8, 36, or `space` for the circle. `Tape::tie` expands the slot weights to all sixty-four samples on the forward pass and sums the gradient back into each slot on the backward pass. It is checked by finite differences.

`phinet::sphere` is the transform side. `basis.rs` defines `DEGREES = 8`, `HARMONICS = 64`, `index_of(l, m) = l² + l + m`, `degree_of(k) = ⌊√k⌋` and `order_of(k) = k − (l² + l)`. `real_harmonic(k, θ, φ)` evaluates the orthonormal real harmonic from an associated Legendre recurrence without the Condon–Shortley phase. Negative `m` takes `sin(|m|φ)` and positive `m` takes `cos(mφ)`, both with a factor `√2`. `grid.rs` gives `gauss_legendre` and `Grid`, a Gauss–Legendre by equiangular grid. `transform.rs` gives `Sphere`, whose analysis and synthesis matrices are exact inverses on the band to 1e-9. `mod.rs` adds `degree_gate(coeffs, max_degree)`, which zeroes every degree above the limit, and `degree_energy(coeffs)`, which sums `|c|²` by degree.

phifno re-exports all of it and adds `config()`, `anisotropic()`, `circle()` and `Word`.

## 4. What was measured

All Austen numbers come from phiano's `bin/operator_sweep` over three novels, Emma, Persuasion, and Sense and Sensibility. The pool is 1,228 words seen twenty or more times. There are 204,592 training examples. Chance MRR is 0.0063. Scores are MRR as a multiple of chance, on validation and on test. The table is `data/sweep/results.md` as it stands.

| arm | params | objective | best epoch | valid ×chance | test ×chance | test hit@10 | s |
|---|---|---|---|---|---|---|---|
| hinge (shipped) | 8360 | hinge vs one decoy | 3 | 14.7× | 14.6× | 0.216 | 142 |
| softmax β=8 | 8360 | cross-entropy over the table | 3 | 18.0× | 18.8× | 0.246 | 395 |
| softmax, wide+deep, AdamW+cosine | 50016 | cross-entropy over the table | 3 | 24.5× | 24.3× | 0.311 | 984 |
| softmax, wide+deep | 50016 | cross-entropy over the table | 8 | 30.7× | 30.3× | 0.348 | 3189 |
| hinge, sphere tie | 1192 | hinge vs one decoy | 3 | 12.8× | 13.1× | 0.182 | 89 |
| softmax β=8, sphere tie | 1192 | cross-entropy over the table | 3 | 13.2× | 13.5× | 0.189 | 408 |
| hinge, anisotropic tie | 4776 | hinge vs one decoy | 3 | 16.0× | 15.6× | 0.254 | 85 |

The optimiser is Adam in every row except the AdamW row. Identity, the operator that returns its input, sits at 7.9× chance on the same task.

Read the three hinge rows together. Same objective, same three epochs, same pool. The circle has 8,360 parameters and reaches 14.7× valid and 14.6× test. The sphere has 1,192 and reaches 12.8× and 13.1×. The anisotropic tie has 4,776 and reaches 16.0× and 15.6×, with test hit@10 0.254 against the circle's 0.216. It is also the fastest of the three, 85 s against 89 s and 142 s.

Read the two softmax β=8 rows together. The circle rises from 14.7× to 18.0× valid. The sphere rises from 12.8× to 13.2×.

The synthetic degree filter is `phifno/examples/filter.rs`: 200 items, 40 epochs. Circle MRR 0.662, identity 0.646, sphere 1.000.

The transform was checked against an outside reference. `phinet/examples/sphere_parity.py` compares `phinet::sphere` with `scipy.special.sph_harm_y` on all sixty-four harmonics over the 8 × 16 grid. The largest difference is 2.4e-15.

Each arm was run once. No seed spread is reported for any row, so the gap between 16.0× and 14.7× has no measured error bar. No sphere or anisotropic arm has run wide and deep. The anisotropic tie has not run at softmax.

## 5. The one idea

Sixty-four is eight squared. Everything else in this paper follows from that.

A word in Phical is sixty-four relations. Phinet reads them as a function on a circle, because a list of phasors indexed by channel can be treated as periodic samples, and a periodic function has a Fourier series. Phinet also reads the octaves of that spectrum as a hexagram, because six bits index sixty-four. The helix of doc 84 reads the same index in base four as a codon, because three bases of four make sixty-four. The sphere is the fourth reading. The harmonics of degree below eight make sixty-four.

On the sphere, a function is sorted by scale. Degree 0 is one channel, the mean. Degree 1 is three channels, the coarsest tilt. Degree 7 is fifteen channels, the finest detail the band holds. An operator that treats every rotation alike can only scale each degree as a whole. It cannot tell one order of a degree from another, because a rotation moves energy between the orders of a degree and never between degrees. So the equivariant operator has one weight per degree. In phinet's layer that is one complex `width × width` matrix per degree, eight matrices where the circle keeps sixty-four slots.

That is a strong prior. It says the operator cares how much of a word lives at each scale and does not care where on the sphere it lives. For a task built that way, the prior is exactly right, and the synthetic filter shows it: MRR 1.000. For English next-word ranking, it is too strong: 12.8×, and a ceiling at 13.2× under softmax.

The anisotropic tie loosens the prior by one step. It keeps the reflection `φ → −φ`, which swaps the sign of each sine harmonic and leaves each cosine harmonic alone. A weight shared by the cosine and sine partners of one `|m|` commutes with that reflection. Different `|m|` get different weights. Thirty-six slots sit between the isotropic eight and the circle's sixty-four. On Austen that middle ground wins.

The one idea, restated as a rule for choosing a tie: impose the symmetry the data has and no more. Rotation of a word's channel sphere is a symmetry Austen does not have. Reflection is one it tolerates, at least under an arbitrary channel placement and at three epochs of hinge.

phifno 1.0.0 is the crate that carries this reading. It adds no new learning machinery. It names the configuration, exports the transform, and wraps a word as a function on the sphere. The engine work is in phinet. The crate is a way of saying, with a version number, that this reading exists and was measured.

## 6. Sixty-four channels as `(l, m)`

The packing is `k = l² + l + m`. Degree `l` owns the channels from `l²` to `l² + 2l`, and the middle of each block is `m = 0`. `degree_of(k)` finds the block by `⌊√k⌋`. `order_of(k)` subtracts the block's centre `l² + l`. The table works the channels doc 84 names,. The codon column is `k` in base four over `ACGT`, as the helix writes it. The harmonic column gives the longitude factor only. The anisotropic slot is `l(l+1)/2 + |m|` from `Tie::groups`.

| k | codon | l | m | harmonic | sphere slot | anisotropic slot |
|---|---|---|---|---|---|---|
| 8 | AGA | 2 | 2 | cos 2φ | 2 | 5 |
| 9 | AGC | 3 | −3 | sin 3φ | 3 | 9 |
| 10 | AGG | 3 | −2 | sin 2φ | 3 | 8 |
| 11 | AGT | 3 | −1 | sin φ | 3 | 7 |
| 12 | ATA | 3 | 0 | zonal | 3 | 6 |
| 14 | ATG | 3 | 2 | cos 2φ | 3 | 8 |

Doc 84 gives real words for some of these channels. In the persona lexicon, `codons/AGG/words.tsv` lists who carries `of`, which is channel 10. On the synthetic quest's helix, the word `w0` has five active codons: `AGA` (is-a), `AGC` (near), `AGG` (unlike), `AGT` (part-of), `ATA` (used-for). Those are channels 8, 9, 10, 11 and 12.

Read those five relations through the two ties. Under `Tie::Sphere`, channels 9, 10, 11 and 12 are all degree 3 and share one weight matrix per layer. Near, unlike, part-of and used-for would be scaled alike, and only is-a, at degree 2, would get its own matrix. Under `Tie::Anisotropic` the five channels sit in five slots: 5, 9, 8, 7 and 6. Each quest relation gets its own weight. Channel 10 shares its slot with channel 14, its cosine partner at `|m| = 2`. That is the one sharing the reflection asks for.

So the isotropic tie merges relations whose only common property is that their indices fall between 9 and 15. The merging is an artefact of the index. The quest was not run through either tie. This example shows the grouping and claims no score.

`degree_gate` and phifno's `Word::at_degree` act on the same blocks. `at_degree(3)` keeps channels 0 through 15, sixteen of sixty-four. `at_degree(0)` keeps channel 0, the mean. `Word::energy` returns eight numbers, one per block, and for a unit-norm word they sum to 1 within 1e-9, as phifno's doctest asserts.

## 7. One weight per degree

`Tie::Sphere` is the isotropic tie. `Tie::groups` maps each of the sixty-four samples to `⌊√k⌋`, so eight slots hold the weights and `Tape::tie` copies each slot's weight to the `2l+1` samples of its degree. `FnoConfig` with this tie skips the FFT. The samples are the coefficients already.

The parameter counts show the saving. phifno's doctest asserts `fno.parameter_count() == 1192` for `config()`, which is space 64, width 8, two layers, all eight degrees kept. The circle of the same width and depth, `circle()`, has 8,360. The anisotropic tie has 4,776. The differences between the three counts are arithmetic on those cited numbers. 8,360 − 1,192 is 7,168, which is 56 × 128, and the circle holds 56 more slots than the sphere. 4,776 − 1,192 is 3,584, which is 28 × 128, and the anisotropic tie holds 28 more slots than the sphere. Each slot therefore costs 128 parameters in this configuration, consistent with one complex `8 × 8` matrix per slot in each of two layers. The remaining 168 parameters in each count sit outside the tied spectral path. This decomposition is derived and was not printed by any run.

Why is one weight per degree the equivariant operator? A rotation of the sphere maps each degree's `2l+1` harmonics among themselves. It never mixes degrees. An operator that scales each degree's block by a single factor commutes with every such rotation.

There is a thing a per-degree operator cannot do, and the history of the tests records it. It cannot rotate the sphere. A rotation about the pole shifts the phase of each order by an amount proportional to `m`. Doc 84 says it plainly: "a phase per order is" what a rotation needs. The first version of the rotation test assumed the per-degree operator could perform one. That was wrong. The shipped test says so. `phinet::sphere` does provide rotation about the pole as a transform, and it is tested. The operator does not learn one.

The synthetic filter shows the tie where it belongs. `phifno/examples/filter.rs` builds 200 items whose target is a per-degree filter of the input, and trains for 40 epochs. The sphere tie ranks every target first, MRR 1.000. The circle reaches 0.662, and identity 0.646. The circle's extra freedom does not help, because its slots follow Fourier modes of the channel index, and the degree blocks are not Fourier modes of the channel index.

```
cd phifno
cargo run --release --example filter
```

On Austen the same tie reaches 12.8× valid and 13.1× test under hinge, hit@10 0.182, in 89 s. That is 88–90% of the circle's ranking with a seventh of its parameters.

## 8. The anisotropic tie

`Tie::Anisotropic` gives one weight per `(l, |m|)`. For each degree it keeps the zonal harmonic `m = 0` alone and pairs each cosine harmonic `m > 0` with its sine partner `−m`. Degree `l` then has `l + 1` slots. Summed over `l < 8` that is `1 + 2 + … + 8 = 36`. `Tie::groups` computes the slot as `l(l+1)/2 + |m|`, where `l(l+1)/2` is the number of slots used by all lower degrees. The last slot, `l = 7`, `|m| = 7`, is 35, so `Tie::degrees` returns 36.

The symmetry it keeps is the reflection `φ → −φ`. Under that reflection `cos(mφ)` is unchanged and `sin(mφ)` changes sign. An operator whose weight is the same on both partners maps a reflected word to the reflected output.

The measurement is one row of the sweep. Hinge against one decoy, Adam, three epochs, pool 1,228. Valid 16.0×, test 15.6×, test hit@10 0.254, 85 s, 4,776 parameters. Against the circle at the same setting, 14.7× and 14.6×, hit@10 0.216, 142 s, 8,360 parameters. The phifno README states the comparison as "beats the circle at 57% of its parameters", and 4,776 over 8,360 is 57%.

phifno exports it as `anisotropic()`:

```rust
pub fn anisotropic() -> FnoConfig {
    FnoConfig { space: HARMONICS, width: 8, layers: 2, modes: DEGREES,
                margin: 0.2, lr: 2e-3, tie: Tie::Anisotropic }
}
```

The configuration matches `config()` in every field except the tie. That makes the three arms, `circle()`, `config()` and `anisotropic()`, a controlled set. Width, layers, margin and learning rate are held. Only the grouping of the spectral weights moves.

Why would a tie with fewer parameters rank better than a free circle? The sources do not measure a reason. Two readings fit the numbers and neither is tested. One is regularisation. Tying weights across partners forces the operator to treat each cosine-sine pair as one thing, and a smaller model can generalise better from 204,592 examples in three epochs. The other is grouping. The circle's slots follow Fourier modes along the channel index, which mixes channels in a way unrelated to degree, while the anisotropic slots at least separate channels that the isotropic tie merges. The worked example in section 6 shows that separation on the quest's channels. Neither reading has a control built for it.

What the result does say is that full rotation symmetry costs the isotropic tie its ranking, and that dropping rotation while keeping reflection recovers more than the loss. The ordering on Austen, hinge, three epochs, is sphere 12.8×, circle 14.7×, anisotropic 16.0× valid. That ordering is the finding of doc 84.

## 9. Why the sphere saturates

The softmax arm of the sweep is where the isotropic tie shows its limit. Softmax at β=8 replaces the hinge against one sampled decoy with cross-entropy over the whole table of 1,228 words. For the circle that change is worth a great deal. Valid rises from 14.7× to 18.0×, test from 14.6× to 18.8×, hit@10 from 0.216 to 0.246. For the sphere tie it is worth little. Valid rises from 12.8× to 13.2×, test from 13.1× to 13.5×, hit@10 from 0.182 to 0.189.

The richer objective gives the model more signal per example. A model with room to use it improves. A model without room does not. The sphere tie has 1,192 parameters, and eight weight slots per layer channel pair. Doc 84 draws the conclusion in those terms: "eight weights per channel pair is a ceiling, not a start."

The cost did not fall with the parameters. The softmax sphere arm took 408 s against the softmax circle's 395 s. Under hinge the sphere was faster, 89 s against 142 s. The softmax objective scores every word in the table for each example, and that work does not depend on how the spectral weights are tied. PHIENT_FACTS records that the engine is allocation- and bandwidth-bound at these sizes, with a 1 MB operation taking 2–5 ms. The sources do not break the 408 s down further. It is not measured where the time goes.

The saturation is a statement about Austen under this channel placement. On the synthetic filter it is the only correct one. It says that English next-word ranking, as phiano poses it, needs the operator to tell apart channels that share a degree. Rotation symmetry forbids exactly that.

It also says something about the shipped operator. The circle at softmax, wide and deep, with 50,016 parameters, reaches 30.7× valid and 30.3× test at eight epochs, hit@10 0.348, in 3,189 s. With AdamW and a cosine schedule it reaches 24.5× and 24.3× at three epochs. Widening the sphere multiplies the size of each slot's matrix and leaves the slot count at eight. Whether that is enough is not measured. The anisotropic tie has 36 slots, and whether it follows the circle upward under width and softmax is the open question. It is not measured.

PHIENT_FACTS, in an earlier line, lists "sphere at softmax/wide+deep unrun" under things not done. Its later line records the sphere at softmax β=8 as run, and so do the README and the sweep table. The later records are preferred. The sphere at softmax has run and saturates. The sphere and the anisotropic tie at wide and deep have not run.

## 10. phifno 1.0.0

phifno is a small crate: `src/lib.rs` and one example. Its README calls it "the spherical Fourier neural operator, for words". It holds no training code of its own. It re-exports from phinet:

```rust
pub use phinet::sphere::{degree_energy, degree_gate, degree_of, index_of,
    order_of, real_harmonic, Grid, Sphere, DEGREES, HARMONICS};
pub use phinet::{Fno, FnoConfig, Tie};
```

It adds three configurations and one type. `config()` is the shipped sphere configuration: `space: HARMONICS`, `width: 8`, `layers: 2`, `modes: DEGREES`, `margin: 0.2`, `lr: 2e-3`, `tie: Tie::Sphere`. `anisotropic()` is the same with `Tie::Anisotropic`. `circle()` is the same with `modes: 12` and `Tie::Circle`, for comparison.

`Word` wraps a `Function` of sixty-four coefficients, and `Word::new` asserts the length with the message "a word is 64 coefficients". It has four methods. `energy()` returns `degree_energy`, the word's spectrum by scale, which is invariant under rotation. `at_degree(max_degree)` returns the word with every degree above the limit zeroed, the word at a coarser resolution. `on_grid(&sphere)` samples the word on the sphere's minimal exact grid, 8 × 16, through `Sphere::synthesis`. `agreement_at(other, max_degree)` is the cosine between two words after both are cut to the same degree, which says how much of their agreement lives at the coarse scales.

The use given in the README:

```rust
use phifno::{config, Word};
use phinet::{Fno, unit_random};
let fno = Fno::new(config(), 1);
let w = Word::new(unit_random(64, 3));
let by_scale = w.energy();
let coarse = w.at_degree(3);
```

The crate-level doctest builds the same operator and asserts `parameter_count() == 1192`, and asserts that the energy of a random unit word sums to 1 within 1e-9. The one unit test, `degree_gate_is_monotone_in_agreement_with_itself`, checks that a word agrees with itself at cosine 1 after every truncation from degree 0 to 7, and that its energy sums to 1.

The README's status list has six boxes ticked: the `phinet::sphere` module with five tests; `Tie::Sphere` with `Tape::tie`, finite-difference checked; the measurement against the circle on synthetic and Austen; the sphere at softmax; the anisotropic tie at 16.0×; scipy parity at 2.4e-15. Two boxes are open: the anisotropic tie at softmax or wide and deep, and a word's phasors placed on the sphere by relation kind.

The version number 1.0.0 marks that the interface is fixed. The three configurations and `Word` are what other code may depend on. It does not mark that the sphere is the operator phiano ships. Phiano's `bin/operator_sweep` builds its sphere arms from phinet directly. The sources read here do not show phiano importing phifno. Whether it does is not checked.

`Word::agreement_at` has no measurement behind it. No run in the sources compares real words by agreement at a degree.

## 11. Place in the PML family

PML is Phi machine learning.

phinet is the parent engine. It holds the complex `Tensor` of shape `[b, w, s]`, the `Tape` autograd, the unitary FFT, the `Fno`, joint training, HRR, the sixty-four hexagram gates, and now the sphere. Everything in this paper that learns lives in phinet: `Tie`, `Tape::tie`, `FnoConfig::tie`, and the `phinet::sphere` module with its basis, grid and transform. phinet matches its PyTorch reference on 62 of 62 parity tests. The sphere module adds the scipy parity at 2.4e-15.

phifno 1.0.0 is the child that names the spherical reading. Its `use` lines name phinet and no other crate of the family.

phical is the geometry of the relation circle, the paradigm on which a word is 32–64 phasors with counts. The sphere does not replace phical's circle. Each channel is still a phasor on a closed circle. What the sphere changes is how the sixty-four channels are indexed against each other: by degree and order, where the circle operator indexes them by Fourier mode.

phinum is a sibling child. The sources read for this paper say nothing about how phinum reads the sphere. phiqlm is planned, and nothing of it is built.

phiton is light as units: `LightQuantum`, `SpectralBand`, `PhitonColor`. It meets the sphere through the helix. Each codon of the helix carries a PhiTon wavelength, band and hexagram, and the same codon row carries the sphere's degree and order. The `codons.tsv` file of `bin/helix write` has columns `k`, codon, hexagram, degree, order, `x y z` and relation.

phiano is the word application: Facet, Trainer, generation, personas, quest ingest, and the helix. It is the consumer of the measurement. `bin/operator_sweep` in phiano produced every Austen number in section 4, and `data/sweep/results.md` holds them. The shipped operator in phiano is still the circle.

phiecs is the entity-component world and the quest. Its synthetic oracle has 256 words in families of eight and five relation kinds: is-a, part-of, near, used-for, unlike. Those five land on channels 8 through 12 in the quest's helix, and section 6 shows how the two ties would group them. No quest run has used a sphere or anisotropic tie. It is not measured.

phiace is not mentioned in any source read for this paper. Its relation to the sphere is not stated here.

What depends on the sphere is small today. phifno depends on it by construction. The helix writes the degree and order of each codon, which is a label and not a learned weight. Nothing shipped trains with a spherical tie. What the sphere depends on is phinet's operator and tape, unchanged in form and extended by one field and one tape operation.

## 12. The next experiment

Doc 84 and the phifno README name the same next run: the anisotropic tie at the width and objective of the shipped operator. The README's open box reads "Anisotropic tie at softmax / wide+deep". Doc 84 says "the anisotropic tie at that width and objective is the next run."

The control is the circle at the same width, depth, objective, optimiser and epoch count. The shipped arm in `data/sweep/results.md` is softmax, wide and deep, Adam, eight epochs, 50,016 parameters, 30.7× valid and 30.3× test, hit@10 0.348, 3,189 s. That row is the control. The pool stays at 1,228 words, the data at Emma, Persuasion, and Sense and Sensibility, and chance at MRR 0.0063. The anisotropic arm changes only `tie`. Its parameter count at that width is not given in the sources, so it is not stated here.

Two intermediate arms make the result readable. The first is the anisotropic tie at softmax β=8 at the base width 8, against the circle's 18.0× and 18.8× at 8,360 parameters. That arm answers whether the anisotropic tie saturates the way the isotropic tie did at 13.2×. If it rises with softmax as the circle did, the 36 slots are room enough at this width. If it stalls, the ceiling has moved and not gone. The second is the isotropic sphere at wide and deep, against the same circle control. The sphere's slot count stays at eight at any width, so this arm tests whether widening each slot's matrix helps when the number of slots does not change.

Every arm so far ran once. The next experiment should run each arm, control included, at more than one seed. The gap between 16.0× and 14.7× on validation, and between 15.6× and 14.6× on test, is 1.3× and 1.0× of chance. Without a spread across seeds there is no way to say whether it is larger than the noise of a single run. The sources report no seed spread for any arm.

A second line of experiment is in doc 84's remaining work: placing a word's phasors on the sphere by relation kind, not by channel index. The control there is the same tie with the current index placement. The quest is the natural task, because its five relation kinds are known and sit on five known channels. A placement that puts, say, is-a and part-of at one degree and near and unlike at another would give the degree a meaning. Which placement to try is not decided in the sources, and it is not built.

## 13. What is wrong or missing

The Austen comparison rests on single runs. There is one row per arm in `data/sweep/results.md` and no seed spread anywhere in the sources. The anisotropic win is 16.0× against 14.7× valid and 15.6× against 14.6× test. It is the same direction on both splits and on hit@10, 0.254 against 0.216. That is consistent. It is not a measured confidence.

The comparison is at the smallest setting only. Hinge, width 8, two layers, three epochs. The shipped operator has 50,016 parameters against 8,360, and eight epochs. The claim "beats the circle" holds at the setting measured. At the shipped setting it is not measured.

The channel placement is arbitrary. Channel `k` becomes `Y_l^m` by index, and the index was never chosen with the sphere in mind. Degree does not yet mean scale of relation. Doc 84 says so, and says where it would start to: placing phasors by relation kind. Every sphere number in this paper is therefore a number for an arbitrary placement.

The reason the anisotropic tie wins is not known. Section 8 gives two readings, regularisation and grouping. No control separates them. A tie with 36 slots assigned at random, keeping the parameter count and dropping the reflection, would test regularisation alone. It was not run.

The sources disagree in three places. PHIENT_FACTS gives the paradigm as "32–64 relations"; `phinet/src/sphere/mod.rs`, the phifno README and the lib doc say "64 relations". The paradigm paragraph at the head of this paper follows PHIENT_FACTS verbatim, as the brief requires. PHIENT_FACTS lists "sphere at softmax/wide+deep unrun" in one line and records the sphere at softmax β=8 as run in a later line; the later line, the README and the sweep table agree it ran. PHIENT_FACTS gives the wide+deep softmax circle as 27.6× at three epochs; `data/sweep/results.md` has no 27.6× row, and its three-epoch wide+deep row, with AdamW and cosine, reads 24.5× and 24.3×.

The rotation test was wrong in its first version. It assumed a per-degree operator could rotate the sphere. It cannot. The shipped test corrects it.

Nothing in phiano ships the sphere. phifno is 1.0.0, and the operator phiano generates with is the circle.

The byte deadband is untouched. A channel is stored as one byte, 256 quanta of 2π. The operator works in floating point. Whether a spherical tie changes anything about the deadband is not measured.

There is no weather work. The spherical FNO is a weather model's engine, and FourCastNet 3 forecasts the atmosphere with it. Phient borrowed the tie and applied it to words.

No Phi-4 run exists. The quest has run on the synthetic oracle only, and no sphere tie has run on the quest.

## 14. How it was found

The circle operator came first. The `Tie::Circle` doc comment calls it "the operator of docs/76–78". Those docs built the Fourier neural operator in phinet and swept it on Austen. The sweep's hinge arm, 14.7× and 14.6× with 8,360 parameters, became the shipped baseline, and softmax and wide and deep carried it to 30.7× and 30.3×. Doc 72 is where phiano began keeping a word's source side and target side apart, the two strands the helix later writes per codon.

Doc 84 is the sphere. It opens with the observation that sixty-four is four numbers at once: sixty-four relation channels, sixty-four real harmonics below degree 8, sixty-four hexagrams, and sixty-four codons. Phinet already read a spectrum's octaves as a hexagram. The sphere and the codon were new readings.

The source of the tie is recorded. Doc 84 states that NVIDIA's `torch-harmonics` and `makani`, FourCastNet 3's engine, were cloned and read on 2026-09-27. From that reading came the construction: one complex weight per degree, shared by the `2l+1` orders, commuting with every rotation of the globe.

The build in doc 84 proceeded in a fixed order. First `phinet::sphere`: the orthonormal real harmonics, the Gauss–Legendre by equiangular grid, analysis and synthesis exact to 1e-9, rotation about the pole, the degree gate, and degree energy. Then `Tape::tie`, which expands eight weights to sixty-four and sums the gradient back, checked by finite differences. Then `FnoConfig::tie = Tie::Sphere` with `#[serde(default)]`, so every saved operator still loads. Then phifno, to name it.

The first measurements were the synthetic filter, sphere 1.000 against circle 0.662, and the Austen hinge arm, 12.8× and 13.1×. Within the work of doc 84 the rotation test was corrected, after its first version assumed a per-degree operator could rotate.

PHIENT_FACTS records a second round "later the same day". The sphere ran at softmax β=8 and reached 13.2× and 13.5× in 408 s. That was the saturation. The anisotropic tie was added as a third `Tie` variant, with `Tie::groups` computing `l(l+1)/2 + |m|`, and ran at hinge: 16.0× and 15.6×, hit@10 0.254, 85 s. The scipy parity script ran in the same round, `phinet/examples/sphere_parity.py`, and gave 2.4e-15 over all sixty-four harmonics on the 8 × 16 grid. phifno gained `anisotropic()`, and its lib doc calls it "the tie that beats the circle on Austen".

The helix, section 2 of doc 84, used the sphere's degree and order as two columns of its codon table.

Doc 86, Liskov's three operators, came after. PHIENT_FACTS records that it scored the quest by hexagram. No sphere number appears in its line. The sweep table in `data/sweep/results.md` holds the seven arms as they stand after doc 84.

## 15. What remains

Doc 84's section 3 lists five things. Three concern the sphere, and parts of two are done. The anisotropic tie per `(l, |m|)` is listed as remaining and is also measured in section 1 of the same doc. The measurement supersedes the list. The sphere at softmax is likewise done at β=8 and width 8, and not done at width 16.

What remains for the sphere, stated from the sources:

| item | state | source |
|---|---|---|
| anisotropic tie at softmax | not built as a run | phifno README |
| anisotropic tie at wide+deep | not built as a run | phifno README, doc 84 |
| sphere tie at width 16 | not built as a run | doc 84 §3 |
| phasors placed by relation kind | not built | doc 84 §3, phifno README |
| seed spread on any sphere arm | not measured | none report it |
| sphere tie on the quest | not measured | none report it |

The placement item is the one that would give the sphere a meaning of its own. Today a relation's degree is `⌊√k⌋` of its channel index. If placement were by kind, degree 0 would hold whatever the family decides is a word's mean, low degrees would hold broad relations, and high degrees would hold fine ones. Then `Word::energy` would read as a spectrum of relation scale, and `Word::agreement_at` would say whether two words agree broadly and differ in detail. None of that is true yet. The functions exist. The meaning waits on the placement.

The first Phi-4 run remains. It is the last item in doc 84's list and appears in PHIENT_FACTS under not done. The quest oracle is synthetic. A sphere or anisotropic tie on a real oracle is two steps away: first a Phi-4 run of any kind, then a tie on it.

phiqlm remains planned. Nothing of it is built. Doc 84 lists it among the children that read the same sixty-four channels their own way.

The engine question remains open at these sizes. PHIENT_FACTS says the engine is allocation- and bandwidth-bound, 2–5 ms per 1 MB operation. The sphere at hinge is faster than the circle, 89 s and 85 s against 142 s. At softmax it is not, 408 s against 395 s. Where softmax spends its time is not measured.

The tie variants, the sweep binary and the phifno configurations exist. What remains is runs, with their controls, and one decision about placement.

## 16. Sources

These are the local files read for this paper. No other source was used. Every number above appears in one of them.

`/home/claude/phiano/docs/papers/phient/BRIEF.md` is the brief for the series. It sets the sixteen sections, the style, and the rule that every number be cited as it stands.

`/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` is the list of measured facts an interpreter may cite. It gives the paradigm paragraph, the family, the Austen pool and chance, the sphere and anisotropic numbers, the scipy parity, the engine's bandwidth bound, the byte deadband, and the list of things not done. Three of its lines disagree with the other sources, as section 13 records.

`/home/claude/phiano/docs/84_sphere_and_helix.md`, section 1, is the sphere note. It gives the construction read from `torch-harmonics` and `makani`, the date of that reading, what was built in `phinet::sphere`, `Tape::tie` and `FnoConfig::tie`, the measured table, the saturation, the anisotropic finding, the rotation-test correction, and the scipy parity. Its section 2 was read for the codon names of channels 8 to 12 and for the `codons.tsv` columns. Its section 3 gives what remains.

`/home/claude/phinet/src/sphere/mod.rs` gives the module doc, the packing `k = l² + l + m`, the reading of `l` as scale and `m` as orientation, and the functions `degree_gate` and `degree_energy`.

`/home/claude/phinet/src/sphere/basis.rs` gives `DEGREES`, `HARMONICS`, `index_of`, `degree_of`, `order_of`, the associated Legendre recurrence without the Condon–Shortley phase, and `real_harmonic` with its sine and cosine convention.

`/home/claude/phinet/src/operator/mod.rs` gives the layer formula of the Fourier neural operator, `FnoConfig` with its `tie` field and `#[serde(default)]`, the three `Tie` variants with their doc comments, `Tie::groups` with the anisotropic slot formula, `Tie::degrees`, and `FnoConfig::for_space` with its default `modes` of `space / 5`.

`/home/claude/phifno/README.md` gives the crate's one idea, its measured table, the 57% figure, the use example, the `cargo run --release --example filter` command, and the status list with six boxes ticked and two open.

`/home/claude/phifno/src/lib.rs` gives the re-exports, `config()`, `anisotropic()`, `circle()`, the `Word` type and its four methods, the doctest asserting 1,192 parameters, and the unit test on self-agreement and energy.

`/home/claude/phiano/data/sweep/results.md` is the sweep table of seven arms: parameters, objective, optimiser, best epoch, valid and test MRR as a multiple of chance, test hit@10, and seconds. Section 4 prints it as it stands, with the optimiser column folded into the text.

Files named in these sources and not read for this paper: `phifno/examples/filter.rs`, `phinet/examples/sphere_parity.py`, `phinet/src/sphere/grid.rs`, `phinet/src/sphere/transform.rs`, `phinet/src/sphere/tests.rs`, `phinet/src/operator/tests_sphere.rs`, phiano's `bin/operator_sweep` and `bin/helix`, and docs 72, 76–78, 85 and 86. Their contents enter this paper only as the read sources describe them. Where a claim here rests on one of them, it rests on that description.

## Addendum, 2026-09-28

The run this paper names as next has been made. The anisotropic tie at softmax, width 16, 3 layers, 3 epochs, with 28,512 parameters, reaches 16.5× valid and 16.6× test, hit@10 0.257, in 912 s. The circle at the same shape and epochs reached 27.6× (`data/sweep/results_3ep.md`, doc 78). The sphere ties saturate and the circle stays the shipped operator. The likely cause is that the circle's FFT runs along the channel index, whose order is meaningful in phiano (offsets +1 to +8, then marker relations by frequency), and reading the 64 channels as spherical harmonics discards it. The three-epoch 27.6× row is in `results_3ep.md`, not `results.md`, which resolves the discrepancy noted above.
