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
  let indexPrepared = false;
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
        indexed_position: indexPrepared ? position : position - 1,
        index_generation: "aa".repeat(32),
        is_stale: !indexPrepared,
        degraded: false,
      },
    });
  });
  await page.route("**/index/tasks", async (route) => {
    preparedOperation = route.request().postDataJSON().operation;
    indexPrepared = true;
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

  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await expect(page.getByText("Local acceptance")).toBeVisible();
  await expect(page.getByText(repo, { exact: true })).toBeVisible();
  await expect(page.getByText("admin", { exact: true })).toBeVisible();
  await expect(
    page.getByText(operation, { exact: true }).first(),
  ).toBeVisible();
  await expect(
    page.getByText(sample("capabilities").native_profile, { exact: true }),
  ).toBeVisible();

  await page.getByRole("button", { name: "Search", exact: true }).click();
  await expect(page.getByText("Stale", { exact: true })).toBeVisible();
  await page.getByRole("button", { name: "Prepare index" }).click();
  await expect(page.getByText("Fresh", { exact: true })).toBeVisible();
  expect(preparedOperation).toBe(operation);
  expect(taskPolls).toBe(1);
  expect(statusRequests).toBeGreaterThanOrEqual(2);

  const whileOpen = await page.evaluate(() => ({
    hash: window.location.hash,
    historyLength: window.history.length,
    settingsStorage: Object.keys(localStorage).filter((key) =>
      key.toLowerCase().includes("settings"),
    ),
  }));
  expect(whileOpen).toEqual({
    ...before,
    settingsStorage: [],
  });
  await page.getByRole("button", { name: "Back to jj-store" }).click();
  await expect(page.getByRole("option", { name: "main" })).toBeVisible();
  expect(
    await page.evaluate(() => ({
      hash: window.location.hash,
      historyLength: window.history.length,
    })),
  ).toEqual(before);
  await page.mouse.move(1300, 900);
  await expect(
    page.locator("[data-bits-floating-content-wrapper]:visible"),
  ).toHaveCount(0);
  expect(await new AxeBuilder({ page }).analyze()).toMatchObject({
    violations: [],
  });
});
