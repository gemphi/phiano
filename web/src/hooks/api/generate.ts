import type { GenerateResult, StreamToken } from '../../types';
import { post } from './client';

export const generateText = (
  text: string,
  maxTokens = 32,
  temperature = 0.15,
): Promise<GenerateResult> => post<GenerateResult>('/api/generate', { text, max_tokens: maxTokens, temperature });

export async function streamGenerate(
  text: string,
  onToken: (evt: StreamToken) => void,
  maxTokens = 32,
  temperature = 0.15,
): Promise<void> {
  const r = await fetch('/api/generate/stream', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', Accept: 'text/event-stream' },
    body: JSON.stringify({ text, max_tokens: maxTokens, temperature }),
  });
  if (!r.ok || !r.body) {
    throw new Error(`API error [${r.status}]: ${r.statusText}`);
  }
  const reader = r.body.getReader();
  const decoder = new TextDecoder();
  let buf = '';
  while (true) {
    const { value, done } = await reader.read();
    if (done) break;
    buf += decoder.decode(value, { stream: true });
    const chunks = buf.split('\n\n');
    buf = chunks.pop() ?? '';
    for (const chunk of chunks) {
      const line = chunk.split('\n').find(l => l.startsWith('data:'));
      if (!line) continue;
      const raw = line.slice(5).trim();
      if (!raw) continue;
      const evt = JSON.parse(raw) as StreamToken;
      onToken(evt);
      if (evt.done) return;
    }
  }
}
