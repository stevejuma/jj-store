import { defineConfig } from "@playwright/test";

const baseURL = process.env.JJ_STORE_REAL_BASE_URL;
if (!baseURL) throw new Error("JJ_STORE_REAL_BASE_URL is required");

export default defineConfig({
  testDir: "./tests/real",
  testIgnore:
    process.env.JJ_STORE_REAL_SKIP_PROFILE === "1"
      ? ["**/profiles-emoji.spec.ts"]
      : [],
  outputDir: process.env.JJ_STORE_REAL_SHARD
    ? `./test-results-real-${process.env.JJ_STORE_REAL_SHARD.replace("/", "-")}`
    : "./test-results-real",
  fullyParallel: false,
  workers: 1,
  timeout: 60000,
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
    baseURL,
    browserName: "chromium",
    viewport: { width: 1400, height: 1000 },
    trace: "retain-on-failure",
    screenshot: "off",
  },
});
