import type { FlowResponse } from '../../types';
import { post } from './client';

export const phaseFlow = (text: string, steps = 10): Promise<FlowResponse> =>
  post<FlowResponse>('/api/phase_flow', { text, steps });
