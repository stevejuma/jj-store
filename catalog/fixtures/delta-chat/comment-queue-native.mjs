import { expect } from "@playwright/test";
export async function commentQueueAcceptance(page, base) {
  await page.goto(
    `${base}/iframe.html?id=jj-store-commentable-agent-chat-body--interaction-playground&viewMode=story`,
  );
  const source = page.getByRole("textbox", {
    name: "Assistant explanation",
    exact: true,
  });
  await expect(source).toBeVisible();
  await source.click();
  await page.keyboard.type("First café");
  const inline = page.locator('[data-location="inline"]').getByRole("textbox");
  const pending = page.getByRole("group", {
    name: "Pending comments",
    exact: true,
  });
  const queued = pending.getByRole("textbox");
  await expect(queued).toHaveText("First café");
  await queued.focus();
  await page.keyboard.press("Home");
  await page.keyboard.press("ArrowRight");
  await page.keyboard.type("xy");
  await expect(inline).toHaveText("Fxyirst café");
  await expect(queued).toBeFocused();
  await page.keyboard.type("z");
  await expect(inline).toHaveText("Fxyzirst café");
  await page.keyboard.press("Escape");
  await expect(source).toBeFocused();
  await page.getByRole("button", { name: "Hide body" }).click();
  await page.getByRole("button", { name: "Show body" }).click();
  await expect(queued).toHaveText("Fxyzirst café");
  await pending
    .getByRole("button", { name: "Go to source: Assistant explanation" })
    .focus();
  await expect(pending.locator(".chat-comment-links")).toHaveCSS(
    "opacity",
    "1",
  );
  await page.keyboard.press("Enter");
  await expect(source).toBeFocused();
  await page
    .getByRole("textbox", { name: "Next message", exact: true })
    .fill("Message with comments");
  await page.getByRole("button", { name: "Fail next send" }).click();
  await page.getByRole("button", { name: "Send message" }).click();
  await expect(page.getByRole("alert")).toContainText("retained");
  await expect(queued).toHaveText("Fxyzirst café");
  await expect(
    page.getByRole("textbox", { name: "Next message", exact: true }),
  ).toHaveText("Message with comments");
  await queued.focus();
  await page.keyboard.press(
    process.platform === "darwin" ? "Meta+Enter" : "Control+Enter",
  );
  await expect(pending).toHaveCount(0);
  await expect(
    page.getByRole("textbox", { name: "Next message", exact: true }),
  ).toBeFocused();
  await expect(
    page.getByRole("textbox", { name: "Local message", exact: true }),
  ).toHaveText("Message with comments");
  await expect(
    page.getByRole("img", { name: "Replied", exact: true }),
  ).toHaveCount(0);
  await page.getByRole("button", { name: "Simulate replies" }).click();
  const original = page
    .locator('[data-placement="inline"]')
    .filter({ hasText: "Fxyzirst café" });
  await original.locator(".chat-comment-card").last().hover();
  await expect(original.locator(".chat-comment-links").last()).toHaveCSS(
    "opacity",
    "1",
  );
  await expect(
    original.getByRole("button", { name: "Reply", exact: true }),
  ).toHaveText("");
  await original.getByRole("button", { name: "View reply" }).click();
  await expect(
    page.getByRole("article", { name: "Simulated agent reply" }).locator(".."),
  ).toBeFocused();
  await page
    .locator('[data-placement="reply"]')
    .getByRole("button", { name: "Go to source: Assistant explanation" })
    .click();
  await expect(source).toBeFocused();
  await expect(original.getByRole("img", { name: "Replied" })).toBeVisible();
  console.log(
    "PASS shared queue editing/caret retention, mixed prompt, failed send, focus and source/reply navigation",
  );
  const touch = await page
    .context()
    .browser()
    .newPage({
      hasTouch: true,
      viewport: { width: 390, height: 844 },
    });
  try {
    await touch.goto(
      `${base}/iframe.html?id=jj-store-commentable-agent-chat-body--delta-comments-reference&viewMode=story`,
    );
    const queue = touch.getByRole("group", {
      name: "Pending comments",
      exact: true,
    });
    await expect(queue.locator(".chat-comment-links")).toHaveCSS(
      "opacity",
      "1",
    );
    await queue.getByRole("button", { name: /Go to source/ }).tap();
    await expect(
      touch.locator('[data-chat-reader="reply-0/text/52"]'),
    ).toBeFocused();
    await queue.getByRole("button", { name: "Remove pending comment" }).tap();
    await expect(queue).toHaveCount(0);
    console.log(
      "PASS touch comment actions are visible and navigate/remove pending comments",
    );
  } finally {
    await touch.close();
  }
}
