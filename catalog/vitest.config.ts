import catalogVite from "../.storybook/vite.config.js";
import { fileURLToPath } from "node:url";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { storybookTest } from "@storybook/addon-vitest/vitest-plugin";
import { playwright } from "@vitest/browser-playwright";
import { defineConfig } from "vitest/config";
export default defineConfig({
  plugins: catalogVite.plugins,
  optimizeDeps: catalogVite.optimizeDeps,
  test: {
    projects: [
      {
        plugins: [svelte()],
        test: {
          name: "catalog-unit",
          include: ["catalog/**/*.test.ts", ".storybook/**/*.test.ts"],
        },
      },
      {
        extends: true,
        plugins: [
          storybookTest({
            configDir: fileURLToPath(new URL("../.storybook", import.meta.url)),
          }),
        ],
        test: {
          name: "storybook",
          setupFiles: [".storybook/vitest.setup.ts"],
          testTimeout: 60000,
          browser: {
            enabled: true,
            headless: true,
            fileParallelism: false,
            provider: playwright({
              contextOptions: { viewport: { width: 1440, height: 1200 } },
            }),
            instances: [
              { browser: "chromium", viewport: { width: 1280, height: 800 } },
            ],
          },
        },
      },
    ],
  },
});
