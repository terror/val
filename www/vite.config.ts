import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import { defineConfig } from 'vite';

const root = path.resolve(__dirname, '..');

export default defineConfig({
  base: '/val',
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@examples': path.resolve(root, 'examples'),
    },
  },
  server: {
    fs: {
      allow: [root],
    },
  },
});
