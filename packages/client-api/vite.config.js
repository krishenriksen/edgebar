import { defineConfig } from "vite";

export default defineConfig({
  build: {
    target: "esnext",
    outDir: "dist",
    lib: {
      entry: "src/index.ts",
      formats: ["es"],
      fileName: "index"
    },
    minify: "terser",
    terserOptions: {
      compress: true,
      mangle: true,
      format: {
        comments: false
      }
    }
  }
});