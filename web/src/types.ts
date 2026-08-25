export interface EvalResult {
  coherence: number;
  novelty: number;
  resonance: number;
  overall: number;
  verdict: string;
  vocabulary: number;
}

export interface LearnResult {
  tokens: number;
  vocabulary: number;
  message: string;
}

export interface MultiLearnResult {
  epochs: number;
  tokens: number;
  converged: boolean;
  vocabulary: number;
}

export interface OscEvalResult {
  coherence: number;
  sync: number;
  entropy: number;
  word_count: number;
  dominant_colors: [string, number][];
}

export interface OscTrainResult {
  epochs: number;
  coherence_before: number;
  coherence_after: number;
  sync_before: number;
  sync_after: number;
  converged: boolean;
}

export interface Stats {
  vocabulary: number;
  memory_entries: number;
}

export interface ChatApiResponse {
  response: string;
  speech_act: string;
  direction_of_fit: string;
  words_learned: number;
  definitions_learned: number;
  wiki_learned: string | null;
  vocabulary: number;
  coherence: number;
}

export interface StreamToken {
  token: string;
  step: number;
  collective_phase: number;
  resonance: number;
  done: boolean;
}

export interface ChatMessage {
  role: 'user' | 'assistant';
  text: string;
  speech_act?: string;
  direction_of_fit?: string;
  words_learned?: number;
  definitions_learned?: number;
  wiki_learned?: string | null;
  coherence?: number;
  streaming?: boolean;
  collective_phase?: number;
  resonance?: number;
  eval?: EvalResult;
  oscEval?: OscEvalResult;
}

export interface WordPhasorDetail {
  word: string;
  phase: number;
  amplitude: number;
  effective_phase: number;
  sector: number;
}

export interface ComplexDetail {
  re: number;
  im: number;
  amp: number;
  phase: number;
}

export interface VariationDetail {
  sector: number;
  color: string;
  text: string;
  resonance: number;
  wave: ComplexDetail;
  words: WordPhasorDetail[];
}

export interface InfinityResponse {
  variations: VariationDetail[];
  prompt_wave: ComplexDetail;
}

export interface WordShiftDetail {
  word: string;
  phase_before: number;
  phase_after: number;
  shift: number;
}

export interface InfinityTrainResult {
  success: boolean;
  message: string;
  tokens: number;
  vocabulary: number;
  shifts: WordShiftDetail[];
}

export interface InstructResponse {
  prompt: string;
  output: string;
  vocabulary: number;
}

export interface ReasoningResponse {
  problem: string;
  converged: boolean;
  steps_count: number;
  final_answer: string;
}

export interface LayerSummaryItem {
  level: number;
  sector_count: number;
  clusters_count: number;
}

export interface LayersResponse {
  layers_count: number;
  layer_summaries: LayerSummaryItem[];
}

export interface Phi4LearnResponse {
  vocab_tokens_loaded: number;
  merges_trained: number;
  doc_sentences_trained: number;
  final_vocabulary_size: number;
  message: string;
}

export interface SyntheticResponse {
  accepted_count: number;
  vocabulary: number;
  message: string;
}

export interface GenerateResult {
  prompt: string;
  generated: string;
  vocabulary: number;
  context_phase: number;
  context_amplitude: number;
}

export interface DefineResult {
  word: string;
  definition: string;
  source: string;
  phase?: number;
  amplitude?: number;
  vocabulary: number;
}

export interface FlowNode {
  word: string;
  phase: number;
  amplitude: number;
  band_n: number;
  activation: number;
  novelty: number;
}

export interface FlowEdge {
  from: number;
  to: number;
  coupling: 'bigram' | 'syntax_lag' | 'semantic' | 'anti_phase';
  lag: number;
  weight: number;
}

export interface FlowStep {
  step: number;
  collective_phase: number;
  momentum: number;
  selected_word: string | null;
  resonance_score: number;
  novelty: number;
}

export interface FlowResponse {
  nodes: FlowNode[];
  edges: FlowEdge[];
  trajectory: FlowStep[];
  collective_phase: number;
  momentum: number;
  order_parameter: number;
  novelty: number;
  node_count: number;
  edge_count: number;
}


