import { defineConfig } from "@playwright/test";
import base from "./playwright.config.js";
export default defineConfig(base, {
  testMatch: "help.spec.ts",
  projects: [
    { name: "chromium", use: { browserName: "chromium" } },
    { name: "webkit", use: { browserName: "webkit" } },
  ],
});
