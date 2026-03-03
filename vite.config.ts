import { defineConfig } from "vite";

export default defineConfig({
  clearScreen: false,
  server: {
    port: 1421,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  envPrefix: [
    "VITE_",
    "TAURI_ENV_*",
    "TAURI_PLATFORM",
    "TAURI_ARCH",
    "TAURI_FAMILY",
  ],
  build: {
    target:
      process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome105" : "safari13",
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
});
