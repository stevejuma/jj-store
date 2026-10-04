import { defineConfig } from "@playwright/test";
export default defineConfig({
  testDir: "./tests/browser",
  outputDir: "./test-results",
  fullyParallel: false,
  workers: 1,
  timeout: 30000,
  retries: 0,
  use: {
    baseURL: "http://127.0.0.1:7458",
    browserName: "chromium",
    viewport: { width: 1400, height: 1000 },
    trace: "retain-on-failure",
    screenshot: "only-on-failure",
  },
  webServer: {
    command:
      "cargo run --quiet --locked -p jj-store-http --example browser_preview",
    url: "http://127.0.0.1:7458",
    reuseExistingServer: false,
    timeout: 30000,
    cwd: "..",
  },
});
