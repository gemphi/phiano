import { useState, useCallback } from 'react';
import { BookOpen, Gauge, Waves, MessageSquare, BarChart3, Sparkles, Cpu, FileText, Search } from 'lucide-react';
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
import { DocsPanel } from './components/DocsPanel';
import type { Stats } from './types';

export type View = 'dictionary' | 'chat' | 'docs' | 'studio' | 'infinity' | 'oscillator' | 'learn' | 'eval' | 'stats';

const NAV_ITEMS: { id: View; label: string; icon: typeof BookOpen }[] = [
  { id: 'dictionary', label: 'Dictionary & Story', icon: Search },
  { id: 'chat', label: 'Cognitive Chat', icon: MessageSquare },
  { id: 'learn', label: 'Teach & Entrain', icon: BookOpen },
  { id: 'infinity', label: 'Infinity Resonance', icon: Sparkles },
  { id: 'oscillator', label: '3D Kuramoto Sphere', icon: Waves },
  { id: 'studio', label: 'Phi-4 Studio', icon: Cpu },
  { id: 'docs', label: 'Documentation', icon: FileText },
  { id: 'eval', label: 'Evaluate', icon: Gauge },
  { id: 'stats', label: 'Stats & Manifold', icon: BarChart3 },
];

interface AppProps {
  dark: boolean;
  toggleDark: () => void;
  stats: Stats;
  loading: boolean;
  refreshStats: () => Promise<void>;
}

export function App({ dark, toggleDark, stats, loading, refreshStats }: AppProps) {
  const [view, setView] = useState<View>('dictionary');

  const handleNavigate = useCallback((v: View) => setView(v), []);

  return (
    <div style={{
      display: 'grid',
      gridTemplateColumns: `var(--sidebar-width) 1fr`,
      gridTemplateRows: `var(--header-height) 1fr`,
      gridTemplateAreas: `'sidebar header' 'sidebar main'`,
      minHeight: '100vh',
    }}>
      <Sidebar items={NAV_ITEMS} active={view} onNavigate={handleNavigate} />
      <Header
        dark={dark}
        toggleDark={toggleDark}
        stats={stats}
        loading={loading}
      />
      <main style={{ gridArea: 'main', padding: '1.5rem', overflow: 'auto' }}>
        {view === 'dictionary' && <DictionaryPanel onRefresh={refreshStats} />}
        {view === 'chat' && <ChatPanel onRefresh={refreshStats} />}
        {view === 'learn' && <LearnPanel onRefresh={refreshStats} />}
        {view === 'infinity' && <InfinityPanel onRefresh={refreshStats} />}
        {view === 'oscillator' && <OscillatorPanel onRefresh={refreshStats} />}
        {view === 'studio' && <Phi4StudioPanel onRefresh={refreshStats} />}
        {view === 'docs' && <DocsPanel />}
        {view === 'eval' && <EvalPanel />}
        {view === 'stats' && <StatsPanel stats={stats} />}
      </main>
    </div>
  );
}
