import { useState, useRef, useEffect, useCallback } from 'react';
import { Send, Sparkles } from 'lucide-react';
import type { ChatMessage } from '../types';
import { learnText, evalText, oscEval, generateText } from '../hooks/useApi';

interface ChatPanelProps {
  onRefresh: () => Promise<void>;
}

export function ChatPanel({ onRefresh }: ChatPanelProps) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [busy, setBusy] = useState(false);
  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => { endRef.current?.scrollIntoView({ behavior: 'smooth' }); }, [messages]);

  const send = useCallback(async () => {
    const text = input.trim();
    if (!text || busy) return;
    setInput('');
    setBusy(true);

    const userMsg: ChatMessage = { role: 'user', text };
    setMessages(m => [...m, userMsg]);

    try {
      // 1. Train/learn online
      await learnText(text).catch(() => null);
      await onRefresh().catch(() => null);

      // 2. Generate phase-guided response and evaluate
      const [genRes, evalRes, oscRes] = await Promise.all([
        generateText(text, 24, 0.15).catch(() => null),
        evalText(text).catch(() => null),
        oscEval(text).catch(() => null),
      ]);

      const parts: string[] = [];
      if (genRes && genRes.generated) {
        parts.push(genRes.generated);
      } else {
        parts.push("Phasor pattern recorded in manifold.");
      }

      if (evalRes) {
        parts.push(`\nCoherence: ${evalRes.coherence.toFixed(3)} · Novelty: ${evalRes.novelty.toFixed(3)} · Resonance: ${evalRes.resonance.toFixed(3)} · ${evalRes.verdict}`);
      }
      if (oscRes && oscRes.dominant_colors) {
        const colors = oscRes.dominant_colors.slice(0, 3)
          .map((c: [string, number]) => `${c[0]} (${c[1].toFixed(1)})`).join(', ');
        parts.push(`Oscillator: sync ${oscRes.sync.toFixed(3)}, entropy ${oscRes.entropy.toFixed(3)} [${colors}]`);
      }

      const assistantMsg: ChatMessage = {
        role: 'assistant',
        text: parts.join('\n'),
        eval: evalRes || undefined,
        oscEval: oscRes || undefined,
      };
      setMessages(m => [...m, assistantMsg]);
    } catch (e) {
      setMessages(m => [...m, {
        role: 'assistant', text: `Error: ${e}`,
      }]);
    }
    setBusy(false);
  }, [input, busy, onRefresh]);

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%', maxWidth: '800px', margin: '0 auto' }}>
      <div style={{ flex: 1, overflow: 'auto', display: 'flex', flexDirection: 'column', gap: '1rem' }}>
        {messages.length === 0 && (
          <div className="card animate-in" style={{ textAlign: 'center', padding: '3rem' }}>
            <Sparkles size={32} style={{ color: 'var(--color-primary)', marginBottom: '0.75rem' }} />
            <div className="card-title">Chat with Phiano</div>
            <p style={{ color: 'var(--text-secondary)', fontSize: '0.875rem' }}>
              Type any text and Phiano will learn it, evaluate coherence,<br />
              and analyze it through the oscillator model — like Phi-4.
            </p>
          </div>
        )}
        {messages.map((m, i) => <MessageBubble key={i} msg={m} />)}
        <div ref={endRef} />
      </div>
      <div style={{ display: 'flex', gap: '0.5rem', padding: '1rem 0' }}>
        <textarea
          className="textarea"
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={e => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); send(); } }}
          placeholder="Type text to learn and evaluate..."
          style={{ minHeight: '48px', maxHeight: '120px' }}
          disabled={busy}
        />
        <button className="btn btn-primary" onClick={send} disabled={busy || !input.trim()}>
          {busy ? <div className="spinner" /> : <Send size={16} />}
          Send
        </button>
      </div>
    </div>
  );
}

function MessageBubble({ msg }: { msg: ChatMessage }) {
  const isUser = msg.role === 'user';
  return (
    <div className="animate-in" style={{
      alignSelf: isUser ? 'flex-end' : 'flex-start',
      maxWidth: '85%',
    }}>
      <div style={{
        padding: '0.75rem 1rem',
        borderRadius: 'var(--radius-lg)',
        background: isUser ? 'var(--color-primary)' : 'var(--bg-secondary)',
        color: isUser ? 'var(--text-inverse)' : 'var(--text-primary)',
        border: isUser ? 'none' : '1px solid var(--border-color)',
        fontSize: '0.875rem',
        whiteSpace: 'pre-wrap',
        lineHeight: 1.5,
      }}>
        {msg.text}
      </div>
      {msg.eval && (
        <div style={{ display: 'flex', gap: '0.5rem', marginTop: '0.5rem', flexWrap: 'wrap' }}>
          <Metric label="coh" value={msg.eval.coherence} color="var(--color-primary)" />
          <Metric label="nov" value={msg.eval.novelty} color="var(--color-warning)" />
          <Metric label="res" value={msg.eval.resonance} color="var(--color-success)" />
        </div>
      )}
    </div>
  );
}

function Metric({ label, value, color }: { label: string; value: number; color: string }) {
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: '0.25rem' }}>
      <span style={{ fontSize: '0.7rem', color: 'var(--text-secondary)' }}>{label}</span>
      <div style={{ width: '48px', height: '4px', background: 'var(--bg-secondary)', borderRadius: 'var(--radius-full)' }}>
        <div style={{ width: `${value * 100}%`, height: '100%', background: color, borderRadius: 'var(--radius-full)' }} />
      </div>
    </div>
  );
}
