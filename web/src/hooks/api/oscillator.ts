import type { OscEvalResult, OscTrainResult } from '../../types';
import { post } from './client';

export const oscEval = (text: string): Promise<OscEvalResult> => post<OscEvalResult>('/api/oscillator/eval', { text });
export const oscTrain = (text: string, epochs: number): Promise<OscTrainResult> =>
  post<OscTrainResult>('/api/oscillator/train', { text, epochs });
