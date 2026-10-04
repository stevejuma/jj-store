import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test.beforeEach(async ({ page }) => {
  await page.route("**/v1/bootstrap", (route) =>
    route.fulfill({
      json: {
        version: 1,
        access_mode: "authenticated",
        capabilities: {},
        storage: { ready: true },
        backups: [],
      },
    }),
  );
});
test("bundled Help preserves a draft without a repository and works without external network", async ({
  page,
  context,
}) => {
  const external: string[] = [];
  const violations: string[] = [];
  page.on("console", (message) => {
    if (
      message.type() === "error" &&
      /Content Security Policy|Refused to|violates/i.test(message.text())
    )
      violations.push(message.text());
  });
  await context.route("**/*", (route) => {
    if (new URL(route.request().url()).hostname !== "127.0.0.1") {
      external.push(route.request().url());
      return route.abort();
    }
    return route.continue();
  });
  await page.goto("/");
  const draft = page.getByLabel("Access token");
  await draft.fill("disposable-unsent-draft");
  const route = page.url();
  await page.getByRole("button", { name: "Help", exact: true }).click();
  const dialog = page.getByRole("dialog", { name: "Help", exact: true });
  const help = page.frameLocator('iframe[title="jj-store documentation"]');
  await expect(help.locator("h1")).toContainText("jj-store");
  await expect(dialog).toBeVisible();
  await expect(page.getByRole("button", { name: "Close Help" })).toBeFocused();
  const search = help.getByRole("textbox", { name: "Search" });
  await search.pressSequentially("manifest");
  await expect(help.locator(".md-search-result__list")).toContainText(
    /publication/i,
  );
  await search.press("Escape");
  await expect(dialog).toBeVisible();
  await search.click();
  await help
    .locator('.md-search-result__link[href*="technical/publication"]')
    .first()
    .click();
  await expect(help.locator("h1")).toHaveText("Publication and recovery¶");
  await expect(help.locator(".architecture-diagram img")).toBeVisible();
  expect(
    await help
      .locator(".architecture-diagram img")
      .evaluate(
        (img: HTMLImageElement) => img.complete && img.naturalWidth > 0,
      ),
  ).toBe(true);
  const frame = page
    .frames()
    .find((frame) => frame.url().includes("/help/technical/publication/"))!;
  await frame.evaluate(() => window.scrollTo(0, 500));
  await page.getByRole("button", { name: "Close Help" }).click();
  await expect(dialog).not.toBeVisible();
  await expect(draft).toHaveValue("disposable-unsent-draft");
  await expect(
    page.getByRole("button", { name: "Help", exact: true }),
  ).toBeFocused();
  expect(page.url()).toBe(route);
  await page.getByRole("button", { name: "Help", exact: true }).click();
  await expect(help.locator("h1")).toHaveText("Publication and recovery¶");
  expect(await frame.evaluate(() => window.scrollY)).toBeGreaterThan(100);
  const axe = await new AxeBuilder({ page }).include("dialog").analyze();
  expect(axe.violations).toEqual([]);
  await help.locator("h1").click();
  await page.keyboard.press("Escape");
  await expect(dialog).not.toBeVisible();
  expect(external).toEqual([]);
  expect(violations).toEqual([]);
});
test("standalone deep links, mobile navigation and external link isolation", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/help/technical/architecture/");
  await expect(page.locator("h1")).toHaveText("Architecture¶");
  await expect(page.locator(".architecture-diagram img")).toBeVisible();
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
  await page.goto("/help/contributing/documentation/");
  await expect(page.locator("h1")).toHaveText("Maintaining documentation¶");
  await page.goto("/help/");
  const links = await page.locator('a[href^="https://"]').evaluateAll((links) =>
    links.map((link) => ({
      target: link.getAttribute("target"),
      rel: link.getAttribute("rel"),
    })),
  );
  expect(links.length).toBeGreaterThan(0);
  for (const link of links) {
    expect(link.target).toBe("_blank");
    expect(link.rel).toContain("noopener");
  }
  await page.goto("/");
  await page.getByRole("button", { name: "Help", exact: true }).click();
  await expect(page.getByRole("dialog")).toHaveJSProperty("open", true);
  const box = await page.getByRole("dialog").boundingBox();
  expect(box?.width).toBe(390);
  await page.getByRole("button", { name: "Close Help" }).click();
});
test("missing bundled documentation offers retry and remains dismissible", async ({
  page,
}) => {
  await page.route("**/help/**", (route) =>
    route.fulfill({ status: 404, body: "missing" }),
  );
  await page.goto("/");
  await page.getByRole("button", { name: "Help", exact: true }).click();
  await expect(page.getByRole("alert")).toContainText(
    "Documentation could not be loaded",
    { timeout: 20000 },
  );
  await expect(page.getByRole("button", { name: "Retry" })).toBeVisible();
  await page.unroute("**/help/**");
  await page.getByRole("button", { name: "Retry" }).click();
  await expect(page.frameLocator("iframe").locator("h1")).toContainText(
    "jj-store",
  );
  await page.getByRole("button", { name: "Close Help" }).click();
  await expect(page.getByRole("dialog")).not.toBeVisible();
});

test("Help is available from the installation sidebar before a repository exists", async ({
  page,
}) => {
  await page.route("**/v1/bootstrap", (route) =>
    route.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities: {},
        storage: { ready: true },
        backups: [],
      },
    }),
  );
  await page.route("**/v1/repositories?*", (route) =>
    route.fulfill({
      json: { version: 1, repositories: [], next_cursor: null },
    }),
  );
  await page.goto("/");
  const trigger = page
    .locator(".community-app-shell")
    .getByRole("button", { name: "Help", exact: true });
  await expect(trigger).toBeVisible();
  await trigger.click();
  await expect(page.frameLocator("iframe").locator("h1")).toContainText(
    "jj-store",
  );
  await page.getByRole("button", { name: "Close Help" }).press("Shift+Tab");
  expect(
    await page.evaluate(
      () => document.activeElement?.closest("dialog") !== null,
    ),
  ).toBe(true);
  await page.getByRole("button", { name: "Close Help" }).click();
  await expect(trigger).toBeFocused();
});
