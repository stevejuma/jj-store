import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample, repo } from "./native-browser-fixture.js";
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
async function bootstrap(page: Page, linked = false) {
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
        repositories: linked ? [{ id: repo, name: "Local acceptance" }] : [],
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
  await expect(
    table.getByRole("columnheader", { name: "Repository", exact: true }),
  ).toHaveCount(0);
  const repositoryLink = table
    .locator("tbody tr:first-child td:first-child")
    .getByRole("link");
  await expect(repositoryLink).toHaveAttribute("aria-disabled", "true");
  await expect(repositoryLink).not.toHaveAttribute("href");
  await expect(repositoryLink.locator("svg")).toBeVisible();
  const iconImport = page.getByRole("button", {
    name: "Import history for Codex CLI source 1",
  });
  expect((await iconImport.textContent())?.trim()).toBe("");
  await expect(iconImport).toHaveAttribute("title", "Import history");
  await expect(table.locator("tfoot td")).toHaveAttribute("colspan", "3");
  await expect(table.locator("tfoot time[datetime]")).toBeVisible();
  await statusFilter.click();
  await page.getByRole("option", { name: "Degraded", exact: true }).click();
  await expect(table.locator("tbody > tr")).toHaveCount(1);
  await expect(page.getByText("1 shown", { exact: true })).toBeVisible();
  await expect(
    page.getByText("Codex Desktop source 3", { exact: true }),
  ).toBeVisible();
  const statusSearch = page.getByRole("combobox", {
    name: "Search source statuses",
  });
  await statusSearch.fill("awaiting");
  await statusSearch.press("ArrowDown");
  await statusSearch.press("Enter");
  await expect(table.locator("tbody > tr")).toHaveCount(2);
  await expect(statusFilter).toHaveText("2 statuses");
  await statusSearch.fill("");
  await expect(
    page.getByRole("option", { name: "Degraded", exact: true }),
  ).toHaveAttribute("data-checked", "true");
  await expect(
    page.getByRole("option", { name: "Awaiting activity", exact: true }),
  ).toHaveAttribute("data-checked", "true");
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.screenshot({
    path: test.info().outputPath("capture-status-command.png"),
  });
  await statusSearch.fill("no-matching-status");
  await expect(
    page.getByText("No matching statuses", { exact: true }),
  ).toBeVisible();
  await statusSearch.press("Escape");
  await expect(statusFilter).toBeFocused();
  await statusFilter.click();
  await expect(statusSearch).toHaveValue("");
  await page.getByRole("option", { name: "All statuses", exact: true }).click();
  await expect(page.getByText("2 shown", { exact: true })).toBeVisible();
  await expect(page.getByText("3 loaded", { exact: true })).toBeVisible();
  await expect(page.getByText("5 observed", { exact: true })).toBeVisible();
  await expect(
    page.getByText("Codex Desktop source 3", { exact: true }),
  ).toBeVisible();
  await page
    .getByRole("combobox", { name: "Search source statuses" })
    .press("Escape");
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
  await statusFilter.click();
  const statusPopup = page.getByRole("dialog", {
    name: "Source status filter",
  });
  await expect(statusPopup).toBeVisible();
  const popupBounds = await statusPopup.boundingBox();
  expect(popupBounds!.x).toBeGreaterThanOrEqual(0);
  expect(popupBounds!.x + popupBounds!.width).toBeLessThanOrEqual(390);
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.screenshot({
    path: test.info().outputPath("capture-status-command-mobile.png"),
  });
  await statusSearch.press("Escape");
  await page
    .getByRole("button", { name: "Import history for Codex Desktop source 3" })
    .click();
  await expect(
    page.getByText("completed · Boundary available", { exact: true }),
  ).toBeVisible();
  expect(imported!.selector).toBe(coverage.sources[2].selector);
});

test("cold repository source filtering composes with status and harness filters", async ({
  page,
}) => {
  await bootstrap(page, true);
  const coverage = mixedCaptureSources();
  coverage.sources[0].last_bound_repository = repo;
  coverage.sources[1].last_bound_repository = null;
  coverage.sources[2].last_bound_repository = "de".repeat(16);
  await page.route("**/v1/local/capture**", (r) => {
    const path = new URL(r.request().url()).pathname;
    const value = path.endsWith("/settings")
      ? captureSettings()
      : path.endsWith("/sources")
        ? coverage
        : captureStatus();
    return r.fulfill({ contentType: "application/json", body: pretty(value) });
  });
  await page.goto("/");
  await openAgents(page);
  await page.getByRole("tab", { name: "Sources", exact: true }).click();
  const table = page.getByRole("table", { name: "Capture sources" });
  await expect(table.locator("tbody > tr")).toHaveCount(3);
  await expect(
    page.getByText("Coverage current", { exact: true }),
  ).toBeVisible();
  const repositoryFilter = page.getByRole("button", {
    name: "Filter by source repository",
  });
  await repositoryFilter.click();
  const search = page.getByRole("combobox", {
    name: "Search source repositories",
  });
  await search.fill("Local acceptance");
  await search.press("ArrowDown");
  await search.press("Enter");
  await expect(table.locator("tbody > tr")).toHaveCount(1);
  await expect(
    page.getByText("Codex CLI source 1", { exact: true }),
  ).toBeVisible();
  await search.fill("");
  await page
    .getByRole("option", { name: "Previously bound repository", exact: true })
    .click();
  await expect(repositoryFilter).toHaveText("2 repositories");
  await expect(table.locator("tbody > tr")).toHaveCount(2);
  await page
    .getByRole("option", {
      name: "No recorded repository binding",
      exact: true,
    })
    .click();
  await expect(table.locator("tbody > tr")).toHaveCount(3);
  await expect(page.getByText("3 loaded", { exact: true })).toBeVisible();
  await expect(page.getByText("5 observed", { exact: true })).toBeVisible();
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.screenshot({
    path: test.info().outputPath("source-repository-filter.png"),
  });
  await search.press("Escape");
  await expect(repositoryFilter).toBeFocused();
  await page.getByRole("button", { name: "Filter by harness" }).click();
  await page.getByRole("option", { name: "Codex", exact: true }).click();
  await page.getByRole("button", { name: "Filter by source status" }).click();
  await page.getByRole("option", { name: "Degraded", exact: true }).click();
  await page
    .getByRole("combobox", { name: "Search source statuses" })
    .press("Escape");
  await repositoryFilter.click();
  await expect(search).toHaveValue("");
  await expect(
    page.getByRole("option", {
      name: "No recorded repository binding",
      exact: true,
    }),
  ).toBeVisible();
  await page
    .getByRole("option", { name: "Previously bound repository", exact: true })
    .click();
  await expect(
    page.getByText("No sources match these filters", { exact: true }),
  ).toBeVisible();
  await page
    .getByRole("option", { name: "All repositories", exact: true })
    .click();
  await expect(
    page.getByText("Codex Desktop source 3", { exact: true }),
  ).toBeVisible();
  await search.fill("no-matching-repository");
  await expect(
    page.getByText("No matching repositories", { exact: true }),
  ).toBeVisible();
  await search.press("Escape");
  await page.setViewportSize({ width: 390, height: 844 });
  await repositoryFilter.click();
  await expect(search).toHaveValue("");
  const popup = await page
    .getByRole("dialog", { name: "Source repository filter" })
    .boundingBox();
  expect(popup!.x).toBeGreaterThanOrEqual(0);
  expect(popup!.x + popup!.width).toBeLessThanOrEqual(390);
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.screenshot({
    path: test.info().outputPath("source-repository-filter-mobile.png"),
  });
  await search.press("Escape");
});

test("cold source refresh retains coverage and repository link opens its authorized page", async ({
  page,
}) => {
  await bootstrap(page, true);
  const coverage = mixedCaptureSources();
  coverage.sources[0].last_bound_repository = repo;
  let delay = false;
  let release!: () => void;
  const gate = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route("**/v1/local/capture**", async (r) => {
    const path = new URL(r.request().url()).pathname;
    if (path.endsWith("/sources") && delay) await gate;
    const value = path.endsWith("/settings")
      ? captureSettings()
      : path.endsWith("/sources")
        ? coverage
        : captureStatus();
    return r.fulfill({ contentType: "application/json", body: pretty(value) });
  });
  await page.goto("/");

  await openAgents(page);
  await page.getByRole("tab", { name: "Sources", exact: true }).click();
  const table = page.getByRole("table", { name: "Capture sources" });
  const link = table
    .locator("tbody tr:first-child td:first-child")
    .getByRole("link", { name: "Local acceptance" });
  await expect(link).toHaveAttribute("href", `#/repos/${repo}/files`);
  await expect(table.locator("tbody > tr")).toHaveCount(3);
  const timestamp = await table.locator("tfoot time").getAttribute("datetime");
  delay = true;
  await page
    .getByRole("button", { name: "Refresh capture", exact: true })
    .click();
  await expect(
    table.locator('tfoot [data-ui-component="spinner"]'),
  ).toBeVisible();
  await expect(table.locator("tbody > tr")).toHaveCount(3);
  await expect(table.locator("tfoot time")).toHaveAttribute(
    "datetime",
    timestamp!,
  );
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.screenshot({
    path: test.info().outputPath("source-footer-refreshing.png"),
  });
  release();
  await expect(
    table.locator('tfoot [data-ui-component="spinner"]'),
  ).toHaveCount(0);
  await link.click();
  await expect(page).toHaveURL(new RegExp(`/repos/${repo}/files`));
  await expect(
    page.getByRole("heading", { name: "Agents", level: 1 }),
  ).toHaveCount(0);
});
