# Three words at once: the moment tensor and the relation circle

*Phient interpreted papers, 1 of 16 · 2026-01-12 · on: "Tensor Decompositions for Learning Latent Variable Models" (Anandkumar, Ge, Hsu, Kakade, Telgarsky, 2014, Journal of Machine Learning Research 15:2773–2832; arXiv:1210.7559)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the first of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. The paper is the one that made her name before neural operators did. Its claim is that for a wide class of latent variable models the second and third moments of what you can observe carry the model's parameters as a tensor with a special structure, and that this structure can be pulled apart by a power iteration that behaves, up to noise, the way the matrix power method behaves. The paper is theory. It has no experiments. Its result is a perturbation theorem with explicit constants, and a set of reductions that show which models fall under it.

We read it here for two reasons. The first is that the paper's central object, the third moment E[x₁ ⊗ x₂ ⊗ x₃], is the statistics of three words drawn under one condition. That is the paradigm sentence at the top of this page written as a tensor. The second is that the paper's method, whitening followed by power iteration followed by deflation, is a way of finding the coordinates in which a word's relations become orthogonal, and Phical already commits to a fixed set of 32–64 coordinates it calls channels. The paper asks where those coordinates come from. Phical mostly assumes them.

Sections 2 to 5 state what the paper says, in its own terms and with the numbers it reports, which are constants in theorems and not measurements on data. Sections 6 to 10 place the paper's objects inside the Phient family: the observed vector as a word's function, the eigenvector as a channel, the power map as an operator X ⇒ Y under c, and the deflation loop as the quest. Section 11 lists what Phient has measured that bears on any of this, all of it from the facts file, none of it a tensor decomposition. Section 12 names one experiment that would test the paper's idea inside phinet, with its control. Section 13 says where the analogy breaks, which it does in at least three places. Section 14 is about the person and the years in which she wrote this. Section 15 is what is left to do.

One limit is stated up front. Phient has run no tensor decomposition. Nothing in this paper's method exists in gemphi. Every number about Phient below is one already measured for another purpose, and where the paper's idea has not been tested in phinet, this page says so rather than guessing at what the result would be.

## 2. What the paper claims, in its own terms

The paper's abstract says the work "considers a computationally and statistically efficient parameter estimation method for a wide class of latent variable models---including Gaussian mixture models, hidden Markov models, and latent Dirichlet allocation", and that the method works "by exploiting a certain tensor structure in their low-order observable moments (typically, of second- and third-order)." The parameter estimation problem is reduced to "the problem of extracting a certain (orthogonal) decomposition of a symmetric tensor derived from the moments", and that decomposition is described as "a natural generalization of the singular value decomposition for matrices."

The authors are careful to say that tensor decompositions are, in general, hard. They cite Hillar and Lim for the statement that obtaining such approximations for general symmetric tensors is NP-hard. Their point is that the tensors that come out of latent variable models are not general. They are orthogonally decomposable after a whitening step, and for that class the decomposition "can be efficiently obtained by a variety of approaches, including power iterations and maximization approaches (similar to matrix eigenvector methods)."

The main technical contribution, in the abstract's words, is "a detailed analysis of a robust tensor power method", which establishes "an analogue of Wedin's perturbation theorem for the singular vectors of matrices." Wedin's theorem says how much the singular vectors of a matrix move when the matrix is perturbed. The paper gives the same kind of statement for the eigenvectors that its power method returns from a perturbed tensor. The authors note one difference of kind: for matrices the perturbed object still has singular vectors, so the theorem is about the object; for tensors the perturbed object need not have an orthogonal decomposition at all, so the theorem is about the algorithm. It says what Algorithm 1 returns, not what the tensor has.

The paper places itself in a long line. It names the method of moments as classical, citing Pearson (1894). It names the tensor power method's origin in the independent component analysis literature, citing De Lathauwer et al. (2000), Zhang and Golub (2001) and Kofidis and Regalia (2002). It treats the exchangeable single topic model as the running example and then shows, model by model, that spherical Gaussian mixtures, hidden Markov models, latent Dirichlet allocation, independent component analysis and multi-view models all produce moment tensors of the required form after a model-specific correction.

The version we read is arXiv 1210.7559v4, dated 13 November 2014, which is the version matching the JMLR publication. The first version was posted on 29 October 2012. Five authors are listed: Animashree Anandkumar, Rong Ge, Daniel Hsu, Sham M. Kakade and Matus Telgarsky.

## 3. The mathematics, plainly

Take a symmetric third-order tensor T in ⊗³ℝⁿ. The paper says T has an orthogonal decomposition when

T = Σᵢ₌₁ᵏ λᵢ vᵢ ⊗ vᵢ ⊗ vᵢ

with the vᵢ orthonormal and each λᵢ > 0. A unit vector u is an eigenvector of T with eigenvalue λ when T(I, u, u) = λu, where T(I, u, u) contracts the second and third slots of T against u. Each vᵢ is such an eigenvector with eigenvalue λᵢ. There are other eigenvectors too, certain combinations of the vᵢ, and the paper rules them out by defining a robust eigenvector as an attracting fixed point of the map θ ↦ T(I, θ, θ) / ‖T(I, θ, θ)‖. Only the vᵢ are robust in this sense.

The running model is the exchangeable single topic model. A document has a topic h drawn from k topics with probabilities wᵢ, and three words x₁, x₂, x₃ are drawn independently given h, each a one-hot vector in ℝᵈ. Then

M₂ = E[x₁ ⊗ x₂] = Σᵢ wᵢ μᵢ ⊗ μᵢ,
M₃ = E[x₁ ⊗ x₂ ⊗ x₃] = Σᵢ wᵢ μᵢ ⊗ μᵢ ⊗ μᵢ,

where μᵢ is the word distribution of topic i. The μᵢ are not orthogonal, so whitening is needed. Let M₂ = U D Uᵀ restricted to its k nonzero eigenvalues and define W = U D^{−1/2}. Then M̃₃ = M₃(W, W, W) = Σᵢ wᵢ^{−1/2} μ̃ᵢ^{⊗3} with μ̃ᵢ = √wᵢ Wᵀ μᵢ orthonormal. The recovered eigenvalue for component i is 1/√wᵢ, which gives the topic weight, and the eigenvector gives the topic back through W.

The power method is the iteration θₜ = T(I, θₜ₋₁, θₜ₋₁) / ‖T(I, θₜ₋₁, θₜ₋₁)‖. Lemma 5.1 says that when the largest of the |λᵢ vᵢᵀθ₀| is unique the iteration converges to that vᵢ quadratically:

‖v₁ − θₜ‖² ≤ (2λ₁² Σᵢ≥₂ λᵢ^{−2}) · |λ₂ v₂ᵀθ₀ / λ₁ v₁ᵀθ₀|^{2^{t+1}}.

This is a stronger rate than the matrix power method has, because the tensor map is quadratic in θ and each step squares the ratio of the wrong component to the right one.

Algorithm 1 is the robust version. Draw L random starting points on the unit sphere, run N power steps from each, keep the one with the largest T(θ, θ, θ), run more steps from it, record the pair (λ̂, v̂), and subtract λ̂ v̂^{⊗3} from T. Repeat k times. The subtraction is deflation.

Theorem 5.1 is the perturbation result. If T̂ = T + E with ‖E‖ ≤ ε, where the norm is sup over unit θ of |E(θ, θ, θ)|, and if ε ≤ C₁ λ_min / k, N ≥ C₂ (log k + log log(λ_max / ε)), and L = poly(k) log(1/η), then with probability at least 1 − η there is a permutation π with ‖v_π(j) − v̂ⱼ‖ ≤ 8ε / λ_π(j), |λ_π(j) − λ̂ⱼ| ≤ 5ε, and ‖T − Σⱼ λ̂ⱼ v̂ⱼ^{⊗3}‖ ≤ 55ε.

## 4. Its experiments and numbers as reported

The paper reports no experiments. It contains no dataset, no table of accuracies, no plot of convergence on data. This is stated plainly because the brief for these interpreted papers demands that numbers about her paper be numbers her paper contains, and this one contains constants in theorems and counts of operations, nothing else. The reader who wants empirical results must look to later work, which this page does not cover.

What the paper does report are the quantities in the theorems, and these read as numbers. The tolerance is ε ≤ C₁ λ_min / k. In the whitened topic model λᵢ = 1/√wᵢ, so λ_min belongs to the most frequent topic, and the tolerance shrinks as k grows. The eigenvector error is 8ε / λ, so rare topics, with large λ, are recovered more accurately in direction for a given ε, while the eigenvalue error 5ε does not depend on λ at all. The total residual is 55ε. The paper's contribution over a naive analysis is that this residual does not grow with each deflation step. Its own words: "a naive argument would entail error accumulation in each deflation step, which we show can in fact be avoided."

The iteration counts are also numbers. N ≥ C₂ (log k + log log(λ_max / ε)) power steps per restart is the price of quadratic convergence; the double logarithm is its signature. L = poly(k) log(1/η) restarts is the price of finding a good start on the sphere by chance. The paper does not give the polynomial's degree in the theorem statement as we read it, and we do not supply one.

Section 6 gives the operation counts for practice. In the dense case, where the whitened tensor is an unstructured k × k × k array, extracting all k eigenvectors costs O(k^{5+δ} (log k + log log(1/ε))) time. For sparse bag-of-words data the paper forms the contribution of a document of length ℓ with count vector c to the third moment without enumerating all C(ℓ, 3) word triples, by expanding c ⊗ c ⊗ c and subtracting the diagonal and partly diagonal terms with explicit coefficients. The cost of the whitened tensor's action is O(dk + N), where N is the number of nonzero entries in the term-document matrix and d the vocabulary. For the whitening itself the paper describes a randomized method that avoids forming the d × d second moment.

Two things are absent that a Phient reader will look for. There is no number for how many samples are needed in practice for a given ε, though the theorem, joined to a concentration bound on empirical moments, yields one in principle. And there is no statement about the third moment of real text beyond the single-topic and LDA models. The paper's models are its models. We say what they are and stop.

## 5. The one idea, restated

The one idea is this. If three observations are independent given one hidden cause, then the tensor of their joint expectation is a sum of rank-one terms, one per value of the cause, and a matrix built from two of the observations tells you the basis in which those terms become orthogonal. Once orthogonal, the terms are the fixed points of a simple map, and a power iteration finds them one at a time.

Say it in words. The second moment tells you how to measure. The third moment tells you what there is. Two views of a thing can be explained by a continuum of mixtures, because a symmetric matrix has infinitely many rank-k decompositions. Three views pin the mixture down, because a symmetric tensor with an orthogonal decomposition has a unique one, up to order. That is why the paper needs the third order, and why it says so in its first paragraph.

The second part of the idea is that this uniqueness is also an algorithm. The map θ ↦ T(I, θ, θ) is quadratic in θ. Starting from a random θ and applying it, the component along the largest λᵢ vᵢᵀθ grows fastest, and after normalisation the others die off at a doubly exponential rate. The robust part of the method lives in the wrapper: many random starts, a comparison by T(θ, θ, θ), and a subtraction that removes what has been found so the next run finds something else.

The third part is what the perturbation theorem does with noise. Real moments are estimated from samples, so T̂ = T + E. The theorem says that if E is small compared to λ_min / k, then the algorithm returns vectors within 8ε / λ of the true ones and values within 5ε, and the errors do not pile up through k deflations. This is the sense in which the paper claims an analogue of Wedin: a bound on how much the answer moves for a given amount of noise in the question.

For Phient the idea is worth keeping in a smaller form. Meaning, in the paradigm sentence, is a side effect of pairs of words moving under a condition. The paper says that pairs cannot identify what the words are made of, and that triples can. If the relation channels of Phical are the coordinates in which a word's relations are orthogonal, the paper offers a reason why such coordinates exist and a way to find them from co-occurrence alone. If they are not, the paper offers a test that would show it. Either way the idea, restated for us, is that three words at once under one condition carry information that two do not, and that the extra information is the identity of the condition.

## 6. A word as a function, and the moment tensor as three words under one condition

Phical's word is a function. It takes a relation channel and returns a complex phasor and a count, 32–64 times over. The paper's word is a one-hot vector in ℝᵈ, and its function is the topic's distribution μᵢ over the vocabulary. These look far apart, and the gap matters, but the operation the paper performs on its word is one Phical recognises.

E[x₁ ⊗ x₂] for one-hot words is the co-occurrence matrix of word pairs within a document, normalised. E[x₁ ⊗ x₂ ⊗ x₃] is the co-occurrence tensor of triples. The single topic model says that within one document the words are exchangeable, which means the order does not matter and the condition is the same for all three. In the paradigm sentence, the document is c, the condition. The three words are three draws of w under c. The tensor M₃ is the statistics of "what does w relate to, under c" taken over all c and read three at a time.

The paradigm sentence says meaning is a side effect of pairs moving under a condition. The paper's second moment is that sentence's object. Its third moment is the sentence's object with one more word in the room. What the paper shows is that the second moment fixes a metric (the whitening W) and the third moment fixes the components. Pairs give you the space; triples give you the axes.

Where does Phical's function sit in this? A Phical word is not a vector in ℝᵈ indexed by vocabulary. It is indexed by channel, and its entries are phasors, complex numbers on a circle. The most literal translation is to read the whitened topic vector μ̃ᵢ, which lives in ℝᵏ, as the analogue of a word's channel vector, with k playing the role of the channel count. In the paper, k is the number of topics. In Phical, the channel count is 32–64 and is fixed by design, not discovered.

There is a second translation. Phinet's Tensor is [b, w, s] complex: batch, word, spectrum. A word is a row of s complex entries. The paper's whitened tensor is k × k × k real. If s were k and the phases were dropped, M̃₃ would be the tensor of triple products of word rows, averaged over conditions. The phases cannot be dropped, and Section 13 returns to that. But the shape matches.

The word-as-function view also names what the paper cannot do. The paper's μᵢ is a point in the simplex, a fixed distribution per topic. Phical's word moves. Its relations shift under a condition, which is what the operator does. The paper estimates static parameters and has no operator. Its tensor is a summary of a corpus. That distinction organises the rest of this page.

## 7. The relation channel as a coordinate: whitening chooses the basis

The step in the paper that Phical should read most carefully is whitening. Before whitening, the components μᵢ are in whatever coordinates the vocabulary gives, and they are not orthogonal. After whitening by W = U D^{−1/2}, the components μ̃ᵢ = √wᵢ Wᵀμᵢ are orthonormal. The power method then finds them as the only robust fixed points. So the paper's answer to "what are the right coordinates for a word" is: the ones in which the second moment is the identity and the third moment is diagonal.

Phical's answer is different in kind. A relation channel is a coordinate chosen in advance. A word has 32–64 of them, each a phasor with a count. The facts file says the sphere tie reads the 64 channels as spherical harmonics Y_l^m with l < 8, and the helix reads them as 64 codons. Neither reading comes from data. Both are structures imposed so that the operator has something regular to act on.

The paper suggests a question Phient has not asked: are the channels, as trained, orthogonal in the paper's sense? That would mean two things. The second moment of channel vectors across the lexicon, E[w ⊗ w] over words, would be close to diagonal. And the third moment over triples of words that share a condition would be close to a sum of rank-one terms aligned with the channel axes. The first is a 64 × 64 matrix over the 18,496 words the helix wrote. The second is a 64³ tensor, small enough to form directly. Neither has been done. Section 12 proposes the first as the cheap half of a test.

If the channels are not orthogonal in this sense, the paper says what to do: whiten. Compute the second moment of channel vectors, take W = U D^{−1/2}, and re-express every word as Wᵀw. Whether an operator trained on the rotated basis ranks next words better, worse or the same as one trained on the raw channels is a question with an answer, and no one has it.

There is a subtlety the paper handles that Phical would have to handle too. The whitening matrix depends on the sample, so it is itself noisy, and the theorem's ε includes the error of whitening with an estimated W. In Phical the analogue is that a lexicon trained on Austen would give a different W from one trained on Chesterton. The facts file records persona attribution at 71% across three writers, so the lexicon does carry writer-specific structure. Whether that structure lives in the covariance of the channels is unmeasured.

The channel, then, is the paper's eigenvector, stated in advance. The paper earns its eigenvector by whitening and iteration. Phical takes its channel from a design choice about circles and harmonics. Both are hypotheses about coordinates and only one has been tested against a third moment.

## 8. The operator as X ⇒ Y under c: the power map is an operator with a fixed point

Phical's operator is X ⇒ Y under c: take a word's function, move it, and the result is another word's function, conditioned. Phinet trains such an operator in the frequency domain, with complex weights acting mode by mode. The facts file reports what it does on Austen: 30.7× chance on held-out ranking with 50,016 parameters at 8 epochs.

The paper has one operator, and it is fixed. It is the map θ ↦ T(I, θ, θ) / ‖T(I, θ, θ)‖. It takes a unit vector and returns a unit vector. Its fixed points are the components. Read as X ⇒ Y: X is a guess at a component, Y is a better guess, and c is the tensor T, the corpus's third moment. The condition is the whole corpus.

The contrast with phinet's operator is sharp. Phinet's operator is learned and applied once, from one word to the next. The paper's is fixed and applied N times until it stops moving. Phinet's has parameters; the paper's has none. Phinet's is linear in the frequency domain per mode; the paper's is quadratic in θ. What they share is the shape of the claim: something about the corpus, read as an operator over functions, has an action whose fixed structure is the meaning we want.

There is a way to make the two touch. The facts file says phinet has HRR, holographic reduced representation, the binding of two vectors by circular convolution, equivalently a product of their spectra. If T were the average over conditions of triple products of words, then T(I, θ, θ) would be, for each condition, the third word weighted by how much θ agrees with the first two. That is a bound pair looked up against a memory. In HRR terms it is an unbinding. The power iteration would then read as repeatedly asking the memory "given that I look like this twice, what do I look like a third time" and taking the answer as the new question. Whether phinet's HRR admits this reading in code is a question for the code, and this page does not claim it does.

The fixed-point view also says something about phinet's identity baseline, 7.9× chance on Austen. Identity returns X unchanged, so every word is its own fixed point. The trained operator improves on identity by a factor of nearly four in MRR. The paper's operator improves on a random θ by converging to a component. In both cases the measure is the distance between what the operator returns and something the corpus says is right. The paper's something is a component of the third moment. Phinet's is the next word. These are different targets and this page does not conflate them.

## 9. Where the tensor lands: phinet's [b, w, s], the sphere, and the count

Phinet's Tensor is [b, w, s] complex. The paper's tensor is k × k × k real and symmetric. Their names coincide and their meanings do not, so this section is about the exact places where a Phient reader can put the paper's object down inside phinet without breaking either.

The first place is the third moment of channel vectors. Take a lexicon of words each with s = 64 channels. Take triples of words that share a condition, which in phinet's training data means the pairs in the 204,592 Austen examples extended by a third word from the same context. Form the 64 × 64 × 64 tensor of averaged triple outer products. It has 262,144 entries. The facts file says a 1 MB op costs 2–5 ms in phinet, and this tensor at complex double precision is 4 MB. Forming it is cheap. Nothing in gemphi forms it today.

The second place is the sphere. The facts file says phinet::Tie::Sphere reads the 64 channels as Y_l^m with l < 8 and learns one complex weight per degree. A third moment over spherical-harmonic coefficients has its own structure, since the product of three harmonics integrates to a Wigner 3j coefficient, which is zero unless the degrees satisfy a triangle inequality and the orders sum to zero. So if the channels really are harmonics, the third-moment tensor is sparse in a known pattern before any data is seen. The paper treats T as dense. The sphere tie predicts a pattern. Whether the empirical third moment of sphere-tied words shows the 3j sparsity is a test of the sphere reading that the paper's tensor makes possible. It is unrun.

The third place is the count. A Phical relation is a phasor with a count. The paper's moment is an expectation, a count-weighted average, and the count is the paper's wᵢ. In the whitened tensor the eigenvalue is 1/√wᵢ, so heavier components have smaller eigenvalues and are found later by a power method that takes the largest first. This is backwards from what a Phical reader expects, where the heaviest relation dominates. In the paper's basis the rare component is the loud one. A Phical channel with a low count would, in this reading, be the one the power method finds first.

The fourth place is where the tensor does not land. The paper's tensor is real. Phinet's is complex, with phases on a closed circle, one byte per channel, 256 quanta of 2π. A third moment of phasors is a complex tensor whose entries depend on the sum of three phases. The paper's theory is about real symmetric tensors with positive eigenvalues. Extending it to complex symmetric tensors is not in the paper, and it is not claimed here to work.

## 10. Where the loop lands: deflation as the quest, and the helix as the record

The paper's Algorithm 1 is a loop. Find a component, subtract it, find the next. It runs k times, once per hidden cause. Each pass starts with L random points on the sphere, iterates N times from each, keeps the best, and deflates. The theorem's contribution is that the errors of the k passes do not accumulate.

Phient has a loop too, the quest. The facts file describes it: a phiecs synthetic oracle with 256 words in families of 8, five kinds of relation, 1,280 rows, 293 target words, chance MRR 0.021. The trainer asks the oracle about a word, gets a relation, and moves the word's phasor on the channel for that kind. The measured result with the attraction-only trainer and dithered rounding is 0.594 MRR, hit@1 34.8%, hit@5 97.4%, within-kind 0.747, wrong channel 0.015.

The parallel is structural. The paper's deflation removes a found component so the next search is not distracted by it. The quest's attraction moves a word toward its relation so the next query finds it already there. Both are sequences of local updates against a fixed target. In the paper the target is T and the update is subtraction. In the quest the target is the oracle and the update is a nudge on a circle. The paper proves its loop does not compound error; the quest measures its loop and finds within-kind retrieval at 0.747 with the wrong channel at 0.015. The quest lacks a bound. The paper lacks a measurement.

The paper's random restarts have a partner in the quest's dither. The facts file says nearest rounding froze every pair 8 quanta short, and that dithered rounding moved the shipped 0.082 MRR to 0.594. The paper's L random starts exist because a deterministic start can sit near a saddle. The quest's dither exists because a deterministic rounding sits on a byte boundary. In both, noise added on purpose gets the iteration off a point it would otherwise never leave. Phient did not learn this from the paper. The two arrived at the same fix from different sides.

The helix is where the record of the loop lands. The facts file says the lexicon can be written as a DNA-coded tree, 64 codons per word, 18,496 words in 1.9 s, 75 MB, byte-exact round trip. The paper's output is k pairs (λ̂ⱼ, v̂ⱼ). If the channels were the paper's eigenvectors, the helix would be a way of writing down the decomposition, one codon per channel, with the count as the λ. That is a reading. Whether a decomposition of the third moment would map onto the helix layout is not measured.

The two loops share one more feature, which is where they stop. The paper stops at k. The quest stops when the oracle is exhausted. Neither has a criterion drawn from the data itself.

## 11. What Phient has already measured that bears on it

Nothing in gemphi is a tensor decomposition. This section lists, from the facts file only, the measurements that a reader of the paper would want to know before deciding whether the paper's idea applies, and says for each what it does and does not tell us.

| Phient measurement | value (facts file) | bearing on the paper |
|---|---|---|
| phinet vs PyTorch reference | 62/62 parity tests | the tensor engine is trusted; forming a third moment would be a new op on a tested base |
| Operator on Austen, pool 1,228, 204,592 examples, chance MRR 0.0063 | identity 7.9×; hinge 8,360 params 14.7×/14.6×; softmax β=8 18.0×/18.8×; wide+deep 50,016 params 30.7×/30.3×, hit@10 0.348 | a second-order pair statistic is enough to learn a next-word operator well above chance; says nothing about triples |
| Sphere tie | 1,192 params; Austen hinge 12.8×/13.1×, hit@10 0.182, 89 s vs circle 14.7×/14.6×, 142 s; synthetic degree filter circle 0.662, sphere 1.000 | the 64 channels can be read as harmonics with small loss; a third moment of harmonics has predicted sparsity, unmeasured |
| Persona attribution, three writers, chance 33% | 71% (74/65/86) | the lexicon carries condition-specific structure, which is what the paper's h would be |
| Dialogue persona | 44% sector reader / 72% KN panel, chance 33% | same, weaker |
| Quest, 256 words, chance MRR 0.021 | 0.594 MRR, hit@1 34.8%, hit@5 97.4%; within-kind 0.747; wrong channel 0.015; held-out one hop 0.011, two hops is-a 0.095, used-for 0.093 | a loop of local updates against an oracle converges; no bound |
| Byte deadband | 256 quanta; nudge under half a quantum once sin d < 0.2; dither 71% → 65% attribution | any decomposition would be read through one byte per channel; precision is bounded |
| Helix | 18,496 words, 1.9 s, 75 MB, byte-exact | the lexicon can be dumped and read back exactly, so a whitened copy could be stored beside it |
| Engine | 1 MB op 2–5 ms, bandwidth-bound | a 64³ complex tensor is 4 MB; forming it is cheap |

Three readings follow. First, the operator results are second-order. Every training example is a pair (X, Y) under c. The paper says pairs do not identify the components. Phinet's 30.7× shows the pair statistic is learnable and says nothing about whether the channels are components in the paper's sense. Second, the persona numbers show that the condition c is real in the lexicon and can be recovered from held-out text at roughly twice chance. Whether that mark is a rank-one term in a third moment is unknown. Third, the quest's within-kind 0.747 against wrong-channel 0.015 is the closest thing Phient has to orthogonality across channels. It is a property of the trainer on a synthetic oracle, untested on Austen; the facts file says no Phi-4 run exists.

What is not measured, and must be said: the second moment of channel vectors across the lexicon; the third moment of any triple; any whitening; any power iteration on any tensor; any deflation; any comparison of channel axes to eigenvectors.

## 12. The experiment: a third moment on Austen, with a control

The test is whether the paper's structure is present in Phical's channels. It has a cheap half and a dear half, and the cheap half should be run first because a null result there ends the question.

Cheap half. Take the trained Austen lexicon at the settings the facts file reports for the wide+deep softmax operator, pool 1,228 words seen 20+ times. Each word is a vector of 64 complex phasors with counts. Form the 64 × 64 second moment C = E[w ⊗ w̄] over the pool, weighting each word by its count. Report the ratio of the sum of squared off-diagonal entries to the sum of squared diagonal entries. If the channels are the paper's coordinates, this ratio is near zero. If they are not, it is not. The control is the same lexicon with every word's channel vector randomly permuted per word, which destroys any alignment between channel index and content while preserving every marginal statistic. Report both ratios. This takes one pass over 1,228 words of 64 entries and a 64 × 64 accumulation; at the facts file's 2–5 ms per MB op it is well under a second.

Dear half, run only if the cheap half shows the channels are far from diagonal. Whiten with W = U D^{−1/2} from C. Re-express the lexicon as Wᵀw. Train the hinge operator with 8,360 parameters, 3 epochs, in the whitened basis, and report MRR on valid and test against chance 0.0063. The control is the hinge operator at the same settings in the raw basis, which the facts file gives as 14.7× valid / 14.6× test. Three outcomes are possible. If whitening helps, the paper's coordinates are better than Phical's and the channel design should change. If it hurts, the phases on the circle carry structure that a real whitening destroys, and Section 13's objection stands. If it does nothing, the operator is invariant to the choice and the question is moot for ranking.

The third moment, the paper's actual object, is a separate and harder experiment. Form the 64³ tensor of triple products over triples (X, Y, Z) drawn from the same Austen sentence. Run Algorithm 1 on its real part, with L and N small, and report the top eigenvalues and the cosine of each eigenvector against the channel axes. The control is the same tensor from triples drawn from three different sentences, which removes the shared condition and should, by the paper's theory, flatten the spectrum. Report the ratio of the largest to the k-th eigenvalue in each case. No prediction is made about which way it comes out.

Every number that would come from these runs is unmeasured today. None should be quoted until it has been.

## 13. Where we disagree, and where the analogy breaks

The analogy breaks in three places, and there is one place where Phient's position and the paper's are in tension rather than translation.

The first break is the field. The paper's tensors are real, symmetric, with positive eigenvalues. Phical's channels are complex phasors on a closed circle. A third moment of phasors is a complex tensor whose entries are sums of products e^{i(φ₁+φ₂+φ₃)}, and the sum of three phases is again a phase, a wraparound the paper's real theory has no place for. The convergence proof relies on positive real eigenvalues and on the ordering |λᵢ vᵢᵀθ| having a unique maximum. With complex λᵢ the ordering is by modulus and the argument may rotate. The paper does not address this and this page does not claim its theorem holds.

The second break is the target. The paper estimates parameters of a fixed generative model. There is no operator that moves a word. Phical's whole claim is that the word moves under a condition and that meaning is that movement. The paper's μᵢ is a point in the simplex; the paradigm sentence says Phical stores no point. The paper offers Phical a way to choose coordinates and no way to move in them. A reader who took the paper as a model of meaning would have a model of topics, and topics are conditions.

The third break is the count. In the paper the weight wᵢ enters the whitened tensor as 1/√wᵢ, so common things have small eigenvalues and rare things large ones. In Phical the well-counted relation is the stable one. In the paper's whitened basis the well-weighted component is the quiet one. Any experiment that ports the paper's algorithm to Phical must decide which convention to keep.

The tension is about k. The paper needs k, the number of hidden causes, given in advance, and its tolerance is ε ≤ C₁ λ_min / k. Phical fixes 32–64 channels and never asks how many there should be. If the right coordinates are the eigenvectors of a whitened third moment, then the number of channels is the rank of that tensor, a measured quantity. Sixty-four would then be a hypothesis, and the sphere, the helix and the hexagrams that share the number would be a hypothesis about the rank of English. Phient has not measured that rank. On Austen the sphere loses to the circle, 12.8× to 14.7×. That is one data point and it is not a rank.

Where we disagree is narrower. The paper's abstract calls its decomposition "a natural generalization of the singular value decomposition." A Phical reader would say the SVD generalises to tensors in several inequivalent ways and the paper chose the one a power method can find. The paper is candid that its result is algorithm-dependent. Its coordinates are the coordinates its method can reach, which is the same circularity Phical is open to when it calls its channels harmonics because the harmonic tie trains.

## 14. The person and the moment

The profile that accompanies these papers places this work in the first of its four moves. From 2010 to 2018 she was at UC Irvine, first as assistant professor, then tenured associate, working on large-scale machine learning and high-dimensional statistics, and on tensor decompositions for latent-variable models. The arXiv record for this paper spans exactly that period's middle: first posted 29 October 2012, final version 13 November 2014, published in JMLR the same year.

The profile calls her first decade "tensor methods" and describes the strand as "the recognition that the moments of a latent-variable model form a tensor whose decomposition recovers the model, with guarantees." That is a fair summary of this paper's introduction. The profile says this strand brought the NSF CAREER Award in 2013, a Sloan Fellowship in 2014 and an AFOSR Young Investigator Award in 2015. The dates bracket this paper.

The profile also records her training. Her doctorate was at Cornell in 2009, with the thesis Scalable Algorithms for Distributed Statistical Inference, advised by Lang Tong, and her postdoc was in Alan Willsky's Stochastic Systems Group at MIT. The paper's concern for guarantees and for what an estimator does under perturbation is the concern of that tradition. The paper cites Pearson's method of moments from 1894 and reads its own contribution as making that method computationally practical.

The profile's account of her later work names the Fourier Neural Operator and FourCastNet and gives her stated method as "All of the things that work with deep learning, let's take them, but make them a bit more principled." This paper is from before that. It takes a classical estimator and makes it tractable. The connection between the two halves of her career is not stated in the profile and this page does not invent one. What can be said from the two documents together is that the same author, a decade apart, twice chose to work in a transform domain: here the whitened basis of the second moment, there the Fourier basis of the function space. Both are choices of coordinates in which an operator becomes simple.

The profile is careful to say that nothing in it is her statement about Phient, and this page keeps that. She has not seen phinet. Her paper does not mention words as functions or relations on a circle. This is a reading of her work in our terms, and the responsibility for it is ours.

The moment, for Phient, is the one the profile names in its last section: a partnership is something to earn with a result she would recognise, measured the way she measures. This paper measures with a perturbation bound. The result Phient would need to show her, in these terms, is a third moment of words with a spectrum she could read.

## 15. What remains

What remains is one measurement and three questions.

The measurement is the second moment of channel vectors on the Austen lexicon, with the permuted control, as Section 12 describes. It is under a second of compute and it is decisive for whether the rest of the paper's machinery applies to Phical's channels as trained. Until it is run, every use of the word "orthogonal" about channels on this page is a hypothesis.

The first question is the complex case. The paper's theorem is for real symmetric tensors with positive eigenvalues. Phical's tensors are complex with phases on a closed circle. Whether a power iteration on a complex symmetric third moment converges, and to what, and whether the paper's 8ε / λ bound has any analogue, is not in the paper. Someone would have to prove it or find it proved, and this page does not know of a result to cite.

The second question is the count. The paper's whitened eigenvalue is 1/√wᵢ, which makes the common component the small one. Phical's count is a confidence that makes the common relation the stable one. If the paper's convention were adopted, the rare relation would be found first by the power method and would also be the one the byte represents least reliably. Which convention serves ranking is a question for the dear half of Section 12's experiment, and no answer is claimed.

The third question is k. The paper needs the number of components. Phical has 64 channels by design. If the third moment of Austen has a spectrum with a clear drop, the position of the drop is the paper's estimate of how many coordinates a word needs, and it is either near 64 or it is not. The facts file gives no number for this. Neither the sphere's MRR 1.000 on the synthetic per-degree task nor its 12.8× against the circle's 14.7× on Austen is a rank.

What does not remain, because it is outside the paper: nothing here bears on neural operators, on Fourier layers, or on weather. The facts file says Phient has done no weather work of any kind, and the paper under review has none either. The connection between this paper and phinet's FNO runs through the author. Later papers in this series will take up the operator side.

The one line to carry forward: three words under one condition carry the identity of the condition, and two do not. Phient trains on pairs. If the paper is right, there is information in the third word that the operator has never been shown.

## 16. Sources

| source | what it was used for | URL |
|---|---|---|
| arXiv abstract page, 1210.7559 | title, authors, abstract, version dates (v1 29 Oct 2012, v4 13 Nov 2014), journal reference | https://arxiv.org/abs/1210.7559 |
| arXiv PDF, 1210.7559v4 | section structure, definitions, Theorems 3.1–3.6, whitening, Algorithm 1, Lemma 5.1, Theorem 5.1 and its constants, Section 6 operation counts, absence of experiments, citations to Pearson, Hillar and Lim, De Lathauwer et al., Zhang and Golub, Kofidis and Regalia | https://arxiv.org/pdf/1210.7559v4 |
| JMLR paper page, volume 15 | volume, pages 2773–2832, year 2014, abstract as published | https://jmlr.org/papers/v15/anandkumar14b.html |
| PHIENT_FACTS.md (same folder) | every number about Phient on this page | local |
| 17_ANIMA_ANANDKUMAR_PROFILE.md | every statement about the author in Section 14 | local |

The arXiv id given in the brief, 1210.7559, was correct and was the one used. Where the fetched text of the paper gave a theorem number that could not be confirmed against the section structure, the number was omitted rather than guessed. Every formula in Section 3 was checked against the fetched PDF text. No statement about the paper's follow-up work, its citation count, its empirical use by others, or its reception is made, because none was fetched.

Nothing on this page reports a tensor decomposition run in gemphi. The measurements proposed in Section 12 are proposals. The numbers in Section 11 are the facts file's and were measured for other purposes. All limits named are real.
