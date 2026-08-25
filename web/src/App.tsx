import { useState, useCallback, useEffect } from 'react';
import { Sidebar } from './components/Sidebar';
import { Header } from './components/Header';
import { DictionaryPanel } from './components/DictionaryPanel';
import { LearnPanel } from './components/LearnPanel';
import { EvalPanel } from './components/EvalPanel';
import { OscillatorPanel } from './components/OscillatorPanel';
import { ChatPanel } from './components/ChatPanel';
import { StatsPanel } from './components/StatsPanel';
import { InfinityPanel } from './components/InfinityPanel';
import { Phi4StudioPanel } from './components/Phi4StudioPanel';
import { Docs } from './components/Docs';
import { VersusPanel } from './components/VersusPanel';
import { PhaseTopologyPanel } from './components/PhaseTopologyPanel';
import { SymbolsPanel } from './components/SymbolsPanel';
import type { Stats } from './types';

export type View =
  | 'dictionary'
  | 'chat'
  | 'docs'
  | 'versus'
  | 'topology'
  | 'symbols'
  | 'studio'
  | 'infinity'
  | 'oscillator'
  | 'learn'
  | 'eval'
  | 'stats';

interface AppProps {
  dark: boolean;
  toggleDark: () => void;
  stats: Stats;
  loading: boolean;
  refreshStats: () => Promise<void>;
}

export function App({ dark, toggleDark, stats, loading, refreshStats }: AppProps) {
  const [view, setView] = useState<View>(() => {
    if (typeof window !== 'undefined' && window.location.pathname.startsWith('/docs')) {
      return 'docs';
    }
    return 'dictionary';
  });

  const handleNavigate = useCallback((v: View) => {
    setView(v);
    if (typeof window !== 'undefined') {
      if (v === 'docs') {
        window.history.pushState({}, '', '/docs');
      } else if (window.location.pathname.startsWith('/docs')) {
        window.history.pushState({}, '', '/');
      }
    }
  }, []);

  // Listen for browser popstate back/forward
  useEffect(() => {
    const handlePopState = () => {
      if (window.location.pathname.startsWith('/docs')) {
        setView('docs');
      } else {
        setView('dictionary');
      }
    };
    window.addEventListener('popstate', handlePopState);
    return () => window.removeEventListener('popstate', handlePopState);
  }, []);

  // Standalone Fullscreen Documentation Portal
  if (view === 'docs') {
    return (
      <Docs
        onBackToCockpit={() => handleNavigate('dictionary')}
        dark={dark}
        toggleDark={toggleDark}
      />
    );
  }

  // Cockpit Workspace Layout
  return (
    <div style={{
      display: 'grid',
      gridTemplateColumns: `var(--sidebar-width) 1fr`,
      gridTemplateRows: `var(--header-height) 1fr`,
      gridTemplateAreas: `'sidebar header' 'sidebar main'`,
      minHeight: '100vh',
    }}>
      <Sidebar active={view} onNavigate={handleNavigate} />
      <Header
        dark={dark}
        toggleDark={toggleDark}
        stats={stats}
        loading={loading}
        onNavigate={handleNavigate}
      />
      <main style={{ gridArea: 'main', padding: '1.5rem', overflow: 'auto' }}>
        {view === 'dictionary' && <DictionaryPanel onRefresh={refreshStats} />}
        {view === 'chat' && <ChatPanel onRefresh={refreshStats} />}
        {view === 'learn' && <LearnPanel onRefresh={refreshStats} />}
        {view === 'infinity' && <InfinityPanel onRefresh={refreshStats} />}
        {view === 'oscillator' && <OscillatorPanel onRefresh={refreshStats} />}
        {view === 'studio' && <Phi4StudioPanel onRefresh={refreshStats} />}
        {view === 'versus' && <VersusPanel />}
        {view === 'topology' && <PhaseTopologyPanel />}
        {view === 'symbols' && <SymbolsPanel />}
        {view === 'eval' && <EvalPanel />}
        {view === 'stats' && <StatsPanel stats={stats} />}
      </main>
    </div>
  );
}
