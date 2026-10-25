# The shape is the condition: an operator that reads geometry before it reads the field

*Phient interpreted papers, 15 of 16 · 2026-08-10 · on: "Geometry-Informed Neural Operator for Large-Scale 3D PDEs" (Li, Kovachki, Choy, Li, Kossaifi, Otta, Nabian, Stadler, Hundt, Azizzadenesheli, Anandkumar, 2023, NeurIPS 2023; arXiv:2309.00583)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the fifteenth of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. The paper under the glass is the geometry-informed neural operator, GINO, from NeurIPS 2023. Its problem is a practical one. The Fourier neural operator wants a regular grid, because the fast Fourier transform wants a regular grid. A car does not sit on a regular grid. Its surface is a mesh of a few thousand or a hundred thousand points, and each car in a design study has a different mesh. GINO answers with three parts in a row: a graph neural operator that carries the irregular mesh onto a regular latent box, a Fourier neural operator that does the global work on that box, and a second graph operator that carries the answer back to whatever points you ask for. The shape itself enters as a signed distance function sampled on the same box.

We read it for one reason. In Phical a word is a function on a fixed set of channels, and the operator that moves one word toward another is a Fourier operator in phinet. GINO says something about the fixed set. It says that a global Fourier operator can be indifferent to the sampling of its inputs and outputs, if a local kernel operator stands on each side of it and the kernel is defined by distance in the original space rather than by a fixed count of neighbours. In our terms, the shape of the input is not the field; it is the condition under which the field is read. That is the paradigm sentence with a signed distance function in the place of c.

Sections 2 to 5 state what the paper says and the numbers it reports. Sections 6 to 10 place the encoder, the latent grid, the decoder and the signed distance function inside the Phient family. Section 11 lists what Phient has measured that bears on it, all of it from the facts file and none of it a fluid. Section 12 names the experiment that would test GINO's idea inside phinet, with its control. Section 13 says where the analogy breaks. Section 14 is about the person and the moment. Section 15 is what remains.

One limit is stated up front. Phient has done no work on any PDE, no mesh, no car, no drag coefficient. Nothing in GINO's pipeline exists in gemphi. Where this page says an idea from the paper would apply to phinet, it is saying what the experiment would be, not what its result is.

## 2. What the paper claims, in its own terms

The abstract says the paper proposes "the geometry-informed neural operator (GINO), a highly efficient approach for learning the solution operator of large-scale partial differential equations with varying geometries." It says GINO "uses a signed distance function (SDF) representation of the input shape and neural operators based on graph and Fourier architectures to learn the solution operator." The graph part is there to "transform irregular grids into regular latent grids on which FNO can be efficiently applied."

The claim of principle is discretization convergence. The paper says the trained model "can be applied to arbitrary discretizations of the continuous domain and it converges to the continuum operator as the discretization is refined." The paper contrasts this with graph neural networks that fix a number of nearest neighbours: GINO's graph operators build the graph from "points within an epsilon ball in the original space", so refining the mesh adds points to the ball rather than shrinking the ball.

The claims of result are three. First, on 3D aerodynamics with Reynolds numbers up to five million, the model predicts pressure on car surfaces after being trained on five hundred samples. Second, in a cost-accuracy comparison the paper reports "a 26,000× speed-up compared to optimized GPU-based computational fluid dynamics (CFD) simulators on computing the drag coefficient." The appendix pins the condition: "At a 3% relative error, we find the speed-up from our model which includes drag in the loss to be 26,000×." Third, on new combinations of geometry and inlet velocity the paper claims "one-fourth reduction in error rate compared to deep neural network approaches."

The paper's table of complexity puts its own row last. A graph network is local with cost O(N·degree). A convolutional network is local with cost O(N). A U-Net is global at O(N) but needs a regular grid. A transformer is global at O(N²). A graph neural operator with radius r costs O(N·degree). An FNO is global at O(N log N) but needs the grid. GINO is global, takes an irregular grid, is discretization convergent, and costs O(N log N + N·degree).

The paper is also plain about what it has not done. It says "the trained surrogate model is limited to a specific category of shapes", that "the quality of the model depends on the quality of the training dataset", and that "for CFD with more complex shapes, it is not easy to obtain a large training dataset." It leaves the regularity of the defined operator and parallelism in the encoder for later work. It does not claim to replace the solver; it claims to make a design loop cheap once the solver has produced a few hundred examples.

## 3. The mathematics, plainly

There are three maps and one auxiliary field. Write the input mesh as points y_i in three dimensions, each carrying whatever is known there. The encoder produces a function on a regular grid x^grid. The paper writes it as a sum over a ball: v₀(x^grid) is approximately the sum over y_i in B_r(x^grid) of κ(x^grid, y_i) μ(y_i), where κ is a learned kernel and μ is a weight that stands for the measure of the mesh around y_i. The sum is a Riemann sum for an integral. If the mesh is refined, more points fall in the ball, each with a smaller weight, and the sum approaches the integral. This is the whole of the convergence argument. The kernel is a small network that takes the two positions, with Fourier features so that a plain coordinate difference becomes a set of sines and cosines before the network sees it.

The middle map is an FNO on the box. The grid function from the encoder is concatenated with the signed distance function of the shape, sampled at the same grid points, so that every latent cell knows how far it is from the surface and on which side. The FNO layer is the standard one: a fast Fourier transform of the grid function, multiplication of a retained set of modes by learned complex coefficients, an inverse transform, a local linear term, a nonlinearity. The paper writes the retained modes as an index set I of size α and learns the coefficients c_γ directly. The inlet velocity, a scalar, is not concatenated. It is embedded with Fourier features and fed to an adaptive instance normalisation that outputs a scale and a shift for the latent function, so that the same operator is bent by the velocity rather than given the velocity as a channel.

The decoder is the encoder run backwards in kind. For any output point x^out, the answer is a sum over grid points within a ball of x^out of κ(x^out, y^grid_j) v_l(y^grid_j) μ(y^grid_j). The output points can be the mesh vertices or any other set. They are batched so that memory does not grow with their number.

The paper reports a practical detail of the ball search. For ten thousand points a hash-based neighbour search needs 6 GB of GPU memory where the pairwise method needs 24 GB. The ball radius is a hyperparameter, and the paper's ablations vary it together with the latent resolution so that the number of grid cells inside a ball stays roughly constant. Nothing in the mathematics is new in isolation. The graph operator is the group's 2020 graph kernel network; the FNO is their 2020 Fourier operator. What is new is the order, and the choice to make the grid a latent object the data never sees.

## 4. Its experiments and numbers as reported

Two datasets. The Shape-Net car set has 611 vehicle shapes, 500 for training and 111 for validation, with about 3.7 thousand mesh points on each surface, a fixed inlet velocity of 20 m/s, a stated Reynolds number of about 5 × 10⁶, and pressure from a RANS solver with a k-ε model taking about 50 minutes per simulation. The Ahmed-body set has 551 shapes, 500 for training and 51 for validation, generated from six shape parameters (length, width, height, ground clearance, slant angle, fillet radius) plus an inlet velocity from 10 to 70 m/s, so Reynolds numbers from 4.35 × 10⁵ to 6.82 × 10⁶. Each Ahmed simulation has 7.2 million mesh points with about 100 thousand on the surface, uses an SST k-ω steady-state solve, and takes 7 to 19 hours on two V100 GPUs with 16 CPU cores. The targets are surface pressure and wall shear stress. The loss is a relative L² error, in some runs with a drag-coefficient term added. Training runs 100 epochs with Adam and a step schedule.

The headline tables, test error in relative L²:

| dataset | model | train | test |
|---|---|---|---|
| Shape-Net car (3.7k points) | GNO | 18.16% | 18.77% |
| | GeoFNO (sphere) | 10.79% | 15.85% |
| | UNet (interp) | 12.48% | 12.83% |
| | FNO (interp) | 9.65% | 9.42% |
| | GINO (encoder-decoder) | 7.95% | 9.47% |
| | GINO (decoder) | 6.37% | 7.12% |
| Ahmed body (100k points) | MeshGraphNet | 9.08% | 13.88% |
| | UNet (interp) | 9.93% | 11.16% |
| | FNO (interp) | 12.97% | 12.59% |
| | GINO (encoder-decoder) | 9.36% | 9.01% |
| | GINO (decoder) | 9.34% | 8.31% |

The "decoder" variant drops the learned encoder and feeds the signed distance function alone into the FNO, using the graph operator only on the way out. It wins on both sets. The ablation on latent resolution at fixed radius 0.055 goes 13.59% test at 32³, 10.20% at 48³, 8.47% at 64³, 7.87% at 80³. When the radius is scaled with resolution the gap closes: 10.10% at 32 with radius 0.110, 9.17% at 48 with 0.073, 7.89% at 80 with 0.044. A radius ablation at 0.025 and 0.035 gives errors from about 12.7% to 13.1% at the smaller radius and 9.4% to 10.3% at the larger.

The super-resolution table trains on meshes subsampled by 2, 4, 6 and 8 and tests on the full mesh never seen in training. GINO's encoder-decoder gives 8.8%, 9.4%, 9.4%, 9.7%. The interpolated UNet gives 16.5%, 13.8%, 13.9%, 15.6%. The interpolated FNO gives 14.2%, 14.1%, 13.3%, 11.5%. One trained sample is evaluated in 0.1 seconds on a V100, counting the SDF computation and the drag calculation, which the paper puts against the GPU-parallel OpenFOAM solve to reach the 26,000× figure at 3% relative error on drag.

## 5. The one idea, restated

The idea is that a global operator need not see the geometry as a grid. It needs to see the geometry as a function on a grid, and it needs local operators on either side whose graphs are defined by physical distance rather than by index. Once that is done, the mesh becomes a way of sampling, not a part of the model. The same trained weights answer at 3.7 thousand points or 100 thousand, at one-eighth of the mesh or the whole of it, and the answer changes by a percentage point rather than a factor.

There is a second idea folded inside, and the paper's own ablation exposes it. The best variant is the one that throws the learned encoder away and gives the FNO only the signed distance function. The shape, encoded as a distance field, is enough for the global operator to work from. The mesh-borne data is needed only at the output, to say where the answer is wanted. In the paper's problem the input is the shape and the output is a field on the shape, so this makes sense: the shape is the whole of the input, and a distance function is a complete and smooth way to write it.

Restated for Phient, the idea is this. Take a fixed latent box. Put the condition on the box as a smooth field. Run the Fourier operator there. Read the answer off at whatever points the question asks. Do not let the question's sampling into the operator. The operator is X ⇒ Y under c, where c is the distance field and the sampling of X and Y is nobody's business but the encoder's and decoder's.

A third idea is the practical one that the group keeps returning to. The saving is not in the solve but in the loop. Five hundred solves at 7 to 19 hours each buy a model that answers in 0.1 seconds, and the design study that follows runs at the model's speed. The paper says this in terms of drag. It is also, put plainly, the case for building a trainer once and a sampler often, which is how phiano is arranged.

## 6. A word as a function on the latent box

In Phical a word is 32–64 relations, each a complex phasor on a closed circle with a count. That is a function on a finite set of channels. In phinet the tensor that holds a batch of words is `[b,w,s]` complex, where the last axis is the channel axis the unitary FFT runs over. Nothing in this is a mesh. Every word has the same channels in the same order. From GINO's side, Phical has already made the choice GINO makes at its centre: a fixed regular latent grid on which the Fourier operator acts, with the same grid for every input.

What Phical lacks is the two sides of GINO. There is no encoder from an irregular set of observations onto the channels, and no decoder from the channels to an arbitrary set of query points. A word's relations are counted directly into channels by the trainer. The channels are the grid, and the data is born on the grid. This is the situation of the paper's interpolated FNO baseline, where the mesh data is resampled onto the box before the operator sees it, and in the paper that baseline is a few points worse than GINO on the car and three to four points worse on the Ahmed body.

The analogy sharpens if one asks what an irregular sample of a word would be. A word seen in a corpus is not seen on 64 channels at once. It is seen one context at a time: this neighbour, that distance, that side. Each occurrence is a point in the space of contexts, and the trainer's job is to sum those points into the channel function. That summation is the encoder. In the shipped trainer it is a fixed rule: the relation kind picks the channel, and the count and the phasor accumulate. GINO would say the rule could be a kernel, defined by distance in the context space, with a Riemann weight for how dense the observations are around each context. Phical's helix already gives each channel a fixed coordinate, a codon in base-4 ACGT and a position on the B-DNA helix at 10.5 codons per turn and 0.34 nm rise. Those coordinates are the x^grid of GINO's encoder. What is missing is a y_i, a coordinate for the raw observation before it lands on a channel.

So the first placement is this. A word in phinet is GINO's latent function v₀, already on the box. The channel geometry in the helix is the box's coordinate system. The encoder, in Phient, is the trainer's counting rule, and it has never been learned.

## 7. The relation channel as a coordinate, and the ball around it

GINO's graph operators do their sum over a ball in physical space. Whether a mesh point contributes to a grid cell is decided by distance, and the radius is a hyperparameter the paper sweeps from 0.025 to 0.110 in a domain normalised to [-1, 1]. The paper's ablation is direct about what the radius does: at small radius the errors sit near 13%, and at a radius that keeps a constant number of cells in each ball the error tracks the resolution.

Phical's channels are on a closed circle with 256 quanta of 2π per byte. Two channels are near if their positions on the circle are near, and the helix gives each channel a second neighbourhood, along the strand. There is no ball in the shipped code. Each relation kind writes to its own channel, and a wrong channel reads as noise. The quest measurement in the facts file makes that concrete: within-kind MRR 0.747, wrong channel 0.015, offset reading 0.005, against chance 0.021. Reading a channel other than the one written to returns nothing. That is a radius of zero.

A radius of zero is the paper's k-nearest-neighbour case pushed to k = 1, and the paper's argument against fixed-k is that it does not converge when the sampling is refined. In Phical's terms: if the relation kinds were split finer, so that "is-a" became five sub-kinds, a zero-radius reader would find each sub-channel five times emptier and the ranking would degrade. A reader with a ball would sum the five and see the same count as before. The facts file has no measurement of this. It is stated here as what GINO predicts, not as what phinet does.

The sphere tie gives a second coordinate system in which a ball is natural. In `phinet::Tie::Sphere` the 64 channels are Y_l^m with l < 8, and there is one complex weight per degree l, 1,192 parameters against the circle's 8,360. A ball in that space is a band of degrees, and a low-pass in l is exactly the kind of smoothing GINO's encoder does by summing a ball with a smooth kernel. On the synthetic degree-filter task (200 items, 40 epochs) the sphere reached MRR 1.000 where the circle reached 0.662, barely above identity at 0.646. On Austen the sphere at 3 epochs gave 12.8× valid and 13.1× test against the circle's 14.7× and 14.6×, at a quarter of the time (89 s against 142 s). That is the closest Phient has come to GINO's finding that the latent geometry, not the sampling, sets the error.

## 8. The operator as X ⇒ Y under c, with c a distance field

GINO's central move is to write the condition as a field on the box. The shape is not a list of vertices handed to the network. It is a signed distance function, sampled at every latent cell, and concatenated with the encoded data. The inlet velocity is handled differently again: it becomes a scale and shift on the latent function through adaptive instance normalisation. Two conditions, two mechanisms, and in the ablation the shape-as-field alone is enough to beat every baseline.

Phical's operator is X ⇒ Y under c. In phinet the operator is `Fno` with `FnoConfig { space, width, layers, modes, margin, lr }`, trained on Austen so that one word's function moves toward the next, and its best measured result is 30.7× valid and 30.3× test against chance MRR 0.0063 at 50,016 parameters and 8 epochs. In that experiment c is implicit. The condition is the corpus and the ordering; it is not a field on the channels.

GINO says c can be made a field. There are two candidates already in gemphi. The first is the relation kind, which the quest oracle names: is-a, part-of, near, used-for, unlike. In the shipped design the kind is a channel address. GINO would make it a smooth function on all 64 channels, high near the kind's home and falling off with distance on the circle, concatenated with the word before the operator. Then a single operator would serve all five kinds, bent by c, instead of five channel regions each read alone. The second candidate is the persona. Persona attribution over Chesterton, Austen and Carroll reached 71% on held-out four-sentence paragraphs (74/65/86 against chance 33%), and dialogue replies were judged as their persona 44% by the sector reader and 72% by the Kneser-Ney panel. A persona in phiano is a Facet, and a Facet is a field on the channels, a mask of which relations this writer moves. That is already the shape of a distance function on the box. What has not been tried is feeding it to the operator as GINO feeds the SDF, rather than applying it after.

The velocity mechanism has a Phient reading too. A scalar condition that scales and shifts the latent function is a global phase and gain on the phasors. In a byte-quantised channel a global shift is a rotation of every phasor by the same number of quanta, and the deadband result in the facts file (a nudge of 0.06·sin d falls under half a quantum once sin d < 0.2) says such a rotation will be exact only if it clears a quantum. That is a constraint GINO does not have, because its latent is floating point.

## 9. The graph in phiecs, the tensor in phinet, the sphere in phifno

GINO's parts map onto the Phient family cleanly enough to be named one by one. The graph neural operator on the input side is a sum over neighbours within a radius, with a learned kernel and a measure weight. In gemphi the object that holds entities and their neighbourhoods is phiecs, the entity-component world, and the synthetic quest oracle lives there: 256 words in families of 8, 1,280 rows over five relation kinds, 293 target words. A phiecs world already knows which words are near which, and by which kind. That is the graph. What phiecs does not have is a kernel: the edge carries a kind and a count, not a learned function of the two endpoints' positions.

The FNO on the latent box is phinet's `Fno`. It has the unitary FFT, the retained modes, the learned complex weights, the local path and the `Tape` for autograd, and it has been checked against a PyTorch reference at 62/62 parity tests. GINO's latent grid is three-dimensional at 32³ to 88³ cells; phinet's is one-dimensional at 64 channels. The paper's resolution ablation, where test error falls from 13.59% at 32³ to 7.87% at 80³, is the kind of sweep phinet could run over channel count, and has not.

The sphere is phifno 1.0.0, the spherical operator, and `Tie::Sphere` in phinet. GINO does not use a sphere; its box is Cartesian. The group's later weather work moved to spherical harmonics for the globe, and Phient's sphere tie is the one place the family already runs a harmonic basis. The relevance to GINO is indirect: the paper's GeoFNO baseline, which maps the car onto a sphere before the FNO, is the worst of the Fourier variants on the car at 15.85% test, and the paper's lesson is that a smooth latent box with the shape as a field beats a clever coordinate change. That is a caution for Phient, whose sphere tie also loses to the circle on Austen (12.8× against 14.7×) while winning on the synthetic degree-filter task. Whether the sphere's win on structure survives on text is not settled by either paper.

The decoder is the piece with no home. In GINO the decoder reads the latent function off at arbitrary points. In phiano, generation reads a word's function and ranks the next word; decoding on 40 held-out prompts by 30 tokens gave the operator nearest-k a Kneser-Ney perplexity of 66.9 (was 97.2) against the trigram sampler's 13.3, with shared 4-grams across prompts at 8.7% for the operator against 25.7% for the sampler. That is a reader, not a decoder in GINO's sense. It reads the channels at their own positions. A GINO decoder would read the channels at a query position that is not a channel, and phiano has no such query.

## 10. Where the agent loop lands: the quest as a design study

GINO's economic argument is that a surrogate turns a design study from a queue of solves into a loop. Five hundred CFD runs are the price of admission; after that each candidate shape costs 0.1 seconds. The paper's Ahmed-body set is literally a design grid, six parameters on a lattice, with velocity as a seventh.

Phient's loop is the quest. In phiecs the quest is a synthetic oracle that poses relation questions and scores the trainer's answers, and the facts file records what the loop has reached: the shipped trainer at 0.082 MRR, and attraction-only with dithered rounding at 0.594 MRR, hit@1 34.8%, hit@5 97.4%, against chance 0.021. Held-out generalisation is thin: one hop 0.011, two hops is-a 0.095 and used-for 0.093, with part-of, near and unlike at about chance. The loop learns what it is shown and does not yet learn the rule.

Read against GINO, the quest is the training set and not yet the surrogate. GINO's five hundred samples are the analogue of the 1,280 oracle rows. GINO's held-out test, new geometry and velocity combinations at 8.31% error, is the analogue of the two-hop held-out at 0.095. The paper's claim of a one-fourth error reduction on unseen combinations is the claim Phient's quest cannot yet make about unseen hops. That is the honest comparison and it is not flattering. The paper's model generalises across the design lattice because the lattice is smooth and the SDF is smooth in the lattice parameters. The quest's oracle is discrete, families of 8 with five kinds, and there is no smooth coordinate along which two hops are near one hop.

The helix may be where such a coordinate comes from. The lexicon as a DNA-coded tree gives every channel a codon, a hexagram, a sphere degree and order, a position in three-dimensional space on the B-DNA helix, and a PhiTon wavelength, band and hex. It ships: 18,496 words in 1.9 seconds, 75 MB, byte-exact round trip. If a two-hop relation is a walk of two codons along the strand, then the strand is the smooth coordinate, and GINO's encoder over a ball would sum one-hop evidence into a two-hop channel by proximity. This is a design, not a result. The facts file records the helix as a storage format and a round trip, not as a coordinate used by any trainer.

The agent in GINO's loop is a human designer choosing the next shape. In phiano the agent is the quest ingest and the persona's dialogue. Neither has been closed into an optimisation loop that proposes the next question from the last answer. GINO's paper has no such loop either; it stops at the surrogate and says the loop is now affordable.

## 11. What Phient has already measured that bears on it

Every number here is from the facts file and was measured for another purpose. None of it is a PDE.

| Phient measurement | bears on GINO's |
|---|---|
| phinet vs PyTorch reference: 62/62 parity tests | the FNO block at the centre |
| Operator on Austen: identity 7.9× chance; hinge 8,360 params 14.7×/14.6×; softmax β=8 18.0×/18.8×; wide+deep 50,016 params 30.7×/30.3× at 8 epochs, hit@10 0.348 | a global operator learning X ⇒ Y on a fixed latent grid |
| Sphere tie 1,192 params: Austen 12.8×/13.1× at 89 s vs circle 14.7×/14.6× at 142 s; degree-filter task circle 0.662, sphere 1.000 | latent geometry choosing the error, as in the resolution and GeoFNO ablations |
| Quest: shipped 0.082 MRR; attraction-only + dither 0.594, hit@1 34.8%, hit@5 97.4%; within-kind 0.747; wrong channel 0.015; offset 0.005 | a radius-zero reader against GINO's ball |
| Held-out 20%: one hop 0.011; two hops is-a 0.095, used-for 0.093; others ≈ chance | generalisation to unseen combinations |
| Byte deadband: 256 quanta, nudge under half a quantum once sin d < 0.2; nearest rounding froze every pair 8 quanta short; dither costs attribution 71% → 65% | a quantised latent that GINO does not have |
| Helix: 18,496 words in 1.9 s, 75 MB, byte-exact | a coordinate system for the channels |
| Engine: allocation/bandwidth-bound, 1 MB op 2–5 ms | GINO's memory numbers, 6 GB vs 24 GB for the neighbour search |

Three of these deserve a sentence. The Austen operator is the one measurement in gemphi of the object at GINO's centre, an FNO learning a map between functions on a fixed grid, and its best figure is 30.7× chance. GINO reports relative L² error, not MRR, and the two are not convertible; what can be said is that both are held-out numbers against a named chance or baseline. The quest's wrong-channel result, 0.015 against chance 0.021, is Phient's measurement of what a zero-radius reader does, and it is the number that GINO's ball argument is about. The engine's bandwidth-bound finding is the same finding the paper makes when it reports the hash grid at a quarter of the pairwise memory: at these sizes the cost is moving data, not multiplying it.

What is not measured: no sweep of channel count, no learned encoder, no query at a non-channel position, no sphere at softmax or wide+deep, no Phi-4 run. The facts file says so and this page repeats it.

## 12. The experiment that would test the paper's idea inside phinet

The paper's idea, stripped to what phinet can hold, is that a ball-summing encoder in front of a fixed-grid Fourier operator lets the operator ignore how its inputs were sampled. The test is a resampling test, the same one the paper runs in its super-resolution table.

Setup. Take the Austen operator as it stands: pool of 1,228 words seen 20 or more times, 204,592 training examples, chance MRR 0.0063, the hinge configuration at 8,360 parameters so the run is cheap. Add one thing: a split of the training examples by context density. For each target word, subsample its contexts by 2, 4, 6 and 8 before the trainer counts them into channels, exactly as the paper subsamples its meshes. Train four operators, one per subsampling. Test every one on the full held-out set, which was counted at full density.

Treatment. Replace the trainer's counting rule with a ball encoder: each context lands not on one channel but on every channel within a radius on the circle, weighted by a smooth kernel of the circular distance and by a Riemann weight that is the inverse of how many contexts fell nearby. Sweep the radius over a few values in quanta.

Control. The shipped counting rule, radius zero, at every subsampling. The paper's prediction is that the control's MRR falls with subsampling and the treatment's holds roughly flat, as its interpolated FNO falls from 14.2% to 11.5% erratically while GINO holds at 8.8% to 9.7%. Report valid and test multiples of chance and hit@10, as the facts file reports them.

A second, smaller test isolates the SDF idea. Take the quest oracle in phiecs and its attraction-only trainer at 0.594 MRR. Instead of giving each relation kind its own channel region, give the operator one shared region and concatenate a kind field: a smooth bump on the circle centred on the kind's home. Control: the shipped channel-per-kind design. The question is whether within-kind MRR (0.747) and held-out two-hop MRR (0.095 for is-a) move, and in which direction. The paper's decoder-only ablation predicts the field alone carries the kind.

Both tests are cheap by the engine's own measurement, allocation-bound at 2 to 5 ms per megabyte op. Neither has been run. If they had, their numbers would be in the facts file and this section would cite them.

## 13. Where we disagree or where the analogy breaks

The analogy breaks in four places and each should be named.

First, GINO's latent is three-dimensional and continuous, and the shape it conditions on is a smooth function of a smooth boundary. Phical's latent is 64 channels on a circle, each a byte, and the byte deadband is measured: a nudge of 0.06·sin d falls under half a quantum once sin d is under 0.2, and nearest rounding froze every pair 8 quanta short. GINO's encoder sums many small contributions with Riemann weights, and in a byte latent many small contributions is exactly the case that freezes. Dither fixes the freeze and costs six points of attribution (71% to 65%), which is why it ships off. A ball encoder in phinet would inherit this cost or would have to be summed in floating point before quantisation. The paper has no such problem and says nothing that helps with it.

Second, GINO's convergence claim is about refining a mesh of a fixed continuous domain. A corpus is not a mesh of anything. Seeing a word in more contexts is not a finer sampling of a fixed function unless the function is defined as the limit of the counts, and that is a definition, not a theorem. The paper's discretization-convergence argument, that the Riemann sum tends to the integral, needs a measure on the domain. Phical's contexts have counts but no measure the trainer has ever been told about.

Third, GINO's best variant drops the encoder. The paper's data is the shape, and the shape is the SDF. Phical's data is not a shape; it is the relations themselves, and there is no known field that summarises a word's relations without being those relations. The decoder-only reading, that the condition alone suffices, does not transfer.

Fourth, the paper's costs are dominated by the solver, 7 to 19 hours per sample on two GPUs, so a model at 0.1 seconds is a different kind of thing. Phient's cost is the trainer over 204,592 examples, and the facts file records no timing that makes the sampler a different kind of thing from the trainer. The 26,000× number has no analogue and this page does not manufacture one.

Where we disagree, mildly: the paper's complexity table gives transformers O(N²) and calls them discretization convergent. Both are true in the paper's sense. But the paper's argument that a fixed-k graph is not convergent applies with equal force to a fixed-window attention, and the table does not say so. Phical's own reader is fixed-window in a stronger sense, one channel per kind, and this page counts that against Phient before it counts anything against the paper.

## 14. The person and the moment

The profile places the paper in the group's line from tensor methods to operators. The Fourier neural operator (Li, Kovachki, Azizzadenesheli, Liu, Bhattacharya, Stuart, Anandkumar, 2020) did its global kernel as a multiplication in the frequency domain and was resolution-invariant by construction. FourCastNet (February 2022) put an adaptive FNO on a 720 × 1440 global grid at 0.25°, and later versions moved to spherical harmonics for the globe. The GINO paper, submitted in September 2023 and published at NeurIPS 2023, is the group's answer to the objection that the FNO's regular grid rules out real geometry. It comes with the same discipline the profile quotes her stating as a method: "All of the things that work with deep learning, let's take them, but make them a bit more principled."

The first two authors are Zongyi Li and Nikola Kovachki, the same pair who led the FNO and the graph kernel network. Kamyar Azizzadenesheli and Anima Anandkumar close the list. Between them are NVIDIA researchers, which fits the profile's account of her as senior director of machine-learning research there since 2018 while holding the Bren chair at Caltech, and fits the paper's datasets, which are the kind of industrial CFD that a chip company's research lab can afford to generate. The Ahmed-body set at 7 to 19 GPU-hours per sample is not a university budget.

The moment is 2023. The profile lists a Guggenheim Fellowship and a Schmidt Sciences AI2050 Senior Fellowship that year, AAAI Fellowship, and a briefing to the US President's Council of Advisors on Science and Technology on AI and science. The profile also quotes her view that "we have foundation models for language, not for physics", and that transformers cannot be the substrate for physics at scale because a few hundred grid points per dimension is a context of hundreds of billions of tokens. GINO is a paper in that argument. Its complexity table is the argument in one row: global, irregular, convergent, O(N log N + N·degree).

Nothing in this section is her view of Phient. The profile says plainly that Phient has done no weather work, that "Fourier" in gemphi means phinet's operator over word functions, and that no partnership is agreed with anyone. This page keeps to that. What it takes from the moment is narrower: in 2023 the group was making its operator indifferent to how its input was sampled, and that is a question Phient has not yet asked of its own operator.

## 15. What remains

For the paper, what remains is what its authors list: one category of shapes at a time, a model no better than its five hundred solves, no time dependence, no physics term in the loss, no proof of regularity for the defined operator, and an encoder that does not yet run in parallel. These are stated in the paper and are not this page's findings.

For Phient, what remains is the two experiments in section 12, both unrun. The resampling test would say whether phinet's operator is sensitive to how densely a word was observed, and whether a ball encoder on the circle removes that sensitivity. The kind-field test would say whether one operator under a smooth condition can do what five channel regions do now. Neither needs new machinery beyond a kernel over circular distance and a Riemann weight, and the engine's own timing (allocation-bound, 2 to 5 ms per MB op) says neither would be expensive.

Behind both is the measure problem from section 13. GINO's convergence rests on a measure on the domain that the mesh samples. Phical has counts. Whether counts can be given the role of a measure, so that seeing a word more often is refinement rather than reweighting, is a question about what a word is, and it is the paradigm sentence's own question. The facts file records that meaning is a side effect of pairs of words moving under a condition. It does not record what the limit of that motion is as the corpus grows, and until something is measured about that, this page will not say.

The sphere remains half-measured: it wins on the synthetic degree-filter task at 1.000 against 0.662 and loses on Austen at 12.8× against 14.7×, and it has not been run at softmax or wide+deep. GINO's GeoFNO result, the sphere mapping losing to the box with an SDF, is a reason to run it before assuming.

The helix remains a format. It gives every channel a coordinate in four systems and round-trips byte-exact. No trainer uses those coordinates for anything. If the ball encoder of section 12 is built, the helix is where its distance would come from, and that would be the first use.

Nothing about a Phi-4 run, weather, or partnership has changed. The oracle is synthetic. The next paper in this series is the last.

## 16. Sources

arXiv abstract page for the paper, fetched 2026-08-10: https://arxiv.org/abs/2309.00583

arXiv PDF, version 1, from which the architecture, datasets, tables 1 to 7 and the appendix speed-up statement were read: https://arxiv.org/pdf/2309.00583v1

NeurIPS 2023 poster page, confirming venue and author list: https://neurips.cc/virtual/2023/poster/72670

NeurIPS 2023 proceedings entry, found by search and not fetched from this session: https://proceedings.neurips.cc/paper_files/paper/2023/hash/70518ea42831f02afc3a2828993935ad-Abstract-Conference.html

Phient facts: `PHIENT_FACTS.md` in this folder. Profile: `../17_ANIMA_ANANDKUMAR_PROFILE.md`. Every Phient number on this page is from the first; every biographical statement is from the second.
