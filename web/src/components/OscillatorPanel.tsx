import { useState, useCallback } from 'react';
import { Waves, Activity } from 'lucide-react';
import { oscEval, oscTrain } from '../hooks/useApi';
import type { OscEvalResult, OscTrainResult } from '../types';

interface OscillatorPanelProps {
  onRefresh: () => Promise<void>;
}

export function OscillatorPanel({ onRefresh }: OscillatorPanelProps) {
  const [text, setText] = useState('');
  const [evalResult, setEvalResult] = useState<OscEvalResult | null>(null);
  const [trainResult, setTrainResult] = useState<OscTrainResult | null>(null);
  const [busy, setBusy] = useState(false);

  const doEval = useCallback(async () => {
    if (!text.trim() || busy) return;
    setBusy(true);
    try { setEvalResult(await oscEval(text)); } catch {}
    setBusy(false);
  }, [text, busy]);

  const doTrain = useCallback(async () => {
    if (!text.trim() || busy) return;
    setBusy(true);
    try {
      const r = await oscTrain(text, 10);
      setTrainResult(r);
      const er = await oscEval(text);
      setEvalResult(er);
      await onRefresh();
    } catch {}
    setBusy(false);
  }, [text, busy, onRefresh]);

  return (
    <div style={{ maxWidth: '640px', margin: '0 auto', display: 'flex', flexDirection: 'column', gap: '1.5rem' }}>
      <div className="card animate-in">
        <div className="card-title"><Waves size={18} style={{ verticalAlign: 'middle', marginRight: '0.5rem' }} />Oscillator Model</div>
        <textarea className="textarea" value={text} onChange={e => setText(e.target.value)}
          placeholder="Enter text for oscillator analysis..." style={{ minHeight: '100px' }} />
        <div style={{ display: 'flex', gap: '0.75rem', marginTop: '0.75rem' }}>
          <button className="btn btn-primary" onClick={doEval} disabled={busy || !text.trim()}>
            {busy ? <div className="spinner" /> : <Waves size={16} />} Evaluate
          </button>
          <button className="btn btn-ghost" onClick={doTrain} disabled={busy || !text.trim()}>
            <Activity size={16} /> Train (10 epochs)
          </button>
        </div>
      </div>

      {evalResult && (
        <div className="card animate-in">
          <div className="card-title">Oscillator Evaluation</div>
          <OscMetrics result={evalResult} />
        </div>
      )}

      {trainResult && (
        <div className="card animate-in">
          <div className="card-title">Training Result</div>
          <div style={{ fontSize: '0.85rem' }}>
            <div className="metric-row"><span className="metric-label">Epochs</span><span className="metric-value">{trainResult.epochs}</span></div>
            <div className="metric-row"><span className="metric-label">Converged</span>
              <span className="metric-value" style={{ color: trainResult.converged ? 'var(--color-success)' : 'var(--color-warning)' }}>
                {trainResult.converged ? 'Yes' : 'No'}
              </span>
            </div>
            <div className="metric-row"><span className="metric-label">Coherence</span>
              <span className="metric-value">{trainResult.coherence_before.toFixed(3)} → {trainResult.coherence_after.toFixed(3)}</span>
            </div>
            <div className="metric-row"><span className="metric-label">Sync</span>
              <span className="metric-value">{trainResult.sync_before.toFixed(3)} → {trainResult.sync_after.toFixed(3)}</span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function OscMetrics({ result }: { result: OscEvalResult }) {
  const metrics = [
    { label: 'Coherence (order parameter)', value: result.coherence, max: 1, color: 'var(--color-primary)' },
    { label: 'Sync (avg pairwise)', value: result.sync, max: 1, color: 'var(--color-warning)' },
    { label: 'Spectral entropy', value: result.entropy, max: 4, color: 'var(--color-success)' },
  ];
  return (
    <div>
      {metrics.map(m => (
        <div key={m.label} style={{ marginBottom: '0.75rem' }}>
          <div className="metric-row">
            <span className="metric-label">{m.label}</span>
            <span className="metric-value">{m.value.toFixed(4)}</span>
          </div>
          <div className="bar-track">
            <div className="bar-fill" style={{ width: `${Math.min(100, (m.value / m.max) * 100)}%`, background: m.color }} />
          </div>
        </div>
      ))}
      <div className="metric-row" style={{ marginTop: '0.5rem' }}>
        <span className="metric-label">Words</span><span className="metric-value">{result.word_count}</span>
      </div>
      {result.dominant_colors.length > 0 && (
        <div style={{ marginTop: '0.75rem', display: 'flex', gap: '0.5rem', flexWrap: 'wrap' }}>
          {result.dominant_colors.slice(0, 5).map(([color, amp], i) => (
            <span key={i} className="badge">{color} ({amp.toFixed(1)})</span>
          ))}
        </div>
      )}
    </div>
  );
}
