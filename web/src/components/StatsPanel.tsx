import { BarChart3, Database, Brain, Zap } from 'lucide-react';
import type { Stats } from '../types';

interface StatsPanelProps {
  stats: Stats;
}

export function StatsPanel({ stats }: StatsPanelProps) {
  const items = [
    { icon: Database, label: 'Vocabulary', value: stats.vocabulary.toLocaleString(), color: 'var(--color-primary)' },
    { icon: Brain, label: 'Memory Entries', value: stats.memory_entries.toLocaleString(), color: 'var(--color-info)' },
  ];

  return (
    <div style={{ maxWidth: '640px', margin: '0 auto' }}>
      <div className="card animate-in">
        <div className="card-title"><BarChart3 size={18} style={{ verticalAlign: 'middle', marginRight: '0.5rem' }} />Statistics</div>
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '1rem' }}>
          {items.map(({ icon: Icon, label, value, color }) => (
            <div key={label} style={{
              padding: '1.25rem',
              background: 'var(--bg-secondary)',
              borderRadius: 'var(--radius-lg)',
              border: '1px solid var(--border-color)',
              textAlign: 'center',
            }}>
              <Icon size={28} style={{ color, marginBottom: '0.5rem' }} />
              <div style={{ fontSize: '1.5rem', fontWeight: 700, color: 'var(--text-primary)' }}>{value}</div>
              <div style={{ fontSize: '0.8rem', color: 'var(--text-secondary)', marginTop: '0.25rem' }}>{label}</div>
            </div>
          ))}
        </div>
        <div style={{ marginTop: '1.25rem', padding: '1rem', background: 'var(--bg-secondary)', borderRadius: 'var(--radius-md)', display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <Zap size={16} style={{ color: 'var(--color-warning)' }} />
          <span style={{ fontSize: '0.8rem', color: 'var(--text-secondary)' }}>
            Phiano uses Kuramoto oscillator synchronization for language understanding - a non-transformer architecture.
          </span>
        </div>
      </div>
    </div>
  );
}
