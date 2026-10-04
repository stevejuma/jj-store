import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { fileURLToPath } from "node:url";
import { uiDevelopmentServer } from "../scripts/dev-ui-config.mjs";
import { documentationPlugin } from "../scripts/docs/assets.mjs";
export default defineConfig({
  root: fileURLToPath(new URL(".", import.meta.url)),
  plugins: [
    tailwindcss(),
    svelte(),
    documentationPlugin(
      fileURLToPath(new URL("../docs/build", import.meta.url)),
    ),
  ],
  resolve: { dedupe: ["svelte", "bits-ui"] },
  worker: { format: "es" },
  // Match the catalog: source-distributed rune modules use the normal compiler.
  optimizeDeps: {
    exclude: ["@lapismd/design-core"],
    include: [
      "@lapismd/design-core > @lapismd/mira",
      "@lapismd/mira/**",
      "debug",
      "extend",
      "@tanstack/svelte-virtual",
    ],
  },
  build: {
    assetsInlineLimit: 0,
    manifest: true,
    sourcemap: false,
    target: "es2022",
  },
  server: uiDevelopmentServer(),
  preview: { host: "127.0.0.1", strictPort: true },
});
