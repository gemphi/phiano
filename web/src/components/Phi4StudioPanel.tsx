import { useState, useEffect } from 'react';
import { Cpu, Terminal, Network, GitBranch, Play, RefreshCw, Layers, CheckCircle2, ArrowRight, Zap } from 'lucide-react';
import type { InstructResponse, ReasoningResponse, LayersResponse, Phi4LearnResponse, SyntheticResponse } from '../types';

interface Phi4StudioPanelProps {
  onRefresh: () => Promise<void>;
}

export function Phi4StudioPanel({ onRefresh }: Phi4StudioPanelProps) {
  const [activeTab, setActiveTab] = useState<'instruct' | 'reason' | 'layers' | 'model'>('instruct');

  // Instruction State
  const [instructPrompt, setInstructPrompt] = useState('write code for rust mutex and thread safety');
  const [instructResult, setInstructResult] = useState<InstructResponse | null>(null);
  const [instructLoading, setInstructLoading] = useState(false);

  // Reasoning State
  const [reasonProblem, setReasonProblem] = useState('ownership borrowing lifetime thread concurrency');
  const [reasonResult, setReasonResult] = useState<ReasoningResponse | null>(null);
  const [reasonLoading, setReasonLoading] = useState(false);

  // Layers State
  const [layersData, setLayersData] = useState<LayersResponse | null>(null);
  const [layersLoading, setLayersLoading] = useState(false);

  // Model Ingestion State
  const [phi4Loading, setPhi4Loading] = useState(false);
  const [phi4Result, setPhi4Result] = useState<Phi4LearnResponse | null>(null);
  const [synthLoading, setSynthLoading] = useState(false);
  const [synthResult, setSynthResult] = useState<SyntheticResponse | null>(null);

  const fetchLayers = async () => {
    setLayersLoading(true);
    try {
      const res = await fetch('/api/layers');
      if (res.ok) {
        const data = await res.json();
        setLayersData(data);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLayersLoading(false);
    }
  };

  useEffect(() => {
    if (activeTab === 'layers') {
      fetchLayers();
    }
  }, [activeTab]);

  const handleRunInstruct = async () => {
    if (!instructPrompt.trim()) return;
    setInstructLoading(true);
    try {
      const res = await fetch('/api/instruct', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: instructPrompt }),
      });
      if (res.ok) {
        const data = await res.json();
        setInstructResult(data);
        onRefresh();
      }
    } catch (e) {
      console.error(e);
    } finally {
      setInstructLoading(false);
    }
  };

  const handleRunReasoning = async () => {
    if (!reasonProblem.trim()) return;
    setReasonLoading(true);
    try {
      const res = await fetch('/api/reason', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: reasonProblem }),
      });
      if (res.ok) {
        const data = await res.json();
        setReasonResult(data);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setReasonLoading(false);
    }
  };

  const handleLearnPhi4 = async () => {
    setPhi4Loading(true);
    try {
      const res = await fetch('/api/phi4/learn', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: '' }),
      });
      if (res.ok) {
        const data = await res.json();
        setPhi4Result(data);
        onRefresh();
      }
    } catch (e) {
      console.error(e);
    } finally {
      setPhi4Loading(false);
    }
  };

  const handleRunSynthetic = async () => {
    setSynthLoading(true);
    try {
      const res = await fetch('/api/synthetic', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: '' }),
      });
      if (res.ok) {
        const data = await res.json();
        setSynthResult(data);
        onRefresh();
      }
    } catch (e) {
      console.error(e);
    } finally {
      setSynthLoading(false);
    }
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '1.5rem', maxWidth: 1100, margin: '0 auto' }}>
      {/* Studio Header */}
      <div style={{
        padding: '1.5rem 2rem',
        borderRadius: 'var(--radius-lg)',
        background: 'linear-gradient(135deg, rgba(99, 102, 241, 0.12), rgba(168, 85, 247, 0.08))',
        border: '1px solid rgba(99, 102, 241, 0.25)',
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
      }}>
        <div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem', marginBottom: '0.25rem' }}>
            <Cpu size={24} color="#818cf8" />
            <h2 style={{ fontSize: '1.5rem', fontWeight: 700, color: 'var(--text-primary)' }}>
              Phi-4 Reasoning & Architecture Studio
            </h2>
          </div>
          <p style={{ fontSize: '0.875rem', color: 'var(--text-secondary)' }}>
            Phase-space oscillator reasoning, persona instruction following, 4-layer hierarchical abstraction, and Phi-4 knowledge ingestion.
          </p>
        </div>
      </div>

      {/* Tabs */}
      <div style={{ display: 'flex', gap: '0.5rem', borderBottom: '1px solid var(--border-color)', paddingBottom: '0.5rem' }}>
        <button
          onClick={() => setActiveTab('instruct')}
          style={{
            display: 'flex', alignItems: 'center', gap: '0.5rem',
            padding: '0.625rem 1.25rem', borderRadius: 'var(--radius-md)',
            background: activeTab === 'instruct' ? 'var(--color-primary-light)' : 'transparent',
            color: activeTab === 'instruct' ? 'var(--color-primary)' : 'var(--text-secondary)',
            fontWeight: 600, border: 'none', cursor: 'pointer',
          }}
        >
          <Terminal size={18} />
          Instruction Execution (Phase 4)
        </button>

        <button
          onClick={() => setActiveTab('reason')}
          style={{
            display: 'flex', alignItems: 'center', gap: '0.5rem',
            padding: '0.625rem 1.25rem', borderRadius: 'var(--radius-md)',
            background: activeTab === 'reason' ? 'var(--color-primary-light)' : 'transparent',
            color: activeTab === 'reason' ? 'var(--color-primary)' : 'var(--text-secondary)',
            fontWeight: 600, border: 'none', cursor: 'pointer',
          }}
        >
          <GitBranch size={18} />
          Reasoning Pathfinding (Phase 6)
        </button>

        <button
          onClick={() => setActiveTab('layers')}
          style={{
            display: 'flex', alignItems: 'center', gap: '0.5rem',
            padding: '0.625rem 1.25rem', borderRadius: 'var(--radius-md)',
            background: activeTab === 'layers' ? 'var(--color-primary-light)' : 'transparent',
            color: activeTab === 'layers' ? 'var(--color-primary)' : 'var(--text-secondary)',
            fontWeight: 600, border: 'none', cursor: 'pointer',
          }}
        >
          <Layers size={18} />
          Hierarchical Depth (Phase 3)
        </button>

        <button
          onClick={() => setActiveTab('model')}
          style={{
            display: 'flex', alignItems: 'center', gap: '0.5rem',
            padding: '0.625rem 1.25rem', borderRadius: 'var(--radius-md)',
            background: activeTab === 'model' ? 'var(--color-primary-light)' : 'transparent',
            color: activeTab === 'model' ? 'var(--color-primary)' : 'var(--text-secondary)',
            fontWeight: 600, border: 'none', cursor: 'pointer',
          }}
        >
          <Network size={18} />
          Phi-4 Ingestion & Synthetic (Phase 5)
        </button>
      </div>

      {/* Tab 1: Instruction Execution */}
      {activeTab === 'instruct' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
          <div style={{ background: 'var(--bg-card)', padding: '1.5rem', borderRadius: 'var(--radius-lg)', border: '1px solid var(--border-color)' }}>
            <label style={{ display: 'block', fontWeight: 600, marginBottom: '0.5rem', color: 'var(--text-primary)' }}>
              Instruction Prompt:
            </label>
            <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.75rem', flexWrap: 'wrap' }}>
              {[
                'write code for rust mutex channel thread safety',
                'explain ownership and borrowing in rust',
                'write a haiku about hockey and ice',
                'benchmark and analyze phiano performance',
              ].map((sample) => (
                <button
                  key={sample}
                  onClick={() => setInstructPrompt(sample)}
                  style={{
                    fontSize: '0.75rem', padding: '0.25rem 0.625rem', borderRadius: '999px',
                    background: 'var(--bg-secondary)', border: '1px solid var(--border-color)',
                    color: 'var(--text-secondary)', cursor: 'pointer',
                  }}
                >
                  {sample}
                </button>
              ))}
            </div>

            <textarea
              rows={3}
              value={instructPrompt}
              onChange={(e) => setInstructPrompt(e.target.value)}
              style={{
                width: '100%', padding: '0.75rem', borderRadius: 'var(--radius-md)',
                background: 'var(--bg-input)', border: '1px solid var(--border-color)',
                color: 'var(--text-primary)', fontFamily: 'var(--font-mono)', fontSize: '0.875rem',
                resize: 'vertical',
              }}
            />

            <div style={{ display: 'flex', justifyContent: 'flex-end', marginTop: '0.75rem' }}>
              <button
                onClick={handleRunInstruct}
                disabled={instructLoading}
                className="btn-primary"
                style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}
              >
                {instructLoading ? <RefreshCw className="spin" size={16} /> : <Play size={16} />}
                Execute Instruction
              </button>
            </div>
          </div>

          {instructResult && (
            <div style={{ background: 'var(--bg-card)', padding: '1.5rem', borderRadius: 'var(--radius-lg)', border: '1px solid var(--border-color)' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '0.75rem' }}>
                <span style={{ fontSize: '0.875rem', fontWeight: 600, color: 'var(--color-primary)' }}>
                  Execution Response
                </span>
                <span style={{ fontSize: '0.75rem', color: 'var(--text-secondary)' }}>
                  Vocabulary: {instructResult.vocabulary} words
                </span>
              </div>
              <pre style={{
                background: 'var(--bg-secondary)', padding: '1rem', borderRadius: 'var(--radius-md)',
                color: 'var(--text-primary)', whiteSpace: 'pre-wrap', fontFamily: 'var(--font-mono)', fontSize: '0.875rem',
              }}>
                {instructResult.output}
              </pre>
            </div>
          )}
        </div>
      )}

      {/* Tab 2: Reasoning Pathfinding */}
      {activeTab === 'reason' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
          <div style={{ background: 'var(--bg-card)', padding: '1.5rem', borderRadius: 'var(--radius-lg)', border: '1px solid var(--border-color)' }}>
            <label style={{ display: 'block', fontWeight: 600, marginBottom: '0.5rem', color: 'var(--text-primary)' }}>
              Problem / Conceptual Chain:
            </label>
            <input
              type="text"
              value={reasonProblem}
              onChange={(e) => setReasonProblem(e.target.value)}
              style={{
                width: '100%', padding: '0.75rem', borderRadius: 'var(--radius-md)',
                background: 'var(--bg-input)', border: '1px solid var(--border-color)',
                color: 'var(--text-primary)', fontFamily: 'var(--font-mono)', fontSize: '0.875rem',
              }}
            />
            <div style={{ display: 'flex', justifyContent: 'flex-end', marginTop: '0.75rem' }}>
              <button
                onClick={handleRunReasoning}
                disabled={reasonLoading}
                className="btn-primary"
                style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}
              >
                {reasonLoading ? <RefreshCw className="spin" size={16} /> : <Zap size={16} />}
                Solve via Phase-Space Pathfinding
              </button>
            </div>
          </div>

          {reasonResult && (
            <div style={{ background: 'var(--bg-card)', padding: '1.5rem', borderRadius: 'var(--radius-lg)', border: '1px solid var(--border-color)' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginBottom: '1rem' }}>
                <CheckCircle2 color={reasonResult.converged ? '#10b981' : '#f59e0b'} size={20} />
                <span style={{ fontWeight: 600, color: 'var(--text-primary)' }}>
                  {reasonResult.converged ? `Converged in ${reasonResult.steps_count} Traversal Steps` : `Traversed ${reasonResult.steps_count} Steps`}
                </span>
              </div>

              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', flexWrap: 'wrap', padding: '1rem', background: 'var(--bg-secondary)', borderRadius: 'var(--radius-md)' }}>
                {reasonResult.final_answer.split(' -> ').map((node, i, arr) => (
                  <div key={i} style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                    <span style={{
                      padding: '0.375rem 0.75rem', borderRadius: 'var(--radius-sm)',
                      background: 'var(--color-primary-light)', color: 'var(--color-primary)',
                      fontWeight: 600, fontSize: '0.875rem',
                    }}>
                      {node}
                    </span>
                    {i < arr.length - 1 && <ArrowRight size={16} color="var(--text-secondary)" />}
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      )}

      {/* Tab 3: Hierarchical Depth */}
      {activeTab === 'layers' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(240px, 1fr))', gap: '1rem' }}>
            {layersData?.layer_summaries.map((layer) => {
              const titles = ['Surface Lexicon', 'Concept Clusters', 'Domain Sectors', 'Meta-Patterns'];
              return (
                <div
                  key={layer.level}
                  style={{
                    background: 'var(--bg-card)', padding: '1.5rem', borderRadius: 'var(--radius-lg)',
                    border: '1px solid var(--border-color)', display: 'flex', flexDirection: 'column', gap: '0.5rem',
                  }}
                >
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <span style={{ fontSize: '0.75rem', textTransform: 'uppercase', color: 'var(--color-primary)', fontWeight: 700 }}>
                      Layer {layer.level}
                    </span>
                    <span style={{ fontSize: '0.75rem', padding: '0.125rem 0.5rem', borderRadius: '999px', background: 'var(--bg-secondary)' }}>
                      {layer.sector_count} Sectors
                    </span>
                  </div>
                  <h3 style={{ fontSize: '1.125rem', fontWeight: 600, color: 'var(--text-primary)' }}>
                    {titles[layer.level] || `Layer ${layer.level}`}
                  </h3>
                  <div style={{ marginTop: 'auto', paddingTop: '0.75rem', borderTop: '1px solid var(--border-color)', fontSize: '0.875rem', color: 'var(--text-secondary)' }}>
                    Active Centroids: <strong style={{ color: 'var(--text-primary)' }}>{layer.clusters_count}</strong>
                  </div>
                </div>
              );
            })}
          </div>

          <div style={{ display: 'flex', justifyContent: 'center', marginTop: '0.5rem' }}>
            <button
              onClick={fetchLayers}
              disabled={layersLoading}
              className="btn-secondary"
              style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}
            >
              <RefreshCw className={layersLoading ? 'spin' : ''} size={16} />
              Recompute Hierarchical Centroids
            </button>
          </div>
        </div>
      )}

      {/* Tab 4: Phi-4 & Synthetic Ingestion */}
      {activeTab === 'model' && (
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '1.5rem' }}>
          {/* Phi-4 Model Ingestion */}
          <div style={{ background: 'var(--bg-card)', padding: '1.5rem', borderRadius: 'var(--radius-lg)', border: '1px solid var(--border-color)' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginBottom: '0.75rem' }}>
              <Network size={20} color="var(--color-primary)" />
              <h3 style={{ fontSize: '1.125rem', fontWeight: 600, color: 'var(--text-primary)' }}>
                Phi-4 Model References
              </h3>
            </div>
            <p style={{ fontSize: '0.875rem', color: 'var(--text-secondary)', marginBottom: '1.25rem', lineHeight: 1.5 }}>
              Ingests 100,352 tiktoken vocabulary tokens, 5,000+ BPE merge pairs, and multi-modal technical reasoning definitions directly from the <code>refs/Phi-4-multimodal-instruct/</code> repository.
            </p>

            <button
              onClick={handleLearnPhi4}
              disabled={phi4Loading}
              className="btn-primary"
              style={{ width: '100%', display: 'flex', justifyContent: 'center', alignItems: 'center', gap: '0.5rem' }}
            >
              {phi4Loading ? <RefreshCw className="spin" size={16} /> : <Play size={16} />}
              Ingest & Learn Phi-4 Weights
            </button>

            {phi4Result && (
              <div style={{ marginTop: '1rem', padding: '0.75rem', background: 'var(--bg-secondary)', borderRadius: 'var(--radius-md)', fontSize: '0.8125rem', lineHeight: 1.6 }}>
                <div>✓ Tokens Loaded: <strong>{phi4Result.vocab_tokens_loaded}</strong></div>
                <div>✓ Merges Coupled: <strong>{phi4Result.merges_trained}</strong></div>
                <div>✓ Doc Chords Ingested: <strong>{phi4Result.doc_sentences_trained}</strong></div>
                <div>✓ Total Vocabulary: <strong>{phi4Result.final_vocabulary_size}</strong> words</div>
              </div>
            )}
          </div>

          {/* Synthetic Data Curriculum */}
          <div style={{ background: 'var(--bg-card)', padding: '1.5rem', borderRadius: 'var(--radius-lg)', border: '1px solid var(--border-color)' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginBottom: '0.75rem' }}>
              <Zap size={20} color="#a855f7" />
              <h3 style={{ fontSize: '1.125rem', fontWeight: 600, color: 'var(--text-primary)' }}>
                Self-Curated Synthetic Curriculum
              </h3>
            </div>
            <p style={{ fontSize: '0.875rem', color: 'var(--text-secondary)', marginBottom: '1.25rem', lineHeight: 1.5 }}>
              Generates in-memory synthetic sentence variations, measures them with the evaluator for coherence and resonance, and retrains the manifold on high-quality pairs.
            </p>

            <button
              onClick={handleRunSynthetic}
              disabled={synthLoading}
              className="btn-primary"
              style={{ width: '100%', display: 'flex', justifyContent: 'center', alignItems: 'center', gap: '0.5rem', background: 'linear-gradient(135deg, #a855f7, #6366f1)' }}
            >
              {synthLoading ? <RefreshCw className="spin" size={16} /> : <Play size={16} />}
              Run Synthetic Curriculum Pipeline
            </button>

            {synthResult && (
              <div style={{ marginTop: '1rem', padding: '0.75rem', background: 'var(--bg-secondary)', borderRadius: 'var(--radius-md)', fontSize: '0.8125rem', lineHeight: 1.6 }}>
                <div>✓ High-Quality Accepted: <strong>{synthResult.accepted_count}</strong> sentences</div>
                <div>✓ Total Vocabulary: <strong>{synthResult.vocabulary}</strong> words</div>
                <div style={{ color: 'var(--color-primary)', marginTop: '0.25rem' }}>{synthResult.message}</div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
