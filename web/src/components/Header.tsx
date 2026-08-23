import { Sun, Moon, Database, Brain } from 'lucide-react';
import type { Stats } from '../types';

interface HeaderProps {
  dark: boolean;
  toggleDark: () => void;
  stats: Stats;
  loading: boolean;
}

export function Header({ dark, toggleDark, stats, loading }: HeaderProps) {
  return (
    <header style={{
      gridArea: 'header',
      display: 'flex', alignItems: 'center', justifyContent: 'space-between',
      padding: '0 1.5rem',
      borderBottom: '1px solid var(--border-color)',
      background: 'var(--bg-card)',
      backdropFilter: 'blur(20px)',
      WebkitBackdropFilter: 'blur(20px)',
      zIndex: 10,
    }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
        <span className="badge">
          <Database size={14} />
          {stats.vocabulary.toLocaleString()} words
        </span>
        <span className="badge">
          <Brain size={14} />
          {stats.memory_entries.toLocaleString()} memories
        </span>
      </div>

      <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
        {loading && <div className="spinner" />}
        <button className="btn-icon" onClick={toggleDark} aria-label="Toggle theme">
          {dark ? <Sun size={18} /> : <Moon size={18} />}
        </button>
      </div>
    </header>
  );
}
