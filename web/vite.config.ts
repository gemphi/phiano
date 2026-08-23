import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  server: {
    fs: {
      allow: [
        path.resolve(__dirname, '..'),
        path.resolve(__dirname, '../..'),
        path.resolve(__dirname, '../../..'),
        path.resolve(__dirname, '../../../phinix'),
        path.resolve(__dirname, '../../phinix'),
      ],
    },
    proxy: {
      '/api': 'http://127.0.0.1:3002',
    },
  },
  build: {
    outDir: 'dist',
  },
});
