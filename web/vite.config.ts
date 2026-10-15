import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { fileURLToPath } from "node:url";
import { uiDevelopmentServer } from "../scripts/dev-ui-config.mjs";
import { documentationPlugin } from "../scripts/docs/assets.mjs";
import { viewModuleUrls } from "../scripts/view-module-urls.mjs";
export default defineConfig({
  root: fileURLToPath(new URL(".", import.meta.url)),
  plugins: [
    viewModuleUrls(
      fileURLToPath(new URL(".", import.meta.url)).replace(/\/$/, ""),
    ),
    tailwindcss(),
    svelte(),
    documentationPlugin(
      fileURLToPath(new URL("../docs/build", import.meta.url)),
    ),
  ],
  resolve: { dedupe: ["svelte", "bits-ui"] },
  worker: { format: "es" },
  // The Svelte plugin prebundles package components and rune modules together.
  optimizeDeps: {
    include: [
      "@lapismd/design-core/shell",
      "@lapismd/design-core/forms",
      "@lapismd/design-core/ai",
      "@lapismd/design-core/diff",
      "@lapismd/design-core/workspace",
      "@lapismd/design-core/filter",
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
    cssCodeSplit: false,
    rolldownOptions: {
      preserveEntrySignatures: "strict",
      output: {
        codeSplitting: {
          includeDependenciesRecursively: true,
          groups: [
            {
              name: "editor-languages",
              test: /node_modules\/@codemirror\/(?:legacy-modes|lang-[^/]+)\//,
            },
            {
              name: "icons",
              test: /node_modules\/@lucide\/svelte\//,
            },
            {
              name: "design-shared",
              test: /node_modules\/@lapismd\/design-core\/dist\/shared\//,
            },
          ],
        },
      },
    },
  },
  server: uiDevelopmentServer(),
  preview: { host: "127.0.0.1", strictPort: true },
});
