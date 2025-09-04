import { env } from 'node:process';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import vue from '@vitejs/plugin-vue';
import tailwind from '@tailwindcss/vite';
import { fileURLToPath, URL } from 'node:url';

export default defineConfig({
  plugins: [
    wasm(),
    tailwind(),
    vue({ features: { optionsAPI: false } }),
  ],
  clearScreen: false,
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('src', import.meta.url)),
    },
  },
  build: {
    chunkSizeWarningLimit: 5000,
    copyPublicDir: true,
    emptyOutDir: true,
    minify: true,
    target: 'baseline-widely-available',
    sourcemap: Boolean(env.TAURI_ENV_DEBUG),
  },
  server: {
    port: 1420,
    strictPort: true,
    host: env.TAURI_DEV_HOST ?? false,
  },
});
