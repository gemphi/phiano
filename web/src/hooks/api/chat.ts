import type { ChatApiResponse } from '../../types';
import { post } from './client';

export const chatMessage = (text: string): Promise<ChatApiResponse> => post<ChatApiResponse>('/api/chat', { text });
