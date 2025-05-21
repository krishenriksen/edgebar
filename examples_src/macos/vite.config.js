import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  plugins: [solidPlugin()],
  build: {
    target: "esnext",
    minify: "terser",
    terserOptions: {
      compress: true,
      mangle: true,
      format: {
        comments: false,
      },
    },
  },
  base: "./",
});