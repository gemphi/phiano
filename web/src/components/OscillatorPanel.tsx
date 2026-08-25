import { useState, useCallback, useRef, useEffect } from 'react';
import { Waves, Activity, Sparkles, Box, Rotate3d } from 'lucide-react';
import { ThreeDMode } from '@phiace/puijs';
import { oscEval, oscTrain } from '../hooks/api/oscillator';
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
    <div style={{ maxWidth: '980px', margin: '0 auto', display: 'flex', flexDirection: 'column', gap: '1.5rem' }}>
      {/* 3D Topological Manifolds & Objects Studio (16 Interactive Visualizations) */}
      <div className="card animate-in">
        <div className="card-title" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <span>
            <Rotate3d size={18} style={{ verticalAlign: 'middle', marginRight: '0.5rem', color: 'var(--color-primary)' }} />
            3D Topological Manifolds & Harmonic Graphs
          </span>
          <span style={{ fontSize: '0.75rem', fontWeight: 500, color: 'var(--text-secondary)' }}>
            16 Interactive 3D Objects · PUI Engine
          </span>
        </div>
        <p style={{ color: 'var(--text-secondary)', fontSize: '0.875rem', marginBottom: '1rem', lineHeight: 1.5 }}>
          Explore continuous phase manifolds (T² torus), Riemann hyperspheres (S²), Klein bottle immersions,
          16-layer hypercube lattices, and Lorentz attractor phase trajectories in real-time 3D.
        </p>
        <ThreeDMode
          initialObject="torus_manifold"
          height={420}
          renderStyle="hybrid"
          palette="chromatic"
          showControls={true}
          showCatalog={true}
        />
      </div>

      <div className="card animate-in">
        <div className="card-title">
          <Waves size={18} style={{ verticalAlign: 'middle', marginRight: '0.5rem', color: 'var(--color-primary)' }} />
          Kuramoto Oscillator Manifold & Phase Synchronization
        </div>
        <p style={{ color: 'var(--text-secondary)', fontSize: '0.875rem', marginBottom: '1rem', lineHeight: 1.5 }}>
          Text is mapped onto continuous phase oscillators. Phase coherence ($R$) measures collective semantic alignment,
          and pairwise sync evaluates local syntactic harmonic coupling.
        </p>
        <textarea
          className="textarea"
          value={text}
          onChange={e => setText(e.target.value)}
          placeholder="Enter a concept, sentence, or question to analyze phase resonance..."
          style={{ minHeight: '90px' }}
        />
        <div style={{ display: 'flex', gap: '0.75rem', marginTop: '0.75rem' }}>
          <button className="btn btn-primary" onClick={doEval} disabled={busy || !text.trim()}>
            {busy ? <div className="spinner" /> : <Waves size={16} />} Evaluate Phase Sync
          </button>
          <button className="btn btn-ghost" onClick={doTrain} disabled={busy || !text.trim()}>
            <Activity size={16} /> Entrain Kuramoto (10 epochs)
          </button>
        </div>
      </div>

      {evalResult && (
        <div style={{ display: 'grid', gridTemplateColumns: 'minmax(280px, 340px) 1fr', gap: '1.5rem', alignItems: 'start' }}>
          <div className="card animate-in" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', textAlign: 'center' }}>
            <div className="card-title" style={{ alignSelf: 'flex-start', marginBottom: '0.5rem' }}>
              <Sparkles size={16} style={{ marginRight: '0.35rem', color: 'var(--color-primary)' }} />
              Phase Manifold Ring
            </div>
            <PhaseWheelCanvas result={evalResult} />
            <div style={{ marginTop: '0.75rem', fontSize: '0.8rem', color: 'var(--text-secondary)' }}>
              Kuramoto Order Parameter $R = {evalResult.coherence.toFixed(3)}$
            </div>
          </div>

          <div className="card animate-in">
            <div className="card-title">Harmonic Metrics</div>
            <OscMetrics result={evalResult} />
          </div>
        </div>
      )}

      {trainResult && (
        <div className="card animate-in">
          <div className="card-title">Entrainment Convergence</div>
          <div style={{ fontSize: '0.85rem' }}>
            <div className="metric-row"><span className="metric-label">Epochs</span><span className="metric-value">{trainResult.epochs}</span></div>
            <div className="metric-row"><span className="metric-label">Converged</span>
              <span className="metric-value" style={{ color: trainResult.converged ? 'var(--color-success)' : 'var(--color-warning)', fontWeight: 600 }}>
                {trainResult.converged ? 'Yes (Harmonic Equilibrium)' : 'Tuning'}
              </span>
            </div>
            <div className="metric-row"><span className="metric-label">Coherence Shift</span>
              <span className="metric-value">{trainResult.coherence_before.toFixed(3)} → {trainResult.coherence_after.toFixed(3)}</span>
            </div>
            <div className="metric-row"><span className="metric-label">Pairwise Coupling</span>
              <span className="metric-value">{trainResult.sync_before.toFixed(3)} → {trainResult.sync_after.toFixed(3)}</span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function PhaseWheelCanvas({ result }: { result: OscEvalResult }) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const dpr = window.devicePixelRatio || 1;
    const size = 260;
    canvas.width = size * dpr;
    canvas.height = size * dpr;
    ctx.scale(dpr, dpr);

    const cx = size / 2;
    const cy = size / 2;
    const radius = 95;

    ctx.clearRect(0, 0, size, size);

    // 1. Draw 8 Chromatic Sectors background ring
    const colors = [
      '#ef4444', '#f97316', '#eab308', '#22c55e',
      '#06b6d4', '#3b82f6', '#6366f1', '#a855f7',
    ];
    const numSectors = 8;
    const angleStep = (2 * Math.PI) / numSectors;

    for (let i = 0; i < numSectors; i++) {
      const startAngle = i * angleStep - Math.PI / 2;
      const endAngle = (i + 1) * angleStep - Math.PI / 2;

      ctx.beginPath();
      ctx.arc(cx, cy, radius, startAngle, endAngle);
      ctx.arc(cx, cy, radius - 16, endAngle, startAngle, true);
      ctx.closePath();
      ctx.fillStyle = colors[i] + '33'; // 20% opacity
      ctx.fill();

      ctx.strokeStyle = colors[i] + '66';
      ctx.lineWidth = 1;
      ctx.stroke();
    }

    // 2. Outer circle border
    ctx.beginPath();
    ctx.arc(cx, cy, radius + 2, 0, 2 * Math.PI);
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.15)';
    ctx.lineWidth = 1.5;
    ctx.stroke();

    // 3. Central Order Parameter Vector R * exp(i * psi)
    const R = Math.min(1.0, Math.max(0.0, result.coherence));
    const arrowLength = radius * R;
    const psi = 0; // Center orientation

    ctx.save();
    ctx.translate(cx, cy);

    // Vector line
    ctx.beginPath();
    ctx.moveTo(0, 0);
    ctx.lineTo(arrowLength * Math.cos(psi), arrowLength * Math.sin(psi));
    ctx.strokeStyle = '#3b82f6';
    ctx.lineWidth = 3.5;
    ctx.lineCap = 'round';
    ctx.stroke();

    // Head circle
    ctx.beginPath();
    ctx.arc(arrowLength * Math.cos(psi), arrowLength * Math.sin(psi), 5, 0, 2 * Math.PI);
    ctx.fillStyle = '#60a5fa';
    ctx.fill();
    ctx.strokeStyle = '#ffffff';
    ctx.lineWidth = 1.5;
    ctx.stroke();

    // Center pivot
    ctx.beginPath();
    ctx.arc(0, 0, 4, 0, 2 * Math.PI);
    ctx.fillStyle = '#3b82f6';
    ctx.fill();

    ctx.restore();
  }, [result]);

  return (
    <canvas
      ref={canvasRef}
      style={{
        width: '260px',
        height: '260px',
        borderRadius: '50%',
        background: 'rgba(0,0,0,0.15)',
        boxShadow: 'inset 0 0 20px rgba(0,0,0,0.2)',
      }}
    />
  );
}

function OscMetrics({ result }: { result: OscEvalResult }) {
  const metrics = [
    { label: 'Coherence (order parameter R)', value: result.coherence, max: 1, color: 'var(--color-primary)' },
    { label: 'Pairwise Harmonic Sync', value: result.sync, max: 1, color: 'var(--color-warning)' },
    { label: 'Spectral Entropy (information capacity)', value: result.entropy, max: 4, color: 'var(--color-success)' },
  ];
  return (
    <div>
      {metrics.map(m => (
        <div key={m.label} style={{ marginBottom: '0.85rem' }}>
          <div className="metric-row">
            <span className="metric-label">{m.label}</span>
            <span className="metric-value">{m.value.toFixed(4)}</span>
          </div>
          <div className="bar-track" style={{ height: '8px', borderRadius: '4px' }}>
            <div className="bar-fill" style={{ width: `${Math.min(100, (m.value / m.max) * 100)}%`, background: m.color, borderRadius: '4px' }} />
          </div>
        </div>
      ))}
      <div className="metric-row" style={{ marginTop: '0.75rem', borderTop: '1px solid var(--border-color)', paddingTop: '0.5rem' }}>
        <span className="metric-label">Tokens Evaluated</span><span className="metric-value">{result.word_count}</span>
      </div>
      {result.dominant_colors.length > 0 && (
        <div style={{ marginTop: '0.75rem', display: 'flex', gap: '0.5rem', flexWrap: 'wrap' }}>
          {result.dominant_colors.slice(0, 6).map(([color, amp], i) => (
            <span key={i} className="badge" style={{ fontSize: '0.75rem', fontWeight: 600 }}>
              {color} ({amp.toFixed(1)})
            </span>
          ))}
        </div>
      )}
    </div>
  );
}

