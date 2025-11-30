import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import wasmPack from "vite-plugin-wasm-pack";
import wasm from "vite-plugin-wasm";

let idk = wasmPack("../client-wasm");

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(), wasm(), idk],
  build: {
    rollupOptions: {
      output: {
        entryFileNames: `bundle.js`,
        chunkFileNames: `[name].[hash].js`,
        assetFileNames: `[name].[ext]`,
      },
    },
  },
  css: {
    preprocessorOptions: {
      scss: {
        silenceDeprecations: [
          "import",
          "mixed-decls",
          "color-functions",
          "global-builtin",
          "legacy-js-api",
        ],
      },
    },
  },
});
