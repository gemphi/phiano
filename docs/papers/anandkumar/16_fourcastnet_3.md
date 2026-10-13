# The sphere, the noise and the score: what a probabilistic weather operator asks of a word operator

*Phient interpreted papers, 16 of 16 · 2026-09-14 · on: "FourCastNet 3: A geometric approach to probabilistic machine-learning weather forecasting at scale" (Bonev, Kurth, Mahesh, Bisson, Kossaifi, Kashinath, Anandkumar, Collins, Pritchard, Keller, 2025, arXiv:2507.12144)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the last of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. The paper is FourCastNet 3, the July 2025 preprint that turns the FourCastNet line into a probabilistic ensemble model built on the sphere. A note on attribution first. The brief we were given named Mardani, Hundt and Kautz among the authors. The arXiv page for 2507.12144 does not. The authors of record are Boris Bonev, Thorsten Kurth, Ankur Mahesh, Mauro Bisson, Jean Kossaifi, Karthik Kashinath, Anima Anandkumar, William D. Collins, Michael S. Pritchard and Alexander Keller. We use that list and that id. The paper has two versions, 16 and 18 July 2025.

The paper's claim has three parts. First, the geometry: every convolution in the model is a convolution on the sphere, either global through the spherical harmonic transform or local through a discrete-continuous filter, so the model never sees the planet as a rectangle. Second, the probability: the model takes a noise field drawn from diffusion processes on the sphere as an extra input, is trained with a loss built on the continuous ranked probability score, and so learns to produce an ensemble rather than a single blurred mean. Third, the scale: training on 1024 GPUs by splitting the sphere itself across devices, and inference of a 60-day global forecast at 0.25° in under four minutes on one GPU.

We read it for three reasons. The first is that phinet already has a sphere tie, one complex weight per spherical-harmonic degree with 64 channels laid out as Y_l^m for l < 8. That is the convolution theorem on the sphere, which is exactly what the paper's global block uses. The paper is the most complete published account of what that idea looks like when it is finished. The second is that the paper's central problem is the blur that comes from training a deterministic step against a mean-squared loss and composing it. Phient's decoding loop composes an operator thirty tokens deep and has its own measure of collapse, the shared 4-grams across prompts. The third is that the paper's answer, a noise input and a scoring rule that rewards a calibrated spread, is a thing phinet has not built, and it is a thing that can be stated precisely enough to test.

Sections 2 to 5 state the paper in its own terms. Sections 6 to 10 place its objects in the Phient family. Section 11 lists what Phient has measured that bears on it. Section 12 names one experiment and its control. Section 13 says where the analogy breaks. Section 14 is about the person and the moment. Section 15 is what is left.

One limit is stated first. Phient has done no weather work of any kind. There is no ERA5 in gemphi, no grid of the earth, no ensemble, no CRPS. Every Phient number below was measured on words.

## 2. What the paper claims, in its own terms

The abstract is the paper's own summary of its claims, and we follow it. FourCastNet 3 "advances global weather modeling by implementing a scalable, geometric machine learning (ML) approach to probabilistic ensemble forecasting." The approach "is designed to respect spherical geometry and to accurately model the spatially correlated probabilistic nature of the problem, resulting in stable spectra and realistic dynamics across multiple scales."

On skill, the paper says the model "delivers forecasting accuracy that surpasses leading conventional ensemble models and rivals the best diffusion-based methods, while producing forecasts 8 to 60 times faster than these approaches." In the body the comparison is made explicit: the model "beats the gold-standard physics-based NWP model IFS-ENS" and "nearly matches the medium-range forecast skill of GenCast." The speed figure is given as "8x over GenCast and 60x over IFS-ENS."

On calibration and long rollouts, the abstract claims that "in contrast to other ML approaches, FourCastNet 3 demonstrates excellent probabilistic calibration and retains realistic spectra, even at extended lead times of up to 60 days." The body says the model is designed to "retain stable predictions and accurate spectra well into the subseasonal range with lead times of up to 60 days," and that its design "mitigates the issue of blurring" that deterministic models show and addresses the "build-up of small-scale noise" that other probabilistic models show.

On architecture, the claim is that "all of these advances are realized using a purely convolutional neural network architecture tailored for spherical geometry." There is no attention and no transformer. The paper calls the architecture a spherical neural operator and describes it as based on the ConvNeXt block adapted to the sphere, with layer normalization deliberately left out.

On scale, the abstract says that "scalable and efficient large-scale training on 1024 GPUs and more is enabled by a novel training paradigm for combined model- and data-parallelism, inspired by domain decomposition methods in classical numerical models." The training framework is named as Makani, and the paper states that "all key components, including training and inference code, are fully open-source," with code at github.com/NVIDIA/makani.

On inference, the abstract says the model "enables rapid inference on a single GPU, producing a 60-day global forecast at 0.25°, 6-hourly resolution in under 4 minutes." The closing claim is that the model's "computational efficiency, medium-range probabilistic skill, spectral fidelity, and rollout stability at subseasonal timescales make it a strong candidate for improving meteorological forecasting and early warning systems through large ensemble predictions."

These are the paper's claims. They are claims about the atmosphere. Nothing in them is about words, and we keep that separation until section 6.

## 3. The mathematics, plainly

There are three mathematical objects in the paper, and each is simpler than its name.

The first is convolution on the sphere. On a flat grid, a convolution slides a filter and the convolution theorem says the result is the product of Fourier coefficients. On the sphere the same theorem holds with the spherical harmonic transform in place of the Fourier transform. A function on the sphere is written as a sum of harmonics Y_l^m, with degree l and order m, and a rotation-invariant filter is a multiplier on each degree. The paper's global block does exactly this: the "global convolution filters are parameterized in the spectral domain by leveraging the convolution theorem on the sphere and the associated spherical harmonic transform (SHT)." The transform is done with torch-harmonics, described as "a library for differentiable signal processing on the sphere." The local block is a discrete-continuous convolution, a filter defined as a continuous function on the sphere and sampled at the grid, which gives a local receptive field that still respects the geometry. The processor stacks "8 neural operator blocks," and the paper reports that "a ratio of four local blocks to one global block yields the best forecast skill." The embedding dimension is 641.

The second object is the hidden Markov model. A deterministic step maps the state at time n to the state at time n plus six hours. The paper adds a hidden variable: "a hidden Markov model is obtained by conditioning FourCastNet 3 on a stochastic latent variable whose temporal dynamics are governed by a diffusion process on the sphere." Concretely the model takes an extra input z_n, a noise vector drawn from eight spherical diffusion processes with different spatial and temporal scales. The noise is defined in the spectral domain with a standard deviation per degree, σ_l = F_0 exp(−k_T l(l+1)/2), so a small k_T gives noise that is broad on the sphere and a large k_T gives noise that is fine. Different draws of z_n give different members. The ensemble is the set of runs.

The third object is the loss. The continuous ranked probability score compares a predicted distribution with one observed value. For an ensemble it is the mean absolute error of the members against the truth, less half the mean absolute difference between members. It is minimized by an ensemble that is both accurate and honestly spread. The paper's loss is "a composite probabilistic loss function based on the continuously ranked probability score." It combines "the spatial, point-wise CRPS loss term with a loss term in the spectral domain" that "weights spectral coefficients according to their multiplicity." The spectral term is what keeps the small scales alive. A point-wise score alone lets a model win by being smooth.

## 4. Its experiments and numbers as reported

The paper's numbers are of four kinds: the model, the training, the skill, and the speed. We report only what the paper states.

| Item | As reported |
|---|---|
| Input and output grid | 721 × 1440 equiangular, 0.25° |
| Latent grid | 360 × 720 Gaussian |
| Time step | 6 hours |
| Atmospheric variables | z, t, u, v, q at 13 pressure levels |
| Surface variables | u10m, v10m, u100m, v100m, t2m, msl, tcwv |
| Auxiliary inputs | land-sea mask, orography, solar zenith angle |
| Processor | 8 neural operator blocks, four local to one global |
| Embedding dimension | 641 |
| Total parameters | 710,867,670 |
| Noise processes | 8 spherical diffusion processes |
| Ensemble size in training | 16 |
| Ensemble size in evaluation | 50 |
| Training data | ERA5, 1980 to 2016 |
| Test year | 2020, out of sample |

Training is in three stages. In the first, "208,320 gradient descent steps" are taken on 6-hour pairs with a batch size of 16 and an ensemble size of 16, on 1024 NVIDIA H100 GPUs for 78 hours. In the second, "5,040 steps" are taken with four autoregressive rollouts on 512 NVIDIA A100 GPUs for 15 hours. In the third, fine-tuning on 256 NVIDIA H100 GPUs takes 8 hours, using 2012 to 2016 data to correct for drift in the distribution. The reason a model with 710 million parameters can be trained on 1024 GPUs at this resolution is spatial model parallelism: the sphere is split into 4 to 16 pieces across devices, and ensemble and batch parallelism sit on top of that. The paper says this is inspired by domain decomposition in classical numerical models.

The skill results are given for the year 2020 with 12-hourly initial conditions. The paper reports CRPS, ensemble-mean RMSE, spread-skill ratio and rank histograms, and states that the "50-member FCN3 ensemble forecast is well-calibrated with spread-skill ratios approaching 1." It notes that the ensemble is slightly over-dispersive in the first 24 hours and then relaxes to a flat rank histogram, and concludes that "larger ensembles are justifiable" since 50 members were used against 16 in training. Against baselines, the paper says the model beats IFS-ENS and nearly matches GenCast on medium-range skill. We do not reproduce per-variable CRPS values here because we did not extract them from the figures.

On spectra the paper reports that the power spectral density slopes are kept at all wavenumbers, with relative error at high frequencies bounded in a band from −0.2 to 0.2, and that this holds to 60 days. On speed, a single 15-day forecast runs in 60 seconds on one H100, a 60-day forecast in under 4 minutes, and the comparison is "8x over GenCast and 60x over IFS-ENS," with GenCast timed at 8 minutes at half the temporal resolution on a TPU v5.

## 5. The one idea, restated

The one idea is this: if the geometry is right and the score is right, a single step can be composed for two months without the field decaying into a blur or a hiss.

The first FourCastNet learned a deterministic step and measured it by squared error. The trouble with a squared error is known and the paper names it: an uncertain future has many possible states, and the single prediction that minimizes squared error is their average, which is smoother than any of them. Compose that step and the smoothness compounds. The spectrum falls at high wavenumbers and the forecast goes soft. The diffusion models that came after fix the blur by sampling, but the paper says they show the opposite failure at long lead times, a build-up of small-scale noise.

FourCastNet 3 answers with three moves that only work together. The noise input makes the model a family of steps rather than one step. The CRPS loss makes the family accountable: an ensemble is scored on how well its spread covers the truth, so the members are pushed apart where the future is uncertain and pulled together where it is not. The spectral term in the loss scores the family on its wavenumber content, so it cannot win the point-wise score by being smooth. And the sphere holds it all in place: because every filter is a filter on the sphere, there is no seam at the dateline and no pinch at the poles where errors could seed.

Stated in Phient's grammar, the idea is that an operator X ⇒ Y under c should be trained to produce a distribution of Y, scored as a distribution, with the geometry of the domain built into the operator rather than learned. That is a stronger discipline than phinet has today. Phinet's operator ranks candidates for the next word and is scored by its rank. It does not sample, and its spread is not measured. The paper's discipline, if it can be moved, would give phinet a score for its uncertainty and a spectral penalty for its collapse.

## 6. A word as a function: the field and the phasor

The paper's state is a field: at every point of a 721 × 1440 grid, a vector of about 72 quantities, encoded into 641 channels on a coarser grid. Phient's state is also a field, though a very different one. A word is 32–64 relations, each a complex phasor on a closed circle with a count. In phinet this is a tensor [b,w,s], complex, with a Tape for autograd, and the sphere tie lays 64 channels out as Y_l^m with l < 8, which is exactly 8² = 64.

The correspondence is exact at the level of type and loose at the level of content. In both cases the object the operator moves is a function on a compact domain, transformed to a spectral basis, multiplied by learned complex weights, and transformed back. In phinet the domain is the relation circle, or in the sphere tie the sphere, and the function's value at a coordinate is a phasor with a count. In FourCastNet 3 the domain is the earth and the value is wind, pressure, temperature and humidity.

What the paper adds to this picture is that the field is not enough. A field with a deterministic operator gives one future. The paper's field has a companion: z_n, eight noise fields on the same sphere, each drawn from a diffusion process at its own scale. The state passed to the operator is the pair (field, noise). This has a natural reading in Phical. A word's function is the pair (phasor, count), and the count is exactly the part of a relation that is not a position. The paper's noise is a position with no count, a place on the sphere at random. The two decompositions are not the same, and we are not claiming that Phient's count is the paper's noise. But both say that a function alone does not carry uncertainty, and that uncertainty has to be given its own channel.

The second thing the paper adds is the resolution of the field. Phinet has 64 channels; the paper has 641 channels at 360 × 720 points. The gap is about five orders of magnitude in the size of the state. This matters for what transfers. The paper's spectral loss is meaningful because there are 360 degrees of harmonics to keep honest. In the sphere tie there are 8. The idea that the spectrum of the field should be preserved under composition still makes sense at l < 8, but it is a different kind of measurement, and one Phient has not made.

## 7. The relation channel as a coordinate: patches, modes, and the sphere

In the first FourCastNet the coordinate was a patch, and the patches were mixed by an adaptive Fourier operator that treated the flat grid as periodic. The paper we read now removes that. The coordinate is a point on the sphere, and the basis is the harmonics. The reason given is stability at long lead times: a filter that respects the geometry does not create artifacts at the seam or the poles, and a model that does not create artifacts has nothing to amplify when it is composed.

Phient's coordinate is the relation channel. A channel is one of 64 slots, each holding a phasor and a count, and in the sphere tie each slot is one harmonic Y_l^m with l < 8 and |m| ≤ l. The tie has one complex weight per degree, so weights are shared across the orders of each degree. That is a rotation-invariant filter on the sphere, which is the same constraint the paper's global block imposes. The paper's local block has no counterpart in phinet. Phinet has no notion of a neighbourhood on the sphere, only of a degree.

The facts file records what this tie measured on Austen. The sphere tie has 1,192 parameters against the circle's 8,360. On the Austen hinge task at 3 epochs it reaches 12.8× chance on validation and 13.1× on test, hit@10 0.182, in 89 seconds, where the circle reaches 14.7× and 14.6× in 142 seconds. On the synthetic degree-filter task with 200 items and 40 epochs, the circle gets MRR 0.662 against an identity of 0.646, and the sphere gets 1.000. So the sphere is a weaker general tie on Austen with a seventh of the parameters, and a perfect tie on the task it was shaped for.

Here is where the paper's ratio matters. The paper found that four local blocks to one global block was best. A pure spectral filter, one weight per degree, is the global block alone. The paper does not claim that the global block alone is enough; it claims the mix is. Phinet's sphere tie is the global block alone, and the Austen result, 12.8× against 14.7×, is consistent with the paper's finding that a purely global filter loses something. Whether a local block on the relation sphere would recover it is not measured. Phient read the makani and torch-harmonics sources on 2026-09-27, and the facts file records the sphere tie as what phinet built from that reading. The local convolution was not built.

## 8. The operator as X ⇒ Y under c: one step, then a family of steps

The Phical operator is X ⇒ Y under c. In phinet's Austen work X is a word's function, Y is the next word's, and c is the text between them. The operator is trained on 204,592 examples and scored by how it ranks the true next word among a pool of 1,228, with chance MRR 0.0063.

FourCastNet 3 has the same shape with one addition. X is the atmosphere at n, Y is the atmosphere at n plus six hours, and c is a set of fixed inputs: land-sea mask, orography, and the solar zenith angle, which is the time of day and year written as a field on the sphere. The addition is z_n. The operator is X ⇒ Y under (c, z), and z is drawn fresh at every step from a process whose temporal dynamics are a diffusion on the sphere. So the operator does not produce Y. It produces a family of Y, one per draw of z, and that family is what the loss scores.

This is a real difference and it is the one the paper is about. Phinet's softmax head at β = 8 produces a ranking over the pool, and the ranking is a distribution in a weak sense. But it is a distribution over which word, not a distribution over what function. The paper's ensemble is a set of functions, each a complete state. In Phient's terms it would be a set of complete word functions, each with 64 phasors and counts, and the loss would compare that set to the true next word's function.

The paper's training makes a second point about composition. Stage one trains single steps. Stage two trains with four autoregressive rollouts, so the loss sees the operator composed four times and the noise drawn four times. Stage three fine-tunes on recent years. Phinet's decoding loop composes the operator thirty tokens deep on 40 prompts and measures the result by perplexity under a Kneser-Ney model, 66.9 for the operator's nearest-k decoding against 13.3 for a trigram sampler, and by shared 4-grams across prompts, 8.7% for the operator against 25.7% for the sampler. Phinet has never trained through the composition. Every Austen operator was trained on single pairs. The paper's second stage says that training through the rollout is one of the things that keeps the rollout stable, and phinet has not tried it.

## 9. Where the sphere, the graph and the tensor land

The Phient family has a place for each of the paper's objects, and it is worth saying which is which without stretching.

The sphere lands in phinet::Tie::Sphere. This is the tie already described: 64 channels as Y_l^m with l < 8, one complex weight per degree, a unitary transform, learned in the same Tape as the circle tie. It is the paper's global block with no local block and no encoder. The paper's encoder is a single local spherical convolution from 721 × 1440 to 360 × 720; phinet's function is already in the basis, so there is nothing to encode.

The tensor lands in phinet's Tensor [b,w,s] complex. The paper's tensors are far larger, and the paper's contribution here is not the tensor but its split. Spatial model parallelism cuts the sphere into 4 to 16 pieces across GPUs, with ensemble and batch parallelism on top. Phinet has no parallelism of that kind and no need for it at its size. The facts file records that the engine is allocation and bandwidth-bound at these sizes, with a 1 MB op taking 2 to 5 ms. The paper's engineering is for a regime three or four orders of magnitude above that.

The graph does not appear in this paper. The earlier papers in this series, the graph kernel network and the multipole operator, put the kernel on a graph; the paper we read now is a return to a fixed basis on a fixed domain. In Phient the graph is phiecs: the entity-component world where the quest lives. The paper has no graph because the atmosphere has no entities. Words do.

The noise lands nowhere yet. Phinet has a byte deadband, which is a source of quantization error: a channel is one byte, 256 quanta of 2π, and the nudge 0.06·sin d falls under half a quantum once sin d < 0.2, so nearest rounding froze every pair 8 quanta short, and dithered rounding was added to break the freeze. Dither is noise. But it is noise added to escape a rounding floor, not noise drawn as a conditioning input, and it is not scored. The facts file records its cost: dither takes corpus attribution from 71% to 65%, so it ships off by default. That is the closest thing in Phient to the paper's z_n, and it is not close.

The helix lands beside all of this as the store. The lexicon as a DNA-coded tree encodes each channel as a codon, with a sphere degree and order per codon, and round-trips 18,496 words in 1.9 seconds and 75 MB, byte-exact. The paper's equivalent is the checkpoint, and it has nothing to say about how a state is stored, only how it is moved.

## 10. The agent loop: the quest and the ensemble

The paper is not about agents, and we do not pretend it is. But its ensemble has a shape that the quest already has, and the shape is worth stating.

An ensemble forecast is a set of trajectories from one starting state. Each is a legitimate run of the same operator. What makes them different is the noise, and what makes them useful is that their spread is calibrated: where they disagree, the atmosphere is uncertain, and where they agree it is not. The paper measures this with the spread-skill ratio and rank histograms and reports the ratio approaching 1 for a 50-member ensemble.

The quest in phiecs is a walk. Given a target word and a kind of relation, is-a, part-of, near, used-for or unlike, the agent moves from a word along a channel and reads what it lands on. The facts file records the synthetic oracle: 256 words in families of 8, 1,280 rows, 293 target words, chance MRR 0.021. The shipped trainer reached 0.082 MRR. Attraction-only training with dithered rounding reached 0.594 MRR, hit@1 34.8%, hit@5 97.4%, with within-kind 0.747, wrong channel 0.015, and offset reading 0.005. Held out 20%, one hop was 0.011, two hops is-a 0.095, used-for 0.093, and part-of, near and unlike about chance.

Read as an ensemble, these numbers say something. Hit@5 97.4% and hit@1 34.8% together mean that the walk almost always has the answer in its top five and often does not have it first. That is a distribution with a spread. It is not a calibrated spread, because nothing in the quest scores it as one. And the held-out numbers say that the spread does not transfer: one hop on unseen words is at chance.

The paper's loop suggests what the quest lacks. The atmosphere model is run 50 times from one state with different noise, and the set is scored. The quest is run once. If the quest's walk took a noise input and were run many times, the set of landing words would be an ensemble, and CRPS on the relation circle, which is a well-defined quantity for a phasor and a set of phasors, would score it. The paper's second lesson, training through the rollout, maps onto the two-hop held-out results: two hops is-a at 0.095 was learned from one-hop training, and the paper says the composed step should be in the loss. None of this is done. It is a reading of the paper, not a result.

## 11. What Phient has already measured that bears on it

Everything in this section is from the facts file, and nothing is added.

| Phient measurement | Bears on |
|---|---|
| phinet vs PyTorch reference: 62/62 parity tests | The engine the sphere tie runs on is checked against a reference. |
| Sphere tie: 1,192 params; Austen hinge 3 epochs 12.8× valid / 13.1× test, hit@10 0.182, 89 s | The paper's global block, alone, on words. |
| Circle tie: 8,360 params; 14.7× / 14.6×, 142 s | The comparison the sphere tie is measured against. |
| Synthetic degree-filter task (200 items, 40 epochs): circle MRR 0.662 (identity 0.646), sphere 1.000 | The sphere tie is exact on a task defined by degree. |
| Softmax wide+deep (50,016 params): 27.6× at 3 epochs, 30.7× / 30.3× at 8, hit@10 0.348 | The best deterministic operator on Austen; the sphere at this head is unrun. |
| Decoding (40 prompts × 30 tokens): operator nearest-k KN perplexity 66.9 (was 97.2), union 66.8, sampler 13.3 | Composition of the operator thirty steps deep. |
| Shared 4-grams across prompts: sampler 25.7%, operator 8.7% | The nearest Phient measure of collapse under composition. |
| Byte deadband: 256 quanta; nudge under half a quantum once sin d < 0.2; nearest rounding froze every pair 8 quanta short | The one place noise enters phinet, as dither. |
| Dither costs attribution 71% → 65%; ships off | The measured cost of that noise. |
| Quest: 0.594 MRR, hit@1 34.8%, hit@5 97.4%; held-out one hop 0.011 | A spread with no calibration; composition that does not transfer. |
| Engine bandwidth-bound; 1 MB op 2–5 ms | Where the cost lives at Phient's size. |

The sphere row is the one that speaks to the paper directly. Phinet built a spectral filter on the sphere with one weight per degree, and on Austen it is somewhat below the circle tie with a seventh of the parameters, while on a synthetic task built around degree it is perfect. The paper's finding that four local blocks to one global block beats a global block alone is a hint about why. That hint is not tested.

The decoding rows speak to the paper's problem, blur under composition, and to its measure. Phient's operator produces less repetition across prompts than the sampler, 8.7% against 25.7%, and worse perplexity, 66.9 against 13.3. The paper's ensembles are measured by spectra and calibration. Phient's decoding is measured by perplexity and repetition. These are different instruments and the numbers are not comparable across them.

What is not measured is longer than what is. No ensemble has been run in phinet. No CRPS has been computed. No spectral loss has been used. The sphere tie has not been run with the softmax or wide+deep head. No operator has been trained through its own composition. No local convolution on the relation sphere exists. The facts file says so, and we say so.

## 12. The experiment that would test the paper's idea inside phinet

The paper's idea, in the form that can be moved to phinet, is this: give the operator a noise input, train it as a family of steps under CRPS with a spectral term, and measure whether the family is calibrated and whether its spectrum survives composition. Here is the experiment, with its control.

The task is the Austen operator as it stands: pool 1,228 words seen 20 or more times, 204,592 training examples, chance MRR 0.0063, hinge head, 3 epochs, sphere tie. That is the configuration measured at 12.8× valid and 13.1× test in 89 seconds, and it is the control. Nothing about it changes.

The treatment adds a noise channel. Following the paper, draw z as a random field on the sphere in the harmonic basis, with a standard deviation per degree that falls with l(l+1), so the noise is broad rather than fine. Phinet's sphere tie has 8 degrees, so this is 8 scalars. Concatenate z to the input function and train the operator on the pair. At each training step draw 16 samples of z, run the operator 16 times, and score the 16 outputs against the true next word's function with CRPS computed per channel on the relation circle, plus a term that compares the per-degree power of the ensemble to the per-degree power of the target. The 16 is the paper's training ensemble size and there is no reason from Phient's side to change it.

Three measurements decide it. First, ranking: MRR and hit@10 of the ensemble mean against the pool, reported against the control's 12.8× and 0.182. If the ensemble mean ranks worse, the paper's method costs skill on words, and that is a result. Second, calibration: the spread-skill ratio of the ensemble on the held-out set, which has no control because the control has no spread. A ratio near 1 would be the paper's result reproduced on words. Third, composition: run the decoding loop on the 40 prompts for 30 tokens with an ensemble of 16 at every step, and report KN perplexity and shared 4-grams against the operator's 66.9 and 8.7%. The paper's claim is that the spectral term prevents collapse under composition. On words, collapse is repetition, and shared 4-grams is the measure phinet already has.

Two things are stated plainly. Cost: 16 runs per step on a bandwidth-bound engine means about 16 times the control's 89 seconds, so around 24 minutes, which is not a barrier. Risk: CRPS on a circle is well-defined for a phasor but the count is not a phasor, and how to score the count is a decision that has to be made before the run and reported with it. If the count is left out of the score, the result is about phase only, and the paper says nothing about how that would go.

## 13. Where we disagree, or where the analogy breaks

The analogy breaks in four places, and it is better to name them than to lean on it.

The first is the domain. The paper's sphere is the earth. The values at neighbouring points are correlated because the air moves between them, and the harmonic basis is right because the dynamics are rotation-invariant to a good approximation. Phinet's sphere is a layout: 64 channels assigned to Y_l^m with l < 8. There is no physical claim that a word's relations are rotation-invariant on that sphere, and the Austen result, sphere below circle, is at least consistent with the claim being false. The synthetic degree-filter result at 1.000 shows the tie is exact when the task is defined by degree, which is what a spectral filter should do, and says nothing about whether words are.

The second is the noise. The paper's noise is a physical prior: the atmosphere is uncertain at every scale and the diffusion processes encode which scales. Phient has no such prior. A word's next word is uncertain, but the uncertainty is over which word, and the paper's method scores uncertainty over a field. Mapping "which word" onto "which field" is the step the experiment in section 12 takes, and it may be the wrong step.

The third is the score. The paper claims a diffusion model of the same lineage shows a build-up of small-scale noise and a deterministic model shows blur, and that CRPS with a spectral term sits between them. Phinet's decoding is measured by perplexity under a model that phinet is not, a Kneser-Ney trigram, and by repetition across prompts. The operator scores badly on perplexity and well on repetition. We do not know that these two measures are the blur and the hiss, and we should not say they are.

The fourth is scale, and here we disagree with a reading rather than the paper. The paper's engineering, 1024 GPUs and spatial domain decomposition, is the reason its result exists at 0.25°. A reader might take it that the method needs that scale. The method does not; the atmosphere does. Phinet's engine at 1 MB an op is a regime where the paper's parallelism is irrelevant, and the paper's three ideas, sphere, noise, score, are each small. That cuts both ways. If the ideas work at phinet's size, they work for reasons that have nothing to do with the paper's scale, and the paper cannot be cited as evidence for them.

One more thing. The paper says its model excludes precipitation and that it is evaluated on one year, 2020. The first FourCastNet was praised for precipitation. We take the paper at its word that this is a scoping choice, and we note it because a word model that excluded the hardest word class would be asked the same question.

## 14. The person and the moment

Everything here is from the profile and nothing is claimed about her intentions.

Anima Anandkumar is the Bren Professor of Computing and Mathematical Sciences at Caltech and, since 2018, senior director of machine-learning research at NVIDIA. She is the pioneer of neural operators, of the Fourier Neural Operator in particular, and the senior author of FourCastNet. On 16 March 2026 the United Nations appointed her to its Scientific Advisory Board. She was named to the Time100 AI list in 2026 and received the Time100 Impact Award in 2025. The profile records her stated view: "We have foundation models for language, not for physics," and her method in her words, "All of the things that work with deep learning, let's take them, but make them a bit more principled."

The paper we read is her group's work, and her place in its author list is seventh of ten. The profile describes the first FourCastNet as putting an adaptive FNO on the whole planet and says that "later versions moved to spherical harmonics to respect the geometry of the globe; FourCastNet 3 holds stable over long global rollouts." That is what this paper is. The first author is Boris Bonev, and the paper credits torch-harmonics as the library that does the spherical transform, which is the same idea the profile gives as hers: do the expensive part in the frequency domain, on the right basis.

The moment for Phient is the end of this series. Sixteen papers, from tensor decompositions to this one, have been read through one paradigm. The profile says that Phinet is, mechanically, her idea applied to a different domain, and that the step from her work to ours is the claim that a word is a field and meaning is the operator that evolves it under a condition. This paper adds a second claim to hers that Phient has not made: that the operator should be probabilistic, scored as a distribution, and built on a geometry it does not have to learn. The profile's second honest note holds here too. Nothing in this series is her endorsement. No partnership has been agreed with anyone, and the facts file says so. The profile says a partnership is something to earn with a result she would recognise, an operator over a function space she has not seen, measured the way she measures. The paper we read is the clearest statement in the series of how she measures. Calibration, spectra, and stability under composition are the three things it asks for, and none of them has been measured on a word.

## 15. What remains

What remains is mostly work, and it can be listed by what the paper asks for.

The sphere tie needs its other heads. It has been run with the hinge head at 3 epochs. The circle tie has been run with softmax at β = 8 and with a wide and deep head to 30.7×. The sphere at those heads is unrun, and until it is, the Austen comparison, 12.8× against 14.7×, is a comparison at the weakest configuration. The paper's local block, a discrete-continuous convolution on the sphere, has no phinet counterpart, and the paper's evidence that the mix beats the global block alone is the strongest reason to build one.

The noise channel and the score need to exist. Section 12 describes the run. What it needs before the run is a decision on how to score the count, since CRPS on the relation circle is defined for a phasor and Phient's relation is a phasor with a count. That is a design decision, not a measurement, and it should be written down with its reasons.

Composition needs to be in the loss. The paper's second training stage puts four steps of rollout in the loss. Phinet has trained on single pairs only, and the quest's held-out two-hop numbers, 0.095 for is-a and 0.093 for used-for, are what one-hop training gives. Training through two hops is a small change to the trainer and a direct test of the paper's claim on the quest.

Calibration needs an instrument. Phinet's decoding is measured by perplexity and repetition. Neither is a spread-skill ratio or a rank histogram. If the operator is to produce an ensemble, those two instruments have to be written, and they are short.

The larger thing that remains is the one this series has circled for sixteen papers. Every Phient number in these papers is from Austen, three writers, or a synthetic oracle of 256 words. No Phi-4 run exists. The paper we read trained on 36 years of the atmosphere and tested on a 37th. The analogous discipline for Phient is a held-out corpus that was never seen, at a size where the shared 4-gram measure means something, and that has not been done. The last thing that remains is what the facts file says at its end: Phient has done no weather work of any kind, and this paper, the most finished of the sixteen, is the one where that matters least, because its three ideas are about operators and not about weather.

## 16. Sources

Fetched 2026-09-14 for this paper.

Bonev, B., Kurth, T., Mahesh, A., Bisson, M., Kossaifi, J., Kashinath, K., Anandkumar, A., Collins, W. D., Pritchard, M. S., Keller, A. FourCastNet 3: A geometric approach to probabilistic machine-learning weather forecasting at scale. arXiv:2507.12144, v1 16 July 2025, v2 18 July 2025. Abstract page: https://arxiv.org/abs/2507.12144

Same paper, HTML full text (v2), used for the architecture, training, noise and loss details quoted above: https://arxiv.org/html/2507.12144v2

The paper states its code is at https://github.com/NVIDIA/makani. It was not fetched when this paper was written; the repository was cloned on 2026-09-27 (commit ce1366b, 2026-09-09) and read for phiano docs/84, together with its spherical-harmonic engine https://github.com/NVIDIA/torch-harmonics (commit 69cdf96) and the FNO reference https://github.com/neuraloperator/neuraloperator (commit 00b7d86). Copies without `.git` are at `gemphi\ext\makani`, `gemphi\ext\torch-harmonics` and `gemphi\ext\neuraloperator` (each with a `PROVENANCE.txt`), beside `gemphi\ext\context-language-models` of paper 24.

Phient sources, in the same folder: PHIENT_FACTS.md for every Phient number, and ../17_ANIMA_ANANDKUMAR_PROFILE.md for section 14.

A note on the brief. The brief named Mardani, Hundt and Kautz as authors. They are not on the arXiv author list for 2507.12144, and this paper uses the list on the arXiv page.
