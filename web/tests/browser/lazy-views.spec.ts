import { expect, test, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";
import { api, operation, repo } from "./native-browser-fixture.js";

const manifest = JSON.parse(
  readFileSync(
    new URL("../../dist/.vite/manifest.json", import.meta.url),
    "utf8",
  ),
);
const chunk = (name: string): string =>
  manifest[
    Object.keys(manifest).find((key) => key.endsWith(`/${name}.svelte`))!
  ].file;

async function settings(page: Page) {
  const expand = page.getByRole("button", {
    name: "Expand left sidebar",
    exact: true,
  });
  if (await expand.isVisible()) await expand.click();
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
}

test("a failed view chunk offers explicit retry without reloading or replaying commands", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  let imports = 0;
  let mutations = 0;
  page.on("request", (request) => {
    if (request.method() === "POST") mutations += 1;
  });
  await page.route(`**/${chunk("StoreSettings")}*`, async (route) => {
    if (++imports === 1)
      await route.fulfill({ status: 503, body: "temporarily unavailable" });
    else await route.continue();
  });
  await page.goto(`/#/repos/${repo}/files?at_op=${operation}`);
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await settings(page);
  await expect(page.getByText("The view could not be loaded.")).toBeVisible();
  await page.getByRole("button", { name: "Retry", exact: true }).click();
  await expect(
    page.getByRole("region", { name: "Settings content" }),
  ).toBeVisible();
  expect(imports).toBe(2);
  expect(mutations).toBe(0);
  expect(new URL(page.url()).hash).toContain(`at_op=${operation}`);
});

test("cold built shell loads selected views on demand and reuses completed imports", async ({
  page,
}) => {
  const requests: string[] = [];
  page.on("request", (request) =>
    requests.push(new URL(request.url()).pathname),
  );
  await api(page, { access: "local_trusted" });
  const opened = Date.now();
  await page.goto(`/#/repos/${repo}/files?at_op=${operation}`);
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  expect(requests.some((path) => /\.svelte(?:\.ts)?$/.test(path))).toBe(false);
  console.log(
    "Cold built Files measurement",
    JSON.stringify({
      requests: requests.length,
      renderedMillis: Date.now() - opened,
      timing: await page.evaluate(() => {
        const navigation = performance.getEntriesByType(
          "navigation",
        )[0] as PerformanceNavigationTiming;
        return {
          domContentLoaded: navigation.domContentLoadedEventEnd,
          load: navigation.loadEventEnd,
          paints: performance
            .getEntriesByType("paint")
            .map(({ name, startTime }) => ({ name, startTime })),
        };
      }),
    }),
  );
  for (const view of [
    "StoreSettings",
    "StoreReviewThreads",
    "StoreAgentExecutionHost",
    "StoreProjects",
  ])
    expect(requests).not.toContain(`/${chunk(view)}`);
  await settings(page);
  await expect(
    page.getByRole("region", { name: "Settings content" }),
  ).toBeVisible();
  expect(
    requests.filter((path) => path === `/${chunk("StoreSettings")}`),
  ).toHaveLength(1);
  await page.getByRole("button", { name: "Back to jj-store" }).click();
  await settings(page);
  await expect(
    page.getByRole("region", { name: "Settings content" }),
  ).toBeVisible();
  expect(
    requests.filter((path) => path === `/${chunk("StoreSettings")}`),
  ).toHaveLength(1);
});

test("navigation away from a delayed view cannot render its obsolete completion", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  let release!: () => void;
  const pending = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route(`**/${chunk("StoreSettings")}`, async (route) => {
    await pending;
    await route.continue();
  });
  await page.goto(`/#/repos/${repo}/files?at_op=${operation}`);
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await settings(page);
  await expect(
    page.getByRole("status", { name: "Loading Settings", exact: true }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Back to jj-store" }).click();
  release();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await expect(
    page.getByRole("region", { name: "Settings content" }),
  ).toHaveCount(0);
  await settings(page);
  await expect(
    page.getByRole("region", { name: "Settings content" }),
  ).toBeVisible();
});
