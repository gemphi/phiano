import React from 'react';
import {
  MessageSquare,
  Search,
  BookOpen,
  Waves,
  Network,
  Sparkles,
  Cpu,
  Gauge,
  BarChart3,
  FileText,
  Swords,
  Sigma,
  Zap,
} from 'lucide-react';
import type { View } from '../App';

export interface NavItem {
  id: View;
  label: string;
  description: string;
  icon: React.ElementType;
  badge?: string;
}

export interface NavCategory {
  title: string;
  items: NavItem[];
}

export const SIDEBAR_CATEGORIES: NavCategory[] = [
  {
    title: 'Cognitive Engine',
    items: [
      { id: 'chat', label: 'Cognitive Chat', description: 'Searle Speech Acts & Reasoning', icon: MessageSquare, badge: 'Live' },
      { id: 'dictionary', label: 'Dictionary & Grounding', description: '215k Word Definitions & Stories', icon: Search },
      { id: 'learn', label: 'Teach & Entrain', description: 'Real-Time Hebbian Learning', icon: BookOpen },
    ],
  },
  {
    title: 'Dynamical Manifolds',
    items: [
      { id: 'oscillator', label: '3D Kuramoto Sphere', description: '16 3D Objects & Phase Ring', icon: Waves, badge: '3D' },
      { id: 'topology', label: 'Phase Flow Topology', description: 'Resonant Nodes & Trajectories', icon: Network },
      { id: 'infinity', label: 'Infinity Resonance', description: 'Harmonic Winding Field', icon: Sparkles },
    ],
  },
  {
    title: 'Studio & Telemetry',
    items: [
      { id: 'studio', label: 'Phi-4 Studio', description: 'Instruction Tuning & Multi-Layers', icon: Cpu },
      { id: 'eval', label: 'Harmonic Evaluation', description: 'Spectral Entropy & Coherence', icon: Gauge },
      { id: 'stats', label: 'Manifold Telemetry', description: 'Memory Layers & Vocabulary', icon: BarChart3 },
    ],
  },
  {
    title: 'Theory & Architecture',
    items: [
      { id: 'docs', label: 'Documentation', description: 'Foundations & API Guides', icon: FileText, badge: 'Docs' },
      { id: 'versus', label: 'Phiano vs PyTorch', description: 'Architectural Comparison', icon: Swords },
      { id: 'symbols', label: 'Math Symbols', description: 'Phase Math Symbol Reference', icon: Sigma },
    ],
  },
];

interface SidebarProps {
  active: View;
  onNavigate: (v: View) => void;
}

export function Sidebar({ active, onNavigate }: SidebarProps) {
  return (
    <aside style={{
      gridArea: 'sidebar',
      background: 'var(--gradient-sidebar)',
      backdropFilter: 'blur(20px)',
      WebkitBackdropFilter: 'blur(20px)',
      borderRight: `1px solid var(--border-color)`,
      display: 'flex',
      flexDirection: 'column',
      padding: '1.25rem 0.85rem',
      gap: '1.25rem',
      zIndex: 11,
      overflowY: 'auto',
      userSelect: 'none',
    }}>
      {/* Brand Header */}
      <header style={{
        display: 'flex',
        alignItems: 'center',
        gap: '0.75rem',
        padding: '0.5rem 0.5rem 0.25rem 0.5rem',
      }}>
        <span style={{
          width: '34px',
          height: '34px',
          borderRadius: '10px',
          background: 'linear-gradient(135deg, var(--color-primary) 0%, #8b5cf6 100%)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          color: '#ffffff',
          boxShadow: '0 4px 12px rgba(99, 102, 241, 0.35)',
        }}>
          <Zap size={18} />
        </span>
        <span style={{ display: 'flex', flexDirection: 'column' }}>
          <span style={{
            fontFamily: 'var(--font-heading)',
            fontSize: '1.2rem',
            fontWeight: 800,
            color: 'var(--text-primary)',
            lineHeight: 1.1,
            letterSpacing: '-0.02em',
          }}>
            Phiano
          </span>
          <span style={{
            fontSize: '0.7rem',
            fontWeight: 500,
            color: 'var(--text-secondary)',
            letterSpacing: '0.02em',
          }}>
            Phase Instrument
          </span>
        </span>
      </header>

      {/* Categorized Navigation */}
      <nav style={{ display: 'flex', flexDirection: 'column', gap: '1.25rem' }}>
        {SIDEBAR_CATEGORIES.map((category) => (
          <section key={category.title} style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
            <span style={{
              fontSize: '0.68rem',
              fontWeight: 700,
              textTransform: 'uppercase',
              letterSpacing: '0.06em',
              color: 'var(--text-secondary)',
              padding: '0 0.5rem 0.35rem 0.5rem',
              opacity: 0.85,
              display: 'block',
            }}>
              {category.title}
            </span>

            <nav style={{ display: 'flex', flexDirection: 'column', gap: '0.15rem' }}>
              {category.items.map(({ id, label, icon: Icon, badge }) => {
                const isActive = active === id;
                return (
                  <button
                    key={id}
                    onClick={() => onNavigate(id)}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      padding: '0.55rem 0.75rem',
                      borderRadius: 'var(--radius-md)',
                      width: '100%',
                      fontSize: '0.825rem',
                      fontWeight: isActive ? 600 : 500,
                      color: isActive ? 'var(--color-primary)' : 'var(--text-primary)',
                      background: isActive ? 'var(--color-primary-light)' : 'transparent',
                      border: isActive ? '1px solid var(--border-color)' : '1px solid transparent',
                      transition: 'all var(--transition-fast)',
                      cursor: 'pointer',
                      textAlign: 'left',
                    }}
                  >
                    <span style={{ display: 'flex', alignItems: 'center', gap: '0.65rem', minWidth: 0 }}>
                      <Icon
                        size={17}
                        style={{
                          flexShrink: 0,
                          color: isActive ? 'var(--color-primary)' : 'var(--text-secondary)',
                        }}
                      />
                      <span style={{
                        overflow: 'hidden',
                        textOverflow: 'ellipsis',
                        whiteSpace: 'nowrap',
                      }}>
                        {label}
                      </span>
                    </span>

                    {badge && (
                      <span style={{
                        fontSize: '0.65rem',
                        fontWeight: 700,
                        padding: '0.1rem 0.4rem',
                        borderRadius: '10px',
                        background: isActive ? 'var(--color-primary)' : 'var(--border-color)',
                        color: isActive ? 'var(--text-inverse)' : 'var(--text-secondary)',
                        flexShrink: 0,
                      }}>
                        {badge}
                      </span>
                    )}
                  </button>
                );
              })}
            </nav>
          </section>
        ))}
      </nav>
    </aside>
  );
}
