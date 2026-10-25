# The kernel is a relation: message passing before the Fourier transform

*Phient interpreted papers, 3 of 16 · 2026-02-09 · on: "Neural Operator: Graph Kernel Network for Partial Differential Equations" (Li, Kovachki, Azizzadenesheli, Liu, Bhattacharya, Stuart, Anandkumar, 2020, arXiv:2003.03485)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the third of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. The paper is the first to carry the phrase "neural operator" in its title. It was posted to arXiv on 7 March 2020, before the Fourier Neural Operator that most people now mean by the phrase. It is the idea before the Fourier transform arrived, and that is why we read it. It shows the idea raw: a learned kernel, integrated over a neighbourhood, iterated a fixed number of times, with the integral done by message passing on a graph.

The paper's claim is that one set of network weights can describe a map between two spaces of functions, and therefore between any two discretisations of those spaces. The proof it offers is experimental. A network trained on a 16 × 16 grid is tested on a 241 × 241 grid and its error stays on the same scale. It reports where the method is beaten by a classical reduced basis method, and says in its last section that the cost of its own kernel grows as the square of the number of nodes.

We read the paper for one reason above the others. Its kernel takes as input two points and the values of the coefficient at those two points, and returns a matrix. That is a relation between two places under a condition. Phical's whole claim is that a word is a set of relations under conditions and nothing else. The graph kernel network puts that claim into a PDE solver and measures it. Section 13 says where the correspondence breaks.

Sections 2 to 5 state what the paper says, in its own terms and with its numbers. Sections 6 to 10 place the paper's objects inside the Phient family: the coefficient function as a word, the edge feature as a channel, the iterated kernel as the operator X ⇒ Y under c, the graph as the quest, and the missing Fourier step as the thing phinet already has. Section 11 lists what Phient has measured that bears on any of this. Section 12 names one experiment and its control.

One limit is stated at the top. Phient has run no graph kernel network. Nothing in gemphi does message passing on a graph. Every number about Phient below was measured for another purpose. Where the paper's idea has a counterpart in phinet, the counterpart is the Fourier layer that came after this paper, and phinet has not measured the version that came before.

## 2. What the paper claims, in its own terms

The abstract begins by naming what neural networks have been. They map a finite-dimensional Euclidean space to a set of classes, or one finite-dimensional space to another. The paper's purpose is to generalise this. Its words are that it wants neural networks that "can learn mappings between infinite-dimensional spaces (operators)". The key innovation, again in its words, is that "a single set of network parameters, within a carefully designed network architecture, may be used to describe mappings between infinite-dimensional spaces and between different finite-dimensional approximations of those spaces".

The construction is stated in one sentence. The infinite-dimensional map is approximated "by composing nonlinear activation functions and a class of integral operators". The integral is computed "by message passing on graph networks". That sentence is the whole method. Everything else in the paper is the choice of what goes into the kernel, how the graph is built, and how the cost is kept down.

The contributions section says what it believes to be new. Along with the contemporaneous work of Bhattacharya and co-authors, the method is claimed to be, "to the best of our knowledge, the first practical deep learning method that is designed to learn maps between infinite-dimensional spaces". The claim is guarded by "practical" and by "to the best of our knowledge". The literature review names what it is set against. Fully convolutional networks are "mesh-dependent and therefore their error grows when moving to a larger grid". Physics-informed neural networks require training a new network for every new parameter. The reduced basis method is the closest classical relative.

The practical consequence promised is that a learned network "can generalize among different approximation methods for the PDE (such as finite difference or finite element methods) and among approximations corresponding to different underlying levels of resolution and discretization". The experiments test exactly this. Train at one resolution, test at another. Train on a sample of nodes, test on a different sample. The abstract's last sentence is modest. Experiments "confirm that the proposed graph kernel network does have the desired properties and show competitive performance compared to the state of the art solvers". Section 4.3 will show a classical method beating it on the same grid.

The paper's section headings tell its shape. Introduction, with a literature review and a list of contributions. Problem setting. Graph kernel network. Experiments, split into a supervised setting, a semi-supervised setting, and a comparison with benchmarks. Discussion and future work. There is no appendix of proofs and no approximation theorem. This paper is an architecture and a set of tables.

## 3. The mathematics, plainly

The problem is a second-order elliptic equation on the unit square. Write it as −∇·(a(x)∇u(x)) = f(x) for x in D = [0,1]², with u = 0 on the boundary. The map the paper wants to learn takes the coefficient a to the solution u. The paper is explicit about a point that matters for us: "while the PDE (3) is linear, the solution operator F† is not". The equation is linear in u. The map from a to u is nonlinear.

The coefficient functions are drawn at random. The paper writes the law as μ = ψ_# N(0, (−Δ + 9I)^−2), a Gaussian random field pushed forward through a function ψ that "takes the value 12 on the positive part of the real line and 3 on the negative". So each a is piecewise constant with two values, 12 and 3.

The network is an iteration on a hidden function v. Start with v₀(x) = P(x, a(x), a_ε(x), ∇a_ε(x)) + p, where a_ε is a Gaussian-smoothed copy of a and P is a linear lift to n dimensions. Then for t from 0 to T − 1,

v_{t+1}(x) = σ( W v_t(x) + ∫_D κ_φ(x, y, a(x), a(y)) v_t(y) ν_x(dy) ).

W is an n × n matrix, the local path. The integral is the nonlocal path. The kernel κ_φ is a neural network from ℝ^{2(d+1)} to ℝ^{n×n}. It takes two points and the coefficient at each, and returns a matrix that acts on v_t(y). The measure ν_x is the Lebesgue measure restricted to a ball B(x, r) around x. At the end, u(x) = Q v_T(x) + q projects back to one dimension.

The discretisation is a graph. The nodes are the K grid points. Each node x is connected to the nodes within B(x, r). The edge feature is e(x, y) = (x, y, a(x), a(y)). The integral becomes a mean over neighbours,

v_{t+1}(x) = σ( W v_t(x) + (1/|N(x)|) Σ_{y ∈ N(x)} κ_φ(e(x, y)) v_t(y) ).

This is the message passing step of a graph neural network.

The cost is the problem. With K nodes and a radius that covers a fixed fraction of the square, the number of edges grows as K². The paper's answer is a Nyström approximation. Uniformly sample m ≪ K nodes, build the graph on them, and repeat l times. The paper's sentence is that this leads "to the more favorable scaling O(lm²)".

## 4. Its experiments and numbers as reported

The supervised experiments fix n = 64, T = 6, ReLU as σ, and κ as "a 3-layer feed-forward network with widths (6, 512, 1024, n²)", trained with Adam at learning rate 1e-4 for 200 epochs. The error reported is relative ℓ₂ test error.

Table 1 trains on a full s × s grid and tests on a full s' × s' grid, with r = 0.10 and N = 100 training pairs. The rows we could read from the PDF are these.

| train s | test s' = 16 | s' = 31 | s' = 61 |
|---|---|---|---|
| 16 | 0.0525 | 0.0591 | 0.0585 |
| 31 | 0.0787 | 0.0538 | 0.0588 |

The paper's table has a third row for s = 61 that our text extraction did not return, so we do not quote it. The paper's reading is that "for each row, the test errors at different resolutions remain on the same scale".

Table 2 varies the number of training pairs at s = s' = 31 and r = 0.10. N = 10 gives train 0.0089, test 0.0931. N = 100 gives 0.0183 and 0.0478. N = 1000 gives 0.0255 and 0.0345.

The semi-supervised section samples m nodes from the training grid and m' from the test grid. The paper sets m = m' = 200, l = 2 samples per training pair, l' = 1 per test pair, N = 100 training pairs, N' = 100 test pairs, and r = r' = 0.25. Table 3 is the resolution transfer on sampled grids.

| train s | test s' = 61 | s' = 121 | s' = 241 |
|---|---|---|---|
| 16 | 0.0717 | 0.0768 | 0.0724 |
| 31 | 0.0726 | 0.0710 | 0.0722 |
| 61 | 0.0687 | 0.0728 | 0.0723 |
| 121 | 0.0687 | 0.0664 | 0.0685 |
| 241 | 0.0649 | 0.0658 | 0.0649 |

Table 4 varies N and l at s = 241: N = 100 gives 0.0786, 0.0687, 0.0690 for l = 1, 2, 4. Table 5 varies m and m' at s = 121, r = 0.15, l = 5; at m = 800, m' = 800 gives 0.0478 and m' = 100 gives 0.1084. Table 6 varies the radius: at m = 400, r = 0.05 gives 0.099 with 3,354 edges and r = 0.40 gives 0.040 with 55,919 edges. Table 7 varies the kernel network at s = 241; the best cell is width 1024, depth 3, at 0.0591.

Table 8 is the benchmark, N = 1024 pairs on a 421 × 421 grid, trained and tested on the same full grid.

| method | s = 85 | s = 141 | s = 211 | s = 421 |
|---|---|---|---|---|
| NN | 0.1716 | 0.1716 | 0.1716 | 0.1716 |
| FCN | 0.0253 | 0.0493 | 0.0727 | 0.1097 |
| PCA+NN | 0.0299 | 0.0298 | 0.0298 | 0.0299 |
| RBM | 0.0244 | 0.0251 | 0.0255 | 0.0259 |
| GKN | 0.0346 | 0.0332 | 0.0342 | 0.0369 |

GKN here uses r = 0.25 and m = 300. The paper says the reduced basis method "has the best performance but the solutions can only be evaluated on the same mesh as the training data", and that GKN "enjoys competitive performance against all other methods while being able to generalize to different mesh geometries".

## 5. The one idea, restated

Strip the PDE away and the paper says this. A function is known at some points. To compute a new function from it, do not learn a weight for each point. Learn one rule that, given two points and what the function is at each, returns how much the state at the second point should move the state at the first. Apply that rule over every pair within reach. Add a local term. Apply a nonlinearity. Repeat a fixed number of times. Because the rule is written in terms of positions and values and never in terms of indices, it does not care how many points there are or where they lie.

The kernel network κ is the rule. The ball of radius r is what "within reach" means. The T iterations are the depth. W lets a point depend on itself.

Two consequences follow, and the paper tests both. First, resolution transfer. A rule about pairs of points and values is the same rule on a fine grid as on a coarse one, so a network trained coarse should work fine. Table 1 and Table 3 test this and the errors stay on one scale. Second, partial observation. If the rule is about pairs, it can be applied to whichever pairs are available. Table 5 tests this by training on some nodes and testing on others.

Two costs follow too, and the paper names both. The number of pairs within reach grows with the square of the number of points, so the Nyström sampling is required. And a rule about pairs, applied by a mean over neighbours, is a Monte Carlo estimate of an integral. The error of that estimate shows in Table 4: more samples l, lower error, at every N.

The idea has a relative in older mathematics. The solution of a linear elliptic equation can be written as an integral against a Green's function G(x, y), and G depends on the coefficient a. The paper's kernel sits in the same position, wrapped in enough nonlinearity to handle a nonlinear map. The paper frames its method as a class of integral operators; the Green's function comparison is ours.

For Phient the restatement is shorter still. The state at a point is what it relates to, within reach, under the condition given by the coefficient. That is the paradigm sentence with x in place of w and a in place of c. The rest of this paper is about how far that substitution can be pushed.

## 6. The coefficient as a word

In the paper the input is a function a on the unit square. It takes the value 12 or 3 at every point. Its whole content is the shape of the boundary between the two. The network sees a as a set of (x, a(x)) pairs, plus a smoothed copy and its gradient, and it can see any subset of those pairs.

In Phical the input is a word. A word is 32–64 relations, each a complex phasor on a closed circle with a count. The phinet Tensor that holds it has shape [b, w, s], complex, and the s axis is the set of relation channels. This is the function view of a word: the word is a function on its channels, valued in phasors.

The correspondence is direct at one level and strained at another. It is direct in that both objects are functions. Both are read by an operator that does not care which coordinates are present. The paper's Table 5, where a network trained on 200 sampled nodes is tested on 800, is the kind of experiment that only makes sense for a function. Phical's promise, that a word with 32 channels and a word with 64 channels are the same kind of thing, is the same promise.

It is strained in that the paper's coordinates are points in the plane with a metric on them. Two nodes are neighbours when they are within r of each other, and r is a length. Phical's channels are 64 relation kinds, laid out in the helix as 64 codons, and in the sphere tie as spherical harmonics Y_l^m with l < 8. The helix and the sphere each give the channels a geometry, but neither is the geometry of the unit square. There is no radius r on the channel axis that means what r means in the paper.

There is also the matter of value. The paper's a is real and takes two values. Phical's phasor is a complex number on a closed circle, and a channel is one byte, 256 quanta of 2π. The circle has consequences the paper never meets: the byte deadband, where a nudge of 0.06·sin d falls under half a quantum once sin d < 0.2, and nearest rounding froze every pair 8 quanta short. The paper's coefficient is a float.

So the reading is this. The paper's a is a word in the sense that it is a function read by coordinates. It differs from a word in that its coordinates have a metric Phical lacks, and its values lack a circle Phical has. Section 12 uses the first half of this and keeps the second half in view.

## 7. The edge feature as a channel under a condition

The paper's kernel takes e(x, y) = (x, y, a(x), a(y)). Two places, and the condition at each. The kernel returns an n × n matrix, and that matrix says how the state at y moves the state at x. This is a relation between two places, conditioned on what the coefficient is at each. The edge feature is the paper's "under what conditions", and the kernel is its "what it relates to".

In Phical the condition c is a word in its own right, and the relation channel is the coordinate along which two words are compared. The quest in phiecs makes the condition explicit as a kind: is-a, part-of, near, used-for, unlike. Each kind is a channel. A row in the quest oracle is a pair of words and a kind, and the trainer's job is to learn a map that, given the first word and the kind, lands on the second.

Line the two up. The paper has (x, y, a(x), a(y)) and returns a matrix. The quest has (w₁, w₂, kind) and asks for a phasor move. In both cases the learned object is a function of features of the pair rather than a table indexed by the pair. That is what lets the paper's kernel generalise to nodes it never saw, and it is what let the quest measure a within-kind MRR of 0.747 on words that share a family, against 0.015 on the wrong channel. In phiecs, as in the paper, a relation read on the wrong condition is noise.

There is one thing the paper's edge feature does that Phical's channel does not yet do. It includes the positions x and y themselves as well as the values at them. The kernel can learn that nearer pairs matter more. Phical's channel has a count but no position. The count says how many times the relation was seen. It does not say where on any axis the two words sit. The helix gives every word a position along a DNA-coded tree, 64 codons as base-4 ACGT of channel, B-DNA xyz at 10.5 codons per turn and 0.34 nm rise. It was built for storage, 18,496 words in 1.9 s at 75 MB with byte-exact round trip, and has not been used as an input to any operator. If the kernel's inputs are positions and values, the helix xyz is the position and the phasor is the value.

We do not claim this works. We claim only that the edge feature and the channel-under-condition are the same slot in two machines, and that the paper fills it with one more thing.

## 8. The iteration as X ⇒ Y under c

The paper's update is v_{t+1} = σ(W v_t + K v_t), applied T = 6 times. Read it as an operator. It takes a function v_t and returns a function v_{t+1}, and it depends on a through the kernel. So each step is X ⇒ Y under c with X = v_t, Y = v_{t+1}, c = a.

Phinet's operator has the same shape and a different kernel. Its Fno layer does the nonlocal part as a multiplication in the frequency domain: unitary FFT along the channel axis, learned complex weights on the retained modes, inverse FFT, plus a local linear path, then a nonlinearity. This paper's layer replaces the FFT with a mean over neighbours weighted by a learned matrix-valued kernel.

The difference matters. The Fourier layer assumes the kernel depends on x − y only, which is what makes it a convolution and lets the FFT do it. The graph kernel depends on x and y separately and on a(x) and a(y). The Fourier layer handles the coefficient by feeding it into v₀. The graph kernel handles it in the kernel itself. This paper's method is therefore the more general one and the more expensive one.

Where does phinet's operator sit on this line? The Austen operator trains on 204,592 examples from a pool of 1,228 words seen 20+ times, and ranks the next word against chance MRR of 0.0063. Identity gives 7.9× chance. The hinge tie at 8,360 params gives 14.7× valid and 14.6× test. The wide and deep softmax at 50,016 params gives 27.6× at 3 epochs and 30.7× valid, 30.3× test at 8 epochs, with hit@10 of 0.348. Every one of those is a Fourier-style tie. None has a kernel that depends on the condition at both ends of a pair. Phinet has not built the paper's kernel.

The paper gives a reason to think about it. In Table 6 the error at m = 400 drops from 0.099 at r = 0.05 to 0.040 at r = 0.40 while the edge count rises from 3,354 to 55,919. Reach matters, and reach costs. The Fourier layer gets full reach for the price of an FFT because it gives up dependence on the pair. Phinet inherited that trade without measuring the other side. The engine is allocation and bandwidth bound at these sizes, a 1 MB op costing 2–5 ms, so a kernel that scales as m² would be felt at once.

The reading is that phinet's Fno layer is X ⇒ Y under c with c entering through v₀, and this paper's layer is X ⇒ Y under c with c entering through the kernel. Both are operators. Only one has been measured in gemphi.

## 9. The graph as the quest

The paper turns an integral into a graph. Nodes are the sampled points, edges join points within r, and the edge feature carries the condition at both ends. Training samples m = 200 nodes from a grid of s² and does so l times per training pair. The graph is drawn fresh each time, and the network is trained to be indifferent to which view it gets.

Phiecs has a graph already, and it is the quest. The synthetic oracle has 256 words in families of 8, five kinds of relation, 1,280 rows, 293 target words, and chance MRR of 0.021. A row is an edge and the kind is its condition. The trainer's job is to move a word's phasor along the edge type to the target.

The numbers the quest has produced sit as follows. The shipped trainer reached 0.082 MRR. Attraction only with dithered rounding reached 0.594 MRR, hit@1 of 34.8%, hit@5 of 97.4%. Within kind, 0.747. On the wrong channel, 0.015. On the held-out 20%, one hop gave 0.011 and two hops gave 0.095 for is-a, 0.093 for used-for, and roughly chance for part-of, near and unlike.

The held-out numbers are the ones this paper speaks to. One hop on held-out words is at chance. Two hops on is-a and used-for is four to five times chance. The paper's whole design is that a rule about pairs, learned on some pairs, applies to pairs it has not seen, because the rule is a function of the edge feature. The quest's one-hop held-out result says the shipped rule knows the edges it saw. The two-hop result says something composes, but only on two of five kinds.

The paper's Nyström step is the part the quest lacks. Training on l fresh samples of m nodes per pair forces the network to learn the rule rather than the instance. Table 4 measures it: at N = 100, l = 1 gives 0.0786 and l = 4 gives 0.0690. The quest trains on the 1,280 rows as given. There is no resampling of the word graph, no random sub-graph per epoch, nothing that makes the trainer prove it has a rule. That is a concrete gap, and section 12 turns it into an experiment.

One caution. The paper's graph has a metric and the quest's does not. Within reach in the paper means within a distance r in the plane. Within reach in the quest means within a family, and family membership is a label the oracle assigns. Table 6 shows that reach is the strongest single lever the paper has. The quest has not turned that lever because it does not have it.

## 10. The sphere, the tensor, the helix, and the missing Fourier step

The paper never takes a Fourier transform. Phinet's operator is a Fourier operator on the channel axis. Its sphere tie, phinet::Tie::Sphere, puts one complex weight per spherical harmonic degree, with 64 = 8² channels read as Y_l^m for l < 8, and comes to 1,192 params. On Austen at hinge, 3 epochs, it gives 12.8× valid and 13.1× test, hit@10 of 0.182, in 89 s, against the circle tie's 14.7× and 14.6× at 8,360 params in 142 s. On the synthetic degree-filter task, 200 items and 40 epochs, the circle reaches MRR 0.662 with identity at 0.646 and the sphere reaches 1.000.

Every one of those ties is a choice of basis. The circle tie is Fourier on a closed circle. The sphere tie is Fourier on a sphere. Both assume the kernel is diagonal in that basis. That assumption is what makes the sphere tie small. It is also what the graph kernel network refuses to make.

So the paper is, for phinet, the road not taken. The graph kernel is what you build when you do not know the right basis or do not believe there is one. Table 7 is the cost: a kernel network of width 1024 and depth 3 to reach 0.0591 on a 241 grid. Phinet's sphere tie gets to its numbers with 1,192 params because the basis does the work.

The tensor lands here too. The paper's kernel returns an n × n matrix per edge, n = 64. That is a rank-2 tensor per pair, and there are tens of thousands of pairs in Table 6. Phinet's Tensor is [b, w, s] with s the channel axis, and its ties are one weight per mode. This paper's tensor is the message, the matrix that carries y's state to x. Phinet has no message tensor. Its ties multiply and add in a fixed basis.

The helix is the one place in the family where the paper's positional kernel might find a home. The helix gives every word xyz coordinates, 10.5 codons per turn and 0.34 nm rise, and every codon a hexagram, a sphere degree and order, and a PhiTon wavelength, band and hex. A ball of radius r on the helix is a definable thing. Whether it means anything is not measured.

The agent loop lands last. The paper's T = 6 iterations are a loop of fixed length with no agent in it. Phiano's quest ingest and generation are loops with a reader in them. The paper has nothing to say about that, and we do not pretend it does.

## 11. What Phient has already measured that bears on it

Nothing in gemphi is a graph kernel network. What follows is what has been measured that touches the paper's claims, and only that.

On the claim that one set of weights describes a map between function spaces, phinet's operator on Austen is the nearest measurement. The wide and deep softmax tie at 50,016 params reaches 30.7× chance on valid and 30.3× on test after 8 epochs, hit@10 of 0.348, from 204,592 training examples over a pool of 1,228 words. It has not been tested at a different channel count, so resolution transfer has no counterpart in gemphi.

On the claim that the right basis makes the operator small, the sphere tie is the measurement. 1,192 params to the circle's 8,360, 12.8× and 13.1× to the circle's 14.7× and 14.6×, 89 s to 142 s, on Austen hinge at 3 epochs. On the synthetic degree filter, sphere 1.000 to circle 0.662. The paper makes the opposite bet. On phinet's data the diagonal basis is enough.

On the claim that a rule about pairs generalises to unseen pairs, the quest is the measurement, and it goes against the shipped trainer. Attraction only with dither reaches 0.594 MRR and hit@5 of 97.4% on seen rows. Held-out one hop is 0.011 against chance of 0.021. Held-out two hops is 0.095 on is-a and 0.093 on used-for, chance on the other three kinds. The paper's Table 5 shows a network trained on 200 nodes tested on 800 unseen nodes at 0.0562. Phient has no such number. What it has says the shipped rule does not yet transfer.

On the claim that the kernel should depend on the condition at both ends, the quest's wrong-channel figure of 0.015 and within-kind figure of 0.747 say that the condition is decisive. That is consistent with the paper's design and is not a test of it.

On cost, the engine is allocation and bandwidth bound at these sizes, with a 1 MB op at 2–5 ms. The paper's edge counts, 55,919 at m = 400 and r = 0.40, would each carry an n × n matrix at n = 64. That is an allocation pattern phinet has not tried and would feel.

On the byte, the deadband result stands apart. A channel is one byte, 256 quanta of 2π. The nudge 0.06·sin d falls under half a quantum once sin d < 0.2, and nearest rounding froze every pair 8 quanta short. Dither fixes the freeze and costs corpus attribution 71% to 65%, so it ships off. Any port of the paper's kernel into a byte channel would meet this floor.

Persona attribution and dialogue do not bear on this paper.

## 12. The experiment: a pair kernel in the quest, with a control

The paper's idea, cut down to what phinet can hold, is this. Replace a tie that has one weight per mode with a tie that has a small network taking features of the pair and returning the move. Train it with fresh sub-samples of the graph each epoch. Test it on pairs it never saw.

The place to run it is the quest, because the quest already has a graph with typed edges and a held-out split, and because its held-out numbers are the weakest in the facts file. The experiment has four parts.

The data is the phiecs synthetic oracle as it stands: 256 words, families of 8, kinds is-a, part-of, near, used-for, unlike, 1,280 rows, 293 targets, held-out 20%. Chance MRR is 0.021.

The treatment is a pair kernel. For a row (w₁, w₂, kind), the kernel takes the phasors of w₁ and w₂ on the kind's channel, and the kind, and returns a move for w₁. Following the paper, the kernel is a small feed-forward network, and following the paper's Table 7, it starts at width 64 and depth 2, the smallest cell reported. Each epoch draws l = 2 random sub-graphs of m rows from the training rows and trains on those, so the kernel never sees the same graph twice. This is the paper's Nyström step transplanted whole.

The control is the shipped attraction-only trainer with dithered rounding, retrained on the same sub-sampled schedule but with its existing per-channel move. This separates two things the paper bundles together: the pair kernel and the resampling. If the control improves on held-out with resampling alone, the gain is the sampling. If only the treatment improves, the gain is the kernel.

The measurements are the quest's own: MRR, hit@1, hit@5 on the seen rows, within kind, wrong channel, and held-out MRR at one hop and two hops per kind. The seen-row baseline to beat is 0.594 MRR. The held-out baseline is 0.011 at one hop, 0.095 and 0.093 at two hops on is-a and used-for, chance on the rest. The dithered rounding stays on in both arms and its cost is known.

A second, smaller experiment follows if the first shows anything. Give the kernel the helix xyz of w₁ and w₂ as two more inputs, exactly as the paper gives its kernel x and y. The control is the same kernel without them.

What the experiment cannot test is resolution transfer, the paper's headline. The quest has one channel count. A version that trains at 32 channels and tests at 64 would need a word representation that exists at both, and that has not been built.

## 13. Where we disagree, and where the analogy breaks

The analogy breaks in three places and we disagree with the paper in one.

The first break is the metric. The paper's domain is the unit square with a Euclidean distance, and the ball B(x, r) is the strongest lever in its tables. Phical's channel axis has no such distance. The circle tie and the sphere tie give the channels a geometry, but it is the geometry of a basis, used to make the kernel diagonal rather than to decide which pairs are within reach. The helix gives words a position, but that position was built for storage and has no measured relation to meaning. Any port of the paper's ball into phinet would have to invent a radius.

The second break is the value. The paper's coefficient is a float taking two values, 12 and 3. Its hidden state v is a float vector of width 64. Phical's channel is a phasor on a closed circle, quantised to one byte. The byte quantisation is a floor the paper's method never meets. The deadband, where the nudge falls under half a quantum at sin d < 0.2 and rounding froze every pair 8 quanta short, would act on a pair kernel exactly as it acts on the shipped trainer.

The third break is the loop. The paper runs T = 6 fixed iterations on a single input and stops. There is no reader. Phiano's generation runs a loop with a reader that ranks candidates, and its decoding numbers, operator nearest-k at perplexity 66.9 against a trigram sampler at 13.3, are numbers about that loop. The paper does not touch that.

The disagreement is about what to hold fixed. The paper holds the kernel's form fixed and lets its inputs vary, so the kernel is a function of positions and values. Later the same group would hold the basis fixed and let the weights per mode vary, and that is the road phinet took. Our sphere result says that on phinet's data the fixed basis wins on params, on time, and on the degree filter task. The paper's own argument for its method is generality. We think generality is worth the price only when the basis is unknown, and on the channel axis Phical has committed to a basis. So we read this paper as the right idea at the wrong level for Phical's channels, and possibly the right level for the helix, where the basis is unknown and the metric exists.

We hold that view lightly. It rests on one sphere measurement at hinge and three epochs. The sphere at softmax and at wide and deep is unrun.

## 14. The person and the moment

The paper is dated March 2020. She was two years into her two posts, Bren Professor at Caltech and senior director of machine learning research at NVIDIA, both from 2018. The Caltech post came with the co-founding of the AI4Science initiative, and this paper is what that initiative looked like at the start: a seven-author Caltech paper with her name at the end, on a PDE, with no tensor in it.

The decade before this paper was tensors. Her first ten years, at UC Irvine and then Amazon, were about the recognition that the moments of a latent variable model form a tensor whose decomposition recovers the model, with guarantees. The first two papers in this series are from that decade. This paper is the turn. There is no decomposition in it and no guarantee. There is an architecture and eight tables. The method she has described for her later work, in her own words, is "all of the things that work with deep learning, let's take them, but make them a bit more principled". This paper takes message passing on graphs, which worked, and makes it an integral operator, which is principled.

The paper was also the first of a sequence. Later in 2020 the same seven authors would post the Fourier Neural Operator, which does the integral as a multiplication in frequency space, and which is the method Quanta profiled in 2021 and the method FourCastNet put on the whole planet in 2022. This paper is the version where the integral is still an integral. Its last section says where its cost lies, in edges that scale as K², and proposes multi-grid and fast multipole methods to bring that down. The Fourier transform was the answer that arrived instead.

Her view now is that transformers cannot be the substrate for physics at scale, because a few hundred grid points per dimension is a context of hundreds of billions to a trillion tokens, and an operator that learns in function space treats scale as what it is. This paper is an early instance of that argument: a network whose parameters do not depend on the grid, tested by changing the grid.

The recognition came later. The Time100 lists, the IEEE Kiyo Tomiyasu Award, the Blavatnik Award, the UN Scientific Advisory Board appointment on 16 March 2026, are all after this paper and mostly after FourCastNet. In March 2020 she was a professor and a research director posting a preprint. This paper does not claim she has read anything of Phient's. A partnership is something to earn with a result she would recognise: an operator over a function space she has not seen, measured the way she measures.

## 15. What remains

Everything in section 12 remains. Phient has no pair kernel, no sub-graph resampling in the quest trainer, and no held-out number better than chance at one hop. The first experiment is the one to run. Its cost is bounded: the quest is 1,280 rows, the kernel starts at width 64 and depth 2, and the engine is fast at these sizes. Its risk is that the byte deadband swallows the kernel's moves as it swallowed the shipped trainer's, and the dithered rounding that fixes that costs corpus attribution 71% to 65%, so the trade will have to be reported.

The helix experiment remains behind it. The helix has coordinates and the paper's kernel wants coordinates. Whether the helix's xyz means anything to an operator is a question with a cheap test and no answer.

The resolution transfer claim, the paper's headline, remains untestable in gemphi as it stands. A word representation that exists at 32 channels and at 64 channels and is the same word at both is what the paradigm sentence promises with its "32–64" and what nothing yet measures.

The sphere at softmax and at wide and deep remains unrun. Section 13's disagreement with the paper rests on the sphere at hinge, and the hinge is the weakest tie in the Austen table. If the sphere loses its edge at wide and deep, the argument that a fixed basis beats a pair kernel on the channel axis loses its evidence.

No weather work exists in gemphi and none is proposed by this paper. The paper's PDE is a Darcy-type equation on a square. The group's later work took the same operator to the atmosphere. Phient's operators are over words. The step from a coefficient field on a square to a word on a circle is the whole of this paper's interpretation, and it is an interpretation rather than a result.

No Phi-4 run exists. The quest is a synthetic oracle. Every held-out number in this paper is from 256 made-up words in families of 8. The paper's data is synthetic too, but its tables go up to N = 1024 pairs on a 421 × 421 grid, and the quest has not scaled at all.

No partnership is agreed with anyone. This paper is one of sixteen readings. It states what the paper says, from the paper, and what Phient has measured, from the facts file, and it draws one line between them. The line is that the kernel is a relation under a condition, and that Phical stores nothing else. Whether the line holds under measurement is what section 12 is for.

## 16. Sources

Li, Z., Kovachki, N., Azizzadenesheli, K., Liu, B., Bhattacharya, K., Stuart, A., Anandkumar, A. "Neural Operator: Graph Kernel Network for Partial Differential Equations." arXiv:2003.03485, v1, submitted 7 March 2020. Abstract page: https://arxiv.org/abs/2003.03485. PDF: https://arxiv.org/pdf/2003.03485v1. Both were fetched for this paper. The abstract, contributions, problem setting, architecture equations, hyperparameters, Tables 2 through 8, the first two rows of Table 1, the Figure 1 caption, and the discussion section were read from the PDF. The third row of Table 1 was not recovered by our text extraction and is not quoted. The ar5iv HTML rendering at https://ar5iv.labs.arxiv.org/html/2003.03485 was requested and returned only the abstract and introduction; it was not used for any number.

Every number about Phient is from PHIENT_FACTS.md in this folder. Every fact about the author is from 17_ANIMA_ANANDKUMAR_PROFILE.md in the parent folder, whose own sources are listed there: the Caltech News release of 16 March 2026 on the UN Scientific Advisory Board appointment, the Latent Space interview, the Wikipedia article, the FourCastNet preprint arXiv:2202.11214, the Blavatnik Awards profile, and the Caltech CMS faculty page. No other source was used.

Not consulted: the Fourier Neural Operator paper that followed this one, the group's later journal treatment of neural operators, or any secondary account of this paper. Where this paper refers to those, it does so only through the profile's statements about them. The Green's function comparison in section 5 is our own framing, not the paper's.
