import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const native = process.env.JJ_STORE_REAL_REPOSITORY!;
const local = process.env.JJ_STORE_SETTINGS_LOCAL_REPOSITORY;

test("repository settings restores exact Stacks context and Details state", async ({
  page,
}) => {
  test.skip(
    !local,
    "Local JJ registration is required for the Stacks entry scenario.",
  );
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(`/#/repos/${local}/stacks?stack_tab=workspace`);
  const gear = page.getByRole("button", {
    name: "Repository settings",
    exact: true,
  });
  await expect(gear).toBeVisible({ timeout: 30000 });
  await expect(gear).toBeEnabled();
  await page
    .getByRole("button", { name: "Toggle details", exact: true })
    .click();
  await expect(
    page.getByRole("navigation", { name: "Stack workspace views" }),
  ).toBeVisible();
  const before = page.url();
  const reads: string[] = [];
  page.on("request", (request) => {
    if (
      request.method() === "GET" &&
      /\/v1\/(?:local\/)?repos\/[^/]+\/(?:log|operations|references|files|commits)/.test(
        request.url(),
      )
    )
      reads.push(request.url());
  });
  await gear.click();
  await expect(
    page.getByRole("heading", { name: "Identity and snapshot", level: 2 }),
  ).toBeVisible();
  await expect(page).toHaveURL(/\/settings\?section=general&at_op=/);
  await expect(
    page.getByRole("navigation", { name: "Stack workspace views" }),
  ).not.toBeVisible();
  for (const name of ["Search", "Storage", "Sync", "Protection"])
    await expect(page.getByRole("tab", { name, exact: true })).toHaveCount(0);
  expect(reads).toEqual([]);
  await page
    .getByRole("button", { name: "Back to repository", exact: true })
    .click();
  await expect(page).toHaveURL(before);
  await expect(
    page.getByRole("navigation", { name: "Stack workspace views" }),
  ).toBeVisible();
  expect(errors).toEqual([]);
});

test("repository Settings deep links reload and browser history restore subtabs", async ({
  page,
}) => {
  await page.goto(`/#/repos/${native}/settings?section=search&tab=scopes`);
  await expect(
    page.getByRole("tab", { name: "File scopes", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  const scopesURL = page.url();
  await page.reload();
  await expect(
    page.getByRole("tab", { name: "File scopes", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  const inner = page.getByRole("navigation", {
    name: "Settings page navigation",
  });
  await inner.getByRole("tab", { name: "Advanced", exact: true }).click();
  await expect(page).toHaveURL(/section=search&tab=advanced/);
  await page.goBack();
  await expect(page).toHaveURL(scopesURL);
  await expect(
    page.getByRole("tab", { name: "File scopes", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await page.goForward();
  await expect(
    inner.getByRole("tab", { name: "Advanced", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await page
    .getByRole("button", { name: "Back to repository", exact: true })
    .click();
  await expect(page).toHaveURL(new RegExp(`/repos/${native}/files`));
});

test("global Settings contains installation configuration and returns to repository Settings", async ({
  page,
}) => {
  await page.goto(`/#/repos/${native}/settings?section=general`);
  await expect(
    page.getByRole("heading", { name: "Identity and snapshot", level: 2 }),
  ).toBeVisible();
  const before = page.url();
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  const nav = page.getByRole("navigation", { name: "Settings navigation" });
  for (const name of ["Appearance", "Local service", "Agents", "Compute"])
    await expect(nav.getByRole("button", { name, exact: true })).toBeVisible();
  for (const name of ["Repository", "Search", "Storage", "Sync"])
    await expect(nav.getByRole("button", { name, exact: true })).toHaveCount(0);
  await expect(
    page.getByRole("combobox", { name: "Settings repository" }),
  ).toHaveCount(0);
  await page.getByRole("tab", { name: "Protection", exact: true }).click();
  await expect(
    page.getByRole("heading", { name: "Installation protection rules" }),
  ).toBeVisible();
  await page.getByRole("tab", { name: "Storage", exact: true }).click();
  await expect(
    page.getByRole("heading", { name: "Authority storage configuration" }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Back to jj-store", exact: true })
    .click();
  await expect(page).toHaveURL(before);
  await expect(
    page.getByRole("heading", { name: "Identity and snapshot", level: 2 }),
  ).toBeVisible();
});

test("native folder drafts survive category changes and Back with accessible mobile navigation", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto(`/#/repos/${native}/settings?section=folders`);
  const input = page.getByRole("textbox", { name: "Folder path", exact: true });
  await expect(input).toBeVisible();
  await input.fill("/retained/settings-navigation-draft");
  await page.evaluate(() => {
    location.hash = location.hash.replace("section=folders", "section=general");
  });
  await expect(
    page.getByRole("heading", { name: "Identity and snapshot", level: 2 }),
  ).toBeVisible();
  await page.goBack();
  await expect(input).toHaveValue("/retained/settings-navigation-draft");
  await page
    .getByRole("button", { name: "Back to repository", exact: true })
    .click();
  await expect(
    page.getByRole("button", { name: "Repository settings", exact: true }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Repository settings", exact: true })
    .click();
  await page.evaluate(() => {
    location.hash = location.hash.replace("section=general", "section=folders");
  });
  await expect(input).toHaveValue("/retained/settings-navigation-draft");
  await page.getByRole("button", { name: "Cancel", exact: true }).click();
  await expect(input).toHaveValue("");
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
});

test("Sync observation settles without resubscribing or flickering", async ({
  page,
}) => {
  let reads = 0;
  page.on("request", (request) => {
    if (request.method() === "GET" && /\/local\/sync$/.test(request.url()))
      reads++;
  });
  await page.goto(`/#/repos/${native}/settings?section=sync`);
  await expect(
    page.getByRole("heading", { name: "Upstream sync" }),
  ).toBeVisible();
  // Bounded quiet interval: periodic fallback is five seconds and no command is issued.
  await page.waitForTimeout(1500);
  expect(reads).toBeLessThanOrEqual(2);
  await expect(
    page.getByText(
      "No upstream is configured. Repository work remains local.",
      { exact: true },
    ),
  ).toBeVisible();
  const before = reads;
  await page
    .getByRole("button", { name: "Refresh settings", exact: true })
    .click();
  await expect.poll(() => reads).toBe(before + 1);
});
