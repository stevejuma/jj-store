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
  await expect(help.locator(".md-content h1")).toContainText("jj-store");
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
  await expect(help.locator(".md-content h1")).toHaveText(
    "Publication and recovery¶",
  );
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
  await expect(help.locator(".md-content h1")).toHaveText(
    "Publication and recovery¶",
  );
  expect(await frame.evaluate(() => window.scrollY)).toBeGreaterThan(100);
  const axe = await new AxeBuilder({ page }).include("dialog").analyze();
  expect(axe.violations).toEqual([]);
  await help.locator(".md-content h1").click();
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
  await expect(page.locator(".md-content h1")).toHaveText("Architecture¶");
  await expect(page.locator(".architecture-diagram img")).toBeVisible();
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
  await page.goto("/help/contributing/documentation/");
  await expect(page.locator(".md-content h1")).toHaveText(
    "Maintaining documentation¶",
  );
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
test("failed Help shows the response error in an app toast and recovers through Retry", async ({
  page,
}) => {
  await page.route("**/help/**", (route) =>
    route.fulfill({ status: 503, body: "Documentation service unavailable" }),
  );
  await page.goto("/");
  await page.getByRole("button", { name: "Help", exact: true }).click();
  await expect(page.getByRole("alert")).toContainText(
    "Documentation service unavailable",
  );
  const toast = page
    .getByRole("region", { name: "Notifications" })
    .getByRole("alert");
  await expect(toast).toBeVisible();
  await expect(toast.getByRole("button", { name: "Retry" })).toBeVisible();
  await expect(page.locator('dialog > [role="alert"]')).toHaveCount(0);
  await expect(page.getByRole("button", { name: "Retry" })).toBeVisible();
  await page.unroute("**/help/**");
  await page.getByRole("button", { name: "Retry" }).click();
  await expect(
    page.frameLocator("iframe").locator(".md-content h1"),
  ).toContainText("jj-store");
  await expect(toast).toHaveCount(0);
  await page.getByRole("button", { name: "Close Help" }).click();
  await expect(page.getByRole("dialog")).not.toBeVisible();
});

test("Help shows a skeleton until its initial document is ready", async ({
  page,
}) => {
  let release!: () => void;
  const gate = new Promise<void>((resolve) => (release = resolve));
  await page.route("**/help/", async (route) => {
    await gate;
    await route.continue();
  });
  await page.goto("/");
  await page.getByRole("button", { name: "Help", exact: true }).click();
  const skeleton = page.getByRole("status", {
    name: "Loading documentation",
    exact: true,
  });
  await expect(skeleton).toHaveAttribute("aria-busy", "true");
  await expect(
    skeleton.locator('[data-ui-component="skeleton"]').first(),
  ).toBeVisible();
  const layout = await skeleton.evaluate((element) => {
    const box = (selector: string) => {
      const row = element.querySelector(selector)!;
      const rect = row.getBoundingClientRect();
      return {
        x: rect.x,
        width: rect.width,
        background: getComputedStyle(row).backgroundColor,
      };
    };
    return {
      navigation: box(".store-help-skeleton__navigation"),
      content: box(".store-help-skeleton__content"),
      contents: box(".store-help-skeleton__contents"),
    };
  });
  expect(layout.navigation.width).toBe(288);
  expect(layout.contents.width).toBe(224);
  await page.setViewportSize({ width: 1070, height: 988 });
  await expect(
    skeleton.locator(".store-help-skeleton__navigation"),
  ).toBeHidden();
  await expect(
    skeleton.locator(".store-help-skeleton__contents"),
  ).toBeVisible();
  await page.setViewportSize({ width: 390, height: 844 });
  await expect(
    skeleton.locator(".store-help-skeleton__navigation"),
  ).toBeHidden();
  await expect(skeleton.locator(".store-help-skeleton__contents")).toBeHidden();
  await expect(skeleton.locator(".store-help-skeleton__content")).toBeVisible();
  await page.setViewportSize({ width: 1400, height: 1000 });
  release();
  await expect(
    page.frameLocator("iframe").locator(".md-content h1"),
  ).toContainText("jj-store");
  await expect(skeleton).toHaveCount(0);
  const content = await page
    .frameLocator("iframe")
    .locator(".md-content")
    .evaluate((element) => {
      const rect = element.getBoundingClientRect();
      return {
        x: rect.x,
        width: rect.width,
        background: getComputedStyle(element).backgroundColor,
      };
    });
  expect(content.background).toBe(layout.content.background);
  expect(Math.abs(content.x - layout.content.x)).toBeLessThan(2);
  expect(Math.abs(content.width - layout.content.width)).toBeLessThan(2);
});

test("Help page navigation keeps its document and chrome without a frame reload", async ({
  page,
}) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Help", exact: true }).click();
  await expect(
    page
      .frameLocator('iframe[title="jj-store documentation"]')
      .locator(".md-content h1"),
  ).toContainText("jj-store");
  const frame = page.frames().find((frame) => frame.url().endsWith("/help/"))!;
  await expect(frame.locator(".md-content h1")).toContainText("jj-store");
  await frame.evaluate(() =>
    Object.assign(window, {
      helpOriginalDocument: document,
      helpOriginalHeader: document.querySelector("header"),
    }),
  );
  let documentRequests = 0;
  page.on("request", (request) => {
    if (
      request.resourceType() === "document" &&
      request.url().includes("/help/")
    )
      documentRequests++;
  });
  await frame.getByRole("link", { name: "browsing", exact: true }).click();
  await expect(frame.locator(".md-content h1")).toHaveText(
    "History, files and comparisons¶",
  );
  expect(
    await frame.evaluate(() => {
      const original = window as Window & {
        helpOriginalDocument?: Document;
        helpOriginalHeader?: Element;
      };
      return (
        document === original.helpOriginalDocument &&
        document.querySelector("header") === original.helpOriginalHeader
      );
    }),
  ).toBe(true);
  expect(documentRequests).toBe(0);
  await frame
    .getByRole("link", { name: "History, files and comparisons", exact: true })
    .click();
  expect(documentRequests).toBe(0);
  // The joint history entry belongs to the same-document iframe, so there is
  // no top-level load event for Playwright's page.goBack to wait on.
  await page.evaluate(() => window.history.back());
  await expect(frame.locator(".md-content h1")).toContainText("jj-store");
  await page.evaluate(() => window.history.forward());
  await expect(frame.locator(".md-content h1")).toHaveText(
    "History, files and comparisons¶",
  );
});

test("Help page loads keep chrome available and report failed navigation through Retry", async ({
  page,
}) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Help", exact: true }).click();
  const help = page.frameLocator('iframe[title="jj-store documentation"]');
  await expect(help.locator(".md-content h1")).toContainText("jj-store");
  let release!: () => void;
  const gate = new Promise<void>((resolve) => (release = resolve));
  await page.route("**/help/guides/browsing/", async (route) => {
    await gate;
    await route.fulfill({ status: 503, body: "Page temporarily unavailable" });
  });
  await help.getByRole("link", { name: "browsing", exact: true }).click();
  await expect(
    help.getByRole("status", { name: "Loading documentation", exact: true }),
  ).toHaveAttribute("aria-busy", "true");
  await expect(help.getByRole("textbox", { name: "Search" })).toBeVisible();
  release();
  const toast = page
    .getByRole("region", { name: "Notifications" })
    .getByRole("alert");
  await expect(toast).toContainText("Page temporarily unavailable");
  await expect(help.locator(".md-content h1")).toContainText("jj-store");
  await page.unroute("**/help/guides/browsing/");
  await toast.getByRole("button", { name: "Retry" }).click();
  await expect(help.locator(".md-content h1")).toHaveText(
    "History, files and comparisons¶",
  );
  await expect(toast).toHaveCount(0);
});

test("a superseded Help page cannot replace the newer selection", async ({
  page,
}) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Help", exact: true }).click();
  const help = page.frameLocator('iframe[title="jj-store documentation"]');
  await expect(help.locator(".md-content h1")).toContainText("jj-store");
  let release!: () => void;
  const gate = new Promise<void>((resolve) => (release = resolve));
  await page.route("**/help/guides/browsing/", async (route) => {
    await gate;
    await route.continue().catch(() => {});
  });
  await help.getByRole("link", { name: "browsing", exact: true }).click();
  await expect(
    help.getByRole("status", { name: "Loading documentation", exact: true }),
  ).toBeVisible();
  await help
    .locator(".md-sidebar--primary label")
    .filter({ hasText: "Guides" })
    .first()
    .click();
  await help.locator('.md-sidebar--primary a[href*="guides/editing/"]').click();
  await expect(help.locator(".md-content h1")).toHaveText(
    "Editing and conflicts¶",
  );
  release();
  await expect(
    help.getByRole("status", { name: "Loading documentation", exact: true }),
  ).toHaveCount(0);
  await expect(help.locator(".md-content h1")).toHaveText(
    "Editing and conflicts¶",
  );
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
  await expect(
    page.frameLocator("iframe").locator(".md-content h1"),
  ).toContainText("jj-store");
  await page.getByRole("button", { name: "Close Help" }).press("Shift+Tab");
  expect(
    await page.evaluate(
      () => document.activeElement?.closest("dialog") !== null,
    ),
  ).toBe(true);
  await page.getByRole("button", { name: "Close Help" }).click();
  await expect(trigger).toBeFocused();
});
