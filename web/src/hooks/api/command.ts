import { post } from './client';

export const runCommand = (text: string): Promise<{ output: string }> => post('/api/command', { text });
