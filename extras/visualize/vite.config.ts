import { resolve } from "node:path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        "2024-day14": resolve(__dirname, "2024/day14/index.html"),
        "2024-day18": resolve(__dirname, "2024/day18/index.html"),
      },
    },
  },
  base: process.env.VITE_BASENAME,
});
