import { chromium, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
const [origin, session, title] = process.argv.slice(2);
if (
  !/^http:\/\/127\.0\.0\.1:\d+$/.test(origin) ||
  !/^[a-f0-9]{32}$/.test(session)
)
  throw Error("Expected isolated metadata fixture");
const browser = await chromium.launch();
try {
  const context = await browser.newContext({
    viewport: { width: 1440, height: 1100 },
  });
  const page = await context.newPage(),
    errors = [],
    resizeWarnings = [];
  page.on("pageerror", (error) =>
    (error.message.includes("ResizeObserver") ? resizeWarnings : errors).push(
      error.message,
    ),
  );
  const writes = [];
  page.on("request", (request) => {
    if (
      request.method() === "POST" &&
      request.url().includes("/agents/metadata")
    )
      writes.push(request.postDataJSON());
  });
  await page.goto(`${origin}/#/agents/sessions/${session}`);
  await expect(
    page
      .getByRole("region", { name: "Selected agent session", exact: true })
      .locator(".lc-community-inbox__thread-header")
      .first(),
  ).toContainText(title);
  await page
    .getByRole("button", { name: "Session options", exact: true })
    .click();
  await page
    .getByRole("menuitem", { name: "Conversation details", exact: true })
    .click();
  const sidebar = page.getByRole("complementary", {
    name: "Conversation details",
    exact: true,
  });
  await expect(
    sidebar.getByRole("textbox", { name: "Rename conversation" }),
  ).toBeVisible();
  await expect(
    sidebar.getByText("metadata-discovery-keyword", { exact: true }),
  ).toBeVisible();
  await expect(sidebar.getByText(title, { exact: true })).toBeVisible();
  await sidebar
    .getByRole("textbox", { name: "Rename conversation" })
    .fill("Retained UI draft");
  await page
    .getByRole("button", { name: "Close conversation details" })
    .click();
  await page
    .getByRole("button", { name: "Session options", exact: true })
    .click();
  await page
    .getByRole("menuitem", { name: "Conversation details", exact: true })
    .click();
  await expect(
    sidebar.getByRole("textbox", { name: "Rename conversation" }),
  ).toHaveValue("Retained UI draft");
  await page
    .getByRole("searchbox", { name: "Search loaded conversations" })
    .fill("metadata-discovery-keyword");
  await expect(
    page.getByRole("button", { name: new RegExp(`^Open session ${title}`) }),
  ).toBeVisible();
  const axe = await new AxeBuilder({ page })
    .disableRules(["color-contrast"])
    .analyze();
  expect(axe.violations).toEqual([]);
  expect(errors).toEqual([]);
  expect(writes).toEqual([]);
  console.log(
    JSON.stringify({
      metadataBrowser: "pass",
      writes: writes.length,
      resizeWarnings,
    }),
  );
} finally {
  await browser.close();
}
