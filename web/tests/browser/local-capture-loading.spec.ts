import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample } from "./native-browser-fixture.js";
import {
  captureStatus,
  captureSettings,
  mixedCaptureSources,
  captureReceipt,
} from "../../../catalog/fixtures/capture-management.js";
import { decode, pretty } from "../../src/lib/api/client.js";
import type { CaptureImport } from "../../src/lib/controller/capture-types.js";

async function openAgents(page: Page) {
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  await page.getByRole("button", { name: "Agents", exact: true }).click();
}
async function bootstrap(page: Page) {
  await api(page, { access: "local_trusted" });
  await page.route("**/v1/bootstrap", (r) =>
    r.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities: {
          ...sample("capabilities"),
          local_capture_management: true,
        },
        storage: { ready: true },
        repositories: [],
        backups: [],
      },
    }),
  );
  await page.route("**/v1/local/repositories", (r) =>
    r.fulfill({ json: { version: 1, bindings: [] } }),
  );
}

test("cold capture loading keeps guarded controls and recovers from busy reads", async ({
  page,
}) => {
  await bootstrap(page);
  let release!: () => void;
  const gate = new Promise<void>((resolve) => {
    release = resolve;
  });
  const attempts = new Map<string, number>();
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await page.route("**/v1/local/capture**", async (r) => {
    await gate;
    expect(r.request().method()).toBe("GET");
    const path = new URL(r.request().url()).pathname;
    const count = (attempts.get(path) ?? 0) + 1;
    attempts.set(path, count);
    if (count <= 2)
      return r.fulfill({
        status: 409,
        json: { version: 1, code: "capture_control_busy" },
      });
    return r.fulfill({
      contentType: "application/json",
      body: pretty(
        path.endsWith("/settings")
          ? captureSettings()
          : path.endsWith("/sources")
            ? mixedCaptureSources()
            : captureStatus(),
      ),
    });
  });
  await page.goto("/");
  await openAgents(page);
  const pause = page.getByRole("button", { name: "Pause capture" });
  await expect(pause).toBeVisible();
  await expect(pause).toBeDisabled();
  await expect(
    page.getByRole("button", { name: "Rescan capture" }),
  ).toBeDisabled();
  await expect(
    page.getByRole("status", { name: "Loading capture status" }),
  ).toBeVisible();
  await page.getByRole("tab", { name: "Configuration", exact: true }).click();
  await expect(
    page.getByRole("status", { name: "Loading capture configuration" }),
  ).toBeVisible();
  await page.getByRole("tab", { name: "Sources", exact: true }).click();
  await expect(
    page.getByRole("status", { name: "Loading capture sources" }),
  ).toBeVisible();
  await page.screenshot({
    path: test.info().outputPath("capture-loading.png"),
  });
  release();
  await expect(
    page.getByRole("table", { name: "Capture sources" }),
  ).toBeVisible();
  await expect(page.getByText("5 observed", { exact: true })).toBeVisible();
  await expect(
    page.getByRole("status", { name: "Loading capture sources" }),
  ).toHaveCount(0);
  await page.getByRole("tab", { name: "Capture", exact: true }).click();
  await expect(pause).toBeEnabled();
  await expect(
    page.getByText("Capture request unavailable", { exact: true }),
  ).toHaveCount(0);
  expect([...attempts.values()]).toEqual([3, 3, 3]);
  expect(errors).toEqual([]);
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
});

test("cold grouped Codex configuration and harness-filtered source table retain identities", async ({
  page,
}) => {
  await bootstrap(page);
  let imported: CaptureImport | null = null;
  const coverage = mixedCaptureSources();
  await page.route("**/v1/local/capture**", async (r) => {
    const path = new URL(r.request().url()).pathname;
    let value: unknown = captureStatus();
    if (path.endsWith("/settings")) value = captureSettings();
    if (path.endsWith("/sources")) value = coverage;
    if (path.endsWith("/import")) {
      imported = decode(r.request().postData()!) as CaptureImport;
      const result = captureReceipt(imported);
      const source = coverage.sources.find(
        (s) => s.selector === imported!.selector,
      )!;
      result.receipt.profile = source.profile;
      result.receipt.product_version = source.product_version;
      value = result;
    }
    return r.fulfill({ contentType: "application/json", body: pretty(value) });
  });
  await page.goto("/");
  await openAgents(page);
  await page.getByRole("tab", { name: "Configuration", exact: true }).click();
  const group = page.getByRole("button", { name: "Codex configuration" });
  const roots = page.getByRole("textbox", { name: "Codex CLI folders" });
  await roots.fill("/tmp/grouped-browser/sessions");
  await group.click();
  await expect(group).toHaveAttribute("aria-expanded", "false");
  await expect(roots).toBeHidden();
  await expect(
    page.getByRole("spinbutton", { name: "Concurrent readers" }),
  ).toBeVisible();
  await expect(
    page.getByRole("switch", { name: "Enable automatic capture" }),
  ).toBeVisible();
  await group.press("Enter");
  await expect(roots).toHaveValue("/tmp/grouped-browser/sessions");
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.screenshot({
    path: test.info().outputPath("codex-configuration.png"),
  });
  await page.getByRole("tab", { name: "Sources", exact: true }).click();
  const table = page.getByRole("table", { name: "Capture sources" });
  await expect(table.locator("tbody > tr")).toHaveCount(3);
  const filter = page.getByRole("button", { name: "Filter by harness" });
  await filter.click();
  await page.getByRole("option", { name: "Codex", exact: true }).click();
  await expect(table.locator("tbody > tr")).toHaveCount(2);
  const statusFilter = page.getByRole("button", {
    name: "Filter by source status",
  });
  await expect(
    filter.locator(
      "xpath=ancestor::div[contains(@class, 'store-capture-sources__table')]",
    ),
  ).toHaveCount(0);
  await expect(
    statusFilter.locator(
      "xpath=ancestor::div[contains(@class, 'store-capture-sources__table')]",
    ),
  ).toHaveCount(0);
  const filterBounds = await statusFilter.boundingBox();
  const tableBounds = await table.boundingBox();
  expect(filterBounds!.y + filterBounds!.height).toBeLessThan(tableBounds!.y);
  expect(
    Math.abs(
      filterBounds!.x +
        filterBounds!.width -
        tableBounds!.x -
        tableBounds!.width,
    ),
  ).toBeLessThan(2);
  await expect(
    table.locator(
      'tbody tr:first-child td:first-child img[src="/agent-icons/codex.svg"]',
    ),
  ).toBeVisible();
  const iconImport = page.getByRole("button", {
    name: "Import history for Codex CLI source 1",
  });
  expect((await iconImport.textContent())?.trim()).toBe("");
  await expect(iconImport).toHaveAttribute("title", "Import history");
  await expect(table.locator("tfoot td")).toHaveAttribute("colspan", "4");
  await expect(table.locator("tfoot time[datetime]")).toBeVisible();
  await statusFilter.click();
  await page.getByRole("option", { name: "Degraded", exact: true }).click();
  await expect(table.locator("tbody > tr")).toHaveCount(1);
  await expect(page.getByText("1 shown", { exact: true })).toBeVisible();
  await expect(
    page.getByText("Codex Desktop source 3", { exact: true }),
  ).toBeVisible();
  await statusFilter.click();
  await page.getByRole("option", { name: "All statuses", exact: true }).click();
  await expect(page.getByText("2 shown", { exact: true })).toBeVisible();
  await expect(page.getByText("3 loaded", { exact: true })).toBeVisible();
  await expect(page.getByText("5 observed", { exact: true })).toBeVisible();
  await expect(
    page.getByText("Codex Desktop source 3", { exact: true }),
  ).toBeVisible();
  await filter.click();
  await page.getByRole("option", { name: "OpenCode", exact: true }).click();
  await expect(
    page.getByText("No sources match these filters", { exact: true }),
  ).toBeVisible();
  await filter.click();
  await page.getByRole("option", { name: "Cursor", exact: true }).click();
  await expect(
    page.getByRole("button", {
      name: "Import history for Cursor Desktop source 2",
    }),
  ).toBeDisabled();
  await filter.click();
  await page
    .getByRole("option", { name: "All harnesses", exact: true })
    .click();
  const styles = await page
    .locator(".store-capture-sources__table")
    .evaluate((e) => {
      const style = getComputedStyle(e);
      return {
        border: style.borderTopWidth,
        radius: style.borderTopLeftRadius,
      };
    });
  expect(styles.border).toBe("1px");
  expect(parseFloat(styles.radius)).toBeGreaterThan(0);
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.screenshot({
    path: test.info().outputPath("capture-sources-table.png"),
  });
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.screenshot({
    path: test.info().outputPath("capture-sources-mobile.png"),
  });
  await page
    .getByRole("button", { name: "Import history for Codex Desktop source 3" })
    .click();
  await expect(
    page.getByText("completed · Boundary available", { exact: true }),
  ).toBeVisible();
  expect(imported!.selector).toBe(coverage.sources[2].selector);
});
