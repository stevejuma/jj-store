import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample, mutationSample } from "./native-browser-fixture.js";
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
      .getByRole("navigation", { name: "Settings navigation" })
      .getByRole("button", { name: "Back to jj-store" }),
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
  await page.getByRole("button", { name: "Back to jj-store" }).click();
  expect(await page.evaluate(() => location.hash)).toBe(original);
});

test("captured sync targets survive Settings navigation and timeout", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/capabilities", (r) =>
    r.fulfill({
      json: { ...sample("capabilities"), local_sync_management: true },
    }),
  );
  await page.route("**/local/sync", (r) =>
    r.fulfill({ json: { version: 1, configured: true, worker: syncWorker() } }),
  );
  const barrier = {
    version: 1,
    repository: "ab".repeat(16),
    local_replica: "01".repeat(16),
    peer_replica: "02".repeat(16),
    push_epoch: "03".repeat(16),
    pull_epoch: "04".repeat(16),
    push_through: 9,
    pull_through: 12,
  };
  let captures = 0,
    waits = 0;
  await page.route("**/local/sync/barrier", (r) => {
    captures++;
    return r.fulfill({ json: barrier });
  });
  await page.route("**/local/sync/wait", (r) => {
    waits++;
    expect(r.request().postDataJSON().barrier).toEqual(barrier);
    return r.fulfill({
      json: { version: 1, barrier, reached: waits > 1, status: syncWorker() },
    });
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
  await page.getByRole("button", { name: "Sync", exact: true }).click();
  await page.getByRole("tab", { name: "Progress" }).click();
  await page.getByRole("button", { name: "Wait for current sync" }).click();
  await expect(
    page.getByText("Captured target is still pending"),
  ).toBeVisible();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("button", { name: "Sync", exact: true }).click();
  await page.getByRole("tab", { name: "Progress" }).click();
  await page.getByRole("button", { name: "Continue waiting" }).click();
  await expect(page.getByText("Captured target reached")).toBeVisible();
  expect(captures).toBe(1);
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  expect(await page.evaluate(() => location.hash)).toBe(hash);
});

test("Sync reconciliation uses fresh heads and keeps the historical route after unknown recovery", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/capabilities", (r) =>
    r.fulfill({
      json: {
        ...sample("capabilities"),
        local_sync_management: true,
        typed_mutations: ["reconcile_operations"],
      },
    }),
  );
  await page.route("**/local/sync", (r) =>
    r.fulfill({ json: { version: 1, configured: true, worker: syncWorker() } }),
  );
  await page.goto("/");
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  const hash = await page.evaluate(() => location.hash);
  const fresh = {
    ...sample("snapshot"),
    heads: ["aa".repeat(64), "bb".repeat(64)],
  };
  await page.route("**/snapshot", (r) => r.fulfill({ json: fresh }));
  let requestId = "";
  await page.route("**/mutations/preview", (r) => {
    const request = r.request().postDataJSON();
    expect(request.expected.heads).toEqual(fresh.heads);
    expect(request.command).toEqual({
      type: "reconcile_operations",
      base: fresh.heads[0],
    });
    requestId = request.request_id;
    const value = mutationSample("mutations/preview", "prepared");
    value.preview.request_id = requestId;
    return r.fulfill({ json: value });
  });
  await page.route("**/mutations", (r) =>
    r.fulfill({ json: { status: "outcome_unknown", request_id: requestId } }),
  );
  await page.route("**/mutations/*", (r) => {
    if (r.request().url().endsWith("/preview")) return r.fallback();
    expect(r.request().url()).toContain(requestId);
    const value = mutationSample("mutations/{request_id}", "committed");
    value.receipt.request_id = requestId;
    return r.fulfill({ json: value });
  });
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  await page.getByRole("button", { name: "Sync", exact: true }).click();
  await page
    .getByRole("button", { name: "Reconcile operations", exact: true })
    .click();
  let dialog = page.getByRole("dialog", { name: "Reconcile operations" });
  await dialog.getByRole("button", { name: "Preview reconciliation" }).click();
  await dialog.getByRole("button", { name: "Apply command" }).click();
  await expect(dialog.getByText("Publication outcome unknown")).toBeVisible();
  await page.keyboard.press("Escape");
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("button", { name: "Sync", exact: true }).click();
  await page.getByRole("button", { name: "Recover reconciliation" }).click();
  dialog = page.getByRole("dialog", { name: "Reconcile operations" });
  await dialog.getByRole("button", { name: "Check status" }).click();
  await expect(dialog.getByText("Repository updated")).toBeVisible();
  expect(await page.evaluate(() => location.hash)).toBe(hash);
});
