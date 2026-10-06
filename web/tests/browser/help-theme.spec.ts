import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test("embedded Help follows live application appearance across pages and reopening", async ({
  page,
  context,
}) => {
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
  await page.addInitScript(
    () =>
      window === window.top &&
      localStorage.setItem(
        "jj-store.appearance.v1",
        JSON.stringify({
          version: 1,
          theme: "nord",
          mode: "dark",
          fontSize: "larger",
        }),
      ),
  );
  const external: string[] = [];
  await context.route("**/*", (route) => {
    if (new URL(route.request().url()).hostname !== "127.0.0.1") {
      external.push(route.request().url());
      return route.abort();
    }
    return route.continue();
  });
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  page.on("console", (message) => {
    if (/Content Security Policy|Refused to|violates/i.test(message.text()))
      errors.push(message.text());
  });
  await page.goto("/");
  await page.getByRole("button", { name: "Help", exact: true }).click();
  const help = page.frameLocator('iframe[title="jj-store documentation"]');
  await expect(help.locator("html")).toHaveAttribute(
    "data-store-theme",
    "nord",
  );
  await expect(help.locator("html")).toHaveAttribute(
    "data-store-font",
    "larger",
  );
  await expect(help.locator('[data-md-component="palette"]')).toBeHidden();
  const parentBackground = await page
    .locator("body")
    .evaluate((root) => getComputedStyle(root).backgroundColor);
  expect(
    await help
      .locator(".md-content")
      .evaluate((root) => getComputedStyle(root).backgroundColor),
  ).toBe(parentBackground);
  await expect(help.locator("article.md-typeset")).toHaveCSS(
    "font-size",
    "15px",
  );
  await expect(
    help.getByRole("link", { name: "Architecture", exact: true }),
  ).toBeHidden();
  await help
    .locator(".md-nav--primary label.md-nav__link")
    .filter({ hasText: "Technical details" })
    .click();
  await help.getByRole("link", { name: "Architecture", exact: true }).click();
  await expect(help.locator("h1")).toHaveText("Architecture¶");
  await expect(help.locator(".architecture-diagram img")).toHaveAttribute(
    "src",
    /\.dark\.svg$/,
  );
  const peer = await context.newPage();
  await peer.goto("/help/");
  await peer.evaluate(() =>
    localStorage.setItem(
      "jj-store.appearance.v1",
      JSON.stringify({
        version: 1,
        theme: "buzz",
        mode: "light",
        fontSize: "smaller",
        glass: true,
        prominent: true,
        opacity: 80,
      }),
    ),
  );
  await expect(help.locator("html")).toHaveAttribute(
    "data-store-theme",
    "buzz",
  );
  await expect(help.locator("article.md-typeset")).toHaveCSS(
    "font-size",
    "13px",
  );
  await expect(help.locator(".architecture-diagram img")).not.toHaveAttribute(
    "src",
    /\.dark\.svg$/,
  );
  await expect(help.locator("h1")).toHaveText("Architecture¶");
  await expect(help.locator("html")).toHaveAttribute(
    "data-store-prominent",
    "true",
  );
  const articleColor = await help
    .locator("article.md-typeset")
    .evaluate((element) => getComputedStyle(element).color);
  await expect(
    help.getByRole("link", { name: "Architecture", exact: true }),
  ).toHaveCSS("background-color", articleColor);
  expect(
    await help
      .locator(".md-header")
      .evaluate((element) => getComputedStyle(element).backgroundColor),
  ).not.toBe("rgba(0, 0, 0, 0)");
  await page.getByRole("button", { name: "Close Help" }).click();
  await peer.evaluate(() =>
    localStorage.setItem(
      "jj-store.appearance.v1",
      JSON.stringify({
        version: 1,
        theme: "original-dark",
        mode: "dark",
        fontSize: "default",
      }),
    ),
  );
  await page.getByRole("button", { name: "Help", exact: true }).click();
  await expect(help.locator("html")).toHaveAttribute(
    "data-store-theme",
    "original-dark",
  );
  await expect(help.locator("h1")).toHaveText("Architecture¶");
  await expect(help.locator(".architecture-diagram img")).toHaveAttribute(
    "src",
    /\.dark\.svg$/,
  );
  const frame = page
    .frames()
    .find((frame) => frame.url().includes("/help/technical/architecture/"))!;
  expect(
    await frame
      .locator(".architecture-diagram img")
      .evaluate(
        (image: HTMLImageElement) => image.complete && image.naturalWidth > 0,
      ),
  ).toBe(true);
  await page.getByRole("button", { name: "Close Help" }).press("Escape");
  await expect(
    page.getByRole("button", { name: "Help", exact: true }),
  ).toBeFocused();
  await peer.close();
  expect(external).toEqual([]);
  expect(errors).toEqual([]);
});

test("standalone documentation retains themed search and responsive navigation", async ({
  page,
}) => {
  await page.emulateMedia({ colorScheme: "dark" });
  await page.goto("/help/");
  await expect(page.locator("html")).toHaveAttribute(
    "data-store-theme",
    "original-dark",
  );
  await expect(page.locator('[data-md-component="palette"]')).toBeVisible();
  const install = page.getByRole("link", {
    name: "Installation and setup",
    exact: true,
  });
  await expect(install).toBeHidden();
  const gettingStarted = page
    .locator(".md-nav--primary label.md-nav__link")
    .filter({ hasText: "Getting started" });
  await gettingStarted.click();
  await expect(install).toBeVisible();
  await gettingStarted.click();
  await expect(install).toBeHidden();
  const axe = await new AxeBuilder({ page }).analyze();
  expect(axe.violations).toEqual([]);
  await page.locator('.md-header label[title="Switch to light mode"]').click();
  await expect(page.locator("html")).toHaveAttribute(
    "data-store-theme",
    "original",
  );
  await page.locator('.md-header label[title="Switch to dark mode"]').click();
  await expect(page.locator("html")).toHaveAttribute(
    "data-store-theme",
    "original-dark",
  );
  await page
    .getByRole("textbox", { name: "Search", exact: true })
    .pressSequentially("manifest");
  await expect(page.locator(".md-search-result__list")).toContainText(
    /publication/i,
  );
  await page
    .getByRole("textbox", { name: "Search", exact: true })
    .press("Escape");
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/help/technical/architecture/");
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
  await expect(page.locator(".architecture-diagram img")).toHaveAttribute(
    "src",
    /\.dark\.svg$/,
  );
  await page.locator('.md-header label[for="__drawer"]').click();
  await expect(page.locator("#__drawer")).toBeChecked();
  await expect
    .poll(
      async () => (await page.locator(".md-sidebar--primary").boundingBox())?.x,
    )
    .toBe(0);
});
