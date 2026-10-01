import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { fileURLToPath } from "node:url";
export default defineConfig({
  root: fileURLToPath(new URL(".", import.meta.url)),
  plugins: [tailwindcss(), svelte()],
  resolve: { dedupe: ["svelte", "bits-ui"] },
  worker: { format: "es" },
  build: {
    assetsInlineLimit: 0,
    manifest: true,
    sourcemap: false,
    target: "es2022",
  },
  server: { host: "127.0.0.1", strictPort: true },
  preview: { host: "127.0.0.1", strictPort: true },
});
