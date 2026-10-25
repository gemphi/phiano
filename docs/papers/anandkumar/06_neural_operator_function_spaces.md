# The map is the meaning: an operator between spaces of words

*Phient interpreted papers, 6 of 16 · 2026-03-23 · on: "Neural Operator: Learning Maps Between Function Spaces" (Kovachki, Li, Liu, Azizzadenesheli, Bhattacharya, Stuart, Anandkumar, 2023, Journal of Machine Learning Research 24(89):4061–4157; arXiv:2108.08481)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the sixth of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. The third paper was the graph kernel network, the fourth the multipole graph operator, the fifth the Fourier neural operator. Each of those was one method with its own benchmarks. This paper gathers them. It was first posted to arXiv on 19 August 2021, revised five times, the last on 2 May 2024, and published in the Journal of Machine Learning Research in 2023 as article 89 of volume 24, pages 4061 to 4157. It is the reference cited when the words "neural operator" are written without a qualifier.

The paper's claim is a definition and a theorem. The definition says that a neural operator is a composition of linear integral operators and pointwise nonlinearities, lifted from the input function and projected to the output, and that such a composition acts on functions and not on the grids they are sampled on. The theorem says the class can approximate any continuous operator between suitable function spaces, uniformly on compact sets. The paper then names four ways of paying for the integral, graph, multipole, low rank and Fourier, and states the cost of each. The experiments are on Burgers, Darcy and Navier–Stokes, gathered under one framework.

We read it here because it is the paper that says what phinet is. Phinet's operator, `FnoConfig { space, width, layers, modes, margin, lr }`, is this paper's Fourier parameterisation with a word's 64 channels in the place of a PDE's grid. The paradigm sentence says a word is nothing but its relations. This paper says an operator is nothing but its action on functions, and that the grid must not leak into the parameters. Those are one discipline in two vocabularies, and section 13 says where they part.

Sections 2 to 5 state what the paper says, in its own terms and with the numbers we could verify. Sections 6 to 10 place its objects inside the Phient family: the input function as a word, the domain coordinate as the relation channel, the operator as X ⇒ Y under c, the four parameterisations against phinet's one, and the stack of layers as the quest and the helix. Section 11 lists what Phient has measured that bears on it. Section 12 names one experiment and its control.

One limit is stated at the top. We fetched the arXiv abstract, the front sections of the arXiv HTML and PDF, and the front of the JMLR PDF. Every fetch was cut off before the numerical section. Section 4 therefore reports only the results the paper repeats in its abstract and introduction. No table entry from the experiments section is quoted here, because we did not see it.

## 2. What the paper claims, in its own terms

The abstract begins with what neural networks have been. "The classical development of neural networks has primarily focused on learning mappings between finite dimensional Euclidean spaces or finite sets." The paper's proposal is "a generalization of neural networks to learn operators, termed neural operators, that map between infinite dimensional function spaces." The form of the generalisation is stated in one sentence: "We formulate the neural operator as a composition of linear integral operators and nonlinear activation functions."

Two properties are then claimed for the class. The first is expressive power. "We prove a universal approximation theorem for our proposed neural operator, showing that it can approximate any given nonlinear continuous operator." The second is independence from the grid. "The proposed neural operators are also discretization-invariant, i.e., they share the same model parameters among different discretization of the underlying function spaces." The introduction sharpens the second claim into three conditions a model must meet. It must act on any discretisation of the input function. It must be evaluable at any point of the output domain. It must converge to a continuum operator as the discretisation is refined. The formal definition asks that, for a fixed parameter and any compact set of inputs, the discretised model's error against the continuum operator goes to zero as the mesh is refined.

The introduction then makes the claim that sets this paper apart. In its words, "Neural operators are the only known class of models that guarantee both discretization-invariance and universal approximation." That sentence is the paper's thesis. The paper's Table 1 sets neural operators beside networks, DeepONets and interpolation, and its summary row reads that "Neural Operators are discretization invariant deep learning methods that output functions and can approximate any operator."

The four parameterisations are named next. "We introduce four classes of efficient parameterization, viz., graph neural operators, multi-pole graph neural operators, low-rank neural operators, and Fourier neural operators." The application is the one the group had been pursuing since 2020: "learning surrogate maps for the solution operators of partial differential equations." The abstract ends with the result: on "the Burgers, Darcy subsurface flow, and the Navier-Stokes equations", the proposed operators "have superior performance compared to existing machine learning based methodologies, while being several orders of magnitude faster than conventional PDE solvers."

The paper also positions DeepONet, the main rival framework, inside its own. It states that "by introducing an adjustment to the DeepONet architecture, we propose the DeepONet-Operator model that fits into the full operator learning framework", and its Proposition 5 reads that "A neural operator with a point-wise parameterized first kernel and discretized integral operators yields a DeepONet." The claim is that the rival is a special case. The paper's first version carried a longer title, "Neural Operator: Learning Maps Between Function Spaces With Applications to PDEs". The final version dropped the last four words.

## 3. The mathematics, plainly

The object is a map G from an input function a, defined on a domain D, to an output function u, defined on a domain D′. The paper writes the learned map as a composition. Its equation (6) reads G_θ := Q ∘ σ_T(W_{T−1} + K_{T−1} + b_{T−1}) ∘ … ∘ σ_1(W_0 + K_0 + b_0) ∘ P. Read from the right. P is the lifting. The paper says it uses "a pointwise function ℝ^{d_a} → ℝ^{d_{v_0}}" to map the input into a wider channel space, and that "we usually choose d_{v_0} > d_a". Then come T layers. Then Q is the projection, again "a pointwise function", back to the output's channels, with "d_{v_T} > d_u" the usual choice. Neither looks at more than one point at a time.

A layer is equation (10). v_{t+1}(x) = σ_{t+1}( W_t v_t(Π_t(x)) + ∫_{D_t} κ^{(t)}(x, y) v_t(y) dν_t(y) + b_t(x) ). The first term is local: W_t is a matrix applied to the value at one point. The second is the integral kernel operator, equation (7): (K_t(v_t))(x) = ∫_{D_t} κ^{(t)}(x, y) v_t(y) dν_t(y) for every x in D_{t+1}. Here κ is a learned function of two points that returns a matrix, ν is a measure on the domain, and the integral is what makes the layer nonlocal. The third term is a bias function. Then σ acts pointwise. The whole cost of the method is in the kernel integral, and the four parameterisations are four ways to compute it.

The universal approximation result is deferred to a late section, Theorems 11 to 14 in the version we read. The introduction states its content: the models "are universal approximators of continuous operators acting between Banach spaces, and can uniformly approximate any continuous operator defined on a compact set of a Banach space." The paper also splits the total error of a trained model into "a sum of the discretization error and the approximation error." Discretisation invariance controls the first. The theorem controls the second.

The Fourier parameterisation is the one phinet uses. The kernel is taken to be translation invariant, κ(x, y) = κ(x − y), so the integral is a convolution and a product in the frequency domain. The layer becomes u(x) = F^{−1}(R_φ · F(v))(x). The paper says: "We thus parametrize R_φ directly as complex-valued (k_max × m × n)-tensor comprising a collection of truncated Fourier modes." The kept modes are the set Z_{k_max} of frequencies whose index in each dimension is at most k_max,j, counted from either end. On the choice of k_max: "In practice, we have found the choice k_max,j roughly around 1/3 to 2/3 of the maximum number of Fourier modes in the Fast Fourier Transform of the grid valuation of the input function provides desirable performance. In our empirical studies, we set k_max,j = 12."

## 4. Its experiments and numbers, as reported

The reason this section is short is stated plainly. Every fetch of the paper, the arXiv PDF in versions 1 and 6, the arXiv HTML, the ar5iv rendering and the JMLR PDF, was cut off before the numerical section. In one version the last heading we could see was "5.1 Poisson Equation". We did not see a single table entry from the Burgers, Darcy or Navier–Stokes experiments, so none is quoted. Readers who want them should open the JMLR PDF at the URL in section 16. What follows is what the paper says about its results in the parts we did see.

The introduction states the headline for the hardest problem. "For the two-dimensional Navier-Stokes equation, when learning the entire flow map, the method achieves <1% error for a Reynolds number of 20 and 8% error for a Reynolds number of 200." The method here is the Fourier neural operator.

The speed claim has two numbers. "The proposed Fourier neural operator (FNO) has an inference time that is three orders of magnitude faster than the pseudo-spectral method used to generate the data for the Navier-Stokes problem (Chandler and Kerswell, 2013) – 0.005s compared to the 2.2s on a 256 × 256 uniform spatial grid." The comparison is against the solver that made the training data. The parts we saw do not give the hardware for either timing.

On discretisation, Figure 1's caption reads "Discretization-Invariant operator has convergent predictions on a mesh refinement". The text says that for Darcy flow the "approximation error is approximately constant as we refine the discretization", and that a model trained at one resolution generalises to other mesh sizes without retraining. The paper also applies the learned operator inside a Bayesian inverse problem and says it "does not suffer from accuracy degradation when used in downstream applications." We saw no timing for that experiment.

The cost of each parameterisation is stated in the section on computation, which we did see.

| method | how the integral is paid for | cost as the paper states it |
|---|---|---|
| dense kernel | every pair of points | O(J²) |
| GNO | Nyström subsample of J′ points, truncated domain | O(J J′) |
| LNO | kernel as sum of r rank-one products | O(r J) |
| MGNO | kernel split by range, V-cycle | O(J) |
| FNO | product on k_max Fourier modes | O(J log J) by FFT, O(J k_max) for the truncated series |

J is the number of grid points. Of the Fourier method the paper says "the majority of the computational cost lies in computing the Fourier transform F(v) and its inverse", and that its fast form "can be applied in more general geometries via Fourier continuations." Nothing else about the experiments is claimed here.

## 5. The one idea, restated

The one idea is that the thing to learn is the map, and the grid is not part of the map. Everything else in the paper follows from taking that seriously.

If the map is between functions, then its parameters must not depend on how many points the functions are sampled at. That is discretisation invariance, and it is why the layer is an integral and not a matrix multiply over grid values. A matrix over grid values has a shape that is the grid's shape. An integral kernel κ(x, y) has a shape that is the domain's shape. Sample the domain finely or coarsely and the kernel is the same function.

If the map is between functions, then the question of expressive power is a question about operators, and the answer must be given in the language of function spaces. That is the universal approximation theorem. It says nothing about any grid. It says that for any continuous operator on a compact set of inputs, and any tolerance, there is a neural operator within that tolerance everywhere on the set.

If the map is between functions, then the cost of the map is the cost of the integral, and the practical work is to make the integral cheap without making it local. Local is easy and useless: a pointwise map cannot move information across the domain. The paper's four parameterisations are four bargains with the integral. The graph method samples it. The low-rank method factors it. The multipole method splits it by range. The Fourier method assumes translation invariance and buys the whole integral for the price of an FFT. The paper is honest that each bargain gives something up. The Fourier bargain gives up general geometry, which the paper says can be recovered with Fourier continuation, and it gives up high frequencies, which the paper's choice of k_max = 12 makes explicit.

Restated for Phient: the operator is the thing. The paradigm sentence says a word is nothing but its relations under conditions. This paper says an operator is nothing but its action on functions, independent of the sampling. Put the two together and the claim becomes that a language model is an operator on the space of word functions, that the number of channels a word is sampled at is a discretisation and not a property of the word, and that what should be learned, and what should carry the parameters, is the map from one word's function to the next under a condition. Phinet was built on that claim. The next five sections say how far it holds.

## 6. A word as a function

The paper's input is a function a on a domain D. For Darcy flow it is a coefficient field, one value at every point of a square. For Navier–Stokes it is a vorticity field on a torus. The first thing the operator does is lift it: the pointwise map P takes the value at each point and widens it to d_{v_0} channels, with d_{v_0} larger than d_a. The paper says this is the usual choice.

Phical's input is a word. The paradigm sentence says the word is 32 to 64 relations, each a complex phasor on a closed circle with a count. Read as this paper reads its inputs, a word is a function whose domain is the set of relation channels and whose value at a channel is a unit complex number weighted by a count. The domain is discrete and small. The paper's domain is continuous and large. That is the first difference and it is not small. But the paper's framework asks only that the domain carry a measure and that the function be sampled on it, and 64 channels with a count on each is a measure space, if a plain one.

Phinet's tensor is `Tensor [b, w, s]` complex: a batch, a set of words, and a spatial axis s. The spatial axis is the channel axis. Phinet's `Fno` lifts along that axis in the way the paper's P does, to a width set by `FnoConfig.width`, and projects back at the end. The complex phasor is the value of the function at a point of the domain, and the count is a weight on the measure ν. The paper's integral ∫ κ(x, y) v(y) dν(y) becomes, in phinet, a sum over channels y of a kernel times the phasor at y, weighted by how often that relation was seen.

The consequence the paper insists on is that the parameters must not know how many channels there are. In phinet they partly do not. `FnoConfig.modes` sets how many Fourier modes are kept, and the learned complex weights live on the modes, which is the paper's R_φ of shape k_max × m × n. A word with 32 channels and a word with 64 pass through the same weights. Whether the operator then behaves the same, in the paper's sense of converging as the channel count grows, has not been measured. The facts file records no run at two channel counts on one task. Section 12 asks for that run.

One thing in the facts file bears on the lift. The sphere tie, `phinet::Tie::Sphere`, reads the 64 channels as 8² spherical harmonics Y_l^m with l < 8 and learns one complex weight per degree. That is a lift that knows the domain's geometry, which the paper's pointwise P does not. Section 9 returns to it.

## 7. The relation channel as a coordinate

In the paper, the coordinate x is a point of the domain, and the kernel κ(x, y) is a function of two coordinates. The kernel is the whole of the operator's nonlocal behaviour. What the layer does at x depends on v at every y, weighted by κ(x, y). For the Fourier parameterisation the kernel is a function of x − y only.

In Phical the coordinate is the relation channel. Channel y is one of a word's 32 to 64 relations, and the phasor at y says where on the closed circle that relation sits. A kernel κ(x, y) on channels is a statement about how relation y should move relation x. The paper's translation-invariant kernel says that statement depends only on how far apart the channels are in the channel ordering. That is a strong claim and it deserves to be said plainly. It says the channels have an order, that the order is circular, and that relation 5 moves relation 7 the way relation 40 moves relation 42.

Phinet makes exactly that claim when it runs a unitary FFT along the channel axis. The FFT treats the channel index as a position on a circle of length s. The paper's Z_{k_max} keeps the modes whose index is at most k_max from either end of that circle. Phinet's `modes` does the same. Every measured Austen result in the facts file, from the identity operator's 7.9× chance to the wide and deep softmax operator's 30.7× on validation, was obtained under this assumption. It has never been tested against its alternative, a dense kernel over channel pairs with no order at all. Section 12 names that as the control.

The paper has one argument for why translation invariance is the right prior in its domain: the physics is the same everywhere in the box. Phical has no such argument for its channels. The channels are relations, and there is no reason relation 5 and relation 7 should interact the way relation 40 and relation 42 do. What Phient has instead is the byte deadband. The facts file says a channel is one byte, 256 quanta of 2π, so each channel is itself a circle. The FFT along channels is a circle of circles. The paper's coordinate is a point in a box. Section 13 counts the mismatch.

One more thing the paper says about coordinates bears on Phical. The output can be evaluated "at any point of the output domain." The kernel is a function, so u(x) can be asked for at an x that was never in the training grid. For Phical this would mean asking for the phasor of a relation that was never a channel. Phient has no such operation. Whether it would mean anything is a question about what channels are, and the facts file does not settle it.

## 8. The operator as X ⇒ Y under c

The paradigm names an operator X ⇒ Y and a condition c. The paper names an operator G from a to u and a parameter θ. The correspondence is direct for the operator and less direct for the condition.

The paper's G_θ is a fixed map. Once trained, the same θ acts on every input a. The condition, in the paper's world, is the input itself. A Darcy coefficient field is the condition under which the pressure field is what it is. In Phical the condition c is the setting under which two words are related: the sentence, the persona, the quest's relation kind. The operator X ⇒ Y under c takes the function of word X and returns the function it should move toward, and c selects which movement.

Phinet's training, as the facts file records it, is a ranking task on Austen. The operator takes a word's function and is scored on whether the next word's function ranks near the top of the pool of 1,228. The 204,592 training examples are pairs (X, Y) from three novels. The condition c there is the corpus, held fixed. The identity operator, which returns X unchanged, ranks Y at 7.9× chance, the baseline that says how much of the relation is in the word before any operator moves it. The hinge operator with 8,360 parameters reaches 14.7× on validation and 14.6× on test. The softmax operator with β = 8 reaches 18.0× and 18.8×. The wide and deep softmax operator with 50,016 parameters reaches 27.6× at 3 epochs and 30.7× on validation and 30.3× on test at 8 epochs, with hit@10 of 0.348. Those are the operator's numbers and the only ones.

The paper's operator learns a solution map with a physical ground truth. Phinet's operator has no solver behind it. The next word in Austen is not the solution of anything. The ranking score says how well the operator's output points at what Austen wrote, a weaker kind of truth than a PDE's. The paper's framework does not require a physical ground truth, only a distribution over inputs and a target operator. But its theorem assumes the target operator is continuous, and the map from a word to the next word in a novel is not. Section 13 counts that too.

The one place Phical's condition appears as a coordinate in the paper's sense is the quest. There the relation kind, is-a, part-of, near, used-for, unlike, selects a channel, and the facts file's "wrong channel 0.015" says the operator read on the wrong channel returns nothing. That is c as a coordinate. The paper has no equivalent.

## 9. Four bargains and phinet's one

The paper's central practical contribution is the set of four parameterisations, and phinet has one of them. This section places each against what Phient has.

The graph neural operator samples the integral at J′ points and truncates the kernel to a neighbourhood. The third interpreted paper covered it. Nothing in gemphi does message passing on a graph. The multipole operator splits the kernel by range and links coarse and fine graphs with a V-cycle. The fourth interpreted paper covered it. Nothing in gemphi has a V-cycle. The low-rank operator writes κ(x, y) as a sum of r products φ^{(j)}(x) ψ^{(j)}(y), so the integral becomes r inner products. Nothing in gemphi does this either, though it is the closest of the three to something Phient could build quickly: a rank-r kernel over 64 channels is two 64 × r complex matrices.

The Fourier operator is phinet. `FnoConfig { space, width, layers, modes, margin, lr }` sets the domain, the lift width, the number of layers T, the k_max, a ranking margin and a learning rate. The unitary FFT is the paper's F. The learned complex weights on the retained modes are the paper's R_φ. The `Tape` trains θ. The facts file says the engine is allocation and bandwidth bound at these sizes, a 1 MB op taking 2 to 5 ms, and the paper says of its Fourier method that "the majority of the computational cost lies in computing the Fourier transform F(v) and its inverse." The two statements agree about where the time goes.

There is a fifth parameterisation in gemphi that the paper does not have. The sphere tie, `phinet::Tie::Sphere`, reads the 64 channels as the spherical harmonics Y_l^m with l < 8 and keeps one complex weight per degree l. The facts file records it at 1,192 parameters against the circle's 8,360, reaching 12.8× on validation and 13.1× on test on Austen hinge at 3 epochs, against the circle's 14.7× and 14.6×, in 89 s against 142 s. On a synthetic degree-filter task of 200 items over 40 epochs the sphere reaches MRR 1.000 where the circle reaches 0.662 and the identity 0.646. The sphere tie is the paper's Fourier idea with the circle replaced by the sphere, which is what the profile says FourCastNet later did for the globe. Phifno 1.0.0 is the spherical operator as a child crate.

The table below is the paper's four and Phient's two.

| parameterisation | in the paper | in gemphi | measured |
|---|---|---|---|
| graph (GNO) | yes | no | nothing |
| multipole (MGNO) | yes | no | nothing |
| low rank (LNO) | yes | no | nothing |
| Fourier on a circle (FNO) | yes | phinet Fno | Austen 30.7×/30.3×, 50,016 params |
| Fourier on a sphere | no | Tie::Sphere, phifno | Austen hinge 12.8×/13.1×, 1,192 params; degree filter 1.000 |

The sphere at softmax and wide+deep is unrun. The facts file says so.

## 10. Layers as the quest, and the helix

The paper's operator is T layers deep. Each layer is the same shape: a local term, a kernel integral, a bias, a nonlinearity. Each layer moves the whole function once. The composition is what lets a shallow kernel produce a deep map.

Phient has two places where a function is moved repeatedly, and neither is a stack of layers in the paper's sense. The first is the quest. In phiecs the quest is an entity-component world with a synthetic oracle: 256 words in families of 8, five relation kinds, 1,280 rows. A trainer nudges pairs of words toward each other on the channel selected by the relation kind. The facts file records that the shipped trainer reached MRR 0.082 against chance 0.021, and that an attraction-only trainer with dithered rounding reached 0.594, hit@1 34.8%, hit@5 97.4%, within-kind 0.747. That is one hop. The paper's single layer is one hop too. The quest's two-hop numbers on held-out words are 0.095 for is-a and 0.093 for used-for, with part-of, near and unlike at about chance, and one hop on held-out at 0.011. The paper would say two hops is two layers, and that a two-layer operator composes kernels. The quest's two hops are two applications of a nudge, close to the paper's iteration and far from its learned composition.

The second place is the helix. The facts file describes the lexicon as a DNA-coded tree: 64 codons as base-4 ACGT of the channel, hexagram, sphere degree and order, B-DNA geometry at 10.5 codons per turn and 0.34 nm rise, a PhiTon wavelength and band per codon. 18,496 words serialise in 1.9 s to 75 MB and round-trip byte-exact. The helix is a coordinate system on the channels, and the paper has no equivalent. What the paper has is the claim that the operator is independent of the coordinate system. The helix gives a channel a position on a helix, a codon, a hexagram and a spherical degree at once. The Fourier layer uses only the circular position. The sphere tie uses only the spherical degree. No operator in gemphi uses the codon or the hexagram as a coordinate for the kernel, and the facts file records no measurement of the 64 hexagram gates.

Where the paper's stack of layers lands most exactly is phiano's generation loop. Decoding applies the operator, reads a word, and applies it again. The facts file reports that loop over 40 held-out prompts and 30 tokens: the operator's nearest-k decoding at a Kneser-Ney perplexity of 66.9, improved from 97.2, against the trigram sampler's 13.3, with the operator sharing 8.7% of its 4-grams across prompts against the sampler's 25.7%. The operator is worse at predicting Austen and better at not repeating itself. That loop is the paper's composition with a discrete readout between layers, which the paper never does.

## 11. What Phient has already measured that bears on it

Every number in this section is in the facts file and was measured for another purpose. None was measured to test this paper.

The first fact that bears on the paper is parity. Phinet passes 62 of 62 parity tests against a PyTorch reference. That says the engine computes the Fourier layer the way the paper's own reference implementation does, for the ops covered by those tests. It does not say the engine learns the same operators, only that the arithmetic agrees.

The second is the Austen operator. The paper's whole framework is a ranking of parameterisations on a fixed task, and the facts file holds one such ranking, on one task. Identity 7.9× chance. Hinge with 8,360 parameters 14.7× on validation, 14.6× on test. Softmax with β = 8 18.0× and 18.8×. Softmax wide and deep with 50,016 parameters 27.6× at 3 epochs, 30.7× and 30.3× at 8 epochs, hit@10 0.348. Chance MRR is 0.0063 on a pool of 1,228 words seen 20 or more times, with 204,592 training examples. The paper would want the same table across channel counts, which Phient has not run.

The third is the sphere tie against the circle, which is the nearest thing in the facts file to the paper's comparison of parameterisations. On Austen hinge at 3 epochs the sphere reaches 12.8× and 13.1× with 1,192 parameters in 89 s; the circle reaches 14.7× and 14.6× with 8,360 parameters in 142 s. On the synthetic degree-filter task the sphere reaches MRR 1.000, the circle 0.662, the identity 0.646. The second result is a case where the operator's parameterisation matches the task's symmetry and the paper's translation-invariant kernel does not.

The fourth is the engine's cost. A 1 MB op takes 2 to 5 ms, and the engine is allocation and bandwidth bound at these sizes. The paper says the Fourier method's cost is in the transforms. The two statements are consistent and neither has been tested against the other.

The fifth is the decoding loop, section 10, and the sixth is the quest at one and two hops, also section 10. The seventh is persona attribution, 71% overall on three writers against 33% chance, with 74, 65 and 86 by writer, and dialogue at 44% by the sector reader and 72% by the Kneser-Ney panel. Those are operator outputs judged downstream, which is what the paper's Bayesian inverse problem is about, and they say nothing about discretisation.

Two things bear on the paper by their absence. The facts file records no run of the Austen operator at two channel counts. It records the sphere at softmax and at wide and deep as unrun. Both gaps are the paper's central claim untested in phinet.

## 12. The experiment that would test the paper's idea inside phinet

The paper's idea, stripped to what phinet can test, is discretisation invariance: the same parameters, applied to the same function sampled at two resolutions, give the same answer up to a discretisation error that shrinks as the resolution grows. Phinet's resolution is the channel count. The experiment is to train the Austen operator at one channel count and evaluate it at another, with the weights untouched.

The setup uses what the facts file already names. The task is Austen ranking on the pool of 1,228 words, 204,592 training examples, chance MRR 0.0063. The operator is the hinge operator, because it has a measured sphere counterpart and is the cheapest to run twice. The words are built at 32 channels and at 64 from the same corpus by the same ingest. The operator is trained at 32 with `modes` fixed below 16, so that the retained modes exist in both discretisations. It is then evaluated at 64 without retraining, and the reverse. The measure is MRR against chance, the same as every other Austen number.

The paper's prediction is specific. If phinet's operator is discretisation invariant in the paper's sense, the operator trained at 32 and evaluated at 64 should score close to the operator trained and evaluated at 64, and the gap should be the discretisation error, which the paper says is approximately constant under refinement. If the operator is a grid model wearing a Fourier coat, the gap will be large and the cross-resolution score will fall toward the identity's 7.9×.

The control is named. It is a dense kernel over channel pairs, a 32 × 32 or 64 × 64 complex matrix per layer with no FFT and no truncation, trained and evaluated at one channel count only. That is the paper's O(J²) baseline. It cannot be evaluated across channel counts at all, which is the point. Its score at 32 and at 64 says what the Fourier prior costs or gains at one resolution, and the cross-resolution run says what the prior buys that the dense kernel cannot. A second control is the identity operator, already measured at 7.9×, the floor.

Two more measurements belong in the same run. The first is time per epoch at each channel count, because the facts file's 2 to 5 ms per 1 MB op predicts a near-linear scaling and so does the paper's O(J log J). The second is the sphere tie in the same protocol, 4² harmonics against 8², because its degree-filter result of 1.000 is the strongest single number in the facts file and no one has asked whether it survives a change of resolution.

What the experiment cannot test is the universal approximation theorem. The theorem is about continuous operators on Banach spaces, and the map from a word to the next word in Austen is not one. The experiment tests the paper's engineering claim and leaves its theorem alone.

## 13. Where we disagree or where the analogy breaks

The analogy breaks in four places and it is better to count them than to smooth them.

The first is the domain. The paper's function lives on a box or a torus with a continuum of points. A word lives on 32 to 64 channels. The paper's argument for discretisation invariance is that the grid samples a continuum and the operator should converge as the sample grows. There is no continuum of relations that 64 channels sample. Whether a 128-channel word is a finer sampling of the same word, or a different word, is a question Phical has not answered and the paper cannot answer for it. Until it is answered, "discretisation invariance" in phinet is a property of the code and not of the meaning.

The second is the kernel's prior. The paper's Fourier layer assumes the kernel depends on x − y because the physics is homogeneous. Phinet's FFT along the channel axis assumes the same for relations, and nothing in gemphi says why relation channels should be homogeneous. The byte deadband makes it worse: each channel is a circle of 256 quanta, so the domain is a circle of circles, and the FFT is taken along only one of them. The sphere tie's degree-filter result, 1.000 against the circle's 0.662, is a measurement that the prior matters, and its Austen result, 12.8× against 14.7×, is a measurement that the sphere prior is not the right one either.

The third is continuity. The universal approximation theorem is about continuous operators. The map from a word to the next word in a novel is not continuous in any topology Phical has defined. The theorem therefore says nothing about whether phinet's operator can learn it. What transfers is the discretisation claim, which is engineering, and section 12 tests it.

The fourth is ground truth. The paper's operators are trained against a solver that is correct to a tolerance. Phinet's are trained against Austen, who is correct only in the sense that she wrote what she wrote. The paper's error is a relative L² norm against a known function. Phinet's score is a rank against a pool. A rank at 30.7× chance says nothing about how far the output function is from the target, only that the target is near the top of the list. The paper's Bayesian inverse problem needs the first kind of error and Phient can only produce the second.

Where we disagree with the paper is narrow. The paper says that neural operators "are the only known class of models that guarantee both discretization-invariance and universal approximation." Phient has no basis to dispute that. But the paper's DeepONet section shows the framework absorbing its rival by definition, and Phient's sphere tie is a parameterisation the paper does not list. A framework that can absorb any model that acts on functions says less by saying more. The paper's value to Phient is the four costed bargains with the integral and the discipline that the grid must not enter the weights. Its theorem is not something phinet can use.

## 14. The person and the moment

The profile is the only source for this section. Its facts are hers; the reading is ours.

By the time the first version of this paper was posted, in August 2021, the sequence it gathers was already public. The graph kernel network in March 2020, the multipole operator in June 2020, the Fourier neural operator later that year. The profile records that Quanta profiled the Fourier work in 2021. This paper is the one written afterwards, to say what the three had in common and to prove it. The author list is the same seven names as the graph kernel paper's, with Kovachki first this time and Li second. Anandkumar is last, in the position of the senior author.

The profile places her in three institutions at once during those years: the Bren Professorship at Caltech from 2018, the senior directorship of machine learning research at NVIDIA from 2018, and the AI4Science initiative she co-founded at Caltech. FourCastNet, which put an adaptive Fourier operator on the whole planet, was posted in February 2022, six months after this paper's first version. The profile records that it ran at 0.25° on a 720 × 1440 grid, produced a week's forecast in under two seconds, and later moved to spherical harmonics to respect the geometry of the globe. That last move is the one Phient's sphere tie copies.

The profile quotes her view of what the operator work is for: "We have foundation models for language, not for physics." Her argument, as the profile records it, is that a transformer over a few hundred grid points per dimension is a context of hundreds of billions to a trillion tokens, and that an operator that learns in function space treats scale as what it is. This paper is the mathematical form of that argument. Its method, in her words from the profile, is to take "all of the things that work with deep learning" and "make them a bit more principled."

The date on this page is 23 March 2026. The profile records that one week earlier, on 16 March 2026, the United Nations appointed her to its Scientific Advisory Board, the fifteen-member body that advises the Secretary-General, with Yoshua Bengio and Thomas Südhof among the members. The profile also records her first decade of tensor methods, her Time100 AI listing in 2026, and her habit of campaigning for the things she thinks are wrong: the renaming of NIPS, the end of gender segregation in admissions at IIT Madras. The profile is explicit that nothing in it is her endorsement of Phient and that no partnership exists or has been discussed. The facts file says the same. This page reads her paper. It does not speak for her.

## 15. What remains

What remains is mostly measurement, and one question that measurement cannot settle.

The measurement is section 12. Phinet has never been run at two channel counts on one task with one set of weights. Until it is, the word "operator" in phinet's name is borrowed and not earned. The run is cheap. The hinge operator at 8,360 parameters trains in 142 s on the circle and 89 s on the sphere, and a 32-channel version will be faster. The dense-kernel control is a matrix multiply and an afternoon. The result will either show that phinet's parameters live on the modes, as the paper requires, or that they live on the grid, in which case the paper's framework does not describe phinet and this page should be rewritten.

The second piece of measurement is the sphere at softmax and at wide and deep. The facts file lists both as unrun. The sphere tie's degree-filter result is the strongest number Phient has, and its Austen result is weaker than the circle's, and no one knows whether the gap closes with a better loss because no one has looked.

The third is the low-rank operator. Of the paper's four parameterisations it is the only one Phient could build without a new data structure, two 64 × r complex matrices per layer, and it is the natural control for the question of whether the channel axis has any order at all. If a rank-8 kernel with no Fourier prior matches the circle's 14.7×, the FFT is doing nothing for Austen that a factored matrix would not.

The question measurement cannot settle is what a finer word is. The paper's whole argument rests on a continuum that the grid samples. Phical has 32 to 64 channels and the helix gives each a codon, a hexagram, a spherical degree and a wavelength. None of those says what channel 65 would be. If channels are relations, then a finer word has more relations and is a different word. If channels are samples of a field, then a finer word is the same word seen more closely. Phical's paradigm sentence says the first. Phinet's engine assumes the second. The paper does not resolve the tension, because in its domain there is no tension, and Phient has to resolve it before it can claim the paper's result.

Nothing on this page should be read as a result. Every number about Phient is one already in the facts file. Every claim about the paper is one we saw in the parts of it we could fetch, and the experiments section is not among them. No weather work has been done. No Phi-4 run has been made. No partnership exists.

## 16. Sources

The following were fetched on 2026-09-27 for this page. The arXiv abstract page gave the title, authors, abstract, version history and journal reference. The arXiv PDF in versions 1 and 6, the arXiv HTML for version 6, the ar5iv rendering, and the JMLR PDF each gave the front sections through the parameterisation and computation chapter; none of the five fetches reached the numerical experiments, and no table entry from that section appears on this page.

arXiv abstract, Kovachki, Li, Liu, Azizzadenesheli, Bhattacharya, Stuart, Anandkumar, "Neural Operator: Learning Maps Between Function Spaces", arXiv:2108.08481, v1 19 August 2021, v6 2 May 2024. https://arxiv.org/abs/2108.08481

arXiv PDF, version 6. https://arxiv.org/pdf/2108.08481v6

arXiv PDF, version 1, under the title "Neural Operator: Learning Maps Between Function Spaces With Applications to PDEs". https://arxiv.org/pdf/2108.08481v1

arXiv HTML, version 6. https://arxiv.org/html/2108.08481v6

ar5iv rendering. https://ar5iv.labs.arxiv.org/html/2108.08481

Journal of Machine Learning Research 24(89):4061–4157, 2023, PDF. https://www.jmlr.org/papers/volume24/21-1524/21-1524.pdf

Phient's own facts and the profile of the author are the two files named in the brief, `PHIENT_FACTS.md` in this folder and `17_ANIMA_ANANDKUMAR_PROFILE.md` in the parent folder. The profile's own sources are listed at its foot and were not re-fetched for this page.
