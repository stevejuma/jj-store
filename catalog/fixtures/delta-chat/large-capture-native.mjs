import { nativeStoryControl } from "../native-story-controls.mjs";
import { expect } from "@playwright/test";
export async function largeCaptureAcceptance(page, base) {
  await page.goto(
    `${base}/iframe.html?id=jj-store-commentable-agent-chat-body--large-code-and-diff-middle-folding&viewMode=story&embed=true`,
  );
  const code = page.getByRole("textbox", { name: "Large captured code" });
  const patch = page.getByRole("textbox", { name: "Large captured patch" });
  await expect(code).toBeVisible({ timeout: 60000 });
  for (const source of [code, patch]) {
    await expect(source).toContainText("line60 = 60");
    await expect(source).not.toContainText("line30 = 30");
    expect(
      await source.evaluate(
        (node) =>
          getComputedStyle(node.closest(".chat-text-block")).backgroundColor,
      ),
    ).not.toBe("rgba(0, 0, 0, 0)");
  }
  await page
    .getByRole("button", { name: "Show 40 more lines in Large captured code" })
    .focus();
  await page.keyboard.press("Space");
  await expect(code).toContainText("line30 = 30");
  const less = page.getByRole("button", {
    name: "Show less in Large captured code",
  });
  await expect(less).toHaveAttribute("aria-expanded", "true");
  await less.focus();
  await page.keyboard.press("Enter");
  await expect(code).not.toContainText("line30 = 30");
  await expect(code).toContainText("line60 = 60");
  await nativeStoryControl(page, "Hide body");
  await nativeStoryControl(page, "Show body");
  const more = page.getByRole("button", {
    name: "Show 40 more lines in Large captured code",
  });
  await expect(more).toHaveAttribute("aria-expanded", "false");
  await more.focus();
  await page.keyboard.press("Space");
  await expect(code).toContainText("line30 = 30");
  await expect(
    page.locator('[data-location="inline"][data-editing="true"]'),
  ).toHaveCount(0);
  await page.keyboard.type("Check hidden line");
  const composer = page.locator(
    '[data-location="inline"][data-editing="true"]',
  );
  await expect(
    page
      .getByRole("group", { name: "Pending comments", exact: true })
      .locator("blockquote"),
  ).toHaveText("export const line13 = 13;");
  await expect(composer.getByRole("textbox")).toHaveText("Check hidden line");
  await page.getByRole("button", { name: "Send message" }).click();
  await nativeStoryControl(page, "Hide body");
  await nativeStoryControl(page, "Show body");
  await expect(code).toContainText("line30 = 30");
  await expect(
    page.getByRole("article", { name: "Comment by You" }),
  ).toContainText("Check hidden line");
  await expect(
    page.getByRole("button", {
      name: "Show 40 more lines in Large captured patch",
    }),
  ).toBeVisible();
}
