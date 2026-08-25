import type { DefineResult } from '../../types';
import { post } from './client';

export const fetchDefinition = (word: string): Promise<DefineResult> => post<DefineResult>('/api/define', { word });
