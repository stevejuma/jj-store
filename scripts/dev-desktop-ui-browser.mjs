import { runUiDevelopment } from "./dev-ui.mjs";
import assert from "node:assert/strict";
import { expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

/** Production screens against the disposable real backend, without native IPC. */
export async function assertDesktopBrowserUi(page) {
  for (const width of [1400, 900]) {
    await page.setViewportSize({ width, height: 1000 });
    const header = page.getByRole("navigation", {
      name: "Application navigation",
    });
    await expect(header).toBeVisible();
    await expect(header).not.toHaveAttribute("data-tauri-drag-region");
    const shell = page.locator("[data-desktop-chrome='macos']");
    await expect(shell).toBeVisible();
    const geometry = await header.evaluate((element) => {
      const toggle = element.querySelector("button");
      const box = element.getBoundingClientRect();
      const button = toggle.getBoundingClientRect();
      return { height: box.height, clearance: button.left - box.left };
    });
    assert.equal(geometry.height, 40);
    assert.ok(geometry.clearance >= 80);
  }
  await page.setViewportSize({ width: 1400, height: 1000 });
  assert.equal(await page.locator("[data-tauri-drag-region]").count(), 0);
  await page.getByRole("button", { name: "Expand left sidebar" }).click();
  await page.getByRole("button", { name: "Help", exact: true }).click();
  const help = page.getByRole("dialog", { name: "Help", exact: true });
  await expect(help).toBeVisible();
  await expect(help.locator("header")).toHaveClass(/\bdesktop\b/);
  await expect(help.locator("header")).not.toHaveAttribute(
    "data-tauri-drag-region",
  );
  await expect(
    page
      .frameLocator("iframe[title='jj-store documentation']")
      .locator("[data-md-component='container']"),
  ).toBeVisible();
  await page.getByRole("button", { name: "Close Help" }).click();
  await page.getByRole("button", { name: "Collapse left sidebar" }).click();
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Appearance", exact: true }).click();
  const dark = page.getByRole("radio", { name: "Dark", exact: true });
  await dark.click();
  await expect(page.locator("html")).toHaveClass(/dark/);
  const glass = page.getByRole("switch", { name: "Glass background" });
  if (!(await glass.isChecked())) await glass.click();
  await expect(page.locator("html")).toHaveAttribute(
    "data-store-glass",
    "false",
  );
  await page.reload();
  await expect(dark).toBeChecked();
  await expect(glass).toBeChecked();
  await expect(page.locator("html")).toHaveAttribute(
    "data-store-glass",
    "false",
  );
  await page.getByRole("button", { name: "Agents", exact: true }).click();
  await expect(
    page.getByRole("switch", { name: "Keep awake while agents are active" }),
  ).toBeDisabled();
  const audit = await new AxeBuilder({ page }).analyze();
  assert.deepEqual(
    audit.violations,
    [],
    "Desktop browser settings must pass Axe",
  );
  await page
    .getByRole("button", { name: "Back to jj-store", exact: true })
    .click();
  assert.equal(
    await page.getByRole("button", { name: /terminal/i }).count(),
    0,
  );
  assert.equal(
    await page.evaluate(() => typeof window.__TAURI_INTERNALS__),
    "undefined",
  );
  assert.equal(
    await page.evaluate(() => typeof window.__JJ_STORE_DESKTOP__),
    "undefined",
  );
}

export async function restoreBrowserPresentation(root, environment, page) {
  const ordinary = await runUiDevelopment(root, environment);
  assert.equal(ordinary.presentation, "browser");
  await page.reload();
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .waitFor();
  assert.equal(await page.locator("[data-desktop-chrome='macos']").count(), 0);
  return ordinary;
}
