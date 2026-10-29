import { test } from "@playwright/test";
import { defineVisualSuite } from "@lapismd/storybook-addon-visual-delta/playwright";

test.beforeEach(async ({ page }) => {
  await page.addInitScript(() => {
    document.addEventListener(
      "DOMContentLoaded",
      () => {
        document.documentElement.setAttribute(
          "data-fixture-controls-capture",
          "",
        );
      },
      { once: true },
    );
  });
});

defineVisualSuite();
