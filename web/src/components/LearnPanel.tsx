import { useState, useCallback } from 'react';
import { BookOpen, Layers, Search, Sparkles, Compass, CheckCircle2, Globe, Volume2 } from 'lucide-react';
import { learnText, learnMulti, fetchDefinition } from '../hooks/useApi';
import type { LearnResult, MultiLearnResult, DefineResult } from '../types';

interface LearnPanelProps {
  onRefresh: () => Promise<void>;
}

export function LearnPanel({ onRefresh }: LearnPanelProps) {
  const [text, setText] = useState('');
  const [epochs, setEpochs] = useState(5);
  const [warmup, setWarmup] = useState(2);
  const [result, setResult] = useState<LearnResult | null>(null);
  const [multiResult, setMultiResult] = useState<MultiLearnResult | null>(null);
  const [busy, setBusy] = useState(false);

  // Dictionary Lookup State
  const [searchWord, setSearchWord] = useState('');
  const [dictResult, setDictResult] = useState<DefineResult | null>(null);
  const [dictBusy, setDictBusy] = useState(false);

  const doLearn = useCallback(async () => {
    if (!text.trim() || busy) return;
    setBusy(true);
    try {
      const r = await learnText(text);
      setResult(r);
      await onRefresh();
    } catch (e) {
      setResult({ tokens: 0, vocabulary: 0, message: `Error: ${e}` });
    }
    setBusy(false);
  }, [text, busy, onRefresh]);

  const doMulti = useCallback(async () => {
    if (!text.trim() || busy) return;
    setBusy(true);
    try {
      const r = await learnMulti(text, epochs, warmup);
      setMultiResult(r);
      await onRefresh();
    } catch (e) {
      setMultiResult({ epochs: 0, tokens: 0, converged: false, vocabulary: 0 });
    }
    setBusy(false);
  }, [text, epochs, warmup, busy, onRefresh]);

  const lookupDefinition = useCallback(async (wordToSearch?: string) => {
    const query = wordToSearch || searchWord;
    if (!query.trim() || dictBusy) return;
    setDictBusy(true);
    try {
      const res = await fetchDefinition(query.trim().toLowerCase());
      setDictResult(res);
    } catch (e) {
      setDictResult({
        word: query,
        definition: `Failed to lookup: ${e}`,
        source: 'Error',
        vocabulary: 0,
      });
    }
    setDictBusy(false);
  }, [searchWord, dictBusy]);

  return (
    <div style={{ maxWidth: '840px', margin: '0 auto', display: 'flex', flexDirection: 'column', gap: '1.5rem' }}>
      
      {/* CARD 1: Teach Phiano Anything */}
      <div className="card animate-in">
        <div className="card-title" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <span>
            <BookOpen size={18} style={{ verticalAlign: 'middle', marginRight: '0.5rem', color: '#6366f1' }} />
            Teach Phiano Anything (Episodic Harmonic Entrainment)
          </span>
          <span style={{ fontSize: '0.75rem', color: 'var(--text-secondary)' }}>
            Zero Duplicates · Continuous Phase Tuning
          </span>
        </div>

        <p style={{ fontSize: '0.85rem', color: 'var(--text-secondary)', marginBottom: '0.75rem' }}>
          Paste any sentence, definition, legal clause, article, or story. Phiano tunes existing word phasors or creates unique entries on its non-linear Kuramoto manifold.
        </p>

        <textarea
          className="textarea"
          value={text}
          onChange={e => setText(e.target.value)}
          placeholder="Enter or paste text to teach Phiano (e.g. 'money (mŭn′ē) n. 1. A medium that can be exchanged for goods and services...')"
          style={{ minHeight: '120px', fontFamily: 'var(--font-mono, monospace)', fontSize: '0.88rem' }}
        />

        <div style={{ display: 'flex', gap: '0.75rem', marginTop: '1rem', flexWrap: 'wrap', alignItems: 'center' }}>
          <button className="btn btn-primary" onClick={doLearn} disabled={busy || !text.trim()}>
            {busy ? <div className="spinner" /> : <Sparkles size={16} />} Teach Online (1 Epoch)
          </button>
          
          <button className="btn btn-ghost" onClick={doMulti} disabled={busy || !text.trim()}>
            <Layers size={16} /> Multi-Epoch Deep Entrain ({epochs} Epochs)
          </button>

          <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'center', marginLeft: 'auto' }}>
            <label style={{ fontSize: '0.8rem', color: 'var(--text-secondary)' }}>Epochs:</label>
            <input
              type="number"
              className="input"
              style={{ width: '65px', padding: '0.35rem 0.5rem' }}
              value={epochs}
              onChange={e => setEpochs(Math.max(1, +e.target.value))}
            />
          </div>
        </div>

        {/* Training Feedback */}
        {result && (
          <div style={{ marginTop: '1.2rem', padding: '0.85rem', background: 'rgba(99, 102, 241, 0.08)', borderRadius: '8px', border: '1px solid rgba(99, 102, 241, 0.2)' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginBottom: '0.5rem', color: '#818cf8', fontWeight: 600 }}>
              <CheckCircle2 size={16} /> Online Learning Complete
            </div>
            <div className="metric-row"><span className="metric-label">Tokens Ingested</span><span className="metric-value">{result.tokens}</span></div>
            <div className="metric-row"><span className="metric-label">Total Unique Lexicon</span><span className="metric-value">{result.vocabulary.toLocaleString()} words</span></div>
          </div>
        )}

        {multiResult && (
          <div style={{ marginTop: '1.2rem', padding: '0.85rem', background: 'rgba(16, 185, 129, 0.08)', borderRadius: '8px', border: '1px solid rgba(16, 185, 129, 0.2)' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginBottom: '0.5rem', color: '#34d399', fontWeight: 600 }}>
              <CheckCircle2 size={16} /> Multi-Epoch Harmonic Convergence
            </div>
            <div className="metric-row"><span className="metric-label">Epochs Swept</span><span className="metric-value">{multiResult.epochs}</span></div>
            <div className="metric-row"><span className="metric-label">Tokens Trained</span><span className="metric-value">{multiResult.tokens}</span></div>
            <div className="metric-row"><span className="metric-label">Manifold Converged</span><span className="metric-value" style={{ color: multiResult.converged ? '#34d399' : '#fbbf24' }}>{multiResult.converged ? 'Yes (Attractor Locked)' : 'Phase Distributed'}</span></div>
            <div className="metric-row"><span className="metric-label">Total Vocabulary</span><span className="metric-value">{multiResult.vocabulary.toLocaleString()} words</span></div>
          </div>
        )}
      </div>

      {/* CARD 2: Interactive Dictionary Inspector (Click & Lookup) */}
      <div className="card animate-in">
        <div className="card-title" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <span>
            <Search size={18} style={{ verticalAlign: 'middle', marginRight: '0.5rem', color: '#38bdf8' }} />
            Dictionary Inspector & Word Phase Coordinates
          </span>
          <span style={{ fontSize: '0.75rem', color: 'var(--text-secondary)' }}>
            Free Dictionary API & 102K Offline Chunks
          </span>
        </div>

        <p style={{ fontSize: '0.85rem', color: 'var(--text-secondary)', marginBottom: '0.75rem' }}>
          Search any word in the English language to view its pronunciation, parts of speech, sub-senses, and physical phase coordinates ($\phi, A$) on the circle.
        </p>

        <div style={{ display: 'flex', gap: '0.75rem' }}>
          <input
            type="text"
            className="input"
            style={{ flex: 1 }}
            placeholder="Type any word (e.g. 'money', 'sex', 'oscillator', 'quantum', 'juno')..."
            value={searchWord}
            onChange={e => setSearchWord(e.target.value)}
            onKeyDown={e => e.key === 'Enter' && lookupDefinition()}
          />
          <button className="btn btn-primary" onClick={() => lookupDefinition()} disabled={dictBusy || !searchWord.trim()}>
            {dictBusy ? <div className="spinner" /> : <Search size={16} />} Look Up
          </button>
        </div>

        {/* Preset quick test chips */}
        <div style={{ display: 'flex', gap: '0.5rem', marginTop: '0.75rem', flexWrap: 'wrap' }}>
          <span style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', alignSelf: 'center' }}>Try:</span>
          {['money', 'sex', 'oscillator', 'resonance', 'sesquipedalian', 'gravity'].map(w => (
            <button
              key={w}
              onClick={() => { setSearchWord(w); lookupDefinition(w); }}
              style={{
                background: 'rgba(255,255,255,0.05)',
                border: '1px solid rgba(255,255,255,0.1)',
                color: 'var(--text-primary)',
                padding: '0.2rem 0.55rem',
                borderRadius: '12px',
                fontSize: '0.75rem',
                cursor: 'pointer'
              }}
            >
              {w}
            </button>
          ))}
        </div>

        {/* Dictionary Definition Output */}
        {dictResult && (
          <div style={{
            marginTop: '1.25rem',
            padding: '1.25rem',
            background: 'var(--bg-secondary, rgba(15, 23, 42, 0.6))',
            borderRadius: '10px',
            border: '1px solid rgba(56, 189, 248, 0.25)'
          }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', borderBottom: '1px solid rgba(255,255,255,0.1)', paddingBottom: '0.6rem', marginBottom: '0.75rem' }}>
              <div style={{ display: 'flex', alignItems: 'baseline', gap: '0.75rem' }}>
                <span style={{ fontSize: '1.25rem', fontWeight: 700, color: '#38bdf8', textTransform: 'capitalize' }}>
                  {dictResult.word}
                </span>
                <span style={{ fontSize: '0.8rem', color: 'var(--text-secondary)' }}>
                  <Globe size={12} style={{ verticalAlign: 'middle', marginRight: '0.25rem' }} />
                  {dictResult.source}
                </span>
              </div>

              {/* Polar Coordinates */}
              {dictResult.phase !== undefined && dictResult.phase !== null && (
                <div style={{ display: 'flex', gap: '0.75rem', fontSize: '0.75rem', background: 'rgba(56, 189, 248, 0.1)', padding: '0.25rem 0.6rem', borderRadius: '6px' }}>
                  <span><Compass size={12} style={{ verticalAlign: 'middle', marginRight: '0.2rem' }} /> Phase: <b>{((dictResult.phase * 180 / Math.PI) % 360).toFixed(1)}°</b></span>
                  <span>Amp (Mass): <b>{(dictResult.amplitude ?? 1.0).toFixed(2)}</b></span>
                </div>
              )}
            </div>

            {/* Formatted Definition Text */}
            <div style={{
              whiteSpace: 'pre-wrap',
              fontSize: '0.9rem',
              lineHeight: '1.6',
              color: 'var(--text-primary)',
              fontFamily: 'inherit'
            }}>
              {dictResult.definition}
            </div>
          </div>
        )}
      </div>

    </div>
  );
}
