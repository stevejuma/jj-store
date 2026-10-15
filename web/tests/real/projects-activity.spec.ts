import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test("native project task and review pages share pinned commit activity", async ({
  page,
  request,
}) => {
  const home = process.env.JJ_STORE_REAL_REPOSITORY!;
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(`/#/projects?tab=activity&scope=${home}`);
  const view = page.getByRole("region", { name: "Projects and tasks" });
  await expect(
    view.getByRole("tab", { name: "Activity", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  const feed = view.getByRole("list", { name: "Repository activity" });
  await expect(feed).toBeVisible();
  const activityViewport = view
    .locator('.store-projects__scroll [data-ui-part="scroll-area-viewport"]')
    .first();
  await expect(activityViewport).toBeVisible();
  await expect(
    feed
      .locator('[data-ui-component="community-projects-virtual-rows"]')
      .first(),
  ).toBeVisible();
  await expect(
    feed.getByRole("link", { name: /Open commit/ }).first(),
  ).toHaveAttribute(
    "href",
    new RegExp(`/repos/${home}/log\\?at_op=[0-9a-f]+&commit=`),
  );
  // This seed captures agent tools without claiming code associations.
  await expect(
    feed.getByRole("button", { name: "Agent context", exact: true }),
  ).toHaveCount(0);
  const operation = new URLSearchParams(
    (await feed
      .getByRole("link", { name: /Open commit/ })
      .first()
      .getAttribute("href"))!.split("?")[1],
  ).get("at_op")!;
  await page.screenshot({
    path: test.info().outputPath("project-activity.png"),
  });
  const activityPath = page.url();
  await activityViewport.evaluate((el) =>
    el.scrollTo(0, Math.min(60, el.scrollHeight - el.clientHeight)),
  );
  const activityScroll = await activityViewport.evaluate((el) => el.scrollTop);
  await feed
    .getByRole("link", { name: /Open commit/ })
    .first()
    .click();
  await expect(
    page.getByRole("tab", { name: "Commits", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await expect(
    page.getByRole("button", { name: "Application menu", exact: true }),
  ).toBeEnabled();
  const back = page.getByRole("button", { name: "Back", exact: true });
  await expect(back).toBeEnabled();
  await back.click();
  await expect(feed).toBeVisible();
  expect(page.url()).toBe(activityPath);
  await expect
    .poll(() => activityViewport.evaluate((el) => el.scrollTop))
    .toBe(activityScroll);
  const forward = page.getByRole("button", { name: "Forward", exact: true });
  await expect(forward).toBeEnabled();
  await forward.click();
  await expect(
    page.getByRole("tab", { name: "Commits", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await back.click();
  await expect(feed).toBeVisible();
  await view.getByRole("tab", { name: "Projects", exact: true }).click();
  await view
    .getByRole("button", {
      name: "Open project Native repository browser",
      exact: true,
    })
    .click();
  await view.getByRole("tab", { name: "Activity", exact: true }).click();
  await expect(feed).toBeVisible();
  await view.getByRole("tab", { name: "Tasks", exact: true }).click();
  await view
    .getByRole("button", {
      name: "Open task Ship repository browsing",
      exact: true,
    })
    .click();
  const task = view.getByRole("article", { name: "Task details" });
  await expect(
    task.getByRole("button", { name: "Description", exact: true }),
  ).toHaveAttribute("aria-expanded", "true");
  for (const name of ["Subtasks"])
    await expect(
      task.getByRole("button", { name, exact: true }),
    ).toHaveAttribute("aria-expanded", "false");
  await expect(
    task.getByRole("button", { name: "Activity", exact: true }),
  ).toHaveAttribute("aria-expanded", "true");
  await expect(task.getByRole("list", { name: "Task comments" })).toBeVisible();
  await expect(
    task.getByRole("list", { name: "Repository activity" }),
  ).toHaveCount(0);
  const selected = page.url();
  await page.reload();
  await expect(task).toBeVisible();
  expect(page.url()).toBe(selected);
  await expect(
    task.getByRole("button", { name: "Activity", exact: true }),
  ).toHaveAttribute("aria-expanded", "true");
  await page.goto(`/#/repos/${home}/reviews?at_op=${operation}`);
  await page
    .getByRole("button", { name: /^Open Revision open review/ })
    .first()
    .click();
  const review = page.getByRole("article", { name: "Review details" });
  await expect(review).toBeVisible();
  await expect(
    page.getByRole("tab", { name: "Reviews", exact: true }),
  ).toHaveCount(0);
  await expect(
    page.getByRole("banner", { name: "Workspace section header" }),
  ).toHaveCount(0);
  await expect(
    page.locator('[aria-label="Workspace section header"]'),
  ).toHaveCount(0);
  for (const name of ["Description", "Files changed", "Commits", "Activity"])
    await expect(
      review.getByRole("button", { name, exact: true }),
    ).toBeVisible();
  await review
    .getByRole("button", { name: "Files changed", exact: true })
    .click();
  await expect(
    review.getByRole("complementary", { name: "Changed files" }),
  ).toBeVisible({ timeout: 15000 });
  await expect(
    review.locator('[data-ui-component="file-diff-composer"]'),
  ).toBeVisible();
  const stats = review.locator(
    '.community-projects__diff-summary [data-ui-component="file-change-stats"]',
  );
  await expect(stats).toContainText("+");
  await expect(stats).toContainText("-");
  await review.getByRole("button", { name: "Commits", exact: true }).click();
  await expect(
    review.getByRole("list", { name: "Review commits" }),
  ).toBeVisible();
  await expect(
    review.getByRole("list", { name: "Repository activity" }),
  ).toBeVisible();
  const axe = await new AxeBuilder({ page })
    .include(".community-projects__review-detail")
    .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
    .analyze();
  expect(axe.violations).toEqual([]);
  await page
    .locator(
      '.store-collaboration__review-scroll [data-ui-part="scroll-area-viewport"]',
    )
    .first()
    .evaluate((el) => el.scrollTo(0, 0));
  await page.screenshot({
    path: test.info().outputPath("native-review-detail.png"),
  });
  await page.setViewportSize({ width: 420, height: 900 });
  expect(
    await review.evaluate((el) => el.scrollWidth <= el.clientWidth + 1),
  ).toBe(true);
  await page.setViewportSize({ width: 1400, height: 1000 });
  await review
    .getByRole("button", { name: "Back to reviews", exact: true })
    .click();
  await expect(
    page.getByRole("list", { name: "Review threads" }),
  ).toBeVisible();
  await page.goto(
    `/#/repos/${home}/reviews?at_op=${operation}&review_view=columns`,
  );
  await expect(
    page.getByRole("list", { name: "Review threads" }),
  ).toBeVisible();
  await expect(page.locator('[aria-label="Review layout"]')).toHaveCount(0);
  await expect(page.locator('[data-ui-component="column-canvas"]')).toHaveCount(
    0,
  );
  await page.screenshot({
    path: test.info().outputPath("native-review-list.png"),
  });
  const headers = {
    Origin: new URL(page.url()).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const reviewsResponse = await request.get(
    `/v1/repos/${home}/reviews/threads?limit=50`,
    { headers },
  );
  expect(reviewsResponse.ok()).toBe(true);
  const anchored = (await reviewsResponse.json()).threads.find(
    (row: any) =>
      row.anchor?.target?.target === "agent_artifact" &&
      row.anchor.target.value.owner.kind === "tool",
  );
  expect(anchored).toBeTruthy();
  await page.goto(
    `/#/repos/${home}/reviews?at_op=${operation}&thread=${anchored.thread_id}`,
  );
  await review
    .getByRole("button", { name: "Agent context", exact: true })
    .click();
  await expect(
    review.getByRole("log", { name: "Conversation messages", exact: true }),
  ).toBeVisible();
  await expect(
    review.getByRole("button", { name: /^Show details for / }).first(),
  ).toBeVisible();
  await page.screenshot({
    path: test.info().outputPath("anchored-agent-review.png"),
  });
  expect(errors).toEqual([]);
});
