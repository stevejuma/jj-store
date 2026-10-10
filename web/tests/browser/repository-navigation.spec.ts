import { expect, test } from "@playwright/test";
import { api, operation, repo } from "./native-browser-fixture.js";

const labels = [
  "Overview",
  "Branches",
  "Commits",
  "Op Log",
  "Agents",
  "Reviews",
  "Projects",
];

test("right rail destinations match repository tabs and preserve workspace snapshot", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.goto(
    `/#/repos/${repo}/files?at_op=${operation}&workspace=desktop`,
  );
  const overviewInsets = await page
    .locator(".store-application__body-regions")
    .evaluate((el) => {
      const style = getComputedStyle(el);
      return [
        style.paddingTop,
        style.paddingLeft,
        style.paddingRight,
        style.paddingBottom,
      ].map(Number.parseFloat);
    });
  // JST-UI-015 places the inset in the file card, with a flush body divider.
  expect(overviewInsets).toEqual([0, 0, 0, 0]);
  await expect(page.locator(".store-application__body-regions")).toHaveCSS(
    "border-top-width",
    "1px",
  );
  await expect(page.locator(".store-application__body-regions")).toHaveCSS(
    "margin-top",
    "8px",
  );
  await expect(
    page.getByRole("combobox", { name: "Repository workspace" }),
  ).toContainText("desktop");
  await expect(
    page.getByRole("tablist", { name: "Repository sections" }).getByRole("tab"),
  ).toHaveText(labels);
  await page.getByRole("button", { name: "Toggle details" }).click();
  const menu = page.getByRole("navigation", { name: "Repository views" });
  const rail = page.getByRole("complementary", { name: "Repository details" });
  await expect(rail.locator("section, dl")).toHaveCount(0);
  for (const [label, tab, route] of [
    ["Branches", "Branches", "branches"],
    ["JJ Log", "Commits", "log"],
    ["Op Log", "Op Log", "oplog"],
    ["Agents", "Agents", "agents"],
    ["Reviews", "Reviews", "reviews"],
  ]) {
    await menu.getByRole("button", { name: label, exact: true }).click();
    await expect(
      menu.getByRole("button", { name: label, exact: true }),
    ).toHaveAttribute("aria-current", "page");
    await expect(
      page.getByRole("tab", { name: tab, exact: true }),
    ).toHaveAttribute("data-state", "active");
    const hash = new URL(page.url()).hash;
    expect(
      await page
        .locator(".store-application__body-regions")
        .evaluate((el) => getComputedStyle(el).padding),
    ).toBe("0px");
    expect(hash).toContain(`/repos/${repo}/${route}?`);
    expect(hash).toContain(`at_op=${operation}`);
    expect(hash).toContain("workspace=desktop");
  }
  await page.getByRole("button", { name: "Toggle details" }).click();
  await page.getByRole("button", { name: "Expand left sidebar" }).click();
  const left = page.getByRole("complementary", {
    name: "Repository navigation sidebar",
  });
  for (const label of ["Branches", "JJ Log", "Op Log", "Reviews"])
    await expect(
      left.getByRole("button", { name: label, exact: true }),
    ).toHaveCount(0);
  // Agents and Threads are intentional installation-wide navigation entries.
  await expect(
    left.getByRole("button", { name: "Agents", exact: true }),
  ).toHaveCount(1);
  await expect(
    left.getByRole("button", { name: "Threads", exact: true }),
  ).toHaveCount(1);
});

test("phone overflow supports keyboard selection, reload, resize and right rail navigation", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await api(page, { access: "local_trusted" });
  await page.goto(
    `/#/repos/${repo}/files?at_op=${operation}&workspace=desktop`,
  );
  expect(
    await page
      .locator(".store-application__body-regions")
      .evaluate((el) => el.scrollWidth - el.clientWidth),
  ).toBeLessThanOrEqual(0);
  const more = page.getByRole("button", { name: "More repository sections" });
  await expect(more).toBeVisible();
  await expect(
    page.getByRole("tab", { name: "Overview", exact: true }),
  ).toBeVisible();
  await more.press("Enter");
  const reviews = page.getByRole("menuitem", { name: "Reviews", exact: true });
  await reviews.press("Enter");
  await expect(more).toHaveAttribute("aria-current", "page");
  await more.press("Enter");
  await expect(reviews).toHaveAttribute("aria-current", "page");
  await reviews.press("Escape");
  await expect(more).toBeFocused();
  await page.reload();
  await expect(more).toHaveAttribute("aria-current", "page");
  expect(new URL(page.url()).hash).toContain("workspace=desktop");
  await page
    .getByRole("button", { name: "Open details sidebar", exact: true })
    .click();
  const menu = page.getByRole("navigation", { name: "Repository views" });
  await expect(menu).toBeVisible();
  await menu.getByRole("button", { name: "Op Log", exact: true }).click();
  await expect(menu).not.toBeVisible();
  await expect(
    page.getByRole("tablist", { name: "Repository sections" }),
  ).toBeVisible();
  expect(new URL(page.url()).hash).toContain(`/repos/${repo}/oplog?`);
  expect(new URL(page.url()).hash).toContain(`at_op=${operation}`);
  await page.setViewportSize({ width: 1400, height: 1000 });
  await expect(more).toHaveCount(0);
  await expect(
    page.getByRole("tablist", { name: "Repository sections" }).getByRole("tab"),
  ).toHaveText(labels);
  await expect(
    page.getByRole("tab", { name: "Op Log", exact: true }),
  ).toHaveAttribute("data-state", "active");
});

test("request errors use notifications while the right rail stays minimal", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route(`**/v1/repos/${repo}/log?*`, (route) =>
    route.fulfill({
      status: 503,
      json: {
        version: 1,
        code: "storage_unavailable",
        message: "Fixture history unavailable",
      },
    }),
  );
  await page.goto(`/#/repos/${repo}/log?at_op=${operation}`);
  const notice = page.getByRole("region", { name: "Notifications" });
  await expect(notice.getByText("Repository request failed")).toBeVisible();
  await page.getByRole("button", { name: "Toggle details" }).click();
  const rail = page.getByRole("complementary", { name: "Repository details" });
  await expect(rail.getByText("JJ Log/Changes")).toBeVisible();
  await expect(rail.locator("section, dl")).toHaveCount(0);
  await expect(rail.getByText("Repository request failed")).toHaveCount(0);
});
