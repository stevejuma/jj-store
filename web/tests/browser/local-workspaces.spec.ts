import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample } from "./native-browser-fixture.js";

test("local folder settings preserve drafts and link without changing repository context", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/capabilities", (route) =>
    route.fulfill({
      json: {
        ...sample("capabilities"),
        local_workspace_links: true,
        local_folder_picker: true,
      },
    }),
  );
  let picks = 0;
  await page.route("**/local/workspaces/pick", async (route) => {
    expect(route.request().method()).toBe("POST");
    expect(route.request().postDataJSON()).toEqual({});
    picks++;
    await route.fulfill({
      json: {
        version: 1,
        path: picks === 1 ? "/workspaces/example with spaces" : null,
      },
    });
  });
  const requests: unknown[] = [];
  let revision = 7;
  const links: Array<{
    id: string;
    path: string;
    profile: string;
    availability: string;
  }> = [];
  await page.route("**/local/workspaces", (route) =>
    route.fulfill({ json: { version: 1, revision, links } }),
  );
  await page.route("**/local/workspaces/link", async (route) => {
    expect(route.request().headers()["authorization"]).toBeUndefined();
    expect(route.request().headers()["x-jj-store-protocol"]).toBe("1");
    const body = route.request().postDataJSON();
    requests.push(body);
    if (requests.length === 1) {
      revision = 8;
      await route.fulfill({
        status: 409,
        json: { code: "workspace_links_changed" },
      });
      return;
    }
    links.push({
      id: "aa".repeat(32),
      path: body.path,
      profile: "native",
      availability: "ready",
    });
    revision++;
    await route.fulfill({ json: { version: 1, revision, links } });
  });
  await page.goto("/");
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  const originalAppBounds = (await page
    .getByRole("option", { name: "main", exact: true })
    .boundingBox())!;
  const originalHash = await page.evaluate(() => location.hash);
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  await expect(
    page
      .getByRole("navigation", { name: "Settings page navigation" })
      .getByRole("button", { name: "Back to app" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await expect(
    page.getByRole("button", { name: /^Copy full Repository ID:/ }),
  ).toBeVisible();
  expect(
    (await page
      .getByRole("button", { name: /^Copy full Operation head 1:/ })
      .boundingBox())!.width,
  ).toBeLessThan(200);
  await expect(
    page.getByRole("textbox", { name: "Repository ID" }),
  ).toHaveCount(0);
  const tabs = page.getByRole("tablist", {
    name: "Repository settings sections",
  });
  const tabBounds = (await tabs.boundingBox())!;
  const contentBounds = (await page
    .getByRole("region", { name: "Settings content" })
    .boundingBox())!;
  const navigation = page.getByRole("navigation", {
    name: "Settings page navigation",
  });
  const headingBounds = (await navigation
    .getByRole("heading", { name: "Repository", level: 1 })
    .boundingBox())!;
  expect(tabBounds.y - contentBounds.y).toBeLessThan(16);
  expect(headingBounds.x + headingBounds.width).toBeLessThan(tabBounds.x);
  await expect(navigation.locator("h1 svg[aria-hidden='true']")).toBeVisible();
  await expect(page.locator(".store-settings h1")).toHaveCount(0);
  expect(
    (await page.locator(".store-settings").boundingBox())!.width,
  ).toBeCloseTo(contentBounds.width, 0);
  await page.getByRole("tab", { name: "Local folders", exact: true }).click();
  const input = page.getByRole("textbox", { name: "Folder path" });
  await expect(page.getByText("No local folders linked.")).toBeVisible();
  await page.getByRole("button", { name: "Browse…" }).click();
  await expect(input).toHaveValue("/workspaces/example with spaces");
  expect(requests).toHaveLength(0);
  await page.getByRole("button", { name: "Browse…" }).click();
  await expect(input).toHaveValue("/workspaces/example with spaces");
  await page.getByRole("button", { name: "Search", exact: true }).click();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("tab", { name: "Local folders", exact: true }).click();
  await expect(input).toHaveValue("/workspaces/example with spaces");
  await input.press("Enter");
  await expect(page.getByText(/workspace links changed/)).toBeVisible();
  await expect(input).toHaveValue("/workspaces/example with spaces");
  await page
    .getByRole("button", { name: "Refresh folders", exact: true })
    .click();
  await expect(page.getByText(/workspace links changed/)).toHaveCount(0);
  await page.getByRole("button", { name: "Link folder", exact: true }).click();
  await expect(
    page.getByText("/workspaces/example with spaces", { exact: true }),
  ).toBeVisible();
  await expect(input).toHaveValue("");
  expect(requests).toEqual([
    {
      version: 1,
      expected_revision: 7,
      path: "/workspaces/example with spaces",
    },
    {
      version: 1,
      expected_revision: 8,
      path: "/workspaces/example with spaces",
    },
  ]);
  await page.setViewportSize({ width: 390, height: 844 });
  await expect(input).toBeVisible();
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
  expect(await new AxeBuilder({ page }).analyze()).toMatchObject({
    violations: [],
  });
  await page.setViewportSize({ width: 1400, height: 1000 });
  await page.getByRole("button", { name: "Back to app" }).click();
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  const restoredAppBounds = (await page
    .getByRole("option", { name: "main", exact: true })
    .boundingBox())!;
  expect(restoredAppBounds.x).toBeCloseTo(originalAppBounds.x, 0);
  expect(restoredAppBounds.width).toBeCloseTo(originalAppBounds.width, 0);
  expect(await page.evaluate(() => location.hash)).toBe(originalHash);
});
