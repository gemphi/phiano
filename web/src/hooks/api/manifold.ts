import { post } from './client';

export const saveManifold = (): Promise<{ status: string; vocabulary: number; message: string }> =>
  post<{ status: string; vocabulary: number; message: string }>('/api/save', {});
