import { BookOpen } from 'lucide-react';
import type { View } from '../App';

interface SidebarProps {
  items: { id: View; label: string; icon: typeof BookOpen }[];
  active: View;
  onNavigate: (v: View) => void;
}

export function Sidebar({ items, active, onNavigate }: SidebarProps) {
  return (
    <aside style={{
      gridArea: 'sidebar',
      background: 'var(--bg-card)',
      backdropFilter: 'blur(20px)',
      WebkitBackdropFilter: 'blur(20px)',
      borderRight: `1px solid var(--border-color)`,
      display: 'flex',
      flexDirection: 'column',
      padding: '1.25rem 0.75rem',
      gap: '0.25rem',
      zIndex: 11,
    }}>
      <div style={{
        display: 'flex', alignItems: 'center', gap: '0.5rem',
        padding: '0.75rem 1rem', marginBottom: '0.5rem',
      }}>
        <span style={{
          fontFamily: 'var(--font-heading)',
          fontSize: '1.25rem', fontWeight: 700,
          color: 'var(--text-primary)',
        }}>
          Phiano
        </span>
      </div>

      <nav style={{ display: 'flex', flexDirection: 'column', gap: '0.125rem' }}>
        {items.map(({ id, label, icon: Icon }) => (
          <button
            key={id}
            className={`btn-icon ${active === id ? 'active' : ''}`}
            onClick={() => onNavigate(id)}
            style={{
              display: 'flex', alignItems: 'center', gap: '0.625rem',
              padding: '0.625rem 0.875rem', borderRadius: 'var(--radius-md)',
              width: '100%', justifyContent: 'flex-start',
              fontSize: '0.875rem', fontWeight: 500,
              color: active === id ? 'var(--color-primary)' : 'var(--text-secondary)',
              background: active === id ? 'var(--color-primary-light)' : 'transparent',
              transition: 'all var(--transition-fast)',
            }}
          >
            <Icon size={18} />
            {label}
          </button>
        ))}
      </nav>
    </aside>
  );
}
