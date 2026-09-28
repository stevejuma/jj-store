import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
export default defineConfig({
  plugins: [tailwindcss(), svelte()],
  // Process source-distributed Svelte rune modules through the normal compiler.
  optimizeDeps: {
    include: [
      "@lucide/svelte/icons/book-open",
      "@lucide/svelte/icons/check",
      "@lucide/svelte/icons/chevron-left",
      "@lucide/svelte/icons/chevron-right",
      "@lucide/svelte/icons/columns-2",
      "@lucide/svelte/icons/copy",
      "@lucide/svelte/icons/folder-git-2",
      "@lucide/svelte/icons/git-branch",
      "@lucide/svelte/icons/git-commit-horizontal",
      "@lucide/svelte/icons/history",
      "@lucide/svelte/icons/panel-right",
      "@lucide/svelte/icons/rows-2",
      "@lucide/svelte/icons/triangle-alert",
      "@lucide/svelte/icons/wrap-text",
      "@lucide/svelte/icons/log-in",
      "@lucide/svelte/icons/log-out",
      "@lucide/svelte/icons/x",
      "@lucide/svelte/icons/code-xml",
      "@lucide/svelte/icons/eye",
      "@lucide/svelte/icons/file",
      "@lucide/svelte/icons/file-code-2",
      "@lucide/svelte/icons/file-search",
      "@lucide/svelte/icons/file-warning",
      "@lucide/svelte/icons/folder",
      "@lucide/svelte/icons/chevron-down",
      "@lucide/svelte/icons/loader-circle",
      "@lucide/svelte/icons/search",
      "@tanstack/svelte-virtual",
    ],
    exclude: [
      "@lapismd/design-core",
      "@lapismd/design-core/shell",
      "@lapismd/design-core/shadcn/column-canvas",
    ],
  },
});
