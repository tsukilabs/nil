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
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('src', import.meta.url)),
    },
  },
  clearScreen: false,
  publicDir: 'src/public',
  build: {
    target: 'baseline-widely-available',
    chunkSizeWarningLimit: 5000,
    copyPublicDir: true,
    emptyOutDir: true,
    sourcemap: false,
    minify: env.NIL_MINIFY !== 'false',
  },
  server: {
    port: 1420,
    strictPort: true,
    host: env.TAURI_DEV_HOST ?? false,
  },
});
