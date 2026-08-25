import { useState, useCallback } from 'react';
import {
  Network,
  Play,
  Loader2,
  Activity,
  Gauge,
  Zap,
} from 'lucide-react';
import { phaseFlow } from '../hooks/api/phaseFlow';
import type { FlowResponse, FlowNode } from '../types';

export function PhaseTopologyPanel() {
  const [input, setInput] = useState('the mushroom is growing');
  const [flow, setFlow] = useState<FlowResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [steps, setSteps] = useState(10);

  const run = useCallback(async () => {
    setLoading(true);
    try {
      const res = await phaseFlow(input, steps);
      setFlow(res);
    } catch (e) {
      console.error('PhaseFlow error:', e);
    } finally {
      setLoading(false);
    }
  }, [input, steps]);

  return (
    <div style={{
      display: 'flex',
      flexDirection: 'column',
      gap: '1rem',
      height: 'calc(100vh - 120px)',
    }}>
      {/* INPUT BAR */}
      <div style={{
        background: 'var(--card-bg, #111827)',
        borderRadius: '12px',
        border: '1px solid var(--border-color, #1f2937)',
        padding: '1rem 1.25rem',
        display: 'flex',
        alignItems: 'center',
        gap: '0.75rem',
      }}>
        <Network size={20} style={{ color: '#8b5cf6', flexShrink: 0 }} />
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => { if (e.key === 'Enter' && !loading) run(); }}
          placeholder="Enter text to visualize phase topology..."
          style={{
            flex: 1,
            padding: '0.5rem 0.75rem',
            borderRadius: '8px',
            border: '1px solid var(--border-color, #374151)',
            background: '#0f172a',
            color: '#f9fafb',
            fontSize: '0.9rem',
          }}
        />
        <label style={{ display: 'flex', alignItems: 'center', gap: '0.4rem', fontSize: '0.8rem', color: '#9ca3af' }}>
          Steps
          <input
            type="number"
            value={steps}
            onChange={(e) => setSteps(Math.max(1, Math.min(50, parseInt(e.target.value) || 10)))}
            style={{
              width: '50px',
              padding: '0.3rem',
              borderRadius: '6px',
              border: '1px solid var(--border-color, #374151)',
              background: '#0f172a',
              color: '#f9fafb',
              fontSize: '0.8rem',
              textAlign: 'center',
            }}
          />
        </label>
        <button
          onClick={run}
          disabled={loading || !input.trim()}
          style={{
            padding: '0.5rem 1rem',
            borderRadius: '8px',
            border: 'none',
            background: loading ? '#4b5563' : '#7c3aed',
            color: '#fff',
            cursor: loading ? 'not-allowed' : 'pointer',
            display: 'flex',
            alignItems: 'center',
            gap: '0.4rem',
            fontSize: '0.85rem',
            fontWeight: 600,
          }}
        >
          {loading ? <Loader2 size={16} className="animate-spin" /> : <Play size={16} />}
          {loading ? 'Computing...' : 'Visualize'}
        </button>
      </div>

      {/* MAIN CONTENT GRID */}
      <div style={{
        flex: 1,
        display: 'grid',
        gridTemplateColumns: '1fr 320px',
        gap: '1rem',
        overflow: 'hidden',
      }}>
        {/* PHASE CIRCLE VISUALIZATION */}
        <div style={{
          background: 'var(--card-bg, #111827)',
          borderRadius: '12px',
          border: '1px solid var(--border-color, #1f2937)',
          padding: '1.5rem',
          overflow: 'hidden',
          display: 'flex',
          flexDirection: 'column',
        }}>
          <div style={{
            display: 'flex',
            alignItems: 'center',
            gap: '0.5rem',
            marginBottom: '0.75rem',
          }}>
            <Activity size={18} style={{ color: '#8b5cf6' }} />
            <h2 style={{
              fontSize: '1.1rem',
              fontWeight: 700,
              color: '#f9fafb',
              margin: 0,
            }}>
              Phase Topology
            </h2>
            {flow && (
              <span style={{
                fontSize: '0.75rem',
                color: '#9ca3af',
                marginLeft: 'auto',
              }}>
                {flow.node_count} nodes · {flow.edge_count} edges
              </span>
            )}
          </div>
          {flow ? (
            <PhaseCircle flow={flow} />
          ) : (
            <div style={{
              flex: 1,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#4b5563',
              fontSize: '0.9rem',
            }}>
              Enter text and click Visualize to see the phase topology
            </div>
          )}
        </div>

        {/* METRICS SIDEBAR */}
        <div style={{
          display: 'flex',
          flexDirection: 'column',
          gap: '0.75rem',
          overflow: 'hidden',
        }}>
          {flow && <MetricsPanel flow={flow} />}
          {flow && flow.trajectory.length > 0 && <TrajectoryPanel flow={flow} />}
        </div>
      </div>
    </div>
  );
}

function PhaseCircle({ flow }: { flow: FlowResponse }) {
  const radius = 180;
  const centerX = 250;
  const centerY = 220;

  const nodePositions = flow.nodes.map((node) => {
    const angle = node.phase - Math.PI / 2;
    const r = radius * (0.5 + 0.5 * Math.min(1, node.amplitude));
    return {
      ...node,
      x: centerX + r * Math.cos(angle),
      y: centerY + r * Math.sin(angle),
    };
  });

  const collectiveAngle = flow.collective_phase - Math.PI / 2;
  const collectiveX = centerX + (radius + 30) * Math.cos(collectiveAngle);
  const collectiveY = centerY + (radius + 30) * Math.sin(collectiveAngle);

  const couplingColors: Record<string, string> = {
    bigram: '#3b82f6',
    syntax_lag: '#8b5cf6',
    semantic: '#10b981',
    anti_phase: '#ef4444',
  };

  return (
    <svg
      viewBox="0 0 500 440"
      style={{ width: '100%', height: '100%', maxHeight: '400px' }}
    >
      {/* Background circle */}
      <circle cx={centerX} cy={centerY} r={radius} fill="none" stroke="#1f2937" strokeWidth={1} strokeDasharray="4 4" />
      <circle cx={centerX} cy={centerY} r={radius * 0.5} fill="none" stroke="#1f2937" strokeWidth={0.5} strokeDasharray="2 4" />

      {/* Edges */}
      {flow.edges.map((edge, i) => {
        const from = nodePositions[edge.from];
        const to = nodePositions[edge.to];
        if (!from || !to) return null;
        const color = couplingColors[edge.coupling] || '#6b7280';
        return (
          <line
            key={`edge-${i}`}
            x1={from.x}
            y1={from.y}
            x2={to.x}
            y2={to.y}
            stroke={color}
            strokeWidth={1 + edge.weight * 2}
            strokeOpacity={0.4 + edge.weight * 0.3}
          />
        );
      })}

      {/* Collective phase arrow */}
      <line
        x1={centerX}
        y1={centerY}
        x2={collectiveX}
        y2={collectiveY}
        stroke="#fbbf24"
        strokeWidth={2}
        strokeDasharray="5 3"
        opacity={0.8}
      />
      <circle cx={collectiveX} cy={collectiveY} r={4} fill="#fbbf24" />

      {/* Nodes */}
      {nodePositions.map((node, i) => {
        const color = phaseToColor(node.phase);
        const nodeRadius = 6 + node.amplitude * 4;
        return (
          <g key={`node-${i}`}>
            <circle
              cx={node.x}
              cy={node.y}
              r={nodeRadius}
              fill={color}
              fillOpacity={0.3 + 0.4 * Math.min(1, node.activation)}
              stroke={color}
              strokeWidth={1.5}
            />
            <text
              x={node.x}
              y={node.y - nodeRadius - 4}
              textAnchor="middle"
              fill="#e5e7eb"
              fontSize="10"
              fontFamily="Inter, sans-serif"
            >
              {node.word.length > 12 ? node.word.slice(0, 10) + '…' : node.word}
            </text>
          </g>
        );
      })}

      {/* Center label */}
      <text x={centerX} y={centerY + 4} textAnchor="middle" fill="#6b7280" fontSize="9" fontFamily="monospace">
        φ={flow.collective_phase.toFixed(2)}
      </text>
    </svg>
  );
}

function MetricsPanel({ flow }: { flow: FlowResponse }) {
  const metrics = [
    { label: 'Collective Phase', value: flow.collective_phase.toFixed(3), icon: Zap, color: '#fbbf24' },
    { label: 'Order Parameter R', value: flow.order_parameter.toFixed(3), icon: Gauge, color: '#10b981' },
    { label: 'Momentum', value: flow.momentum.toFixed(3), icon: Activity, color: '#3b82f6' },
    { label: 'Novelty', value: flow.novelty.toFixed(3), icon: Network, color: '#8b5cf6' },
  ];

  return (
    <div style={{
      background: 'var(--card-bg, #111827)',
      borderRadius: '12px',
      border: '1px solid var(--border-color, #1f2937)',
      padding: '1rem 1.25rem',
      display: 'flex',
      flexDirection: 'column',
      gap: '0.6rem',
    }}>
      <h3 style={{
        fontSize: '0.8rem',
        fontWeight: 700,
        color: '#c4b5fd',
        textTransform: 'uppercase',
        letterSpacing: '0.05em',
        margin: 0,
      }}>
        Metrics
      </h3>
      {metrics.map((m) => {
        const Icon = m.icon;
        return (
          <div key={m.label} style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            padding: '0.4rem 0.6rem',
            borderRadius: '6px',
            background: 'rgba(0,0,0,0.2)',
          }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '0.4rem' }}>
              <Icon size={14} style={{ color: m.color }} />
              <span style={{ fontSize: '0.78rem', color: '#9ca3af' }}>{m.label}</span>
            </div>
            <span style={{
              fontSize: '0.85rem',
              fontWeight: 700,
              color: m.color,
              fontFamily: 'monospace',
            }}>
              {m.value}
            </span>
          </div>
        );
      })}
    </div>
  );
}

function TrajectoryPanel({ flow }: { flow: FlowResponse }) {
  return (
    <div style={{
      background: 'var(--card-bg, #111827)',
      borderRadius: '12px',
      border: '1px solid var(--border-color, #1f2937)',
      padding: '1rem 1.25rem',
      flex: 1,
      overflow: 'hidden',
      display: 'flex',
      flexDirection: 'column',
    }}>
      <h3 style={{
        fontSize: '0.8rem',
        fontWeight: 700,
        color: '#c4b5fd',
        textTransform: 'uppercase',
        letterSpacing: '0.05em',
        margin: 0,
        marginBottom: '0.5rem',
      }}>
        Trajectory
      </h3>
      <div style={{
        flex: 1,
        overflowY: 'auto',
        display: 'flex',
        flexDirection: 'column',
        gap: '0.3rem',
      }}>
        {flow.trajectory.map((step, i) => (
          <div key={i} style={{
            display: 'flex',
            alignItems: 'center',
            gap: '0.5rem',
            padding: '0.35rem 0.5rem',
            borderRadius: '4px',
            background: 'rgba(0,0,0,0.15)',
            fontSize: '0.75rem',
          }}>
            <span style={{ color: '#6b7280', fontFamily: 'monospace', width: '24px' }}>
              {String(step.step).padStart(2, '0')}
            </span>
            {step.selected_word && (
              <span style={{ color: '#a78bfa', fontWeight: 600 }}>
                {step.selected_word}
              </span>
            )}
            <span style={{ color: '#fbbf24', fontFamily: 'monospace', marginLeft: 'auto' }}>
              φ={step.collective_phase.toFixed(2)}
            </span>
            <span style={{ color: '#10b981', fontFamily: 'monospace' }}>
              R={step.resonance_score.toFixed(2)}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}

function phaseToColor(phase: number): string {
  const deg = ((phase * 180 / Math.PI) % 360 + 360) % 360;
  return `hsl(${deg.toFixed(0)}, 80%, 60%)`;
}
