# SPEC-002: Kuramoto Phase Synchronization & Order Parameter Dynamics

## 1. Executive Summary & Theoretical Grounding

> **Deep Learning Concept Reference (Chollet DL Book §14.2)**:
> *"Systems operating in high-dimensional continuous domains undergo non-linear phase transitions when sub-components synchronize. Detecting out-of-distribution extremes requires tracking global coherence parameters that quantify collective synchronization versus independent dispersion."*

Phiano models interacting agents, semantic concepts, and multi-venue financial order flow as a coupled network of non-linear Kuramoto oscillators. The global **Kuramoto Order Parameter** $R(t)$ provides an exact quantitative metric of collective phase synchronization, predicting flash crashes, herd behavior, and conceptual lock-in before they destabilize operations.

---

## 2. Architectural Hierarchy Tree

```
phiano::kuramoto / phiano::coherence
├── Coupled Kuramoto Oscillator Network
│   ├── Individual Oscillator Phases: θ_i(t) ∈ [0, 2π) for i = 1..N
│   ├── Natural Intrinsic Frequencies: ω_i in R (Lorentzian/Gaussian distribution)
│   ├── Global Coupling Strength: K ≥ 0
│   ├── Pairwise Interaction Matrix: K_ij = K * A_ij (Network Topology)
│   └── Differential Dynamics: dθ_i/dt = ω_i + (K/N) * sum_j sin(θ_j - θ_i)
├── Order Parameter Calculation Subsystem
│   ├── Complex Order Vector: Z(t) = R(t) * exp(i * Ψ(t)) = (1/N) * sum_j exp(i * θ_j)
│   ├── Coherence Magnitude: R(t) = |Z(t)| ∈ [0.0, 1.0]
│   ├── Mean Collective Phase: Ψ(t) = atan2(Im(Z), Re(Z))
│   └── Variance of Phase Distribution: Var(θ) = 1.0 - R(t)
├── Non-Linear Regime & Cascade Classifier
│   ├── Incoherent Dispersed Phase: R(t) < 0.35 (Healthy independent activity)
│   ├── Partial Coherence Phase: 0.35 ≤ R(t) < 0.80 (Structured semantic alignment)
│   ├── Hyper-Synchronized Phase Lock: R(t) ≥ 0.90 (Flash crash / cascade alert)
│   └── Phase Velocity Detector: dR/dt (Detects sudden onset of runaway synchronization)
└── Numerical Integration Engine
    ├── 4th-Order Runge-Kutta (RK4) Step Integrator
    └── Fast Mean-Field Vector Optimizer: O(N) evaluation replacing O(N^2) pairwise loops
```

---

## 3. Component Interaction & Execution Flow

```mermaid
flowchart TD
    A[N Interacting Nodes: Venues, Agents, Concepts] --> B[Extract Instantaneous Phases: θ_1..θ_N]
    
    B --> C[Kuramoto Network State Vector]
    
    subgraph "Kuramoto Dynamic Integration"
        C --> D[Compute Pairwise Couplings: (K/N) * sin(θ_j - θ_i)]
        D --> E[Runge-Kutta 4th Order RK4 Step]
        E --> F[Update Phase Vector: θ_t+Δt]
    end
    
    F --> G[Compute Complex Mean: Z_t = (1/N) * sum(exp(i * θ_j))]
    G --> H[Extract Magnitude R_t and Mean Phase Ψ_t]
    
    H --> I{Regime Classification}
    I -- R_t < 0.35 --> J[Dispersed / Independent State]
    I -- 0.35 ≤ R_t < 0.80 --> K[Harmonic Alignment State]
    I -- R_t ≥ 0.90 --> L[Critical Phase Lock Cascade Alert]
    
    L --> M[Emit Emergency Signal to Phixum Risk Circuit Breakers]
    H --> N[Stream Coordinates to Puijs 3D Bloch Sphere]
    
    subgraph "Downstream Observability"
        N --> N1[Render Phase Arrow on Sphere]
        N --> N2[Trigger Cascade Shockwave Pulse]
    end
```

---

## 4. Technical Specification & Data Structures

### 4.1 Kuramoto Synchronization Metrics

| Metric Symbol | Name | Nominal Range | Phase Transition Range | Critical Alarm Threshold | Downstream System Action | SLA Response |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **$R(t)$** | Order Parameter | $[0.15, 0.55]$ | $0.60 \to 0.85$ | $\ge 0.90$ | Emergency quote widening & cancellation | $<50\mu\text{s}$ |
| **$\Psi(t)$** | Mean Phase Angle | $[0, 2\pi)$ | Uniform drift | Stationary lock | Identifies anchor direction of cascade | $<50\mu\text{s}$ |
| **$\frac{dR}{dt}$** | Synchrony Velocity | $(-0.1, +0.1)$ | $>+0.25/\text{s}$ | $>+0.50/\text{s}$ | Pre-emptive risk margin multiplier ramp-up | $<100\mu\text{s}$ |
| **$K_{\text{eff}}$** | Effective Coupling | $[0.5, 2.0]$ | $>3.5$ | $\ge 5.0$ | Indicates runaway feedback loop in market | $<1\text{ms}$ |
| **$\sigma_{\theta}^2$** | Circular Variance | $[0.45, 0.85]$ | $0.15 \to 0.40$ | $\le 0.10$ | Alerts on loss of market diversity | $<100\mu\text{s}$ |

### 4.2 Mathematical Formulations

#### 4.2.1 Differential Evolution Equation
$$\frac{d\theta_i}{dt} = \omega_i + \frac{K}{N} \sum_{j=1}^N \sin(\theta_j - \theta_i)$$

#### 4.2.2 Fast Mean-Field Vector Transformation
By trigonometric identity, the pairwise interaction simplifies to mean-field form:
$$\frac{d\theta_i}{dt} = \omega_i + K R(t) \sin(\Psi(t) - \theta_i)$$
This reduces computational complexity from $O(N^2)$ to $O(N)$ per integration step, enabling real-time simulation of $10,000+$ oscillators at $1,000\text{ Hz}$.

---

## 5. Rust Implementation Signatures

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KuramotoNetwork {
    pub natural_frequencies: Vec<f64>,
    pub phases: Vec<f64>,
    pub coupling_k: f64,
}

impl KuramotoNetwork {
    pub fn new(natural_frequencies: Vec<f64>, coupling_k: f64) -> Self;
    pub fn step_rk4(&mut self, dt: f64);
    pub fn order_parameter(&self) -> (f64, f64); // (R, Psi)
    pub fn is_phase_locked(&self, threshold: f64) -> bool;
    pub fn set_coupling(&mut self, k: f64);
    pub fn circular_variance(&self) -> f64;
    pub fn compute_phase_velocity(&self, dt: f64, prev_r: f64) -> f64;
}

pub struct KuramotoCascadeDetector {
    network: KuramotoNetwork,
    alarm_threshold: f64,
    cooldown_ticks: usize,
}

impl KuramotoCascadeDetector {
    pub fn new(network: KuramotoNetwork, alarm_threshold: f64) -> Self;
    pub fn update_and_check(&mut self, dt: f64) -> Option<CascadeAlertEvent>;
}
```

---

## 6. Verification & Test Criteria

1. **Theoretical Critical Coupling $K_c$**: For Lorentzian natural frequency distribution $g(\omega) = \frac{\gamma}{\pi(\omega^2 + \gamma^2)}$, the network must transition from $R \approx 0$ to $R > 0$ at theoretical threshold $K_c = 2\gamma$.
2. **Mean-Field Equivalence**: The $O(N)$ mean-field formulation must match the full $O(N^2)$ pairwise sum within machine precision ($\epsilon < 10^{-14}$).
3. **Integration Stability**: RK4 step must conserve phase bounds $[0, 2\pi)$ across $1,000,000$ iterations without floating-point drift.
4. **Sub-Millisecond Execution**: Integrating a 1,000-node network for 100 timesteps must execute in $<2.5\text{ms}$ on a single core.
