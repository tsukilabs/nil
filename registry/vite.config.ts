import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import tailwind from '@tailwindcss/vite';

export default defineConfig({
  plugins: [
    tailwind(),
    vue({ features: { optionsAPI: false } }),
  ],
  base: '/nsr/',
  build: {
    outDir: '../docs/dist/nsr',
    copyPublicDir: true,
    emptyOutDir: true,
    minify: true,
    target: 'baseline-widely-available',
  },
});
