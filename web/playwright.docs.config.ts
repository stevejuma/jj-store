import { defineConfig } from "@playwright/test";
import base from "./playwright.config.js";
export default defineConfig(base, {
  outputDir: "./test-results-help",
  testMatch: "help*.spec.ts",
  testIgnore: [],
  projects: [
    { name: "chromium", use: { browserName: "chromium" } },
    { name: "webkit", use: { browserName: "webkit" } },
  ],
});
