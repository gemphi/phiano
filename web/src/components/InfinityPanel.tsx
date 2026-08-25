import { useState, useEffect, useRef, useCallback } from 'react';
import { Sparkles, Info, RefreshCw, ZoomIn, Award, Play } from 'lucide-react';
import { visualizeInfinity, trainInfinity } from '../hooks/api/infinity';
import type { InfinityResponse, VariationDetail, WordPhasorDetail, WordShiftDetail } from '../types';

interface InfinityPanelProps {
  onRefresh: () => Promise<void>;
}

const COLOR_MAP: Record<string, string> = {
  crimson: '#e11d48',
  red: '#ef4444',
  scarlet: '#f87171',
  orange: '#ea580c',
  amber: '#d97706',
  gold: '#fbbf24',
  yellow: '#ca8a04',
  lime: '#65a30d',
  green: '#16a34a',
  emerald: '#059669',
  teal: '#0d9488',
  blue: '#2563eb',
  indigo: '#4f46e5',
  violet: '#7c3aed',
  magenta: '#c026d3',
  rose: '#db2777',
};

const SUGGESTIONS = [
  "I wish to go to bed with you",
  "The heart has its reasons which reason knows nothing of",
  "To be or not to be",
  "Semantic similarity is measured by destructive interference",
  "Words are keys and phasors are notes"
];

export function InfinityPanel({ onRefresh }: InfinityPanelProps) {
  const [inputText, setInputText] = useState(SUGGESTIONS[0]);
  const [data, setData] = useState<InfinityResponse | null>(null);
  const [selectedSector, setSelectedSector] = useState<number | null>(null);
  const [busy, setBusy] = useState(false);
  const [trainBusy, setTrainBusy] = useState(false);
  const [editText, setEditText] = useState('');
  const [trainMessage, setTrainMessage] = useState('');
  const [shifts, setShifts] = useState<WordShiftDetail[]>([]);
  const [hoveredWordIndex, setHoveredWordIndex] = useState<number | null>(null);

  const canvasRef = useRef<HTMLCanvasElement>(null);

  const loadData = useCallback(async (textToLoad: string) => {
    if (!textToLoad.trim() || busy) return;
    setBusy(true);
    setTrainMessage('');
    setShifts([]);
    try {
      const res = await visualizeInfinity(textToLoad);
      setData(res);
      // Select the winner (the sector with highest resonance) by default
      if (res.variations.length > 0) {
        let maxIdx = 0;
        let maxRes = -1;
        res.variations.forEach((v, idx) => {
          if (v.resonance > maxRes) {
            maxRes = v.resonance;
            maxIdx = idx;
          }
        });
        setSelectedSector(res.variations[maxIdx].sector);
        setEditText(res.variations[maxIdx].text);
      }
    } catch (e) {
      console.error(e);
    }
    setBusy(false);
  }, [busy]);

  useEffect(() => {
    loadData(inputText);
  }, []);

  const selectedVariation = data?.variations.find(v => v.sector === selectedSector) || null;

  // Set editable text when selected sector changes
  useEffect(() => {
    if (selectedVariation) {
      setEditText(selectedVariation.text);
      setTrainMessage('');
      setShifts([]);
    }
  }, [selectedSector, selectedVariation]);

  // Draw the vector summation diagram on the canvas
  useEffect(() => {
    if (!canvasRef.current || !selectedVariation) return;
    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear and draw grid
    const width = canvas.width;
    const height = canvas.height;
    ctx.clearRect(0, 0, width, height);

    const centerX = width / 2;
    const centerY = height / 2;

    // Grid lines
    ctx.strokeStyle = 'var(--border-color)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(0, centerY); ctx.lineTo(width, centerY);
    ctx.moveTo(centerX, 0); ctx.lineTo(centerX, height);
    ctx.stroke();

    // Concentric circles
    ctx.strokeStyle = 'rgba(128, 128, 128, 0.15)';
    ctx.beginPath();
    ctx.arc(centerX, centerY, 50, 0, 2 * Math.PI);
    ctx.arc(centerX, centerY, 100, 0, 2 * Math.PI);
    ctx.arc(centerX, centerY, 150, 0, 2 * Math.PI);
    ctx.stroke();

    // Scale calculations
    const words = selectedVariation.words;
    if (words.length === 0) return;

    // Find max accumulated vector size to scale appropriately
    let currentX = 0;
    let currentY = 0;
    let maxDist = 1.0;
    words.forEach(w => {
      currentX += w.amplitude * Math.cos(w.effective_phase);
      currentY += w.amplitude * Math.sin(w.effective_phase);
      const dist = Math.sqrt(currentX * currentX + currentY * currentY);
      if (dist > maxDist) maxDist = dist;
    });

    const scale = 140 / maxDist; // Fit within 150px radius

    // Draw word vectors
    let startX = centerX;
    let startY = centerY;

    words.forEach((w, index) => {
      const dx = w.amplitude * Math.cos(w.effective_phase) * scale;
      const dy = w.amplitude * Math.sin(w.effective_phase) * scale; // Canvas Y goes down, so we subtract to map standard math angles
      const endX = startX + dx;
      const endY = startY - dy; // Subtracting dy because Cartesian Y goes up and Canvas Y goes down

      const isHovered = hoveredWordIndex === index;
      ctx.lineWidth = isHovered ? 4 : 2;
      ctx.strokeStyle = isHovered ? 'var(--color-primary)' : '#888888';

      // Draw vector line
      ctx.beginPath();
      ctx.moveTo(startX, startY);
      ctx.lineTo(endX, endY);
      ctx.stroke();

      // Draw arrow head
      const angle = Math.atan2(-dy, dx); // note the negative dy for canvas space
      ctx.fillStyle = isHovered ? 'var(--color-primary)' : '#888888';
      ctx.beginPath();
      ctx.moveTo(endX, endY);
      ctx.lineTo(endX - 10 * Math.cos(angle - Math.PI / 6), endY - 10 * Math.sin(angle - Math.PI / 6));
      ctx.lineTo(endX - 10 * Math.cos(angle + Math.PI / 6), endY - 10 * Math.sin(angle + Math.PI / 6));
      ctx.fill();

      // Draw word text label
      if (isHovered) {
        ctx.fillStyle = 'var(--text-primary)';
        ctx.font = 'bold 11px sans-serif';
        ctx.fillText(w.word, endX + 8, endY - 4);
      }

      startX = endX;
      startY = endY;
    });

    // Draw target final superposition wave vector
    const finalX = startX;
    const finalY = startY;
    const colorHex = COLOR_MAP[selectedVariation.color] || 'var(--color-primary)';
    
    ctx.lineWidth = 3;
    ctx.strokeStyle = colorHex;
    ctx.setLineDash([4, 4]);
    ctx.beginPath();
    ctx.moveTo(centerX, centerY);
    ctx.lineTo(finalX, finalY);
    ctx.stroke();
    ctx.setLineDash([]); // Reset line dash

    // Draw final arrow head
    const finalAngle = Math.atan2(finalY - centerY, finalX - centerX);
    ctx.fillStyle = colorHex;
    ctx.beginPath();
    ctx.moveTo(finalX, finalY);
    ctx.lineTo(finalX - 12 * Math.cos(finalAngle - Math.PI / 6), finalY - 12 * Math.sin(finalAngle - Math.PI / 6));
    ctx.lineTo(finalX - 12 * Math.cos(finalAngle + Math.PI / 6), finalY - 12 * Math.sin(finalAngle + Math.PI / 6));
    ctx.fill();

    // Label final phasor
    ctx.fillStyle = colorHex;
    ctx.font = 'bold 12px sans-serif';
    ctx.fillText(`Z_sum (${selectedVariation.wave.amp.toFixed(2)})`, finalX + 10, finalY + 12);

  }, [selectedVariation, hoveredWordIndex]);

  const handleTrain = async () => {
    if (!editText.trim() || trainBusy) return;
    setTrainBusy(true);
    setTrainMessage('');
    setShifts([]);
    try {
      const res = await trainInfinity(editText);
      if (res.success) {
        setTrainMessage(res.message);
        setShifts(res.shifts);
        await onRefresh();
        // Refresh visualization based on original input to see how the landscape shifted
        const visRes = await visualizeInfinity(inputText);
        setData(visRes);
      }
    } catch (e) {
      setTrainMessage(`Error: ${e}`);
    }
    setTrainBusy(false);
  };

  // Find max resonance for SVG wheel scaling
  const maxResonance = data ? Math.max(...data.variations.map(v => v.resonance)) : 1.0;

  return (
    <div style={{ display: 'grid', gridTemplateColumns: '1fr', gap: '2rem', maxWidth: '1200px', margin: '0 auto' }}>
      
      {/* Search Input Card */}
      <div className="card animate-in">
        <div className="card-title" style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <Sparkles size={20} style={{ color: 'var(--color-primary)' }} />
          Infinity Resonance & 64 Variations
        </div>
        <p style={{ color: 'var(--text-secondary)', fontSize: '0.85rem', marginBottom: '1rem' }}>
          Compose 64 parallel sector variations of a prompt (the 64 ways). Visualize their destructive and constructive interference compared against infinity.
        </p>

        {/* Suggestion list */}
        <div style={{ display: 'flex', gap: '0.5rem', flexWrap: 'wrap', marginBottom: '1rem' }}>
          {SUGGESTIONS.map((s, idx) => (
            <button
              key={idx}
              className="badge"
              style={{ cursor: 'pointer', border: '1px solid transparent' }}
              onClick={() => { setInputText(s); loadData(s); }}
            >
              {s}
            </button>
          ))}
        </div>

        <div style={{ display: 'flex', gap: '0.75rem' }}>
          <input
            className="input"
            value={inputText}
            onChange={e => setInputText(e.target.value)}
            placeholder="Type a sentence to visualize..."
            onKeyDown={e => { if (e.key === 'Enter') loadData(inputText); }}
          />
          <button className="btn btn-primary" onClick={() => loadData(inputText)} disabled={busy}>
            {busy ? <div className="spinner" /> : <Play size={16} />} Analyze
          </button>
        </div>
      </div>

      {data && (
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(350px, 1fr))', gap: '2rem' }}>
          
          {/* 64-Sector Color Wheel Visualization */}
          <div className="card animate-in" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
            <div className="card-title" style={{ alignSelf: 'flex-start', width: '100%', display: 'flex', justifyContent: 'space-between' }}>
              <span>64-Sector Resonance Spectrum</span>
              <span className="badge">Max Resonance: {maxResonance.toFixed(1)}</span>
            </div>
            
            <div style={{ position: 'relative', width: '320px', height: '320px', margin: '1rem 0' }}>
              <svg width="320" height="320" viewBox="0 0 320 320" style={{ transform: 'rotate(-90deg)' }}>
                {data.variations.map((v) => {
                  const s = v.sector;
                  const res = v.resonance;
                  const color = COLOR_MAP[v.color] || '#8b5cf6';
                  
                  // Sector boundaries
                  const startAngle = (s * 2 * Math.PI) / 64;
                  const endAngle = ((s + 1) * 2 * Math.PI) / 64;
                  
                  // Scale radius proportional to resonance
                  // Min radius is 40, max radius is 145
                  const r = 40 + (res / maxResonance) * 105;
                  
                  // Compute SVG Arc Path
                  const x1 = 160 + r * Math.cos(startAngle);
                  const y1 = 160 + r * Math.sin(startAngle);
                  const x2 = 160 + r * Math.cos(endAngle);
                  const y2 = 160 + r * Math.sin(endAngle);
                  
                  const isSelected = selectedSector === s;
                  
                  return (
                    <path
                      key={s}
                      d={`M 160 160 L ${x1} ${y1} A ${r} ${r} 0 0 1 ${x2} ${y2} Z`}
                      fill={color}
                      stroke={isSelected ? 'var(--text-primary)' : 'rgba(0,0,0,0.1)'}
                      strokeWidth={isSelected ? 2 : 0.5}
                      opacity={isSelected ? 1.0 : 0.7}
                      style={{
                        cursor: 'pointer',
                        transition: 'all 0.2s ease',
                        filter: isSelected ? 'drop-shadow(0 0 6px var(--color-primary))' : 'none',
                      }}
                      onClick={() => setSelectedSector(s)}
                    >
                      <title>Sector {s} ({v.color}) Resonance: {res.toFixed(2)}</title>
                    </path>
                  );
                })}
                {/* Center Core */}
                <circle cx="160" cy="160" r="38" fill="var(--bg-card)" stroke="var(--border-color)" strokeWidth="1" />
                <text
                  x="160"
                  y="164"
                  textAnchor="middle"
                  fill="var(--text-primary)"
                  style={{
                    fontSize: '10px',
                    fontWeight: 'bold',
                    transform: 'rotate(90deg) translate(0px, -320px)', // adjust text rotation
                    transformOrigin: '160px 160px'
                  }}
                >
                  ∞ RES
                </text>
              </svg>
            </div>
            
            <div style={{ width: '100%', fontSize: '0.8rem', color: 'var(--text-secondary)', textAlign: 'center' }}>
              Hover or click on the slices of the flower wheel to see what variant of the sentence is produced in that sector direction.
            </div>
          </div>

          {/* Zoom In & Suggestions Detail Card */}
          <div className="card animate-in" style={{ display: 'flex', flexDirection: 'column', gap: '1.25rem' }}>
            <div className="card-title" style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
              <ZoomIn size={18} /> Zoom In: Sector {selectedSector} Suggestion
            </div>
            
            {selectedVariation ? (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
                {/* Generated Text */}
                <div style={{
                  padding: '1rem',
                  background: 'var(--bg-secondary)',
                  borderLeft: `4px solid ${COLOR_MAP[selectedVariation.color] || 'var(--color-primary)'}`,
                  borderRadius: '0 var(--radius-md) var(--radius-md) 0',
                  fontSize: '1rem',
                  fontWeight: 500,
                  color: 'var(--text-primary)'
                }}>
                  "{selectedVariation.text}"
                </div>

                <div style={{ display: 'flex', gap: '0.5rem', fontSize: '0.75rem', flexWrap: 'wrap' }}>
                  <span className="badge" style={{ backgroundColor: COLOR_MAP[selectedVariation.color] + '22', color: COLOR_MAP[selectedVariation.color] }}>
                    Sector {selectedVariation.sector} ({selectedVariation.color})
                  </span>
                  <span className="badge" style={{ backgroundColor: 'var(--color-info)22', color: 'var(--color-info)' }}>
                    Resonance: {selectedVariation.resonance.toFixed(3)}
                  </span>
                  <span className="badge" style={{ backgroundColor: 'var(--color-success)22', color: 'var(--color-success)' }}>
                    Superposition Amplitude: {selectedVariation.wave.amp.toFixed(3)}
                  </span>
                </div>

                {/* Phasor Canvas */}
                <div>
                  <div className="metric-label" style={{ marginBottom: '0.25rem', display: 'flex', alignItems: 'center', gap: '0.25rem' }}>
                    <Info size={12} /> Word Phasor Addition (Tip-to-Tail)
                  </div>
                  <div style={{
                    background: 'var(--bg-input)',
                    borderRadius: 'var(--radius-lg)',
                    border: '1px solid var(--border-color)',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    padding: '0.5rem'
                  }}>
                    <canvas ref={canvasRef} width="300" height="300" style={{ maxWidth: '100%', height: 'auto' }} />
                  </div>
                </div>

                {/* Words Details List */}
                <div>
                  <div className="metric-label" style={{ marginBottom: '0.5rem' }}>Words Phasor Coordinates</div>
                  <div style={{ display: 'flex', flexDirection: 'column', gap: '0.35rem', maxHeight: '180px', overflowY: 'auto' }}>
                    {selectedVariation.words.map((w, idx) => (
                      <div
                        key={idx}
                        style={{
                          display: 'flex',
                          justifyContent: 'space-between',
                          padding: '0.4rem 0.5rem',
                          background: hoveredWordIndex === idx ? 'var(--color-primary-light)' : 'transparent',
                          border: `1px solid ${hoveredWordIndex === idx ? 'var(--color-primary)' : 'var(--border-color)'}`,
                          borderRadius: 'var(--radius-sm)',
                          fontSize: '0.8rem',
                          transition: 'all 0.15s ease',
                          cursor: 'pointer'
                        }}
                        onMouseEnter={() => setHoveredWordIndex(idx)}
                        onMouseLeave={() => setHoveredWordIndex(null)}
                      >
                        <span style={{ fontWeight: 600 }}>{w.word}</span>
                        <span style={{ color: 'var(--text-secondary)', fontFamily: 'monospace' }}>
                          A: {w.amplitude.toFixed(2)} | φ: {w.effective_phase.toFixed(2)} rad
                        </span>
                      </div>
                    ))}
                  </div>
                </div>

              </div>
            ) : (
              <div style={{ color: 'var(--text-secondary)', fontSize: '0.875rem' }}>
                Select a sector on the wheel to zoom in on its wave parameters.
              </div>
            )}
          </div>

        </div>
      )}

      {/* Back-and-Forth Learning & Tuning loop */}
      {selectedVariation && (
        <div className="card animate-in">
          <div className="card-title" style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
            <Award size={18} />
            Learning Loop & Kuramoto Tuning
          </div>
          <p style={{ color: 'var(--text-secondary)', fontSize: '0.85rem', marginBottom: '1rem' }}>
            Modify this sector's suggestion to your liking and train Phiano. Phiano will adjust the phase angles of the words using Kuramoto attraction. The entire resonance wheel will update in real time.
          </p>

          <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
            <textarea
              className="textarea"
              value={editText}
              onChange={e => setEditText(e.target.value)}
              placeholder="Edit the sentence here..."
              style={{ fontSize: '1rem', padding: '0.75rem', minHeight: '60px' }}
            />
            
            <button
              className="btn btn-primary"
              style={{ alignSelf: 'flex-start' }}
              onClick={handleTrain}
              disabled={trainBusy || !editText.trim()}
            >
              {trainBusy ? <div className="spinner" /> : <RefreshCw size={16} />} Train Phiano on this sentence
            </button>

            {trainMessage && (
              <div style={{
                padding: '0.75rem 1rem',
                backgroundColor: 'var(--color-primary-light)',
                borderRadius: 'var(--radius-md)',
                color: 'var(--text-primary)',
                fontSize: '0.875rem',
                borderLeft: '4px solid var(--color-primary)'
              }}>
                {trainMessage}
              </div>
            )}

            {shifts.length > 0 && (
              <div>
                <div className="metric-label" style={{ marginBottom: '0.5rem' }}>Manifold Phase Shifts (Transparency Log)</div>
                <div style={{
                  display: 'grid',
                  gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))',
                  gap: '0.5rem',
                  maxHeight: '200px',
                  overflowY: 'auto'
                }}>
                  {shifts.map((s, idx) => {
                    const isPositive = s.shift > 0;
                    return (
                      <div
                        key={idx}
                        style={{
                          padding: '0.5rem',
                          background: 'var(--bg-secondary)',
                          borderRadius: 'var(--radius-md)',
                          fontSize: '0.75rem',
                          display: 'flex',
                          justifyContent: 'space-between',
                          alignItems: 'center'
                        }}
                      >
                        <span style={{ fontWeight: 600 }}>{s.word}</span>
                        <span style={{
                          color: isPositive ? 'var(--color-success)' : 'var(--color-error)',
                          fontWeight: 'bold',
                          fontFamily: 'monospace'
                        }}>
                          {isPositive ? '+' : ''}{s.shift.toFixed(4)} rad
                        </span>
                      </div>
                    );
                  })}
                </div>
              </div>
            )}
          </div>
        </div>
      )}

    </div>
  );
}
