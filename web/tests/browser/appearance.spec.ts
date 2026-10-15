import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, repo, operation, sample } from "./native-browser-fixture.js";
import { createThreadTransport } from "../../../catalog/fixtures/thread-transport.js";

async function openSettings(page: import("@playwright/test").Page) {
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  await page.getByRole("button", { name: "Appearance", exact: true }).click();
}

test("cold device appearance restores under the production CSP and propagates across tabs", async ({
  page,
  context,
}) => {
  const violations: string[] = [];
  await context.addInitScript(() => {
    document.addEventListener("securitypolicyviolation", (event) =>
      console.error(
        `CSP:${event.violatedDirective}:${event.blockedURI}:${event.sourceFile}:${event.lineNumber}:${event.columnNumber}`,
      ),
    );
  });
  page.on("console", (message) => {
    if (message.text().startsWith("CSP:")) violations.push(message.text());
  });
  const requests: string[] = [];
  context.on("request", (request) => requests.push(request.url()));
  const setup = async (target: import("@playwright/test").Page) => {
    await api(target, { access: "local_trusted" });
    const transport = createThreadTransport().transport;
    // A healthy empty directory keeps this cold lane scoped to Appearance.
    await target.route(
      new RegExp(`/v1/repos/${repo}/threads(?:\\?|$)`),
      async (route) => {
        const value = await transport.list();
        await route.fulfill({
          json: {
            ...value,
            items: { ...value.items, threads: [] },
            snapshot: {
              ...value.snapshot,
              repo_id: repo,
              operation_heads: [operation],
            },
          },
        });
      },
    );
    await target.route("**/v1/bootstrap", (route) =>
      route.fulfill({
        json: {
          version: 1,
          access_mode: "local_trusted",
          capabilities: sample("capabilities"),
          storage: { ready: true },
          repositories: [{ id: repo, name: "Local acceptance" }],
          backups: [],
        },
      }),
    );
  };
  await setup(page);
  await page.goto(`/#/repos/${repo}/threads`);
  await expect(
    page.getByRole("button", { name: "Open profile menu for Local" }),
  ).toBeVisible();
  const route = await page.evaluate(() => location.hash);
  await openSettings(page);
  await page.getByRole("radio", { name: "Dark", exact: true }).click();
  await page.getByRole("button", { name: /Theme style,/ }).click();
  await page.getByRole("button", { name: "Use Nord theme" }).click();
  await page.getByRole("button", { name: "Use Lilac accent" }).click();
  await page.getByRole("tab", { name: "Preferences", exact: true }).click();
  await page.getByRole("radio", { name: "Larger", exact: true }).click();
  await page.getByRole("radio", { name: "Spacious", exact: true }).click();
  await page.getByRole("radio", { name: "Rich", exact: true }).click();
  await page.getByRole("radio", { name: "Focus", exact: true }).click();
  await page.getByRole("tab", { name: "Theme", exact: true }).click();
  await page.getByRole("switch", { name: "Glass background" }).click();
  const preference = await page.evaluate(() =>
    localStorage.getItem("jj-store.appearance.v1"),
  );
  await expect(page.locator("html")).toHaveAttribute(
    "data-store-theme",
    "nord",
  );
  await expect(page.locator("html")).toHaveAttribute(
    "data-store-font",
    "larger",
  );
  await expect(page.locator("html")).toHaveClass(/dark/);
  const beforeZoom = await page.evaluate(() => ({
    root: getComputedStyle(document.documentElement).fontSize,
    zoom: getComputedStyle(document.documentElement).zoom,
  }));
  expect(beforeZoom).toEqual({ root: "16px", zoom: "1" });
  await page
    .getByRole("button", { name: "Back to jj-store", exact: true })
    .click();
  expect(await page.evaluate(() => location.hash)).toBe(route);
  await page.reload();
  await expect(page.locator("html")).toHaveAttribute(
    "data-store-theme",
    "nord",
  );
  expect(
    await page.evaluate(() => localStorage.getItem("jj-store.appearance.v1")),
  ).toBe(preference);
  const peer = await context.newPage();
  await setup(peer);
  await peer.goto(`/#/repos/${repo}/threads`);
  await expect(peer.locator("html")).toHaveAttribute(
    "data-store-theme",
    "nord",
  );
  await openSettings(page);
  await page.getByRole("radio", { name: "Light", exact: true }).click();
  await expect(peer.locator("html")).toHaveAttribute(
    "data-store-theme",
    "buzz",
  );
  await page.getByRole("switch", { name: "Prominent active tab" }).click();
  await page.emulateMedia({ reducedMotion: "reduce" });
  const a11y = await new AxeBuilder({ page }).analyze();
  expect(a11y.violations).toEqual([]);
  await page.screenshot({
    path: "/tmp/jj-store-appearance.png",
    fullPage: true,
  });
  await page.getByRole("radio", { name: "System", exact: true }).click();
  await page.emulateMedia({ colorScheme: "dark", reducedMotion: "reduce" });
  await expect(page.locator("html")).toHaveAttribute(
    "data-store-theme",
    "buzz-dark",
  );
  await page.emulateMedia({ colorScheme: "light", reducedMotion: "reduce" });
  await expect(page.locator("html")).toHaveAttribute(
    "data-store-theme",
    "buzz",
  );
  await page.setViewportSize({ width: 390, height: 900 });
  await expect(
    page
      .getByRole("navigation", { name: "Settings page navigation" })
      .getByRole("tablist", {
        name: "Appearance settings sections",
        exact: true,
      }),
  ).toBeVisible();
  expect(
    await page
      .locator(".store-appearance")
      .evaluate((el) => el.getBoundingClientRect().width),
  ).toBeLessThanOrEqual(390);
  await page.getByRole("tab", { name: "Preferences", exact: true }).click();
  await page
    .getByRole("radio", { name: "Rich", exact: true })
    .scrollIntoViewIfNeeded();
  const mobileA11y = await new AxeBuilder({ page }).analyze();
  expect(mobileA11y.violations).toEqual([]);
  await page.screenshot({
    path: "/tmp/jj-store-appearance-mobile.png",
    fullPage: true,
  });
  expect(violations).toEqual([]);
  expect(
    requests.filter((url) => !url.startsWith(new URL(page.url()).origin)),
  ).toEqual([]);
  await peer.close();
});
