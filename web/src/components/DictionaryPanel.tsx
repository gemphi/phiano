import { useState, useCallback, useMemo } from 'react';
import {
  Search, BookOpen, Sparkles, Compass, Globe, PenTool,
  ArrowRight, ArrowLeft, RotateCcw, Activity, Layers, Tag,
  Cpu, Zap, Volume2, ShieldCheck, ChevronRight
} from 'lucide-react';
import { fetchDefinition } from '../hooks/api/dictionary';
import type { DefineResult } from '../types';

interface DictionaryPanelProps {
  onRefresh: () => Promise<void>;
}

// Converts a phase angle in radians [0, 2π) to a rich HSL chromatic color
function phaseToHsl(phaseRad?: number, alpha = 1.0): string {
  if (phaseRad === undefined || phaseRad === null) return `rgba(99, 102, 241, ${alpha})`;
  const deg = ((phaseRad * 180 / Math.PI) % 360 + 360) % 360;
  return `hsla(${deg.toFixed(1)}, 85%, 62%, ${alpha})`;
}

function phaseToSecondaryHsl(phaseRad?: number, alpha = 1.0): string {
  if (phaseRad === undefined || phaseRad === null) return `rgba(56, 189, 248, ${alpha})`;
  const deg = (((phaseRad * 180 / Math.PI) + 40) % 360 + 360) % 360;
  return `hsla(${deg.toFixed(1)}, 80%, 55%, ${alpha})`;
}

// Component to render text with every single word clickable
function ClickableText({ text, onWordClick }: { text: string; onWordClick: (word: string) => void }) {
  const parts = useMemo(() => {
    return text.split(/(\s+|[.,;:"'!?()[\]{}\n-]+)/);
  }, [text]);

  return (
    <span>
      {parts.map((token, i) => {
        const isWord = /^[a-zA-Z0-9'-]+$/.test(token) && !/^[0-9]+$/.test(token) && token.length > 1;
        if (token === '\n') return <br key={i} />;
        if (!isWord) return <span key={i}>{token}</span>;

        return (
          <span
            key={i}
            onClick={(e) => {
              e.stopPropagation();
              onWordClick(token);
            }}
            className="clickable-word"
            title={`Click to inspect '${token}'`}
          >
            {token}
          </span>
        );
      })}
    </span>
  );
}

export function DictionaryPanel({ onRefresh }: DictionaryPanelProps) {
  const [searchWord, setSearchWord] = useState('money');
  const [dictResult, setDictResult] = useState<DefineResult | null>(null);
  const [busy, setBusy] = useState(false);
  const [history, setHistory] = useState<string[]>(['money']);
  const [historyIdx, setHistoryIdx] = useState<number>(0);

  // RiverFlow Story Composer State
  const [storyPrompt, setStoryPrompt] = useState('In the ancient temple of Juno Moneta the minted gold coins became legal tender for wealth and commerce');
  const [composedStory, setComposedStory] = useState<string>('');
  const [storySector, setStorySector] = useState<string>('');
  const [storyCoherence, setStoryCoherence] = useState<number | null>(null);
  const [storyBusy, setStoryBusy] = useState(false);

  const lookupDefinition = useCallback(async (wordToSearch?: string, addToHistory = true) => {
    const raw = (wordToSearch || searchWord).trim();
    const cleanWord = raw.replace(/[^a-zA-Z0-9'-]/g, '').toLowerCase();
    if (!cleanWord || busy) return;

    setBusy(true);
    try {
      const res = await fetchDefinition(cleanWord);
      setDictResult(res);
      setSearchWord(cleanWord);

      if (addToHistory) {
        setHistory(prev => {
          const newHist = prev.slice(0, historyIdx + 1);
          if (newHist[newHist.length - 1] !== cleanWord) {
            newHist.push(cleanWord);
          }
          return newHist;
        });
        setHistoryIdx(prev => prev + 1);
      }
    } catch (e) {
      setDictResult({
        word: cleanWord,
        definition: `Lookup error: ${e}`,
        source: 'Error',
        vocabulary: 0,
      });
    }
    setBusy(false);
  }, [searchWord, busy, historyIdx]);

  const goHistory = (delta: number) => {
    const targetIdx = historyIdx + delta;
    if (targetIdx >= 0 && targetIdx < history.length) {
      setHistoryIdx(targetIdx);
      const targetWord = history[targetIdx];
      lookupDefinition(targetWord, false);
    }
  };

  const composeStory = useCallback(async () => {
    if (!storyPrompt.trim() || storyBusy) return;
    setStoryBusy(true);
    try {
      const r = await fetch('/api/compose', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: storyPrompt }),
      });
      const data = await r.json();
      setComposedStory(data.text || '');
      setStorySector(`${data.winning_sector} (${data.winning_color})`);
      setStoryCoherence(data.coherence ?? 1.0);
      await onRefresh();
    } catch (e) {
      setComposedStory(`Composition error: ${e}`);
    }
    setStoryBusy(false);
  }, [storyPrompt, storyBusy, onRefresh]);

  // Active theme gradient colors derived dynamically from the current word's phase
  const primaryColor = useMemo(() => phaseToHsl(dictResult?.phase, 1.0), [dictResult?.phase]);
  const primaryGlow = useMemo(() => phaseToHsl(dictResult?.phase, 0.15), [dictResult?.phase]);
  const primaryBorder = useMemo(() => phaseToHsl(dictResult?.phase, 0.35), [dictResult?.phase]);
  const secondaryColor = useMemo(() => phaseToSecondaryHsl(dictResult?.phase, 1.0), [dictResult?.phase]);
  const phaseDeg = useMemo(() => {
    if (dictResult?.phase === undefined || dictResult?.phase === null) return 157.0;
    return (((dictResult.phase * 180 / Math.PI) % 360 + 360) % 360);
  }, [dictResult?.phase]);

  return (
    <div style={{ maxWidth: '1080px', margin: '0 auto', display: 'flex', flexDirection: 'column', gap: '1.75rem' }}>
      
      {/* 1. DYNAMIC CHROMATIC HERO BANNER (Morphs color with word phase) */}
      <div className="card animate-in" style={{
        background: `linear-gradient(135deg, ${primaryGlow} 0%, rgba(15, 23, 42, 0.7) 60%, ${phaseToSecondaryHsl(dictResult?.phase, 0.08)} 100%)`,
        borderColor: primaryBorder,
        boxShadow: `0 8px 32px 0 rgba(0, 0, 0, 0.37), 0 0 24px 0 ${phaseToHsl(dictResult?.phase, 0.12)}`,
        transition: 'all 0.4s cubic-bezier(0.16, 1, 0.3, 1)',
        padding: '1.75rem',
      }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '1rem' }}>
          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '0.6rem', marginBottom: '0.35rem' }}>
              <div style={{
                width: '14px', height: '14px', borderRadius: '50%',
                background: primaryColor,
                boxShadow: `0 0 12px ${primaryColor}`
              }} />
              <h2 style={{
                fontSize: '1.5rem', fontWeight: 800,
                background: `linear-gradient(135deg, #ffffff 40%, ${primaryColor} 100%)`,
                WebkitBackgroundClip: 'text',
                WebkitTextFillColor: 'transparent',
                letterSpacing: '-0.02em',
                margin: 0
              }}>
                Phiano Dynamic Harmonic Dictionary & Story Studio
              </h2>
            </div>
            <p style={{ fontSize: '0.9rem', color: 'var(--text-secondary)', maxWidth: '640px', lineHeight: 1.5 }}>
              Click <b>any word</b> anywhere to explore infinite semantic phase connections. Color gradients automatically adapt to each word's physical position on the complex manifold.
            </p>
          </div>

          {/* Real-Time Phase Coordinates Pill */}
          <div style={{
            display: 'flex', alignItems: 'center', gap: '0.85rem',
            background: 'rgba(0, 0, 0, 0.4)',
            border: `1px solid ${primaryBorder}`,
            padding: '0.6rem 1rem',
            borderRadius: '12px',
            backdropFilter: 'blur(10px)'
          }}>
            <div>
              <div style={{ fontSize: '0.7rem', color: 'var(--text-secondary)', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
                Manifold Phase
              </div>
              <div style={{ fontSize: '1.15rem', fontWeight: 800, color: primaryColor, fontVariantNumeric: 'tabular-nums' }}>
                {phaseDeg.toFixed(1)}°
              </div>
            </div>
            <div style={{ width: '1px', height: '28px', background: 'rgba(255, 255, 255, 0.1)' }} />
            <div>
              <div style={{ fontSize: '0.7rem', color: 'var(--text-secondary)', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
                Inertia Mass
              </div>
              <div style={{ fontSize: '1.15rem', fontWeight: 800, color: secondaryColor, fontVariantNumeric: 'tabular-nums' }}>
                {(dictResult?.amplitude ?? 1.0).toFixed(2)}
              </div>
            </div>
          </div>
        </div>

        {/* BREADCRUMB HISTORY BAR */}
        {history.length > 1 && (
          <div style={{
            display: 'flex', alignItems: 'center', gap: '0.4rem',
            marginTop: '1.25rem', paddingTop: '1rem',
            borderTop: '1px solid rgba(255, 255, 255, 0.08)',
            overflowX: 'auto', paddingBottom: '0.2rem'
          }}>
            <button
              className="btn-icon"
              onClick={() => goHistory(-1)}
              disabled={historyIdx <= 0}
              style={{ padding: '0.3rem 0.5rem', opacity: historyIdx <= 0 ? 0.3 : 1 }}
              title="Previous word in history"
            >
              <ArrowLeft size={14} />
            </button>
            <button
              className="btn-icon"
              onClick={() => goHistory(1)}
              disabled={historyIdx >= history.length - 1}
              style={{ padding: '0.3rem 0.5rem', opacity: historyIdx >= history.length - 1 ? 0.3 : 1 }}
              title="Next word in history"
            >
              <ArrowRight size={14} />
            </button>

            <span style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', marginLeft: '0.25rem', marginRight: '0.25rem' }}>
              Trail:
            </span>

            {history.map((hWord, idx) => (
              <button
                key={idx}
                onClick={() => { setHistoryIdx(idx); lookupDefinition(hWord, false); }}
                style={{
                  background: idx === historyIdx ? primaryColor : 'rgba(255, 255, 255, 0.05)',
                  color: idx === historyIdx ? '#000000' : 'var(--text-secondary)',
                  border: idx === historyIdx ? `1px solid ${primaryColor}` : '1px solid rgba(255, 255, 255, 0.08)',
                  fontWeight: idx === historyIdx ? 700 : 500,
                  fontSize: '0.75rem',
                  padding: '0.2rem 0.6rem',
                  borderRadius: '20px',
                  cursor: 'pointer',
                  whiteSpace: 'nowrap',
                  transition: 'all 0.15s ease'
                }}
              >
                {hWord}
              </button>
            ))}
          </div>
        )}
      </div>

      {/* 2. MAIN TWO-COLUMN WORKSPACE */}
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(480px, 1fr))', gap: '1.5rem' }}>
        
        {/* COLUMN 1: LIVE INTERACTIVE DICTIONARY INSPECTOR */}
        <div className="card animate-in" style={{
          display: 'flex', flexDirection: 'column',
          borderColor: primaryBorder,
          background: 'var(--bg-card)',
          position: 'relative'
        }}>
          <div className="card-title" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <span style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
              <BookOpen size={18} style={{ color: primaryColor }} />
              Live Interactive Dictionary
            </span>
            <span style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', fontWeight: 400 }}>
              Click ANY word to hop definitions
            </span>
          </div>

          {/* Search Bar */}
          <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <input
              type="text"
              className="input"
              style={{
                borderColor: primaryBorder,
                boxShadow: `0 0 0 1px ${primaryGlow}`,
                fontSize: '0.92rem'
              }}
              placeholder="Search or click any word (e.g. money, sex, juno, oscillator)..."
              value={searchWord}
              onChange={e => setSearchWord(e.target.value)}
              onKeyDown={e => e.key === 'Enter' && lookupDefinition()}
            />
            <button
              className="btn btn-primary"
              onClick={() => lookupDefinition()}
              disabled={busy || !searchWord.trim()}
              style={{
                background: `linear-gradient(135deg, ${primaryColor} 0%, ${secondaryColor} 100%)`,
                boxShadow: `0 4px 14px 0 ${primaryGlow}`,
                color: '#ffffff'
              }}
            >
              {busy ? <div className="spinner" /> : <Search size={16} />} Look Up
            </button>
          </div>

          {/* Quick Concept Chips */}
          <div style={{ display: 'flex', gap: '0.4rem', flexWrap: 'wrap', marginBottom: '1rem' }}>
            {['money', 'sex', 'oscillator', 'resonance', 'sesquipedalian', 'gold', 'currency', 'gravity', 'quantum', 'moneta'].map(w => (
              <button
                key={w}
                onClick={() => lookupDefinition(w)}
                style={{
                  background: searchWord.toLowerCase() === w ? primaryGlow : 'rgba(255, 255, 255, 0.03)',
                  border: searchWord.toLowerCase() === w ? `1px solid ${primaryColor}` : '1px solid var(--border-color)',
                  color: searchWord.toLowerCase() === w ? primaryColor : 'var(--text-secondary)',
                  padding: '0.22rem 0.6rem',
                  borderRadius: '12px',
                  fontSize: '0.75rem',
                  fontWeight: searchWord.toLowerCase() === w ? 600 : 400,
                  cursor: 'pointer',
                  transition: 'all 0.15s ease'
                }}
              >
                {w}
              </button>
            ))}
          </div>

          {/* Rich Definition Card */}
          <div style={{
            flex: 1,
            padding: '1.25rem',
            background: 'var(--bg-secondary)',
            borderRadius: '12px',
            border: `1px solid ${primaryBorder}`,
            minHeight: '340px',
            maxHeight: '520px',
            overflowY: 'auto'
          }}>
            {dictResult ? (
              <div>
                {/* Header info */}
                <div style={{
                  display: 'flex', justifyContent: 'space-between', alignItems: 'baseline',
                  borderBottom: `1px solid ${primaryBorder}`,
                  paddingBottom: '0.75rem', marginBottom: '1rem',
                  flexWrap: 'wrap', gap: '0.5rem'
                }}>
                  <div>
                    <span style={{ fontSize: '1.6rem', fontWeight: 800, color: primaryColor, textTransform: 'capitalize', letterSpacing: '-0.02em' }}>
                      {dictResult.word}
                    </span>
                    <span style={{ fontSize: '0.8rem', color: 'var(--text-secondary)', marginLeft: '0.75rem' }}>
                      <Globe size={12} style={{ verticalAlign: 'middle', marginRight: '0.3rem' }} />
                      {dictResult.source}
                    </span>
                  </div>

                  <span className="badge" style={{ background: primaryGlow, color: primaryColor, borderColor: primaryBorder }}>
                    <Compass size={12} /> {phaseDeg.toFixed(1)}° Phase
                  </span>
                </div>

                {/* Clickable definition text */}
                <div style={{
                  fontSize: '0.94rem',
                  lineHeight: '1.75',
                  color: 'var(--text-primary)',
                  letterSpacing: '0.01em'
                }}>
                  <ClickableText
                    text={dictResult.definition}
                    onWordClick={(w) => lookupDefinition(w)}
                  />
                </div>
              </div>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '100%', color: 'var(--text-secondary)', textAlign: 'center', padding: '2rem' }}>
                <Search size={36} style={{ color: primaryColor, opacity: 0.4, marginBottom: '0.75rem' }} />
                <p style={{ fontSize: '0.95rem' }}>Search any word or click any term in the story to view its rich dictionary definition and phase resonance.</p>
              </div>
            )}
          </div>
        </div>

        {/* COLUMN 2: RIVERFLOW HARMONIC STORY COMPOSER */}
        <div className="card animate-in" style={{
          display: 'flex', flexDirection: 'column',
          borderColor: 'var(--border-color)',
          background: 'var(--bg-card)'
        }}>
          <div className="card-title" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <span style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
              <PenTool size={18} style={{ color: '#a855f7' }} />
              RiverFlow Story Composer
            </span>
            <span style={{ fontSize: '0.75rem', color: 'var(--text-secondary)' }}>
              Non-Linear Phase Traversal
            </span>
          </div>

          <p style={{ fontSize: '0.85rem', color: 'var(--text-secondary)', marginBottom: '0.75rem' }}>
            Provide opening keywords or a topic prompt. Phiano composes harmonic narrative text across neighboring phase sectors:
          </p>

          <textarea
            className="textarea"
            value={storyPrompt}
            onChange={e => setStoryPrompt(e.target.value)}
            placeholder="Enter opening story prompt..."
            style={{ minHeight: '85px', marginBottom: '0.75rem', fontSize: '0.88rem' }}
          />

          <button
            className="btn btn-primary"
            onClick={composeStory}
            disabled={storyBusy || !storyPrompt.trim()}
            style={{
              background: 'linear-gradient(135deg, #9333ea 0%, #6366f1 100%)',
              boxShadow: '0 4px 14px 0 rgba(147, 51, 234, 0.3)',
              marginBottom: '1rem'
            }}
          >
            {storyBusy ? <div className="spinner" /> : <Sparkles size={16} />} Compose Harmonic Story
          </button>

          {/* Generated Story Box */}
          <div style={{
            flex: 1,
            padding: '1.25rem',
            background: 'var(--bg-secondary)',
            borderRadius: '12px',
            border: '1px solid var(--border-color)',
            minHeight: '340px',
            maxHeight: '520px',
            display: 'flex',
            flexDirection: 'column'
          }}>
            <div style={{
              display: 'flex', justifyContent: 'space-between', alignItems: 'center',
              borderBottom: '1px solid rgba(255, 255, 255, 0.08)',
              paddingBottom: '0.6rem', marginBottom: '0.85rem'
            }}>
              <span style={{ fontSize: '0.88rem', fontWeight: 600, color: 'var(--text-primary)' }}>
                Composed Narrative (Click any word to inspect):
              </span>
              {storySector && (
                <span className="badge" style={{ background: 'rgba(168, 85, 247, 0.15)', color: '#c084fc', borderColor: 'rgba(168, 85, 247, 0.3)' }}>
                  Sector {storySector}
                </span>
              )}
            </div>

            {composedStory ? (
              <div style={{
                flex: 1,
                overflowY: 'auto',
                fontSize: '0.94rem',
                lineHeight: '1.8',
                color: 'var(--text-primary)',
                letterSpacing: '0.01em'
              }}>
                <ClickableText
                  text={composedStory}
                  onWordClick={(w) => lookupDefinition(w)}
                />
              </div>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '100%', color: 'var(--text-secondary)', textAlign: 'center', padding: '2rem' }}>
                <PenTool size={36} style={{ color: '#a855f7', opacity: 0.4, marginBottom: '0.75rem' }} />
                <p style={{ fontSize: '0.95rem' }}>Click "Compose Harmonic Story" to generate narrative text. You can click any word in the generated story to hop to its dictionary definition!</p>
              </div>
            )}
          </div>
        </div>

      </div>

    </div>
  );
}
