import { repositorySettings } from "./repository-settings-navigation.js";
import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample } from "./native-browser-fixture.js";
import { syncWorker } from "../../../catalog/fixtures/sync-settings.js";

test("operational toasts deduplicate, retain dismissal and clear on recovery", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/capabilities", (route) =>
    route.fulfill({
      json: { ...sample("capabilities"), local_sync_management: true },
    }),
  );
  let unavailable = false;
  const failure = { status: 503, json: { error: "sync_unavailable" } };
  await page.route("**/local/sync", (route) =>
    route.fulfill(
      unavailable
        ? failure
        : { json: { version: 1, configured: true, worker: syncWorker() } },
    ),
  );
  await page.route("**/local/sync/control", (route) => {
    unavailable = true;
    return route.fulfill(failure);
  });
  await page.goto("/");
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  await expect(
    page.getByRole("heading", { name: "Local service", level: 1 }),
  ).toBeVisible();
  await repositorySettings(page, "Sync");
  await page.getByRole("button", { name: "Pause sync", exact: true }).click();
  const toast = page.locator(
    '[data-notification-toasts] [data-severity="error"]',
  );
  await expect(toast).toHaveCount(1);
  await expect(toast).toContainText("Sync status unavailable");
  const id = await toast.getAttribute("data-notification-id");
  await page.getByRole("button", { name: "Refresh settings" }).click();
  await expect(toast).toHaveCount(1);
  await expect(toast).toHaveAttribute("data-notification-id", id!);
  await page
    .getByRole("button", { name: "Dismiss Sync status unavailable" })
    .click();
  await page.getByRole("button", { name: "Refresh settings" }).click();
  await expect(
    page.getByRole("button", { name: "Refresh settings" }),
  ).toBeEnabled();
  await expect(toast).toHaveCount(0);
  await page.getByRole("button", { name: "Back to repository" }).click();
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  await expect(
    page.getByRole("heading", { name: "Local service", level: 1 }),
  ).toBeVisible();
  await repositorySettings(page, "Sync");
  await expect(toast).toHaveCount(0);
  unavailable = false;
  await page.getByRole("button", { name: "Refresh settings" }).click();
  await expect(
    page.getByRole("button", { name: "Pause sync", exact: true }),
  ).toBeEnabled();
  await page.getByRole("button", { name: "Pause sync", exact: true }).click();
  await expect(toast).toHaveCount(1);
  await page.setViewportSize({ width: 390, height: 844 });
  const bounds = await toast.boundingBox();
  expect(bounds!.x).toBeGreaterThanOrEqual(0);
  expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(390);
  expect(await new AxeBuilder({ page }).analyze()).toMatchObject({
    violations: [],
  });
  unavailable = false;
  await page.getByRole("button", { name: "Refresh settings" }).click();
  await expect(toast).toHaveCount(0);
  await page.reload();
  await expect(page.locator("[data-notification-id]")).toHaveCount(0);
});
