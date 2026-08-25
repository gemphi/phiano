import { useState } from 'react';
import {
  Sun,
  Moon,
  Database,
  Brain,
  Save,
  Check,
  Palette,
  Menu as MenuIcon,
  FileText,
  MessageSquare,
  Waves,
  Cpu,
  Swords,
  ChevronDown,
} from 'lucide-react';
import { Badge, Button, usePuiTheme } from '@phiace/puijs';
import type { Stats } from '../types';
import type { View } from '../App';
import { saveManifold } from '../hooks/api/manifold';

interface HeaderProps {
  dark: boolean;
  toggleDark: () => void;
  stats: Stats;
  loading: boolean;
  onNavigate?: (v: View) => void;
}

export function Header({ dark, toggleDark, stats, loading, onNavigate }: HeaderProps) {
  const { setTheme, brandId, setBrandId, themeStyle, setThemeStyle, brands } = usePuiTheme();
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [showBrandMenu, setShowBrandMenu] = useState(false);
  const [showNavMenu, setShowNavMenu] = useState(false);

  const handleSave = async () => {
    if (saving) return;
    setSaving(true);
    try {
      await saveManifold();
      setSaved(true);
      setTimeout(() => setSaved(false), 2500);
    } catch {
      // ignore
    }
    setSaving(false);
  };

  const currentBrand = brands.find((b) => b.id === brandId) || brands[0];

  const quickNavItems: { id: View; label: string; desc: string; icon: any }[] = [
    { id: 'chat', label: 'Cognitive Chat', desc: 'Searle Speech Acts & Multi-Step Reasoning', icon: MessageSquare },
    { id: 'oscillator', label: '3D Kuramoto Sphere', desc: '16 3D Topological Objects & Phase Ring', icon: Waves },
    { id: 'studio', label: 'Phi-4 Studio', desc: 'Instruction Tuning & Multi-Layers', icon: Cpu },
    { id: 'docs', label: 'Documentation', desc: 'Architecture, Math & API References', icon: FileText },
    { id: 'versus', label: 'Phiano vs PyTorch', desc: 'Transformer Benchmark Comparison', icon: Swords },
  ];

  return (
    <header style={{
      gridArea: 'header',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'space-between',
      padding: '0 1.5rem',
      borderBottom: '1px solid var(--border-color)',
      background: 'var(--gradient-header)',
      backdropFilter: 'blur(20px)',
      WebkitBackdropFilter: 'blur(20px)',
      zIndex: 10,
    }}>
      {/* Left: Platform Navigation Menu & Live Metrics */}
      <nav style={{ display: 'flex', alignItems: 'center', gap: '0.85rem' }}>
        {/* Platform Quick Menu Dropdown */}
        <section style={{ position: 'relative' }}>
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setShowNavMenu((o) => !o)}
            title="Open Platform Navigation Menu"
            style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', padding: '0.4rem 0.75rem' }}
          >
            <MenuIcon size={17} />
            <span style={{ fontSize: '0.825rem', fontWeight: 600 }}>Menu</span>
            <ChevronDown size={16} style={{ opacity: 0.7, marginLeft: '0.4rem' }} />
          </Button>

          {showNavMenu && (
            <section
              style={{
                position: 'absolute',
                top: '130%',
                left: 0,
                minWidth: '300px',
                width: 'max-content',
                maxWidth: '380px',
                background: 'var(--gradient-dropdown)',
                backdropFilter: 'blur(24px)',
                WebkitBackdropFilter: 'blur(24px)',
                border: '1px solid var(--border-color)',
                borderRadius: '12px',
                padding: '0.75rem',
                boxShadow: 'var(--shadow-lg)',
                zIndex: 100,
                display: 'flex',
                flexDirection: 'column',
                gap: '0.3rem',
              }}
              onMouseLeave={() => setShowNavMenu(false)}
            >
              <span style={{
                fontSize: '0.68rem',
                fontWeight: 700,
                color: 'var(--text-secondary)',
                padding: '0.25rem 0.5rem',
                textTransform: 'uppercase',
                letterSpacing: '0.05em',
                display: 'block',
              }}>
                Platform Workspaces
              </span>

              {quickNavItems.map((item) => {
                const Icon = item.icon;
                return (
                  <button
                    key={item.id}
                    onClick={() => {
                      onNavigate?.(item.id);
                      setShowNavMenu(false);
                    }}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      gap: '0.75rem',
                      padding: '0.55rem 0.7rem',
                      borderRadius: '8px',
                      background: 'transparent',
                      border: 'none',
                      cursor: 'pointer',
                      textAlign: 'left',
                      transition: 'all var(--transition-fast)',
                    }}
                    onMouseEnter={(e) => {
                      e.currentTarget.style.background = 'var(--color-primary-light)';
                    }}
                    onMouseLeave={(e) => {
                      e.currentTarget.style.background = 'transparent';
                    }}
                  >
                    <span style={{
                      width: '30px',
                      height: '30px',
                      borderRadius: '7px',
                      background: 'var(--color-primary-light)',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      color: 'var(--color-primary)',
                      flexShrink: 0,
                    }}>
                      <Icon size={16} />
                    </span>
                    <span style={{ minWidth: 0, display: 'flex', flexDirection: 'column' }}>
                      <span style={{ fontSize: '0.825rem', fontWeight: 600, color: 'var(--text-primary)' }}>
                        {item.label}
                      </span>
                      <span style={{ fontSize: '0.7rem', color: 'var(--text-secondary)', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                        {item.desc}
                      </span>
                    </span>
                  </button>
                );
              })}
            </section>
          )}
        </section>

        {/* Live Engine Indicator */}
        <section style={{
          display: 'inline-flex',
          alignItems: 'center',
          gap: '0.5rem',
          padding: '0.3rem 0.65rem',
          borderRadius: '20px',
          background: 'rgba(16, 185, 129, 0.1)',
          border: '1px solid rgba(16, 185, 129, 0.25)',
          fontSize: '0.72rem',
          fontWeight: 700,
          color: 'var(--color-success)',
        }}>
          <span style={{
            width: '7px',
            height: '7px',
            borderRadius: '50%',
            background: 'var(--color-success)',
            boxShadow: '0 0 8px var(--color-success)',
            display: 'inline-block',
          }} />
          <span>PHASE MANIFOLD LIVE</span>
        </section>

        {/* Stats Badges */}
        <Badge variant="secondary" style={{ display: 'inline-flex', alignItems: 'center', gap: '0.35rem', padding: '0.3rem 0.65rem' }}>
          <Database size={13} />
          <span>{stats.vocabulary.toLocaleString()} words</span>
        </Badge>
        <Badge variant="secondary" style={{ display: 'inline-flex', alignItems: 'center', gap: '0.35rem', padding: '0.3rem 0.65rem' }}>
          <Brain size={13} />
          <span>{stats.memory_entries.toLocaleString()} memories</span>
        </Badge>
      </nav>

      {/* Right: Palantir Theme Switcher, Checkpoints & Theme Mode */}
      <nav style={{ display: 'flex', alignItems: 'center', gap: '0.625rem', position: 'relative' }}>
        {/* Brand Switcher Button & Dropdown */}
        <section style={{ position: 'relative' }}>
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setShowBrandMenu((o) => !o)}
            title="Switch Palantir Brand Theme"
            style={{ display: 'flex', alignItems: 'center', gap: '0.45rem', padding: '0.4rem 0.75rem' }}
          >
            <Palette size={15} style={{ color: currentBrand.colors.primary }} />
            <span style={{ fontSize: '0.78rem', fontWeight: 500 }}>{currentBrand.name}</span>
            <ChevronDown size={16} style={{ opacity: 0.7, marginLeft: '0.4rem' }} />
          </Button>

          {showBrandMenu && (
            <section
              style={{
                position: 'absolute',
                top: '130%',
                right: 0,
                minWidth: '240px',
                width: 'max-content',
                maxWidth: '320px',
                background: 'var(--gradient-dropdown)',
                backdropFilter: 'blur(24px)',
                WebkitBackdropFilter: 'blur(24px)',
                border: '1px solid var(--border-color)',
                borderRadius: '12px',
                padding: '0.65rem',
                boxShadow: 'var(--shadow-lg)',
                zIndex: 100,
                display: 'flex',
                flexDirection: 'column',
                gap: '0.25rem',
              }}
              onMouseLeave={() => setShowBrandMenu(false)}
            >
              <span style={{
                fontSize: '0.68rem',
                fontWeight: 700,
                color: 'var(--text-secondary)',
                padding: '0.25rem 0.5rem',
                textTransform: 'uppercase',
                letterSpacing: '0.05em',
                display: 'block',
              }}>
                PALANTIR THEMES
              </span>
              {brands.map((b) => (
                <button
                  key={b.id}
                  onClick={() => {
                    setBrandId(b.id);
                    setShowBrandMenu(false);
                  }}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    padding: '0.45rem 0.6rem',
                    borderRadius: '6px',
                    fontSize: '0.78rem',
                    color: brandId === b.id ? 'var(--color-primary)' : 'var(--text-primary)',
                    background: brandId === b.id ? 'var(--color-primary-light)' : 'transparent',
                    border: 'none',
                    cursor: 'pointer',
                    width: '100%',
                    textAlign: 'left',
                    transition: 'all var(--transition-fast)',
                  }}
                >
                  <span style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                    <span style={{
                      width: '10px',
                      height: '10px',
                      borderRadius: '50%',
                      background: b.colors.primary,
                      display: 'inline-block',
                    }} />
                    <span style={{ fontWeight: brandId === b.id ? 600 : 400 }}>{b.name}</span>
                  </span>
                  {brandId === b.id && <Check size={14} style={{ color: 'var(--color-primary)' }} />}
                </button>
              ))}

              <section style={{ borderTop: '1px solid var(--border-color)', marginTop: '0.35rem', paddingTop: '0.35rem' }}>
                <span style={{
                  fontSize: '0.68rem',
                  fontWeight: 700,
                  color: 'var(--text-secondary)',
                  padding: '0.25rem 0.5rem',
                  textTransform: 'uppercase',
                  letterSpacing: '0.05em',
                  display: 'block',
                }}>
                  THEME STYLE
                </span>
                <nav style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '0.25rem', padding: '0 0.25rem' }}>
                  {(['glass', 'elevated', 'gradient', 'flat'] as const).map((st) => (
                    <button
                      key={st}
                      onClick={() => {
                        setThemeStyle(st);
                        setShowBrandMenu(false);
                      }}
                      style={{
                        padding: '0.3rem 0.4rem',
                        fontSize: '0.72rem',
                        borderRadius: '6px',
                        textTransform: 'capitalize',
                        background: themeStyle === st ? 'var(--color-primary-light)' : 'transparent',
                        color: themeStyle === st ? 'var(--color-primary)' : 'var(--text-secondary)',
                        border: '1px solid var(--border-color)',
                        cursor: 'pointer',
                        fontWeight: themeStyle === st ? 600 : 400,
                      }}
                    >
                      {st}
                    </button>
                  ))}
                </nav>
              </section>
            </section>
          )}
        </section>

        {/* 1-Click Save Checkpoint */}
        <Button
          variant={saved ? 'secondary' : 'ghost'}
          size="sm"
          onClick={handleSave}
          disabled={saving}
          title="Checkpoint learned continuous manifold and memories to disk"
        >
          {saving ? (
            <span className="spinner" style={{ width: '12px', height: '12px' }} />
          ) : saved ? (
            <Check size={14} style={{ color: 'var(--color-success)', marginRight: '0.2rem' }} />
          ) : (
            <Save size={14} style={{ marginRight: '0.2rem' }} />
          )}
          <span style={{ color: saved ? 'var(--color-success)' : undefined }}>{saved ? 'Saved' : 'Save Checkpoint'}</span>
        </Button>

        {loading && <span className="spinner" />}

        {/* Theme Mode Toggle (Dark / Light) */}
        <Button
          variant="icon"
          size="sm"
          onClick={() => {
            toggleDark();
            setTheme(dark ? 'light' : 'dark');
          }}
          aria-label="Toggle theme mode"
          title={`Current Theme: ${dark ? 'Dark' : 'Light'}`}
        >
          {dark ? <Sun size={17} /> : <Moon size={17} />}
        </Button>
      </nav>
    </header>
  );
}
