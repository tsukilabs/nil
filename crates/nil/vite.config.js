/* eslint-disable perfectionist/sort-objects */
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import tailwind from '@tailwindcss/vite';
import dev from 'vite-plugin-vue-devtools';
import { fileURLToPath, URL } from 'node:url';

export default defineConfig({
  plugins: [vue(), tailwind(), dev()],
  clearScreen: false,
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  build: {
    chunkSizeWarningLimit: 1000,
    copyPublicDir: true,
    emptyOutDir: true,
    minify: true,
    target: 'esnext',
  },
  server: {
    port: 1420,
    strictPort: true,
    host: process.env.TAURI_DEV_HOST || false,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
});
