# The kernel is a convolution: learned weights on kept modes and the word as a field

*Phient interpreted papers, 5 of 16 · 2026-03-09 · on: "Fourier Neural Operator for Parametric Partial Differential Equations" (Li, Kovachki, Azizzadenesheli, Liu, Bhattacharya, Stuart, Anandkumar, 2021, International Conference on Learning Representations (ICLR 2021); arXiv:2010.08895)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the fifth of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. It is the paper that most people mean when they say neural operator. The third paper in this series was the graph kernel network, a learned kernel integrated by message passing, with a cost that grew as the square of the number of nodes. The fourth was the multipole operator, which cut that cost to linear by splitting the kernel into ranges. This paper takes the other road. It gives up the general kernel and keeps only the convolution. A convolution is a multiplication in Fourier space, so the whole integral becomes one FFT, one multiplication by learned complex weights on a few low modes, and one inverse FFT. The paper was posted to arXiv on 18 October 2020 and appeared as a poster at ICLR 2021.

The paper's claim is that this restriction loses little and gains a great deal. It reports that the Fourier neural operator beats every learning-based method it compares against on Burgers' equation, Darcy flow and two-dimensional Navier-Stokes at fixed resolution. It reports that the operator can be trained at one resolution and evaluated at a finer one with no retraining, which it calls zero-shot super-resolution, and that it is the first learned method to do so on turbulent flow. It reports inference up to three orders of magnitude faster than the solver that made its training data.

We read it here because phinet is this paper's architecture in a different domain. Phinet's Fno is a unitary FFT, learned complex weights on the retained modes, an inverse FFT, a local linear path and a nonlinearity. That is the paper's Fourier layer. The difference is what the function is. In the paper it is a field on a grid, a coefficient or a vorticity. In Phical it is a word, 32–64 relations, each a complex phasor on a closed circle with a count. The paper is the origin of phinet's operator and the clearest statement of what that operator assumes.

Sections 2 to 5 state what the paper says, in its own terms and with its numbers. Sections 6 to 10 place its objects inside the Phient family: the input function as a word, the Fourier mode as a channel, the layer as X ⇒ Y under c, the sphere and the tensor, and the recurrent form as the agent loop. Section 11 lists what Phient has measured that bears on it, all from the facts file. Section 12 names one experiment with its control. Section 13 says where the analogy breaks. Section 14 is the person and the moment. Section 15 is what remains.

One limit is stated at the top. Phient has solved no PDE. There is no Burgers, no Darcy, no Navier-Stokes and no weather in gemphi. Every number about Phient below was measured on words.

## 2. What the paper claims, in its own terms

The abstract begins with the shift the group had been making for a year. "The classical development of neural networks has primarily focused on learning mappings between finite-dimensional Euclidean spaces. Recently, this has been generalized to neural operators that learn mappings between function spaces." The consequence for PDEs is stated plainly. "For partial differential equations (PDEs), neural operators directly learn the mapping from any functional parametric dependence to the solution. Thus, they learn an entire family of PDEs, in contrast to classical methods which solve one instance of the equation."

Then the new step. "In this work, we formulate a new neural operator by parameterizing the integral kernel directly in Fourier space, allowing for an expressive and efficient architecture." The experiments are named: "We perform experiments on Burgers' equation, Darcy flow, and Navier-Stokes equation." And the three claims that the rest of the paper defends. "The Fourier neural operator is the first ML-based method to successfully model turbulent flows with zero-shot super-resolution. It is up to three orders of magnitude faster compared to traditional PDE solvers. Additionally, it achieves superior accuracy compared to previous learning-based solvers under fixed resolution."

The contributions section says the same three things with more edge. The method "is the first work that learns the resolution-invariant solution operator for the family of Navier-Stokes equation in the turbulent regime." The architecture, "by construction," "shares the same learned network parameters irrespective of the discretization used." And it "consistently outperforms all existing deep learning methods."

The paper is exact about what it keeps and what it drops. The general neural operator of the earlier papers had a kernel that depended on two points and the coefficient at both. This paper writes: "If we remove the dependence on the function a and impose κφ(x, y) = κφ(x−y), we obtain that (3) is a convolution operator." The convolution theorem then turns the integral into a product of transforms. The conclusion says it in one sentence. "We introduce the Fourier neural operator, a novel deep learning architecture able to learn mappings between infinite-dimensional spaces of functions; the integral operator is restricted to a convolution, and instantiated through a linear transformation in the Fourier domain."

The paper is also clear about resolution. "Since parameters are learned directly in Fourier space, resolving the functions in physical space simply amounts to projecting on the basis e^{2πi⟨x,k⟩}." A weight attached to a mode is a weight attached to a frequency, and a frequency means the same thing on any grid.

Two further claims sit in the body. The paper places the method inside a Bayesian inverse problem and reports the wall-clock difference against the solver. And it distinguishes two ways of handling time, a two-dimensional operator run recurrently and a three-dimensional operator over space and time at once, and says which it prefers. Both are in section 4.

Every quotation above was read from the arXiv abstract page or from the v3 PDF through a text extraction. Where our extraction returned a paraphrase rather than a sentence, we have not quoted it.

## 3. The mathematics, plainly

The setting is the neural operator of the earlier papers. An input function a on a domain D is to be mapped to an output function u on the same domain. The paper first lifts the input pointwise. "The input a ∈ A is first lifted to a higher dimensional representation v0(x) = P(a(x)) by the local transformation P." Then it iterates. Definition 1 gives the update: "vt+1(x) := σ(W vt(x) + K(a; φ)vt(x)), ∀x ∈ D." W is a linear map from ℝ^dv to ℝ^dv applied at each point. K(a; φ) is a kernel integral operator with parameters φ. σ is an activation applied componentwise. At the end the state is projected back. "The output u(x) = Q(vT(x)) is the projection."

The earlier papers let K be a general integral against a learned kernel κφ(x, y, a(x), a(y)). This paper restricts it. "By letting κφ(x, y, a(x), a(y)) = κφ(x−y) in (3) and applying the convolution theorem," the integral becomes a product in Fourier space. Definition 3 is the Fourier layer: "K(φ)vt(x) = F⁻¹(Rφ · (Fvt))(x) ∀x ∈ D." F is the Fourier transform and F⁻¹ its inverse. R is the transform of the kernel, and the paper learns R directly rather than κ.

R is a complex tensor of shape k_max × d_v × d_v, with the modes truncated and conjugate symmetry imposed so that the result is real. On a discrete grid the multiplication is, for each kept mode k and each output channel l, "(R · (Fvt))_{k,l} = Σ_{j=1}^{dv} R_{k,l,j} (Fvt)_{k,j}, k = 1, ..., k_max." So at every kept frequency there is a full d_v × d_v complex matrix mixing the channels, and at every dropped frequency there is nothing.

The truncation is the whole economy. A general Fourier transform on n points costs O(n²). The paper notes that "since we truncate the series the complexity is in fact O(nk_max), while the FFT has complexity O(n log n)." The multiplication by R touches only "k_max < n modes, so the inner multiplication has complexity O(k_max)." The paper calls the result quasi-linear. In the benchmarks the paper keeps "k_max,j = 12 which yields k_max = 12^d parameters per channel" in d dimensions; in the training section it gives k_max,j = 16 with d_v = 64 for the one-dimensional problem and k_max,j = 12 with d_v = 32 for the two-dimensional ones.

The pointwise path W is not decoration. The Fourier layer is periodic by construction, and the paper says the linear transform W "keeps the track of non-periodic boundary." The convolution reaches everywhere at once and cannot see an edge. W sees only the point and can. The nonlinearity σ between layers is what lets the composition of global linear steps and local nonlinear steps represent a nonlinear operator. The paper describes the whole as "linear, global integral operators (via the Fourier transform) and nonlinear, local activation functions."

That is the entire method. Lift, then four times: transform, multiply the low modes by learned complex matrices, transform back, add a local linear map, apply the activation. Then project.

## 4. Its experiments and numbers as reported

The paper uses four Fourier layers throughout. Training runs 500 epochs of Adam at an initial learning rate of 0.001, halved every 100 epochs, with 1,000 training instances and 200 test instances by default. Batch normalisation and ReLU are used. Everything runs on "a single Nvidia V100 GPU with 16GB memory."

The first benchmark is the one-dimensional viscous Burgers equation, "∂_tu + ∂_x(u²/2) = ν∂_xxu, x ∈ (0,1), t ∈ (0,1]" with ν = 0.1, mapping the initial condition to the solution at time 1. Table 3 gives relative error against resolution.

| method | 256 | 512 | 1024 | 2048 | 4096 | 8192 |
|---|---|---|---|---|---|---|
| NN | 0.4714 | 0.4561 | 0.4803 | 0.4645 | 0.4779 | 0.4452 |
| FCN | 0.0958 | 0.1407 | 0.1877 | 0.2313 | 0.2855 | 0.3238 |
| PCANN | 0.0398 | 0.0395 | 0.0391 | 0.0383 | 0.0392 | 0.0393 |
| GNO | 0.0555 | 0.0594 | 0.0651 | 0.0663 | 0.0666 | 0.0699 |
| LNO | 0.0212 | 0.0221 | 0.0217 | 0.0219 | 0.0200 | 0.0189 |
| MGNO | 0.0243 | 0.0355 | 0.0374 | 0.0360 | 0.0364 | 0.0364 |
| FNO | 0.0149 | 0.0158 | 0.0160 | 0.0146 | 0.0142 | 0.0139 |

The second is Darcy flow, "−∇·(a(x)∇u(x)) = f(x), x ∈ (0,1)²," with the coefficient drawn as "a ∼ µ where µ = ψ#N(0,(−Δ+9I)⁻²)." Table 4:

| method | s = 85 | s = 141 | s = 211 | s = 421 |
|---|---|---|---|---|
| NN | 0.1716 | 0.1716 | 0.1716 | 0.1716 |
| FCN | 0.0253 | 0.0493 | 0.0727 | 0.1097 |
| PCANN | 0.0299 | 0.0298 | 0.0298 | 0.0299 |
| RBM | 0.0244 | 0.0251 | 0.0255 | 0.0259 |
| GNO | 0.0346 | 0.0332 | 0.0342 | 0.0369 |
| LNO | 0.0520 | 0.0461 | 0.0445 | not reported |
| MGNO | 0.0416 | 0.0428 | 0.0428 | 0.0420 |
| FNO | 0.0108 | 0.0109 | 0.0109 | 0.0098 |

On both problems the FNO row is flat across resolution and lowest at every column. On Darcy it is below RBM, which had beaten every learned method in the two earlier papers.

The third is two-dimensional Navier-Stokes in vorticity form, "∂_tw + u·∇w = ν∆w + f, x ∈ (0,1)², t ∈ (0,T]," at viscosities 1e-3, 1e-4 and 1e-5 with final times T = 50, 30 and 20, N = 1000 training instances except one column at N = 10000, 200 test instances, on a 64 × 64 grid. Table 1:

| method | params | time/epoch | ν=1e-3, T=50 | ν=1e-4, T=30, N=1000 | ν=1e-4, T=30, N=10000 | ν=1e-5, T=20 |
|---|---|---|---|---|---|---|
| FNO-3D | 6,558,537 | 38.99 s | 0.0086 | 0.1918 | 0.0820 | 0.1893 |
| FNO-2D | 414,517 | 127.80 s | 0.0128 | 0.1559 | 0.0834 | 0.1556 |
| U-Net | 24,950,491 | 48.67 s | 0.0245 | 0.2051 | 0.1190 | 0.1982 |
| TF-Net | 7,451,724 | 47.21 s | 0.0225 | 0.2253 | 0.1168 | 0.2268 |
| ResNet | 266,641 | 78.47 s | 0.0701 | 0.2871 | 0.2311 | 0.2753 |

FNO-2D "maps the solution at the previous 10 time steps to the next time step" and is run recurrently. FNO-3D "performs convolution in space-time" and "maps the initial time steps directly to the full trajectory." The paper finds "the 3-d method to be more expressive and easier to train." Figure 1 shows super-resolution: "trained on 64 × 64 × 20 dataset; evaluated on 256 × 256 × 80," in space and in time. On speed: "On a 256 × 256 grid, the Fourier neural operator has an inference time of only 0.005s compared to the 2.2s of the pseudo-spectral method." In the Bayesian inverse problem, MCMC requires "30,000 evaluations of the forward operator," which take "2.5 minutes" with FNO against "over 18 hours" with the solver, and "accounting for data generation and training time (offline steps) which take 12 hours, using FNO is still faster."

## 5. The one idea, restated

Strip the PDE away and the paper says this. If the way one point affects another depends only on the distance between them, then the whole pattern of influence is a convolution, and a convolution is diagonal in the Fourier basis. So do not learn the pattern of influence point by point. Learn one complex number per frequency, or one small complex matrix per frequency if the state has several channels. Keep only the low frequencies, because the fields that matter are smooth enough that the high ones carry little. Between such layers, apply a pointwise nonlinearity and a pointwise linear map, so that the composition can bend and can see edges.

Three consequences follow, and the paper tests each.

First, a weight per frequency is a weight per physical scale, and a physical scale does not change when the grid is refined. So the same weights serve at 64 × 64 and at 256 × 256. That is the resolution invariance in the contributions, and Figure 1 is its demonstration.

Second, the cost is the cost of an FFT plus a small multiplication, quasi-linear in the points. That is where the "three orders of magnitude" comes from: 0.005 s against 2.2 s on one 256 × 256 instance.

Third, the restriction to a convolution is a strong prior, and priors help when they are true. On Darcy, where the two earlier papers' learned kernels lost to a reduced basis, the FNO wins at 0.0098 to 0.0109 against RBM's 0.0244 to 0.0259. On Burgers, where the earlier kernels had won, the FNO wins by more. On turbulent Navier-Stokes at ν = 1e-5, where nothing has an easy time, FNO-2D reaches 0.1556 against U-Net's 0.1982.

The paper is honest about what the prior costs. The convolution cannot depend on a. The kernel in the earlier papers took a(x) and a(y) as inputs. The Fourier kernel cannot. The paper does not hide this and says the dependence on a enters through the lifting and through the iterated nonlinearity. That is the price of the FFT.

For Phient the restatement is one sentence. A function has scales, a relation between two places that depends only on their separation is one weight per scale, and one weight per scale is the same operator at every resolution. Phinet's operator was built on this sentence. The question this paper puts to Phical is whether a word's relation channels are a domain on which separation means anything. Section 13 says that the honest answer is that phinet's FFT runs along an axis that has no metric, and that the operator's measured gains cannot yet be attributed to the convolution prior.

## 6. The word as the input function

In the paper the input is a function a on a domain, lifted pointwise to a state v0 = P(a) of width d_v, and the output is a function u on the same domain, projected pointwise from the last state, u = Q(v_T). The domain is a unit interval or a unit square. Between P and Q every layer keeps the domain and changes the values.

In Phical the input is a word. A word is 32–64 relations, each a complex phasor on a closed circle with a count. The phinet Tensor that holds it has shape [b, w, s], complex, where s is the channel axis. Read that as the paper reads its input. The domain is the channel axis. The value at each point of the domain is a phasor. The lifting P is what the Trainer does when it builds a word's function from its relations. The projection Q is the reading, the nearest-k or the softmax that turns a function back into a ranking over the pool.

The paper's state v_t is real and has d_v channels at each point. Phical's state is already complex and has one value per point. So a phinet word is a d_v = 1 complex field, and the paper's channel-mixing matrix R_k of shape d_v × d_v collapses to a single complex weight per mode. That is what phinet's Fno tie is. The two earlier papers had a kernel with position and coefficient as inputs. This paper drops position in favour of separation and drops the coefficient. Phinet's tie drops both and keeps one complex number per mode. The reduction is exact, and it says that phinet's operator is the paper's method at its narrowest setting.

There is one place where the analogy adds something the paper's input does not have. Every Phical relation has a count. The paper's a(x) is a value. It has no evidence weight. A count is how often the pair was seen under the condition, and it is available to the operator as a second field on the same domain, if the operator reads it. Nothing in the facts file says the Austen operator reads counts. This page does not claim it does.

The paper also has a stronger notion of what a function is than Phical has. Its functions are elements of Banach spaces, and its claim of learning "mappings between infinite-dimensional spaces of functions" rests on the input being samplable at any resolution. A word with 64 channels is a function on 64 points. Whether it is a sample of something with a continuum underneath, so that 32 channels and 64 channels are the same word at two resolutions, is what the paradigm sentence's "32–64" promises and what nothing has measured. The paper's method depends on that promise being true. Section 12 is built to test it.

## 7. The mode as a channel, the channel as a coordinate

The paper's central object is R, a complex tensor of shape k_max × d_v × d_v. Its first index is a Fourier mode. Its last two mix channels. The paper keeps k_max,j = 12 or 16 modes per dimension and drops the rest. A mode is a coordinate on which the operator acts, and the operator is diagonal across modes: the weight at mode k touches only the transform at mode k.

Phical has two things it calls coordinates, and the paper forces us to say which is which.

The first is the channel. A word is 32–64 relations, and a relation is a channel. Under the FFT along the channel axis, a channel is a point in physical space and a mode is a frequency across channels. So the paper's mode is a pattern across relations, and its weight per mode says how a pattern across relations propagates to the next word. The channel itself is the domain point, the x in a(x).

The second is the relation kind. The quest oracle has five kinds, is-a, part-of, near, used-for, unlike, and the measured fact is that a relation lives on its channel and nowhere else. Within kind 0.747 MRR. Wrong channel 0.015. Offset reading 0.005. That is the paper's "under what conditions" said in numbers. It is also something the paper's method cannot express. A convolution is the same at every point of the domain. If channel 7 is is-a and channel 12 is part-of, and they behave differently, that is a dependence on x that a translation-invariant kernel cannot see. The paper's answer is W, the pointwise linear map that "keeps the track of non-periodic boundary." In phinet's terms W is the only part of the layer that can tell one channel from another.

So the paper hands Phical a decomposition. The Fourier path carries what is the same across channels, the way a pattern of relations moves as a whole. The local path W carries what differs by channel, the kind. The quest's wrong-channel number, 0.015, says the second part is nearly everything for the quest. The Austen operator, 30.7× chance on held-out ranking with a wide and deep softmax tie at 50,016 params, says the first part carries something on prose. Which part of the Austen gain is Fourier and which is local has not been measured, and section 12 asks for that.

There is a third reading, and it is the sphere. phinet::Tie::Sphere reads the 64 channels as spherical harmonics Y_l^m with l < 8 and learns one complex weight per degree l. A degree is a mode on the sphere in exactly the sense that k is a mode on the torus. The sphere tie is this paper's R with the FFT replaced by the spherical harmonic transform and with the weight shared across order m within a degree. It has 1,192 params against the circle's 8,360. On the synthetic degree-filter task the sphere reaches MRR 1.000 against the circle's 0.662. That is the one measurement in gemphi of a weight per mode doing what this paper says a weight per mode does.

## 8. The Fourier layer as X ⇒ Y under c

The paper's layer is v_{t+1} = σ(W v_t + F⁻¹(R · F v_t)). Read as an operator it takes a function on the domain and returns a function on the domain. Four of them, between P and Q, make the map a ↦ u. Where is c? In the paper it is the coefficient a, and it enters only at the lifting, v_0 = P(a). After that every layer is the same regardless of a. This is the paper's largest departure from the earlier papers, whose kernel took a at both ends.

Phinet's operator has the same outer shape. Its Fno layer takes a word's function on the channel axis and returns another, with the nonlocal part done as a unitary FFT, learned complex weights on the retained modes, inverse FFT, plus a local path and a nonlinearity. It is trained to move one word's function toward the next word's function. So X is the current word, Y the next, and the operator is one map, learned across the corpus. The Austen measurement is this operator against chance MRR 0.0063 on a pool of 1,228 words seen 20 or more times and 204,592 training examples. Identity 7.9×. Hinge at 8,360 params 14.7× valid and 14.6× test. Softmax at β = 8 18.0× and 18.8×. Softmax wide and deep at 50,016 params 27.6× at 3 epochs, 30.7× valid and 30.3× test at 8 epochs, hit@10 0.348.

The condition is what Phinet does differently, and this paper shows how narrow the difference is. In the paper c enters once, through P. In phinet's Austen operator there is no separate c at all. The condition is the corpus. One operator is trained on three Austen novels and that is the condition under which X ⇒ Y. The paper's method and phinet's agree that the operator itself does not see c layer by layer.

The quest is where c is explicit, and the quest does not use a Fourier layer. Its trainer moves one word toward one target on one channel, the channel being the kind. That is a condition entering not through P but through the choice of domain point. It is closer to the paper's W than to its R. The quest's attraction-only trainer with dithered rounding reaches 0.594 MRR, hit@1 34.8%, hit@5 97.4% on seen rows against chance 0.021, and the shipped trainer reaches 0.082. Neither number involves an FFT.

So the family splits along this paper's own seam. Phinet's Austen operator is the paper's Fourier path with the condition folded into training. The quest is the paper's local path with the condition as a channel. The paper says a layer needs both. The Austen operator has both, R and W, and the quest has only W. What has not been measured is whether the Austen operator's gain over identity, from 7.9× to 30.7×, sits in R or in W. Phinet's wide and deep tie at 50,016 params against the hinge at 8,360 says a larger tie is better, and says nothing about W alone.

## 9. Where the sphere, the tensor and the loop land

The sphere first. The paper's transform is the FFT on a torus. Its later descendants moved to spherical harmonics, as the profile records for FourCastNet. Phinet already has that step as a tie. phinet::Tie::Sphere reads 64 = 8² channels as Y_l^m with l < 8 and keeps one complex weight per degree, 1,192 params. On Austen at hinge and 3 epochs it reaches 12.8× valid and 13.1× test with hit@10 0.182 in 89 s, against the circle tie's 14.7× and 14.6× at 8,360 params in 142 s. On the synthetic degree-filter task, 200 items and 40 epochs, the sphere reaches MRR 1.000 against the circle's 0.662 with identity at 0.646. The sphere is this paper's method on a different manifold, with a seventh of the parameters, and it is slightly worse on prose and exact on the task built for it. The sphere at softmax and at wide and deep is unrun.

The tensor next. The paper's R is k_max × d_v × d_v complex, with d_v = 32 or 64. Its Table 1 counts FNO-3D at 6,558,537 params and FNO-2D at 414,517. Phinet's Tensor is [b, w, s] complex with s the channel axis, and its ties are one weight per mode. There is no d_v × d_v mixing at any mode, because a Phical word has one complex value per channel and nothing to mix. The paper's tensor is the thing phinet does not have, and section 12 does not propose adding it, because the paradigm sentence says a word is one phasor per relation.

The paper's FNO-2D is run "recurrently" in time: it "maps the solution at the previous 10 time steps to the next time step" and is applied again to its own output. That is a loop with no reader. The state at step t + 1 is a function of the states at t − 9 to t and of nothing else. The paper prefers FNO-3D, which does no loop and maps the initial steps to the whole trajectory at once.

Phiano's generation is a loop with a reader. The operator proposes a next-word function, the reading turns it into a ranking, a sampler chooses, and the chosen word is the next X. On 40 held-out prompts of 30 tokens, the trigram sampler reaches Kneser-Ney perplexity 13.3, the operator nearest-k 66.9 (was 97.2), and the union 66.8. Shared 4-grams across prompts are 25.7% for the sampler and 8.7% for the operator. The paper's recurrent form has the shape of phiano's loop without the reader. Its finding that the recurrent form is the harder one to train is not the same measurement as the perplexity gap, and this page does not say it is.

The quest's agent loop is absent from the paper, whose four layers are a fixed depth with no reader between them. The helix lands nowhere in this paper. The paper has no hierarchy, no tree and no code. The helix's sphere degree and order per codon is the only place the two touch: the helix stores, per codon, the degree that the sphere tie learns a weight for. That is a shared label and not a shared operation.

## 10. The kept modes as the deadband, and the byte

The paper keeps k_max,j = 12 or 16 modes per dimension and discards the rest. It says the truncation is what makes the cost O(n k_max) rather than O(n²), and it says nothing is lost that matters, because the fields it learns on are smooth. That is a claim about where the information in a function lives. It lives at low frequency.

Phical has a claim about where the information in a channel lives, and it is at the other end. A channel is one byte, 256 quanta of 2π. The nudge 0.06·sin d falls under half a quantum once sin d < 0.2, at 11°, and nearest rounding froze every pair 8 quanta short. Dither costs corpus attribution 71% to 65% and ships off by default. That is a floor on the smallest change a channel can hold, and the paper's truncation is a ceiling on the finest pattern the operator can see. They are different limits on different axes. The byte is a limit on the value at one point of the domain. The truncation is a limit on the frequency across the domain.

The paper offers a way to think about what the byte does to the operator. A phasor rounded to a byte is a function on the channel axis plus a quantisation error, and the error is white across modes if the rounding is unbiased. Dither is what makes it unbiased. So an FFT of a dithered word sees the true low modes plus a flat noise floor, and an FFT of a nearest-rounded word sees the true low modes plus a floor that is correlated with the signal and can freeze it. The paper's Fourier layer, keeping only low modes, is the kind of filter that would pass the true signal and drop most of a white floor. That is a reason, in the paper's terms, that the operator might tolerate dither well. It has not been measured. The dither cost of 71% to 65% was measured on corpus attribution, which reads the sector and not the modes.

The paper's other economy is that R is small when k_max is small. Phinet's ties are already small by the paper's standard, 8,360 params for the circle at hinge and 1,192 for the sphere, against the paper's 414,517 for FNO-2D. The engine's measured cost is not in the tie. Phinet is allocation and bandwidth bound at these sizes, with a 1 MB op at 2–5 ms. The paper's O(n log n) is about points on the grid. Phinet's cost is about bytes moved, and at 64 channels the FFT is not where the time goes.

What the paper does say, and what carries, is that the number of kept modes is a hyperparameter with a meaning. Keep fewer and the operator is smoother and cheaper. Keep more and it can see sharper patterns across channels. Phinet's FnoConfig has a modes field. The facts file records no sweep over it. That is a gap this paper names.

## 11. What Phient has already measured that bears on it

Nothing in gemphi solves a PDE, and no Phient number is a relative L² error on a grid. What follows is what has been measured that touches this paper's claims, and only that.

On the claim that the architecture is correct as an implementation of the Fourier layer, phinet has 62 of 62 parity tests against a PyTorch reference. This says phinet's primitives, including the unitary FFT and the Tape autograd, agree with the reference. It says nothing about whether the operator learns.

On the claim that a Fourier operator learns a useful map, the Austen operator is the measurement. Pool 1,228 words seen 20 or more times, 204,592 training examples, chance MRR 0.0063. Identity 7.9× chance. Hinge at 8,360 params 14.7× valid and 14.6× test. Softmax at β = 8 18.0× and 18.8×. Softmax wide and deep at 50,016 params 27.6× at 3 epochs, 30.7× valid and 30.3× test at 8 epochs, hit@10 0.348. The gain from identity to the best tie is the gain that the Fourier layer plus its local path plus training gives on prose. The split between the Fourier path and the local path has not been measured.

On the claim that a weight per mode is the right shape for an operator, the sphere tie is the nearest measurement. One complex weight per spherical-harmonic degree, l < 8, 1,192 params. Austen hinge 3 epochs 12.8× valid and 13.1× test, hit@10 0.182, 89 s, against the circle's 14.7× and 14.6× at 8,360 params in 142 s. Synthetic degree filter, 200 items and 40 epochs: sphere 1.000, circle 0.662, identity 0.646.

On the claim that the operator can be run recurrently to generate, decoding is the measurement. 40 held-out prompts by 30 tokens: trigram sampler Kneser-Ney perplexity 13.3, operator nearest-k 66.9 (was 97.2), union 66.8. Shared 4-grams across prompts: sampler 25.7%, operator 8.7%. The operator is worse on perplexity and less repetitive.

On the claim that the condition enters once, through the lifting, and not through the kernel, the quest is the counter-measurement. Within kind 0.747, wrong channel 0.015, offset reading 0.005. Held-out 20%: one hop 0.011, two hops is-a 0.095 and used-for 0.093, part-of, near and unlike about chance 0.021. The quest's condition is a channel and its whole result depends on which channel. That is the paper's W and not its R.

On the byte, the deadband. One byte per channel, 256 quanta. Nudge under half a quantum at sin d < 0.2. Nearest rounding froze every pair 8 quanta short. Dither 71% to 65%, off by default.

On cost, phinet is allocation and bandwidth bound at these sizes, 1 MB op 2–5 ms.

On the claim of zero-shot super-resolution, nothing. No Phient operator has been trained at one channel count and read at another. On the claim of an inverse problem, nothing. Persona attribution at 71% and dialogue at 44% and 72% were measured with no operator in the loop and do not bear on this paper.

## 12. The experiment: the same word at 32 and at 64 channels, with the local path as control

The paper's headline is that a weight per mode is a weight per scale, so the operator trained on one grid works on a finer one. The paradigm sentence says a word is 32–64 relations. Those two claims meet in one experiment, and it has not been run.

The data is Austen as it stands: Emma, Persuasion, Sense and Sensibility, pool of 1,228 words seen 20 or more times, 204,592 training examples, chance MRR 0.0063, the same valid and test splits as the facts file.

The treatment is the paper's Figure 1 on words. Build every word's function at 32 channels and at 64 channels. The 32-channel word must be a sample of the 64-channel word, in the paper's sense: the same relations at half the resolution, with the 64-channel function recoverable from the 32 by projecting onto the Fourier basis as the paper describes. If the Trainer cannot build words that way, the experiment stops there and the finding is that Phical's 32 and 64 are two representations and not two resolutions. That finding would be worth having.

If it can, train the circle tie at hinge on the 32-channel words with the same modes count that the 64-channel run uses, so that the kept modes are the same physical scales. Read the trained tie on the 64-channel words with no further training. Measure valid and test MRR against chance and hit@10.

The first control is the 64-channel run as shipped, 14.7× valid, 14.6× test, 8,360 params. The paper predicts the 32-to-64 transfer should land near it, as its Darcy row sits at 0.0108 to 0.0098 from s = 85 to 421.

The second control is what the paper's Table 1 cannot separate and the Austen table cannot either: the local path alone. Train a tie with the Fourier path removed, W only, at 64 channels, same epochs and same loss. Its distance from 7.9× says what the pointwise path alone gives. Its distance from 14.7× says what the Fourier path adds. The 32-to-64 transfer should be tried on this tie too. The paper says W is what tracks the non-periodic boundary, and a W trained at 32 channels has no 64-channel form unless it is pointwise in a way that survives resampling. If the W-only tie does not transfer and the full tie does, the paper's claim is confirmed in the one place Phical can test it.

A third arm is the modes sweep. Hold 64 channels and vary the modes field of FnoConfig from a few to all. The paper's claim that smooth fields live at low frequency has a word version: if MRR holds as modes fall, a word is smooth across its channels and the paper's economy applies. If MRR collapses as soon as modes fall below the channel count, a word is not a smooth field and the FFT is a basis change and nothing more.

The measurements are the Austen ones: MRR as a multiple of chance, hit@10, valid and test separately. Dither is off in every arm, as shipped. The sphere tie should be run in the same arms if time allows, since it is unrun at softmax and wide and deep in any case.

What this cannot test is the paper's speed claim. Phinet is bandwidth bound at these sizes and the FFT is not the cost.

## 13. Where we disagree, and where the analogy breaks

The analogy breaks in three places and we disagree with the paper in one.

The first break is the domain. The paper's FFT runs along a spatial axis where distance means something, and the convolution assumption κ(x, y) = κ(x − y) says the influence of one point on another depends on their separation. Phinet's FFT runs along the channel axis. Whether channel 7 is any nearer to channel 8 than to channel 40 is not a fact about Phical. It is a fact about how the Trainer assigns relations to channels, and the facts file records no assignment rule that makes the channel axis a metric space. The sphere tie is the one place where the axis has a geometry, because the 64 channels are read as Y_l^m and the degree is a scale on a real sphere. On the circle, the paper's central assumption is unexamined. The Austen operator may be gaining from the FFT as a basis change and from the local path, and not from any convolution structure at all. Section 12's second control is there to say which.

The second break is the value. The paper's state is real, of width d_v, and R mixes channels at each mode with a d_v × d_v complex matrix. Phical's state is one phasor per channel, on a closed circle, quantised to a byte. There is nothing to mix at a mode, so R collapses to a scalar. The paper's expressiveness lives in that matrix; its FNO-3D has 6,558,537 params, most of them in R. Phinet's ties are three to four orders of magnitude smaller and have no matrix. The paper's results say nothing about whether a scalar-per-mode operator can do what its matrix-per-mode operator does.

The third break is the byte. The paper's fields are floating point on a V100. Phical's channel is one of 256 values, and the nudge falls under half a quantum at sin d < 0.2. The paper never meets a deadband. Its resolution invariance is a statement about a continuum underneath the grid. The byte is a floor on the value and not on the grid, so the two are on different axes, and section 10 says why they might coexist. That is a reason and not a measurement.

The disagreement is about the condition. The paper moves a from the kernel to the lifting. Its kernel sees no condition, and the condition enters once, at v_0 = P(a). The paper's tables say the loss from dropping it is more than repaid by the FFT. We think that is true on Darcy and on Burgers and doubt it on words. The quest's numbers are the reason. Within kind 0.747, wrong channel 0.015. The condition on a word selects the channel, and an operator that ignores which channel it is on reads at chance. The paper's W can carry that, and only W can. So on words we would expect the local path to do more of the work than it does on a PDE, and the Fourier path less. We hold this on the quest alone, which has no Fourier layer, and on no Austen measurement that separates the two paths. Section 12 would settle it. Until it is run, this is a view and not a finding.

## 14. The person and the moment

The paper was posted to arXiv on 18 October 2020, revised on 20 March 2021 and again on 17 May 2021, and appeared as a poster at ICLR 2021. It is the third neural operator paper of that year from the same seven authors, after the graph kernel network in March and the multipole operator in June. Her name is last on all three. She was two years into her two posts, Bren Professor at Caltech and senior director of machine-learning research at NVIDIA, both from 2018, and co-founder of Caltech's AI4Science initiative.

This is the paper the profile calls the idea that changed weather. The profile's account of it is short: "FFT, keep the low modes, multiply by learned complex weights, inverse FFT, add a local linear path, nonlinearity, repeat." It is "resolution-invariant by construction and, for turbulent flows, orders of magnitude faster than the solvers it learned from." Quanta profiled the work in 2021. In February 2022, FourCastNet put an adaptive FNO on the whole planet at 0.25° resolution, 20 prognostic variables, a week's forecast in under two seconds, roughly 45,000× faster than numerical weather prediction on a node-hour basis and 12,000× less energy than IFS. Later versions moved to spherical harmonics. That whole line runs through this paper's Definition 3.

The year is where her two decades meet. Her first decade was tensor methods, moments as tensors and decompositions with guarantees, the NSF CAREER Award in 2013 and a Sloan Fellowship in 2014. This paper's central object is a tensor R of shape k_max × d_v × d_v, and the paper's whole gain is a structural restriction on it, diagonal across modes, that makes it small. The move is the one she has since described as her method: "All of the things that work with deep learning, let's take them, but make them a bit more principled." Convolutions worked. The convolution theorem is principled. The paper joins them and lets the FFT do what the message passing had done at quadratic cost.

Her later argument follows from here. "We have foundation models for language, not for physics." Transformers, she has said, cannot be the substrate for physics at scale, because a few hundred grid points per dimension is a context of hundreds of billions to a trillion tokens, and an operator in function space treats scale as what it is. Figure 1 of this paper, trained at 64 × 64 × 20 and evaluated at 256 × 256 × 80, is that argument's first picture.

The recognition came later: Fellow of the IEEE in 2021, the Blavatnik Award in 2024, the IEEE Kiyo Tomiyasu Award in 2025, the Time100 AI list in 2026, and on 16 March 2026 her appointment to the UN Scientific Advisory Board. In October 2020 she was a professor and a research director posting a preprint. This paper does not claim she has read anything of Phient's. No partnership is agreed with anyone. A partnership is something to earn with a result she would recognise, and the result this paper points to is section 12: an operator over a function space she has not seen, trained at one resolution and read at another, with the control that says why it worked.

## 15. What remains

Everything in section 12 remains. No Phient operator has been trained at 32 channels and read at 64. Whether a Phical word at 32 channels is a sample of the same word at 64 has not been asked of the Trainer. The paper's headline is untested on words, and it is the cheapest test in this series: the Austen data exists, the circle tie exists, and the only new code is a resampling of words and a W-only tie.

The split between the Fourier path and the local path remains unmeasured. Every Austen number is the full layer. The paper's own Table 1 does not separate them either. On words the split matters more than on a PDE, because the quest says the condition lives on the channel and only W can see it.

The modes sweep remains. FnoConfig has a modes field and the facts file records no run that varies it. The paper's economy, that smooth fields live at low frequency, has a one-line word version and it has not been run.

The channel axis as a metric space remains an assumption. On the circle, no rule in the facts file makes channel 7 nearer to channel 8 than to channel 40, and without one the convolution prior is a basis change. The sphere tie has a geometry and is the honest place to make the paper's argument. It is unrun at softmax and at wide and deep.

The tensor R remains absent. Phical has one phasor per channel and nothing to mix at a mode. Whether that is enough is a question this paper cannot answer, because every one of its models mixes.

The byte remains. The paper's fields are floating point. Phical's are bytes with a deadband at sin d < 0.2, and dither costs 71% to 65% on attribution. Section 10 gives a reason that a low-mode filter might tolerate dithered bytes well. It is a reason and nothing more.

The paper's speed claim remains out of reach for the usual cause. Phinet is allocation and bandwidth bound at these sizes, 2–5 ms per 1 MB op, and the FFT is not where the time goes. The paper's 0.005 s per instance is on a V100 on a 256 × 256 grid. The two are not comparable and this page has not compared them.

No weather work exists in gemphi and none is proposed by this paper. No Phi-4 run exists; the quest is a synthetic oracle of 256 words. No PDE has been solved. No partnership is agreed with anyone.

This paper is one of sixteen readings. It states what the paper says, from the paper, and what Phient has measured, from the facts file, and it draws one line between them. The line is that phinet's operator is this paper's layer at its narrowest setting, a scalar per mode on a one-channel complex field, and that the paper's two claims, invariance to resolution and the economy of low modes, are both cheap to test on words and both untested. Section 12 is the test.

## 16. Sources

Li, Z., Kovachki, N., Azizzadenesheli, K., Liu, B., Bhattacharya, K., Stuart, A., Anandkumar, A. "Fourier Neural Operator for Parametric Partial Differential Equations." arXiv:2010.08895, v1 submitted 18 October 2020, v2 20 March 2021, v3 17 May 2021. Abstract page: https://arxiv.org/abs/2010.08895 and https://arxiv.org/abs/2010.08895v3. PDF: https://arxiv.org/pdf/2010.08895. All three were fetched for this paper. The abstract, author list and version history were read from the abstract page. Definitions 1 and 3, the discrete form of the R multiplication, the complexity sentences, the lifting and projection sentences, the convolution sentences, the boundary sentence about W, the training hyperparameters, the hardware sentence, the three benchmark definitions, Tables 1, 3 and 4, the Figure 1 caption, the super-resolution and speed sentences, the Bayesian inverse problem numbers, the recurrent and 3D distinction, and the conclusion were read from the PDF through a text extraction. The paper's "up to three orders of magnitude" and "consistently outperforms" phrases are quoted from the abstract and contributions as extracted. The extraction gave k_max,j = 12 in one place and k_max,j = 16 with d_v = 64 for the 1D problem and k_max,j = 12 with d_v = 32 for the 2D problems in another; both are reported in section 3 as given. Table 2 of the PDF, which the paper uses for the Navier-Stokes super-resolution errors, was not recovered by our extraction and no number from it is stated. The subject categories on arXiv are cs.LG and math.NA.

ICLR 2021 poster page: https://iclr.cc/virtual/2021/poster/3281, which confirms the title, the venue as an ICLR 2021 poster, and the seven authors in the order used in the front matter. The OpenReview forum for the paper was not reachable from this session and is not cited.

The ar5iv rendering was not fetched for this paper.

Every number about Phient is from PHIENT_FACTS.md in this folder. Every fact about the author is from 17_ANIMA_ANANDKUMAR_PROFILE.md in the parent folder, whose own sources are listed there: the Caltech News release of 16 March 2026, the Latent Space interview, the Wikipedia article, the FourCastNet preprint arXiv:2202.11214, the Blavatnik Awards profile, and the Caltech CMS faculty page. The third and fourth papers in this series, 03_graph_kernel_network.md and 04_multipole_graph_operator.md, were read for continuity; their Darcy and Burgers tables are extended by this paper's Tables 3 and 4, which add LNO and FNO rows and, on Burgers, begin at resolution 256 rather than 64.

Not consulted: the group's later journal treatment of neural operators, the FourCastNet paper beyond what the profile says of it, the spherical Fourier neural operator, the Quanta article, or any secondary account of this paper. The reading of the channel axis as a domain without a metric in section 13, the split of the layer into a Fourier path and a local path as the carriers of pattern and of kind in section 7, and the dither argument in section 10 are our framing, not the paper's.
