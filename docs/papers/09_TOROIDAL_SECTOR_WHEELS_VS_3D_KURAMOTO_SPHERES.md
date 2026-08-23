# Toroidal Sector Wheels vs. 3D Kuramoto Spheres: Visualizing Glass-Box Latent Spaces

**Authors:**
- **Phi** (Lead Architect & Principal Investigator, Phiano Project) — `phi@phiano.org`
- **Dr. Zuzanna Stamirowska** (Complex Systems Theory & Network Dynamics, École Polytechnique / Sciences Po) — `zuzanna@phiano.org`

---

## Abstract

A major obstacle to the safe deployment of artificial intelligence is the uninterpretable black-box nature of high-dimensional vector embeddings in $\mathbb{R}^d$. Dimensionality reduction techniques like t-SNE or UMAP introduce severe non-linear distortions and lack dynamic temporal fidelity.

In this paper, we present the geometric visualization engines of Phiano: **2D Toroidal Sector Wheels** ($\mathbb{S}^1$) and **3D Riemannian Kuramoto Spheres** ($\mathbb{S}^2$). We formulate the mapping from complex spectral phasors to physical chromatic coordinates (hue, saturation, brightness), demonstrate how semantic fields appear as continuous color topologies, and show how human operators can intuitively observe, diagnose, and steer AI reasoning in real-time.

---

## 1. The Glass-Box Visual Architecture

In Phiano, interpretability is not an afterthought added post-hoc; it is an intrinsic consequence of the underlying physics:

$$\text{Phase Angle } \phi \in [0, 2\pi) \longleftrightarrow \text{Chromatic Hue } \theta \in [0^\circ, 360^\circ)$$
$$\text{Amplitude } A \in \mathbb{R}^+ \longleftrightarrow \text{Saturation / Mass}$$
$$\text{Latitude } \theta_{\text{lat}} \in [-\pi/2, \pi/2] \longleftrightarrow \text{Brightness / Abstraction Level}$$

```
                           THE 3D KURAMOTO SPHERE
                           
                                North Pole (+90°)
                           [Pure Mathematical Abstract]
                                       │
                               ╭───────┴───────╮
                            ╭──╯   Brightness  ╰──╮
                          ╭─╯          ▲          ╰─╮
                         │             │             │
        180° Cyan ◄──────┼─────────────┼─────────────┼──────► 0° Red
    (Passive/Reflective) │             ▼             │     (Action/Concrete)
                         │       Hue (Longitude)     │
                          ╰─╮                     ╭─╯
                            ╰──╮       ▲       ╭──╯
                               ╰───────┬───────╯
                                       │
                                South Pole (-90°)
                            [Emotive / Raw Affect]
```

---

## 2. 2D Toroidal Sector Wheel

The 2D Toroidal Sector Wheel divides the unit circle $\mathbb{S}^1$ into 16 fundamental chromatic sectors:

| Sector Index | Phase Range (Radians) | Semantic Domain | Dominant Hue |
| :---: | :---: | :--- | :--- |
| **0** | $[0.00, 0.39)$ | Concrete Action, Motion, Physics | Crimson Red ($0^\circ$) |
| **2** | $[0.78, 1.18)$ | Construction, Engineering, Tools | Amber Orange ($45^\circ$) |
| **4** | $[1.57, 1.96)$ | Biological Life, Nature, Growth | Emerald Green ($90^\circ$) |
| **6** | $[2.36, 2.75)$ | Social Relations, Communication | Cyan ($135^\circ$) |
| **8** | $[3.14, 3.53)$ | Contemplation, Philosophy, Logic | Royal Blue ($180^\circ$) |
| **10** | $[3.93, 4.32)$ | Metaphysics, Abstract Time | Deep Violet ($225^\circ$) |
| **12** | $[4.71, 5.10)$ | Affect, Emotion, Timbre | Magenta ($270^\circ$) |
| **14** | $[5.50, 5.89)$ | Negation, Void, Boundaries | Dark Carmine ($315^\circ$) |

---

## 3. 3D Kuramoto Spherical Projection

In Oscillator Mode (`om`), phasors are mapped to the 2-sphere $\mathbb{S}^2$:

$$\mathbf{r}_k = \left( \sin\theta_k \cos\phi_k, \, \sin\theta_k \sin\phi_k, \, \cos\theta_k \right) \in \mathbb{R}^3$$

The Order Parameter Vector $\mathbf{R} = \frac{1}{N} \sum_k \mathbf{r}_k$ indicates the global orientation of the discourse in 3D cognitive space.

---

## 4. Real-Time WebGL Streaming

Phiano streams real-time spherical coordinates via WebSocket to the Phiano Web UI (`http://localhost:5173/`), rendering dynamic particle clouds and Kuramoto attraction fields at 60 frames per second.

---

## 5. Conclusion

By replacing opaque Euclidean embeddings with chromatic non-Euclidean manifolds, Phiano turns artificial intelligence into a true glass box. Human operators can literally *see* thoughts form as spinning harmonic wave topologies.
