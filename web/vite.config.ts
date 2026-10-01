import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { tursoBundleBuildPlugin } from "@lapismd/lapis-community/build/vite";
import { fileURLToPath } from "node:url";
export default defineConfig({
  root: fileURLToPath(new URL(".", import.meta.url)),
  plugins: [tursoBundleBuildPlugin(), tailwindcss(), svelte()],
  worker: { format: "es", plugins: () => [tursoBundleBuildPlugin()] },
  build: { manifest: true, sourcemap: false, target: "es2022" },
  server: { host: "127.0.0.1", strictPort: true },
  preview: { host: "127.0.0.1", strictPort: true },
});
