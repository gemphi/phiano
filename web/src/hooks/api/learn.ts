import type { LearnResult, MultiLearnResult } from '../../types';
import { post } from './client';

export const learnText = (text: string): Promise<LearnResult> => post<LearnResult>('/api/learn', { text });
export const learnMulti = (
  text: string,
  epochs: number,
  warmup: number,
): Promise<MultiLearnResult> => post<MultiLearnResult>('/api/learn_multi', { text, epochs, warmup });
