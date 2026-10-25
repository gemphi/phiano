# The planet as one function: what a weather operator teaches a word operator

*Phient interpreted papers, 9 of 16 · 2026-05-04 · on: "FourCastNet: A Global Data-driven High-resolution Weather Model using Adaptive Fourier Neural Operators" (Pathak, Subramanian, Harrington, Raja, Chattopadhyay, Mardani, Kurth, Hall, Li, Azizzadenesheli, Hassanzadeh, Kashinath, Anandkumar, 2022, arXiv:2202.11214)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the ninth of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. The paper is FourCastNet, the February 2022 preprint that took the Fourier Neural Operator off the textbook PDEs of the earlier papers and put it on the whole atmosphere. Its claim is narrow and large at once. Narrow, because the model does one thing: given the state of twenty atmospheric variables on a 0.25° global grid at one instant, it produces the state six hours later, and it is run again on its own output until a week has passed. Large, because the paper reports that this single learned step matches the European Centre's operational forecast system at short lead times on the big slow variables, beats it on precipitation, and does so in under two seconds for a week rather than the hours a numerical model needs on thousands of nodes.

We read it here for three reasons. The first is that the paper's central object, a field on a grid that is transformed to the frequency domain, multiplied by learned weights, transformed back and advanced one step, is exactly phinet's operator on a word function, run on a different domain. Phinet's Fno takes a tensor [b,w,s], applies a unitary FFT, multiplies the retained modes by learned complex weights, and returns. FourCastNet's AFNO does the same on patches of the planet. The second reason is that FourCastNet is trained one step ahead and then fine-tuned two steps ahead, and its forecast is the composition of that step twenty-eight times. Phient has an operator that moves one word toward the next, and a decoding loop that composes it thirty tokens deep. The paper is a careful study of what happens when a learned step is composed. The third reason is that the paper's later versions moved to spherical harmonics, and phinet already has a sphere tie with one weight per spherical-harmonic degree. The paper is where the sphere first became necessary.

Sections 2 to 5 state what the paper says, in its own terms and with the numbers it reports. Sections 6 to 10 place the paper's objects inside the Phient family. Section 11 lists what Phient has measured that bears on any of this, all from the facts file. Section 12 names one experiment that would test the paper's idea inside phinet, with its control. Section 13 says where the analogy breaks. Section 14 is about the person and the moment. Section 15 is what is left.

One limit is stated first. Phient has done no weather work of any kind. There is no ERA5 in gemphi, no grid of the earth, no forecast. Every number about Phient below was measured on words, and where the paper's idea has not been tried on words we say it has not been tried.

## 2. What the paper claims, in its own terms

FourCastNet is short for Fourier Forecasting Neural Network. The abstract calls it a global data-driven weather forecasting model that provides accurate short to medium-range global predictions at 0.25° resolution. The paper is explicit about what it forecasts well. It says the model accurately forecasts high-resolution, fast-timescale variables such as the surface wind speed, precipitation, and atmospheric water vapor, and that this matters for wind energy planning and for extreme events such as tropical cyclones, extra-tropical cyclones, and atmospheric rivers.

The comparison the paper makes is with the ECMWF Integrated Forecasting System, IFS. The abstract says FourCastNet matches the forecasting accuracy of IFS at short lead times for large-scale variables, while outperforming IFS for variables with complex fine-scale structure, including precipitation. This is a claim of parity on the slow fields and superiority on the fast ones, and the paper's own plots are the evidence. The paper does not print a table of ACC or RMSE values against IFS in the text. The curves are in the figures, and the text says the predictions are very competitive with IFS.

The second claim is speed. The abstract says the model generates a week-long forecast in less than 2 seconds, orders of magnitude faster than IFS. Section 4 makes this precise: about 45,000 times faster than traditional NWP models on a node-hour basis, and about 12,000 times less energy to generate a forecast. The paper draws the consequence itself. Speed of this kind enables the creation of rapid and inexpensive large-ensemble forecasts with thousands of ensemble members, and the paper says fast generation of 1,000-member ensembles dramatically changes what is possible in probabilistic weather forecasting.

The third claim is a position rather than a measurement. The abstract says data-driven deep learning models such as FourCastNet are a valuable addition to the meteorology toolkit to aid and augment NWP models. The paper does not say it replaces them. Section 6 lists what the model lacks: it has no data assimilation component, so it cannot yet generate up-to-the-minute forecasts; it imposes no physics constraints; and whether it generalises under the climate change expected in coming decades is an open question the authors raise and do not answer.

The scope is twenty variables. Table 1 lists them: 10 m winds U10 and V10, 2 m temperature, surface pressure, mean sea-level pressure, wind and geopotential at 1000 hPa, temperature, wind, geopotential and relative humidity at 850 and 500 hPa, geopotential at 50 hPa, and total column water vapour. Each training sample is a tensor of shape 721 × 1440 × 20. Precipitation is not among the twenty. It is diagnosed by a second model from the outputs of the first. The whole is trained on ERA5 reanalysis from 1979 to 2015, validated on 2016 and 2017, and tested on 2018 and beyond.

## 3. The mathematics, plainly

The model is a vision transformer whose attention has been replaced by a Fourier mixing step. The paper credits the architecture to Guibas et al. and describes it as combining the Fourier Neural Operator approach of Li et al. with a ViT backbone. The reason is cost. Self-attention over N tokens costs N². The tokens here are the patches of a 720 × 1440 grid cut 8 × 8, which is 90 × 180, or 16,200. The Fourier mixing costs on the order of N log N, and the paper states this.

One AFNO layer does three things. First it takes the grid of patch embeddings, each of dimension 768, and applies a two-dimensional discrete Fourier transform across the patch grid, so each token becomes a coefficient z_{m,n} indexed by two wavenumbers. Second it applies, to each coefficient, a small two-layer MLP whose weight matrices are block-diagonal, with the same weights shared across all wavenumbers, followed by a soft-thresholding S_λ(x) = sign(x) max(|x| − λ, 0) that pushes small coefficients to zero. The threshold λ is 10⁻². Third it applies the inverse transform and adds the result to the input as a residual. The paper's Table 3 gives the rest: eight blocks in the block-diagonal MLP, depth 12, MLP ratio 4, GELU activation, no dropout.

The step in the frequency domain is a shared nonlinear map applied to every mode, where the original FNO had a fixed weight per mode. That is what "adaptive" means in the name. The map does not know which wavenumber it is looking at. It looks at the 768 values of the coefficient and the soft threshold decides what to keep. What survives is a sparse set of modes, chosen by content rather than by a fixed cutoff.

The training objective is one step of time. Let X(k) be the twenty-variable field at time k, with time steps six hours apart. Pre-training learns X(k) ⇒ X(k+1) directly, with a loss comparing the prediction to the ERA5 field. Fine-tuning then learns two steps. The model produces X(k+1) from X(k), feeds its own output back in to produce X(k+2), and the loss compares both outputs to their ground truth. The paper is explicit that this second phase exists to make the model stable when composed.

Inference is composition. A forecast of length L is the model applied L times to its own output. A week is 28 steps. The precipitation model is a separate AFNO with an added 2D convolutional layer with periodic padding and a ReLU, trained on the target log(1 + TP/ε) with ε = 10⁻⁵, and it reads the backbone's output at each step rather than being composed itself.

The metrics are latitude-weighted by cos(lat) normalised to mean one, so the poles count for what they are. ACC is the correlation between predicted and true anomalies from a long-term climatological mean. RMSE is the weighted root mean square of the raw difference.

## 4. Its experiments and numbers as reported

The training set is ERA5 at hourly resolution, sub-sampled to snapshots six hours apart, re-gridded from ECMWF's Gaussian grid to a regular grid by the Copernicus Climate Data Store interpolation. The paper says the training set consists of 54,020 samples and the validation set of 2,920. Variables are standardised to zero mean and unit variance. Pre-training runs for 80 epochs at learning rate 5 × 10⁻⁴, fine-tuning for 50 epochs at 1 × 10⁻⁴, and the precipitation model for 25 epochs at 2.5 × 10⁻⁴, all on a cosine schedule with a global batch size of 64. The paper says end to end training takes about 16 hours wall-clock time on a cluster of 64 Nvidia A100 GPUs.

Skill is reported as ACC and RMSE curves against lead time for the test year 2018, alongside IFS. The text does not print the values. It says the predictions are very competitive with IFS. We do not read numbers off the figures here. What the text states as a number is limited to the following.

| what | reported |
|---|---|
| week-long forecast | under 2 seconds |
| 100-member 24-hour ensemble | 7 seconds, one node with 4 A100s |
| speed vs NWP, node-hour basis | about 45,000× at 18 km; 145,000× at 30 km |
| energy vs IFS | about 12,000× less at 18 km; 24,000× at 30 km |
| Table 2 ratios, IFS / FCN at 18 km | latency 44,727; energy 12,318 |
| ensemble | 100 members, initial-condition Gaussian noise scaled by σ = 0.3 |
| ensemble mean gain | beyond 70 h for U10, beyond 100 h for Z500 |
| TCWV in an atmospheric river | ACC > 0.6 beyond 8 days |
| extreme precipitation, RQE | FourCastNet underestimates extreme percentiles by ~35%, IFS by ~15% |

The paper quotes IFS at an 18 km grid with the archive at 30 km, and gives FourCastNet's own number at 30 km with the 18 km figure extrapolated. The 45,000 is the extrapolated one.

The case studies are three. Hurricane Michael, initialised on 7 October 2018, is forecast from a tropical depression to a category 5 hurricane to landfall within 72 hours, and the paper says the model fails to fully forecast the extent of the sharp drop in pressure between 36 and 48 hours. Super Typhoon Mangkhut, initialised 8 September 2018 at 00:00 UTC, has its formation and track captured. An atmospheric river initialised 4 April 2018 at 00:00 UTC is followed in total column water vapour.

With 100 ensemble members the mean is worse than the single run under 48 hours, because averaging smooths fine structure, and better beyond 70 hours for wind and 100 hours for geopotential. Section 5 compares against DLWP of Weyn et al., a convolutional network on a cubed sphere at 2°, and says FourCastNet works at 8 times the resolution with more variables. No parameter count is stated in the text.

## 5. The one idea, restated

The idea is that the state of the atmosphere at one instant is a function on the sphere, that the next state is another function on the same sphere, and that the map between them can be learned as an operator in the frequency domain and then composed with itself to reach any horizon. Everything else in the paper is the engineering that makes the idea fit in memory and stay stable when composed.

Three consequences follow and the paper makes each one.

The first is that the learned step must be cheap in the number of grid points. The whole reason the AFNO exists is that a transformer over sixteen thousand patches is too expensive in attention, and the Fourier transform gives global mixing at N log N. The paper's claim of parity with IFS is downstream of that choice. So is the speed claim. The two-second forecast and the twelve-thousand-fold energy figure are consequences of the operator being a transform, a pointwise map, and a transform back.

The second is that composition is the whole test. A model that predicts six hours ahead well and drifts by day three has learned a map that is fine on the training distribution and bad on its own outputs. The paper's two-step fine-tuning is a direct response to this. It trains the step on its own product. The ensemble results say the same thing from the other side: at long lead times the deterministic run has drifted enough that the mean over perturbations is better.

The third is that cheapness changes what questions can be asked. Section 3.4 and section 6 are about ensembles because a thousand members are affordable only if one member costs seconds. The paper's practical argument for probabilistic forecasting of extremes rests on that arithmetic.

Restated as a sentence in the paradigm's shape: the atmosphere at time k, under the condition that six hours pass, becomes the atmosphere at time k+1, and the operator that carries it is learned once in the frequency domain and composed as many times as the horizon demands. X ⇒ Y under c, where c is Δt = 6 h. The paper also says clearly what the idea does not give. It gives no physics, no conservation, no assimilation of new observations into a running forecast, and no promise about a climate the training years did not contain. That honesty is the part of the paper we most want to copy.

## 6. A word as a function: the field and the phasor

In FourCastNet a sample is a tensor of shape 721 × 1440 × 20. Two axes are position on the planet and one is which variable. The model treats the position axes as the domain of a function and the variable axis as the value. The Fourier transform runs over position. The learned map runs over value. The paper never says this in these words, but the split is in Table 3: the DFT is two-dimensional over patches, the MLP is over the 768-dimensional embedding of each patch.

Phical makes the same split on a word. A word is 32–64 relations, each a complex phasor on a closed circle with a count. The relation index is the domain. The phasor is the value. Phinet's Tensor [b,w,s] has a batch axis, a word axis, and a spectral axis, and the unitary FFT runs over the spectral axis while learned complex weights act on what the transform produces. So a word in phinet is a function on a small closed domain of 64 channels, where FourCastNet's sample is a function on a large closed domain of 16,200 patches. The domain in both cases is periodic. FourCastNet's is periodic in longitude and, on the regular grid the paper uses, treated as periodic in latitude by the DFT as well. Phical's circle is periodic by construction.

The count is where the analogy needs care. FourCastNet's value at a grid point is twenty standardised real numbers. Phical's value at a channel is a phasor plus a count of how many times that relation was seen. FourCastNet has no count. Every grid point is equally trusted because ERA5 fills every cell. Phical's counts exist because a corpus does not fill every channel of every word equally, and a channel seen twice should not move a word the way a channel seen two thousand times does.

What the analogy gives back is the sense in which phinet is one of FourCastNet's kind. Both take a function to the frequency domain, act there, and return. In FourCastNet the return is a new atmosphere. In phinet it is a new word function, and the Austen measurements are of exactly this: an operator trained to move one word's function toward the next word's, scored by where the true next word ranks. The 30.7× chance at 8 epochs for the wide and deep softmax operator is that number. It is a ranking measure and not an ACC.

The paper's soft threshold sits beside the byte deadband. FourCastNet pushes small Fourier coefficients to zero on purpose, with λ = 10⁻². Phinet's channel is one byte, 256 quanta of 2π, and a nudge of 0.06·sin d falls under half a quantum once sin d < 0.2. One system chose sparsity. The other found it in its arithmetic and had to measure it.

## 7. The relation channel as a coordinate: patches, modes, and the sphere

FourCastNet has three coordinate systems and moves between them in every layer. The first is the grid, 720 by 1440, one cell per quarter degree. The second is the patch grid, 90 by 180, after the 8 × 8 patch embedding. The third is the wavenumber grid, the same size as the patch grid but indexed by frequency in each direction. The Fourier step lives entirely in the third. The MLP acts on a single coefficient there and does not see its neighbours, which is why the shared block-diagonal weights are sufficient: mixing across positions has already happened in the transform.

Phical's channel is the coordinate that plays the wavenumber's role. A word's 64 channels are the modes of its function, and the operator's learned weights act on those modes. The facts file records that the sphere tie interprets 64 = 8² channels as spherical harmonics Y_l^m with l < 8, one complex weight per degree, and that this costs 1,192 parameters against the circle's 8,360. This is the same move FourCastNet's successors made when they replaced the planar DFT with a spherical harmonic transform. The 2022 paper does not do that. It treats the planet as a periodic rectangle, and the paper does not discuss the distortion this introduces at the poles beyond the latitude weighting in the metrics. But the profile notes that later versions moved to spherical harmonics to respect the geometry of the globe, and phinet has that tie already, at a size where it can be measured in ninety seconds.

The measurement, cited exactly, is that on Austen at 3 epochs with hinge loss the sphere tie scores 12.8× valid and 13.1× test with hit@10 0.182 in 89 s, where the circle scores 14.7× and 14.6×, 8,360 params, in 142 s. On a synthetic filter-by-degree task, 200 items and 40 epochs, the circle reaches MRR 0.662 against identity 0.646 and the sphere reaches 1.000. So the sphere tie is a weaker general operator at a seventh of the parameters and a perfect one when the task is genuinely spherical. That is the pattern the FourCastNet lineage found: the sphere is right when the domain is a sphere.

The honest gap is that phinet's sphere is a naming convention over 64 channels and not a geometry over a globe. The tie says: treat channel index (l, m) as a harmonic and share weights by l. FourCastNet's successors say: the data lives on S², transform it with the right basis. The second claim is about the world. The first is about parameter sharing. They rhyme. They are not the same thing.

The coordinate view also gives a reading of the AFNO's adaptivity. The MLP with soft threshold is a per-mode gate that chooses, from the content of the coefficient, whether that mode carries the forecast. Phinet has 64 hexagram gates. Whether they could be made adaptive in that sense is not measured.

## 8. The operator as X ⇒ Y under c: one step, then composition

FourCastNet's operator has a condition that is a number: Δt = 6 h. Every application of the model is X(k) ⇒ X(k+1) under the condition that six hours have passed. The condition never changes. It is fixed by the sub-sampling of ERA5 before training starts. This is what makes composition the right test. If the condition is always the same, the only way to reach a week is to apply the step twenty-eight times, and the only way the step can be good at a week is to be good on its own output.

Phient's operator has a condition too, and the facts file gives the relation channels it runs on. In the quest, the condition is the kind of relation: is-a, part-of, near, used-for, unlike. The oracle has 256 words in families of 8, 1,280 rows, 293 target words, and chance MRR 0.021. The operator's job is: given word X and kind c, produce the function that Y has. The measured result of the shipped trainer is 0.082 MRR. The attraction-only trainer with dithered rounding reaches 0.594 MRR, hit@1 34.8%, hit@5 97.4%, with within-kind MRR 0.747. The wrong-channel reading is 0.015 and the offset reading 0.005, which is to say both fall below chance. That last pair is the control that matters here. It says the operator's output lives in the channel of its condition and nowhere else. FourCastNet has no analogue because it has one condition.

Composition is where Phient has one number and it is unflattering. In the quest, held-out at 20%, one hop scores 0.011, below chance. Two hops score 0.095 for is-a and 0.093 for used-for, and part-of, near and unlike are at chance. Two hops beat one, and we have no explanation. FourCastNet's two-step fine-tuning is a training method and this is a test, so the two are not the same measurement.

The decoding measurements are Phient's other composition test. Over 40 held-out prompts of 30 tokens, the operator's nearest-k decoding reaches a Kneser-Ney perplexity of 66.9, down from 97.2, against 13.3 for the trigram sampler; the union sits at 66.8. Shared 4-grams across prompts are 25.7% for the sampler and 8.7% for the operator. So the operator composed thirty deep is less fluent under a trigram judge and repeats itself far less.

The clean statement is this. FourCastNet's operator is X ⇒ Y under a fixed c, composed L times. Phient's operators are X ⇒ Y under a c that is itself a coordinate, composed at most twice in any measurement. The paper's discipline of training the step on its own output has not been tried on a word operator in gemphi.

## 9. Where the sphere, the graph and the tensor land

The paper does not use a sphere, a graph, or a tensor decomposition, and it is worth saying so before placing its parts in the family. It uses a regular grid, a DFT, and a transformer backbone. The sphere arrives in its successors. The graph is absent; the earlier Graph Kernel Network and multipole papers in this series are the ones where the kernel is a graph, and FourCastNet replaces all of that with the transform. The tensor is only the sample, 721 × 1440 × 20, and nothing is decomposed.

So the placement is by function. The 720 × 1440 × 20 sample lands in phinet's Tensor [b,w,s]: batch, word, spectral. The correspondence is rough. Phinet's s axis is a one-dimensional closed domain of 64 complex entries. The paper's model needs a second spatial dimension. Phinet does not have one.

The DFT and inverse land in phinet's unitary FFT, which passes 62 of 62 parity tests against a PyTorch reference. The AFNO's per-mode MLP lands in Fno's learned complex weights, with one difference already named: phinet's weights are per mode and fixed once trained, where the AFNO's are shared across modes and nonlinear. The soft threshold has no designed counterpart. The byte deadband is a threshold the engine imposes at half a quantum whether or not the operator wanted it.

The precipitation model lands in phiano. It is a second operator that reads the first's output and produces a derived quantity that is not part of the state. In phiano, persona attribution reads a word function and produces a writer: three writers, held-out 4-sentence paragraphs, 71% overall against 33% chance, 74/65/86 by writer. That is a diagnostic model over the state, not a step of the state. It is FourCastNet's precipitation head in shape, though not in what it reads.

The ensemble lands nowhere yet. Phient has no measurement in which the same prompt is perturbed a hundred times and the outputs spread. Dialogue judged as its persona in 44% of 18 replies by the sector reader and 72% by the Kneser-Ney panel is a single-run number.

The helix lands beside the paper as a data structure and not a model. The lexicon as a DNA-coded tree, 64 codons as base-4 ACGT of channel, with hexagram, sphere degree and order, stores 18,496 words in 1.9 s and 75 MB with a byte-exact round trip. FourCastNet's stored object is a checkpoint, and a checkpoint does not need to round-trip a lexicon.

The engine note that phinet is allocation and bandwidth-bound at these sizes, with a 1 MB op taking 2–5 ms, is the one measurement that speaks to the paper's Section 4. Phinet is at a size where the arithmetic is not the cost at all. That agrees with the paper's premise and says nothing about whether phinet would stay cheap at FourCastNet's scale. It has not been run there.

## 10. The agent loop: the quest and the forecast

FourCastNet's inference loop is the simplest agent loop there is. Take a state, apply the operator, take the output as the new state, repeat. There is no decision in the loop, no branching, no reading of the environment between steps. That is what makes it a forecast rather than a control. The paper's Section 6 names the missing piece: data assimilation, the step in which new observations are folded into the running state. Without it the loop is open. With it the loop would be closed, and the model would be something closer to what phiecs calls a quest.

The quest in phiecs is an entity-component world in which a word is asked, under a kind, for what it relates to, and the answer is scored against an oracle. It is a loop with a read: the world says what is true, and the trainer moves the word. The number that matters here is the split between the shipped trainer at 0.082 MRR and the attraction-only trainer with dithered rounding at 0.594. The difference is in what the loop does with the answer. Attraction only, with dither to escape the byte deadband, lands seven times higher. The paper's fine-tuning story has the same shape: the change that made composition stable was in the training loop, and the architecture was left alone.

Dither has a cost the facts file states exactly. It moves corpus attribution from 71% to 65%, so it ships off by default. FourCastNet's soft threshold has a cost too, reported in a different place: the model underestimates the extreme percentiles of precipitation by about 35% where IFS underestimates by about 15%. A model that zeroes small modes on purpose will be smooth where the world is sharp. Whether phinet's deadband does the same to rare relations has not been measured.

The helix closes the loop in a different sense. It is where a quest's result is written down so that the next quest reads it. The paper has no equivalent because a forecast is emitted and discarded. What it stores is the operator's checkpoint, and its claim that one checkpoint served Hurricane Michael, Typhoon Mangkhut and an April atmospheric river is a claim that the operator generalises across the states it moves. Phient's version of that claim is the wrong-channel and offset readings, 0.015 and 0.005, which say the quest operator does not answer in a channel it was not asked in. It is a small and specific kind of generalisation. It is measured.

One loop the paper has and Phient does not is the ensemble. Perturb the initial condition with Gaussian noise at σ = 0.3, run 100 members, and read the spread. Nothing in gemphi runs the same prompt many times under noise and reports agreement.

## 11. What Phient has already measured that bears on it

Every number here is from the facts file and is cited as it stands there. None of it was measured for the purpose of this paper.

| Phient measurement | bears on |
|---|---|
| phinet vs PyTorch reference: 62/62 parity tests | the transform is correct before any operator claim is made |
| Austen operator, pool 1,228 words seen 20+ times, 204,592 examples, chance MRR 0.0063: identity 7.9×; hinge 8,360 params 14.7×/14.6×; softmax β=8 18.0×/18.8×; wide+deep 50,016 params 27.6× at 3 epochs, 30.7×/30.3× at 8, hit@10 0.348 | the one-step operator X ⇒ Y on a word function |
| Sphere tie 1,192 params: 12.8×/13.1×, hit@10 0.182, 89 s, vs circle 14.7×/14.6×, 8,360 params, 142 s; synthetic degree filter: circle 0.662 (identity 0.646), sphere 1.000 | the spherical basis the paper's successors adopted |
| Decoding, 40 prompts × 30 tokens: sampler KN 13.3, operator nearest-k 66.9 (was 97.2), union 66.8; shared 4-grams sampler 25.7%, operator 8.7% | composition of the operator thirty deep |
| Quest, chance 0.021: shipped 0.082; attraction-only + dither 0.594, hit@1 34.8%, hit@5 97.4%, within-kind 0.747; wrong channel 0.015; offset 0.005; held-out one hop 0.011, two hops is-a 0.095, used-for 0.093, others ≈ chance | the condition as a coordinate; the control that the answer stays in its channel |
| Byte deadband: 256 quanta of 2π; 0.06·sin d under half a quantum once sin d < 0.2; nearest rounding froze pairs 8 quanta short; dither costs attribution 71% → 65% | the paper's soft threshold and its cost on extremes |
| Persona attribution 71% (74/65/86), chance 33%; dialogue 44% / 72%, chance 33% | a diagnostic head over the state, like the precipitation model |
| Helix: 18,496 words in 1.9 s, 75 MB, byte-exact | storage; no counterpart in the paper |
| Engine bandwidth-bound, 1 MB op 2–5 ms | the paper's premise that the operator is cheap |

Three readings of this table are safe. First, phinet's operator is a Fourier operator on a function space and it has been measured against chance on held-out data with a stated pool and chance rate. That is the minimum a reader of FourCastNet would ask. Second, the sphere tie's two results are the closest Phient comes to the paper's lineage: a spherical basis that is worse in general and perfect on a spherical task, at a seventh of the parameters. Third, the quest's channel controls are a negative result the paper does not report: what the operator does in a coordinate it was not conditioned on.

Three readings are not safe. Phient's ranking multiples are not ACC and cannot be compared to forecast skill. The decoding perplexities are against a trigram judge and say nothing about drift in the operator's own space. The quest's held-out numbers are on a synthetic oracle of 256 words, and the facts file says plainly that there has been no Phi-4 run.

Not measured, and stated as such: any adaptive per-mode gate; any two-step training of the word operator on its own output; any ensemble under input noise; any spherical projection of a word's relations onto a geometric sphere; anything at all involving weather.

## 12. The experiment that would test the paper's idea inside phinet

The paper's idea, stripped of weather, is that a one-step operator becomes a good many-step operator when it is fine-tuned on its own output. The test inside phinet is to do that to the Austen operator and measure composition.

The setup is the one already measured. Austen, pool of 1,228 words seen 20+ times, 204,592 training examples, chance MRR 0.0063. Take the wide and deep softmax operator at 50,016 params, which reaches 30.7×/30.3× at 8 epochs, as the pre-trained one-step model. That is the paper's phase one.

Phase two is the paper's fine-tuning, transcribed. For each training pair (X, Y) where a third word Z follows Y in the text, apply the operator to X to get Ŷ, apply the operator again to Ŷ to get Ẑ, and train on the sum of the losses of Ŷ against Y and Ẑ against Z. Run this for a fixed small number of epochs at a lower learning rate, as the paper does, 50 epochs at a fifth of the pre-training rate being its ratio. The output is an operator that has seen its own product.

The measurement is composition at depth. On held-out text, apply the operator k times from a seed word and rank the true k-th following word against the pool. Report MRR as a multiple of chance and hit@10 at k = 1, 2, 3, 5, for both operators. The paper's prediction is that k = 1 changes little and k ≥ 2 improves. The quest's odd held-out result, two hops at 0.095 against one hop at 0.011, is a reason to run this on the corpus operator, where the one-step is already strong.

The control is the same operator trained for the same additional epochs at the same lower learning rate on one-step pairs only. Without it, any gain at k ≥ 2 could be more training rather than two-step training. A second control is the identity, which scores 7.9× at k = 1 and composes to itself, so it gives the floor.

Two secondary readings would say whether the analogy holds beyond the headline. The first is drift in the operator's own space: the mean angular distance, per channel, between Ŷ after k steps and the true word's function, as a function of k. The paper measures drift as ACC against lead time; this is its phinet form. The second is the byte deadband under composition: how many channels fall under half a quantum at each step. The facts file records that nearest rounding froze every pair 8 quanta short at a single step. Composition may make that worse.

This experiment cannot test the ensemble claim. That needs a noise scale for a word's 64 phasors, and none is measured. It is a second experiment. Cost is not a question: the facts file gives 142 s for the circle operator at 3 epochs on hinge, so fifty epochs of two-step training is under an hour by that rate.

## 13. Where we disagree, or where the analogy breaks

The analogy breaks in four places and we say each.

First, the domain. FourCastNet's function lives on a two-dimensional periodic grid of 16,200 patches, each carrying 768 values, and the transform runs over the two spatial dimensions. Phical's word lives on a one-dimensional closed circle of 64 channels. A word has no second dimension. The paper's efficiency argument, that global mixing in two dimensions is cheap in the frequency domain, has nothing to do at Phient's size, where the engine is bandwidth-bound at a megabyte. The paper's cost argument begins where phinet's ends.

Second, the condition. FourCastNet's condition is one number and never changes. Phient's condition is a coordinate, the relation channel, and the quest's whole result is that the operator answers in that coordinate and not in others. The paper has no condition to vary and so no wrong-channel control. This is a place where Phient asks a question the paper does not, and we do not claim the paper should have.

Third, the count. Every grid cell in ERA5 is filled and equally trusted. Every relation in Phical carries a count, and the count is a claim about how much to trust the angle. FourCastNet has no mechanism for a cell that was seen twice. Phical's arithmetic is built around it. The paper's smoothing of extremes, 35% under on precipitation percentiles, is the failure that a count would be built to resist, and we cannot say a count would resist it because no one has tried.

Fourth, the sphere. The paper treats the earth as a periodic rectangle. Its successors did not, and phinet's sphere tie names channels as harmonics. But phinet's sphere is a parameter-sharing scheme over an index and the successors' sphere is a transform on S². The synthetic result, sphere at 1.000 against circle at 0.662, is a result about sharing weights by degree on a task built to reward that. It is not a result about geometry. Reading it as one would be the kind of claim the brief tells us not to make.

Where we disagree with the paper is smaller and it is about a word. The abstract says the model outperforms IFS on precipitation while Section 3.6 reports that it underestimates extreme precipitation percentiles by about 35% against IFS's 15%. The first is about the field. The second is about the tail. A reader who takes the abstract without Section 3.6 will think the model is better at rain, and it is better at the mean of rain and worse at the worst of it. Phient's facts file has the same shape of trade-off in dither, which raises the quest's MRR from 0.082 to 0.594 and lowers attribution from 71% to 65%, and its response was to ship it off and state the cost in the same sentence. We would have preferred the abstract to carry the tail.

One thing we do not disagree with. The paper does not claim to replace numerical weather prediction. It claims to augment it, and it lists what it cannot do. That is the right posture for an operator that learned a map and not a law.

## 14. The person and the moment

The paper was submitted on 22 February 2022 by Jaideep Pathak, with Anima Anandkumar as the last author, which in this field is the senior one. The author list spans NVIDIA, Lawrence Berkeley National Laboratory, Rice, Caltech and, by the affiliations on the preprint, Michigan and Purdue. Two of the co-authors, Zongyi Li and Kamyar Azizzadenesheli, are authors of the original Fourier Neural Operator paper of 2020 that the profile describes as her group's idea: FFT, keep the low modes, multiply by learned complex weights, inverse FFT, add a local linear path, nonlinearity, repeat.

The moment is the one the profile sets out. She had been at NVIDIA since 2018 as senior director of machine-learning research and at Caltech since the same year as Bren Professor, where she co-founded the AI4Science initiative. The FNO paper had been profiled by Quanta in 2021. FourCastNet is the first time the operator idea was put on a problem that the world already measured at operational scale, against a reference model, IFS, that a national centre runs every day. The profile calls it the first AI weather model to forecast at the resolution of the world's numerical weather centres.

Her stated view of where this goes, from the profile, is the sentence "We have foundation models for language, not for physics." The argument behind it is that a transformer over a few hundred grid points per dimension is a context of hundreds of billions to a trillion tokens, and that an operator that learns in function space treats scale as what it is. FourCastNet is the paper in which that argument was first made with a global grid. The AFNO's replacement of attention by a transform is the argument in code.

The profile also records her method, in her words: "All of the things that work with deep learning, let's take them, but make them a bit more principled." FourCastNet is that method applied literally. It takes a vision transformer, which works, and replaces the part that does not scale with a transform that has a reason.

What came after, from the profile only: later versions moved to spherical harmonics, FourCastNet 3 holds stable over long global rollouts, the model is open source, and the same machinery reached plasma-disruption prediction for fusion, drone landing in turbulence, and design problems in medicine. The recognition that followed includes the Time100 Impact Award in 2025 for using AI to accelerate scientific discovery and, on 16 March 2026, appointment to the United Nations Scientific Advisory Board.

Nothing here is her view of Phient. The profile is explicit that a partnership is Phient's aim and not her statement, and that it is something to earn with a result she would recognise, measured the way she measures. This paper's Section 12 is an attempt to name such a result. It has not been run.

## 15. What remains

What remains is mostly the experiment in Section 12, and it should be run before this paper is cited as anything more than a reading.

Two-step fine-tuning of the Austen operator, with the one-step-more-epochs control, is the direct test. It costs under an hour by the facts file's timing and it would give Phient its first measured composition curve for the corpus operator, at k = 1, 2, 3, 5, which is the form of curve the paper reports for weather. Until it exists, every sentence above about composition rests on the quest's two numbers, 0.011 at one hop and 0.095 at two, and those are on a synthetic oracle.

The adaptive gate is the second thing. The AFNO's contribution over the FNO is that the per-mode map is shared, nonlinear and thresholded, so that which modes carry the output is chosen by content. Phinet's Fno has fixed complex weights per mode and 64 hexagram gates whose adaptivity is not measured. An AFNO-style layer over the 64 channels is small to write. Whether it beats the fixed weights on Austen at the same parameter count is not known.

The ensemble is the third thing and it needs a noise scale first. The paper perturbs initial conditions at σ = 0.3 in standardised units. A word function's natural unit is the quantum, 2π/256, and the facts file's deadband result says a nudge under half a quantum does nothing. So the smallest meaningful perturbation is known. What a hundred perturbed decodes of the same prompt agree on has not been measured.

The sphere is the fourth thing and the one where the facts file already says what is unrun: the sphere tie at softmax and at wide and deep. The circle at wide and deep reaches 30.7×. The sphere at hinge reaches 12.8×. Whether the gap closes at the larger operator is a single run.

Weather is not on the list. There is no weather work in gemphi, and this paper has made no case that there should be. What the paper offers Phient is a discipline: train the step, then train it on its own output, measure it composed, report the tail as well as the mean, and say what the model cannot do. That can be adopted without a single grid point of the atmosphere.

The last thing is the one the profile names. A result she would recognise is an operator over a function space she has not seen, measured the way she measures. The measurements in Section 11 are its first ones. The composition curve of Section 12 would let Phient's word operator be read beside FourCastNet's on the same axis, which is skill against how many times the step has been applied.

## 16. Sources

Pathak, J., Subramanian, S., Harrington, P., Raja, S., Chattopadhyay, A., Mardani, M., Kurth, T., Hall, D., Li, Z., Azizzadenesheli, K., Hassanzadeh, P., Kashinath, K., Anandkumar, A. FourCastNet: A Global Data-driven High-resolution Weather Model using Adaptive Fourier Neural Operators. arXiv:2202.11214, v1 submitted 22 February 2022. Abstract page: https://arxiv.org/abs/2202.11214. Full text as fetched: https://arxiv.org/pdf/2202.11214v1. Subjects listed on arXiv: physics.ao-ph, cs.LG. The arXiv id given in the brief was correct and is the one used.

Facts about the paper in Sections 2 to 5 were taken from the abstract page and the v1 PDF: the abstract, Table 1 (variables), Table 2 (node-hours and energy against IFS), Table 3 (hyperparameters), Sections 2.1 to 2.4, 3.1 to 3.6, 4, 5 and 6, and the appendix definitions of latitude-weighted ACC and RMSE. Where the text gives no number, for instance the ACC and RMSE curves against IFS in Figure 6, we have said so and not read values off the figures. No parameter count is stated in the paper's text and none is given here.

Every number about Phient is from PHIENT_FACTS.md in this folder, cited exactly. Every fact about Anima Anandkumar in Section 14 is from 17_ANIMA_ANANDKUMAR_PROFILE.md in the parent folder, whose own sources are the Caltech News release of 16 March 2026, the Latent Space interview, the Wikipedia article, the Blavatnik Awards profile and the Caltech CMS faculty page. The FNO paper (Li, Kovachki, Azizzadenesheli, Liu, Bhattacharya, Stuart, Anandkumar, 2020) and the AFNO paper (Guibas et al., 2022) are cited as FourCastNet cites them; neither was fetched for this paper, and no claim here about either goes beyond what FourCastNet says of them. The DLWP comparison is as FourCastNet describes Weyn et al. (2020); that paper was not fetched.

Nothing in this document is a measurement of weather. Phient has done no weather work, and the paper has been read for its operator and its method of measuring composition, which is where it bears on a word.
