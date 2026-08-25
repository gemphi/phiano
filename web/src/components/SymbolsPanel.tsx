import { useState } from 'react';
import { Sigma, ChevronRight } from 'lucide-react';

interface GreekSymbol {
  name: string;
  char: string;
  unicode: string;
  category: 'Phase Core' | 'Coupling' | 'Learning' | 'Topology';
  meaning: string;
  phianoUse: string;
  formula: string;
  formulaDesc: string;
  docRef: string;
  color: string;
}

const SYMBOLS: GreekSymbol[] = [
  {
    name: 'Theta',
    char: 'θ',
    unicode: 'U+03B8',
    category: 'Phase Core',
    meaning: 'Phase angle — the angular position of a word on the unit circle [0, 2π)',
    phianoUse: 'Every word in the Facet has a θ that IS its position. No external positional encoding needed. θ wraps at 2π, creating natural recursion.',
    formula: 'dθᵢ/dt = ωᵢ + (K/N) Σⱼ sin(θⱼ − θᵢ + βᵢⱼ)',
    formulaDesc: 'Kuramoto phase evolution — each word\'s phase drifts toward synchronization with its neighbors',
    docRef: 'Page 1: The Transformer Problem',
    color: '#a78bfa',
  },
  {
    name: 'Beta',
    char: 'β',
    unicode: 'U+03B2',
    category: 'Coupling',
    meaning: 'Directional syntax lag — the learned phase offset from word i to word j',
    phianoUse: 'βᵢⱼ is asymmetric: β(dog→bites) ≠ β(bites→dog). Learned per word pair via EMA. This is Phiano\'s replacement for RoPE positional encoding.',
    formula: 'βᵢⱼ = (1−η)·βᵢⱼ + η·(θⱼ − θᵢ) mod 2π',
    formulaDesc: 'Exponential moving average of observed phase differences — learns directional syntax',
    docRef: 'Page 4: Asymmetric Syntax Coupling',
    color: '#8b5cf6',
  },
  {
    name: 'Omega',
    char: 'ω',
    unicode: 'U+03C9',
    category: 'Phase Core',
    meaning: 'Angular frequency — the natural oscillation speed of each word\'s phasor',
    phianoUse: 'Each word has its own ωᵢ, determined by its frequency band n and the fine-structure constant α. Words with similar ω synchronize faster.',
    formula: 'ωᵢ = 2π · fᵢ where fᵢ = nᵢ · α',
    formulaDesc: 'Natural frequency from harmonic band level and fine-structure constant',
    docRef: 'Page 1: The Transformer Problem',
    color: '#6366f1',
  },
  {
    name: 'Alpha',
    char: 'α',
    unicode: 'U+03B1',
    category: 'Topology',
    meaning: 'Fine-structure constant — the spectral spacing between harmonic frequency bands',
    phianoUse: 'The effective phase of a word is θ + n·α. This creates a multi-dimensional spectral space where band_n shifts the phase by α per level.',
    formula: 'Z = A · e^(i·(θ + n·α))',
    formulaDesc: 'Complex wave representation — band level n shifts the phase by α',
    docRef: 'Page 2: Phase Manifold (C³² Torus)',
    color: '#3b82f6',
  },
  {
    name: 'Pi',
    char: 'π',
    unicode: 'U+03C0',
    category: 'Phase Core',
    meaning: 'The half-circle — anti-phase, phase wrapping, and periodicity',
    phianoUse: 'Phase wraps at 2π (recursion). Anti-phase correction pushes wrong words π radians away from correct words. Destructive interference at π.',
    formula: 'θ_correct = θ_wrong + π (mod 2π)',
    formulaDesc: 'Anti-phase pulse — the wrong concept is pushed to maximum destructive interference',
    docRef: 'Page 7: In-Chat Self-Correction',
    color: '#ef4444',
  },
  {
    name: 'Phi',
    char: 'φ',
    unicode: 'U+03C6',
    category: 'Topology',
    meaning: 'Golden ratio (1.618...) — the most irrational number, used for uniform phase seeding',
    phianoUse: 'Words are seeded at θ = (len · φ) mod 2π, producing the sunflower spiral — the most uniform distribution on the circle. Also used for multi-frequency harmonic spacing.',
    formula: 'θ_seed = (|word| · φ) mod 2π  |  φ = (1+√5)/2',
    formulaDesc: 'Golden angle seeding — maximum initial separation between words',
    docRef: 'Page 2: Phase Manifold (C³² Torus)',
    color: '#fbbf24',
  },
  {
    name: 'Sigma',
    char: 'Σ',
    unicode: 'U+03A3',
    category: 'Coupling',
    meaning: 'Summation — aggregation of phase contributions across all coupled neighbors',
    phianoUse: 'The Kuramoto coupling sums sin(θⱼ − θᵢ + βᵢⱼ) over all neighbors j. The 16 cognitive agents also Σ their phase signals into a collective phase.',
    formula: 'R = |(1/N) Σⱼ e^(iθⱼ)|',
    formulaDesc: 'Order parameter R — measures global phase synchronization across all words',
    docRef: 'Page 6: Hebbian Wave Plasticity',
    color: '#10b981',
  },
  {
    name: 'Delta',
    char: 'Δ',
    unicode: 'U+0394',
    category: 'Learning',
    meaning: 'Phase difference — the local signal that drives Hebbian plasticity',
    phianoUse: 'Instead of global loss gradients, Phiano uses Δθ = sin(θ_target − θ_current) as a local, per-word learning signal. No backpropagation needed.',
    formula: 'Δθ = sin(θ_target − θ_current)  |  θ += η · Δθ',
    formulaDesc: 'Hebbian phase update — each word drifts toward its context, locally',
    docRef: 'Page 6: Hebbian Wave Plasticity',
    color: '#06b6d4',
  },
];

const CATEGORIES = ['Phase Core', 'Coupling', 'Learning', 'Topology'] as const;

export function SymbolsPanel() {
  const [selected, setSelected] = useState<GreekSymbol | null>(SYMBOLS[0]);
  const [filter, setFilter] = useState<string | null>(null);

  const filtered = filter ? SYMBOLS.filter(s => s.category === filter) : SYMBOLS;

  return (
    <div style={{
      display: 'flex',
      flexDirection: 'column',
      gap: '1rem',
      height: 'calc(100vh - 120px)',
    }}>
      {/* HEADER */}
      <div style={{
        background: 'var(--card-bg, #111827)',
        borderRadius: '12px',
        border: '1px solid var(--border-color, #1f2937)',
        padding: '1.25rem 1.5rem',
        display: 'flex',
        alignItems: 'center',
        gap: '0.75rem',
      }}>
        <Sigma size={24} style={{ color: '#a78bfa' }} />
        <div>
          <h1 style={{
            fontSize: '1.4rem',
            fontWeight: 800,
            color: '#f9fafb',
            margin: 0,
            letterSpacing: '-0.02em',
          }}>
            Mathematical Symbols
          </h1>
          <p style={{
            fontSize: '0.8rem',
            color: '#9ca3af',
            margin: '0.2rem 0 0 0',
          }}>
            Greek letters from the Phiano vs Transformer docs — rendered in SVG
          </p>
        </div>
      </div>

      {/* MAIN LAYOUT */}
      <div style={{
        flex: 1,
        display: 'grid',
        gridTemplateColumns: '340px 1fr',
        gap: '1rem',
        overflow: 'hidden',
      }}>
        {/* SYMBOL GRID */}
        <div style={{
          background: 'var(--card-bg, #111827)',
          borderRadius: '12px',
          border: '1px solid var(--border-color, #1f2937)',
          padding: '1rem',
          overflowY: 'auto',
          display: 'flex',
          flexDirection: 'column',
          gap: '0.75rem',
        }}>
          {/* CATEGORY FILTERS */}
          <div style={{
            display: 'flex',
            flexWrap: 'wrap',
            gap: '0.3rem',
          }}>
            <FilterChip label="All" active={filter === null} onClick={() => setFilter(null)} />
            {CATEGORIES.map(cat => (
              <FilterChip key={cat} label={cat} active={filter === cat} onClick={() => setFilter(cat)} />
            ))}
          </div>

          {/* SYMBOL CARDS */}
          <div style={{
            display: 'grid',
            gridTemplateColumns: '1fr 1fr',
            gap: '0.5rem',
          }}>
            {filtered.map(sym => (
              <SymbolCard
                key={sym.name}
                sym={sym}
                isSelected={selected?.name === sym.name}
                onClick={() => setSelected(sym)}
              />
            ))}
          </div>
        </div>

        {/* DETAIL VIEW */}
        {selected && <SymbolDetail sym={selected} />}
      </div>
    </div>
  );
}

function FilterChip({ label, active, onClick }: { label: string; active: boolean; onClick: () => void }) {
  return (
    <button
      onClick={onClick}
      style={{
        padding: '0.25rem 0.6rem',
        borderRadius: '6px',
        border: '1px solid',
        borderColor: active ? '#8b5cf6' : '#374151',
        background: active ? 'rgba(139, 92, 246, 0.18)' : 'transparent',
        color: active ? '#c4b5fd' : '#9ca3af',
        fontSize: '0.72rem',
        fontWeight: 600,
        cursor: 'pointer',
        transition: 'all 0.15s ease',
      }}
    >
      {label}
    </button>
  );
}

function SymbolCard({ sym, isSelected, onClick }: { sym: GreekSymbol; isSelected: boolean; onClick: () => void }) {
  return (
    <button
      onClick={onClick}
      style={{
        background: isSelected ? 'rgba(139, 92, 246, 0.12)' : 'rgba(0,0,0,0.2)',
        border: `1px solid ${isSelected ? sym.color + '66' : '#1f2937'}`,
        borderRadius: '10px',
        padding: '0.75rem 0.5rem',
        cursor: 'pointer',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        gap: '0.3rem',
        transition: 'all 0.15s ease',
      }}
    >
      <SymbolSVG char={sym.char} color={sym.color} size={48} />
      <span style={{
        fontSize: '0.72rem',
        fontWeight: 600,
        color: isSelected ? sym.color : '#9ca3af',
      }}>
        {sym.name}
      </span>
    </button>
  );
}

function SymbolDetail({ sym }: { sym: GreekSymbol }) {
  return (
    <div style={{
      background: 'var(--card-bg, #111827)',
      borderRadius: '12px',
      border: `1px solid ${sym.color}33`,
      padding: '2rem',
      overflowY: 'auto',
      display: 'flex',
      flexDirection: 'column',
      gap: '1.5rem',
    }}>
      {/* BIG SVG SYMBOL */}
      <div style={{
        display: 'flex',
        alignItems: 'center',
        gap: '2rem',
        paddingBottom: '1.5rem',
        borderBottom: `1px solid ${sym.color}22`,
      }}>
        <SymbolSVG char={sym.char} color={sym.color} size={120} withCircle />
        <div>
          <div style={{
            display: 'flex',
            alignItems: 'center',
            gap: '0.5rem',
            marginBottom: '0.3rem',
          }}>
            <span style={{
              fontSize: '0.7rem',
              fontWeight: 700,
              color: sym.color,
              background: sym.color + '18',
              padding: '0.2rem 0.5rem',
              borderRadius: '6px',
              letterSpacing: '0.05em',
            }}>
              {sym.category.toUpperCase()}
            </span>
            <span style={{
              fontSize: '0.7rem',
              color: '#6b7280',
              fontFamily: 'monospace',
            }}>
              {sym.unicode}
            </span>
          </div>
          <h2 style={{
            fontSize: '2rem',
            fontWeight: 800,
            color: '#f9fafb',
            margin: 0,
            letterSpacing: '-0.02em',
          }}>
            {sym.name}
          </h2>
          <p style={{
            fontSize: '0.9rem',
            color: '#9ca3af',
            margin: '0.4rem 0 0 0',
            lineHeight: 1.5,
          }}>
            {sym.meaning}
          </p>
        </div>
      </div>

      {/* FORMULA */}
      <div>
        <div style={{
          fontSize: '0.7rem',
          fontWeight: 700,
          color: sym.color,
          textTransform: 'uppercase',
          letterSpacing: '0.05em',
          marginBottom: '0.6rem',
        }}>
          Mathematical Formula
        </div>
        <div style={{
          background: 'rgba(0,0,0,0.3)',
          borderRadius: '10px',
          border: `1px solid ${sym.color}22`,
          padding: '1.25rem 1.5rem',
          display: 'flex',
          flexDirection: 'column',
          gap: '0.5rem',
        }}>
          <div style={{
            fontFamily: "'Latin Modern Math', 'STIX Two Math', 'Cambria Math', serif",
            fontSize: '1.4rem',
            color: sym.color,
            textAlign: 'center',
            lineHeight: 1.8,
          }}>
            {sym.formula}
          </div>
          <div style={{
            fontSize: '0.8rem',
            color: '#9ca3af',
            textAlign: 'center',
            fontStyle: 'italic',
          }}>
            {sym.formulaDesc}
          </div>
        </div>
      </div>

      {/* PHIANO USAGE */}
      <div>
        <div style={{
          fontSize: '0.7rem',
          fontWeight: 700,
          color: sym.color,
          textTransform: 'uppercase',
          letterSpacing: '0.05em',
          marginBottom: '0.6rem',
        }}>
          How Phiano Uses It
        </div>
        <p style={{
          fontSize: '0.9rem',
          color: '#d1d5db',
          lineHeight: 1.7,
          margin: 0,
        }}>
          {sym.phianoUse}
        </p>
      </div>

      {/* DOC REFERENCE */}
      <div style={{
        display: 'flex',
        alignItems: 'center',
        gap: '0.5rem',
        paddingTop: '1rem',
        borderTop: '1px solid #1f2937',
      }}>
        <ChevronRight size={14} style={{ color: '#6b7280' }} />
        <span style={{
          fontSize: '0.78rem',
          color: '#6b7280',
        }}>
          Referenced in: <span style={{ color: sym.color, fontWeight: 600 }}>{sym.docRef}</span>
        </span>
      </div>
    </div>
  );
}

function SymbolSVG({
  char,
  color,
  size = 64,
  withCircle = false,
}: {
  char: string;
  color: string;
  size?: number;
  withCircle?: boolean;
}) {
  const id = `grad-${char.charCodeAt(0)}`;
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 100 100"
      style={{ display: 'block' }}
    >
      <defs>
        <radialGradient id={id} cx="35%" cy="30%" r="80%">
          <stop offset="0%" stopColor={color} stopOpacity={0.9} />
          <stop offset="100%" stopColor={color} stopOpacity={0.3} />
        </radialGradient>
        <filter id={`glow-${id}`}>
          <feGaussianBlur stdDeviation="2" result="blur" />
          <feMerge>
            <feMergeNode in="blur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>

      {withCircle && (
        <>
          <circle cx="50" cy="50" r="46" fill="none" stroke={color} strokeWidth="1" strokeOpacity="0.3" strokeDasharray="3 3" />
          <circle cx="50" cy="50" r="38" fill={`url(#${id})`} fillOpacity="0.08" />
        </>
      )}

      <text
        x="50"
        y="50"
        textAnchor="middle"
        dominantBaseline="central"
        fill={`url(#${id})`}
        fontSize={withCircle ? "52" : "44"}
        fontFamily="'Latin Modern Math', 'STIX Two Math', 'Cambria Math', 'Times New Roman', serif"
        fontStyle="italic"
        fontWeight="700"
        filter={`url(#glow-${id})`}
      >
        {char}
      </text>
    </svg>
  );
}
