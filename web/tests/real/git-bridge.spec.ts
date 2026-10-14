import { expect, test } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { join } from "node:path";

// The owning Rust fixture starts a fresh actual RustFS/native server and creates
// an external Git merge before this browser starts. No synthetic controller.
test("Git import refreshes automatically", async ({ page }) => {
  const workspace = process.env.JJ_STORE_GIT_CYCLE_WORKSPACE!;
  const root = process.env.JJ_STORE_REAL_ROOT!;
  await page.goto("/");
  const main = page.getByRole("option", { name: "main", exact: true });
  await expect(main).toBeVisible();
  await main.click();
  await page.getByRole("tab", { name: "Commits", exact: true }).click();
  await expect(
    page.getByRole("button", { name: "Native Git feature", exact: true }),
  ).toBeVisible();
  const frozen = new URL(page.url()).hash;
  await expect(
    page.getByRole("button", { name: "External Git merge", exact: true }),
  ).toHaveCount(0);
  execFileSync(
    join(root, "target/debug/jj-store-jj"),
    ["--no-pager", "-R", workspace, "git", "fetch"],
    { env: process.env, stdio: "pipe" },
  );
  await expect(
    page.getByRole("button", { name: "External Git merge", exact: true }),
  ).toBeVisible();
  expect(new URL(page.url()).hash).not.toBe(frozen);
  expect(
    await page.evaluate(() => JSON.stringify({ ...localStorage })),
  ).not.toContain("jst1_");
});
