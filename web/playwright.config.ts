import { defineConfig } from "@playwright/test";
import { previewCommand } from "../scripts/browser-tests.mjs";
export default defineConfig({
  testDir: "./tests/browser",
  testIgnore: ["**/help*.spec.ts"],
  outputDir: process.env.JJ_STORE_BROWSER_SHARD
    ? `./test-results-browser-${process.env.JJ_STORE_BROWSER_SHARD.replace("/", "-")}`
    : "./test-results",
  fullyParallel: false,
  workers: 1,
  timeout: 30000,
  retries: 0,
  maxFailures:
    process.env.JJ_STORE_CI_REPORTS === "1"
      ? 0
      : process.env.CI === "true"
        ? 1
        : 0,
  reporter:
    process.env.JJ_STORE_CI_REPORTS === "1"
      ? [["line"], ["junit"]]
      : process.env.CI
        ? "line"
        : "list",
  use: {
    baseURL: "http://127.0.0.1:7458",
    browserName: "chromium",
    viewport: { width: 1400, height: 1000 },
    trace: "retain-on-failure",
    screenshot: "only-on-failure",
  },
  webServer: {
    command: process.env.JJ_STORE_BROWSER_PREVIEW_EXECUTABLE
      ? previewCommand(process.env.JJ_STORE_BROWSER_PREVIEW_EXECUTABLE)
      : process.env.CI === "true" && process.env.JJ_STORE_CI_PREBUILT === "1"
        ? "target/debug/examples/browser_preview"
        : "node scripts/managed-cargo.mjs run --quiet --locked -p jj-store-http --example browser_preview",
    url: "http://127.0.0.1:7458",
    reuseExistingServer: false,
    timeout: 30000,
    cwd: "..",
  },
});
