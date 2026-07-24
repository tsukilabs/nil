import { env } from "node:process";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwind from "@tailwindcss/vite";
import { fileURLToPath, URL } from "node:url";
import { env_Var } from "@tsukilabs/nil-bindings";

const minify = env[env_Var.NIL_MINIFY_SOURCE] !== "false";

export default defineConfig({
  plugins: [
    tailwind({ optimize: { minify } }),
    vue({ features: { optionsAPI: false } }),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("src", import.meta.url)),
      "@ui": fileURLToPath(new URL("src/components/ui", import.meta.url)),
    },
  },
  clearScreen: false,
  publicDir: "src/public",
  build: {
    chunkSizeWarningLimit: 5000,
    copyPublicDir: true,
    emptyOutDir: true,
    sourcemap: false,
    minify,
    rolldownOptions: {
      experimental: {
        attachDebugInfo: import.meta.env.DEV ? "simple" : "none",
      },
    },
  },
  server: {
    port: 1420,
    strictPort: true,
    host: env.TAURI_DEV_HOST ?? false,
    watch: {
      ignored: ["**/src-tauri", "**/*.rs"],
    },
  },
});
