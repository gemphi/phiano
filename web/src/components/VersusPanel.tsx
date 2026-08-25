import { useState, useMemo } from 'react';
import {
  ChevronLeft,
  ChevronRight,
  Lightbulb,
  Zap,
  Layers,
} from 'lucide-react';
import { VERSUS_DOCS, type VersusDoc } from '../data/versus_docs';

export function VersusPanel() {
  const [activeIdx, setActiveIdx] = useState(0);
  const [drawerOpen, setDrawerOpen] = useState(true);

  const doc = useMemo(() => VERSUS_DOCS[activeIdx], [activeIdx]);

  return (
    <div style={{
      display: 'flex',
      height: 'calc(100vh - 120px)',
      gap: '0.75rem',
    }}>
      {/* DRAWER — slide in/out tab navigation */}
      {drawerOpen && (
        <div style={{
          width: '240px',
          flexShrink: 0,
          background: 'var(--bg-secondary, #111827)',
          borderRadius: 'var(--radius-lg, 12px)',
          border: '1px solid var(--border-color, #1f2937)',
          display: 'flex',
          flexDirection: 'column',
          overflow: 'hidden',
        }}>
          <DrawerHeader onClose={() => setDrawerOpen(false)} />
          <DrawerTabs
            docs={VERSUS_DOCS}
            activeIdx={activeIdx}
            onSelect={(i) => { setActiveIdx(i); setDrawerOpen(false); }}
          />
        </div>
      )}

      {/* COLLAPSED DRAWER TOGGLE */}
      {!drawerOpen && (
        <button
          onClick={() => setDrawerOpen(true)}
          style={{
            width: '40px',
            flexShrink: 0,
            background: 'var(--bg-secondary, #111827)',
            border: '1px solid var(--border-color, #1f2937)',
            borderRadius: 'var(--radius-lg, 12px)',
            cursor: 'pointer',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            color: 'var(--color-primary, #8b5cf6)',
          }}
        >
          <ChevronRight size={20} />
        </button>
      )}

      {/* MAIN CONTENT — side-by-side panels */}
      <div style={{
        flex: 1,
        display: 'flex',
        flexDirection: 'column',
        gap: '0.75rem',
        overflow: 'hidden',
      }}>
        {/* HEADER */}
        <div style={{
          background: 'var(--bg-secondary, #111827)',
          borderRadius: 'var(--radius-lg, 12px)',
          border: '1px solid var(--border-color, #1f2937)',
          padding: '1.25rem 1.5rem',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
        }}>
          <div>
            <div style={{
              display: 'flex',
              alignItems: 'center',
              gap: '0.5rem',
              marginBottom: '0.3rem',
            }}>
              <Layers size={18} style={{ color: 'var(--color-primary, #8b5cf6)' }} />
              <span style={{
                fontSize: '0.7rem',
                fontWeight: 700,
                color: 'var(--color-primary, #8b5cf6)',
                textTransform: 'uppercase',
                letterSpacing: '0.05em',
              }}>
                Phiano vs PyTorch · Page {activeIdx + 1} of {VERSUS_DOCS.length}
              </span>
            </div>
            <h1 style={{
              fontSize: '1.5rem',
              fontWeight: 800,
              color: 'var(--text-primary, #f9fafb)',
              margin: 0,
              letterSpacing: '-0.02em',
            }}>
              {doc.title}
            </h1>
          </div>
          <NavButtons
            activeIdx={activeIdx}
            total={VERSUS_DOCS.length}
            onPrev={() => setActiveIdx(Math.max(0, activeIdx - 1))}
            onNext={() => setActiveIdx(Math.min(VERSUS_DOCS.length - 1, activeIdx + 1))}
          />
        </div>

        {/* SIDE-BY-SIDE PANELS */}
        <div style={{
          display: 'grid',
          gridTemplateColumns: '1fr 1fr',
          gap: '0.75rem',
          flex: 1,
          overflow: 'hidden',
        }}>
          <SidePanel
            label={doc.phianoSide.label}
            points={doc.phianoSide.points}
            code={doc.phianoSide.code}
            accent="#8b5cf6"
            bgAccent="rgba(139, 92, 246, 0.08)"
            badge="PHIANO"
          />
          <SidePanel
            label={doc.pytorchSide.label}
            points={doc.pytorchSide.points}
            code={doc.pytorchSide.code}
            accent="#3b82f6"
            bgAccent="rgba(59, 130, 246, 0.08)"
            badge="PYTORCH"
          />
        </div>

        {/* KEY INSIGHT CALLOUT */}
        <div style={{
          background: 'rgba(139, 92, 246, 0.12)',
          border: '1px solid rgba(139, 92, 246, 0.35)',
          borderRadius: 'var(--radius-lg, 12px)',
          padding: '1rem 1.5rem',
          display: 'flex',
          alignItems: 'flex-start',
          gap: '0.75rem',
        }}>
          <Lightbulb size={20} style={{ color: '#c4b5fd', flexShrink: 0, marginTop: '2px' }} />
          <div>
            <div style={{
              fontSize: '0.7rem',
              fontWeight: 700,
              color: '#c4b5fd',
              textTransform: 'uppercase',
              letterSpacing: '0.05em',
              marginBottom: '0.3rem',
            }}>
              Key Insight
            </div>
            <p style={{
              margin: 0,
              fontSize: '0.9rem',
              lineHeight: 1.6,
              color: 'var(--text-primary, #e5e7eb)',
              fontStyle: 'italic',
            }}>
              {doc.insight}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

function DrawerHeader({ onClose }: { onClose: () => void }) {
  return (
    <div style={{
      padding: '1rem 1.25rem',
      borderBottom: '1px solid var(--border-color, #1f2937)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'space-between',
    }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
        <Zap size={18} style={{ color: '#8b5cf6' }} />
        <span style={{
          fontSize: '0.85rem',
          fontWeight: 700,
          color: 'var(--text-primary, #f9fafb)',
        }}>
          Versus Index
        </span>
      </div>
      <button
        onClick={onClose}
        style={{
          background: 'transparent',
          border: 'none',
          cursor: 'pointer',
          color: 'var(--text-secondary, #9ca3af)',
          padding: '0.25rem',
        }}
      >
        <ChevronLeft size={18} />
      </button>
    </div>
  );
}

function DrawerTabs({
  docs,
  activeIdx,
  onSelect,
}: {
  docs: VersusDoc[];
  activeIdx: number;
  onSelect: (i: number) => void;
}) {
  return (
    <div style={{
      flex: 1,
      overflowY: 'auto',
      padding: '0.5rem',
      display: 'flex',
      flexDirection: 'column',
      gap: '0.15rem',
    }}>
      {docs.map((d, i) => {
        const isActive = i === activeIdx;
        return (
          <button
            key={d.id}
            onClick={() => onSelect(i)}
            style={{
              padding: '0.6rem 0.75rem',
              borderRadius: 'var(--radius-md, 8px)',
              background: isActive ? 'rgba(139, 92, 246, 0.18)' : 'transparent',
              border: 'none',
              borderLeft: isActive ? '3px solid #8b5cf6' : '3px solid transparent',
              cursor: 'pointer',
              textAlign: 'left',
              transition: 'all 0.15s ease',
              display: 'flex',
              flexDirection: 'column',
              gap: '0.15rem',
            }}
          >
            <div style={{
              fontSize: '0.7rem',
              fontWeight: 600,
              color: isActive ? '#a78bfa' : 'var(--text-secondary, #6b7280)',
            }}>
              {String(i + 1).padStart(2, '0')} · {d.tab}
            </div>
            <div style={{
              fontSize: '0.78rem',
              fontWeight: isActive ? 600 : 400,
              color: isActive ? '#c4b5fd' : 'var(--text-secondary, #9ca3af)',
              lineHeight: 1.3,
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap',
            }}>
              {d.title}
            </div>
          </button>
        );
      })}
    </div>
  );
}

function NavButtons({
  activeIdx,
  total,
  onPrev,
  onNext,
}: {
  activeIdx: number;
  total: number;
  onPrev: () => void;
  onNext: () => void;
}) {
  return (
    <div style={{ display: 'flex', gap: '0.4rem' }}>
      <button
        onClick={onPrev}
        disabled={activeIdx === 0}
        style={{
          padding: '0.4rem 0.6rem',
          borderRadius: 'var(--radius-md, 8px)',
          border: '1px solid var(--border-color, #374151)',
          background: 'transparent',
          cursor: activeIdx === 0 ? 'not-allowed' : 'pointer',
          color: activeIdx === 0 ? 'var(--text-secondary, #4b5563)' : 'var(--text-primary, #e5e7eb)',
          opacity: activeIdx === 0 ? 0.4 : 1,
          display: 'flex',
          alignItems: 'center',
          gap: '0.3rem',
          fontSize: '0.8rem',
        }}
      >
        <ChevronLeft size={16} /> Prev
      </button>
      <button
        onClick={onNext}
        disabled={activeIdx === total - 1}
        style={{
          padding: '0.4rem 0.6rem',
          borderRadius: 'var(--radius-md, 8px)',
          border: '1px solid var(--border-color, #374151)',
          background: 'transparent',
          cursor: activeIdx === total - 1 ? 'not-allowed' : 'pointer',
          color: activeIdx === total - 1 ? 'var(--text-secondary, #4b5563)' : 'var(--text-primary, #e5e7eb)',
          opacity: activeIdx === total - 1 ? 0.4 : 1,
          display: 'flex',
          alignItems: 'center',
          gap: '0.3rem',
          fontSize: '0.8rem',
        }}
      >
        Next <ChevronRight size={16} />
      </button>
    </div>
  );
}

function SidePanel({
  label,
  points,
  code,
  accent,
  bgAccent,
  badge,
}: {
  label: string;
  points: string[];
  code?: string;
  accent: string;
  bgAccent: string;
  badge: string;
}) {
  return (
    <div style={{
      background: 'var(--bg-secondary, #111827)',
      borderRadius: 'var(--radius-lg, 12px)',
      border: `1px solid ${accent}33`,
      padding: '1.25rem',
      overflowY: 'auto',
      display: 'flex',
      flexDirection: 'column',
      gap: '0.85rem',
    }}>
      {/* BADGE + LABEL */}
      <div style={{
        display: 'flex',
        alignItems: 'center',
        gap: '0.5rem',
        paddingBottom: '0.75rem',
        borderBottom: `1px solid ${accent}22`,
      }}>
        <span style={{
          fontSize: '0.65rem',
          fontWeight: 800,
          color: accent,
          background: bgAccent,
          padding: '0.2rem 0.5rem',
          borderRadius: '6px',
          letterSpacing: '0.05em',
        }}>
          {badge}
        </span>
        <span style={{
          fontSize: '0.95rem',
          fontWeight: 700,
          color: 'var(--text-primary, #f9fafb)',
        }}>
          {label}
        </span>
      </div>

      {/* POINTS */}
      <ul style={{
        margin: 0,
        paddingLeft: '1.1rem',
        display: 'flex',
        flexDirection: 'column',
        gap: '0.4rem',
      }}>
        {points.map((p, i) => (
          <li key={i} style={{
            fontSize: '0.85rem',
            lineHeight: 1.5,
            color: 'var(--text-primary, #d1d5db)',
          }}>
            {p}
          </li>
        ))}
      </ul>

      {/* CODE BLOCK */}
      {code && (
        <pre style={{
          background: 'var(--bg-input)',
          borderRadius: 'var(--radius-md, 8px)',
          padding: '0.85rem',
          overflow: 'auto',
          fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
          fontSize: '0.78rem',
          lineHeight: 1.5,
          color: accent,
          border: `1px solid ${accent}33`,
          margin: 0,
        }}>
          {code}
        </pre>
      )}
    </div>
  );
}
