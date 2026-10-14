import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import {
  api,
  operation,
  repo,
  sample,
  samples,
} from "./native-browser-fixture.js";

test("local trusted bootstrap auto-opens the sole repository without credentials", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  let searchCaughtUp = false;
  let preparedOperation: string | null = null;
  let taskPolls = 0;
  let statusRequests = 0;
  await page.route("**/v1/bootstrap", async (route) => {
    const headers = route.request().headers();
    expect(headers["x-jj-store-protocol"]).toBe("1");
    expect(headers["authorization"]).toBeUndefined();
    await route.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities: sample("capabilities"),
        storage: { ready: true },
        repositories: [{ id: repo, name: "Local acceptance" }],
        backups: [],
      },
    });
  });
  await page.route("**/search/status", async (route) => {
    statusRequests += 1;
    const position = sample("snapshot").published_seq;
    await route.fulfill({
      json: {
        version: 1,
        repository: repo,
        epoch: sample("snapshot").epoch,
        repository_position: position,
        durable_position: position,
        indexed_position: searchCaughtUp ? position : position - 1,
        index_generation: "aa".repeat(32),
        is_stale: !searchCaughtUp,
        degraded: false,
      },
    });
  });
  await page.route("**/index/tasks", async (route) => {
    preparedOperation = route.request().postDataJSON().operation;
    const running = samples.find(
      (entry: any) =>
        entry.path === "/v1/repos/{repo}/index/tasks" &&
        entry.method === "post",
    );
    await route.fulfill({ status: 202, json: running.data });
  });
  await page.route("**/index/tasks/*", async (route) => {
    taskPolls += 1;
    await route.fulfill({ json: sample("index/tasks/{task}") });
  });

  await page.goto("/");
  await expect(
    page.getByRole("button", { name: "Open profile menu for Local" }),
  ).toBeVisible();
  await expect(page.getByRole("option", { name: "main" })).toBeVisible();
  const before = await page.evaluate(() => ({
    hash: window.location.hash,
    historyLength: window.history.length,
  }));
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  await expect(
    page.getByRole("heading", { name: "Local service", level: 1 }),
  ).toBeVisible();
  await expect(page.getByText("Local trusted")).toBeVisible();
  await expect(page.getByText("Ready", { exact: true })).toBeVisible();
  await expect(page.getByText("connected", { exact: true })).toBeVisible();
  await expect(
    page.getByText(/^(live|reconnecting)$/, { exact: true }),
  ).toBeVisible();

  await page
    .getByRole("button", { name: "Back to jj-store", exact: true })
    .click();
  expect(new URL(page.url()).hash).toBe(before.hash);
  await page
    .getByRole("button", { name: "Repository settings", exact: true })
    .click();
  await expect(
    page.getByRole("tab", { name: "General", exact: true }),
  ).toHaveAttribute("aria-selected", "true");

  await expect(page.getByText("Local acceptance")).toBeVisible();
  await expect(page.getByText(repo, { exact: true })).toBeVisible();
  await expect(page.getByText("admin", { exact: true })).toBeVisible();
  await expect(
    page.getByText(operation, { exact: true }).first(),
  ).toBeVisible();
  await page.getByRole("tab", { name: "Advanced", exact: true }).click();
  await expect(
    page.getByText(sample("capabilities").native_profile, { exact: true }),
  ).toBeVisible();

  await Promise.all([
    page.waitForResponse("**/search/status"),
    page.getByRole("tab", { name: "Search", exact: true }).click(),
  ]);
  await page.getByRole("tab", { name: "Health", exact: true }).click();
  await expect(page.getByText("Stale", { exact: true })).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Prepare graph index" }),
  ).toHaveCount(0);
  const beforePreparation = statusRequests;
  await page.getByRole("tab", { name: "Advanced", exact: true }).click();
  await page.getByRole("button", { name: "Prepare graph index" }).click();
  await expect(
    page.getByText("Graph index preparation completed."),
  ).toBeVisible();
  expect(statusRequests).toBe(beforePreparation);
  await Promise.all([
    page.waitForResponse("**/search/status"),
    page.getByRole("tab", { name: "Search", exact: true }).click(),
  ]);
  await expect(page.getByText("Stale", { exact: true })).toBeVisible();
  expect(statusRequests).toBe(beforePreparation + 1);
  searchCaughtUp = true;
  await page.getByRole("button", { name: "Refresh status" }).click();
  await expect(page.getByText("Fresh", { exact: true })).toBeVisible();
  expect(preparedOperation).toBe(operation);
  expect(taskPolls).toBe(1);
  expect(statusRequests).toBe(beforePreparation + 2);

  const whileOpen = await page.evaluate(() => ({
    hash: window.location.hash,
    historyLength: window.history.length,
    settingsStorage: Object.keys(localStorage).filter((key) =>
      key.toLowerCase().includes("settings"),
    ),
  }));
  expect(whileOpen.hash).toContain(`/repos/${repo}/settings`);
  expect(whileOpen.hash).toContain(`at_op=${operation}`);
  expect(whileOpen.historyLength).toBeGreaterThan(before.historyLength);
  expect(whileOpen.settingsStorage).toEqual([]);
  await page.getByRole("button", { name: "Back to repository" }).click();
  await expect(page.getByRole("option", { name: "main" })).toBeVisible();
  expect(await page.evaluate(() => window.location.hash)).toBe(before.hash);
  await page.mouse.move(1300, 900);
  await expect(
    page.locator("[data-bits-floating-content-wrapper]:visible"),
  ).toHaveCount(0);
  expect(await new AxeBuilder({ page }).analyze()).toMatchObject({
    violations: [],
  });
});
