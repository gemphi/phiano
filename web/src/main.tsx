import { StrictMode, useState, useEffect, useCallback } from 'react';
import { createRoot } from 'react-dom/client';
import './styles/globals.css';
import { App } from './App';
import type { Stats } from './types';
import { fetchStats } from './hooks/useApi';

function Root() {
  const [dark, setDark] = useState(() => {
    try { return localStorage.getItem('theme') === 'dark'; } catch { return false; }
  });
  const [stats, setStats] = useState<Stats>({ vocabulary: 0, memory_entries: 0 });
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    document.documentElement.classList.toggle('dark', dark);
    try { localStorage.setItem('theme', dark ? 'dark' : 'light'); } catch {}
  }, [dark]);

  const refreshStats = useCallback(async () => {
    setLoading(true);
    try { setStats(await fetchStats()); } catch {}
    setLoading(false);
  }, []);

  useEffect(() => { refreshStats(); }, [refreshStats]);

  return (
    <StrictMode>
      <App
        dark={dark}
        toggleDark={() => setDark(d => !d)}
        stats={stats}
        loading={loading}
        refreshStats={refreshStats}
      />
    </StrictMode>
  );
}

createRoot(document.getElementById('root')!).render(<Root />);
