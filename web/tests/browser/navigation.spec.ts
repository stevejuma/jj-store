import { expect, test } from "@playwright/test";
import {
  api,
  connect,
  operation,
  repo,
  token,
} from "./native-browser-fixture.js";

test("hash route restores snapshot and file through controls reload and browser history", async ({
  page,
}) => {
  await api(page, { changed: true });
  await connect(page);
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  expect(new URL(page.url()).hash).toBe(
    `#/repos/${repo}/branches?at_op=${operation}`,
  );

  await page.getByRole("tab", { name: "Files" }).click();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Open file bin" }).click();
  await expect(
    page.getByRole("region", { name: "Source preview for bin" }),
  ).toContainText("window.sourceExecuted=true");
  const deepHash = new URL(page.url()).hash;
  expect(deepHash).toContain(`#/repos/${repo}/files?`);
  expect(deepHash).toContain(`at_op=${operation}`);
  expect(deepHash).toContain("bookmark=main");
  expect(deepHash).toContain("file=bin");
  expect(deepHash).not.toContain(token);

  await expect(
    page.getByRole("button", {
      name: "Refresh snapshot (new history available)",
    }),
  ).toBeVisible();
  await page.getByRole("button", { name: /Refresh snapshot/ }).click();
  await expect(
    page.getByRole("region", { name: "Source preview for bin" }),
  ).toBeVisible();
  expect(new URL(page.url()).hash).toBe(deepHash);

  await page.getByRole("button", { name: "Back" }).click();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Forward" }).click();
  await expect(
    page.getByRole("region", { name: "Source preview for bin" }),
  ).toBeVisible();

  await page.reload();
  await expect(page.getByLabel("Access token", { exact: true })).toHaveValue(
    "",
  );
  await page.getByLabel("Access token", { exact: true }).fill(token);
  await page.getByRole("button", { name: "Continue" }).click();
  await expect(
    page.getByRole("region", { name: "Source preview for bin" }),
  ).toContainText("window.sourceExecuted=true");
  expect(new URL(page.url()).hash).toBe(deepHash);
});

test("malformed owned route is explicit before authentication", async ({
  page,
}) => {
  await api(page);
  await page.goto(`/#/repos/${repo}/files?file=..%2Fsecret`);
  await expect(page.getByRole("alert")).toContainText(
    "Invalid repository route value.",
  );
  await expect(page.getByLabel("Access token", { exact: true })).toHaveValue(
    "",
  );
});
