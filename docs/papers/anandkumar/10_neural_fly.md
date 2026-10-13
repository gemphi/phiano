# Twelve minutes of wind: a shared basis and a coefficient that moves

*Phient interpreted papers, 10 of 16 · 2026-05-18 · on: "Neural-Fly enables rapid learning for agile flight in strong winds" (O'Connell, Shi, Shi, Azizzadenesheli, Anandkumar, Yue, Chung, 2022, Science Robotics 7(66):eabm6597; arXiv:2205.06908)*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Our abstract

This is the tenth of sixteen papers in which Phient reads one paper by Anima Anandkumar's group through its own paradigm. The paper is a robotics paper, not a language paper. It is about a quadrotor that has to fly a figure-eight through gusts of wind that it has never felt before, and about how much of what the wind does to the drone can be learned once and how much has to be learned again each time the wind changes. The answer the paper gives is precise. Almost all of it can be learned once. What is left over is four numbers per axis of force, and those four numbers can be tracked online by an adaptive law that runs in ten milliseconds on a Raspberry Pi 4.

We read it here for one reason. The paper separates a residual force into a basis that does not depend on the wind and a coefficient that does, and it trains the basis so that a classifier cannot tell which wind it came from. That is a clean statement of the thing Phical takes as its founding sentence: a word is what it relates to, under what conditions. The basis is the word. The coefficient is the condition. The adversarial loss is a rule for keeping the two apart. Phical has no such rule. It assumes the channel is the coordinate and the condition is the thing that moves it, and it has never checked whether its channels leak the condition. The paper gives us a way to ask.

Sections 2 to 5 say what the paper claims, its equations, and the numbers it reports, taken from the arXiv text and the Caltech and GitHub records. Sections 6 to 10 place its objects in the Phient family: the wind-invariant basis as a word's function, the coefficient vector as a coordinate on the relation channel, the control law as an operator X ⇒ Y under c, the discriminator as a test the quest oracle could run, and the composite adaptation loop as the online half of the helix. Section 11 lists what Phient has measured that bears on any of this. Section 12 names one experiment inside phinet with its control. Section 13 says where the analogy breaks, and it breaks in the place that matters most, which is that a drone has a ground truth for force and a word has none. Section 14 is about the person and the year. Section 15 is what is left.

One limit is stated up front. Phient has flown nothing, has no controller, and has no adversarial training of any kind. Every number about Phient below was measured for another purpose. Where the paper's method would need code that does not exist in gemphi, this document says so.

## 2. What the paper claims, in its own terms

The paper's problem is stated in its first paragraph. Precise flight in fast, changing wind is hard because the relation between wind and the aircraft's handling is not well understood. The aerodynamics of a small quadrotor near its own downwash, near a gate, or in a gust are not something a physics model captures cheaply, and a conventional controller either ignores them or fights them with gain.

The claim is that these effects, across many wind conditions, share a common structure, and that the part which changes with the wind lives in a low-dimensional space. The paper's phrase is that "aerodynamics in different wind conditions share a common representation and the wind-specific part lies in a low-dimensional space." Given that, the authors propose to learn the shared part once, offline, and to adapt only the low-dimensional part online.

The offline learner is called Domain Adversarially Invariant Meta-Learning, DAIML. It uses twelve minutes of flight data. That is the number the abstract leads with, and the paper repeats it: about two minutes in each of six steady wind conditions, logged at 50 Hz, which the paper counts as 36,000 data points. Training the representation on a desktop took five minutes.

The online learner is a composite adaptive controller. It updates the linear coefficients that combine the learned basis elements. It uses both the tracking error and the prediction error to do so, and it has a covariance-like matrix that is updated alongside the coefficients, in a form the paper compares to a Kalman filter.

The paper's headline result is measured in the Caltech Real Weather Wind Tunnel, an array of 1,296 individually controlled fans, at wind speeds up to 12.1 m/s. The evaluation winds go well beyond the winds seen in training, which topped out at 6.1 m/s. The paper reports that Neural-Fly achieves substantially smaller tracking error than state-of-the-art conventional controllers, and it gives the numbers, which are in section 4.

Two further claims are made. The first is a theorem: the closed loop is exponentially stable and the tracking error is bounded by terms in the representation error, the measurement noise, and the rate at which the true coefficient moves. The second is transfer. A representation trained on data from a different drone, an Intel Aero, and used on the custom quadrotor still beats every baseline that does not learn a representation. The paper calls that variant Neural-Fly-Transfer.

That is the whole claim. A residual force that factors, a basis learned to be blind to the condition, a coefficient tracked online, a bound, and a wind tunnel.

## 3. The mathematics, plainly

The vehicle is written as a rigid body. The paper's dynamics are M(q)q̈ + C(q, q̇)q̇ + g(q) = u + f(q, q̇, w). Here q is the position, u is the commanded force, and f is everything the model does not know: the residual aerodynamic force, which depends on the state and on the wind w.

The central line of the paper is a factorisation. The residual is approximated as f(q, q̇, w) ≈ φ(q, q̇) a(w). The basis φ is a deep network that reads the state and returns a small set of basis functions. The coefficient a is a vector that depends only on the wind. In the implementation, φ has input dimension 11: the three components of velocity, the four of the attitude quaternion, and the four motor PWM signals. Its layers are 11 → 50 → 60 → 50 → 4, so it returns four basis values. There is a separate four-vector of coefficients for each axis of force, so a_x, a_y, a_z ∈ ℝ⁴. The paper uses spectral normalisation on φ to control its Lipschitz constant.

DAIML trains φ against a discriminator h. Given a batch from wind condition k, the inner step solves a least-squares problem for the best coefficient, a*(φ) = argmin_a ‖y − φ(x) a‖², over an adaptation batch, and then scores the reconstruction on a disjoint training batch. The outer objective is max over h, min over φ and a, of the reconstruction loss minus α times the cross-entropy loss of h at predicting k from φ(x). The discriminator tries to read the wind off the basis. The basis is trained so that it cannot. The paper describes α as a hyperparameter that controls the degree of regularisation and gives no value in the main text. If the norm of a* exceeds a threshold γ, it is normalised.

Online, the controller is u = M(q)ẍ_r + C(q, q̇)q̇_r + g(q) − K s − φ(q, q̇) â, where s = q̇ − q̇_r is the composite tracking error. The estimate â is updated by two coupled equations. The first is ȧ̂ = −λ â − P φᵀ R⁻¹ (φ â − y) + P φᵀ s. Its three terms are a decay toward zero, a correction from the prediction error, and a correction from the tracking error. The second is Ṗ = −2λ P + Q − P φᵀ R⁻¹ φ P, which is the update of a gain matrix P, and this is the part the paper compares to a Kalman filter.

Theorem 1 states that the position error converges exponentially to a ball whose radius is sup over t of C₁‖d(t)‖ + C₂‖ε(t)‖ + C₃(λ‖a(t)‖ + ‖ȧ(t)‖), where d is the representation error, ε is the measurement noise, and the last term charges for how large and how fast the true coefficient is. Nothing in the bound requires the wind to have been seen before.

## 4. Its experiments and numbers as reported

The platform is a custom quadrotor of 2.53 kg, 85 cm across, with a thrust-to-weight ratio of 2.2 and a Raspberry Pi 4 running the controller at 50 Hz. Training data came from six steady wind conditions from 0 to 22.0 km/h (6.1 m/s), two minutes each. Evaluation was at 0, 15.1, 30.6 and 43.6 km/h (0, 4.2, 8.5 and 12.1 m/s), plus a sinusoidal wind of 30.6 + 8.6 sin(t) km/h. The GitHub data release lists eight steady speeds from 0 to 12.1 m/s plus the sinusoid, mapped to fan duty cycles from 0 to 100%.

The task is a figure-eight, 2.5 m wide and 1.5 m tall, with a lap time of 6.28 s. The baselines are a nonlinear tracking controller with PID, INDI (incremental nonlinear dynamics inversion), and L1 adaptive control. Two ablations of the method are also run. Neural-Fly-Constant replaces the learned basis with a constant. Neural-Fly-Transfer uses a basis trained on the Intel Aero drone's data.

The paper's Table 1 gives position tracking error in centimetres, RMS and mean, as follows.

| wind (m/s) | nonlinear | INDI | L1 | NF-Constant | NF-Transfer | Neural-Fly |
|---|---|---|---|---|---|---|
| 0 | 11.9 / 10.8 | 7.3 / 6.3 | 4.6 / 4.2 | 5.4 / 5.0 | 3.7 / 3.4 | 3.2 / 2.9 |
| 4.2 | 10.7 / 9.9 | 6.4 / 5.9 | 5.8 / 5.2 | 6.1 / 5.7 | 4.8 / 4.4 | 4.0 / 3.7 |
| 8.5 | 16.3 / 14.7 | 8.5 / 8.2 | 12.1 / 11.1 | 7.5 / 6.9 | 6.2 / 5.9 | 5.8 / 5.3 |
| 12.1 | 23.9 / 21.6 | 10.7 / 10.1 | 22.7 / 21.3 | 12.7 / 11.2 | 10.2 / 9.4 | 9.4 / 8.7 |

The paper summarises these as a 66% improvement over the nonlinear controller, 42% over L1 and 35% over INDI. Two things in the table should be read slowly. L1 is nearly as good as Neural-Fly in still air and falls apart at 12.1 m/s, where its error is more than double Neural-Fly's. And Neural-Fly-Transfer, whose basis never saw this drone, is second best at every speed.

Timing is reported for the online phase. The whole adaptation step takes 10 ms on the Raspberry Pi 4, of which the feedforward pass of φ is 3.5 ms and the coefficient update is 3.0 ms. A gate-passing experiment with 110 cm gates, slightly wider than the 85 cm drone, is shown with long-exposure and high-speed photographs; the paper reports no success rate for it and says only that the drone tracked its trajectories and flew safely through. An outdoor test with GPS state estimation, in wind up to 17 km/h (4.9 m/s), gave a mean tracking error of 7.5 cm.

The paper's own list of what still limits it is code execution time, communication delay and attitude tracking delay. It names model predictive control as future work.

## 5. The one idea, restated

Strip the drone away and one sentence remains. A quantity that depends on a state and a condition can be written as a function of the state times a coefficient of the condition, and the function can be trained so that the condition cannot be read back out of it.

The first half of that sentence is a low-rank claim. Across conditions, the residual force is not arbitrary. It moves in a four-dimensional subspace whose basis is fixed by the state. The paper does not prove this; it chooses h = 4 and shows the controller works, which is the engineering form of the argument.

The second half is the part the paper adds to the literature. Meta-learning a shared basis is older than this paper. What DAIML adds is the adversary. Without it, nothing stops the network from putting wind information into φ, so that φ for the 8.5 m/s condition and φ for the 12.1 m/s condition are different functions of the same state, and the coefficient a carries nothing. With it, the discriminator punishes any such leak, and the wind is forced into a. The paper says the alternative of random Fourier features was tried and rejected because many features would be redundant or irrelevant. The learned basis is small because the adversary made it honest.

Then there is the third thing, which is not in the sentence but is why the paper works in a wind tunnel. Once the basis is fixed, adapting a is a linear problem. Linear problems have adaptive laws with proofs. The composite adaptation of section 3 is a standard shape from adaptive control, with the twist that P is updated like a Kalman covariance so that the coefficient is trusted in proportion to how well the basis has been predicting. That is what makes the ten milliseconds possible. A deep network is run forward once per step. Nothing is trained online.

In Phient's words the idea reads as follows. The word is φ. It is what the state relates to, and it is meant to be the same word in every wind. The condition is a. It is four numbers per axis, and it is where all of the wind lives. The relation between them is a product. And the training rule is a rule for keeping the condition out of the word. Phical says the first three of these as a paradigm. It has never said the fourth. The rest of this document is about what happens when it does.

## 6. The wind-invariant basis as a word's function

In Phical a word is a function on a small set of coordinates. It has 32–64 relation channels, each a complex phasor on a closed circle with a count. Apply the word to a context and the phasors say where the word sits with respect to each relation.

The basis φ in Neural-Fly is the same kind of object with the roles named differently. Its input is an eleven-dimensional state. Its output is four numbers. Those four numbers are not the force. They are the coordinates in which the force is easy to write. Change the wind and the coordinates stay put; only the weights on them change. That is exactly the discipline of the relation channel. A word's channels are supposed to be the same channels whatever the sentence around it. What the sentence does is move the phasors.

The correspondence can be made a little tighter. In Neural-Fly the state q, q̇ is the thing that is looked up, and φ is the lookup. In phiano the word is the thing that is looked up, and its Facet is the lookup. The state changes continuously along a trajectory. The word changes discretely along a sentence. But in both cases the thing on the right of the lookup is a short vector of basis values, and in both cases that vector is meant to be reusable across conditions the trainer never saw. Neural-Fly's test of reusability is the 12.1 m/s row of Table 1, where the wind is twice the strongest training wind and the learned basis still wins. Phical's test of reusability is a held-out prompt.

Where the correspondence is loose is the size. Neural-Fly's basis has four elements per axis. Phical's word has 64 channels. Neural-Fly can afford four because it has one target, a three-dimensional force, and a linear coefficient to absorb the rest. Phical has no such target; the word's channels have to carry every relation the word will ever be asked about, so it has more of them. Neither number is derived. The paper picks h = 4. The facts file records 64 = 8² channels chosen so that they can be read as spherical harmonics Y_l^m with l < 8 when the sphere tie is used. Both are choices that the experiments then justify or do not.

One more point. Neural-Fly's φ is a deep network with spectral normalisation. Its output is not constrained to a circle. Phical's channel is one byte, 256 quanta of 2π, on a closed circle. The paper's basis lives in ℝ⁴. Phical's lives on a torus. That difference is where section 13 will start.

## 7. The coefficient vector as a coordinate on the relation channel

The vector a in Neural-Fly is the condition. Everything the wind does that the basis cannot see ends up there. It has four entries per axis, and its motion over time is what the online loop tracks.

Phical's condition c is less explicit. The paradigm says a word is what it relates to under conditions, and the operator X ⇒ Y is trained under c, but the condition is not a vector the code holds. It is whichever context the trainer supplies, and its effect is spread across the phasor rotations of every channel. There is no place in phinet where one can read off "this is the condition and this is the word" the way one reads off a and φ in the control law.

That is the sharpest thing this paper says to Phient. Neural-Fly shows that when the condition is pulled out as its own small vector, three things follow. The condition can be tracked with a linear law. The law has a proof. And the basis can be tested for leakage, because there is a discriminator that tries to read the condition off it. None of those three is available while the condition is implicit.

What would a coordinate for the condition look like in Phical? The nearest thing that exists is the quest's kind. The phiecs oracle has five relation kinds, is-a, part-of, near, used-for and unlike, and the measured result is that reading the wrong channel for a kind gives MRR 0.015 against a within-kind 0.747. That gap is the same shape as the gap between φ's four basis values and a's four coefficients. The kind is a condition that selects which channels to read. It is the closest thing in gemphi to the wind.

But the analogy has a hole, and it is the same hole as in section 6. In Neural-Fly the condition is continuous and the coefficient moves. The sinusoidal wind at 8.5 + 2.4 sin(t) m/s is a condition that never sits still, and the composite law follows it. In the quest the kind is one of five labels and does not move. Phical has no measured case of a condition that drifts and a coefficient that follows it. The persona work is the nearest: three writers, a paragraph, 71% attribution against 33% chance. But the persona is inferred from the text, not tracked from a stream of errors, and there is no update law.

So the reading of a as a channel coordinate is a reading, not a mapping. What the paper gives Phical is the demand that the condition be a thing with a size, a decay rate λ, and a gain P. Phical has none of those. Section 12 proposes the smallest version of them that could be measured.

## 8. The control law as X ⇒ Y under c

The Phical operator takes a word's function X and moves it toward another word's function Y under a condition c. In phinet the operator is an Fno: unitary FFT, learned complex weights on the retained modes, inverse FFT, trained on pairs. The Austen operator at its best reaches 30.7× chance on held-out ranking at 8 epochs.

Neural-Fly's controller is also an operator that moves one function toward another under a condition. The function X is the current state along the trajectory. The function Y is the reference trajectory ẍ_r, q̇_r. The condition is the wind, carried in â. The law u = M ẍ_r + C q̇_r + g − K s − φ â is the operator. Its output is a force. The −K s term pulls X toward Y. The −φ â term cancels what the condition would otherwise do.

The reading is exact up to one substitution. In phinet the operator is learned and the condition is implicit. In Neural-Fly the operator is written down from mechanics and only the residual is learned, and the condition is explicit in the last term. Neural-Fly learns the smallest thing it can get away with learning. Phinet learns the whole map.

That difference is not an accident of domain. It is the paper's method and it is Anandkumar's stated method in the profile: take what works and make it principled. In Neural-Fly the principled part is the rigid-body model, which is exact, and the adaptive law, which has a theorem. The learned part is a four-output network trained for five minutes. Phinet's ratio is reversed. Its principled part is the FFT and the phasor circle. Its learned part is everything else.

There is a Phient object that sits closer to Neural-Fly's shape than the Fno does. The Austen hinge operator has 8,360 parameters and reaches 14.7× valid / 14.6× test. The sphere tie has 1,192 parameters, one complex weight per spherical-harmonic degree, and reaches 12.8× / 13.1× on the same task in 89 s against the circle's 142 s. That is a smaller learned part with a fixed geometric part doing more of the work. On the synthetic degree-filter task the sphere tie reaches MRR 1.000 where the circle reaches 0.662 and identity 0.646. The sphere tie is the phinet operator that most resembles φ a: a few learned weights on a basis chosen for its geometry.

What phinet does not have is the s term. There is no tracking error in a language operator, because there is no reference trajectory. The nearest thing is the ranking loss during training. But that is offline. Neural-Fly's s is computed every 20 ms in flight and fed back into â. Section 10 is about whether the helix or the quest could hold such a loop.

## 9. Where the discriminator lands

The part of DAIML that has no Phient counterpart is the discriminator h. It is a classifier that reads φ(x) and tries to say which of the K wind conditions the sample came from. The basis is trained to make h fail. The paper says the alternative without the adversary lets the wind leak into the basis, which is why the adversary is there.

Phient has an object that could play h, and it is the quest oracle in phiecs. The oracle is synthetic: 256 words in families of 8, five kinds, 1,280 rows, 293 target words, chance MRR 0.021. It knows the kind of every row. A discriminator over the quest would be a reader that, given a word's channels, tries to name the kind that produced the training row. If it succeeds, the kind has leaked into the channels. If it fails, the channels are kind-invariant in the sense the paper means.

The measured numbers already say something here, though they were not measured for this. The within-kind MRR is 0.747 and the wrong-channel MRR is 0.015. That is a strong dependence of the reading on which channel is read under which kind. It says the kind is carried by the choice of channel, which is what one wants. It does not say whether the kind also leaks into the phasor values themselves. No one has trained a classifier to find out. So the adversarial question is open and the tool to close it exists in outline.

The other place the discriminator lands is phinet's Fno itself. The Fno keeps low modes and multiplies them by learned complex weights. Neural-Fly rejected random Fourier features because many would be redundant or irrelevant. The point of DAIML is that a learned basis of four is worth more than a random basis of many. Phinet's basis is not random, it is the Fourier basis on the channel index, and the operator learns which modes to keep. The question DAIML asks of it is whether the retained modes are the same modes under every condition. If a persona operator and an Austen operator keep different modes, that is the leak the adversary would catch.

The tensor also lands here, in a small way. The paper's coefficient is four numbers per axis, three axes, so twelve numbers that are all the wind. The helix stores a codon per channel, 64 codons as base-4 ACGT, with a sphere degree and order and a PhiTon wavelength per codon. A condition in the helix, if it were made explicit, would be a small vector that rotates codons, and it would be the same size order as Neural-Fly's a. That is a design remark and not a measurement. Nothing in the helix currently holds a condition.

## 10. The composite loop as the online half of the helix

Neural-Fly has two clocks. The slow one runs once, for five minutes on a desktop, and produces φ. The fast one runs every 20 ms on the drone and produces â. The fast clock is what makes the paper a control paper rather than a learning paper. The controller is never retrained. It only re-weights.

Phient has the slow clock. Phiano's Trainer, the Austen runs at 3 and 8 epochs, the quest trainer that reached 0.594 MRR with attraction-only and dithered rounding, these are all offline. What Phient does not have is any fast clock at all. Generation runs the operator forward. Nothing in generation feeds an error back into the word.

The helix is the natural place to ask for one. It is the lexicon as a DNA-coded tree, 18,496 words in 1.9 s and 75 MB, byte-exact on the round trip. It is a store. Neural-Fly's fast loop is not a store; it is a state â and a gain P that change with every sample. If the helix held, beside each codon, a coefficient and a gain, then a dialogue turn could update them from a prediction error the way the drone updates them from a force error. That is the sketch. The measured dialogue result is 18 replies judged as their persona at 44% by the sector reader and 72% by the Kneser-Ney panel against 33% chance, and those were produced without any per-turn adaptation.

The byte deadband is the obstacle. A channel is one byte, and a nudge of 0.06 sin d falls under half a quantum once sin d < 0.2, about 11°. Nearest rounding froze every pair 8 quanta short. Neural-Fly has no such floor. Its â is a float and its P can make a step as small as it likes. If Phical's fast loop wrote back into bytes, the deadband would stop it before the coefficient converged, which is exactly what it did to the quest trainer before dithering. Dither fixed the quest but cost corpus attribution, 71% down to 65%, and ships off by default. A fast loop in the helix would have to keep â outside the byte and round only at the end.

The Kalman-like P also has a reading. Every Phical channel carries a count. The count is how many times the relation has been seen. Neural-Fly's P is how much the coefficient should be trusted, and it grows with Q and shrinks with the information φᵀ R⁻¹ φ. A count is a crude P: it only ever grows. The paper's second equation says P should also decay at rate 2λ and shrink when the basis has been informative. That is a stronger idea of what a count is for, and Phical could adopt it without touching the phasor.

## 11. What Phient has measured that bears on it

Everything in this section is from the facts file, and none of it was measured with this paper in mind.

Phinet itself is verified against a PyTorch reference: 62 of 62 parity tests pass. That matters because the paper's fast loop needs a forward pass that is correct at 50 Hz, and phinet's engine is measured as allocation and bandwidth bound at these sizes, with a 1 MB op at 2–5 ms. Neural-Fly's forward pass of φ takes 3.5 ms on a Raspberry Pi 4. The two numbers are not comparable, since one is a Rust tensor op and the other is a four-layer network on a different machine, but they are the same order and both are inside a 20 ms budget.

On operators: the Austen task has a pool of 1,228 words seen 20 or more times, 204,592 training examples, and chance MRR of 0.0063. The identity baseline is 7.9× chance. The hinge operator with 8,360 parameters reaches 14.7× valid / 14.6× test. Softmax at β = 8 reaches 18.0× / 18.8×. The wide and deep softmax with 50,016 parameters reaches 27.6× at 3 epochs and 30.7× / 30.3× at 8 epochs with hit@10 of 0.348. These are the numbers for the learned operator. The sphere tie, 1,192 parameters, reaches 12.8× / 13.1× and hit@10 0.182 in 89 s. The sphere at softmax and wide-and-deep is unrun.

On the small-basis question: the synthetic degree-filter task with 200 items over 40 epochs gives circle MRR 0.662, identity 0.646 and sphere 1.000. That is the one Phient result where a small, geometrically chosen basis beats a larger free one outright, and it is the closest thing to the paper's claim that four honest basis functions beat many random ones.

On conditions: the quest gives 0.082 MRR for the shipped trainer and 0.594 for attraction-only with dithered rounding, hit@1 34.8%, hit@5 97.4%. Within-kind is 0.747, wrong channel 0.015, offset reading 0.005. On the held-out 20%, one hop is 0.011; two hops is 0.095 for is-a, 0.093 for used-for, and around chance for part-of, near and unlike. So the kind is carried by the channel, and the held-out generalisation is weak.

On attribution: three writers, held-out four-sentence paragraphs, 71% overall (74/65/86) against 33% chance. Dialogue: 44% and 72% against 33%.

On decoding: 40 held-out prompts by 30 tokens, trigram sampler Kneser-Ney perplexity 13.3, operator nearest-k 66.9 (was 97.2), union 66.8. Shared 4-grams across prompts: sampler 25.7%, operator 8.7%.

Nothing has been measured about condition drift, about online adaptation, about leakage of a condition into channels, or about any control task. Phient has done no weather work and has flown nothing.

## 12. The experiment inside phinet

The experiment is to test the paper's central move, the adversary, on the one Phient task where the condition is known: the quest. The quest oracle labels every row with a kind. The kind is the wind.

Train the quest as shipped, attraction-only with dithered rounding, to its measured 0.594 MRR. Freeze the word channels. Then train a small classifier that reads a word's 64 phasors and predicts which kind was used to write the training rows that touched it. If the classifier reaches chance, the channels are kind-invariant. If it is well above chance, the kind has leaked into the phasor values and not only into the choice of channel. That is the diagnostic half, and it costs nothing but the classifier.

The second half is DAIML proper. Retrain the quest with the classifier as an adversary at weight α, so that the word channels are pushed to make the classifier fail while the kind-specific reading is pushed into a per-kind coefficient of size h. Choose h = 4 to match the paper. The reading under a kind becomes channels times coefficient rather than channels selected by kind. Measure MRR, hit@1 and hit@5 on the same 1,280 rows, and measure the held-out 20% at one and two hops.

The control is the same retraining with α = 0. That is the meta-learning without the adversary, which the paper says leaks. If α = 0 and α > 0 give the same held-out numbers, the adversary does nothing on this task and the paper's move does not transfer. If α > 0 raises the held-out two-hop numbers from their measured 0.095 and 0.093 for is-a and used-for, and lifts part-of, near and unlike off chance, the adversary is doing what it does in the wind tunnel: forcing the condition out of the word and into a coordinate that generalises.

Three things are held fixed: the dither setting, the oracle of 256 words and 1,280 rows, and the shipped reader, so that the number is comparable to 0.594.

There is one thing to report that the paper reports and Phient never has. Neural-Fly gives its numbers at winds it never trained on. The quest analogue is a sixth kind held out entirely from training, with the coefficient for it fitted from a handful of rows at test time by least squares, which is the paper's inner step. If that coefficient, with frozen channels, gives a within-kind MRR anywhere near the 0.747 measured for trained kinds, the shared basis is real. If it stays at the held-out floor, it is not.

None of this exists. The classifier, the adversarial loss, the per-kind coefficient and the held-out kind are all unwritten. The experiment is stated so that it can be run and so that its control is named before it is.

## 13. Where the analogy breaks

It breaks first at the target. Neural-Fly has a ground truth. The residual force y is measured from the vehicle's acceleration and the commanded thrust, and every training example is a real pair (state, force). A word has no such pair. The quest oracle supplies one synthetically, and Austen supplies a ranking task, but there is no physical quantity a word's channels are supposed to reconstruct. The paper's least-squares inner step needs a y. Phical's y is whatever the trainer decides to call the next word. That is the deepest break and no amount of reading fixes it.

It breaks second at the geometry. The paper's basis lives in ℝ⁴ and its coefficient in ℝ⁴, and their product is a real force. Phical's channel is a phasor on a closed circle, one byte, 256 quanta of 2π. A linear coefficient on a circle is not a linear map. It is a rotation and a scaling of a complex number, and once the byte is applied, it is a rotation only. The composite adaptation law is built on the linearity of φ a in a. On the circle that linearity holds in the complex plane and is lost at the byte. The deadband result is the measured form of that loss.

It breaks third at the loop. Neural-Fly's guarantee is a theorem about a closed loop with a reference trajectory and a tracking error s. Language has no reference trajectory. Generation is open-loop. The 40 held-out prompts by 30 tokens were decoded without any error signal. One could invent an s from the reader's ranking, but that would be an offline loss dressed as a control error, and the theorem would not carry.

It breaks fourth at the meaning of invariance. The paper wants φ blind to the wind because the wind is a nuisance. In Phical the condition is not a nuisance. The paradigm's last clause says meaning is a side effect of pairs of words moving under a condition. If the channels were made fully blind to the condition, there would be nothing left to move. The adversary in a Phient version would have to be partial, which the paper's α already allows, but the paper never faces the case where the condition is the point.

There is one place where we would say the paper is stronger than it lets on, and one where it is weaker. Stronger: the transfer result. A basis learned on an Intel Aero beating three tuned controllers on a different airframe is the claim that the basis is about air and not about the drone. That is the paper's real result and it is in a column of a table. Weaker: the 66%, 42% and 35% improvements are from one figure-eight in one tunnel. The outdoor number is a single mean of 7.5 cm at 4.9 m/s with no baseline reported beside it. We repeat it because a reader who takes only the headline will take too much.

## 14. The person and the moment

Everything in this section is from the profile, and nothing about her intentions is claimed.

The paper is dated May 2022. That is the same spring as FourCastNet, February 2022, which put an adaptive Fourier neural operator on the whole planet at 0.25° and produced a week's forecast in under two seconds. It is the year she became a Fellow of the ACM, the year of the Gordon Bell Special Prize for HPC-based COVID-19 research, and the year of a NeurIPS Outstanding Paper. She was then, as now, both Bren Professor at Caltech and senior director of machine-learning research at NVIDIA, with the Caltech AI4Science initiative she co-founded in its fourth year.

Neural-Fly is a Caltech paper. Its senior author is Soon-Jo Chung, and its subject is the aerospace side of the campus, the wind tunnel at CAST with its 1,296 fans. The profile records drone landing in turbulence among the places her operator machinery reached, and this paper is where that strand meets her name in a journal. Her role here is not the Fourier operator. Her co-authorship, with Kamyar Azizzadenesheli, sits on the learning side: the meta-learning, the adversarial formulation, the spectral normalisation.

The profile gives her method in her words: take all the things that work with deep learning, and make them a bit more principled. Neural-Fly is a compact instance of that. The deep network is four layers and four outputs. Everything around it is mechanics and adaptive control with a proof. The paper does not try to learn flight. It tries to learn the one thing mechanics cannot write down, and it makes that thing as small as it can be.

It is also of a piece with the first decade the profile describes, the tensor-methods years at UC Irvine, in one specific way. The claim that the wind-dependent part lies in a low-dimensional space is a low-rank claim. Her early work was the recognition that moments of a latent-variable model form a tensor with low-rank structure that can be recovered with guarantees. Neural-Fly's φ a is a rank-four factorisation of a residual over conditions, with an adversary to make the factors honest and an adaptive law to track the small factor. The instinct is the same. Find the small structure, then prove something about it.

The profile also records, from March 2026, her appointment to the UN Scientific Advisory Board. That is not relevant to a 2022 drone paper and we do not pretend it is. What is relevant is that in 2022 the group was already publishing a result whose largest number is twelve minutes of data, and whose method scales down rather than up.

## 15. What remains

The first thing that remains is the diagnostic in section 12. It is cheap. A classifier over 256 words' channels, trained to name a kind, and a single number against chance. If the channels leak, Phical has a specific defect the paper names. If they do not, Phical has a property it never checked and can now state.

The second is the adversary and the coefficient. Both are new code. The quest trainer would need a per-kind vector of size h and a loss term with weight α, and the reader would need to multiply rather than select. The control at α = 0 is part of the same code and costs nothing extra.

The third is a condition that moves. The paper's sinusoidal wind is its hardest case and its most convincing one. Phient has no task where the condition drifts within a run. Dialogue is the candidate, since the persona could be allowed to shift across turns, but the measured dialogue result is at 18 replies and no drift was imposed. A drifting-condition task would have to be built before it could be measured.

The fourth is the count. Section 10 read Neural-Fly's P as a count that can also decay and shrink. That is a change to what a Phical channel stores, from an integer to a scalar gain, and it touches the helix format, which is byte-exact today. It should not be done until the first two items give a reason.

The fifth is the sphere at softmax and wide-and-deep, which the facts file records as unrun. It is not this paper's experiment, but the sphere tie is the phinet object that most resembles a small learned coefficient on a fixed basis, and its ceiling is unknown.

What does not remain, because it cannot be done, is the paper's central test. Neural-Fly is judged by a drone flying through a gate in a wind it never trained on. The success criterion is physical and external. Phical will never have one of those. Its nearest equivalent is a held-out kind read with a coefficient fitted from a few rows, and that is a synthetic oracle judging a synthetic word. The honest statement is that the paper's method could be tried in phinet, its control could be run, and its numbers could be reported, and that even then the thing Neural-Fly proved would not have been proved here. Twelve minutes of wind is a fact about air. The quest is a fact about a table of 1,280 rows.

## 16. Sources

The paper, abstract and full text, as fetched on the date above.

arXiv abstract page: https://arxiv.org/abs/2205.06908

arXiv full text (PDF), the source of the equations, the architecture 11 → 50 → 60 → 50 → 4, Table 1, the timing figures and the limitations: https://arxiv.org/pdf/2205.06908

CaltechAUTHORS record, the source of the journal citation (Science Robotics 7(66), eabm6597, 4 May 2022, DOI 10.1126/scirobotics.abm6597) and the author order: https://authors.library.caltech.edu/114603/

The authors' data and code release, the source of the two drone names, the eight steady wind speeds and the sinusoid in the released data, and the licence: https://github.com/aerorobotics/neural-fly

The Science Robotics page at https://www.science.org/doi/10.1126/scirobotics.abm6597 was not reachable from this session and was not used; the DOI above is taken from the CaltechAUTHORS record and the GitHub citation block.

Phient numbers: PHIENT_FACTS.md in this folder. The person: 17_ANIMA_ANANDKUMAR_PROFILE.md, one folder up.
