import type { Stats } from '../../types';
import { get } from './client';

export const fetchStats = (): Promise<Stats> => get<Stats>('/api/stats');
