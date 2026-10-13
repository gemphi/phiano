# One weight per degree: the sphere tie and the zonal filter

*Phient interpreted papers, 13 of 16 · 2026-06-29 · on: "Spherical Fourier Neural Operators: Learning Stable Dynamics on the Sphere" (Bonev, Kurth, Hundt, Pathak, Baust, Kashinath, Anandkumar, 2023, ICML 2023 oral, PMLR 202; arXiv:2306.03838)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the thirteenth of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. It is the one paper in the series with a direct counterpart already built in gemphi. The paper replaces the discrete Fourier transform inside the Fourier Neural Operator with the spherical harmonic transform, and replaces the learned filter on each Fourier mode with a learned filter on each spherical-harmonic degree. Phinet has a tie called `phinet::Tie::Sphere` that does the second of these things: one complex weight per spherical-harmonic degree, over 64 channels read as Y_l^m with l<8. The child crate phifno 1.0.0 is named for it.

That correspondence is real and it is narrow. The paper's filter is zonal, which means it depends on the degree l and never on the order m. The reason is the convolution theorem on the sphere, which says that convolving with a kernel that has rotational symmetry about the pole multiplies each (l, m) coefficient by a number that depends only on l. Phient's sphere tie has the same shape for a different reason, which was economy of parameters. The two arrive at the same object, and the object has a property that matters for a paradigm in which meaning is movement: a per-degree weight commutes with every rotation of the sphere, so it cannot itself be a rotation. It can sharpen or damp or phase-shift each degree. It cannot turn one word's function into a rotated copy of it.

Sections 2 to 5 state what the paper says and what it measured, in its own terms. Sections 6 to 10 place the paper's objects in the Phient family: the field on the sphere as a word's function, the harmonic index as a channel coordinate, the zonal filter as the operator X ⇒ Y under c, and the year-long rollout as the thing the quest and the helix would need if they were ever to run as dynamics. Section 11 lists what Phient has measured that bears on this, which for once is more than nothing: the sphere tie has been run against the circle tie on Austen and on a synthetic degree-filter task. Section 12 names the experiment that would settle the open question, with its control. Section 13 says where the analogy breaks, and it breaks at the pole. Section 14 is about the people and the year. Section 15 is what remains.

The limits, stated up front. Phient has done no weather work of any kind. Phient has not run the sphere tie at softmax or wide+deep. Phient does no spherical harmonic transform on a grid; its 64 channels are already the coefficients. Every number about Phient below is from the facts file.

## 2. What the paper claims, in its own terms

The abstract opens by saying that "Fourier Neural Operators (FNOs) have proven to be an efficient and effective method for resolution-independent operator learning in a broad variety of application areas across scientific machine learning." It gives the reason: "their ability to accurately model long-range dependencies in spatio-temporal data by learning global convolutions in a computationally efficient manner." Then it names the fault. FNOs "rely on the discrete Fourier transform (DFT), however, DFTs cause visual and spectral artifacts as well as pronounced dissipation when learning operators in spherical coordinates since they incorrectly assume a flat geometry."

The fix is stated in one sentence: "we generalize FNOs on the sphere, introducing Spherical FNOs (SFNOs) for learning operators on spherical geometries." The test is atmospheric forecasting, and the headline is stability: the authors "demonstrate stable auto-regressive rollouts for a year of simulated time (1,460 steps), while retaining physically plausible dynamics." The closing sentence is about climate: "The SFNO has important implications for machine learning-based simulation of climate dynamics that could eventually help accelerate our response to climate change."

In the body, the claim is made concrete on two fronts. The first is the geometry. A latitude-longitude grid treated as a flat periodic box has two edges that are really single points, the poles, and the flat FFT does not know that. The paper says the "FFT-based approach leads to artifacts visible at the poles, which increase in severity for longer rollouts." The second is symmetry. The authors want the learned operator to respect the rotations of the sphere, and they state the definition: "A mapping F is equivariant with respect to two SO(3) group actions Φ_R, Ψ_R both acting onto L²(S²), iff Ψ_R(F[u]) = F(Φ_R[u]) ∀R ∈ SO(3)." They give the reason this is worth wanting: rotational equivariance is "a strong inductive bias to the learned operator" because "physical laws do not change with a changed frame of reference."

The paper is honest about the limit of that symmetry in practice. It says that "any operation acting point-wise on u is trivially equivariant in the continuous setting," and then that "this is not the case in the discrete setting, however, as non-linear functions may introduce arbitrary frequency components, which makes them only approximately equivariant." So the authors "limit ourselves to equivariance in the continuous limit." The paper also proposes a non-linear variant in the frequency domain and states plainly that "the non-linear approach is not equivariant w.r.t. SO(3), however."

So the claim, in the paper's own terms, is three things at once: the right transform for the sphere is the spherical harmonic transform, the right filter is one that commutes with rotation, and the payoff is a model that does not fall apart when run forward for a year.

## 3. The mathematics, plainly

A function on the sphere can be written as a sum over spherical harmonics. The paper writes it as u(θ, ϕ) = Σ_l Σ_{|m|≤l} û(l, m) Y_l^m(θ, ϕ), and notes that the harmonics form "an orthogonal basis of L²(S²)." The index l is the degree, which sets how fine the pattern is. The index m is the order, which runs from −l to l and sets how the pattern is arranged around the axis. For each degree there are 2l+1 orders, so the coefficients up to degree L number (L+1)² in total.

On the circle the convolution theorem says that convolving two functions multiplies their Fourier coefficients. On the sphere there is a version of the same theorem, and the paper quotes it in the Driscoll and Healy form: F[κ ⋆ u](l, m) = 2π √(4π/(2l+1)) F[u](l, m) · F[κ](l, 0). Read the right-hand side carefully. The input's coefficient at (l, m) is multiplied by the kernel's coefficient at (l, 0). The kernel enters only through its m = 0 coefficients. That is what a zonal kernel is: one that depends on the angle from the pole and not on the angle around it. The result is a filter that multiplies each degree by one number and treats every order inside a degree alike.

This is where the parameter count comes from. The paper says that "this approach requires only one filter weight per l to be learned. If L is the maximum zonal mode (0 ≤ l ≤ L), this approach requires L learned filters, as opposed to L(L+1)/2 for the naive extension." The naive extension would learn a separate weight for every (l, m) pair, which is what the flat FNO does with its (k_x, k_y) modes. The zonal choice throws away the m dependence on purpose. What it buys is equivariance: a filter that is the same for every m is unchanged by any rotation, because rotations mix orders within a degree and never mix degrees.

The layer is then K_ϑ[u] = F⁻¹[κ̃_ϑ · F[u]], with F the spherical harmonic transform, and for inputs with many channels the paper says that "κ̃_ϑ(l) is replaced by a complex-valued, learned matrix for each l." So the weight per degree is a matrix over channels, and it is still one matrix per l.

The transform itself is not free. The paper says it chose the "'direct' algorithm (Schaeffer, 2013), as it has advantages over algorithms for the implementation on GPUs," and that "in our experience, this algorithm tends to outperform the asymptotically optimal algorithms." In practice the forward transform is "1D real to complex DFT over the azimuth degrees of freedom, followed by a matrix multiplication."

## 4. Its experiments and numbers, as reported

The paper reports two experiments. The first is the shallow water equations on the rotating sphere, a standard test case for atmospheric solvers. The configuration is "4 (S)FNO blocks, a down-scaling factor of 3, and embedding dimensions of 256," on a grid at "spatial resolution of 256 × 512 and timesteps of 150 seconds." Models were "trained for 20 epochs using a single autoregressive step," followed by a second stage in which "two autoregressive steps are used during training." The paper says "training the individual models takes 240s on a single NVIDIA A6000." Relative L² loss is reported "at 1h and 10h respectively, which correspond to 1 and 10 auto-regressive steps." Table 1 of the paper, as reported:

| model | parameters | L² at 1h | L² at 10h |
|---|---|---|---|
| U-Net | 3.104 × 10⁷ | 2.961 × 10⁻³ | 1.462 × 10⁻¹ |
| FNO, linear | 4.998 × 10⁷ | 8.280 × 10⁻⁴ | 9.958 × 10⁻³ |
| FNO, non-linear | 3.920 × 10⁷ | 8.298 × 10⁻⁴ | 9.784 × 10⁻³ |
| SFNO, linear | 3.518 × 10⁷ | 7.741 × 10⁻⁴ | 7.239 × 10⁻³ |
| SFNO, non-linear | 3.920 × 10⁷ | 7.673 × 10⁻⁴ | 1.558 × 10⁻² |

The linear SFNO is best at ten steps with the fewest parameters of the four operators. The non-linear SFNO is best at one step and worst of the operators at ten. That is the paper's stability claim in miniature.

The second experiment is ERA5. The data are "40 years of ERA5 (1979-2018): 1979-2015 is used for training, 2016 and 2017 are used for validation... and 2018 is held out," sampled "at a temporal frequency of 6 hours and at the native spatial resolution... 0.25 degrees lat-long," in a "26 channel dataset" and a "73 channel dataset." Training ran on "8 NVIDIA DGX machines" for "4 hours" per stage, "40 epochs in the first stage and 5 epochs in the finetuning stage." Table 2 reports relative L² and L¹ "averaged over the finetuning steps at 6 and 12h" and the anomaly correlation coefficient "at 120h (20 autoregressive steps)," on "a sample of 730 2-week-long forecasts." For the 8-layer, 384-embedding models: FNO non-linear, 4.332 × 10⁸ parameters, L² 0.0908/0.0935, ACC 0.6284/0.6611; SFNO linear, 8.296 × 10⁸ parameters, L² 0.0930/0.0933, ACC 0.6271/0.6471; SFNO non-linear, 4.332 × 10⁸ parameters, L² 0.0942/0.0977, ACC 0.6079/0.6489.

The two-week numbers are close across models. The paper's argument is made on the year: "one year-long rollout of the SFNO is computed in 12.8 minutes on a single NVIDIA A6000 GPU, compared to one hour (wall-clock time) for a year-long simulation of IFS on 1000 dual-socket CPU nodes," which it calls "a speedup of close to 5,000x," and the rollout stays physically plausible where the FFT model shows polar artifacts that grow.

## 5. The one idea, restated

The idea is that the symmetry of the domain should be built into the operator, and that on the sphere this has one exact consequence: the learned filter is a function of degree alone.

Everything else in the paper follows from that. The transform changes because the flat FFT does not diagonalise rotations of the sphere and the spherical harmonic transform does. The parameter count drops from L(L+1)/2 to L because the orders within a degree share a weight. The rollout is stable because an operator that commutes with rotation has no preferred axis to accumulate error along, so the poles stop being special. The non-linear variant is offered, tested, and found to give up the long-horizon stability, which is the paper's own control on its own idea.

It helps to say what the idea is not. It is not a claim that the atmosphere is rotationally symmetric. The atmosphere has land and sea and a tilted axis and a rotation that picks out the poles. The paper's own SFNO block adds "a learned position embedding" after the decoder, and the encoder and decoder are "point-wise MLPs with a single hidden layer and GELU activations," which see the position embedding and can break the symmetry where the data demand it. The equivariance is placed in the global convolution, the part that carries information across the whole sphere in one step, and the symmetry-breaking is placed in the local parts. That division of labour is the design.

For Phient the idea lands on a different question. Phical's word is 64 channels, and the sphere tie reads those 64 as the (l, m) pairs with l<8, so that 64 = 8². The channels are not samples of a field on a grid. They are the coefficients. Nothing is transformed. The tie applies one complex weight per l to the eight degrees. The question the paper forces is whether the thing that moves one word toward another under a condition is zonal, in the sense that it should treat all orders within a degree alike, or whether it needs to mix the orders, which is what a rotation does and a per-degree weight cannot.

The paper answers that question for weather, by measurement, in favour of zonal in the global path and free elsewhere. Phient has to answer it for words, and section 11 says how far that has got.

## 6. The word as a function on the sphere

In the paper the input u is a field: a scalar or a stack of channels defined at every point (θ, ϕ) on the sphere. It is a function, and the operator acts on the function, so the same trained weights apply at any resolution. That is the neural operator's founding promise, restated on a curved domain.

In Phical a word is also a function, and the facts file gives its form: 32–64 relations, each a complex phasor on a closed circle with a count. Phinet's tensor is complex and shaped [b, w, s], batch by word by slot. For the circle tie the slots are Fourier modes of a function on the circle. For the sphere tie the slots are read as Y_l^m coefficients, with 64 = 8² channels covering every (l, m) with l<8.

The correspondence is therefore between the paper's û(l, m) and Phical's channel, not between the paper's u(θ, ϕ) and anything in Phient. Phient never evaluates a word at a point on a sphere. It could, since the harmonics are known functions and 64 coefficients define a smooth field up to degree 7, but nothing in gemphi does so and nothing has been measured on such a picture. The honest statement is that Phient's word lives in the paper's frequency domain from the start.

This has a consequence for what "resolution" means. The paper trains on a 256 × 512 grid and its operator would act on any grid because the weights sit on degrees, not on pixels. Phient's operator also sits on degrees, but Phient has only one resolution, l<8, and no finer grid to generalise to. The paper's resolution-independence is a property Phient's design permits and has never exercised. Phient has not measured a word at l<16, and the facts file does not claim it.

What the correspondence does give is a reading of the count. In Phical each relation carries a count, the number of times a pair was seen under a condition. In the paper each coefficient carries an amplitude, and the low degrees carry the large-scale flow while the high degrees carry the fine structure that dissipates. Whether a word's relations split by degree the same way has not been measured.

A word as a function on the sphere is a defensible picture. Two words that share their low-degree coefficients are alike in the large; two that share their high-degree ones are alike in detail. The sphere tie's per-degree weight can scale those two kinds of likeness separately. That is its one power.

## 7. The harmonic index as the relation channel

The paradigm says a word relates to other words under conditions, and the relation channel is the coordinate on which one relation is stored. In the circle tie the channel is a Fourier mode: a single integer frequency k, and the weight per channel is a complex number that rotates and scales that mode. In the sphere tie the channel is a pair (l, m), and the weight is per l only.

The question is what a channel means once it has two indices. The paper answers for fields: l is scale and m is orientation about the axis. Degree 0 is the global mean. Degree 1 is a dipole, three of them, one for each axis. Degree l has 2l+1 orientations of a pattern with l nodal lines. Rotating the sphere mixes the 2l+1 coefficients of each degree among themselves by a unitary matrix and leaves every other degree alone.

For a word this suggests a reading, and the facts file's helix already uses it: the lexicon is a DNA-coded tree in which each of 64 codons is the base-4 ACGT of a channel, a hexagram, and a sphere degree and order. So the 64 channels are indexed three ways at once, and one of those ways is (l, m). The helix stores 18,496 words in 1.9 s at 75 MB with a byte-exact round trip. That is storage rather than learning, and it fixes the coordinate: channel c has a definite (l, m), and the sphere tie's weight for c is the weight for l.

If the reading is right then the relation channels group into eight families by scale, of sizes 1, 3, 5, 7, 9, 11, 13 and 15, and the sphere tie has one weight per family. Under a condition c the operator scales each family. Two relations in the same family are treated alike. Two relations in different families are treated differently. This is an assumption about words. It says that the relations a word has come in scale classes and that a condition acts on a class, not on a member.

The circle tie makes the opposite assumption: 64 channels, 64 independent weights, a condition free to act on every relation on its own. On Austen that freedom cost 8,360 params against the sphere's 1,192 and bought 14.7× against 12.8× on valid, 14.6× against 13.1× on test. The circle is a little better and a lot bigger.

The relation channel as a harmonic index is a claim about which channels a condition should treat alike. The paper's zonal filter is the strongest form of that claim, and Phient's sphere tie inherits it weight for weight.

## 8. The zonal filter as X ⇒ Y under c

The Phical operator is the map that takes one word's function X to another's Y under a condition c. In phinet it is the Fno: a unitary FFT, learned complex weights on the retained modes, an inverse, a local path and a nonlinearity, trained on a Tape. The sphere tie changes one thing, the weights, and the paper is a study of exactly that change on a different domain.

The paper's global convolution is K_ϑ[u] = F⁻¹[κ̃_ϑ · F[u]], with κ̃_ϑ a function of l. Phient's sphere tie is the same multiplication without the F and F⁻¹, because the word is already in coefficients. For a batch of words with the tensor [b, w, s], the tie multiplies slot s by the weight for the degree that s belongs to. Under c, the weight is what is learned, so the operator X ⇒ Y under c is, for the sphere tie, a per-degree complex gain.

Here is the property that has to be stated carefully. A per-degree gain commutes with every rotation of the sphere. That is the paper's whole point and it is also the sphere tie's whole limit. If Y is a rotated copy of X, meaning Y has the same coefficients within each degree mixed by the Wigner matrix for some rotation, then no per-degree gain maps X to Y, because a per-degree gain sends X to a multiple of X within each degree and a rotation does not. The sphere tie can make a word's degree-3 content louder or quieter or shift its phase as a whole. It cannot turn the word by any angle. The circle tie, with one weight per mode, can phase-shift each mode by its own angle, and on the circle that is a rotation, so the circle tie contains every rotation of the circle. The sphere tie contains no rotation of the sphere beyond the trivial ones.

The paper does not need rotations in its global path. Its position embedding and point-wise MLPs supply the axis. Phinet's hinge and softmax heads sit on top of the tie, so the sphere tie is never the whole operator. Whether the head can supply what the tie withholds for words is not known. On Austen it supplied most of it: 12.8× against 14.7× on valid.

So the interpretation is this. The paper says the global part of X ⇒ Y under c should be a per-degree gain and the rest should be local. Phient's sphere tie makes that choice for the global part. Whether words need a rotation in the global part, and therefore whether the sphere tie is the wrong tie for the hardest conditions, is the open question, and section 12 names the experiment.

## 9. The rollout as the quest and the helix

The paper's result that matters most to its authors is not a loss at one step. It is 1,460 steps. Six hours at a time, the model is fed its own output and produces the next state, and after a year of that the fields still look like weather. The paper's two-week ACC numbers are close to the flat FNO's. The year is where the two part.

Phient has two loops that feed their own output back. One is generation in phiano: an operator picks the next word, and the next word becomes the prompt. The facts file reports that on 40 held-out prompts by 30 tokens the operator nearest-k sampler reached a Kneser-Ney perplexity of 66.9, improved from 97.2, against 13.3 for a trigram sampler, and that shared 4-grams across prompts were 8.7% for the operator against 25.7% for the sampler. That is a rollout of 30 steps. It is measured against a language model's perplexity, not against a truth trajectory, so it says the operator's rollouts are less repetitive across prompts and less probable under the trigram model. It does not say they are stable in the paper's sense, and no one has run 1,460 words.

The other loop is the quest in phiecs. There the synthetic oracle has 256 words in families of 8 with kinds is-a, part-of, near, used-for and unlike, 1,280 rows, 293 target words, chance MRR 0.021. Attraction-only training with dithered rounding reached 0.594 MRR, hit@1 34.8%, hit@5 97.4%, and within-kind 0.747. That is a one-step map. The held-out figures are the rollout: one hop 0.011, two hops is-a 0.095, used-for 0.093, and part-of, near and unlike about chance. Two hops is a two-step rollout and it works for two kinds out of five, at a level well below one step within training.

The paper's lesson for the quest is about what makes a second step possible. In the paper the second step works because the operator's error has no preferred direction to pile up in. In the quest the second step works for is-a and used-for and fails for part-of, near and unlike. The facts file does not say why, and this page does not guess.

The helix is the third place a rollout lands, and there it is not dynamics at all. The helix stores each word as codons with sphere degree and order among the codon's readings, on B-DNA geometry at 10.5 codons per turn and 0.34 nm rise. It is a static index on the same (l, m) coordinates the sphere tie learns on. What it gives the tie is a fixed meaning for each degree, so that the degrees a trained tie damps or amplifies are the degrees the lexicon was filed under.

## 10. Where the sphere lands in phifno

phifno 1.0.0 is the child crate the facts file calls the spherical operator. It exists, it is versioned, and it is the reason this paper has a counterpart at all. What it is not is a port of the paper.

The paper's SFNO has, in order: an encoder that is a point-wise MLP, several SFNO blocks each with a spherical harmonic transform, a per-degree complex matrix, an inverse transform, a point-wise MLP and instance norm, a decoder that is a point-wise MLP, a large skip from encoder to decoder, and a learned position embedding. The first and last blocks do up- and down-sampling by truncating degrees. The paper's transform is an FFT in longitude followed by a Legendre matrix multiply in latitude, and the paper says it chose the direct algorithm for GPUs.

phifno has the per-degree complex weight and nothing else from that list, because nothing else is needed when the word arrives as coefficients. There is no transform, no grid, no truncation, no position embedding. The tie is the paper's κ̃_ϑ(l) and that is the whole of the correspondence. The parts of phinet that surround it, the Tape, the unitary FFT for the circle tie, the hinge and softmax heads, the joint training and HRR, are Phient's own and predate the sphere tie.

That leaves the size of the correspondence exact and small. The paper's L is the maximum degree; Phient's L is 7, since l<8 gives 64 channels. The paper's filter is a complex matrix per l over the embedding channels; the facts file says one complex weight per degree, and the whole sphere model on Austen is 1,192 params. The paper's models are 10⁷ to 10⁹ parameters. So Phient's sphere tie is the paper's idea at a scale five to six orders of magnitude smaller, on a domain the paper never considered, without the transform that gives the paper its name.

The engine note in the facts file bears on any attempt to close that gap. Phinet is allocation and bandwidth bound at these sizes, with a 1 MB op costing 2–5 ms. The paper's Legendre multiply is the expensive part of the SHT and the paper reports a per-step runtime of about 500 ms on an A6000 for the full ERA5 model. Phient has no such step, so its per-step cost is the tie plus the head, and the Austen sphere run took 89 s for 3 epochs against the circle's 142 s. The sphere is faster because it is smaller. Nothing has been measured about how the sphere tie would scale to more degrees, and phifno has not been run at l<16 or on any grid.

## 11. What Phient has already measured that bears on it

All of this is from the facts file and nothing else.

The sphere tie is phinet::Tie::Sphere: one complex weight per spherical-harmonic degree, 64 = 8² channels read as Y_l^m with l<8. On Austen, with the hinge head and 3 epochs, it has 1,192 params and reaches 12.8× chance on valid and 13.1× on test, hit@10 0.182, in 89 s. The circle tie under the same head reaches 14.7× on valid and 14.6× on test with 8,360 params in 142 s. Both are above the identity's 7.9×. The Austen setup is Emma, Persuasion and Sense and Sensibility, a pool of 1,228 words seen 20 or more times, 204,592 training examples, and chance MRR 0.0063.

On a synthetic degree-filter task with 200 items and 40 epochs, the circle tie reached MRR 0.662 against an identity of 0.646, and the sphere tie reached 1.000.

| setting | circle | sphere |
|---|---|---|
| Austen hinge, valid | 14.7× | 12.8× |
| Austen hinge, test | 14.6× | 13.1× |
| Austen hinge, params | 8,360 | 1,192 |
| Austen hinge, time, 3 epochs | 142 s | 89 s |
| synthetic degree filter, MRR | 0.662 | 1.000 |

The synthetic task is the paper's argument in Phient's clothes. When the true map is a per-degree gain, a tie with one weight per degree learns it exactly and a tie with one weight per channel barely beats the identity. That is what a matched inductive bias does, and it is what the paper claims for the sphere in the abstract. It is also the easy direction. The task was built so the sphere would win.

Austen is the hard direction. On real words the sphere tie gives up about two times chance against the circle, with a seventh of the parameters. Whether that gap is the cost of forbidding rotations or the cost of having fewer weights of any kind is not separable from these two runs. It is the question section 12 asks.

The remaining measurements in the facts file do not use the sphere tie. The softmax β=8 head at 18.0×/18.8× and the wide+deep softmax at 30.7×/30.3× at 8 epochs with hit@10 0.348 are circle-tie results, and the facts file says the sphere at softmax and wide+deep is unrun. Persona attribution at 71% overall (chance 33%), dialogue, decoding, the quest and the helix are all measured without the sphere tie. The byte deadband constrains the sphere as it constrains the circle: a channel is one byte, 256 quanta of 2π, a nudge of 0.06·sin d falls under half a quantum once sin d < 0.2, and nearest rounding froze every pair 8 quanta short. A per-degree gain is applied to the same bytes.

Phient has done no weather work of any kind, has run no ERA5, and has no rollout longer than 30 tokens. No partnership with anyone has been agreed.

## 12. The experiment that would test the paper's idea inside phinet

The paper's idea is that the global part of the operator should be zonal and the rest should be free to break the symmetry. The open question in Phient is whether words need a rotation in the global part. One experiment separates the two explanations for the Austen gap.

Build a third tie. Call it the degree-block tie. For each degree l it learns a full complex (2l+1) × (2l+1) matrix over the orders of that degree, and nothing across degrees. This tie contains every per-degree gain, so it contains the sphere tie. It contains every rotation of the sphere, because a rotation is exactly a block-diagonal unitary with one block per degree. It does not contain the circle tie, which is diagonal over all 64 channels regardless of degree, and the circle tie does not contain it. Its parameter count is the sum of (2l+1)² for l from 0 to 7, which is 1 + 9 + 25 + 49 + 81 + 121 + 169 + 225 = 680 complex weights, so it sits between the sphere tie and the circle tie in size.

Run all three ties on Austen with the hinge head at 3 epochs, the setting where the sphere and circle numbers already exist, so that 12.8×/13.1× and 14.7×/14.6× are the two fixed points. The control is the sphere tie, since the degree-block tie's only addition to it is mixing within a degree. Report valid and test as multiples of chance, hit@10, params, and seconds, the same five columns as the facts file.

Three outcomes are possible and each means one thing. If the degree-block tie matches the sphere tie, mixing within a degree does not help words, and the circle's edge is some other freedom, most likely independent gains per channel. If it matches or beats the circle tie, within-degree mixing is what words need, the zonal restriction is wrong for this domain, and the sphere tie should be replaced. If it lands between, both freedoms matter, and the next run is a full 64 × 64 complex matrix, the ceiling, at 4,096 complex weights.

A second run is the same three ties on the synthetic degree-filter task, where the sphere tie is at 1.000. The degree-block tie contains it and should match it; if it does not at 40 epochs, the extra freedom is costing optimisation, which is a separate finding.

Then the run the facts file says is missing: the sphere tie under the softmax β=8 head and under wide+deep at 8 epochs, against the circle's 18.0×/18.8× and 30.7×/30.3×. If the sphere's gap to the circle narrows under a stronger head, the paper's division of labour holds for words, with the head doing the work the tie withholds. If it widens, the tie is the bottleneck.

## 13. Where we disagree, or where the analogy breaks

The analogy breaks at the pole, in three ways.

First, the paper has a sphere and Phient has an index. The paper's whole motivation is that a latitude-longitude grid has a geometry the FFT gets wrong, and its SHT is a transform from a grid to coefficients that respects that geometry. Phient never has a grid. Its 64 channels are declared to be coefficients, and the declaration is by fiat: channel c is (l, m) because the helix files it so. No transform is performed, so no artifact is avoided, so the paper's first and most visible result, the disappearance of polar artifacts, has no counterpart in Phient. What Phient shares with the paper is the shape of the weights and only that.

Second, the paper's equivariance is a physical principle and Phient's is an economy. The paper says physical laws do not change with a change of frame, and it puts rotation invariance in the operator for that reason. Words have no such law. There is no known reason the 2l+1 orders of a degree should share a gain. Phient's sphere tie has one weight per degree because 1,192 is smaller than 8,360, and that cost about two times chance on Austen. The paper's argument for the zonal filter is that it is right. Phient's argument is that it is cheap. Those are different arguments.

Third, a per-degree weight cannot represent a rotation, and in Phient's paradigm meaning is movement. The paradigm sentence says meaning is a side effect of pairs of words moving under a condition. On the circle, moving is turning, and the circle tie turns each mode by its own angle. On the sphere, the natural move is a rotation, and the sphere tie forbids it by construction. The paper wants that, because a rotated atmosphere should forecast to a rotated forecast. Phient may not want it. If the condition c is itself a rotation of the relation sphere, so that "under c" means "turned this way," then the sphere tie is exactly the tie that cannot express it. The circle tie can, on the circle. The degree-block tie of section 12 can, on the sphere. The sphere tie cannot, and the Austen gap may be the price.

A smaller disagreement is about what stability is for. The paper's stable year is a year of a dynamical system whose truth exists at every step. Phient's rollouts are generation, and there is no true 1,460th word. The facts file's 8.7% shared 4-grams for the operator against 25.7% for the sampler measures diversity, not fidelity. The paper's notion of stability does not translate to text without a definition Phient has not yet written.

Finally, the paper's non-linear variant, which gives up equivariance and wins at one step and loses at ten, has no counterpart in Phient. Phient's ties are linear and its nonlinearity is in the head, so that finding is one Phient can neither confirm nor deny.

## 14. The person and the moment

The paper is from June 2023 and it is an ICML oral. Its first author is Boris Bonev and the list runs Bonev, Kurth, Hundt, Pathak, Baust, Kashinath, Anandkumar. The NVIDIA research page dates it 6 June 2023. Anandkumar is the last author, the senior position, and the paper sits in the FourCastNet line she leads: Pathak and Kashinath are FourCastNet authors, and the profile records that FourCastNet's later versions moved to spherical harmonics to respect the geometry of the globe. This is the paper in which that move was made and measured.

The profile places her at this time as Bren Professor at Caltech and senior director of machine-learning research at NVIDIA, the lab she opened in Santa Clara in 2018. The paper's hardware, eight DGX machines and an A6000, is that lab's. The profile also records that she was a Guggenheim Fellow and a Schmidt Sciences AI2050 Senior Fellow in 2023 and a Fellow of the AAAI in the same year, and that in 2023 she briefed the US President's Council of Advisors on Science and Technology on AI and science. The SFNO paper is the technical work of that same year.

The moment, in the profile's telling, is the year in which her stated view took its clearest form: "We have foundation models for language, not for physics." Her argument was that transformers cannot be the substrate for physics at scale and that an operator that learns in function space treats scale as what it is. The SFNO paper is a case for the second half of that sentence. It is a model that runs for a year on one GPU in under thirteen minutes, and the reason it can is that it was built on the geometry of the domain rather than on a flat grid of tokens.

The method she describes in the profile, "All of the things that work with deep learning, let's take them, but make them a bit more principled," is visible in the paper's structure. It keeps the FNO, the MLPs, the skip connection, the position embedding and the two-stage fine-tuning. It changes the transform and the filter to match the sphere, states the symmetry as a definition, and then tests a variant that breaks the symmetry to see what it costs. The paper does not assert that equivariance is good. It measures it against its absence and reports both.

Nothing here is her statement about Phient. The profile is explicit that a partnership is Phient's aim and not her word, and that Phient has done no weather work. What the profile does say is that a partnership would be earned with an operator over a function space she has not seen, measured the way she measures. This paper is the closest her group has come to Phient's object, and section 12 is the measurement Phient owes it.

## 15. What remains

The degree-block tie of section 12 has not been built. It is the one run that would say whether the Austen gap between sphere and circle is the cost of forbidding rotation or the cost of fewer weights. Until it exists, the sphere tie's 12.8×/13.1× against the circle's 14.7×/14.6× is a fact with two explanations.

The sphere tie has not been run under the softmax β=8 head or under wide+deep. The facts file says so. The circle tie's best result, 30.7×/30.3× at 8 epochs with hit@10 0.348, has no sphere counterpart, and the paper's division of labour, zonal in the global path and free in the local, can only be judged for words once the sphere tie has been given the same head.

The synthetic degree-filter task has been run once, at 200 items and 40 epochs, and the sphere tie reached 1.000. A matched task on which the truth is a rotation within each degree has not been run. On it the sphere tie should fail and the degree-block tie should succeed, and that pair of results would make the point of section 13 a measurement rather than an argument.

Phient's word has never been drawn as a field on a sphere. The 64 coefficients define one up to degree 7, and drawing it is cheap, but no one has looked, so nothing is claimed about what it looks like or whether two words that are close in MRR are close on the globe.

The paper's stability result has no translation. Phient's longest rollout is 30 tokens on 40 prompts, judged by trigram perplexity and shared 4-grams. A definition of stable generation, and a run long enough to test it, are both missing. The paper's 1,460 steps are not a target for words, since there is no true trajectory, but a rollout of hundreds of words with a measure of drift is a thing that could be run and has not been.

The helix fixes each channel's (l, m), so the sphere tie's degrees have a fixed meaning. Which degrees a condition damps and which it amplifies has not been read out of any trained tie. The weights exist after every Austen run. They have not been reported.

The byte deadband will act on the sphere tie as it acts on the circle, and a gain close to one will move a channel by less than half a quantum and be lost. How often that happens for eight gains against 64 has not been measured.

And the two limits that stand over the whole series stand here. Phient has done no weather work of any kind. No partnership with anyone has been agreed. This paper is the one where the counterpart is closest, and the distance is still the whole of section 13.

## 16. Sources

Bonev, B., Kurth, T., Hundt, C., Pathak, J., Baust, M., Kashinath, K., Anandkumar, A. "Spherical Fourier Neural Operators: Learning Stable Dynamics on the Sphere." ICML 2023 (oral), Proceedings of Machine Learning Research 202. arXiv:2306.03838.

arXiv abstract page, fetched for the title and abstract: https://arxiv.org/abs/2306.03838

arXiv PDF, fetched for the body, equations, architecture, Tables 1 and 2, training details, timings and the discussion of equivariance: https://arxiv.org/pdf/2306.03838

ICML 2023 virtual site, fetched for the author list and the oral designation: https://icml.cc/virtual/2023/oral/25441

NVIDIA Research publication page, fetched for the author list and the 6 June 2023 date: https://research.nvidia.com/publication/2023-06_spherical-fourier-neural-operators-learning-stable-dynamics-sphere

PMLR proceedings page, located by search and not fetched: https://proceedings.mlr.press/v202/bonev23a.html

Phient facts: `PHIENT_FACTS.md` in this folder, the source of every Phient number above.

Profile: `../17_ANIMA_ANANDKUMAR_PROFILE.md`, the source of every statement about the author in section 14.

Not fetched and not relied on: Driscoll and Healy (1994) and Schaeffer (2013), which the paper cites for the spherical convolution theorem and the SHT algorithm. Their content is reported here only as the paper reports it.
