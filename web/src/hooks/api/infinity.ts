import type { InfinityResponse, InfinityTrainResult } from '../../types';
import { post } from './client';

export const visualizeInfinity = (text: string): Promise<InfinityResponse> =>
  post<InfinityResponse>('/api/infinity/visualize', { text });

export const trainInfinity = (text: string): Promise<InfinityTrainResult> =>
  post<InfinityTrainResult>('/api/infinity/train', { text });
