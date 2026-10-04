import { defineConfig } from "@playwright/test";

const baseURL = process.env.JJ_STORE_REAL_BASE_URL;
if (!baseURL) throw new Error("JJ_STORE_REAL_BASE_URL is required");

export default defineConfig({
  testDir: "./tests/real",
  outputDir: "./test-results-real",
  fullyParallel: false,
  workers: 1,
  timeout: 60000,
  retries: 0,
  use: {
    baseURL,
    browserName: "chromium",
    viewport: { width: 1400, height: 1000 },
    trace: "retain-on-failure",
    screenshot: "off",
  },
});
