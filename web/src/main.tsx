import { StrictMode, useState, useEffect, useCallback } from 'react';
import { createRoot } from 'react-dom/client';
import { PuiProvider, usePuiTheme } from '@phiace/puijs';
import './styles/globals.css';
import { App } from './App';
import type { Stats } from './types';
import { fetchStats } from './hooks/api/stats';

type ThemedAppProps = {
  stats: Stats;
  loading: boolean;
  refreshStats: () => void;
};

function ThemedApp({ stats, loading, refreshStats }: ThemedAppProps) {
  const { isDark, setTheme } = usePuiTheme();
  return (
    <App
      dark={isDark}
      toggleDark={() => setTheme(isDark ? 'light' : 'dark')}
      stats={stats}
      loading={loading}
      refreshStats={refreshStats}
    />
  );
}

function Root() {
  const [stats, setStats] = useState<Stats>({ vocabulary: 0, memory_entries: 0 });
  const [loading, setLoading] = useState(false);

  const refreshStats = useCallback(async () => {
    setLoading(true);
    try { setStats(await fetchStats()); } catch {}
    setLoading(false);
  }, []);

  useEffect(() => { refreshStats(); }, [refreshStats]);

  return (
    <StrictMode>
      <PuiProvider defaultTheme="system" defaultThemeStyle="apple" defaultBrand="phiano">
        <ThemedApp stats={stats} loading={loading} refreshStats={refreshStats} />
      </PuiProvider>
    </StrictMode>
  );
}

createRoot(document.getElementById('root')!).render(<Root />);
