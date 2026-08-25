import type { EvalResult } from '../../types';
import { post } from './client';

export const evalText = (text: string): Promise<EvalResult> => post<EvalResult>('/api/eval', { text });
