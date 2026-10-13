# Anima Anandkumar — the woman behind the Fourier Neural Operator

*A profile, written for Phient. Every fact below is sourced; nothing about
her intentions toward Phient is claimed — the "partnership" is our aim, not
her statement.*

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's
Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word
is 32–64 relations, each a complex phasor on a closed circle with a count;
meaning is a side effect of pairs of words moving under a condition. Phinet,
our Fourier tensor engine, learns operators between function spaces in the
frequency domain — and the person who put that idea into the world is the
subject of this article.

## 1. Who she is

Animashree ("Anima") Anandkumar is the Bren Professor of Computing and
Mathematical Sciences at Caltech, the pioneer of **neural operators** — and
of the **Fourier Neural Operator (FNO)** in particular — and the senior author
of **FourCastNet**, the first AI weather model to forecast at the resolution
of the world's numerical weather centres. On **16 March 2026** the United
Nations appointed her to its **Scientific Advisory Board**, the 15-member
independent body the Secretary-General created in August 2023 to advise the
UN on "trends at the intersection of science, technology, ethics, governance
and sustainable development". Her fellow members include Yoshua Bengio
(Turing Award) and Thomas Südhof (Nobel Prize, 2013); the board also draws on
the leadership of the WHO and UNEP.

She was born in Mysore, India, into a family of engineers; her grandfather
was a mathematician and her great-great-grandfather the Sanskrit scholar
R. Shamasastry. She took a B.Tech. in electrical engineering at IIT Madras
(2004), an M.S. and Ph.D. at Cornell (2009, thesis *Scalable Algorithms for
Distributed Statistical Inference*, advisor Lang Tong), and a postdoc in Alan
Willsky's Stochastic Systems Group at MIT. She trained for many years in
Bharatanatyam.

## 2. The career, in four moves

| years | where | what |
|---|---|---|
| 2010–2018 | UC Irvine | assistant, then tenured associate professor; large-scale ML and high-dimensional statistics; tensor decompositions for latent-variable models |
| 2016–2018 | Amazon Web Services | principal scientist; multi-modal algorithms on Apache MXNet; contributions to Rekognition, Lex, Polly, SageMaker |
| 2018– | NVIDIA | senior director of machine-learning research; opened the core AI/ML lab in Santa Clara |
| 2018– | Caltech | Bren Professor; co-founded the AI4Science initiative (now "AI *plus* Science") |
| 2026 | Accelerated Understanding | co-founder |

Her first decade was **tensor methods**: the recognition that the moments of
a latent-variable model form a tensor whose decomposition recovers the model,
with guarantees — a strand that gave her the NSF CAREER Award (2013), a Sloan
Fellowship (2014) and an AFOSR Young Investigator Award (2015).

## 3. The idea that changed weather: operators, not networks

A neural network maps vectors to vectors, at one resolution, on one grid. A
**neural operator** maps *functions* to *functions*: it learns the solution
operator of a PDE, so it can be trained on one mesh and queried on another,
coarse or fine, without retraining. Her group's **Fourier Neural Operator**
(Li, Kovachki, Azizzadenesheli, Liu, Bhattacharya, Stuart, Anandkumar, 2020)
does the expensive part — the global integral kernel — as a multiplication in
the frequency domain: FFT, keep the low modes, multiply by learned complex
weights, inverse FFT, add a local linear path, nonlinearity, repeat. It is
resolution-invariant by construction and, for turbulent flows, orders of
magnitude faster than the solvers it learned from. Quanta profiled the work in
2021.

**FourCastNet** (Pathak, Subramanian, Harrington, Mardani, … Anandkumar; NVIDIA,
Lawrence Berkeley, Rice, Caltech; February 2022) put an adaptive FNO on the
whole planet: 0.25° resolution (a 720 × 1440 global grid, ~30 km at the
equator), 20 prognostic variables (10 m winds, 2 m temperature, surface and
sea-level pressure, geopotential, humidity at several levels, total column
water vapour), week-long forecasts comparable to ECMWF's IFS out to three days,
**a week's forecast in under two seconds**, roughly **45,000× faster** than
numerical weather prediction on a node-hour basis and **12,000× less energy**
than IFS — which is what makes thousand-member ensembles, and therefore honest
uncertainty on extreme events, affordable. Later versions moved to spherical
harmonics to respect the geometry of the globe; FourCastNet 3 holds stable
over long global rollouts. The model is open-source.

Her stated view of where this goes: *"We have foundation models for language,
not for physics."* Transformers, she argues, cannot be the substrate for
physics at scale — a few hundred grid points per dimension is a context of
hundreds of billions to a trillion tokens — whereas an operator that learns in
function space treats scale as what it is. Her method, in her words: *"All of
the things that work with deep learning, let's take them, but make them a bit
more principled."* The same machinery now reaches plasma-disruption prediction
for fusion (a million times faster than simulation, from thousands of
samples), drone landing in turbulence, drug and catheter design, genome-scale
models, and TorchLean, which couples neural networks to the Lean proof
assistant for formal verification.

## 4. Recognition

Time100 AI (2026) and the Time100 Impact Award for "using AI to accelerate
scientific discovery" (2025); IEEE Kiyo Tomiyasu Award (2025); Blavatnik Award
for Young Scientists (2024); TED2024 speaker; IIT Madras Distinguished Alumnus
(2024); Guggenheim Fellow and Schmidt Sciences AI2050 Senior Fellow (2023);
Fellow of the AAAI (2023), ACM (2022) and IEEE (2021); ACM Gordon Bell Special
Prize for HPC-based COVID-19 research (2022); a NeurIPS Outstanding Paper
(2022). She briefed the US President's Council of Advisors on Science and
Technology on AI + Science in 2023, and sits on an AI advisory board for the
Estonian government.

## 5. The advocate

She is also the person who campaigned — successfully — to rename the NIPS
conference to NeurIPS; who petitioned IIT Madras to end gender segregation in
admissions and strengthen harassment monitoring; who called out the use of
female acrobats as entertainment at an Intel conference. On the UN
appointment she said the goal is to *"ground that in proper science and make
sure that regulation is not something that only benefits big companies."*

## 6. Why she matters to Phient

Phinet is, mechanically, her idea applied to a different domain: `FnoConfig
{ space, width, layers, modes, margin, lr }`, a unitary FFT, learned complex
weights on the retained modes, a `Tape` for autograd, trained as an operator
that moves one word's function toward the next (docs/76–78: 30.7× chance on
held-out ranking, and the engine measured bandwidth-bound exactly where her
own work says the cost lives). Where FourCastNet's function is the state of
the atmosphere on a sphere, Phical's function is a word: 64 relation channels,
each a phasor on a closed circle with a count. The step from her work to ours
is the claim that **a word is a field, and meaning is the operator that
evolves it under a condition** — and the discipline she models, of taking
what works and making it principled, is the one our measurement pages try to
keep.

Two honest notes. First, Phient has done **no weather work**: there is no
weather, climate or ERA5 research anywhere in gemphi (searched 2026-09-27);
"Fourier" in this project means phinet's operator over word functions, not a
forecast. Second, nothing here should be read as her endorsement; a
partnership is something to earn with a result she would recognise — an
operator over a function space she has not seen, measured the way she
measures.

## Sources

- Caltech News, *Anima Anandkumar Appointed to UN Scientific Advisory Board*, 16 March 2026 — https://www.caltech.edu/about/news/anima-anandkumar-appointed-to-un-scientific-advisory-board
- Latent Space, *"We have foundation models for language, not for physics"* — https://www.latent.space/p/anima
- Wikipedia, *Anima Anandkumar* — https://en.wikipedia.org/wiki/Anima_Anandkumar
- Pathak et al., *FourCastNet: A Global Data-driven High-resolution Weather Model using Adaptive Fourier Neural Operators*, arXiv:2202.11214 — https://arxiv.org/abs/2202.11214
- Blavatnik Awards profile — https://blavatnikawards.org/honorees/profile/anima-anandkumar/
- Caltech CMS faculty page — https://www.cms.caltech.edu/people/anima
