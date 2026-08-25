export interface VersusDoc {
  id: string;
  tab: string;
  title: string;
  phianoSide: { label: string; points: string[]; code?: string };
  pytorchSide: { label: string; points: string[]; code?: string };
  insight: string;
}

export const VERSUS_DOCS: VersusDoc[] = [
  {
    id: 'problem',
    tab: 'The Problem',
    title: 'Why Attention Isn\'t Enough',
    phianoSide: {
      label: 'Phase-Coupled Oscillators',
      points: [
        'O(n) phase coupling - linear, not quadratic',
        'Live inference learning via Hebbian plasticity',
        'Phase angle IS position - no external encoding',
        'Full interpretability - every word has visible phase',
        'Zero forgetting - new knowledge adds new phasors',
        'Continuous ray-cast decoding, not discrete argmax',
      ],
    },
    pytorchSide: {
      label: 'Self-Attention',
      points: [
        'O(n²) attention - quadratic memory',
        'Frozen weights during inference',
        'Positional encoding bolted on (RoPE)',
        'Black-box attention weights',
        'Catastrophic forgetting on fine-tune',
        'Discrete token sampling from logits',
      ],
    },
    insight: 'The transformer treats language as a sequence of vectors. Phiano treats language as a wave - and waves naturally interfere, resonate, and propagate.',
  },
  {
    id: 'manifold',
    tab: 'Phase Manifold',
    title: 'C³² Torus Topology vs R^d Embedding',
    phianoSide: {
      label: 'C³² Torus',
      points: [
        'Compact toroidal space - no infinity',
        'Phase distance = angular difference (mod 2π)',
        'Position is intrinsic (phase angle)',
        'Amplitude = familiarity (updated per use)',
        'Frequency band = semantic depth (explicit)',
        'Natural periodicity - 2π wrapping = recursion',
        'Golden ratio seeding = uniform distribution',
      ],
      code: `pub struct SpectralPhasor {
    pub phase: f64,      // θ ∈ [0, 2π)
    pub amplitude: f64,  // r ∈ [0, 2.0]
    pub band_n: u32,     // harmonic band
}`,
    },
    pytorchSide: {
      label: 'R^d Euclidean',
      points: [
        'Flat, unstructured space',
        'Cosine similarity (dot product)',
        'Position injected via RoPE (external)',
        'Norm = learned (opaque)',
        'Semantic depth hidden in dimensions',
        'No periodicity',
        'Random initialization',
      ],
      code: `embedding = nn.Embedding(vocab_size, d_model)
x = embedding(token_ids)  # (batch, seq, d)`,
    },
    insight: 'The torus has native topology, periodicity, and group structure. R^d is a flat blob where position must be artificially injected.',
  },
  {
    id: 'multi-freq',
    tab: 'Multi-Frequency',
    title: '32 Harmonic Bands vs 8 Attention Heads',
    phianoSide: {
      label: 'Multi-Frequency Torus',
      points: [
        '32 intrinsic harmonic bands',
        'Bands interfere constructively (synonyms) / destructively (antonyms)',
        'O(n × harmonics) - linear cost',
        'Phase resonance = direct interpretability',
        'Ray-cast decoding: continuous sweep',
      ],
      code: `pub fn resonance(&self, other: &TorusPhasor) -> f64 {
    self.harmonics.iter()
        .zip(other.harmonics.iter())
        .map(|(a, b)| (a - b).cos())
        .sum::<f64>() / 32.0
}`,
    },
    pytorchSide: {
      label: 'Multi-Head Attention',
      points: [
        '8-16 heads (fixed at architecture time)',
        'Heads are independent - no interference',
        'O(n² × heads) - quadratic per head',
        'Attention weights = heat map (indirect)',
        'Softmax sampling: discrete probability',
      ],
      code: `self.attention = nn.MultiheadAttention(
    d_model=512, num_heads=8
)
output, weights = self.attention(q, k, v)`,
    },
    insight: 'Multi-frequency harmonics interfere like waves. Multi-head attention is 8 independent dot products. Wave interference is richer than parallel dot products.',
  },
  {
    id: 'syntax',
    tab: 'Syntax Coupling',
    title: 'Asymmetric β_ij Lag vs RoPE',
    phianoSide: {
      label: 'Directional Phase Lag',
      points: [
        'Position is intrinsic (phase angle)',
        'Lag is asymmetric: β(dog→bites) ≠ β(bites→dog)',
        'Learned per word pair (adaptive)',
        'Updates during inference (Hebbian)',
        'Steers generation via phase kick',
      ],
      code: `// β_ij: directional phase offset
facet.record_phase_lag(prev, next);
// EMA: learns the directional lag
*v = (1.0 - RATE) * *v + RATE * lag`,
    },
    pytorchSide: {
      label: 'RoPE Position Encoding',
      points: [
        'Position is external (rotation matrix)',
        'Symmetric: position 5→6 = 6→5',
        'Fixed function (same for all words)',
        'Static (no learning after training)',
        'No directional syntax',
      ],
      code: `def apply_rope(x, pos):
    angle = pos / (10000 ** (2i / d))
    return x * cos(angle) + rotate90(x) * sin(angle)`,
    },
    insight: 'Phiano learns that "dog bites man" has a different phase trajectory than "man bites dog". RoPE treats them identically - only embeddings differ.',
  },
  {
    id: 'riverflow',
    tab: 'Riverflow',
    title: 'Phase Propagation vs forward()',
    phianoSide: {
      label: 'Riverflow (Phase Wave)',
      points: [
        'Dynamic trajectory on torus',
        'Parameters in Facet (phasors + lags)',
        'Phase waves propagate through manifold',
        'O(n × vocab) per token',
        'Learns during generation (Hebbian)',
        'Phase momentum (inertia)',
        'Natural recursion (2π wrapping)',
        'Native visualization (phase trajectory)',
      ],
      code: `let target = (current_phase + momentum + jitter)
    .rem_euclid(TWO_PI);
let word = torus_ray_cast(facet, target);
current_phase += 0.35 * phase_diff;`,
    },
    pytorchSide: {
      label: 'forward() (Tensor Graph)',
      points: [
        'Dynamic DAG of tensor operations',
        'Parameters in nn.Module weights',
        'Tensors flow through layers',
        'O(n² × layers) per token',
        'Frozen during inference',
        'No momentum (Markovian)',
        'No recursion (linear sequence)',
        'Requires TensorBoard',
      ],
      code: `def forward(self, x):
    attn = self.attention(x, x, x)
    x = self.norm1(x + attn)
    x = self.norm2(x + self.ffn(x))
    return x`,
    },
    insight: 'The riverflow is a wave with momentum on a torus. forward() is a tensor pipeline. Waves have inertia, interference, and periodicity - tensors don\'t.',
  },
  {
    id: 'hebbian',
    tab: 'Hebbian vs Backprop',
    title: 'Wave Plasticity vs Gradient Descent',
    phianoSide: {
      label: 'Hebbian Plasticity',
      points: [
        'No labeled data needed',
        'Online: one example at a time',
        'Local signal (phase difference)',
        'Zero forgetting (additive)',
        'CPU, milliseconds',
        'Learns during inference',
        'Order parameter R = synchronization',
      ],
      code: `let diff = (target_phase - word_phase).sin();
word_phasor.phase += LEARNING_RATE * diff;
word_phasor.amplitude += AMPLITUDE_INCREMENT;`,
    },
    pytorchSide: {
      label: 'Backpropagation',
      points: [
        'Requires labeled data (input → target)',
        'Batch training (32-4096 examples)',
        'Global loss gradient',
        'Catastrophic forgetting',
        'GPU, hours to days',
        'Frozen after deployment',
        'Loss curve = proxy metric',
      ],
      code: `loss = cross_entropy(logits, targets)
loss.backward()  # autograd
optimizer.step()  # w -= lr * grad`,
    },
    insight: 'Humans learn by conversation, not by gradient descent. Phiano learns the way humans do - one example at a time, continuously, without forgetting.',
  },
  {
    id: 'correction',
    tab: 'Self-Correction',
    title: 'Anti-Phase Pulse vs Fine-Tuning',
    phianoSide: {
      label: 'Anti-Phase Pulse (π)',
      points: [
        'Instant - milliseconds, CPU',
        'Surgical - only 2 phasors touched',
        'Zero forgetting - all else unchanged',
        'During conversation (!correct)',
        'Physically meaningful (phase repulsion)',
        'Reversible (apply -π)',
      ],
      code: `// !correct dogs are reptiles|dogs are mammals
let repulsion = correct_phase + PI;
wrong_phase += 0.5 * (repulsion - wrong_phase).sin();`,
    },
    pytorchSide: {
      label: 'Fine-Tuning',
      points: [
        'Minutes to hours, GPU required',
        'Global - affects all weights',
        'High forgetting risk',
        'Separate from conversation',
        'Opaque weight changes',
        'Irreversible',
      ],
      code: `for epoch in range(3):
    loss = cross_entropy(model(wrong), correct)
    loss.backward()
    optimizer.step()  # hope nothing broke`,
    },
    insight: 'Phase repulsion is physically real - the wrong concept is pushed to anti-phase (π away). The transformer can only adjust probabilities through expensive retraining.',
  },
  {
    id: 'dialog',
    tab: 'Dialog Ingestion',
    title: 'Multi-Turn Learning vs RLHF',
    phianoSide: {
      label: 'Dialog Ingestion',
      points: [
        'No reward model needed',
        'Seconds, CPU',
        'No reward hacking',
        'No alignment tax',
        'Live updates (API call)',
        'Transparent phase shifts',
      ],
      code: `// POST /api/dialogue/learn
let count = source.learn_into_facet(
    &mut facet, &mut memo, &trainer
);
// "dialogues_trained": 24`,
    },
    pytorchSide: {
      label: 'RLHF',
      points: [
        'Requires reward model',
        'Weeks on GPU clusters',
        'Reward hacking risk',
        'Alignment tax (degraded benchmarks)',
        'Static after training',
        'Opaque weight changes',
      ],
      code: `# 1. Train reward model on preferences
# 2. Fine-tune LLM with PPO
# 3. Repeat for weeks
# Cost: $10K-$100K+`,
    },
    insight: 'RLHF is alignment through reward signals. Dialog ingestion is alignment through conversation. Humans align through conversation, not reward models.',
  },
  {
    id: 'agents',
    tab: '16 Agents',
    title: 'Cognitive Agents vs Transformer Layers',
    phianoSide: {
      label: '16 Specialized Agents',
      points: [
        'Each agent has a distinct role (Searle\'s philosophy)',
        'SpeechAct, Intentionality, Semantics, Syntax, etc.',
        'Agents contribute phase signals (parliament)',
        'Parallel + aggregated by CollectiveIntention',
        'Add new agent types (extensible)',
        'Named, interpretable roles',
        'Grounded semantics — not just syntax (Chinese Room)',
      ],
    },
    pytorchSide: {
      label: 'N Identical Layers',
      points: [
        'All layers do the same operation (attention + FFN)',
        'No specialization - layer 3 = layer 5',
        'No semantics - just math',
        'Sequential (layer 1 → 2 → ... → N)',
        'Can add layers but they\'re identical',
        'Unnamed, opaque',
      ],
    },
    insight: 'Searle proved syntax ≠ semantics (Chinese Room). Transformers shuffle tokens syntactically — Phiano\'s agents ground meaning through intentional states. Coupling > Attention.',
  },
  {
    id: 'context',
    tab: 'Context Buffer',
    title: 'Wave Superposition vs KV Cache',
    phianoSide: {
      label: 'Context Wave Buffer',
      points: [
        'O(1) memory - one complex number',
        'Exponential decay (recent dominates)',
        'Natural forgetting (like human memory)',
        'Unlimited context (ring buffer)',
        'Phase + amplitude inspectable',
        'Momentum (phase velocity)',
      ],
      code: `pub struct ContextWaveBuffer {
    sum_x: f64,  // ONE number
    sum_y: f64,  // ONE number
}`,
    },
    pytorchSide: {
      label: 'KV Cache',
      points: [
        'O(n × d) - grows with sequence',
        'No decay (all equal weight)',
        'No forgetting (until context limit)',
        'Fixed context window (4K-128K)',
        'Opaque tensors',
        'No momentum (static memory)',
      ],
      code: `past_kv = []
for token in tokens:
    k, v = kv_proj(token)
    past_kv.append((k, v))  # grows forever`,
    },
    insight: 'Context wave = ripple in a pond (fades, has momentum). KV cache = tape recorder (stores everything, no fading). Human memory works like the pond.',
  },
  {
    id: 'raycast',
    tab: 'Ray-Cast',
    title: 'Attractor Decoding vs Autoregressive Sampling',
    phianoSide: {
      label: 'Ray-Cast on Torus',
      points: [
        'Continuous phase trajectory',
        'Metric: phase distance from target',
        'Momentum carries trajectory forward',
        'Multi-frequency resonance (32 bands)',
        'Constructive/destructive interference',
        'Golden ratio jitter (deterministic exploration)',
      ],
      code: `let target_torus = TorusPhasor::from_spectral(&target);
let best = facet.lexicon.iter()
    .map(|(w, p)| (w, target_torus.resonance(&p.into())))
    .max_by(resonance)`,
    },
    pytorchSide: {
      label: 'Softmax Sampling',
      points: [
        'Discrete probability distribution',
        'No distance metric (flat distribution)',
        'Memoryless (no momentum)',
        'Single softmax (no interference)',
        'No interference patterns',
        'Temperature = pure randomness',
      ],
      code: `logits = model.forward(tokens)
probs = softmax(logits / temperature)
next_token = torch.multinomial(probs, 1)`,
    },
    insight: 'Ray-cast = lighthouse beam sweeping the torus. Softmax = random pick from a flat distribution. The beam has direction, speed, and interference patterns.',
  },
  {
    id: 'learning',
    tab: 'Online Learning',
    title: 'Continuous Plasticity vs Gradient Descent',
    phianoSide: {
      label: 'Always Learning',
      points: [
        'Single example (no batches)',
        'Online (train = inference)',
        'CPU, milliseconds',
        'Live after deployment',
        'Zero forgetting (additive)',
        'Local phase difference signal',
        '4 pillars: torus, syntax, dialog, correction',
      ],
    },
    pytorchSide: {
      label: 'Train Then Deploy',
      points: [
        'Batch (millions of examples)',
        'Offline (separate phases)',
        'GPU, days to months',
        'Frozen after deployment',
        'Catastrophic forgetting',
        'Global loss gradient',
        'Requires full retraining for new knowledge',
      ],
    },
    insight: 'Transformer training = studying for an exam. Phiano learning = having a conversation. Humans don\'t retrain to learn a new fact - someone tells you, and you know it.',
  },
  {
    id: 'viz',
    tab: 'Visualization',
    title: 'Native Phase Topology vs TensorBoard',
    phianoSide: {
      label: 'PUI (Native)',
      points: [
        'Live during inference',
        'Phase topology (word positions on torus)',
        'Full manifold visualization',
        'Direct interpretability (phase, amplitude)',
        'Integrated in PUI (same tool as chat)',
        'Watch correction propagate',
        'See all 16 agent contributions',
        'Order parameter R (synchronization)',
      ],
    },
    pytorchSide: {
      label: 'TensorBoard (External)',
      points: [
        'After training (logs, not live)',
        'Loss curves, attention matrices',
        'No topology (flat metrics)',
        'Proxy metrics (loss/accuracy)',
        'Separate tool from model',
        'Can\'t see what went wrong',
        'No agent contributions',
        'No physical meaning',
      ],
    },
    insight: 'TensorBoard shows you loss curves after training. Phiano PUI shows you the manifold evolving during chat. Glass box vs black box.',
  },
  {
    id: 'mckenna',
    tab: 'McKenna Test',
    title: 'Generative Quality: The Spiraling Test',
    phianoSide: {
      label: 'Phase Dynamics (Emergent)',
      points: [
        'Recursion: emergent from 2π wrapping',
        'Novelty: phase distance from origin (measurable)',
        'Self-reference: phase returns to new words (Hebbian shift)',
        'Spiraling: inherent (mod 2π creates cycles)',
        'Learns during generation',
        'Momentum: context wave accumulates velocity',
        'Self-interruption: phase discontinuities',
      ],
    },
    pytorchSide: {
      label: 'Pattern Matching (Imitated)',
      points: [
        'Recursion: simulated via attention (looks back)',
        'Novelty: temperature randomness',
        'Self-reference: pattern-matched ("the dream dreams")',
        'Spiraling: none (linear sequence)',
        'Frozen during generation',
        'No momentum (Markovian)',
        'Self-interruption: rare (trained to be fluent)',
      ],
    },
    insight: 'McKenna\'s prose IS phase dynamics. The transformer can describe it but can\'t be it. Phiano\'s torus IS the structure McKenna described - his Timewave Zero is a phase oscillator.',
  },
  {
    id: 'pui',
    tab: 'PUI Dashboard',
    title: 'Unified Interface vs Fragmented Tooling',
    phianoSide: {
      label: 'PUI (One Tool)',
      points: [
        'Chat + train + visualize + correct in one UI',
        'Live interaction with model internals',
        'Real-time training (every chat message)',
        'Transparent (phase, amplitude, resonance)',
        'Dynamic (live system, not snapshot)',
        '10 panels: Chat, Dictionary, Learn, Eval, Stats, Oscillator, Infinity, Phi4, Docs, Versus',
        'VersusPanel: interactive Phiano vs PyTorch comparison',
      ],
    },
    pytorchSide: {
      label: '5+ Separate Tools',
      points: [
        'Jupyter (code) + TensorBoard (viz) + Flask (API) + Grafana (monitor)',
        'No live interaction with internals',
        'Train first, then deploy (separate)',
        'Opaque (attention weights only)',
        'Static (notebooks are snapshots)',
        'No comparison tool',
      ],
    },
    insight: 'Documentation as a living argument, not a dead reference. The VersusPanel beats PyTorch at its own game by showing both approaches side-by-side, interactively.',
  },
  {
    id: 'future',
    tab: 'The Future',
    title: 'Self-Generating Phase Networks vs Bigger Models',
    phianoSide: {
      label: 'Deeper (Richer Topology)',
      points: [
        'Dynamic phase graph per input (riverflow + topology)',
        'Hierarchical layers: char → word → phrase → sentence → dialog',
        'Self-organizing topology (grows own structure)',
        'Distributed phase coupling (federated via Kuramoto)',
        'Phase space is infinite (32D torus)',
        'Cost: $0 (continuous online learning)',
        'Human analogy: richer connections',
      ],
    },
    pytorchSide: {
      label: 'Bigger (More Parameters)',
      points: [
        'GPT-4: 1.7T params, GPT-5: bigger',
        'Fixed architecture (transformer layers)',
        'Context windows: 1M-10M tokens',
        'Federated learning (gradient averaging)',
        'Diminishing returns (flattening curve)',
        'Cost: $100M+ per training run',
        'Human analogy: bigger brain',
      ],
    },
    insight: 'The transformer scales by brute force (more parameters). Phiano scales by enrichment (richer topology). The phase space is mathematically infinite - the parameter space is not.',
  },
];
