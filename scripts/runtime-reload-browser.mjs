/** Cold built App reload acceptance; isolated instance input only. */
import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
import { chromium, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
let input = "";
for await (const chunk of process.stdin) input += chunk;
const fixture = JSON.parse(input),
  errors = [],
  csp = [];
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({
  viewport: { width: 1200, height: 900 },
});
const page = await context.newPage();
page.setDefaultTimeout(20000);
page.on("pageerror", (e) => errors.push(e.message));
await page.addInitScript(() => {
  globalThis.__reloadCsp = [];
  document.addEventListener("securitypolicyviolation", (e) =>
    globalThis.__reloadCsp.push(e.violatedDirective),
  );
});
try {
  await page.goto(`${fixture.origin}/#/settings?section=service&tab=limits`);
  const field = page.getByRole("spinbutton", {
    name: "Concurrent requests",
    exact: true,
  });
  await expect(field).toHaveValue(
    String(fixture.configuration.concurrent_requests),
  );
  await field.fill("7");
  await expect(
    page.getByRole("button", { name: "Reload saved settings", exact: true }),
  ).toHaveCount(0);
  const sidebar = page.getByLabel("Settings sidebar", { exact: true });
  const help = await sidebar
    .getByRole("button", { name: "Help", exact: true })
    .boundingBox();
  const version = await sidebar
    .locator('[aria-label^="jj-store version"]')
    .boundingBox();
  assert(help && version && version.y >= help.y + help.height);

  const trigger = page.getByRole("button", {
    name: "Application menu",
    exact: true,
  });
  await trigger.focus();
  await page.keyboard.press("Enter");
  await page.getByRole("menuitem", { name: "Reload saved settings" }).click();
  const dialog = page.getByRole("dialog", { name: "Reload saved settings" });
  await dialog.waitFor();
  await expect(
    dialog.getByText("Saved changes awaiting reload", { exact: true }),
  ).toBeVisible();
  const violations = async () =>
    assert.deepEqual(
      (
        await new AxeBuilder({ page }).include('[role="dialog"]').analyze()
      ).violations.map((v) => ({
        id: v.id,
        nodes: v.nodes.map((n) => n.target),
      })),
      [],
    );
  await violations();
  await dialog
    .getByRole("button", { name: "Reload saved settings", exact: true })
    .click();
  await expect(dialog.getByText("Applied", { exact: true })).toBeVisible();
  await mkdir(".local/runtime-reload-review", { recursive: true });
  await page.screenshot({ path: ".local/runtime-reload-review/light.png" });
  await page.emulateMedia({ colorScheme: "dark" });
  await page.waitForFunction(
    () => document.documentElement.dataset.storeTheme === "buzz-dark",
  );
  await dialog.evaluate(async (e) => {
    await Promise.all(
      e
        .getAnimations({ subtree: true })
        .filter((a) => a.effect?.getComputedTiming().iterations !== Infinity)
        .map((a) => a.finished.catch(() => {})),
    );
  });
  await page.screenshot({ path: ".local/runtime-reload-review/dark.png" });

  await violations();
  await page.setViewportSize({ width: 390, height: 844 });
  await expect(dialog).toBeVisible();
  assert(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  );
  await violations();
  await page.screenshot({ path: ".local/runtime-reload-review/mobile.png" });
  await page.keyboard.press("Escape");
  await dialog.waitFor({ state: "hidden" });
  await expect(trigger).toBeFocused();
  await expect(field).toHaveValue("7");
  const url = page.url();
  await page.reload();
  assert.equal(page.url(), url);
  await expect(field).toHaveValue(
    String(fixture.configuration.concurrent_requests),
  );
  await page.goto(
    `${fixture.origin}/#/settings?section=agents&tab=configuration`,
  );
  // Capture controls retain canonical bounds even while their display units change.
  for (const [name, min, max] of [
    ["Concurrent readers slider", "1", "4"],
    ["Discovery interval (seconds) slider", "1", "30"],
    ["Tracked source limit slider", "1", "4096"],
    ["Pending byte budget slider", "1", "67108864"],
  ]) {
    const control = page.getByRole("slider", { name, exact: true });
    await expect(control).toHaveAttribute("aria-valuemin", min);
    await expect(control).toHaveAttribute("aria-valuemax", max);
  }
  for (const [name, unit, endpoints] of [
    ["Discovery interval (seconds)", "ms", ["1,000 ms", "30,000 ms"]],
    ["Pending byte budget", "MiB", ["0 MiB", "64 MiB"]],
  ]) {
    const slider = page.getByRole("slider", {
      name: `${name} slider`,
      exact: true,
    });
    const original = await slider.getAttribute("aria-valuenow");
    await page
      .getByRole("button", { name: `${name} unit`, exact: true })
      .click();
    await page.getByRole("option", { name: unit, exact: true }).click();
    const row = page.locator(".store-numeric-setting").filter({ has: slider });
    await expect(row.locator(".store-numeric-setting__range span")).toHaveText(
      endpoints,
    );
    await expect(slider).toHaveAttribute("aria-valuenow", original);
    await expect(
      page.getByRole("button", { name: "Save configuration", exact: true }),
    ).toBeDisabled();
    await expect(
      page.getByRole("button", { name: "Cancel changes", exact: true }),
    ).toBeDisabled();
  }
  assert(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  );
  await page
    .getByRole("heading", { name: "Capture options", exact: true })
    .scrollIntoViewIfNeeded();
  await page.screenshot({
    path: ".local/runtime-reload-review/cold-capture-unit-labels.png",
  });
  assert.deepEqual(
    (
      await new AxeBuilder({ page })
        .include('[aria-label="Agent capture settings"]')
        .analyze()
    ).violations.map((v) => v.id),
    [],
  );
  csp.push(...(await page.evaluate(() => globalThis.__reloadCsp)));
  assert.deepEqual(errors, []);
  assert.deepEqual(csp, []);
  console.log(
    "PASS cold built App: reload UI, retained unsaved draft, restored deep link, keyboard, light/dark/mobile, unit-aware Capture sliders, Axe and CSP",
  );
} finally {
  await browser.close();
}
