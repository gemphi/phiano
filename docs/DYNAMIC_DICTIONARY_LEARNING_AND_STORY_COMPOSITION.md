# Dynamic Dictionary Learning & Story Composition in Phiano

**Authors:**
- **Phi** (Lead Architect & Principal Investigator, Phiano Project) — `phi@phiano.org`
- **Dr. Zuzanna Stamirowska** (Complex Systems Theory & Network Dynamics, École Polytechnique / Sciences Po) — `zuzanna@phiano.org`

---

## 1. The Multi-Layer Semantic Ingestion Pipeline

When Phiano is fed a structured, multi-domain dictionary entry (such as the complete definition of **"money"** containing financial senses, legal terms, idioms, and classical Latin etymology), it executes a 4-phase cognitive ingestion pipeline without hardcoding:

```
                  STRUCTURED DICTIONARY ENTRY ("money")
                                     │
                                     ▼
      ┌─────────────────────────────────────────────────────────────┐
      │  PHASE 1: MULTI-OCTAVE HARMONIC DECOMPOSITION               │
      │  • Octave I (Surface): (mŭn′ē), moneys, monies, coinage     │
      │  • Octave II (Collocational): "medium of exchange", idioms  │
      │  • Octave III (Polysemy): Banking, Law, Wealth, Wages       │
      │  • Octave IV (Deep): Juno Moneta, Value Invariant           │
      └──────────────────────────────┬──────────────────────────────┘
                                     │
                                     ▼
      ┌─────────────────────────────────────────────────────────────┐
      │  PHASE 2: NON-LINEAR KURAMOTO PHASE ATTRACTION              │
      │  Δφ_i = η · (A_c / A_i) · sin(Φ_centroid - φ_i)             │
      │  Coherence R_c locks the polysemic cluster into equilibrium │
      └──────────────────────────────┬──────────────────────────────┘
                                     │
                                     ▼
      ┌─────────────────────────────────────────────────────────────┐
      │  PHASE 3: TOPOLOGICAL BASIN MAPPING & RIVERFLOW BEAM SEARCH │
      │  • Financial Basin: θ_fin ≈ 45° (Gold/Commerce)             │
      │  • Idiomatic Basin: θ_idm ≈ 135° (Speech/Social)            │
      │  • Historical Basin: θ_hist ≈ 225° (Classical Etymology)    │
      └──────────────────────────────┬──────────────────────────────┘
                                     │
                                     ▼
      ┌─────────────────────────────────────────────────────────────┐
      │  PHASE 4: HARMONIC STORY COMPOSITION                        │
      │  Generates continuous narrative spanning all learned basins │
      └─────────────────────────────────────────────────────────────┘
```

---

## 2. Mathematical Mapping of Polysemic Chords

### 2.1 The Multi-Sense Superposition
Rather than flattening definitions into a single point in $\mathbb{R}^d$, Phiano encodes the headword $Z_{\text{money}}$ and its surrounding sub-senses as coupled spectral phasors:

$$\Psi_{\text{money}} = Z_{\text{head}} + \sum_{k \in \text{Senses}} Z_k = A_{\text{money}} e^{i(\phi + n\alpha)} + \sum_{k} A_k e^{i(\phi_k + n_k \alpha)}$$

- **Sense 1 (Medium of Exchange)**: $\phi_1 \to$ Commerce & Trade sector ($45^\circ$).
- **Sense 2 (Wealth / Assets / Capital)**: $\phi_2 \to$ Saliency & Stored Energy sector ($60^\circ$).
- **Sense 3 (Idioms: "on the money", "in the money")**: $\phi_3 \to$ Precision & Victory sector ($120^\circ$).
- **Sense 4 (Etymology: Temple of Juno Moneta)**: $\phi_4 \to$ Historical Origin sector ($240^\circ$).

### 2.2 RiverFlow Narrative Generation
To compose a story, Phiano’s `RiverFlow` beam search sweeps sequentially through the resonant phase sectors:

$$\text{NextToken}(w_t) = \arg\min_{w \in \text{Sector}(\theta_{t+1})} \alpha |Z_w - \Psi_{\text{narrative}}|^2$$

This produces organic narrative flow where each sentence transitions harmoniously into the next without neural hallucination.

---

## 3. How to Run Training and Story Generation

### CLI Mode:
```bash
# 1. Learn the definition file
phiano> ingest data/definitions/money_complete.txt

# 2. Verify synonyms & resonance
phiano> synonym money 5

# 3. Compose a story
phiano> compose "the merchant of juno moneta and the gold currency" 3
```

### Python / API Pipeline:
```python
import urllib.request, json

API = "http://127.0.0.1:3002/api"

# Train on complete dictionary definition
text = open("data/definitions/money_complete.txt", "r").read()
req = urllib.request.Request(
    f"{API}/learn_multi",
    data=json.dumps({"text": text, "epochs": 5}).encode(),
    headers={"Content-Type": "application/json"}
)
resp = urllib.request.urlopen(req)
print(resp.read().decode())

# Compose story using learned harmonic sectors
comp_req = urllib.request.Request(
    f"{API}/compose",
    data=json.dumps({"text": "the temple of moneta minted coins of pure gold"}).encode(),
    headers={"Content-Type": "application/json"}
)
print(urllib.request.urlopen(comp_req).read().decode())
```
