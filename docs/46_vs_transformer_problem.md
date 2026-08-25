# Part 1: The Transformer Problem - Why Attention Isn't Enough

## The Transformer's Core Limitation

Transformers (Vaswani et al., 2017) revolutionized NLP with self-attention:

$$\text{Attention}(Q, K, V) = \text{softmax}\left(\frac{QK^T}{\sqrt{d_k}}\right)V$$

But self-attention has structural problems that scaling cannot fix:

| Problem | Root Cause | Impact |
|---------|-----------|--------|
| **O(n²) attention** | Every token attends to every other | Quadratic memory, context length limits |
| **Frozen inference** | Weights are fixed during generation | No learning at inference time |
| **Positional encoding hack** | RoPE/sinusoidal bolted on externally | Position is not native to the representation |
| **Black-box attention** | Can't see WHY a token was selected | No interpretability |
| **Catastrophic forgetting** | Fine-tuning overwrites old knowledge | Can't learn continuously |
| **Discrete sampling** | Token = argmax/temperature over logits | No phase dynamics, no interference |

## Phiano's Alternative: Phase-Coupled Oscillators

Phiano replaces attention with **Kuramoto phase synchronization** on a C^32 torus:

$$\frac{d\theta_i}{dt} = \omega_i + \frac{K}{N} \sum_{j} \sin(\theta_j - \theta_i + \beta_{ij})$$

- **O(n) per token**: phase coupling is linear, not quadratic
- **Live inference learning**: Hebbian plasticity updates phases during generation
- **Native position**: phase angle IS position - no external encoding needed
- **Full interpretability**: every word has a visible phase, amplitude, and frequency band
- **Zero forgetting**: new knowledge adds new phasors, old ones stay
- **Continuous decoding**: ray-cast on torus, not discrete argmax

## The Fundamental Shift

| Transformer | Phiano |
|------------|--------|
| Euclidean vector space (R^d) | Complex phase manifold (C^32 torus) |
| Dot-product attention | Phase synchronization (Kuramoto) |
| Learned positional encoding | Phase angle = position (native) |
| Static weights at inference | Hebbian plasticity (live learning) |
| Gradient descent training | Phase relaxation + wave plasticity |
| Discrete token sampling | Ray-cast attractor decoding |
| O(n²) attention cost | O(n) coupling cost |

**Key insight**: The transformer treats language as a sequence of vectors. Phiano treats language as a wave — and waves naturally interfere, resonate, and propagate.

**Philosophical foundation**: John Searle (UC Berkeley) proved that syntax alone is not semantics — his Chinese Room argument (1980) shows that shuffling symbols without grounding produces no understanding. Transformers are Chinese Rooms: they shuffle token vectors through attention without knowing what the tokens mean. Phiano's 16 cognitive agents ground each word in intentional states (aboutness, reference, satisfaction conditions), escaping the Chinese Room through Searle's own framework.
