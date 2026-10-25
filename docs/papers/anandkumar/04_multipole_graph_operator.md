# Short range, long range: the kernel in levels and the relation in hops

*Phient interpreted papers, 4 of 16 · 2026-02-23 · on: "Multipole Graph Neural Operator for Parametric Partial Differential Equations" (Li, Kovachki, Azizzadenesheli, Liu, Stuart, Bhattacharya, Anandkumar, 2020, Advances in Neural Information Processing Systems 33 (NeurIPS 2020); arXiv:2006.09535)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the fourth of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. The third paper was the graph kernel network, the neural operator before the Fourier transform arrived. Its last section named its own cost, a kernel whose edges grow as the square of the number of nodes, and proposed multipole methods as a way out. This paper is that way out. It was posted to arXiv on 16 June 2020, three months after the graph kernel network, and revised on 19 October 2020 for NeurIPS.

The paper's claim is that the kernel of an integral operator can be split by range. Short-range interaction is sparse and high-rank. Long-range interaction is dense and low-rank. If each range gets its own graph, at its own resolution, and the graphs are linked by transition kernels that move state up and down between them, the whole operator can be evaluated in time linear in the number of nodes. The paper calls the result the multipole graph kernel network, MGKN. It shows the linear scaling on a plot, shows the error falling as levels are added, shows the error holding across resolutions, and reports two benchmark tables where MGKN loses narrowly on one problem and wins clearly on the other.

We read it here for one reason above the others. The paper's central object is a kernel written as a sum over ranges, K = K₁ + K₂ + ... + K_L, with the long ranges reached through a coarse set of inducing points. Phical's central object is a word written as 32–64 relations, and Phient's quest measures those relations at one hop and at two hops. One hop is short range. Two hops is long range. The quest's held-out numbers at two hops are the only place in the facts file where a long-range relation has been measured, and they are weak on three kinds out of five. This paper says how a long-range kernel should be built so that it stays cheap.

Sections 2 to 5 state what the paper says, in its own terms, with its numbers. Sections 6 to 10 place the paper's objects inside the Phient family: the multi-level graph as a set of relation ranges, the inducing points as families, the V-cycle as an operator X ⇒ Y under c, the level as a spherical-harmonic degree, and the hierarchy as the helix. Section 11 lists what Phient has measured that bears on it. Section 12 names one experiment and its controls. Section 13 says where the analogy breaks.

One limit is stated at the top. Phient has run no multipole method and no multi-level graph. Nothing in gemphi has a V-cycle. Every number about Phient below was measured for another purpose. Where this paper's idea has no counterpart in phinet, this page says so.

## 2. What the paper claims, in its own terms

The abstract opens with the problem it means to solve. "One of the main challenges in using deep learning-based methods for simulating physical systems and solving partial differential equations (PDEs) is formulating physics-based data in the desired structure for neural networks." Graph neural networks, it says, "have gained popularity in this area since graphs offer a natural way of modeling particle interactions and provide a clear way of discretizing the continuum models."

Then the fault it finds in them. "However, the graphs constructed for approximating such tasks usually ignore long-range interactions due to unfavorable scaling of the computational complexity with respect to the number of nodes. The errors due to these approximations scale with the discretization of the system, thereby not allowing for generalization under mesh-refinement." A local graph on a finer mesh reaches a shorter physical distance, so its error depends on the mesh, and the point of an operator, that it should not care about the mesh, is lost.

The proposal follows. "Inspired by the classical multipole methods, we propose a novel multi-level graph neural network framework that captures interaction at all ranges with only linear complexity. Our multi-level formulation is equivalent to recursively adding inducing points to the kernel matrix, unifying GNNs with multi-resolution matrix factorization of the kernel." And the result: "Experiments confirm our multi-graph network learns discretization-invariant solution operators to PDEs and can be evaluated in linear time."

The contributions section lists four things. The development of "the multipole graph kernel neural network (MGKN) that can capture long-range correlations in graph-based data with a linear time complexity in the nodes." The unification of "GNNs with multi-resolution matrix factorization through the V-cycle algorithm." Verification of "linear time complexity of the proposed methodology." And demonstration of "mesh-invariant solution operators to the Darcy flow and Burgers' equations."

The paper describes what it borrows. "FMM decomposes the kernel matrix into different ranges and hierarchically imposes low-rank structures to the long-range components." It names the second borrowing too: "we use Nyström approximation or the inducing points method by uniformly sampling m < n nodes from the n nodes discretization, which is to approximate the kernel matrix by a low-rank decomposition." The approximation is written K_nn ≈ K_nm K_mm K_mn.

It is careful about one thing. It calls its iteration a V-cycle and adds, in parentheses, "not to confused with multigrid methods". The shape is borrowed from multigrid. The content is a learned kernel at each level.

The conclusion restates the claim: the work presents a "novel multi-scale graph structure which, when deployed with GNNs, captures global properties of the PDE solution operator with a linear time-complexity." The paper does not mention the Fourier neural operator, which the same authors posted later the same year.

## 3. The mathematics, plainly

The setting is the one the graph kernel network used. A differential operator L_a depends on a parameter a, and the PDE is (L_a u)(x) = f(x) for x in D, with the example L_a = −div(a∇·). The map to learn goes from a to u. The paper writes the solution as an integral, u(x) = ∫_D G_a(x, y) [f(y) + (Γ_a u)(y)] dy, and lets a learned kernel stand in for the Green's function.

The kernel operator is (K_a u)(x) = ∫_D κ_φ(a(x), a(y), x, y) u(y) dy, with κ_φ a small neural network, iterated as v^(t) = σ((W + K_a) v^(t−1)) for t = 1, ..., T. That is the graph kernel network, and its cost on n nodes with full reach is n².

The new step is to split the kernel. K = K₁ + K₂ + ... + K_L, where K₁ is the shortest range and K_L the longest. Figure 2's caption says what each piece is like. "K₁ corresponds to short-range interaction; it is sparse but high-rank. K₃ corresponds to long-range interaction; it is dense but low-rank." A dense low-rank matrix can be factored through a small set of points. So the paper writes K ≈ K_{1,1} + K_{1,2} K_{2,2} K_{2,1} + K_{1,2} K_{2,3} K_{3,3} K_{3,2} K_{2,1} + ⋯. Each term reaches further and passes through fewer points.

Three kinds of kernel do the work. K_{l,l} acts within level l, over a ball of radius r_{l,l}. K_{l+1,l} carries state from level l up to the coarser level l+1. K_{l,l+1} carries it back down. Each is an integral of the same form, κ over a ball, with its own parameters φ.

The levels are graphs. "We construct L graph levels, where the finest graph corresponds to the shortest-range interaction K₁, and the coarsest graph corresponds to the longest-range interaction K_L." On a uniform grid of resolution s with s^d = n, "the L multi-level graphs will be grids with resolution s_l = s / 2^(l−1), and consequentially n_l = s_l^d = (s / 2^(l−1))^d". Level 1 is the finest. Level L is the coarsest.

The iteration is the V-cycle. Downward, for l = 1 to L: v̌_{l+1}^(t+1) = σ(v̂_{l+1}^(t) + K_{l+1,l} v̌_l^(t+1)). Upward, for l = L to 1: v̂_l^(t+1) = σ((W_l + K_{l,l}) v̌_l^(t+1) + K_{l,l−1} v̂_{l−1}^(t+1)). State goes down to the coarsest level, is mixed there, and comes back up, each level adding its own within-level kernel and its own local matrix W_l.

The cost is the sum over levels. "The computational complexity is Σ_l O(n_l). By designing the sub-graphs so that n_l decays fast enough, we can obtain linear complexity." With n_l = O(n / 2^l), the sum is O(n). "Combined with a Nyström approximation, we obtain O(m) complexity," where m is the number of sampled nodes.

## 4. Its experiments and numbers as reported

The first problem is Darcy flow, −∇·(a(x)∇u(x)) = f(x) on (0,1)² with u = 0 on the boundary. The coefficients are drawn as a ∼ μ with μ = ψ_# N(0, (−Δ + 9I)^−2), where "the mapping ψ: ℝ → ℝ takes the value 12 on the positive part of the real line and 3 on the negative." Solutions come from "a second-order finite difference scheme on 241 × 241 and 421 × 421 grids." Training and test sizes are both N = 100. The network has "width d_v = 64, iteration T = 5 and kernel network κ_{φ_{l,l}} as a three-layer neural network with width 256 / 2^(l−1)." When the number of levels changes, "we fix coarsest level m_L = 25, r_{L,L} = 2^−1, and let m_l = 25 · 4^(L−l), r_{l,l} = 2^−(L−l)."

Figure 3 has three panels. On the left, "MGKN (blue line) achieves linear time complexity (the time to evaluate one equation) w.r.t. the number of nodes, while GKN (red line) has quadratic complexity." In the middle, "Adding more levels reduces test error." On the right, "MGKN can be trained on a coarse resolution and perform well when tested on a fine resolution." Table 2 gives the middle panel as numbers.

| levels | train error | test error | train time | test time |
|---|---|---|---|---|
| m = [25] | 0.0181 | 0.1054 | 0.50 s | 0.21 s |
| m = [100, 25] | 0.0189 | 0.0781 | 1.59 s | 0.69 s |
| m = [400, 100, 25] | 0.0155 | 0.0669 | 5.24 s | 1.90 s |
| m = [1600, 400, 100, 25] | 0.0165 | 0.0568 | 19.75 s | 7.94 s |

Train error barely moves. Test error halves from one level to four. The times are reported as printed; our extraction did not return what they are times of. Table 3 is the resolution transfer, relative L² error, trained at s and tested at s'.

| train s | s' = 61 | s' = 121 | s' = 241 |
|---|---|---|---|
| 31 | 0.0643 | 0.0594 | 0.0634 |
| 41 | 0.0630 | 0.0606 | 0.0609 |
| 61 | 0.0579 | 0.0590 | 0.0589 |
| 121 | 0.0650 | 0.0628 | 0.0664 |
| 241 | 0.0630 | 0.0680 | 0.0665 |

The paper's reading is that "MGKN achieves similar testing error on s' = 61, 121, 241, independently of the training discretization."

Table 4 is the Darcy benchmark on full grids.

| method | s = 85 | s = 141 | s = 211 | s = 421 |
|---|---|---|---|---|
| NN | 0.1716 | 0.1716 | 0.1716 | 0.1716 |
| GCN | 0.1356 | 0.1414 | 0.1482 | 0.1579 |
| FCN | 0.0253 | 0.0493 | 0.0727 | 0.1097 |
| PCA+NN | 0.0299 | 0.0298 | 0.0298 | 0.0299 |
| RBM | 0.0244 | 0.0251 | 0.0255 | 0.0259 |
| GKN | 0.0346 | 0.0332 | 0.0342 | 0.0369 |
| MGKN | 0.0416 | 0.0428 | 0.0428 | 0.0420 |

The second problem is the one-dimensional viscous Burgers equation, ∂_t u + ∂_x(u²/2) = ν ∂_xx u on (0, 2π) with periodic boundary and ν = 0.1, initial condition u₀ ∼ N(0, 625(−Δ + 25I)^−2), solved at resolution 2¹³ = 8192, again N = 100 train and 100 test. Here the transition kernels "are restricted to be restrictions and prolongations." Table 5:

| method | 64 | 128 | 256 | 512 | 1024 | 2048 | 4096 | 8192 |
|---|---|---|---|---|---|---|---|---|
| NN | 0.4677 | 0.4447 | 0.4714 | 0.4561 | 0.4803 | 0.4645 | 0.4779 | 0.4452 |
| GCN | 0.3135 | 0.3612 | 0.3999 | 0.4138 | 0.4176 | 0.4157 | 0.4191 | 0.4198 |
| FCN | 0.0802 | 0.0853 | 0.0958 | 0.1407 | 0.1877 | 0.2313 | 0.2855 | 0.3238 |
| PCA+NN | 0.0397 | 0.0389 | 0.0398 | 0.0395 | 0.0391 | 0.0383 | 0.0392 | 0.0393 |
| GKN | 0.0367 | 0.0316 | 0.0555 | 0.0594 | 0.0651 | 0.0663 | 0.0666 | 0.0699 |
| MGKN | 0.0174 | 0.0211 | 0.0243 | 0.0355 | 0.0374 | 0.0360 | 0.0364 | 0.0364 |

The paper's verdict: "For Burger's equation, when linear spaces are no longer near-optimal, MGKN is the best performing method."

## 5. The one idea, restated

Strip the PDE away and the paper says this. A state at one point depends on the state at every other point, near and far. The near dependence is detailed and cheap to list, because there are few near points. The far dependence is broad and expensive to list, because there are many far points, but it is also smooth, so it can be summarised. Summarise the far field through a few representative points. Let the near points talk to each other directly. Let every point talk to its representative, let the representatives talk among themselves, and let the representatives report back. Do this at more than one scale, so that what is far at one level is near at the next.

The kernel decomposition K = K₁ + K₂ + ⋯ + K_L is the statement that a relation has ranges. The factorisation through K_{l,l+1} and K_{l+1,l} is the statement that the long ranges pass through few points. The V-cycle is the order of operations: down to the coarsest summary, mix there, back up. The linear cost follows from the geometric decay of the node counts, and nothing else.

Two consequences follow, and the paper tests both. First, adding levels lowers error, because each level adds a range the previous levels could not reach. Table 2 shows test error falling from 0.1054 at one level to 0.0568 at four, with train error flat, so the gain is in generalisation rather than fit. Second, the operator stays mesh-invariant, because reach is set by radii r_{l,l} and by the level structure, both of which are stated in physical units, and neither in node counts. Table 3 shows the errors between 0.0579 and 0.0680 on every pair of train and test resolutions.

One consequence cuts against the method, and the paper reports it. On Darcy flow, where the solution operator is close to linear and a reduced basis is nearly optimal, MGKN at 0.0416 to 0.0428 is beaten by RBM at about 0.025 and by GKN at about 0.034. The multi-level approximation costs accuracy where a full kernel is affordable. On Burgers, where the operator is nonlinear and the linear methods stall, MGKN is best at every resolution.

The paper frames the method through inducing points, and that framing is the one that matters for Phient. An inducing point is a place that stands for a region. Choosing inducing points is choosing what to summarise. The paper chooses uniformly at random on a grid. Phical has no grid, and section 9 says what it has instead.

For Phient the restatement is shorter. A word relates to other words at one hop and at many hops. The one-hop relations are few and exact. The many-hop relations are many and smooth. Store the first directly. Reach the second through a few words that stand for a family. The paper is a recipe for the second half, and the quest has so far measured only that the second half is weak.

## 6. The word as a field with ranges

In the paper the input is a function a on a domain, and the hidden state v is a function of width 64 on the same domain. Both are known at nodes. The paper's addition to the graph kernel network is that the nodes come in levels, with level 1 the finest and level L the coarsest, and that a state lives at every level at once, as v̌_l on the way down and v̂_l on the way up.

In Phical the input is a word. A word is 32–64 relations, each a complex phasor on a closed circle with a count. The phinet Tensor that holds it has shape [b, w, s], complex, with s the channel axis. This is the word as a function on its channels, valued in phasors. The graph kernel network paper read this as one function on one set of coordinates. This paper asks whether the coordinates come in ranges.

They do, in two senses, and neither is the paper's sense exactly.

The first sense is the sphere tie. phinet::Tie::Sphere reads the 64 channels as spherical harmonics Y_l^m with l < 8, 64 = 8² of them, and puts one complex weight per degree l. Degree is a scale. Low degree is the smooth, long-range part of a function on the sphere. High degree is the sharp, short-range part. A tie with one weight per degree is a kernel that acts on each range separately, which is what K = Σ K_l says, though with the ranges chosen by a basis rather than by a graph. The sphere tie is 1,192 params and reaches 12.8× chance valid and 13.1× test on Austen at hinge, 3 epochs, against the circle's 14.7× and 14.6× at 8,360 params.

The second sense is the count. Every Phical relation has a phasor and a count. The count says how often the pair was seen under that condition. A relation seen many times is a near relation in the paper's sense, dense in evidence and cheap to store. A relation seen rarely or reached only through other words is a far relation. The paper says far relations should be reached through a summary. Phical has no summary. A word with 64 channels stores 64 relations and reaches nothing beyond them except by another word's channels, which is a hop.

So the reading is this. A word is a field in the paper's sense, a function on coordinates. Its coordinates carry a scale in the sphere tie and an evidence weight in the count. What the word lacks is the paper's second level: a coarser word, standing for several, through which the long-range relations pass. Section 9 says the quest has the material to build one and has not.

## 7. The inducing point as a family, the level as a condition

The paper approximates the kernel matrix as K_nn ≈ K_nm K_mm K_mn, with m < n inducing points chosen "by uniformly sampling m < n nodes from the n nodes discretization." At each level the inducing points form a coarser graph, and the transition kernels K_{l+1,l} and K_{l,l+1} are integrals of the same κ, with their own parameters, over a ball of radius r_{l+1,l} or r_{l,l+1}. In the Darcy experiments the node counts per level are m_l = 25 · 4^(L−l) and the radii r_{l,l} = 2^−(L−l), so every level up has a quarter of the nodes and twice the reach.

Phiecs has a coarser graph already, and it is a label. The quest oracle has 256 words in families of 8, with five kinds of relation, is-a, part-of, near, used-for, unlike, in 1,280 rows against 293 target words. A family is a set of 8 words that the oracle treats as belonging together. If each family had a representative, there would be 32 of them. That is a level-2 graph with n₂ = n₁ / 8, coarser than the paper's factor of 4 but of the same kind.

The paper's transition kernel depends on (a(x), a(y), x, y), the condition at both ends and the positions. The quest's kind is the condition. So the analogue of K_{2,1} is a map from a word's phasor on a kind's channel to its family's phasor on the same channel, and K_{1,2} is the return. The analogue of K_{2,2} is a relation between families on the kind's channel. None of these exist. The quest's trainer moves one word toward one target on one channel and stops.

What the quest has measured says why this matters. Within kind, the attraction-only trainer with dithered rounding reaches 0.747 MRR. On the wrong channel it reaches 0.015. So a relation lives on its channel and nowhere else, which is the paper's "under what conditions" said as a measurement. On held-out rows, one hop is 0.011 against chance of 0.021, and two hops reach 0.095 on is-a and 0.093 on used-for, with part-of, near and unlike at about chance. Two hops is a long-range relation. The two kinds where it works are the two where a family plausibly stands in for its members. Is-a passes through a family by construction. Used-for is shared within a family. Part-of, near and unlike are relations between particular words and do not summarise.

That is the paper's Figure 2 read on the quest. Is-a is dense and low-rank. Part-of is sparse and high-rank. The paper builds one kernel per range and lets each be what it is. The quest builds one trainer for all five kinds and gets the low-rank ones at two hops and nothing else. We do not claim a family kernel would fix the held-out numbers. We claim the paper's inducing point and the quest's family are the same slot, one filled and one empty.

## 8. The V-cycle as X ⇒ Y under c

The paper's update is a V-cycle. Down: v̌_{l+1}^(t+1) = σ(v̂_{l+1}^(t) + K_{l+1,l} v̌_l^(t+1)). Up: v̂_l^(t+1) = σ((W_l + K_{l,l}) v̌_l^(t+1) + K_{l,l−1} v̂_{l−1}^(t+1)). Read it as an operator. One cycle takes the states at every level and returns the states at every level, and every kernel in it depends on a. So a cycle is X ⇒ Y under c with X the stack of v̂^(t), Y the stack of v̂^(t+1), and c = a. The paper runs T = 5 of them.

Phinet's operator has the same outer shape. Its Fno layer takes a word's function on the channel axis and returns another, with the nonlocal part done as a unitary FFT, learned complex weights on the retained modes, inverse FFT, plus a local path, then a nonlinearity. On Austen, with a pool of 1,228 words seen 20 or more times and 204,592 training examples, the operator ranks the next word against chance MRR 0.0063. Identity gives 7.9× chance. Hinge at 8,360 params gives 14.7× valid and 14.6× test. Softmax at β = 8 gives 18.0× and 18.8×. Softmax wide and deep at 50,016 params gives 27.6× at 3 epochs and 30.7× valid, 30.3× test at 8 epochs, hit@10 0.348.

The difference from this paper is where the levels are. The Fourier layer has levels in the sense that every mode is a scale, and the learned weight per mode is a kernel per range. But it has one graph, the channel axis, and it never coarsens it. There is no second set of nodes, no transition kernel, no state held at two resolutions at once. Phinet's operator is K = Σ_l K_l with the sum done inside one FFT. This paper's operator is the same sum done across separate graphs linked by learned maps. The Fourier version is exact and cheap when the kernel is a convolution. The multipole version is approximate and cheap when it is not.

The paper's Table 4 measures the price of the approximation on a problem where the linear methods are near optimal. GKN at 0.0346 beats MGKN at 0.0416 on the same grid. Table 5 measures the price of not having it where they are not. GKN at 0.0699 loses to MGKN at 0.0364 at s = 8192. Every Austen number is a Fourier tie on one axis. Whether the word operator is closer to Darcy or to Burgers, in the paper's sense, has not been measured.

One more thing in the V-cycle matters. State goes down to the coarsest level before it comes back up, so the long-range summary is computed first and the short-range detail is added last. In phiano's terms the persona would be read before the sentence. Phiano's persona attribution reaches 71% on held-out four-sentence paragraphs across Chesterton, Austen and Carroll, chance 33%, with 74, 65 and 86 by writer, measured with no level structure. Phiano has both levels and no cycle between them.

## 9. The graph in levels as the quest, and the sphere degree as the level

The paper builds its levels by halving the resolution: s_l = s / 2^(l−1), n_l = (s / 2^(l−1))^d. On a grid this is natural. The coarse nodes are a sub-grid, the ball radius doubles, and the transition kernels connect a fine node to the coarse nodes within reach. Table 2 says the fourth level is still paying: from three levels to four, test error goes from 0.0669 to 0.0568.

The quest has no grid, so the paper's construction does not transfer as written. What it has is families of 8 and five kinds. A first level is the 256 words. A second level is the 32 families. A third level could be the kinds themselves, 5 nodes, if a kind can carry a phasor, which phiecs has not said. The quest's held-out results, one hop 0.011, two hops 0.095 on is-a and 0.093 on used-for, chance 0.021, are the only long-range measurements available, and section 12 builds on them.

The sphere tie is where the paper's levels land most exactly, in a different form. The paper's level is a graph with a node count and a radius. The sphere tie's level is a spherical-harmonic degree l < 8, with one complex weight per degree. There are 8 degrees and they are not linked by transition kernels. They are read directly from the 64 channels and written directly back. This is a multi-level kernel with the up and down maps replaced by a fixed basis, the spherical harmonic transform, which is the move the same authors make later with the Fourier transform. The sphere tie is 1,192 params. The circle tie is 8,360. The paper's Darcy kernel networks have width 256 / 2^(l−1) per level, so the model shrinks as the range grows. The sphere tie has one weight per degree, the same size at every scale.

The synthetic degree-filter task is the one place a level has been isolated. 200 items, 40 epochs. The circle tie reaches MRR 0.662 with identity at 0.646, so the circle barely moves. The sphere tie reaches 1.000. The task asks the operator to keep one degree and drop the rest, and the sphere tie can do that because a degree is a level for it. This is consistent with the paper's Figure 3 middle panel, and it is a much smaller claim. The degree filter is synthetic and built to be separable by degree.

The tensor lands where it did in the third paper. The paper's kernels return a d_v × d_v matrix per edge, d_v = 64, at every level and between levels. Phinet's ties are one weight per mode. Phinet has no message tensor and no level tensor.

The agent loop is absent from the paper. Its T = 5 cycles are a fixed loop with no reader. Phiano's quest ingest and generation are loops with a reader. The paper does not speak to that.

## 10. The hierarchy as the helix

The paper's levels are a hierarchy on the domain. Level 1 holds every node. Level 2 holds a quarter of them. Each coarse node stands for the fine nodes in its neighbourhood, and the hierarchy is the reason the cost is linear.

The helix is Phient's one hierarchy. The lexicon is stored as a DNA-coded tree: 64 codons, each the base-4 ACGT of a channel, with a hexagram, a sphere degree and order, B-DNA xyz at 10.5 codons per turn and 0.34 nm rise, and a PhiTon wavelength, band and hex per codon. It holds 18,496 words in 1.9 s at 75 MB, byte-exact round trip. It is a tree, so it has levels. A node near the root stands for every word below it.

That is the shape the paper wants for an inducing-point hierarchy, and it was built for something else. The helix is storage. No operator reads it. No kernel has been given a helix node as an input. Its levels are levels of a code, chosen so that a codon round-trips to a byte, and there is no measurement that says a node near the root summarises its subtree in the paper's sense, that a relation between two subtrees is low-rank and can pass through the roots.

The paper offers a test for whether a hierarchy is the right one. Add a level and see whether the error falls. Table 2 is that test on a grid. The equivalent on the helix would be an operator that reads the tree at depth k and at depth k + 1 and compares. That has not been run and is not proposed in section 12, because the quest's families are a hierarchy with a known answer and the helix's tree is a hierarchy with no answer yet.

The helix also carries the paper's other input. The multipole kernel takes positions x and y as well as values, and the transition kernels are integrals over balls of radius r_{l+1,l}. The helix gives every codon xyz. So the helix is the only place in the family where "within a ball of radius r" has a literal reading. Whether that ball means anything is the open question the third paper left, and this paper adds a second: whether a ball on a coarser turn of the helix means anything more.

One more correspondence and then a limit. Figure 2 says the short-range kernel is sparse and high-rank and the long-range kernel is dense and low-rank. In the helix, the short range is a codon and its neighbours on one turn, and the long range is the tree above. If the helix is the right hierarchy, that split should show as a rank split in the relations between subtrees. Nothing has measured a rank in gemphi. This whole section is a correspondence of shapes. The helix has levels. The paper needs levels. Nothing more has been shown.

## 11. What Phient has already measured that bears on it

Nothing in gemphi is a multipole method, a multi-level graph, or a V-cycle. What follows is what has been measured that touches the paper's claims, and only that.

On the claim that a kernel has ranges and each range deserves its own weights, the sphere tie is the nearest measurement. One complex weight per spherical-harmonic degree, l < 8, 1,192 params. On Austen hinge at 3 epochs, 12.8× valid and 13.1× test, hit@10 0.182, in 89 s, against the circle's 14.7× and 14.6× at 8,360 params in 142 s. On the synthetic degree-filter task, 200 items and 40 epochs, sphere 1.000 against circle 0.662 and identity 0.646. The sphere is smaller and faster and slightly worse on Austen, and exact on the task built for it. The sphere at softmax and at wide and deep is unrun.

On the claim that long-range relations pass through summaries and that this is where generalisation lives, the quest is the measurement. Attraction only with dithered rounding reaches 0.594 MRR, hit@1 34.8%, hit@5 97.4% on seen rows, against chance 0.021 and the shipped trainer's 0.082. Within kind 0.747, wrong channel 0.015, offset reading 0.005. Held-out 20%: one hop 0.011, two hops is-a 0.095 and used-for 0.093, part-of, near and unlike at about chance. The two-hop result is the only long-range number in the facts file. It exists on the two kinds that summarise and not on the three that do not.

On the claim that the method's cost should be linear in nodes, the engine measurement is that phinet is allocation and bandwidth bound at these sizes, with a 1 MB op at 2–5 ms. The paper's cost argument is about node count. Phinet's cost is about bytes moved. A V-cycle on the quest would be small in nodes and would still pay per allocation.

On the byte, the deadband stands apart. A channel is one byte, 256 quanta of 2π. The nudge 0.06·sin d falls under half a quantum once sin d < 0.2, and nearest rounding froze every pair 8 quanta short. Dither costs corpus attribution 71% to 65%, so it ships off by default. A transition kernel from a word to its family and back would write phasors through the same byte and meet the same floor.

On the claim that the coarse level should inform the fine, phiano's persona attribution, 71% overall, and the dialogue result, 18 replies judged as their persona 44% by the sector reader and 72% by the Kneser-Ney panel, chance 33%, were measured with no cycle between a writer-level state and a sentence-level state. They bear on this paper only as the baseline such a cycle would have to beat.

Decoding, operator nearest-k at perplexity 66.9 against the trigram sampler's 13.3, does not bear on this paper. The 62 of 62 parity tests against PyTorch say phinet's primitives are correct and nothing about levels.

## 12. The experiment: a family level in the quest, with two controls

The paper's idea, cut down to what phiecs can hold, is this. Add a second graph whose nodes stand for groups of the first graph's nodes. Learn a map up, a map within the coarse graph, and a map down. Train the whole cycle. Test on relations that were held out, and in particular on relations that need two hops.

The place to run it is the quest, because the quest has families, kinds, a held-out split, and a two-hop number to beat. The experiment has five parts.

The data is the phiecs synthetic oracle as it stands: 256 words in families of 8, kinds is-a, part-of, near, used-for, unlike, 1,280 rows, 293 target words, held-out 20%. Chance MRR is 0.021.

The treatment is a two-level trainer. Level 1 is the 256 words with the existing attraction-only move and dithered rounding. Level 2 is 32 family nodes, one per family, each holding a phasor per kind's channel. Following the paper's Nyström choice, each level-2 node starts as one word sampled uniformly from its family, rather than a mean. Each epoch runs one V-cycle in the paper's order: every word nudges its family's phasor on the row's kind (K_{2,1}), families nudge one another along the rows that cross families on that kind (K_{2,2}), and every family nudges its members back (K_{1,2}). Then the level-1 move runs as shipped. Following the paper's rule of halving width at the coarser level, the level-2 move has half the scale of the level-1 move.

The first control is the shipped attraction-only trainer with dither, same epochs, same rows, no level 2. This is the number in the facts file and it isolates the effect of the level.

The second control is the shipped trainer with one extra pass per epoch over two-hop rows composed from the training set, no level 2. This separates the paper's claim, that the level is what reaches long range, from the cheaper claim that more long-range training rows would do the same.

The measurements are the quest's own. Seen rows: MRR, hit@1, hit@5, within kind, wrong channel. Held-out: one hop and two hops per kind. The baselines to beat are 0.594 MRR on seen rows, 0.011 at one hop, and 0.095 and 0.093 at two hops on is-a and used-for, with the other three kinds at chance. The paper's prediction, read across, is that the level helps two hops most on the kinds that summarise, and that it should not hurt the seen rows much, as Table 2's train error is flat as levels are added. The wrong-channel figure must stay near 0.015 or the level has leaked across conditions.

A third level, one node per kind, is a second experiment if the first shows anything.

What the experiment cannot test is the paper's linear scaling. The quest is 256 words, and cost at that size is allocation bound. It also cannot test resolution transfer, because the quest has one channel count.

## 13. Where we disagree, and where the analogy breaks

The analogy breaks in three places and we disagree with the paper in one.

The first break is the coarsening rule. The paper coarsens by halving a grid. Every coarse node is a fine node, every fine node has coarse nodes within a known radius, and the count per level is (s / 2^(l−1))^d by construction. Phical has no grid. The quest's families are labels the oracle assigned, the sphere's degrees are a basis, and the helix's tree is a code. Each gives a hierarchy, and none of them gives the paper's guarantee that a coarse node sits within reach of the fine nodes it stands for. Section 12 borrows the family because it is the only hierarchy with a known answer, and the choice is ours, not the paper's.

The second break is the value. The paper's state is a real vector of width 64 at every node and every level. Phical's channel is a phasor on a closed circle, quantised to one byte. The byte deadband, where the nudge falls under half a quantum at sin d < 0.2 and rounding froze every pair 8 quanta short, would act on a family node's phasor exactly as it acts on a word's. A level-2 node aggregating 8 words is a mean of 8 phasors, and a mean of phasors on a circle is not on the circle, so the up map has a rounding step the paper never needs.

The third break is what "linear" costs. The paper's cost argument counts nodes, and its Figure 3 plots time against nodes. Phinet's measured cost is allocation and bandwidth, 2–5 ms per 1 MB op. A V-cycle adds allocations at every level and every transition. At the quest's size the node count is irrelevant and the allocation count is everything. The paper's win would not show in phinet until the graph is large enough that nodes dominate bytes, and no such graph exists in gemphi.

The disagreement is about the coarse basis. The paper learns its transition kernels. The sphere tie fixes them, as the spherical harmonic transform, and pays nothing for them. Our one comparison, the degree-filter task at 1.000 for the sphere against 0.662 for the circle, and Austen hinge at 12.8× for 1,192 params against 14.7× for 8,360, says that on the channel axis a fixed multi-scale basis is enough and small. The paper's Table 4 says the same on Darcy, where the fixed-basis PCA+NN at 0.0299 beats MGKN at 0.0416. The paper's answer is Table 5, where on Burgers the fixed bases stall and the learned levels win. So the disagreement is really a question: is a word operator like Darcy or like Burgers? The Austen results are consistent with Darcy. Nothing has tested Burgers. We hold the view lightly, on one sphere run at hinge and 3 epochs, with softmax and wide and deep unrun.

## 14. The person and the moment

The paper was posted on 16 June 2020 and revised on 19 October 2020 for NeurIPS. She was two years into her two posts, Bren Professor at Caltech and senior director of machine-learning research at NVIDIA, both from 2018, and the co-founder of Caltech's AI4Science initiative. The seven authors are the seven of the graph kernel network paper from March, with her name last again.

The year matters. The graph kernel network of March 2020 named its cost, quadratic in nodes, and named multipole and multigrid methods as the ways out. This paper is one of those ways, delivered three months later. Later the same year the same seven authors would post the Fourier neural operator, which is the other way out: give up the general kernel, keep the convolution, and let the FFT reach every point at once. This paper stands between the two. It keeps the general kernel and reaches every point through levels. The Fourier paper is the one Quanta profiled in 2021 and the one FourCastNet put on the whole planet in 2022, a week's forecast in under two seconds. This paper is the road the group looked down and did not take as far.

It is also the paper where her first decade shows most clearly in her second. Her early work was tensor methods: moments as tensors, decompositions with guarantees, the NSF CAREER Award in 2013, a Sloan Fellowship in 2014. The multipole method is a factorisation, K_nn ≈ K_nm K_mm K_mn, applied recursively. The paper's own phrase is "unifying GNNs with multi-resolution matrix factorization of the kernel." That is a low-rank structure imposed on a large matrix, in a paper with no tensor in it. The method she has described for her later work, "all of the things that work with deep learning, let's take them, but make them a bit more principled," fits this paper closely. Message passing on graphs worked. The fast multipole method is principled. The paper joins them.

Her view now is that transformers cannot be the substrate for physics at scale, because a few hundred grid points per dimension is a context of hundreds of billions to a trillion tokens, and an operator in function space treats scale as what it is. This paper is that argument in its early form. Its Table 5 runs Burgers at 8,192 points with error flat from 512 up. Its Table 3 trains at 31 and tests at 241.

The recognition came later: the Time100 lists, the IEEE Kiyo Tomiyasu Award, the Blavatnik Award, the UN Scientific Advisory Board appointment on 16 March 2026. In June 2020 she was a professor and a research director posting a preprint. This paper does not claim she has read anything of Phient's. A partnership is something to earn with a result she would recognise: an operator over a function space she has not seen, measured the way she measures.

## 15. What remains

Everything in section 12 remains. Phient has no second level anywhere, no transition kernel, no V-cycle, and no held-out two-hop number above 0.095. The family-level experiment is the one to run. Its cost is bounded: 256 words, 32 family nodes, five channels, and an engine that is fast at these sizes. Its risk is the byte. A family node is a mean of eight phasors rounded to a byte, and the deadband that froze the shipped trainer 8 quanta short is waiting for it. Dither is the known fix and its cost, 71% to 65% on corpus attribution, is the known price.

The question section 13 left remains. Is the word operator like Darcy or like Burgers? On Darcy the fixed bases were near optimal and the learned levels lost. On Burgers the fixed bases stalled and the learned levels won. Phinet's Austen results are consistent with the first. A task where the sphere tie stalls has not been found and has not been sought. Until one is, the case for learned levels on the channel axis rests on nothing.

The sphere at softmax and at wide and deep remains unrun. The disagreement in section 13 rests on the sphere at hinge and 3 epochs, and the hinge is the weakest tie in the Austen table.

The helix as a hierarchy remains unmeasured. It has levels, positions, and a byte-exact round trip. No operator has read it. The paper's test for a hierarchy, add a level and watch the error, has a helix version that is cheap to state and has not been run.

Resolution transfer, the paper's headline in Table 3, remains untestable in gemphi. A word representation that exists at 32 channels and at 64 and is the same word at both is what the paradigm sentence promises with its "32–64" and what nothing measures.

Linear scaling, the paper's other headline, remains out of reach for a different reason. Phinet is bandwidth bound at these sizes. The paper's cost curve is about nodes. No graph in gemphi is large enough for nodes to be the cost.

No weather work exists in gemphi and none is proposed by this paper. No Phi-4 run exists; the quest is a synthetic oracle of 256 made-up words. No partnership is agreed with anyone.

This paper is one of sixteen readings. It states what the paper says, from the paper, and what Phient has measured, from the facts file, and it draws one line between them. The line is that a relation has ranges, that long ranges pass through summaries, and that Phical stores the short ranges and has nothing yet for the long ones. Whether a family level closes that gap is what section 12 is for.

## 16. Sources

Li, Z., Kovachki, N., Azizzadenesheli, K., Liu, B., Bhattacharya, K., Stuart, A., Anandkumar, A. "Multipole Graph Neural Operator for Parametric Partial Differential Equations." arXiv:2006.09535, v1 submitted 16 June 2020, v2 19 October 2020. Abstract page: https://arxiv.org/abs/2006.09535. PDF: https://arxiv.org/pdf/2006.09535v2. Both were fetched for this paper. The abstract, contributions, problem setting, kernel decomposition, V-cycle equations, complexity argument, Darcy and Burgers setup sentences, Tables 2 through 5, and the captions of Figures 1 to 3 were read from the PDF through a text extraction. Table 1 in the PDF is a table of notation and is not quoted as numbers. The paper's training hyperparameters (optimizer, learning rate, epochs) were not recovered by our extraction and are not stated. The times in Table 2 are reported as printed; the extraction did not return the caption saying what they are times of.

NeurIPS proceedings page: https://proceedings.neurips.cc/paper/2020/hash/4b21cf96d4cf612f239a6c322b10c8fe-Abstract.html, "Part of Advances in Neural Information Processing Systems 33 (NeurIPS 2020)". The proceedings page lists the authors with Stuart before Bhattacharya; the arXiv page lists Bhattacharya before Stuart. The front matter above follows the proceedings order, as given to us; the citation in this section follows arXiv.

The ar5iv rendering at https://ar5iv.labs.arxiv.org/html/2006.09535 was fetched and returned the abstract, the introduction, and the Nyström paragraph; it was used for no number.

Every number about Phient is from PHIENT_FACTS.md in this folder. Every fact about the author is from 17_ANIMA_ANANDKUMAR_PROFILE.md in the parent folder, whose own sources are listed there: the Caltech News release of 16 March 2026, the Latent Space interview, the Wikipedia article, the FourCastNet preprint arXiv:2202.11214, the Blavatnik Awards profile, and the Caltech CMS faculty page. The third paper in this series, 03_graph_kernel_network.md, was read for continuity and for the graph kernel network's Table 8, which this paper's Table 4 extends by two rows.

Not consulted: the Fourier neural operator paper, the group's later journal treatment of neural operators, the fast multipole method literature, or any secondary account of this paper. The reading of is-a as low-rank and part-of as high-rank in section 7, and the Darcy-or-Burgers question in section 13, are our framing, not the paper's.
