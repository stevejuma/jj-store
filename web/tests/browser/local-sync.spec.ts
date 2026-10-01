import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample } from "./native-browser-fixture.js";
import { syncWorker } from "../../../catalog/fixtures/sync-settings.js";

test("local sync controls and grouped progress preserve repository context", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/capabilities", (r) =>
    r.fulfill({
      json: { ...sample("capabilities"), local_sync_management: true },
    }),
  );
  const worker = syncWorker();
  worker.local_frontier = {
    replica: "ab".repeat(16),
    log_epoch: "cd".repeat(16),
    through: 7,
    observed_upper_bound: 9,
  };
  await page.route("**/local/sync", (r) =>
    r.fulfill({ json: { version: 1, configured: true, worker } }),
  );
  const actions: string[] = [];
  await page.route("**/local/sync/control", async (r) => {
    expect(r.request().headers()["authorization"]).toBeUndefined();
    expect(r.request().headers()["x-jj-store-protocol"]).toBe("1");
    const body = r.request().postDataJSON();
    expect(body.version).toBe(1);
    actions.push(body.action);
    worker.phase = body.action === "pause" ? "paused" : "running";
    await r.fulfill({ json: { version: 1, configured: true, worker } });
  });
  await page.goto("/");
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  const original = await page.evaluate(() => location.hash);
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  await expect(
    page
      .getByRole("navigation", { name: "Settings page navigation" })
      .getByRole("button", { name: "Back to app" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Sync", exact: true }).click();
  const tabs = page.getByRole("tablist", { name: "Sync settings sections" });
  const tabBounds = (await tabs.boundingBox())!;
  const contentBounds = (await page
    .getByRole("region", { name: "Settings content" })
    .boundingBox())!;
  const navigation = page.getByRole("navigation", {
    name: "Settings page navigation",
  });
  const headingBounds = (await navigation
    .getByRole("heading", { name: "Sync", level: 1 })
    .boundingBox())!;
  expect(tabBounds.y - contentBounds.y).toBeLessThan(16);
  expect(headingBounds.x + headingBounds.width).toBeLessThan(tabBounds.x);
  await expect(navigation.locator("h1 svg[aria-hidden='true']")).toBeVisible();
  await expect(page.locator(".store-settings h1")).toHaveCount(0);
  await page.getByRole("button", { name: "Pause sync", exact: true }).click();
  await expect(page.getByRole("button", { name: "Retry sync" })).toBeDisabled();
  await page.getByRole("button", { name: "Resume sync" }).click();
  await page.getByRole("button", { name: "Retry sync" }).click();
  expect(actions).toEqual(["pause", "resume", "retry"]);
  await page.getByRole("tab", { name: "Progress" }).click();
  await expect(page.getByText("Local pending positions")).toBeVisible();
  await expect(
    page.getByRole("button", { name: /^Copy full Local frontier replica:/ }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: /^Copy full Local frontier epoch:/ }),
  ).toBeVisible();
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  expect(await new AxeBuilder({ page }).analyze()).toMatchObject({
    violations: [],
  });
  await page.setViewportSize({ width: 1400, height: 1000 });
  await page.getByRole("button", { name: "Back to app" }).click();
  expect(await page.evaluate(() => location.hash)).toBe(original);
});
