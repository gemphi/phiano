import type {
  EvalResult, LearnResult, MultiLearnResult,
  OscEvalResult, OscTrainResult, Stats,
  InfinityResponse, InfinityTrainResult, GenerateResult,
} from '../types';

async function post<T>(url: string, body: Record<string, unknown>): Promise<T> {
  const r = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!r.ok) throw new Error(`API error: ${r.status}`);
  return r.json();
}

export async function fetchStats(): Promise<Stats> {
  const r = await fetch('/api/stats');
  return r.json();
}

export async function evalText(text: string): Promise<EvalResult> {
  return post('/api/eval', { text });
}

export async function learnText(text: string): Promise<LearnResult> {
  return post('/api/learn', { text });
}

export async function learnMulti(
  text: string, epochs: number, warmup: number,
): Promise<MultiLearnResult> {
  return post('/api/learn_multi', { text, epochs, warmup });
}

export async function generateText(text: string, maxTokens = 32, temperature = 0.15): Promise<GenerateResult> {
  return post('/api/generate', { text, max_tokens: maxTokens, temperature });
}

export async function oscEval(text: string): Promise<OscEvalResult> {
  return post('/api/oscillator/eval', { text });
}

export async function oscTrain(text: string, epochs: number): Promise<OscTrainResult> {
  return post('/api/oscillator/train', { text, epochs });
}

export async function runCommand(text: string): Promise<{ output: string }> {
  return post('/api/command', { text });
}

export async function visualizeInfinity(text: string): Promise<InfinityResponse> {
  return post('/api/infinity/visualize', { text });
}

export async function trainInfinity(text: string): Promise<InfinityTrainResult> {
  return post('/api/infinity/train', { text });
}

export async function fetchDefinition(word: string): Promise<import('../types').DefineResult> {
  return post('/api/define', { word });
}



