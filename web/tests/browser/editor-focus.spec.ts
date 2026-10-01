import { readFileSync } from "node:fs";
import { expect, test } from "@playwright/test";
import { api, commit, operation, repo } from "./native-browser-fixture.js";

// Exercise the served application's policy, which rejects injected editor styles.
const assetSource = readFileSync(
  new URL("../../../crates/jj-store-http/src/assets.rs", import.meta.url),
  "utf8",
);
const csp = /const STRICT_CSP:\s*&str\s*=\s*"([^"]+)"/.exec(assetSource)?.[1];
if (!csp) throw new Error("Production asset policy STRICT_CSP was not found");

test.beforeEach(async ({ page }) => {
  await api(page, { access: "local_trusted", editFile: true });
  await page.route("**/", async (route) => {
    const response = await route.fetch();
    await route.fulfill({
      response,
      headers: { ...response.headers(), "content-security-policy": csp },
    });
  });
});

test("file editor content has no focus frame under the production asset policy", async ({
  page,
}) => {
  const response = await page.goto(
    `/#/repos/${repo}/files?at_op=${operation}&bookmark=main&commit=${commit}&file=bin`,
  );
  expect(response?.headers()["content-security-policy"]).toBe(csp);
  await page.getByRole("button", { name: "Edit", exact: true }).click();
  const editor = page.getByRole("textbox", {
    name: "Editing bin file contents",
  });
  await editor.click();
  await expect(editor).toBeFocused();
  await expect(editor).toHaveCSS("outline-style", "none");
  await expect(editor).toHaveCSS("box-shadow", "none");
  await editor.fill("Edited content remains keyboard accessible.");
  await expect(
    page.getByRole("button", { name: "Save changes", exact: true }),
  ).toBeEnabled();
});

test("prose composer content has no focus frame under the production asset policy", async ({
  page,
}) => {
  await page.goto(`/#/repos/${repo}/log?at_op=${operation}&commit=${commit}`);
  await page.getByRole("button", { name: "Repository actions" }).click();
  await page
    .getByRole("menuitem", { name: "Create change", exact: true })
    .click();
  const dialog = page.getByRole("dialog", { name: "Create change" });
  const editor = dialog.getByRole("textbox", { name: "Markdown editor" });
  await editor.click();
  await expect(editor).toBeFocused();
  await expect(editor).toHaveCSS("outline-style", "none");
  await expect(editor).toHaveCSS("box-shadow", "none");
  await editor.fill("Composer content remains editable.");
  await expect(editor).toHaveText("Composer content remains editable.");
  await dialog.getByRole("button", { name: "Cancel", exact: true }).click();
  await expect(dialog).not.toBeVisible();
});
