# The law as a loss: a residual on the relation circle

*Phient interpreted papers, 7 of 16 · 2026-04-06 · on: "Physics-Informed Neural Operator for Learning Partial Differential Equations" (Li, Zheng, Kovachki, Jin, Chen, Liu, Azizzadenesheli, Anandkumar, 2021, arXiv:2111.03794; published in ACM / IMS Journal of Data Science, 2024, doi:10.1145/3648506)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the seventh of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. The earlier papers in this series followed the operator from the graph kernel to the multipole hierarchy to the Fourier transform. Each of those papers learned the operator from data alone. Pairs of input and output functions came from a numerical solver, and the network was asked to match them. This paper asks what happens when the solver is not the only teacher. The equation itself can be a teacher. If the operator's output is a function, the equation can be applied to that function, and whatever is left over is an error the operator can be trained to reduce.

The paper calls the result the physics-informed neural operator, PINO. It was posted to arXiv on 6 November 2021 and revised three times, the last on 29 July 2023. It was published in the ACM / IMS Journal of Data Science in February 2024. The core of it is one loss with two terms. One term compares the operator's output to data. The other term applies the differential equation to the operator's output and measures the residual. The two terms need not live at the same resolution. The paper's first claim is that the data can be coarse and the physics can be fine, and the operator learned from both is better than the operator learned from either.

We read it here for one reason above the others. Phient's operator is trained only on data. The Austen operator in the facts file sees 204,592 examples of one word followed by another and learns to move the first toward the second. Nothing in that training says what a word must satisfy to be a word. The quest oracle is the closest thing gemphi has to a law: it says which relations hold, in which channel, and it is used as data, not as a constraint. This paper shows how a law becomes a loss. It also shows that the loss can be evaluated on instances for which there is no data at all, because the law does not need an answer key. That is the idea we want to carry across.

Sections 2 to 5 state what the paper says, in its own terms, with its numbers. Sections 6 to 10 place the paper's objects inside the Phient family: the residual as a relation that must close, the Fourier derivative as a rotation on the relation circle, the two resolutions as 32 and 64 channels, the fine-tuning phase as a condition, and the inverse problem as the quest read backwards. Section 11 lists what Phient has measured that bears on it. Section 12 names one experiment and its control. Section 13 says where the analogy breaks.

One limit is stated at the top. Phient has no physics loss. No trainer in gemphi differentiates an operator's output and penalises a residual. Nothing here should be read as a result. It is a reading.

## 2. What the paper claims, in its own terms

The abstract states the aim in its first sentence. "In this paper, we propose physics-informed neural operators (PINO) that combine training data and physics constraints to learn the solution operator of a given family of parametric Partial Differential Equations (PDE)." The object learned is an operator, a map from one function to another, for a whole family of equations indexed by a parameter. The family might be Burgers' equation over many initial conditions, or Darcy flow over many coefficient fields.

The paper places itself against two neighbours. The first neighbour is the data-driven neural operator, the Fourier neural operator of the group's earlier work. That operator needs solved instances, and solved instances are expensive. The second neighbour is the physics-informed neural network, PINN, which fits one solution to one instance by penalising the equation's residual at sampled points. PINN needs no data, but the paper says it fails on hard problems, "e.g., in multi-scale dynamic systems such as Kolmogorov flows," and it fails "due to optimization challenges."

The contribution list has five items. The first is the hybrid. "PINO is the first hybrid approach incorporating data and PDE constraints at different resolutions to learn the operator." The second is what the hybrid buys. The model "shows no degradation in accuracy even under zero-shot super-resolution, i.e., being able to predict beyond the resolution of training data." The third is that PINO "succeeds in settings where no training data is available and only PDE constraints are imposed," where PINN does not. The fourth is a way of computing derivatives. Because the operator is a Fourier neural operator, its output is a function whose derivative can be taken in Fourier space exactly, without differentiating point by point through the network. The fifth is an inverse problem, recovering the coefficient of Darcy flow from its solution, which the paper says runs about 3000 times faster than a Markov chain Monte Carlo baseline.

The paper also describes a second use of the operator once it is learned. The learned operator can be taken as an ansatz for a single instance and refined further against the equation alone. The paper calls this instance-wise fine-tuning, and reports that it lowers error where the plain operator plateaus. A third setting, test-time optimisation, does the same refinement at inference. The paper's own summary of what this buys is that PINO trades computation for accuracy on demand.

## 3. The mathematics, plainly

The operator is the Fourier neural operator of the group's 2020 paper. Written out, it is G_θ = Q ∘ (W_L + K_L) ∘ ... ∘ σ(W_1 + K_1) ∘ P. P lifts the input function to a wider set of channels. Each layer adds a pointwise linear map W and an integral operator K, then applies a nonlinearity. Q projects back to the output. The integral operator is a convolution done in Fourier space: (K v)(x) = F⁻¹(R · (F v))(x), where R is a learned complex weight on each retained mode.

The data loss is the one from the earlier papers. Over N instances, J_data ≈ (1/N) Σ_j ∫_D |u_j(x) − G_θ(a_j)(x)|² dx. It needs a solved u_j for every a_j.

The physics loss is new. For a stationary equation written as P(u, a) = 0 on a domain D with boundary condition u = g on ∂D, the paper defines L_pde(a, u_θ) = ∫_D |P(u_θ(x), a(x))|² dx + α ∫_∂D |u_θ(x) − g(x)|² dx. For a dynamic equation du/dt = R(u), there is a residual in time and space, a boundary term, and an initial-condition term weighted by β. Averaging L_pde over instances gives J_pde. Nothing in J_pde needs a solved instance. Only the input a and the equation are needed. The paper uses this to draw "unlimited virtual PDE instances by drawing additional initial conditions or coefficient conditions a_j ∼ μ."

The derivative is where the paper is careful. It names three ways to get du/dx. Numerical differentiation on a grid is fast, O(n log n) with the FFT, but carries truncation error. Pointwise automatic differentiation through the network is exact but slow, because "the number of parameters |θ| is usually much greater than the grid size n." The third way is the paper's own. The operator's last hidden function v_L is a finite Fourier series, so its derivative is exact in mode space: multiply mode k by i·2πk/D. The output derivative is then u′ = Q′(v_L) · F⁻¹(i(2π/D)K · (F v_L)). The paper's sentence is: "Therefore, to exactly compute the derivative of the Fourier neural operator, one just needs to run the numerical Fourier differentiation." Higher derivatives follow by the chain rule.

For fine-tuning, the loss is L_pde(a, G_θ(a)) + α L_op(G_θ_i(a), G_θ_0(a)). The second term is an anchor. The paper says it is there "to bound the further fine-tuned model from the learned operator model," and that "a hard constraint such as the anchor loss L_op relieves such a problem," the problem being drift under the residual alone.

## 4. Its experiments and numbers as reported

The paper tests on three equations. Burgers' equation in one dimension with viscosity ν = 0.01, learning the map from initial condition to solution over t ∈ (0, 1]. Darcy flow in two dimensions, learning the map from coefficient a(x) to solution u(x) with zero boundary. The Navier-Stokes equations in vorticity form, in three variants: a long transient flow at Re = 20 over T ∈ [0, 50], a chaotic Kolmogorov flow at Re = 500, and a lid-driven cavity flow at Re = 500 with no-slip walls. Training inputs are drawn from Gaussian random fields; for Burgers the paper gives u₀ ∼ N(0, 625(−Δ + 25I)⁻²).

Table 1 is the paper's first result. It trains with data at a coarse resolution, adds the physics loss at a finer one, and tests at three resolutions. The numbers are relative L2 error.

| equation | training grid | PDE-loss grid | data only (1×, 2×, 4×) | data + PDE (1×, 2×, 4×) |
|---|---|---|---|---|
| Burgers | 32×25 | 128×100 | 0.32%, 3.32%, 3.76% | 0.17%, 0.28%, 0.38% |
| Darcy | 11×11 | 61×61 | 5.41%, 9.01%, 9.46% | 5.23%, 1.56%, 1.58% |
| Kolmogorov, Re 500 | 64×64×33 | 256×256×65 | 8.28%, 8.27%, 8.30% | 6.04%, 6.02%, 6.01% |

The pattern is the paper's point. With data only, error grows when the test grid is finer than the training grid. With the physics loss on the finer grid, error at 4× stays near error at 1×. The paper states the standard deviations as well, about 0.01% for Burgers and about 0.1% for the other two.

Against baselines on Darcy, the paper reports DeepONet with data at 6.97 ± 0.09%, PINO with data at 1.22 ± 0.03%, and PINO with no data at 1.50 ± 0.03%. On Burgers it reports PI-DeepONet at 1.38% and PINO at 0.38%. On Kolmogorov flow it varies the data: 0 data samples and 2,200 PDE instances give 6.22 ± 0.11%; 800 and 2,200 give 6.01 ± 0.12%; 2,200 and 2,200 give 5.04 ± 0.11%.

For the long transient flow at T = 50, the paper reports PINO without fine-tuning at 2.87% against FNO at 3.04%, PINO with fine-tuning at 1.84%, and a 400× speedup over a GPU pseudo-spectral solver. For Kolmogorov flow at Re = 500 and T = 0.5, Table 4 reports PINN at 18.7% error in 4,577 s and PINO at 0.9% in 473 s with 0.4k data and 160k PDE instances; the paper's summary is "20x smaller error and 25x speedup." Lid-cavity flow reaches 14.52% in two minutes.

The inverse problem recovers the Darcy coefficient. The paper's inverse model gives 2.29% error on u and 97.10% classification accuracy on a; the forward model gives 6.43% and 95.38%; the MCMC posterior mean after 500,000 steps gives 4.52% and 90.30%. The paper says PINN "does not converge in this case."

## 5. The one idea, restated

Strip the paper to one sentence. An operator's output is a function, and a function can be checked against a law without an answer key. Everything else follows from that sentence.

The data loss says: here is the answer, match it. The physics loss says: here is what any answer must satisfy, satisfy it. The first needs a solver to have run. The second needs only the equation. So the second can be evaluated on inputs the solver never saw, and at resolutions the solver never used. The paper's Table 1 is the visible consequence. Coarse answers plus a fine law give an operator that holds at fine resolution. Coarse answers alone give an operator that fails there.

The second half of the idea is about how the check is done. A physics-informed neural network checks its law at sampled points by differentiating the network. That works, but it is expensive and it is a bad optimisation problem on hard equations. The paper's operator is a Fourier series at its last layer, so the law's derivatives are exact and cheap in mode space. The check is done on the function, not on the network. The paper's phrase for this is function-wise differentiation, against the point-wise kind.

The third piece is the anchor. Once an operator is learned, it can be refined on one instance against the law alone. But the law alone can pull the operator anywhere that satisfies it, and for a hard equation that includes places far from the true solution. So the refinement is tied to where the operator started. The learned operator is a prior; the law is the likelihood; the anchor keeps the posterior near the prior.

For Phient the restatement is this. Our operator moves one word's function toward another's, trained on pairs. We have data. We have no law. The question this paper puts to us is whether there is a law for words at all, something a word's 32–64 relations must satisfy to be a word, that could be applied to the operator's output without an answer key. If there is, this paper says how to train against it, how to differentiate it cheaply on a circle, and why to anchor.

## 6. The residual as a relation that must close

Start with what a residual is in Phical terms. In the paper, P(u, a) = 0 says that u is not free: given a, u is determined, and the residual measures the distance from determination. In Phical, a word is determined by its relations. A word is 32–64 phasors with counts. What is the equivalent of an equation on those phasors?

The quest gives the only candidate we have measured. The oracle in phiecs has 256 words in families of 8 and five kinds of relation: is-a, part-of, near, used-for, unlike. A relation of kind k lives in a channel, and a target word sits at a phase in that channel. The facts file records that reading a wrong channel gives MRR 0.015 and reading the right channel gives 0.594 with attraction-only training and dithered rounding. So a relation is a phasor in its channel, and the channel is the coordinate.

Now think of a word as a function on the relation circle, one value per channel. The operator X ⇒ Y under c moves the function of X toward the function of Y. A law for that motion would be a statement that holds for any Y that is a legitimate result, whether or not a data pair was seen. The natural candidate is closure. If X is-a Y and Y is-a Z, the two-hop reading of X should land where the one-hop reading of Z lands. In the paper's notation, the residual for a dynamic equation is du/dt − R(u); for us it is the phase difference between the composed motion and the direct motion. That difference can be measured on any X, Y, Z, including triples with no oracle row.

This is what the paper means by virtual instances. The Darcy loss can be evaluated on a coefficient field a that no solver has ever solved. The closure loss can be evaluated on a triple that no oracle row has ever named. In both cases the law is applied to the operator's own output.

We do not claim this law is true of language. Is-a composes cleanly in an oracle built to compose. Near does not compose: near twice may be far. Unlike twice may be like. The facts file's held-out two-hop numbers are honest about this: 0.095 for is-a, 0.093 for used-for, and part-of, near and unlike at about chance. So the closure law is a law for some kinds and not for others, and any residual loss would need a kind-dependent weight, like the paper's α and β on boundary and initial terms.

## 7. The Fourier derivative as a rotation on the relation circle

The paper's most practical contribution is equation (15). The last hidden function of the operator is a Fourier series, so its derivative is exact: multiply mode k by i·2πk/D, inverse transform, and apply the chain rule through the projection Q. No automatic differentiation through the layers. The paper's sentence is that the exact derivative of the Fourier neural operator is obtained by running numerical Fourier differentiation on its output.

Phinet has the pieces of this and has not assembled them. Its tensor is complex, [b, w, s]. Its FFT is unitary. Its Fno holds learned complex weights on the retained modes. Its Tape does autograd. None of that is a derivative of the output with respect to its coordinate. It is a derivative of the loss with respect to the weights. The two are different objects, and the paper's point is that the first can be had without the second.

What would the coordinate derivative mean on the relation circle? A word's channel value is a phasor, a point on a closed circle with a count. The circle is periodic by construction. Differentiation on a periodic domain is exactly where the Fourier derivative is exact, because there is no boundary and no truncation from the domain's edge. Multiplication of mode k by i·k is a rotation of that mode by a quarter turn, scaled by k. On the sphere tie, where 64 channels are laid out as Y_l^m with l < 8, the equivalent operation is the angular derivative on the sphere, which is also diagonal in the harmonic basis up to a known coupling of orders. The facts file records only that the sphere tie exists and trains, 1,192 params at 12.8× valid and 13.1× test on Austen. It records no derivative.

The byte deadband bears on this. A channel is one byte, 256 quanta of 2π, and the facts file says a nudge of 0.06·sin d falls under half a quantum once sin d < 0.2, so nearest rounding froze every pair 8 quanta short. A derivative is a small difference. A derivative computed on a byte-quantised function would be quantised too, and small residuals would vanish under the same deadband that froze training. Any physics loss on the circle would have to be computed on the continuous phasor before rounding, or with the dither that the facts file says costs attribution 71% to 65%. This is a real constraint that the paper does not face, since its functions are floats on a grid.

Nothing above is measured. Phinet has no d/dx. The paper says how to build one for a Fourier operator, and the relation circle is a domain where that construction is clean.

## 8. Two resolutions as 32 and 64 channels

The paper's Table 1 is a table about resolution. Data at 11×11, physics at 61×61, test at 211×211. The operator does not care which grid it sees, because its weights live on modes, not points. The physics loss on the fine grid teaches it what the coarse data could not show.

Phical's word is 32–64 relations. That range is the closest thing in the facts file to two resolutions of one function. A word with 32 channels and a word with 64 channels are the same word sampled at two densities, if the channels are arranged so that the 32 are a subset of the 64 in mode space. The sphere tie makes this concrete: 64 channels as Y_l^m with l < 8 is a truncation at degree 8, and a 32-channel word would be a truncation lower. The circle tie is the same story in one dimension.

In the paper's terms, training data at 32 channels plus a law at 64 channels would produce an operator whose output at 64 channels is constrained by the law where the data is silent. Whether this helps depends on whether the higher modes carry meaning. The facts file has one number that speaks to this. The synthetic degree-filter task, 200 items over 40 epochs, gives the circle tie MRR 0.662 against identity at 0.646, and the sphere tie 1.000. That task was built so that the answer lives in a particular degree. The sphere tie, which has one weight per degree, finds it. The circle tie, which cannot address degree, does not. So there is at least one task where what mode a relation sits in is the whole task, and a law stated by mode would be a law the operator can obey.

The paper's super-resolution result has a second reading for us. The Austen operator was trained at one channel count and has been tested only at that count. The facts file records 30.7× valid and 30.3× test at 8 epochs for the wide and deep softmax model, 50,016 params, hit@10 0.348. It records no test at a different channel count, because none was run. The paper's claim is that a Fourier operator trained at one resolution and tested at another loses nothing, if a law holds it at the finer one. Phient has not tested the first half of that claim, resolution transfer without a law, let alone the second.

One more point about the two resolutions. The paper's coarse data comes from a solver; its fine physics comes from an equation. Ours would be reversed in cost. Data pairs in Austen are cheap; the corpus supplies 204,592 of them. A law would be expensive, because we would have to state it. The paper's economics, where the law is free and the data is dear, do not carry over. What carries over is the structure: two terms, two grids, one operator.

## 9. Fine-tuning as a condition

The paper's second setting is instance-wise fine-tuning. Take the learned operator G_θ. For one input a, keep optimising θ against L_pde(a, G_θ(a)) plus an anchor α L_op that measures distance from the original operator's output. The long transient flow goes from 2.87% to 1.84%. The operator becomes a prior for a per-instance solve.

In Phical the operator is X ⇒ Y under c. The condition c is what makes one operator many. In phiano the conditions we have measured are personas. The facts file records persona attribution at 71% overall across Chesterton, Austen and Carroll, with per-writer 74%, 65% and 86%, against chance 33%, on held-out four-sentence paragraphs. Dialogue replies were judged as their persona 44% by the sector reader and 72% by the Kneser-Ney panel.

Read the paper's fine-tuning as conditioning. The base operator is the operator over all three writers. Fine-tuning on one writer's law, whatever that is, with an anchor to the base, would be an operator under c = Austen. The anchor is what keeps the Austen operator an operator over English, and stops it from becoming an operator over nothing but the fine-tuning instances. The paper's warning is exact here: without the anchor, the residual alone "does not converge" on the hard case, and even where it converges it can drift to an unphysical solution. A persona without an anchor would drift to a caricature.

The paper's Table 4 is worth holding next to this. PINO from scratch, no data, no PDE instances beyond the one being solved, reaches 0.9% on Kolmogorov flow in 608 s. PINO with 0.4k data reaches the same 0.9% in 536 s. PINO with data and 160k PDE instances reaches 0.9% in 473 s. The error is the same; the time falls. The learned operator does not change the answer; it shortens the road to it. If the analogy holds, a persona learned as a condition on a base operator should reach the same attribution as a persona trained alone, faster. The facts file has no measurement of a persona trained from a base against a persona trained alone, so this is a prediction, not a result.

The test-time setting is the same mechanism at inference. A generated reply could be refined against a law before it is emitted. The facts file's decoding numbers are the baseline this would have to beat: trigram sampler at KN perplexity 13.3, operator nearest-k at 66.9, union 66.8, with shared 4-grams across prompts at 25.7% for the sampler and 8.7% for the operator. Test-time refinement against a closure law would be one more decoder, and it would need those two numbers reported the same way.

## 10. The inverse problem as the quest read backwards

The paper's last experiment recovers the coefficient a from the solution u. It offers two ways. The forward way optimises a guess â so that the learned forward operator applied to â matches u and â satisfies the equation. The inverse way learns a second operator F_θ that maps u to a, trained with the residual on (F_θ(u), u) and an anchor. The inverse model wins, 2.29% on u and 97.10% on a, against the forward model's 6.43% and 95.38%, and both beat the MCMC posterior mean's 4.52% and 90.30%.

The paper's stated reason for the physics loss here is direct: "The plain neural operator model, while accurate in the forward problem, is vulnerable under perturbation and shift of the input a, which is a common behavior of deep-learning models." The residual holds the recovered a to what the paper calls the physically valid manifold, P(a, u) = 0.

The quest in phiecs is a forward problem. Given a word and a kind, read the target's phase in the kind's channel. The facts file's numbers for that direction are 0.594 MRR, hit@1 34.8%, hit@5 97.4%, within-kind 0.747. The backward problem would be: given a target and a kind, which words point to it? Or, more like the paper's inverse: given a word's whole function, what family and what relations generated it? The oracle has 256 words in families of 8, so the generating structure is known and the inverse problem is well posed.

Nothing in the facts file measures the backward direction. Two hops at held-out 20% are the closest: one hop 0.011, two hops is-a 0.095, used-for 0.093, others at chance. Two hops is not an inverse, but it is a composition, and the paper's inverse model is trained with a composition too: F_θ(u) must satisfy the residual against u, which is the forward law applied to the inverse output.

The paper's numbers on this are the most striking in it. The MCMC baseline runs 500,000 steps. The inverse operator runs once. The claimed speedup is 3000×. Phient has no MCMC baseline for anything, and no inverse operator, so there is no number to set beside it. What there is, is a structure. The oracle is an equation whose forward operator we have half learned. The paper says the inverse operator is learned the same way, with the forward law as its residual and the forward operator as its anchor.

## 11. What Phient has already measured that bears on it

Everything in this section is from the facts file. Nothing new is measured for this paper.

The operator exists and matches its reference. Phinet against a PyTorch reference passes 62 of 62 parity tests. The Fno layer, the unitary FFT and the Tape autograd are the parts the paper's method would need, with one part missing: differentiation of the output with respect to its coordinate.

The operator learns from data. On Austen, with a pool of 1,228 words seen 20 or more times and 204,592 training examples against chance MRR 0.0063, identity gives 7.9× chance. The hinge model with 8,360 params gives 14.7× valid and 14.6× test. Softmax at β = 8 gives 18.0× and 18.8×. The wide and deep softmax model with 50,016 params gives 27.6× at 3 epochs and 30.7× valid and 30.3× test at 8 epochs, hit@10 0.348. All of this is data loss only. There is no second term.

The mode basis matters. The sphere tie, one complex weight per spherical-harmonic degree with 64 channels as Y_l^m for l < 8, has 1,192 params and gives 12.8× valid and 13.1× test on Austen hinge at 3 epochs, hit@10 0.182, in 89 s, against the circle tie's 14.7× and 14.6× with 8,360 params in 142 s. On the synthetic degree-filter task with 200 items over 40 epochs, circle MRR is 0.662 with identity at 0.646, and sphere is 1.000. This is the only measurement in gemphi where the function's mode structure is the task, and it is the reason section 8 thinks a law stated by mode could be obeyed.

The oracle is the nearest thing to a law. The quest has 256 words, families of 8, five kinds, 1,280 rows, 293 target words, chance MRR 0.021. The shipped trainer gives 0.082. Attraction-only with dithered rounding gives 0.594, hit@1 34.8%, hit@5 97.4%, within-kind 0.747, wrong channel 0.015, offset reading 0.005. Held out at 20%, one hop gives 0.011, two hops give is-a 0.095, used-for 0.093, and part-of, near and unlike about chance.

Quantisation is a constraint. A channel is one byte, 256 quanta of 2π. The nudge 0.06·sin d falls under half a quantum once sin d < 0.2, and nearest rounding froze every pair 8 quanta short. Dither costs attribution 71% to 65% and ships off.

The engine's cost lives where the paper says it does. Phinet is allocation and bandwidth bound at these sizes, with a 1 MB op at 2 to 5 ms. The paper's argument for function-wise differentiation is that the grid is smaller than the parameter count. At Phient's sizes the parameter count is small, 8,360 to 50,016, and the grid is 32 to 64 channels. The paper's argument would need to be re-checked at those sizes.

Not done, per the facts file: no weather work, no Phi-4 run, sphere at softmax and wide-deep unrun, no partnership agreed with anyone. And, not in the facts file but true of it: no physics loss, no coordinate derivative, no inverse operator.

## 12. The experiment that would test the paper's idea inside phinet

The experiment is a closure loss on the quest. It has one treatment and one control, and both are run on the existing phiecs oracle with the existing trainer.

The control is the attraction-only trainer with dithered rounding as recorded in the facts file: 0.594 MRR, hit@1 34.8%, hit@5 97.4%, held-out 20% two hops is-a 0.095. That configuration is the base operator. Its data loss is the only loss.

The treatment adds a second term. For each is-a pair (X, Y) in the training rows, and each is-a pair (Y, Z), the operator is applied twice, X ⇒ Y ⇒ Z, and the composed phase in the is-a channel is compared to the phase the operator gives for X ⇒ Z directly. The difference is the residual. It is squared, summed over triples, and added to the data loss with a weight. The weight is the paper's α, and it should be swept, since the paper sweeps its own. The residual is computed on the continuous phasor before byte rounding, for the reason given in section 7.

Two things follow the paper exactly. First, the triples used for the residual are not restricted to rows with a Z in the training set. Any Z with an is-a relation from Y counts, including held-out Z, because the residual needs no answer for X ⇒ Z, only that the composed and direct readings agree. This is the paper's virtual instance. Second, the residual is restricted to the kinds that the facts file says compose: is-a and used-for. Part-of, near and unlike are left out of the residual and measured only as outcomes.

The primary measure is the held-out 20% two-hop MRR on is-a, which is 0.095 in the control. The prediction, if the paper's mechanism transfers, is that this rises. The secondary measures are the one-hop MRR, 0.594, which must not fall, and wrong-channel MRR, 0.015, which must not rise. If one-hop falls while two-hop rises, the residual is trading the forward problem for the composition, and the paper's anchor should be added: a term holding the treatment operator near the control operator's output.

A second treatment tests super-resolution in the paper's sense. Train the data loss on words truncated to 32 channels and the closure residual on the full 64. Test at 64. Compare to the control trained and tested at 64. The paper's Table 1 predicts that the 32-plus-law operator is close to the 64 operator. Phient has no number for this.

Cost is reportable: the control runs in the time the facts file records for its configuration. The treatment adds one operator application per triple and one subtraction. At 1 MB ops of 2 to 5 ms, the addition is small, but it must be timed, not assumed.

Nothing in this section has been run. The numbers named are the control's, from the facts file.

## 13. Where we disagree or where the analogy breaks

The first break is the law. The paper has one, stated once, exact everywhere: Burgers' equation is Burgers' equation on every grid and every initial condition. Phient has no law of words. Closure is a guess, and section 6 already said it fails for three of five kinds in an oracle built to be clean. In a corpus it would fail more. The paper's residual is zero for the true solution. Ours would not be zero for the true language, because the true language does not compose. That means the residual would pull toward a language that composes, and the anchor would have to be strong. The paper's anchor is a convenience; ours would be a necessity.

The second break is direction of cost. The paper's data is expensive and its law is free. Ours is the reverse. The paper's headline, an operator trained on no data, has no meaning for Phient, because the corpus is the only thing we have. The physics loss is not a way to need less Austen. At most it is a way to use Austen better.

The third break is the derivative. The paper's residual contains spatial derivatives, so it needs equation (15). A closure residual contains no derivatives, only phase differences. The paper's central technical contribution, exact Fourier differentiation of the operator's output, would not be used by the experiment in section 12. It would be used only if a law for words involved a rate of change along the channel coordinate, and we have no candidate for such a law. It is possible that one exists. The sphere tie's degree structure suggests a smoothness prior, a penalty on high degree, which is a derivative penalty. That is a regulariser, not a physics, and the paper is clear that PINO's residual is the equation, not a smoothness term.

The fourth break is the domain. The paper's own limitation is that the FFT backbone "is hard to extend to higher dimensional problems." Phient's domain is one dimension, a circle, or two, a sphere. The paper's limit does not bind us. But the paper also notes aliasing: pointwise nonlinearities "reintroduce high frequencies," so the operator is discretisation-convergent without being representation-equivalent. On a byte-quantised circle with 64 channels, aliasing is the deadband problem in another form, and the facts file says the deadband froze training. The paper says that in practice aliasing "is typically not an issue." On our circle it was the issue.

The fifth is the softest and the most important. The paper's result is a percent error against a solver's ground truth. Phient's results are MRR against a corpus's next word or an oracle's row. There is no ground truth for what a word means. The paper's physics loss works because the physics is right. A closure loss would work only to the extent that closure is right, and nobody has measured how right it is for English. Section 12 measures the trainer, not the law.

## 14. The person and the moment

Everything in this section is from the profile.

The paper was posted in November 2021. In the profile's timeline, that is the year after the Fourier neural operator and three months before FourCastNet, the global weather model that put an adaptive FNO on a 720 × 1440 grid and ran a week's forecast in under two seconds. The physics-informed operator sits between the two. It is the paper that asks whether the operator learned from a solver can also learn from the equation the solver solves, and whether that lets the operator be trusted at resolutions the solver was too expensive to run.

The profile records her stated view of where this goes: "We have foundation models for language, not for physics." It records her method in her words: "All of the things that work with deep learning, let's take them, but make them a bit more principled." The physics-informed operator is the second sentence applied to the first. PINN was a thing that worked, on easy equations. The neural operator was a thing that worked, on solved data. Making them principled meant combining them so that the law is enforced on a function, exactly in Fourier space, rather than on a network at sampled points.

The profile also records the earlier decade: tensor methods at UC Irvine, the recognition that the moments of a latent-variable model form a tensor whose decomposition recovers the model, with guarantees. There is a line from that to this. A guarantee is a statement that holds without an answer key. The physics loss is a guarantee turned into a gradient.

The profile's two honest notes apply here as they did in the earlier papers. Phient has done no weather work, and "Fourier" in gemphi means phinet's operator over word functions. Nothing in this paper should be read as her endorsement. The profile says a partnership is something to earn with a result she would recognise, an operator over a function space she has not seen, measured the way she measures. The way she measures, in this paper, is a table with three resolutions and two losses, and the number that stays flat across the row. Section 12 is a proposal to make that table for words.

## 15. What remains

The first thing that remains is the law. Section 12 proposes closure on is-a and used-for, because those are the two kinds where the facts file's two-hop numbers are above chance. Whether closure holds well enough in a corpus to be a residual rather than a distortion is not known and has not been measured. The experiment would find out for the oracle. It would say nothing about Austen.

The second is the derivative. Phinet has a unitary FFT and no d/dx. Equation (15) says how to build one for a Fourier operator, and the circle is the domain where it is exact. This is a small addition to phinet and it is unbuilt. It would be needed only if a law involving rates along the channel coordinate appeared, and none has.

The third is resolution transfer. The Austen operator has never been tested at a channel count other than the one it was trained at. The paper's Table 1 has a data-only row that shows what happens without a law: error rises at 2× and 4×. Phient has not even measured its data-only row. That measurement needs no law and could be run first.

The fourth is the anchor. The paper's fine-tuning uses an operator loss to hold the refined model near the base. Phient's personas are trained as conditions on a base operator in principle, but the facts file has no measurement of a persona trained from a base against one trained alone, and no measurement of drift. The paper's Table 4, where the error is constant and the time falls, is the shape of the result to look for.

The fifth is the inverse. The oracle's generating structure is known, so an inverse operator from a word's function to its family and relations is well posed. Nothing has been built. The paper's inverse model, trained with the forward law as residual and the forward operator as anchor, is the recipe.

The sixth is the deadband. Any residual small enough to matter is small enough to vanish under one byte per channel. Section 12 computes the residual before rounding. Whether the trainer's rounded state can follow a gradient computed on the unrounded state is a question the facts file's dither result already raises and does not answer.

None of these is a result. They are the six places where this paper's method and Phient's engine would touch, listed so that the next measurement page can pick one.

## 16. Sources

The following were fetched on 2026-04-06 and are the basis for every claim about the paper in sections 2 to 5, 13 and 14.

Li, Zheng, Kovachki, Jin, Chen, Liu, Azizzadenesheli, Anandkumar. Physics-Informed Neural Operator for Learning Partial Differential Equations. arXiv:2111.03794, v1 6 November 2021, v4 29 July 2023. Abstract page: https://arxiv.org/abs/2111.03794. Full text: https://arxiv.org/pdf/2111.03794v4.

The same paper as published: ACM / IMS Journal of Data Science, February 2024, doi:10.1145/3648506. Record at CaltechAUTHORS: https://authors.library.caltech.edu/records/06m4r-8jf10. The ACM page https://dl.acm.org/doi/10.1145/3648506 returned a 403 when fetched and was not read; the journal name, date and DOI above are from the Caltech record.

All numbers about Phient are from PHIENT_FACTS.md in this folder. All statements about the author are from 17_ANIMA_ANANDKUMAR_PROFILE.md in the parent folder. No other source was used.

The numbers in section 4 were read from the paper's Table 1, Table 2, Table 3, Table 4, the long transient flow paragraph, the lid-cavity paragraph and the inverse problem section, and were checked twice against the PDF. Where the paper reports a standard deviation it is given here; where this page omits one, the paper's table has it and the omission is for space.
