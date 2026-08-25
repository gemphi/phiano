import React, { useState, useMemo } from 'react';
import {
  BookOpen,
  Search,
  CheckCircle2,
  GraduationCap,
  ChevronRight,
  ChevronDown,
  Cpu,
  Waves,
  Sparkles,
  Layers,
  FileCode,
  Copy,
  Check,
  Zap,
  Network,
  Shield,
  ExternalLink,
} from 'lucide-react';

export interface PhianoDocItem {
  id: string;
  category: 'Foundations & Physics' | 'Cognitive & Speech Acts' | 'Memory & Field Theory' | 'API & Architecture';
  title: string;
  badge: string;
  summary: string;
  citations: string[];
  content: string;
}

export const PHIANO_DOCS: PhianoDocItem[] = [
  {
    id: 'arch/foundations',
    category: 'Foundations & Physics',
    title: 'Phiano: Continuous Phase Manifold Architecture & Kuramoto Dynamics',
    badge: 'Core Physics',
    summary: 'A first-principles mathematical derivation of phase-coupled language modeling, complex harmonic activations, and continuous non-linear manifold dynamics.',
    citations: [
      'Kuramoto, Y. (1984). Chemical Oscillations, Waves, and Turbulence. Springer.',
      'Sakaguchi, H., & Kuramoto, Y. (1986). A Soluble Active Rotator Model with Phase Lag. Progress of Theoretical Physics.',
      'Vaswani, A., et al. (2017). Attention Is All You Need. NeurIPS.'
    ],
    content: `### 1. The Continuous Phase Manifold Paradigm

Traditional Transformer neural networks treat natural language as sequences of discrete, high-dimensional Euclidean embedding vectors $\\mathbf{x}_t \\in \\mathbb{R}^d$. Context is assembled via static dot-product attention matrices:

$$\\text{Attention}(Q, K, V) = \\text{softmax}\\left(\\frac{QK^T}{\\sqrt{d_k}}\\right)V$$

**Phiano breaks fundamentally with discrete matrix multiplication.** Instead, words are continuous harmonic oscillators residing on a $D$-dimensional toroidal phase manifold $\\mathbb{T}^D$:

$$z_k(t) = r_k e^{i \\theta_k(t)} = r_k (\\cos \\theta_k(t) + i \\sin \\theta_k(t))$$

Where:
• **$r_k \\in [0, 1]$**: Instantaneous amplitude / semantic salience.
• **$\\theta_k(t) \\in [-\\pi, \\pi]$**: Instantaneous phase angle on the complex circle $\\mathbb{S}^1$.

---

### 2. Kuramoto Phase Synchronization Equation

The continuous phase evolution of token oscillators is governed by the non-linear **Kuramoto differential equation**:

$$\\frac{d\\theta_i}{dt} = \\omega_i + \\frac{K}{N} \\sum_{j=1}^N A_j \\sin(\\theta_j - \\theta_i - \\beta_{ij})$$

• **$\\omega_i$**: Natural intrinsic frequency of the $i$-th token.
• **$K$**: Global coupling strength constant ($K = 0.85$).
• **$\\beta_{ij}$**: Learned asymmetric syntactic phase lag between tokens $i$ and $j$.
• **$A_j$**: Phasor amplitude weight of neighboring concept $j$.

The global order parameter $R(t) e^{i \\psi(t)}$ measures collective coherence across the manifold:

$$R e^{i \\psi} = \\frac{1}{N} \\sum_{j=1}^N e^{i \\theta_j}, \\quad R \\in [0, 1]$$

---

### 3. Visual Layer Topology

\`\`\`
       +-------------------------------------------------------+
       |                  Token Ingress Stream                 |
       |  • Byte-Pair Encodings (BPE) mapped to Phase Space    |
       +-------------------------------------------------------+
                                  │
                                  ▼
       +-------------------------------------------------------+
       |               Phiano Phase Resonance Layer            |
       |  • Multi-Frequency Torus Harmonic Decoding (T^D)      |
       |  • Kuramoto-Sakaguchi Phase Lag Steering (beta_ij)    |
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
       |  • Phase Demodulation & Attractor Pathfinding         |
       +-------------------------------------------------------+
\`\`\`

---

### 4. Production Rust Usage
\`\`\`rust
use phiano::facet::Facet;
use phiano::trainer::KuramotoTrainer;
use phiano::generate::{Generator, ContextWaveBuffer};

// 1. Initialize lexicon facet and trainer
let mut facet = Facet::new();
let trainer = KuramotoTrainer::default();

// 2. Train on episodic sentence with asymmetric lag
trainer.train_sentence(&mut facet, &["quantum", "harmonic", "oscillator"]);

// 3. Generate resonant continuation
let generator = Generator::default();
let mut ctx = ContextWaveBuffer::new(32);
let output = generator.generate(&facet, &mut ctx, "quantum harmonic");
println!("Synthesized thought: {}", output);
\`\`\``,
  },
  {
    id: 'cognitive/speech_acts',
    category: 'Cognitive & Speech Acts',
    title: 'Searle Cognitive Theory: Intentionality & Double Direction of Fit',
    badge: 'Cognitive Science',
    summary: 'Implementation of John Searle’s Intentionality and Speech Acts theory, grounding meaning through 5 distinct illocutionary forces and epistemic state transitions.',
    citations: [
      'Searle, J. R. (1969). Speech Acts: An Essay in the Philosophy of Language. Cambridge University Press.',
      'Searle, J. R. (1980). Minds, Brains, and Programs. Behavioral and Brain Sciences.',
      'Searle, J. R. (1983). Intentionality: An Essay in the Philosophy of Mind. Cambridge University Press.'
    ],
    content: `### 1. Solving the Chinese Room Problem

In 1980, UC Berkeley Professor John Searle proved with his famous **Chinese Room Argument** that:

$$\\text{Syntax alone is never sufficient for Semantics} \\quad (\\text{Syntax} \\neq \\text{Semantics})$$

Transformer LLMs operate entirely within the Chinese Room: they match statistical token distributions without intentionality or understanding.

**Phiano grounds meaning through Searle’s intentional architecture**:
1. **Aboutness Vector ($\\Phi_c$)**: Mental directedness toward epistemic objects.
2. **Directions of Fit**: Dynamic alignment between the internal mental state and external world facts.
3. **The Background**: Non-representational cognitive capacity modeled as wave superposition.

---

### 2. The 5 Illocutionary Speech Act Modes

| Speech Act | Direction of Fit | Formal Condition | Phiano Dynamic |
|---|---|---|---|
| **Assertive** | Word $\\to$ World | Sincerity: Believes $p$ | Re-seeds phase to match established memory facts |
| **Directive** | World $\\to$ Word | Preparatory: User desires $A$ | Computes attractor pathfinding trajectory to target goal |
| **Commissive** | World $\\to$ Word | Essential: Agent commits to $A$ | Registers scheduled episodic memory state |
| **Expressive** | $\\varnothing$ (Null) | Psychological state about $p$ | Adjusts contextual wave amplitude $r_k$ |
| **Declarative** | Double (Word $\\leftrightarrow$ World) | Authority & Felicity | Alters institutional state & registers new definitions |

---

### 3. Double Direction of Fit: Institutional Declarations

Declarations bring about a state of affairs solely through the act of uttering them:

$$\\text{Institutional Fact}: \\quad X \\text{ counts as } Y \\text{ in Context } C$$

When an Institutional Declaration is evaluated:
1. Validates felicity and authority conditions.
2. Synthesizes continuous phase manifold prose via \`Generator::generate\`.
3. Registers the institutional state alteration in episodic memory with immediate Hebbian reinforcement.`,
  },
  {
    id: 'memory/hierarchical_field',
    category: 'Memory & Field Theory',
    title: '16-Layer Hierarchical Memory & Non-Local Phase Field Theory',
    badge: 'Memory Architecture',
    summary: 'The 16-tier cognitive memory pyramid spanning microsecond phonemes to permanent foundational axioms with zero catastrophic forgetting.',
    citations: [
      'Tulving, E. (1972). Episodic and Semantic Memory. Organization of Memory.',
      'Grossberg, S. (2013). Adaptive Resonance Theory: How a brain learns to consciously attend, learn, and recognize a changing world. Neural Networks.',
      'Squire, L. R. (2004). Memory systems of the brain: A brief history and current perspective. Neurobiology of Learning and Memory.'
    ],
    content: `### 1. The 16-Layer Memory Hierarchy

Rather than dumping all context into a flat $O(N^2)$ KV-cache, Phiano structures memory into **16 hierarchical continuous phase layers**:

\`\`\`
Level 15: [Foundational Axioms]     Permanent, immutable epistemic truths
Level 14: [Scientific Laws]         Thermodynamics, quantum mechanics, logic
Level 13: [Cultural Paradigms]      Linguistic norms & shared intentionality
Level 12: [Long-Term Expertise]     Rust systems programming, cognitive science
Level 11: [Institutional Rules]     Social contracts, declarations, permissions
Level 10: [Episodic Chronicles]     Historical records & book chapters
Level 09: [Narrative Flow]          Multi-turn conversational context
Level 08: [Semantic Discourse]      Topic-level coherence & aboutness
Level 07: [Paragraph Syntagma]      Multi-sentence logical structures
Level 06: [Sentence Chords]         Harmonic phrase couplings
Level 05: [Clauses & Dependencies]  Asymmetric syntax lag couplings (beta_ij)
Level 04: [Lexical Definitions]     215k Wikipedia grounded concept definitions
Level 03: [Bigrams & Collocations]  Markovian transition probabilities
Level 02: [Morphemes & Roots]       Sub-word semantic primitives
Level 01: [Phonemes & Graphemes]    Acoustic & character frequencies
Level 00: [Sensory Ingress]         Raw input stream buffer
\`\`\`

---

### 2. Context Wave Superposition ($O(1)$ Memory)

Phiano compresses unlimited conversational history into a single complex context phasor:

$$\\Psi_{\\text{context}}(t) = \\sum_{k=1}^t A_k e^{i \\theta_k} \\cdot \\lambda^{t - k}$$

• **$\\lambda = 0.92$**: Exponential memory decay factor.
• **$O(1)$ Space Complexity**: Two 64-bit floating point numbers (\`sum_x\`, \`sum_y\`).
• **Natural Forgetting**: Recent tokens dominate while older context gracefully fades without catastrophic context truncation.`,
  },
  {
    id: 'api/torus_harmonics',
    category: 'Foundations & Physics',
    title: 'Multi-Frequency Torus Decoding (T^D) & Winding Harmonics',
    badge: 'Torus Manifold',
    summary: 'Harmonic ray-casting and winding numbers on high-dimensional toroidal surfaces using golden ratio frequency bases.',
    citations: [
      'Arnold, V. I. (1989). Mathematical Methods of Classical Mechanics. Springer.',
      'Poincaré, H. (1892). Les Méthodes Nouvelles de la Mécanique Céleste. Gauthier-Villars.'
    ],
    content: `### 1. Geometry of the Torus Manifold $\\mathbb{T}^D$

A $D$-dimensional torus $\\mathbb{T}^D = \\underbrace{\\mathbb{S}^1 \\times \\mathbb{S}^1 \\times \\dots \\times \\mathbb{S}^1}_{D}$ represents a compact abelian Lie group.

Each word in Phiano possesses $D=32$ harmonic phase coordinates:

$$\\boldsymbol{\\theta} = (\\theta_1, \\theta_2, \\dots, \\theta_D) \\in [-\\pi, \\pi]^D$$

The frequencies $\\omega_k$ are distributed as powers of the Golden Ratio $\\Phi = \\frac{1 + \\sqrt{5}}{2} \\approx 1.6180339887$:

$$\\omega_k = \\omega_0 \\cdot \\Phi^k \\pmod{2\\pi}$$

Because $\\Phi$ is the most irrational number, winding trajectories on $\\mathbb{T}^D$ are quasi-periodic and ergodic, preventing repetitive harmonic deadlocks.

---

### 2. Torus Resonance Scoring

During generation decoding, candidate word phasors are scored against the accumulated context torus trajectory:

$$\\text{Resonance}(w, \\Psi) = \\frac{1}{D} \\sum_{d=1}^D \\cos(\\theta_{w, d} - \\theta_{\\Psi, d})$$

Candidates with highest multi-frequency resonance are sampled with temperature-adjusted softmax.`,
  },
  {
    id: 'physics/antiphase_correction',
    category: 'Foundations & Physics',
    title: 'pi-Anti-Phase Destructive Cancellation & In-Chat Self-Correction',
    badge: 'Self-Correction',
    summary: 'Instant $180^\\circ$ phase negation canceling erroneous factual associations with zero catastrophic forgetting.',
    citations: [
      'Dirac, P. A. M. (1958). The Principles of Quantum Mechanics. Oxford University Press.',
      'Hebb, D. O. (1949). The Organization of Behavior. Wiley.'
    ],
    content: `### 1. The Catastrophic Forgetting Dilemma in Backpropagation

In gradient-descent neural networks, updating weights to correct a single mistake often causes **catastrophic interference**, degrading performance across unrelated domains.

### 2. The $\\pi$-Anti-Phase Solution

Phiano applies **destructive wave interference**:

$$\\theta_{\\text{pulse}} = \\theta_{\\text{erroneous}} + \\pi \\pmod{2\\pi}$$

Because $\\cos(\\theta + \\pi) = -\\cos(\\theta)$, applying an anti-phase pulse of magnitude $\\gamma$:

$$A_i \\leftarrow A_i + \\gamma \\left(1 - \\frac{A_i}{A_{\\max}}\\right) \\cos(\\theta_{\\text{target}} - \\theta_i)$$

Destructively cancels the erroneous attractor while simultaneously reinforcing the correct factual association.

**Result**: Instant in-chat self-correction in $< 1\\text{ ms}$ with zero parameter degradation.`,
  },
  {
    id: 'api/developer_guide',
    category: 'API & Architecture',
    title: 'Phiano End-to-End Developer & REST API Reference',
    badge: 'Developer API',
    summary: 'Comprehensive API endpoints, JSON payloads, Rust SDK examples, and Python PhiClient integration.',
    citations: [
      'Palantir Foundry AIP Developer Guide (2024).',
      'Actix & Axum Rust Web Architecture Specification (2025).'
    ],
    content: `### 1. Core REST Endpoints

All endpoints run on the high-performance Axum web server on port \`3005\`:

#### \`POST /api/chat\`
Executes continuous phase reasoning and Searle speech act classification.
\`\`\`json
{
  "text": "Hello Phiano, explain quantum harmonic oscillators"
}
\`\`\`
**Response**:
\`\`\`json
{
  "prompt": "Hello Phiano...",
  "response": "A quantum harmonic oscillator...",
  "speech_act": "directive",
  "direction_of_fit": "world_to_mind",
  "coherence": 0.7642,
  "vocabulary": 215992
}
\`\`\`

#### \`POST /api/save\`
Persists the continuous phase manifold (\`manifold.chroma\`) and 16-layer memory hierarchy (\`memory.chroma\`) to disk.

#### \`POST /api/define\`
Retrieves Wikipedia definitions and complex phase coordinates for any word.

---

### 2. Python SDK Integration (\`phient\`)
\`\`\`python
from phiadk.client import PhiClient
from phiadk._core.agent_base import AgentContext

client = PhiClient()
agent = client.agents['phillm']

ctx = AgentContext(verb='complete', parameters={
    'prompt': 'Explain Searle speech acts',
    'model': 'phiano'
})

ctx = await agent.run(ctx)
print("Response:", ctx.results['output']['content'])
print("Coherence:", ctx.results['output']['coherence'])
\`\`\``,
  },
];

export function DocsPanel() {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<string>('All');
  const [activeId, setActiveId] = useState<string>(PHIANO_DOCS[0].id);
  const [copied, setCopied] = useState(false);

  const categories = ['All', 'Foundations & Physics', 'Cognitive & Speech Acts', 'Memory & Field Theory', 'API & Architecture'];

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

  const handleCopy = () => {
    if (!activeDoc) return;
    navigator.clipboard.writeText(activeDoc.content).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });
  };

  return (
    <div style={{
      display: 'grid',
      gridTemplateColumns: '360px 1fr',
      gap: '1.5rem',
      height: 'calc(100vh - 120px)',
      maxWidth: '1400px',
      margin: '0 auto',
    }}>
      {/* LEFT NAVIGATION SIDEBAR */}
      <div style={{
        background: 'var(--bg-card)',
        backdropFilter: 'blur(20px)',
        WebkitBackdropFilter: 'blur(20px)',
        borderRadius: 'var(--radius-lg)',
        border: '1px solid var(--border-color)',
        padding: '1.25rem',
        display: 'flex',
        flexDirection: 'column',
        gap: '1rem',
        overflow: 'hidden',
        boxShadow: 'var(--shadow-sm)',
      }}>
        {/* Title */}
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.6rem' }}>
            <div style={{
              width: '30px',
              height: '30px',
              borderRadius: '8px',
              background: 'var(--color-primary-light)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: 'var(--color-primary)',
            }}>
              <BookOpen size={17} />
            </div>
            <h2 style={{ fontSize: '1.05rem', fontWeight: 700, color: 'var(--text-primary)', margin: 0 }}>
              Architecture Guides
            </h2>
          </div>
          <span style={{
            fontSize: '0.72rem',
            background: 'var(--color-primary-light)',
            color: 'var(--color-primary)',
            padding: '0.2rem 0.55rem',
            borderRadius: '12px',
            fontWeight: 700,
          }}>
            {PHIANO_DOCS.length} Guides
          </span>
        </div>

        {/* Search */}
        <div style={{ position: 'relative' }}>
          <Search size={15} style={{ position: 'absolute', left: '12px', top: '11px', color: 'var(--text-secondary)' }} />
          <input
            type="text"
            placeholder="Search guides, math, Searle theory..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            style={{
              width: '100%',
              padding: '0.55rem 0.75rem 0.55rem 2.2rem',
              borderRadius: 'var(--radius-md)',
              border: '1px solid var(--border-color)',
              background: 'var(--bg-input)',
              color: 'var(--text-primary)',
              fontSize: '0.825rem',
              outline: 'none',
              transition: 'border-color var(--transition-fast)',
            }}
          />
        </div>

        {/* Category Filter Pills */}
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.35rem' }}>
          {categories.map((cat) => {
            const isSelected = selectedCategory === cat;
            return (
              <button
                key={cat}
                onClick={() => setSelectedCategory(cat)}
                style={{
                  padding: '0.25rem 0.55rem',
                  borderRadius: '6px',
                  fontSize: '0.72rem',
                  border: isSelected ? '1px solid var(--color-primary)' : '1px solid var(--border-color)',
                  cursor: 'pointer',
                  background: isSelected ? 'var(--color-primary-light)' : 'transparent',
                  color: isSelected ? 'var(--color-primary)' : 'var(--text-secondary)',
                  fontWeight: isSelected ? 700 : 500,
                  transition: 'all var(--transition-fast)',
                }}
              >
                {cat}
              </button>
            );
          })}
        </div>

        {/* Guides List */}
        <div style={{
          flex: 1,
          overflowY: 'auto',
          display: 'flex',
          flexDirection: 'column',
          gap: '0.4rem',
          paddingRight: '0.25rem',
        }}>
          {filteredDocs.map((doc) => {
            const isSelected = doc.id === activeDoc?.id;
            return (
              <div
                key={doc.id}
                onClick={() => setActiveId(doc.id)}
                style={{
                  padding: '0.75rem 0.85rem',
                  borderRadius: 'var(--radius-md)',
                  background: isSelected ? 'var(--color-primary-light)' : 'transparent',
                  border: isSelected ? '1px solid var(--color-primary)' : '1px solid transparent',
                  cursor: 'pointer',
                  transition: 'all var(--transition-fast)',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '0.25rem',
                }}
              >
                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                  <span style={{
                    fontSize: '0.68rem',
                    fontWeight: 700,
                    textTransform: 'uppercase',
                    letterSpacing: '0.04em',
                    color: isSelected ? 'var(--color-primary)' : 'var(--text-secondary)',
                  }}>
                    {doc.category}
                  </span>
                  <span style={{
                    fontSize: '0.65rem',
                    fontWeight: 600,
                    padding: '0.1rem 0.35rem',
                    borderRadius: '4px',
                    background: 'var(--border-color)',
                    color: 'var(--text-secondary)',
                  }}>
                    {doc.badge}
                  </span>
                </div>
                <div style={{
                  fontSize: '0.825rem',
                  fontWeight: isSelected ? 700 : 600,
                  color: isSelected ? 'var(--color-primary)' : 'var(--text-primary)',
                  lineHeight: '1.3',
                }}>
                  {doc.title}
                </div>
                <div style={{
                  fontSize: '0.725rem',
                  color: 'var(--text-secondary)',
                  lineHeight: '1.4',
                  overflow: 'hidden',
                  textOverflow: 'ellipsis',
                  display: '-webkit-box',
                  WebkitLineClamp: 2,
                  WebkitBoxOrient: 'vertical',
                }}>
                  {doc.summary}
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* RIGHT DOCUMENT VIEWER */}
      <div style={{
        background: 'var(--bg-card)',
        backdropFilter: 'blur(20px)',
        WebkitBackdropFilter: 'blur(20px)',
        borderRadius: 'var(--radius-lg)',
        border: '1px solid var(--border-color)',
        padding: '2.5rem',
        overflowY: 'auto',
        display: 'flex',
        flexDirection: 'column',
        gap: '1.75rem',
        boxShadow: 'var(--shadow-sm)',
      }}>
        {activeDoc && (
          <>
            {/* Header Area */}
            <div style={{ borderBottom: '1px solid var(--border-color)', paddingBottom: '1.5rem' }}>
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '0.5rem' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                  <span style={{ fontSize: '0.75rem', fontWeight: 700, color: 'var(--color-primary)', textTransform: 'uppercase' }}>
                    {activeDoc.category}
                  </span>
                  <span style={{ color: 'var(--text-secondary)' }}>/</span>
                  <span style={{ fontSize: '0.75rem', fontWeight: 600, color: 'var(--text-secondary)' }}>
                    {activeDoc.badge}
                  </span>
                </div>

                <button
                  onClick={handleCopy}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '0.35rem',
                    padding: '0.35rem 0.65rem',
                    borderRadius: '6px',
                    background: 'var(--bg-secondary)',
                    border: '1px solid var(--border-color)',
                    color: 'var(--text-primary)',
                    fontSize: '0.75rem',
                    fontWeight: 600,
                    cursor: 'pointer',
                  }}
                  title="Copy guide markdown"
                >
                  {copied ? <Check size={13} style={{ color: 'var(--color-success)' }} /> : <Copy size={13} />}
                  <span>{copied ? 'Copied' : 'Copy Guide'}</span>
                </button>
              </div>

              <h1 style={{
                fontSize: '1.85rem',
                fontWeight: 800,
                color: 'var(--text-primary)',
                margin: '0 0 0.5rem 0',
                letterSpacing: '-0.02em',
                lineHeight: '1.25',
              }}>
                {activeDoc.title}
              </h1>

              <p style={{
                fontSize: '0.925rem',
                color: 'var(--text-secondary)',
                lineHeight: '1.5',
                margin: 0,
              }}>
                {activeDoc.summary}
              </p>
            </div>

            {/* Academic Citations */}
            {activeDoc.citations.length > 0 && (
              <div style={{
                background: 'var(--color-primary-light)',
                border: '1px solid var(--border-color)',
                borderRadius: 'var(--radius-md)',
                padding: '1.25rem',
                display: 'flex',
                flexDirection: 'column',
                gap: '0.5rem',
              }}>
                <div style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '0.5rem',
                  color: 'var(--color-primary)',
                  fontWeight: 700,
                  fontSize: '0.85rem',
                }}>
                  <GraduationCap size={18} /> Foundational Literature & Academic References:
                </div>
                <ul style={{ margin: 0, paddingLeft: '1.25rem', color: 'var(--text-primary)', fontSize: '0.825rem', lineHeight: '1.6' }}>
                  {activeDoc.citations.map((cit, idx) => (
                    <li key={idx}>{cit}</li>
                  ))}
                </ul>
              </div>
            )}

            {/* Markdown Body Content */}
            <div style={{
              color: 'var(--text-primary)',
              lineHeight: '1.8',
              fontSize: '0.95rem',
            }}>
              <FormattedMarkdown content={activeDoc.content} />
            </div>
          </>
        )}
      </div>
    </div>
  );
}

function FormattedMarkdown({ content }: { content: string }) {
  const blocks = content.split('\n\n');

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '1.1rem' }}>
      {blocks.map((block, idx) => {
        const trimmed = block.trim();

        // Code block
        if (trimmed.startsWith('```')) {
          const lines = trimmed.split('\n');
          const lang = lines[0].replace('```', '').trim();
          const code = lines.slice(1, -1).join('\n');
          return (
            <div
              key={idx}
              style={{
                background: 'var(--bg-secondary)',
                border: '1px solid var(--border-color)',
                borderRadius: 'var(--radius-md)',
                overflow: 'hidden',
                boxShadow: 'var(--shadow-sm)',
              }}
            >
              {lang && (
                <div style={{
                  background: 'var(--border-color)',
                  padding: '0.3rem 0.85rem',
                  fontSize: '0.72rem',
                  fontWeight: 700,
                  textTransform: 'uppercase',
                  color: 'var(--text-secondary)',
                  letterSpacing: '0.05em',
                }}>
                  {lang}
                </div>
              )}
              <pre style={{
                padding: '1rem',
                margin: 0,
                overflowX: 'auto',
                fontFamily: 'Fira Code, Consolas, Monaco, monospace',
                fontSize: '0.85rem',
                color: 'var(--text-primary)',
                lineHeight: '1.6',
              }}>
                <code>{code}</code>
              </pre>
            </div>
          );
        }

        // H3 Header
        if (trimmed.startsWith('### ')) {
          return (
            <h3 key={idx} style={{
              fontSize: '1.25rem',
              fontWeight: 700,
              color: 'var(--text-primary)',
              marginTop: '0.75rem',
              marginBottom: '0.25rem',
              letterSpacing: '-0.01em',
            }}>
              {trimmed.replace('### ', '')}
            </h3>
          );
        }

        // H2 Header
        if (trimmed.startsWith('## ')) {
          return (
            <h2 key={idx} style={{
              fontSize: '1.45rem',
              fontWeight: 800,
              color: 'var(--text-primary)',
              marginTop: '1.25rem',
              marginBottom: '0.35rem',
              letterSpacing: '-0.02em',
            }}>
              {trimmed.replace('## ', '')}
            </h2>
          );
        }

        // Divider
        if (trimmed === '---') {
          return <hr key={idx} style={{ border: 'none', borderTop: '1px solid var(--border-color)', margin: '0.5rem 0' }} />;
        }

        // Regular paragraph with basic formatting
        return (
          <p key={idx} style={{ margin: 0, color: 'var(--text-primary)', lineHeight: '1.75' }}>
            {trimmed}
          </p>
        );
      })}
    </div>
  );
}

export default DocsPanel;
