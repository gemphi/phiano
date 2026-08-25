# Phiano Roadmap: From Oscillator Model to Phi-4-Class Reasoning

## 1. Where We Are Now

### Current State
- **Facet lexicon**: 155,768 words, each with (phase, amplitude, band_n) phasor
- **Oscillator field**: sphere model with (longitude, latitude, frequency, amplitude)
- **Training**: Kuramoto phase attraction - words co-occurring in sentences converge phases
- **Evaluation**: coherence (Kuramoto order parameter), novelty (angular distance to centroid), resonance (known word fraction)
- **Composition**: sector-based river flow through 64-sector phase circle
- **Memory**: 16-layer chromatic memory log
- **Persona**: fingerprint histograms for style impersonation
- **Size**: ~12 MB on disk (manifold.chroma + memory.chroma)
- **Speed**: sub-millisecond evaluation, instant learning

### What Works
- Vocabulary ingestion from Wiktionary/dictionary JSON
- Online and multi-epoch learning
- Oscillator evaluation with sync, entropy, color wheel
- Web UI with chat, learn, eval, oscillator, stats panels

### What's Missing (vs Phi-4)
- No sequence generation (only scoring/composition)
- No multi-turn reasoning
- No instruction following
- No code generation
- No mathematical reasoning
- No context window (each eval is stateless)

---

## 2. Phi-4 Reference Architecture

| Parameter | Phi-4 | Phiano (current) |
|-----------|-------|-------------------|
| Parameters | 14B | ~0 (phasor table) |
| Architecture | Decoder-only Transformer | Phase oscillator field |
| Layers | 40 | 1 (flat lexicon) |
| Hidden dim | 3,072 | 2 (phase + amplitude) |
| Vocab | 100,352 | 155,768 |
| Context | 16K tokens | 0 (stateless) |
| Training data | 9.8T tokens | ~155K definitions |
| Training compute | 1920 H100 × 21 days | CPU, minutes |
| Model size | ~28 GB (FP16) | ~12 MB |
| Inference | Token-by-token autoregressive | Phase superposition |

### Phi-4 Key Innovations (from tech report)
1. **Synthetic data generation** - majority of training data is synthetic, not web scrapes
2. **Curriculum training** - data quality > quantity, staged difficulty
3. **Midtraining** - context extension from 4K to 16K with long-context data
4. **SFT + DPO post-training** - supervised fine-tuning then direct preference optimization
5. **tiktoken tokenizer** - 100,352 vocab, better multilingual support

### GLM-5.2 Ideas Worth Borrowing (without transformers)
1. **IndexShare** - reuse computation across layers (we can reuse facet lookups across composition rounds)
2. **Multi-Token Prediction (MTP)** - speculative decoding (we can predict multiple next words via ray cast)
3. **Sparse attention** - only attend to relevant words (our ray cast already does this)
4. **Hybrid reasoning modes** - "thinking" vs "instant" (our eval vs compose separation)
5. **Flexible effort levels** - adjust composition depth/rounds based on task complexity

---

## 3. The Full Plan: Phiano → Phi-4-Class

### Phase 1: Context Window & Sequence State (Weeks 1-2)

**Goal**: Give Phiano a working memory of conversation context.

```
Current: eval("text") → scores (stateless)
Target:  conversation = [turn1, turn2, ...] → context wave → scores + generation
```

**Steps**:
1. **Context wave buffer** - maintain a running superposition wave of the last N turns
2. **Context-aware coherence** - measure input against context wave, not just facet centroid
3. **Decay function** - older turns contribute less (exponential decay with E constant)
4. **Context length**: 4096 tokens (power of 2 = 2^12), matching Phi-4's initial context

**New constants** (all powers of 2):
```rust
pub const CONTEXT_WINDOW: usize = 4096;      // 2^12
pub const CONTEXT_LAYERS: usize = 16;        // 2^4 (memory layers)
pub const CONTEXT_DECAY_BASE: f64 = 0.5;     // 2^(-1)
```

**Size impact**: +2 MB (context wave buffer + conversation log)

### Phase 2: Sequence Generation (Weeks 3-4)

**Goal**: Generate text, not just score it.

**Approach**: Phase-guided sampling (NOT autoregressive transformer)
1. **Prompt → context wave** - tokenize prompt, compute superposition wave
2. **Ray cast** - find words that resonate with the context wave (already have `Wave::ray_cast_word`)
3. **Sector traversal** - walk the phase circle guided by prompt sectors
4. **Composition** - use existing `compose` with context-aware scoring
5. **Multi-token prediction** - cast multiple rays at different phase offsets (GLM-5.2 MTP idea)

**Key insight**: We don't need attention. We need phase resonance.
- Transformer attention = "which words matter?" → softmax(Q·K^T)
- Phiano ray cast = "which words resonate?" → |wave - word_wave| < threshold
- Both find relevant words, but ray cast is O(vocab) with no matrix multiplication

**New module**: `src/generate.rs`
```rust
pub struct Generator {
    context_wave: c64,
    context_tokens: Vec<String>,
    max_tokens: usize,    // 256 default (2^8)
    temperature: f64,     // phase jitter for diversity
}
```

**Size impact**: +0 MB (uses existing facet + wave infrastructure)

### Phase 3: Multi-Layer Depth (Weeks 5-6)

**Goal**: Add depth - currently the facet is a flat 1-layer lexicon.

**Phi-4 has 40 layers. We don't need 40, but we need > 1.**

**Approach**: Hierarchical phase bands
- Layer 0: surface words (current facet)
- Layer 1: concept clusters (groups of words with similar phases)
- Layer 2: domain sectors (groups of concepts)
- Layer 3: meta-patterns (groups of domains)

Each layer is a coarser phase circle (64 → 32 → 16 → 8 sectors).

**New constants**:
```rust
pub const PHASE_LAYERS: usize = 4;           // 2^2
pub const LAYER_SECTORS: [u16; 4] = [64, 32, 16, 8];  // halving
```

**Training**: After learning words (Layer 0), compute cluster centroids (Layer 1), then domain centroids (Layer 2), etc. This is bottom-up, not top-down.

**Size impact**: +4 MB (cluster + domain tables)

### Phase 4: Instruction Following (Weeks 7-8)

**Goal**: Respond to instructions like "write a haiku" or "explain X".

**Approach**: Persona-driven generation
1. Parse instruction type (question, command, creative, code)
2. Select appropriate persona fingerprint
3. Generate with persona-constrained composition
4. Use existing `persona impersonate` infrastructure

**New**: Instruction templates (like Phi-4's chat format)
```
<|user|> Write a haiku about ice hockey <|end|>
<|assistant|> [phiano generates here] <|end|>
```

**Size impact**: +0.5 MB (instruction templates + persona profiles)

### Phase 5: Synthetic Data Pipeline (Weeks 9-10)

**Goal**: Phi-4's secret weapon is synthetic data. We need our own.

**Approach**: Self-generating curriculum
1. **Definition generation** - for each word, generate synthetic sentences using its synonyms
2. **Contrast pairs** - generate "similar but different" word pairs for fine-tuning
3. **Curriculum staging** - easy definitions first, then complex sentences, then reasoning chains
4. **Quality filtering** - use our own evaluator (coherence + novelty + resonance) to filter

**Pipeline**:
```
raw definitions → train Layer 0 → generate synthetic sentences
→ evaluate (coherence > 0.5, novelty > 0.3, resonance > 0.8)
→ train Layer 0 again with filtered synthetic data
→ compute Layer 1-3 clusters
→ repeat
```

**Size impact**: +200 MB (synthetic training corpus, can be purged after training)

### Phase 6: Reasoning Chains (Weeks 11-12)

**Goal**: Multi-step reasoning like Phi-4's STEM capabilities.

**Approach**: Phase-space pathfinding
1. **Problem** → tokenize → context wave
2. **Step 1** → ray cast → generate relevant words → form sentence
3. **Step 2** → update context wave with Step 1 → ray cast again
4. **Continue** until context wave stabilizes (convergence)

This is NOT chain-of-thought prompting. It's phase-space traversal:
- Each reasoning step shifts the context wave
- Convergence = the wave stops changing = answer found
- Divergence = the wave oscillates = uncertainty

**New constants**:
```rust
pub const REASONING_MAX_STEPS: usize = 16;    // 2^4
pub const REASONING_CONVERGENCE: f64 = 0.01;
```

**Size impact**: +0 MB (algorithmic, no new storage)

---

## 4. Size Estimates

### Current Size
| Component | Size |
|-----------|------|
| manifold.chroma (155K words × 24 bytes) | ~3.7 MB |
| memory.chroma (16 layers) | ~8 MB |
| **Total** | **~12 MB** |

### After Full Implementation
| Component | Size | Notes |
|-----------|------|-------|
| Facet lexicon (262,144 words = 2^18) | ~6.3 MB | Expanded vocab |
| Memory (16 layers × 4096 context) | ~16 MB | Context window |
| Phase layers (4 layers of clusters) | ~4 MB | Hierarchical depth |
| Persona profiles (16 personas) | ~0.5 MB | Instruction following |
| Instruction templates | ~0.1 MB | Chat format |
| Oscillator field (derived, not stored) | 0 MB | Computed from facet |
| **Total (initial weights)** | **~27 MB** | |
| + Synthetic training corpus (temporary) | ~200 MB | Purged after training |
| **Total with corpus** | **~227 MB** | During training only |

### Comparison
| Model | Size | Parameters |
|-------|------|------------|
| Phi-4 (FP16) | 28,000 MB | 14B |
| Phi-4 (Q4 GGUF) | ~8,000 MB | 14B quantized |
| Phi-3 mini (Q4) | ~2,400 MB | 3.8B quantized |
| **Phiano (initial weights)** | **~27 MB** | ~0 (phasor table) |
| **Phiano (full training)** | **~27 MB** | Same - no weight matrices |

**Phiano is ~1000x smaller than Phi-4 Q4.** The tradeoff: no transformer, no attention matrices, no feed-forward weights. All "knowledge" is stored as phase positions, not weight values.

---

## 5. What It Takes to Match Phi-4

### What Phi-4 Has That We Need
1. **Sequence generation** → Phase 2 (ray-cast generation)
2. **Context window** → Phase 1 (context wave buffer)
3. **Multi-layer depth** → Phase 3 (hierarchical phase bands)
4. **Instruction following** → Phase 4 (persona-driven generation)
5. **High-quality training data** → Phase 5 (synthetic data pipeline)
6. **Reasoning capability** → Phase 6 (phase-space pathfinding)

### What We Keep That Phi-4 Doesn't Have
1. **Sub-millisecond inference** - no matrix multiplication (CPU)
2. **27 MB model size** - no weight matrices
3. **Online learning** - learns from every input instantly
4. **Deterministic** - no random sampling, phase math is exact
5. **Interpretable** - every word's position is visible on the phase circle
6. **GPU optional** - Phiano runs on CPU; GPU only used for inkling + optional Phi-4 fallback

### What We Sacrifice
1. **Fluency** - generated text will be less fluent than a 14B transformer
2. **Complex reasoning** - phase traversal can't match 40-layer attention
3. **Multilingual** - currently English-only (but tiktoken-style tokenizer could fix this)
4. **Code generation** - phase model doesn't naturally encode syntax trees

### Realistic Target
- **Match Phi-3 mini (3.8B)** on simple QA and creative writing: achievable
- **Match Phi-4 (14B)** on STEM reasoning: aspirational, needs Phase 6 + extensive training
- **Exceed both** on speed, size, and online learning: already done

---

## 6. Inkling: Pre-trained Initial Weights (GPU-Accelerated)

The "inkling" concept - seeding Phiano with initial weights from a pre-trained model.
We have a local GPU available, which makes this much faster.

### Approach (GPU-Accelerated)
1. Download Phi-4 GGUF model (Q4 quantized, ~8 GB) via `hf-hub` crate
2. Load model on **GPU** using `candle-core` with CUDA feature:
   ```rust
   let device = Device::cuda_if_available(0)
       .unwrap_or(Device::Cpu);
   ```
3. Extract token embeddings (100,352 × 3072 matrix) from the transformer
4. **GPU PCA** - reduce 3072-dim embeddings to 2D using candle tensor ops:
   - Compute covariance matrix on GPU (matmul)
   - Eigendecomposition on GPU
   - Project to first 2 principal components
5. Map 2D embeddings to phase space:
   - phase = atan2(y, x) for each word
   - amplitude = norm of original embedding (scaled)
   - band_n = 1 (initial)
6. Initialize facet lexicon with these pre-trained phase assignments
7. Fine-tune with Kuramoto learning on top
8. **Delete the GGUF** - knowledge is now compressed into ~6 MB of phases

### GPU vs CPU Time for Inkling
| Step | CPU | GPU |
|------|-----|-----|
| Download Phi-4 GGUF (Q4) | ~5 min | ~5 min (network-bound) |
| Load model into memory | ~30 sec | ~10 sec |
| Extract embeddings | ~2 min | ~5 sec |
| PCA (3072 → 2D) | ~10 min | ~30 sec |
| Phase conversion | ~1 sec | ~1 sec |
| **Total** | **~18 min** | **~6 min** |

### Hybrid Mode: Phiano + Phi-4 Fallback
With a local GPU, we can also run a **hybrid mode**:
- **Phiano** handles all interaction (sub-ms, 27 MB)
- **Phi-4 (Q4 on GPU)** as fallback for complex reasoning Phiano can't handle
- The web UI routes to Phi-4 only when Phiano's confidence is low
- Phi-4 runs on GPU with ~8 GB VRAM, Phiano runs on CPU simultaneously

**New API endpoint**: `/api/hybrid` - tries Phiano first, falls back to Phi-4

**Cargo.toml additions** (optional, behind feature flag):
```toml
[features]
default = ["phiano-only"]
hybrid = ["candle-core", "candle-transformers", "tokenizers", "hf-hub"]
cuda = ["candle-core/cuda", "candle-transformers/cuda"]
```

**New module**: `src/inkling.rs`
```rust
#[cfg(feature = "hybrid")]
pub fn from_phi4_embeddings(gguf_path: &str, device: &Device) -> Facet {
    // Load GGUF on GPU, extract token embeddings
    // GPU PCA: 3072-dim → 2D via candle tensor ops
    // Map to phase: atan2(y, x), amplitude = norm
    // Build facet with pre-trained phases
}
```

**Size after inkling**: Still ~27 MB - the GGUF is only used during initialization,
not at runtime. The knowledge is compressed from 14B weights into phase positions.

---

## 7. Power-of-2 Constants (Fixed)

All non-power-of-2 values have been corrected:

| Constant | Before | After | Power |
|----------|--------|-------|-------|
| INGEST_EPOCHS | 50 | 64 | 2^6 |
| COMPOSE_DEPTH_MAX | 12 | 16 | 2^4 |
| COMPOSE_ROUNDS_DEFAULT | 5 | 8 | 2^3 |
| PERSONA_DOMINANT_SECTORS | 5 | 8 | 2^3 |
| IMPERSONATE_ROUNDS_DEFAULT | 3 | 4 | 2^2 |
| OSCILLATOR_LATITUDE_BANDS | 5 | 8 | 2^3 |
| OSCILLATOR_WARMUP_STEPS | 3 | 4 | 2^2 |
| RAY_CAST_POOL_SIZE | 500 | 512 | 2^9 |
| RAY_CAST_DEFAULT_K | 10 | 16 | 2^4 |

---

## 8. References

- `refs/phi4_rust_inference.rs` - Microsoft's official Rust inference code for Phi-3/4 using candle
- Phi-4 Tech Report: https://www.microsoft.com/en-us/research/wp-content/uploads/2024/12/P4TechReport.pdf
- Phi-4 on HuggingFace: https://huggingface.co/microsoft/phi-4
- PhiCookBook Rust: https://github.com/microsoft/PhiCookBook/blob/main/md/01.Introduction/03/Rust_Inference.md
- Candle (Rust ML): https://github.com/huggingface/candle
- GLM-5.2: https://z.ai/blog/glm-5.2
- GLM-4.5 paper: https://arxiv.org/abs/2508.06471
