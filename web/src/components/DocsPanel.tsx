import React, { useState, useMemo } from 'react';
import {
  BookOpen,
  Search,
  CheckCircle2,
  GraduationCap,
  ChevronRight,
  ChevronDown,
  ArrowRight,
  ArrowLeft,
  Cpu,
  Waves,
  Sparkles,
  Layers,
  FileCode
} from 'lucide-react';

export interface PhianoDocItem {
  id: string;
  category: 'Language Architecture' | 'Phase Resonance' | 'Model Training' | 'Inference API';
  title: string;
  badge: string;
  summary: string;
  citations: string[];
  content: string;
}

const PHIANO_DOCS: PhianoDocItem[] = [
  {
    id: 'arch/overview',
    category: 'Language Architecture',
    title: 'Phiano: Phase Instrument for Language (Architecture & Foundations)',
    badge: 'Core Architecture',
    summary: 'A first-principles tutorial on Phiano phase-coupled language modeling, complex harmonic activations, and multi-layer resonance.',
    citations: [
      'Vaswani, A., et al. (2017). Attention Is All You Need. NeurIPS.',
      'Kuramoto, Y. (1984). Chemical Oscillations, Waves, and Turbulence. Springer.',
      'Su, J., et al. (2024). RoFormer: Enhanced Transformer with Rotary Position Embedding. Neurocomputing.'
    ],
    content: `# Phiano: Phase Instrument for Language — Technical Guide

## 1. What is Phiano?
**Phiano** (Phase Instrument for Language) is an experimental high-dimensional natural language processing architecture. Instead of treating token embeddings as static Euclidean vectors, Phiano models semantic relationships as **phase-coupled harmonic oscillators**:

$$z_k(t) = r_k e^{i \\theta_k(t)} = r_k (\\cos \\theta_k(t) + i \\sin \\theta_k(t))$$

Where:
- $r_k$ represents token amplitude (salience / importance).
- $\\theta_k(t)$ represents the instantaneous phase angle on the unit complex circle.

---

## 2. Why Phase Coupling? (The Core Advantage)
In traditional Transformer architectures:
1. Positional encodings (RoPE / sinusoidal) modulate attention scores externally.
2. Contextual superposition often suffers from interference and attention drift over long contexts.

Phiano introduces **Kuramoto Phase Synchronization**:
- Semantically aligned concepts naturally phase-lock ($\\Delta \\theta \\to 0$).
- Contradictory or noisy contexts remain orthogonal in phase space ($\\Delta \\theta \\to \\pi / 2$).

---

## 3. Visual Layer Topology

\`\`\`
       +-------------------------------------------------------+
       |                  Token Ingress Stream                 |
       |  • Byte-Pair Encodings (BPE) mapped to Phase Space    |
       +-------------------------------------------------------+
                                  │
                                  ▼
       +-------------------------------------------------------+
       |               Phiano Phase Resonance Layer            |
       |  • Multi-head Complex Rotary Transformations          |
       |  • Kuramoto Phase Oscillator Coupling Matrix          |
       +-------------------------------------------------------+
                                  │
                                  ▼
       +-------------------------------------------------------+
       |               Complex Feed-Forward Network            |
       |  • Modulus non-linearities: f(r, theta) = gelu(r)*e^it|
       +-------------------------------------------------------+
                                  │
                                  ▼
       +-------------------------------------------------------+
       |                 Output Language Head                  |
       |  • Phase Demodulation & Next-Token Distribution       |
       +-------------------------------------------------------+
\`\`\`

---

## 4. Production Rust Usage
\`\`\`rust
use phiano::{PhaseModel, PhaseConfig, Tokenizer};

// 1. Initialize Phiano Model Configuration
let config = PhaseConfig {
    dim: 512,
    num_heads: 8,
    num_layers: 6,
    vocab_size: 32000,
};

let model = PhaseModel::new(config);

// 2. Encode text into complex phase trajectory
let prompt = "The resonant harmonics of phase language models";
let tokens = model.tokenize(prompt);
let output_phases = model.forward(&tokens);

println!("Computed Phase Modulus: {:.4}", output_phases.modulus());
\`\`\``
  },
  {
    id: 'phase/oscillator',
    category: 'Phase Resonance',
    title: 'Harmonic Phase Oscillators & Kuramoto Coupling',
    badge: 'Resonance Math',
    summary: 'Tutorial on non-linear phase synchronization, phase velocity dynamics, and complex harmonic states.',
    citations: [
      'Kuramoto, Y. (1975). Self-entrainment of a population of coupled non-linear oscillators. International Symposium on Mathematical Problems in Theoretical Physics.',
      'Strogatz, S. H. (2000). From Kuramoto to Crawford: exploring the onset of synchronization in populations of coupled oscillators. Physica D.'
    ],
    content: `# Harmonic Phase Oscillators & Kuramoto Coupling

## 1. What is Kuramoto Synchronization?
The **Kuramoto model** governs the non-linear interaction of $N$ coupled oscillators with natural frequencies $\\omega_i$:

$$\\frac{d\\theta_i}{dt} = \\omega_i + \\frac{K}{N} \\sum_{j=1}^N \\sin(\\theta_j - \\theta_i)$$

Where:
- $\\theta_i$ is the phase of oscillator $i$.
- $\\omega_i$ is its intrinsic natural angular velocity.
- $K$ is the coupling strength parameter.

---

## 2. Phase Coherence Order Parameter ($R$)
To quantify the macroscopic alignment of the linguistic representation:

$$R e^{i \\psi} = \\frac{1}{N} \\sum_{j=1}^N e^{i \\theta_j}$$

- $R = 1.0$: Complete phase synchronization (all oscillators aligned).
- $R = 0.0$: Incoherent, fully dispersed phase distribution.

---

## 3. Production Rust Implementation
\`\`\`rust
use phiano::oscillator::KuramotoSystem;

let mut system = KuramotoSystem::new(64, 0.25); // 64 oscillators with K=0.25
system.step(0.01); // Advance time step delta_t = 10ms

println!("Current System Order Parameter R: {:.4}", system.order_parameter());
\`\`\``
  },
  {
    id: 'training/phi4',
    category: 'Model Training',
    title: 'Phi-4 Language Studio & Dataset Fine-Tuning',
    badge: 'Training & Eval',
    summary: 'A step-by-step tutorial on tokenizing datasets, computing cross-entropy loss, and training Phiano phase layers.',
    citations: [
      'Microsoft Research (2024). Phi-4 Technical Report.',
      'Loshchilov, I., & Hutter, F. (2019). Decoupled Weight Decay Regularization (AdamW). ICLR.'
    ],
    content: `# Phi-4 Language Studio & Training Tutorial

## 1. How Training Works in Phiano
Phiano trains by minimizing complex phase loss alongside standard cross-entropy:

$$\\mathcal{L}_{\\text{total}} = \\mathcal{L}_{\\text{CE}} + \\lambda \\mathcal{L}_{\\text{phase\_coherence}}$$

Where $\\mathcal{L}_{\\text{phase\_coherence}} = 1.0 - R$ encourages the model to discover compact, synchronized representations for semantic clusters.

---

## 2. Step-by-Step Training Commands
\`\`\`bash
# 1. Run Phiano Unit & Integration Tests
cargo test -p phiano

# 2. Start Language Training Loop
cargo run --release -p phiano -- --train --dataset ./data/sample.txt
\`\`\``
  }
];

export function DocsPanel() {
  const [activeId, setActiveId] = useState<string>('arch/overview');
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [selectedCategory, setSelectedCategory] = useState<string>('All');
  const [expandedCategories, setExpandedCategories] = useState<Record<string, boolean>>({
    'Language Architecture': true,
    'Phase Resonance': true,
    'Model Training': true,
    'Inference API': true,
  });

  const categories = ['All', 'Language Architecture', 'Phase Resonance', 'Model Training'];

  const filteredDocs = useMemo(() => {
    return PHIANO_DOCS.filter((doc) => {
      const matchCat = selectedCategory === 'All' || doc.category === selectedCategory;
      const q = searchQuery.toLowerCase();
      const matchSearch =
        !q ||
        doc.title.toLowerCase().includes(q) ||
        doc.summary.toLowerCase().includes(q) ||
        doc.citations.some((c) => c.toLowerCase().includes(q)) ||
        doc.content.toLowerCase().includes(q);
      return matchCat && matchSearch;
    });
  }, [selectedCategory, searchQuery]);

  const activeDoc = useMemo(() => {
    return PHIANO_DOCS.find((d) => d.id === activeId) || filteredDocs[0] || PHIANO_DOCS[0];
  }, [activeId, filteredDocs]);

  const toggleCategory = (cat: string) => {
    setExpandedCategories((prev) => ({ ...prev, [cat]: !prev[cat] }));
  };

  return (
    <div style={{
      display: 'grid',
      gridTemplateColumns: '340px 1fr',
      gap: '1.5rem',
      height: 'calc(100vh - 120px)',
      fontFamily: 'Inter, system-ui, -apple-system, sans-serif'
    }}>
      {/* LEFT NAVIGATION MENU */}
      <div style={{
        background: 'var(--card-bg, #111827)',
        borderRadius: '12px',
        border: '1px solid var(--border-color, #1f2937)',
        padding: '1.25rem',
        display: 'flex',
        flexDirection: 'column',
        gap: '1rem',
        overflow: 'hidden'
      }}>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.6rem' }}>
            <BookOpen style={{ color: '#8b5cf6' }} size={20} />
            <h2 style={{ fontSize: '1.05rem', fontWeight: 700, color: '#f9fafb', margin: 0 }}>
              Phiano Documentation
            </h2>
          </div>
          <span style={{
            fontSize: '0.7rem',
            background: 'rgba(139, 92, 246, 0.15)',
            color: '#a78bfa',
            padding: '0.15rem 0.5rem',
            borderRadius: '12px',
            fontWeight: 600
          }}>
            {PHIANO_DOCS.length} Guides
          </span>
        </div>

        {/* Search */}
        <div style={{ position: 'relative' }}>
          <Search size={16} style={{ position: 'absolute', left: '10px', top: '10px', color: '#6b7280' }} />
          <input
            type="text"
            placeholder="Search language architecture, math..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            style={{
              width: '100%',
              padding: '0.55rem 0.55rem 0.55rem 2.1rem',
              borderRadius: '8px',
              border: '1px solid var(--border-color, #374151)',
              background: '#0f172a',
              color: '#f9fafb',
              fontSize: '0.85rem'
            }}
          />
        </div>

        {/* Category Pills */}
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.35rem' }}>
          {categories.map((cat) => (
            <button
              key={cat}
              onClick={() => setSelectedCategory(cat)}
              style={{
                padding: '0.25rem 0.6rem',
                borderRadius: '6px',
                fontSize: '0.75rem',
                border: 'none',
                cursor: 'pointer',
                background: selectedCategory === cat ? '#7c3aed' : '#1f2937',
                color: selectedCategory === cat ? '#ffffff' : '#9ca3af',
                fontWeight: selectedCategory === cat ? 600 : 400,
                transition: 'all 0.15s ease'
              }}
            >
              {cat}
            </button>
          ))}
        </div>

        {/* Menu Items */}
        <div style={{ flex: 1, overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
          {['Language Architecture', 'Phase Resonance', 'Model Training'].map((cat) => {
            const items = filteredDocs.filter((d) => d.category === cat);
            if (items.length === 0) return null;
            const isExpanded = expandedCategories[cat] !== false;

            return (
              <div key={cat} style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
                <div
                  onClick={() => toggleCategory(cat)}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    padding: '0.4rem 0.6rem',
                    cursor: 'pointer',
                    borderRadius: '6px',
                    background: '#1e1b4b',
                    color: '#c4b5fd',
                    fontSize: '0.75rem',
                    fontWeight: 700,
                    textTransform: 'uppercase',
                    letterSpacing: '0.05em'
                  }}
                >
                  <div style={{ display: 'flex', alignItems: 'center', gap: '0.4rem' }}>
                    {cat === 'Language Architecture' && <Cpu size={14} />}
                    {cat === 'Phase Resonance' && <Waves size={14} />}
                    {cat === 'Model Training' && <Sparkles size={14} />}
                    <span>{cat}</span>
                  </div>
                  {isExpanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
                </div>

                {isExpanded && (
                  <div style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem', paddingLeft: '0.5rem' }}>
                    {items.map((doc) => {
                      const isSelected = doc.id === activeDoc?.id;
                      return (
                        <div
                          key={doc.id}
                          onClick={() => setActiveId(doc.id)}
                          style={{
                            padding: '0.6rem 0.75rem',
                            borderRadius: '6px',
                            background: isSelected ? 'rgba(124, 58, 237, 0.2)' : 'transparent',
                            borderLeft: isSelected ? '3px solid #8b5cf6' : '3px solid transparent',
                            cursor: 'pointer',
                            transition: 'all 0.15s ease'
                          }}
                        >
                          <div style={{
                            fontSize: '0.825rem',
                            fontWeight: isSelected ? 600 : 500,
                            color: isSelected ? '#a78bfa' : '#e5e7eb',
                            lineHeight: '1.3'
                          }}>
                            {doc.title}
                          </div>
                        </div>
                      );
                    })}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>

      {/* RIGHT DOCUMENT VIEWER */}
      <div style={{
        background: 'var(--card-bg, #111827)',
        borderRadius: '12px',
        border: '1px solid var(--border-color, #1f2937)',
        padding: '2.25rem',
        overflowY: 'auto',
        display: 'flex',
        flexDirection: 'column',
        gap: '1.75rem'
      }}>
        {activeDoc ? (
          <>
            <div style={{ borderBottom: '1px solid #1f2937', paddingBottom: '1.25rem' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginBottom: '0.4rem' }}>
                <span style={{ fontSize: '0.75rem', fontWeight: 700, color: '#c084fc', textTransform: 'uppercase' }}>
                  {activeDoc.category}
                </span>
                <span style={{ color: '#4b5563' }}>/</span>
                <span style={{ fontSize: '0.75rem', color: '#9ca3af' }}>
                  {activeDoc.badge}
                </span>
              </div>
              <h1 style={{ fontSize: '1.85rem', fontWeight: 800, color: '#f9fafb', margin: 0, letterSpacing: '-0.02em' }}>
                {activeDoc.title}
              </h1>
            </div>

            {/* Academic References Box */}
            {activeDoc.citations.length > 0 && (
              <div style={{
                background: 'rgba(88, 28, 135, 0.2)',
                border: '1px solid rgba(139, 92, 246, 0.4)',
                borderRadius: '8px',
                padding: '1.1rem',
                display: 'flex',
                flexDirection: 'column',
                gap: '0.5rem'
              }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', color: '#c4b5fd', fontWeight: 700, fontSize: '0.85rem' }}>
                  <GraduationCap size={18} /> Foundational Literature & Academic References:
                </div>
                <ul style={{ margin: 0, paddingLeft: '1.25rem', color: '#cbd5e1', fontSize: '0.825rem', lineHeight: '1.6' }}>
                  {activeDoc.citations.map((cit, idx) => (
                    <li key={idx}>{cit}</li>
                  ))}
                </ul>
              </div>
            )}

            <div style={{
              color: '#d1d5db',
              lineHeight: '1.75',
              fontSize: '0.95rem'
            }}>
              <pre style={{
                whiteSpace: 'pre-wrap',
                fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
                background: 'transparent',
                margin: 0,
                padding: 0,
                color: '#e5e7eb'
              }}>
                {activeDoc.content}
              </pre>
            </div>
          </>
        ) : null}
      </div>
    </div>
  );
}
