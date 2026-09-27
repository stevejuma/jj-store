import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
export default defineConfig({
  plugins: [svelte()],
  // Process source-distributed Svelte rune modules through the normal compiler.
  optimizeDeps: {
    exclude: [
      "@lapismd/design-core",
      "@lapismd/design-core/shell",
      "@lapismd/design-core/shadcn/column-canvas",
    ],
  },
});
