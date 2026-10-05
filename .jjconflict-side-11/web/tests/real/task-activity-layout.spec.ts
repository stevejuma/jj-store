import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test("task comments use main Activity with a persistent bottom composer", async ({
  page,
}) => {
  const home = process.env.JJ_STORE_REAL_REPOSITORY;
  const project = process.env.JJ_STORE_REAL_PROJECT;
  const task = process.env.JJ_STORE_REAL_TASK;
  test.skip(!home || !project || !task, "Requires a seeded writable task");
  test.setTimeout(120000);
  const route = `/#/projects?home=${home}&project=${project}&task=${task}&tab=tasks`;
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  for (const mobile of [false, true]) {
    await page.setViewportSize(
      mobile ? { width: 390, height: 844 } : { width: 1351, height: 988 },
    );
    await page.addInitScript(
      ({ mobile }) => {
        localStorage.setItem(
          "jj-store.appearance.v1",
          JSON.stringify({
            version: 1,
            mode: mobile ? "dark" : "light",
            theme: mobile ? "github-dark-default" : "github-light-default",
            threads: mobile ? "split" : "focus",
          }),
        );
      },
      { mobile },
    );
    await page.goto(route);
    if (mobile) await page.reload();
    const detail = page.getByRole("article", { name: "Task details" });
    await expect(detail).toBeVisible();
    await expect(
      page.getByRole("complementary", { name: "Task discussion" }),
    ).toHaveCount(0);
    await expect(
      page.getByRole("button", { name: "Open task discussion" }),
    ).toHaveCount(0);
    const activity = detail.getByRole("button", {
      name: "Activity",
      exact: true,
    });
    const subtasks = detail.getByRole("button", {
      name: "Subtasks",
      exact: true,
    });
    expect(
      await subtasks.evaluate(
        (el, next) =>
          Boolean(
            el.compareDocumentPosition(next) & Node.DOCUMENT_POSITION_FOLLOWING,
          ),
        await activity.elementHandle(),
      ),
    ).toBe(true);
    await expect(activity).toHaveAttribute("aria-expanded", "true");
    const comments = detail.getByRole("list", { name: "Task comments" });
    await expect(comments).toBeVisible();
    await expect(
      detail
        .locator(".community-projects__task-activity")
        .getByRole("list", { name: "Task comments" }),
    ).toBeVisible();
    const composer = detail.getByRole("group", {
      name: "Comment",
      exact: true,
    });
    await expect(composer).toBeVisible();
    const content = detail
      .getByRole("region", { name: "Task activity", exact: true })
      .locator(".community-projects__task-activity");
    const contentBox = (await content.boundingBox())!;
    const composerBox = (await composer.boundingBox())!;
    expect(composerBox.y).toBeGreaterThanOrEqual(
      contentBox.y + contentBox.height,
    );
    expect(composerBox.width).toBeGreaterThan(
      (await detail.boundingBox())!.width * 0.85,
    );
    const editor = composer.getByRole("textbox");
    await editor.fill("A local draft kept below Activity");
    await activity.focus();
    await activity.press("Enter");
    await expect(activity).toHaveAttribute("aria-expanded", "false");
    await expect(comments).not.toBeVisible();
    await expect(editor).toBeVisible();
    await expect(editor).toHaveText("A local draft kept below Activity");
    await activity.press("Enter");
    await expect(comments).toBeVisible();
    await expect(editor).toHaveText("A local draft kept below Activity");
    await editor.fill("");
    const first = comments.getByRole("listitem").first();
    const comment = await first.getAttribute("data-message-id");
    expect(comment).toBeTruthy();
    await activity.press("Enter");
    await expect(activity).toHaveAttribute("aria-expanded", "false");
    await page.goto(`${route}&comment=${comment}`);
    const focused = detail.locator(
      `[role="listitem"][data-message-id="${comment}"]`,
    );
    await expect(activity).toHaveAttribute("aria-expanded", "true");
    await expect(focused).toBeVisible();
    await expect(focused).toHaveAttribute("data-highlighted", "true");
    await expect(focused).toBeFocused();
    await expect(
      detail.getByRole("button", { name: "Refresh projects", exact: true }),
    ).toBeEnabled();
    await expect(
      detail.getByRole("group", { name: "Comment", exact: true }),
    ).toBeVisible();
    await page.evaluate(async () => {
      await document.fonts.ready;
      await new Promise<void>((resolve) =>
        requestAnimationFrame(() => requestAnimationFrame(() => resolve())),
      );
    });
    expect(
      await page.evaluate(
        () => document.documentElement.scrollWidth <= innerWidth,
      ),
    ).toBe(true);
    expect(
      (
        await new AxeBuilder({ page })
          .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
          .analyze()
      ).violations,
    ).toEqual([]);
    await page.screenshot({
      path: `/tmp/jj-task-activity-${mobile ? "mobile" : "desktop"}.png`,
    });
  }
  expect(errors).toEqual([]);
});
