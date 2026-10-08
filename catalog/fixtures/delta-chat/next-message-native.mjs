import { expect } from "@playwright/test";
export async function nextMessageAcceptance(page, base) {
  await page.goto(
    `${base}/iframe.html?id=jj-store-commentable-agent-chat-body--inline-next-message-and-fixed-send-actions&viewMode=story&embed=true`,
  );
  const draft = page.getByRole("textbox", { name: "Next message" });
  await expect(draft).toBeVisible({ timeout: 60000 });
  const body = page.getByRole("region", { name: "Agent chat body" });
  const viewport = body.locator('[data-ui-part="scroll-area-viewport"]');
  const frame = await page.locator(".chat-body-fixture").boundingBox();
  expect(frame.x).toBe(0);
  expect(frame.y).toBe(0);
  expect(frame.height).toBe(page.viewportSize().height);
  expect(
    await page.evaluate(() => document.scrollingElement.scrollHeight),
  ).toBe(page.viewportSize().height);
  await page.getByRole("button", { name: "Jump to end" }).click();
  await expect(draft).toBeFocused();
  await expect
    .poll(() =>
      viewport.evaluate(
        (node) => node.scrollHeight - node.clientHeight - node.scrollTop,
      ),
    )
    .toBeLessThanOrEqual(1);
  await page.keyboard.type("First line");
  await page.keyboard.press("Enter");
  await page.keyboard.type("Second line");
  await expect(draft.locator(".cm-line")).toHaveCount(2);
  await expect(
    page.getByRole("textbox", { name: "Local message" }),
  ).toHaveCount(0);
  const footer = page.getByRole("group", { name: "Message actions" });
  const before = await footer.boundingBox();
  const toolbar = page.locator('[data-ui-part="toolbar"]');
  const headerBefore = await toolbar.boundingBox();
  const end = await viewport.evaluate((node) => node.scrollTop);
  expect(end).toBeGreaterThan(0);
  const bounds = await viewport.boundingBox();
  await page.mouse.move(bounds.x + 5, bounds.y + bounds.height / 2);
  await page.mouse.wheel(0, -300);
  await expect
    .poll(() => viewport.evaluate((node) => node.scrollTop))
    .toBeLessThan(end);
  await viewport.evaluate((node) => {
    node.scrollTop = 0;
  });
  await expect.poll(() => viewport.evaluate((node) => node.scrollTop)).toBe(0);
  expect((await footer.boundingBox()).y).toBe(before.y);
  expect((await toolbar.boundingBox()).y).toBe(headerBefore.y);
  await expect(draft).toContainText("Second line");
  await page.getByRole("button", { name: "Send message" }).click();
  await expect(
    page.getByRole("textbox", { name: "Local message" }),
  ).toContainText("Second line");
  await expect(draft).toBeFocused();
  await expect(
    page.getByRole("button", { name: "Send message" }),
  ).toBeDisabled();
  await expect(draft).toBeInViewport();
  console.log(
    "Draft caret/alignment",
    await draft.evaluate((node) => {
      const editor = node.closest(".cm-editor");
      const caret = editor.querySelector(".cm-cursor");
      const style = getComputedStyle(caret);
      const line = node.querySelector(".cm-line").getBoundingClientRect();
      const avatar = node
        .closest(".chat-next-message")
        .querySelector(".chat-next-avatar")
        .firstElementChild.getBoundingClientRect();
      return {
        width: style.borderLeftWidth,
        color: style.borderLeftColor,
        primary: getComputedStyle(node).getPropertyValue("--primary").trim(),
        mutedForeground: getComputedStyle(node)
          .getPropertyValue("--muted-foreground")
          .trim(),
        scale: style.transform,
        firstLineCenter: line.top + line.height / 2,
        avatarCenter: avatar.top + avatar.height / 2,
        placeholder:
          editor.querySelector(".cm-placeholder")?.textContent ?? null,
        readingCaret: (() => {
          const reading = document.querySelector(
            ".chat-text-reader .cm-cursor",
          );
          const css = reading && getComputedStyle(reading);
          return (
            css && { width: css.borderLeftWidth, color: css.borderLeftColor }
          );
        })(),
      };
    }),
  );
}
