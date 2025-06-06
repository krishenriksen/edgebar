import path from "path";
import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

export default defineConfig({
  plugins: [
    solid(),
  ],
  server: {
    port: 4200,
  },
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
  publicDir: './resources',
  base: "./",
  resolve: {
    alias: {
      "~": path.resolve(__dirname, "./src/app"),
    },
  },
});