import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample } from "./native-browser-fixture.js";

test("Storage recovers a lost response without replacing the captured target", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/capabilities", (r) =>
    r.fulfill({
      json: { ...sample("capabilities"), local_storage_management: true },
    }),
  );
  const target = {
    repository: "ab".repeat(16),
    epoch: "cd".repeat(16),
    position: 7,
    revision: 9,
    digest: "ef".repeat(32),
  };
  const limits = { entries: 100, bytes: 100000, timeout_millis: 1000 };
  let operation: any = null;
  const commands: any[] = [];
  await page.route("**/local/storage", (r) =>
    r.fulfill({
      json: {
        version: 1,
        ready: true,
        current: target,
        checkpoint: null,
        limits,
        latest: operation,
      },
    }),
  );
  await page.route("**/local/storage/operations", async (r) => {
    const command = r.request().postDataJSON();
    commands.push(command);
    if (command.action === "prepare_checkpoint") {
      operation = {
        version: 1,
        request_id: command.request_id,
        kind: "checkpoint",
        phase: "prepared",
        target: { ...target },
        limits,
        message: null,
        selection: null,
      };
      await r.abort();
      return;
    }
    expect(command.request_id).toBe(operation.request_id);
    operation = { ...operation, phase: "complete", selection: "selected" };
    await r.fulfill({ json: operation });
  });
  await page.route("**/local/storage/operations/*", (r) =>
    r.fulfill({ json: operation }),
  );
  await page.goto("/");
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  const hash = new URL(page.url()).hash;
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Storage", exact: true }).click();
  await expect(page.getByText("Selected manifest is readable")).toBeVisible();
  await page.getByRole("tab", { name: "Maintenance", exact: true }).click();
  await page
    .getByRole("button", { name: "Prepare checkpoint", exact: true })
    .click();
  await expect(
    page.getByRole("button", { name: "Retry original request" }),
  ).toBeVisible();
  await expect(
    page.getByRole("combobox", { name: "Settings repository" }),
  ).toBeDisabled();
  target.position = 12;
  await page
    .getByRole("navigation", { name: "Settings page navigation" })
    .getByRole("button", { name: "Check status", exact: true })
    .click();
  await expect(
    page.getByRole("button", { name: "Publish checkpoint" }),
  ).toBeEnabled();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("button", { name: "Storage", exact: true }).click();
  await page.getByRole("tab", { name: "Maintenance", exact: true }).click();
  await page.getByRole("button", { name: "Publish checkpoint" }).click();
  await expect(
    page.getByText("The exact captured checkpoint is selected."),
  ).toBeVisible();
  expect(operation.target.position).toBe(7);
  expect(commands).toHaveLength(2);
  expect(new URL(page.url()).hash).toBe(hash);
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
  await page.evaluate(() => {
    location.hash = "#/projects";
  });
  await expect(
    page.getByText("Storage unavailable", { exact: true }),
  ).toBeVisible();
  expect(new URL(page.url()).hash).toBe("#/projects");
  expect(commands).toHaveLength(2);
  const picker = page.getByRole("combobox", { name: "Settings repository" });
  await expect(picker).toHaveText(/Choose repository/);
  await picker.click();
  await page.getByRole("option", { name: /Local acceptance/ }).click();
  await expect(picker).toHaveText(/Local acceptance/);
  await expect(
    page.getByRole("heading", { name: "Storage", level: 1 }),
  ).toBeVisible();
  await expect(
    page.getByRole("tab", { name: "Maintenance", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await page.getByRole("tab", { name: "Overview", exact: true }).click();
  await expect(page.getByText("Selected manifest is readable")).toBeVisible();
});
