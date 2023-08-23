/// <reference types="vitest" />
/// <reference types="vite/client" />

import { configDefaults, defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    sourcemap: true,
  },
  plugins: [react()],
  test: {
    globals: true,
    environment: "jsdom",
    css: true,
    setupFiles: "./__tests__/setup.ts",
    include: ['./__tests__/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    exclude: [...configDefaults.exclude],
    coverage: {
      provider: "v8",
      reporter: ["text"], // "html", "json"
      reportsDirectory: "./__tests__/coverageReports",
    },
  },
  server: {
    host: true,
    port: 3000,
  },
});