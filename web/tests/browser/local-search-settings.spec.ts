import {
  repositorySettings,
  repositorySettingsLayout,
} from "./repository-settings-navigation.js";
import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample } from "./native-browser-fixture.js";
import { searchSettingsValue } from "../../../catalog/fixtures/search-settings.js";
test("Search tabs preserve drafts and separate Save from Apply", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/capabilities", (r) =>
    r.fulfill({
      json: { ...sample("capabilities"), local_search_management: true },
    }),
  );
  const value = searchSettingsValue();
  const snapshot = sample("snapshot");
  await page.route("**/search/status", (r) =>
    r.fulfill({
      json: {
        version: 1,
        repository: snapshot.repo_id,
        epoch: snapshot.epoch,
        repository_position: snapshot.published_seq,
        durable_position: snapshot.published_seq,
        indexed_position: snapshot.published_seq,
        index_generation: "aa".repeat(32),
        is_stale: false,
        degraded: false,
      },
    }),
  );
  let saves = 0,
    applies = 0;
  await page.route("**/local/search", (r) => r.fulfill({ json: value }));
  await page.route("**/local/search/save", async (r) => {
    expect(r.request().headers()["authorization"]).toBeUndefined();
    expect(r.request().headers()["x-jj-store-protocol"]).toBe("1");
    const body = r.request().postDataJSON();
    expect(body.expected_revision).toBe(value.revision);
    saves++;
    if (saves === 1) {
      value.revision = "cd".repeat(32);
      await r.fulfill({
        status: 409,
        json: { code: "search_settings_changed" },
      });
      return;
    }
    value.desired = body.configuration;
    value.revision = "ef".repeat(32);
    await r.fulfill({ json: value });
  });
  await page.route("**/local/search/control", async (r) => {
    applies++;
    expect(r.request().postDataJSON()).toEqual({
      version: 1,
      expected_revision: value.revision,
      action: "apply",
    });
    value.effective = structuredClone(value.desired);
    await r.fulfill({ json: value });
  });
  await page.goto("/");
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  const hash = await page.evaluate(() => location.hash);
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  await expect(
    page
      .getByRole("navigation", { name: "Settings navigation" })
      .getByRole("button", { name: "Back to jj-store" }),
  ).toBeVisible();
  await repositorySettings(page, "Search");
  const tabs = await repositorySettingsLayout(page, "Search settings sections");
  const picker = page.getByRole("combobox", { name: "Settings repository" });
  await page.getByRole("tab", { name: "Configuration", exact: true }).focus();
  await page.keyboard.press("ArrowRight");
  await expect(
    page.getByRole("tab", { name: "File scopes", exact: true }),
  ).toBeFocused();
  await expect(
    page.getByRole("tab", { name: "File scopes", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  const prefixes = page.getByRole("textbox", { name: "Exclude path prefixes" });
  await prefixes.fill("private");
  await expect(picker).toHaveCount(0);
  await page.getByRole("tab", { name: "Health", exact: true }).click();
  await expect(
    page.getByRole("heading", { name: "Text-search health" }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: /^Copy full Index generation:/ }),
  ).toBeVisible();
  await page.getByRole("tab", { name: "File scopes", exact: true }).click();
  await expect(prefixes).toHaveValue("private");
  await repositorySettings(page, "Repository");
  await repositorySettings(page, "Search");
  await page.getByRole("tab", { name: "File scopes", exact: true }).click();
  await expect(prefixes).toHaveValue("private");
  await page
    .getByRole("button", { name: "Save settings", exact: true })
    .click();
  await expect(page.getByText(/search settings changed/)).toBeVisible();
  await expect(prefixes).toHaveValue("private");
  await page
    .getByRole("button", { name: "Refresh settings", exact: true })
    .click();
  await expect(prefixes).toHaveValue("private");
  // Refresh preserves the draft's original revision. Cancel explicitly adopts
  // the newer configuration before re-entering and saving these changes.
  await page
    .getByRole("button", { name: "Cancel changes", exact: true })
    .click();
  await expect(prefixes).toHaveValue("");
  await prefixes.fill("private");
  await page
    .getByRole("button", { name: "Save settings", exact: true })
    .click();
  await expect(page.getByText(/Saved changes need applying/)).toBeVisible();
  expect(applies).toBe(0);
  await page.getByRole("button", { name: "Apply saved settings" }).click();
  await expect(page.getByText(/Matches saved settings/)).toBeVisible();
  expect(applies).toBe(1);
  await expect(picker).toHaveCount(0);
  await page.setViewportSize({ width: 390, height: 844 });
  await expect(
    page.locator('[data-ui-component="app-shell"][data-ui-part="root"]'),
  ).toHaveAttribute("data-display-mode", "mobile");
  await expect(picker).toHaveCount(0);
  const mobileTabsTop = (await tabs.boundingBox())!.y;
  const scroller = page
    .locator(".store-repository-settings__page:visible")
    .locator('[data-ui-part="scroll-area-viewport"]')
    .first();
  await scroller.evaluate((element) => {
    element.scrollTop = 240;
  });
  await expect
    .poll(() => scroller.evaluate((element) => element.scrollTop))
    .toBeGreaterThan(0);
  await expect
    .poll(async () => Math.abs((await tabs.boundingBox())!.y - mobileTabsTop))
    .toBeLessThan(1);
  const health = page.getByRole("tab", { name: "Health", exact: true });
  if (!(await health.isVisible())) {
    const more = page.getByRole("button", {
      name: "More search settings sections",
      exact: true,
    });
    await more.click();
    await page.getByRole("menuitem", { name: "Health", exact: true }).click();
    await expect(more).toHaveAttribute("aria-current", "page");
  } else {
    await health.click();
    await expect(health).toHaveAttribute("aria-selected", "true");
  }
  await expect(
    page.getByRole("heading", { name: "Text-search health", exact: true }),
  ).toBeVisible();
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  expect(await new AxeBuilder({ page }).analyze()).toMatchObject({
    violations: [],
  });
  await page.setViewportSize({ width: 1400, height: 1000 });
  await page.getByRole("button", { name: "Back to repository" }).click();
  expect(await page.evaluate(() => location.hash)).toBe(hash);
});
