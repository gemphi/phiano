# Phiano API Reference

Welcome to the official **Phiano API Reference Manual**, modeled after the PyTorch API specification.

**Phiano** (`phiano`) is the open-source Rust and Python framework for non-Euclidean cognitive spaces, spectral phasor computing, and non-linear Kuramoto phase synchronization.

---

## 🧭 Module Index

| Module | PyTorch Analog | Primary Functionality | Documentation |
| :--- | :--- | :--- | :---: |
| [`phiano::phasor`](#1-module-phianophasor) | `torch.Tensor` | Complex spectral phasors $Z = A e^{i(\phi + n\alpha)}$ on $\mathbb{T}^d$ and $\mathbb{S}^1$. | [Details](#1-module-phianophasor) |
| [`phiano::wave`](#2-module-phianowave) | `torch.nn.functional` | Wave superposition $\Psi = \sum Z_i$, ray-casting, destructive energy delta $\Delta$. | [Details](#2-module-phianowave) |
| [`phiano::layers`](#3-module-phianolayers) | `torch.nn.Module` | 64-layer multi-scale octave continuum, coarse-graining centroids, depth resonance. | [Details](#3-module-phianolayers) |
| [`phiano::trainer`](#4-module-phianotrainer) | `torch.optim` | Non-linear Kuramoto phase coupling, learning rate scheduling, Lyapunov descent. | [Details](#4-module-phianotrainer) |
| [`phiano::cognitive`](#5-module-phianocognitive) | `torch.autograd` (Agency) | John Searle intentionality, aboutness vectors $\Phi_c$, speech act classification. | [Details](#5-module-phianocognitive) |
| [`phiano::persona`](#6-module-phianopersona) | `torch.distributions` | Persona fingerprints, 16-sector histograms, style attribution, impersonation chat. | [Details](#6-module-phianopersona) |
| [`phiano::compose`](#7-module-phianocompose) | `torch.generator` | RiverFlow harmonic sector beam search, recursive tuner, candidate pruning. | [Details](#7-module-phianocompose) |
| [`phiano::oscillator`](#8-module-phianooscillator) | `torch.cuda` / 3D Mode | 3D Riemannian Kuramoto sphere $\mathbb{S}^2$, spectral entropy, chromatic wheel. | [Details](#8-module-phianooscillator) |
| [`phiano::storage`](#9-module-phianostorage) | `torch.save` / `torch.load` | Zero-copy memory-mapped binary persistence, chunk store, Bincode I/O. | [Details](#9-module-phianostorage) |

---

## 1. Module `phiano::phasor`

### `struct SpectralPhasor`

The fundamental computational unit of Phiano, analogous to a complex tensor element in PyTorch. Represents a spinning non-linear oscillator on a continuous $2\pi$ circle $\mathbb{S}^1$ with fine-structure harmonic modulation.

```rust
pub struct SpectralPhasor {
    pub phase: f64,      // Primary angle φ ∈ [0, 2π)
    pub amplitude: f64,  // Familiarity / Mass A > 0
    pub sub_band: i32,   // Quantum sub-band harmonic level n
}
```

#### Constants
- `pub const ALPHA: f64 = 1.0 / 137.035999084;` - The fine-structure coupling constant governing sub-band dispersion.

#### Methods

##### `fn new(phase: f64, amplitude: f64, sub_band: i32) -> Self`
Instantiates a new `SpectralPhasor`.
- **Parameters**:
  - `phase` ($f64$): Initial angle in radians (automatically normalized to $[0, 2\pi)$ via `rem_euclid`).
  - `amplitude` ($f64$): Lexical familiarity weight / inertial mass (enforces $A \ge 0.001$).
  - `sub_band` ($i32$): Quantized energy level.
- **Returns**: `SpectralPhasor`.

##### `fn to_complex(&self) -> num_complex::Complex64`
Computes the exact Cartesian complex representation:
$$Z = A \cdot \exp\left(i (\phi + n \cdot \alpha)\right) = A \cos(\phi + n\alpha) + i A \sin(\phi + n\alpha)$$

##### `fn delta_energy(&self, other: &Self) -> f64`
Calculates the **Destructive Wave Interference Energy Delta** $\mathcal{D}(Z_1, Z_2)$:
$$\Delta = \alpha \cdot |Z_1 - Z_2|^2 = \alpha \left[ A_1^2 + A_2^2 - 2 A_1 A_2 \cos\left( (\phi_1 + n_1\alpha) - (\phi_2 + n_2\alpha) \right) \right]$$
- **Returns**: $f64 \ge 0$. A value approaching $0.0$ indicates perfect harmonic resonance.

##### `fn conjugate(&self) -> Self`
Returns the complex conjugate phasor $Z^* = A e^{-i(\phi + n\alpha)}$.

##### `fn rotate(&mut self, delta_rad: f64)`
Rotates the primary phase angle by $\Delta \phi$ radians: $\phi \leftarrow (\phi + \Delta\phi) \pmod{2\pi}$.

---

## 2. Module `phiano::wave`

### `struct Wave`

Implements continuous complex wave operations over arbitrary sets of phasors.

```rust
pub struct Wave {
    pub value: num_complex::Complex64,
    pub token_count: usize,
}
```

#### Methods

##### `fn sentence(facet: &Facet, tokens: &[String]) -> Self`
Computes the composite sentence wave superposition $\Psi = \sum_{k=1}^N Z_k$.
- **Complexity**: $\mathcal{O}(N)$ time, $\mathcal{O}(1)$ space.

##### `fn arg(&self) -> f64`
Returns the centroid phase angle $\Phi_c = \text{atan2}(\text{Im}(\Psi), \text{Re}(\Psi)) \in [0, 2\pi)$.

##### `fn norm(&self) -> f64`
Returns the absolute amplitude magnitude $|\Psi| = \sqrt{\text{Re}^2 + \text{Im}^2}$.

##### `fn coherence(&self) -> f64`
Computes the **Kuramoto Order Parameter** $R_c = \frac{|\Psi|}{N} \in [0, 1]$.
- $R_c \to 1.0$: High coherence (constructive chord).
- $R_c \to 0.0$: Semantic dissonance / noise.

##### `fn ray_cast(&self, facet: &Facet, cone_angle: f64) -> Vec<(String, f64)>`
Performs a ray-casting intersection sweep through the active lexicon along the directional ray $\Phi_c \pm \frac{\text{cone}}{2}$.

---

## 3. Module `phiano::layers`

### `struct HierarchicalPhaseField`

Implements the **64-Layer Cognitive Octave Continuum** across the four fundamental octaves (Surface, Pattern, Semantic, Deep).

```rust
pub struct HierarchicalPhaseField {
    pub layers: Vec<PhaseLayer>, // 64 layers
}
```

#### Layer Methods

##### `fn new() -> Self`
Initializes a 64-layer hierarchy with sector resolutions halving progressively from continuous $\mathbb{S}^1$ down to 2 meta-centroids.

##### `fn build_hierarchy(&mut self, facet: &Facet)`
Performs bottom-up centroid propagation across all 64 layers in parallel using Rayon.

##### `fn resonate_depth_64(&self, target_phase: f64) -> Vec<(usize, usize, u16, f64)>`
Evaluates multi-scale depth resonance for a query phase across all 4 octaves.
- **Returns**: `Vec<(octave_id, layer_id, sector_id, resonance_score)>`.

---

## 4. Module `phiano::trainer`

### `struct KuramotoTrainer`

Governs non-linear phase attraction and amplitude reinforcement dynamics.

```rust
pub struct KuramotoTrainer {
    pub learning_rate: f64,      // η (default: 0.15)
    pub amplitude_decay: f64,    // γ (default: 0.05)
    pub max_amplitude: f64,      // A_max (default: 50.0)
    pub min_amplitude: f64,      // A_min (default: 0.1)
}
```

#### Methods

##### `fn train_sentence(&self, facet: &mut Facet, tokens: &[String]) -> TrainStats`
Executes discrete Kuramoto updates over the input utterance:
$$\phi_i \leftarrow \phi_i + \eta \cdot \frac{A_c}{A_i} \sin(\Phi_c - \phi_i)$$
$$A_i \leftarrow A_i + \gamma \left(1 - \frac{A_i}{A_{\max}}\right) \cos(\Phi_c - \phi_i)$$

---

## 5. Module `phiano::cognitive`

Implements John Searle’s Intentionality and Speech Acts theory.

### `struct IntentionalityAgent`
- `fn process(facet: &Facet, prompt: &str) -> AgentContribution`
  - Extracts the **Aboutness Vector** $\Phi_c$ and confidence metric.

### `struct SpeechActAgent`
- `fn classify(prompt: &str) -> SpeechActType`
  - Classifies illocutionary force into: `Assertive`, `Directive`, `Commissive`, `Expressive`, `Declarative`.
- `fn felicity_conditions(act: SpeechActType, prompt: &str) -> FelicityConditions`
  - Validates preparatory, sincerity, and essential conditions.

### `struct BackgroundAgent`
- `fn process(context_buffer: &ContextWaveBuffer) -> AgentContribution`
  - Quantifies pre-intentional Background capacity as accumulated context wave amplitude $|\Psi_{\text{context}}|$.

---

## 6. Module `phiano::persona`

### `struct Persona`
```rust
pub struct Persona {
    pub name: String,
    pub fingerprint: PersonaFingerprint,
    pub examples: Vec<String>,
    pub dominant_sector: u16,
}
```

#### Methods
- `fn from_text_block(name: &str, text: &str, facet: &mut Facet) -> Self`
  - Extracts a 16-sector phase histogram fingerprint from unstructured text blocks.
- `fn match_likelihood(&self, text: &str, facet: &Facet) -> f64`
  - Computes style attribution likelihood against the persona’s dominant sector distribution.

---

## 7. Module `phiano::compose`

### `struct RiverFlow`

Implements harmonic sector beam search for text composition.

```rust
pub struct RiverFlow {
    pub beam_width: usize,
    pub sector_step: f64,
    pub energy_threshold: f64,
}
```

#### Methods
- `fn compose(&self, facet: &Facet, prompt: &str, rounds: usize) -> CompositionResult`
  - Generates coherent, fluent sentences by flowing sequentially through resonant phase sectors without neural token hallucinations.

---

## 8. Module `phiano::oscillator`

### `struct OscillatorSphere`

Manages 3D Riemannian Kuramoto sphere projections ($\mathbb{S}^2$).

#### Methods
- `fn project_to_sphere(phasor: &SpectralPhasor) -> (f64, f64, f64)`
  - Converts $(A, \phi, n)$ into 3D Cartesian coordinates $(x, y, z) \in \mathbb{S}^2$.
- `fn spectral_entropy(facet: &Facet) -> f64`
  - Computes global Shannon entropy $\mathcal{H}_\theta = - \sum p_k \log_2 p_k$ across chromatic phase sectors.

---

## 9. Module `phiano::storage`

### `struct Storage`

Provides zero-copy binary serialization using Bincode.

#### Methods
- `fn save_facet(facet: &Facet, path: &Path) -> Result<(), IoError>`
- `fn load_facet(path: &Path) -> Result<Facet, IoError>`
  - Loads 100,000+ phasors in $< 5\text{ ms}$.

---

## 10. Module `phiano::server::chat_intent`

### `enum ChatIntent`

Classifies incoming conversational user messages into semantic intent modes and synthesizes continuous phase attractor responses.

```rust
pub enum ChatIntent {
    Greeting,
    SelfCorrection { statement: String, correction: String },
    Explanation { topic: String },
    PersonalMemory { statement: String },
    InstitutionalDeclaration { declaration: String },
    Recommendation { query: String },
    GeneralQuery { prompt: String },
}
```

#### Intent Variants
- **`Greeting`**: Dynamic salutation synthesized via continuous phase manifold resonance.
- **`SelfCorrection`**: Instantly triggers $\pi$-anti-phase pulse ($180^\circ$) to cancel misconceptions and retrain correct associations with zero catastrophic forgetting.
- **`Explanation`**: Grounds concepts in memory chunk definitions or multi-step attractor pathfinding.
- **`PersonalMemory`**: Registers facts, preferences, and names into the 16-layer memory hierarchy.
- **`InstitutionalDeclaration`**: Evaluates Searle declarative speech acts with World $\leftrightarrow$ Mind double direction of fit, verifies felicity conditions, and registers institutional state alterations.
- **`Recommendation`**: Pathfinds adjacent resonant topics across active memory context.
- **`GeneralQuery`**: Executes multi-step phase reasoning chains across the continuous manifold.

---

## 💻 Complete Rust End-to-End Example

```rust
use phiano::facet::Facet;
use phiano::trainer::KuramotoTrainer;
use phiano::cognitive::SpeechActAgent;
use phiano::compose::RiverFlow;

fn main() {
    // 1. Initialize lexicon facet
    let mut facet = Facet::new();
    
    // 2. Train on episodic knowledge
    let trainer = KuramotoTrainer::default();
    trainer.train_sentence(&mut facet, &["quantum", "harmonic", "oscillator"]);
    
    // 3. Classify speech act
    let act = SpeechActAgent::classify("Please generate a harmonic sequence");
    println!("Detected Speech Act: {:?}", act);
    
    // 4. Compose resonant text via RiverFlow
    let flow = RiverFlow::new(8, 0.45);
    let result = flow.compose(&facet, "harmonic resonance", 3);
    println!("Generated Thought: {}", result.text);
}
```
