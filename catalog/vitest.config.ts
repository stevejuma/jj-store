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
          server: { deps: { inline: ["@lapismd/design-core"] } },
          include: [
            "catalog/**/*.test.ts",
            ".storybook/**/*.test.ts",
            "web/tests/vite/**/*.test.ts",
          ],
        },
      },
      {
        extends: true,
        plugins: [
          storybookTest({
            configDir: fileURLToPath(new URL("../.storybook", import.meta.url)),
            // Register: spec/src/current/governance/09-catalog-validation.md.
            tags: { skip: ["quarantined"] },
          }),
        ],
        test: {
          name: "storybook",
          bail:
            process.env.JJ_STORE_CI_REPORTS === "1"
              ? 0
              : process.env.CI === "true"
                ? 1
                : 0,
          setupFiles: [".storybook/vitest.setup.ts"],
          testTimeout: 60000,
          browser: {
            enabled: true,
            headless: true,
            fileParallelism: false,
            provider: playwright({
              ...(process.env.PW_TEST_CONNECT_WS_ENDPOINT
                ? {
                    connectOptions: {
                      wsEndpoint: process.env.PW_TEST_CONNECT_WS_ENDPOINT,
                      exposeNetwork: "<loopback>",
                    },
                  }
                : {}),
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
