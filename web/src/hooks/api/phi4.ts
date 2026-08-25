import type { LayersResponse, InstructResponse, ReasoningResponse, Phi4LearnResponse, SyntheticResponse } from '../../types';
import { get, post } from './client';

export const fetchLayers = (): Promise<LayersResponse> => get<LayersResponse>('/api/layers');
export const instructText = (text: string): Promise<InstructResponse> => post<InstructResponse>('/api/instruct', { text });
export const reasonText = (text: string): Promise<ReasoningResponse> => post<ReasoningResponse>('/api/reason', { text });
export const learnPhi4 = (): Promise<Phi4LearnResponse> => post<Phi4LearnResponse>('/api/phi4/learn', { text: '' });
export const runSyntheticCurriculum = (): Promise<SyntheticResponse> => post<SyntheticResponse>('/api/synthetic', {});
