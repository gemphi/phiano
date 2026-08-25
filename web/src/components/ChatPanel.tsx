import { useState, useRef, useEffect, useCallback } from 'react';
import { Send, Sparkles, BookOpen, Brain, CheckCircle2, ShieldCheck, Zap, Waves } from 'lucide-react';
import type { ChatMessage } from '../types';
import { chatMessage } from '../hooks/api/chat';
import { learnText } from '../hooks/api/learn';
import { streamGenerate } from '../hooks/api/generate';

interface ChatPanelProps {
  onRefresh: () => Promise<void>;
}

const SUGGESTIONS = [
  "Explain why dolphins are mammals and not fish",
  "Hello! My name is Alex and my daughter Maya loves marine biology",
  "What is the difference between brute facts and institutional facts?",
  "I hereby declare this research project active",
  "How does Kuramoto oscillator synchronization work?",
];

export function ChatPanel({ onRefresh }: ChatPanelProps) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [busy, setBusy] = useState(false);
  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const sendText = useCallback(async (textToSend: string) => {
    const text = textToSend.trim();
    if (!text || busy) return;
    setInput('');
    setBusy(true);

    const userMsg: ChatMessage = { role: 'user', text };
    const draft: ChatMessage = { role: 'assistant', text: '', streaming: true };
    setMessages(m => [...m, userMsg, draft]);

    try {
      learnText(text).catch(() => null);
      await streamGenerate(text, evt => {
        setMessages(m => {
          const next = m.slice();
          const last = next[next.length - 1];
          if (!last || last.role !== 'assistant') return m;
          next[next.length - 1] = {
            ...last,
            text: evt.done ? last.text : `${last.text}${last.text ? ' ' : ''}${evt.token}`,
            streaming: !evt.done,
            collective_phase: evt.collective_phase,
            resonance: evt.resonance,
            speech_act: evt.done ? 'generate' : last.speech_act,
          };
          return next;
        });
      });
      await onRefresh().catch(() => null);
    } catch {
      try {
        const res = await chatMessage(text);
        await onRefresh().catch(() => null);
        setMessages(m => {
          const next = m.slice();
          next[next.length - 1] = {
            role: 'assistant',
            text: res.response,
            speech_act: res.speech_act,
            direction_of_fit: res.direction_of_fit,
            words_learned: res.words_learned,
            definitions_learned: res.definitions_learned,
            wiki_learned: res.wiki_learned,
            coherence: res.coherence,
            streaming: false,
          };
          return next;
        });
      } catch (e) {
        setMessages(m => {
          const next = m.slice();
          next[next.length - 1] = {
            role: 'assistant',
            text: `⚠️ Error communicating with Phiano engine: ${e}`,
            streaming: false,
          };
          return next;
        });
      }
    }
    setBusy(false);
  }, [busy, onRefresh]);

  const send = useCallback(() => {
    sendText(input);
  }, [input, sendText]);

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%', maxWidth: '850px', margin: '0 auto' }}>
      <div style={{ flex: 1, overflow: 'auto', display: 'flex', flexDirection: 'column', gap: '1.25rem', padding: '1rem 0' }}>
        {messages.length === 0 && (
          <div className="card animate-in" style={{ textAlign: 'center', padding: '2.5rem 1.5rem', background: 'var(--bg-secondary)', border: '1px solid var(--border-color)', borderRadius: 'var(--radius-xl)' }}>
            <Sparkles size={36} style={{ color: 'var(--color-primary)', marginBottom: '0.75rem' }} />
            <h2 style={{ fontSize: '1.4rem', fontWeight: 600, marginBottom: '0.5rem', color: 'var(--text-primary)' }}>
              Chat with Phiano
            </h2>
            <p style={{ color: 'var(--text-secondary)', fontSize: '0.9rem', maxWidth: '520px', margin: '0 auto 1.5rem', lineHeight: 1.5 }}>
              Ask questions, teach new facts, or test multi-step reasoning. Phiano continuously tunes its phase manifold in real time with zero catastrophic forgetting.
            </p>
            
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem', maxWidth: '480px', margin: '0 auto', textAlign: 'left' }}>
              <div style={{ fontSize: '0.75rem', fontWeight: 600, textTransform: 'uppercase', letterSpacing: '0.05em', color: 'var(--text-secondary)', marginBottom: '0.25rem' }}>
                Try asking:
              </div>
              {SUGGESTIONS.map((s, idx) => (
                <button
                  key={idx}
                  onClick={() => sendText(s)}
                  disabled={busy}
                  style={{
                    background: 'var(--bg-primary)',
                    border: '1px solid var(--border-color)',
                    borderRadius: 'var(--radius-md)',
                    padding: '0.6rem 0.9rem',
                    fontSize: '0.825rem',
                    color: 'var(--text-primary)',
                    cursor: 'pointer',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '0.5rem',
                    transition: 'all 0.15s ease',
                  }}
                  onMouseEnter={e => (e.currentTarget.style.borderColor = 'var(--color-primary)')}
                  onMouseLeave={e => (e.currentTarget.style.borderColor = 'var(--border-color)')}
                >
                  <Zap size={14} style={{ color: 'var(--color-warning)', flexShrink: 0 }} />
                  <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{s}</span>
                </button>
              ))}
            </div>
          </div>
        )}

        {messages.map((m, i) => (
          <MessageBubble key={i} msg={m} />
        ))}
        <div ref={endRef} />
      </div>

      <div style={{ padding: '1rem 0', background: 'var(--bg-primary)' }}>
        <div style={{ display: 'flex', gap: '0.5rem', position: 'relative' }}>
          <textarea
            className="textarea"
            value={input}
            onChange={e => setInput(e.target.value)}
            onKeyDown={e => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                send();
              }
            }}
            placeholder="Ask a question, teach a fact, or give an instruction (Enter to send)..."
            style={{
              minHeight: '52px',
              maxHeight: '140px',
              borderRadius: 'var(--radius-lg)',
              padding: '0.85rem 1rem',
              fontSize: '0.9rem',
              resize: 'none',
              flex: 1,
            }}
            disabled={busy}
          />
          <button
            className="btn btn-primary"
            onClick={send}
            disabled={busy || !input.trim()}
            style={{
              alignSelf: 'flex-end',
              height: '52px',
              padding: '0 1.25rem',
              borderRadius: 'var(--radius-lg)',
              display: 'flex',
              alignItems: 'center',
              gap: '0.5rem',
              fontWeight: 600,
            }}
          >
            {busy ? <div className="spinner" /> : <Send size={16} />}
            <span>Send</span>
          </button>
        </div>
      </div>
    </div>
  );
}

function MessageBubble({ msg }: { msg: ChatMessage }) {
  const isUser = msg.role === 'user';

  return (
    <div
      className="animate-in"
      style={{
        alignSelf: isUser ? 'flex-end' : 'flex-start',
        maxWidth: isUser ? '80%' : '92%',
        display: 'flex',
        flexDirection: 'column',
        gap: '0.4rem',
      }}
    >
      <div
        style={{
          padding: '0.95rem 1.25rem',
          borderRadius: isUser ? '18px 18px 4px 18px' : '18px 18px 18px 4px',
          background: isUser ? 'var(--color-primary)' : 'var(--bg-secondary)',
          color: isUser ? 'var(--text-inverse)' : 'var(--text-primary)',
          border: isUser ? 'none' : '1px solid var(--border-color)',
          fontSize: '0.925rem',
          lineHeight: 1.6,
          boxShadow: isUser ? '0 3px 12px rgba(59, 130, 246, 0.25)' : '0 1px 6px rgba(0,0,0,0.06)',
        }}
      >
        {isUser ? (
          <div style={{ whiteSpace: 'pre-wrap' }}>{msg.text}</div>
        ) : (
          <>
            {msg.text ? <RichFormattedContent text={msg.text} /> : (
              <span style={{ color: 'var(--text-secondary)', fontStyle: 'italic' }}>
                decoding phase…
              </span>
            )}
            {msg.streaming && (
              <span style={{
                display: 'inline-block',
                width: '0.45rem',
                height: '1em',
                marginLeft: '0.2rem',
                background: 'var(--color-primary)',
                animation: 'pulse 0.9s ease-in-out infinite',
                verticalAlign: 'text-bottom',
              }} />
            )}
          </>
        )}
      </div>

      {!isUser && (
        <div style={{ display: 'flex', gap: '0.4rem', flexWrap: 'wrap', paddingLeft: '0.25rem' }}>
          {msg.speech_act && (
            <span
              style={{
                fontSize: '0.725rem',
                padding: '0.2rem 0.55rem',
                background: 'rgba(59, 130, 246, 0.1)',
                color: 'var(--color-primary)',
                borderRadius: 'var(--radius-full)',
                display: 'inline-flex',
                alignItems: 'center',
                gap: '0.25rem',
                fontWeight: 600,
              }}
            >
              <Brain size={12} />
              {msg.speech_act.toUpperCase()}
            </span>
          )}

          {msg.words_learned !== undefined && msg.words_learned > 0 && (
            <span
              style={{
                fontSize: '0.725rem',
                padding: '0.2rem 0.55rem',
                background: 'rgba(16, 185, 129, 0.12)',
                color: 'var(--color-success)',
                borderRadius: 'var(--radius-full)',
                display: 'inline-flex',
                alignItems: 'center',
                gap: '0.25rem',
                fontWeight: 600,
              }}
            >
              <CheckCircle2 size={12} />
              Learned {msg.words_learned} tokens
            </span>
          )}

          {msg.wiki_learned && (
            <span
              style={{
                fontSize: '0.725rem',
                padding: '0.2rem 0.55rem',
                background: 'rgba(245, 158, 11, 0.12)',
                color: 'var(--color-warning)',
                borderRadius: 'var(--radius-full)',
                display: 'inline-flex',
                alignItems: 'center',
                gap: '0.25rem',
                fontWeight: 600,
              }}
            >
              <BookOpen size={12} />
              Wiki: {msg.wiki_learned}
            </span>
          )}

          {msg.coherence !== undefined && (
            <span
              style={{
                fontSize: '0.725rem',
                padding: '0.2rem 0.55rem',
                background: 'rgba(139, 92, 246, 0.12)',
                color: '#8b5cf6',
                borderRadius: 'var(--radius-full)',
                display: 'inline-flex',
                alignItems: 'center',
                gap: '0.25rem',
                fontWeight: 600,
              }}
            >
              <ShieldCheck size={12} />
              Grounded: {(msg.coherence * 100).toFixed(0)}%
            </span>
          )}

          {msg.collective_phase !== undefined && (
            <span
              style={{
                fontSize: '0.725rem',
                padding: '0.2rem 0.55rem',
                background: 'rgba(14, 165, 233, 0.12)',
                color: '#0ea5e9',
                borderRadius: 'var(--radius-full)',
                display: 'inline-flex',
                alignItems: 'center',
                gap: '0.25rem',
                fontWeight: 600,
                fontFamily: 'ui-monospace, monospace',
              }}
            >
              <Waves size={12} />
              φ {msg.collective_phase.toFixed(3)}
            </span>
          )}

          {msg.resonance !== undefined && (
            <span
              style={{
                fontSize: '0.725rem',
                padding: '0.2rem 0.55rem',
                background: 'rgba(244, 63, 94, 0.12)',
                color: '#f43f5e',
                borderRadius: 'var(--radius-full)',
                display: 'inline-flex',
                alignItems: 'center',
                gap: '0.25rem',
                fontWeight: 600,
                fontFamily: 'ui-monospace, monospace',
              }}
            >
              R {(msg.resonance * 100).toFixed(0)}%
            </span>
          )}
        </div>
      )}
    </div>
  );
}

function RichFormattedContent({ text }: { text: string }) {
  const lines = text.split('\n');

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.45rem' }}>
      {lines.map((line, idx) => {
        const trimmed = line.trim();
        if (!trimmed) {
          return <div key={idx} style={{ height: '0.25rem' }} />;
        }

        // Heading 3: ### Title
        if (trimmed.startsWith('### ')) {
          const headingText = trimmed.replace(/^###\s+/, '');
          return (
            <h4
              key={idx}
              style={{
                fontSize: '1.05rem',
                fontWeight: 700,
                color: 'var(--color-primary)',
                margin: '0.35rem 0 0.1rem',
                display: 'flex',
                alignItems: 'center',
                gap: '0.35rem',
              }}
            >
              {parseInlineMarkdown(headingText)}
            </h4>
          );
        }

        // Bullet point: • or -
        if (trimmed.startsWith('• ') || trimmed.startsWith('- ')) {
          const bulletText = trimmed.replace(/^[•\-]\s+/, '');
          return (
            <div
              key={idx}
              style={{
                display: 'flex',
                alignItems: 'flex-start',
                gap: '0.5rem',
                paddingLeft: '0.25rem',
                fontSize: '0.9rem',
              }}
            >
              <span style={{ color: 'var(--color-primary)', fontWeight: 'bold', lineHeight: 1.4 }}>•</span>
              <div style={{ flex: 1 }}>{parseInlineMarkdown(bulletText)}</div>
            </div>
          );
        }

        // Metadata footer italic line: *Continuous Phase Manifold...*
        if (trimmed.startsWith('*') && trimmed.endsWith('*') && !trimmed.slice(1, -1).includes('\n')) {
          return (
            <div
              key={idx}
              style={{
                fontSize: '0.8rem',
                color: 'var(--text-secondary)',
                fontStyle: 'italic',
                paddingTop: '0.3rem',
                borderTop: '1px dashed var(--border-color)',
                marginTop: '0.25rem',
              }}
            >
              {trimmed.slice(1, -1)}
            </div>
          );
        }

        // Standard paragraph
        return (
          <p key={idx} style={{ margin: 0, fontSize: '0.925rem', lineHeight: 1.55 }}>
            {parseInlineMarkdown(trimmed)}
          </p>
        );
      })}
    </div>
  );
}

function parseInlineMarkdown(text: string): React.ReactNode[] {
  // Parses **bold**, *italic*, `code`
  const parts: React.ReactNode[] = [];
  const regex = /(\*\*[^*]+\*\*|\*[^*]+\*|`[^`]+`)/g;
  let lastIndex = 0;
  let match: RegExpExecArray | null;

  while ((match = regex.exec(text)) !== null) {
    if (match.index > lastIndex) {
      parts.push(text.substring(lastIndex, match.index));
    }
    const token = match[0];
    if (token.startsWith('**') && token.endsWith('**')) {
      parts.push(<strong key={match.index} style={{ fontWeight: 650, color: 'var(--text-primary)' }}>{token.slice(2, -2)}</strong>);
    } else if (token.startsWith('*') && token.endsWith('*')) {
      parts.push(<em key={match.index} style={{ fontStyle: 'italic', color: 'var(--text-secondary)' }}>{token.slice(1, -1)}</em>);
    } else if (token.startsWith('`') && token.endsWith('`')) {
      parts.push(
        <code
          key={match.index}
          style={{
            background: 'rgba(0,0,0,0.06)',
            padding: '0.1rem 0.35rem',
            borderRadius: '4px',
            fontFamily: 'monospace',
            fontSize: '0.85em',
          }}
        >
          {token.slice(1, -1)}
        </code>
      );
    }
    lastIndex = regex.lastIndex;
  }

  if (lastIndex < text.length) {
    parts.push(text.substring(lastIndex));
  }

  return parts.length > 0 ? parts : [text];
}

