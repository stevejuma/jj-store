import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test("repository documents linked projects and Activity filters retain native scope", async ({
  page,
}) => {
  const home = process.env.JJ_STORE_REAL_REPOSITORY!;
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(`/#/repos/${home}/files`);
  const documents = page.getByRole("region", {
    name: "Repository document preview",
  });
  await expect(documents).toBeVisible();
  const pin = new URLSearchParams(page.url().split("?")[1]).get("at_op")!;
  for (const label of [
    "README",
    "Code of conduct",
    "Contributing",
    "License",
    "Security",
  ]) {
    const tab = documents.getByRole("tab", { name: label, exact: true });
    if (await tab.count()) await tab.click();
    else {
      await documents
        .getByRole("button", { name: "More repository documents" })
        .click();
      await page.getByRole("menuitem", { name: label, exact: true }).click();
    }
    const preview = documents.getByRole("region", {
      name: `${label} preview`,
      exact: true,
    });
    await expect(preview.locator(".mira-markdown-preview")).toBeVisible();
    if (label === "README")
      await expect(preview.locator("p code").first()).toHaveText(
        "pnpm dev:reset",
      );
    const selected = documents
      .locator(
        '[data-state="active"] .community-projects__repository-document-tab-indicator, [data-selected] .community-projects__repository-document-tab-indicator',
      )
      .first();
    expect((await selected.boundingBox())!.height).toBeGreaterThanOrEqual(3);
  }
  await page.screenshot({
    path: test.info().outputPath("repository-markdown.png"),
  });
  await page.goto(`/#/repos/${home}/projects?at_op=${pin}`);
  const linked = page.getByRole("region", {
    name: "Linked repository projects",
  });
  await expect(linked).toBeVisible();
  await expect(
    page.getByRole("tab", { name: "Projects", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await expect(
    page.getByRole("tab", { name: "Overview", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("tab", { name: "Activity", exact: true }),
  ).toHaveCount(0);
  const repositoryPath = page.url();
  await expect(
    linked.getByRole("button", { name: /^Open project / }).first(),
  ).toBeVisible();
  const before = await linked
    .getByRole("button", { name: /^Open project / })
    .count();
  expect(before).toBeGreaterThan(0);
  await page.reload();
  await expect(
    linked.getByRole("button", { name: /^Open project / }),
  ).toHaveCount(before);
  expect(page.url()).toBe(repositoryPath);
  await linked
    .getByRole("button", { name: /^Open project / })
    .first()
    .click();
  await expect(
    page.getByRole("tab", { name: "Activity", exact: true }),
  ).toBeVisible();
  await expect(
    page
      .getByRole("tablist", { name: "Project views" })
      .getByRole("tab")
      .first(),
  ).toHaveText("Activity");
  await page.getByRole("tab", { name: "Activity", exact: true }).click();
  const details = page.getByRole("button", {
    name: "Toggle details",
    exact: true,
  });
  if ((await details.getAttribute("aria-expanded")) === "false")
    await details.click();
  const filter = page.getByRole("button", {
    name: "Filter activity",
    exact: true,
  });
  await expect(
    page
      .getByRole("region", { name: "Project filters" })
      .getByRole("button", { name: "Filter activity" }),
  ).toBeVisible();
  await filter.click();
  await page.getByPlaceholder("Find an activity type...").fill("Without");
  await page.getByRole("option", { name: /Without agent context/ }).click();
  await page.getByRole("button", { name: "Close activity filter" }).click();
  await expect(
    page.getByRole("list", { name: "Repository activity" }),
  ).toBeVisible();
  await expect(page).toHaveURL(/activity=unassisted/);
  const filteredPath = page.url();
  const destination = page.getByRole("link", { name: /^Open commit / }).first();
  await destination.click();
  await expect(
    page.getByRole("tab", { name: "Commits", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await page.getByRole("button", { name: "Back", exact: true }).click();
  await expect(filter).toHaveAttribute("aria-pressed", "true");
  expect(page.url()).toBe(filteredPath);
  await page.reload();
  await expect(
    page.getByRole("list", { name: "Repository activity" }),
  ).toBeVisible();
  expect(page.url()).toBe(filteredPath);
  if ((await details.getAttribute("aria-expanded")) === "false")
    await details.click();
  await filter.click();
  await page.getByRole("option", { name: /With agent context/ }).click();
  await page.getByRole("button", { name: "Close activity filter" }).click();
  // The seed has no code associations; absence must remain an honest empty state.
  await expect(
    page.getByText("No matching activity", { exact: true }),
  ).toBeVisible();
  await page.screenshot({
    path: test.info().outputPath("project-activity-filter.png"),
  });
  expect(
    (
      await new AxeBuilder({ page })
        .include(".store-projects")
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
  await page.goto("/#/projects?tab=activity&activity=unassisted");
  await expect(
    page
      .getByRole("tablist", { name: "Projects views" })
      .getByRole("tab")
      .first(),
  ).toHaveText("Activity");
  await expect(
    page.getByRole("list", { name: "Repository activity" }),
  ).toBeVisible();
  expect(errors).toEqual([]);
});
