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
  await expect(
    page.getByRole("form", { name: "Inline comment composer" }),
  ).toHaveCount(0);
  await page.keyboard.type("Check hidden line");
  const composer = page.getByRole("form", { name: "Inline comment composer" });
  await expect(composer.locator("blockquote")).toHaveText(
    "export const line13 = 13;",
  );
  await expect(
    composer.getByRole("textbox", { name: "Markdown editor" }),
  ).toHaveText("Check hidden line");
  await page.getByRole("button", { name: "Add comment" }).click();
  await page.getByRole("button", { name: "Hide body" }).click();
  await page.getByRole("button", { name: "Show body" }).click();
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
