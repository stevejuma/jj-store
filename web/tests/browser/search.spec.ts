import { expect, test } from "@playwright/test";
import {
  api,
  commit,
  connect,
  operation,
  repo,
} from "./native-browser-fixture.js";

test("command search opens native results and routes through stable identities", async ({
  page,
}) => {
  const requests = await api(page);
  await connect(page);
  await page.getByRole("button", { name: "Search repository" }).click();
  const dialog = page.getByRole("dialog", { name: "Search repository" });
  await dialog
    .getByRole("combobox", { name: "Search repository" })
    .fill("binary");
  await dialog.getByRole("option", { name: /binary/u }).click();
  await expect(dialog).not.toBeVisible();
  await expect
    .poll(() =>
      page.evaluate(() => ({
        overflow: document.body.style.overflow,
        pointerEvents: document.body.style.pointerEvents,
      })),
    )
    .toEqual({ overflow: "", pointerEvents: "" });
  await expect(
    page.getByRole("list", { name: "Repository breadcrumb" }),
  ).toHaveCount(0);
  await expect(page.getByRole("tab", { name: "Commits" })).toHaveAttribute(
    "data-state",
    "active",
  );
  const commitHash = new URL(page.url()).hash;
  expect(commitHash).toContain(`at_op=${operation}`);
  expect(commitHash).toContain(`commit=${commit}`);
  expect(commitHash).not.toContain("binary");
  expect(commitHash).not.toContain("cursor");
  expect(requests).toContain("search");

  await expect(page.locator(".store-search-dialog")).toHaveCount(0);
  await page.keyboard.press("Control+K");
  await page
    .getByRole("dialog", { name: "Search repository" })
    .getByRole("combobox", { name: "Search repository" })
    .fill("file");
  await page
    .getByRole("dialog", { name: "Search repository" })
    .getByRole("option", { name: /^bin/u })
    .click();
  await expect(
    page.getByRole("region", { name: "Source preview for bin" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Search repository" }).click();
  await expect(
    page.getByRole("dialog", { name: "Search repository" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Close search" }).click();
  await expect(
    page.getByRole("dialog", { name: "Search repository" }),
  ).not.toBeVisible();
  await page.goBack();
  await expect(
    page.getByRole("list", { name: "Repository breadcrumb" }),
  ).toHaveCount(0);
  await page.goForward();
  await expect(
    page.getByRole("region", { name: "Source preview for bin" }),
  ).toBeVisible();
  const fileHash = new URL(page.url()).hash;
  expect(fileHash).toContain("bookmark=main");
  expect(fileHash).toContain(`commit=${commit}`);
  expect(fileHash).toContain("file=bin");
  expect(fileHash).not.toContain("file%20");
});
