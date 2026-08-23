import { useState, useCallback } from 'react';
import { Gauge } from 'lucide-react';
import { evalText } from '../hooks/useApi';
import type { EvalResult } from '../types';

export function EvalPanel() {
  const [text, setText] = useState('');
  const [result, setResult] = useState<EvalResult | null>(null);
  const [busy, setBusy] = useState(false);

  const doEval = useCallback(async () => {
    if (!text.trim() || busy) return;
    setBusy(true);
    try { setResult(await evalText(text)); } catch {}
    setBusy(false);
  }, [text, busy]);

  return (
    <div style={{ maxWidth: '640px', margin: '0 auto' }}>
      <div className="card animate-in">
        <div className="card-title"><Gauge size={18} style={{ verticalAlign: 'middle', marginRight: '0.5rem' }} />Evaluate (Transform Model)</div>
        <textarea className="textarea" value={text} onChange={e => setText(e.target.value)}
          placeholder="Enter text to evaluate..." style={{ minHeight: '100px' }} />
        <button className="btn btn-primary" onClick={doEval} disabled={busy || !text.trim()} style={{ marginTop: '0.75rem' }}>
          {busy ? <div className="spinner" /> : <Gauge size={16} />} Evaluate
        </button>
        {result && <EvalResults result={result} />}
      </div>
    </div>
  );
}

function EvalResults({ result }: { result: EvalResult }) {
  const metrics = [
    { label: 'Coherence', value: result.coherence, color: 'var(--color-primary)' },
    { label: 'Novelty', value: result.novelty, color: 'var(--color-warning)' },
    { label: 'Resonance', value: result.resonance, color: 'var(--color-success)' },
    { label: 'Overall', value: result.overall, color: 'var(--color-info)' },
  ];
  return (
    <div style={{ marginTop: '1.25rem' }}>
      {metrics.map(m => (
        <div key={m.label} style={{ marginBottom: '0.75rem' }}>
          <div className="metric-row">
            <span className="metric-label">{m.label}</span>
            <span className="metric-value">{m.value.toFixed(4)}</span>
          </div>
          <div className="bar-track">
            <div className="bar-fill" style={{ width: `${m.value * 100}%`, background: m.color }} />
          </div>
        </div>
      ))}
      <div style={{ marginTop: '1rem', padding: '0.75rem', background: 'var(--bg-secondary)', borderRadius: 'var(--radius-md)' }}>
        <span style={{ fontSize: '0.8rem', color: 'var(--text-secondary)' }}>Verdict: </span>
        <span style={{ fontSize: '0.85rem', fontWeight: 600 }}>{result.verdict}</span>
      </div>
    </div>
  );
}
