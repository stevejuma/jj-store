import { expect } from "@playwright/test";
export async function nextMessageAcceptance(page, base) {
  await page.goto(
    `${base}/iframe.html?id=jj-store-commentable-agent-chat-body--inline-next-message-and-fixed-send-actions&viewMode=story&embed=true`,
  );
  const draft = page.getByRole("textbox", { name: "Next message" });
  await expect(draft).toBeVisible({ timeout: 60000 });
  await page.getByRole("button", { name: "Jump to end" }).click();
  await expect(draft).toBeFocused();
  await page.keyboard.type("First line");
  await page.keyboard.press("Enter");
  await page.keyboard.type("Second line");
  await expect(draft.locator(".cm-line")).toHaveCount(2);
  await expect(
    page.getByRole("textbox", { name: "Local message" }),
  ).toHaveCount(0);
  const footer = page.getByRole("group", { name: "Message actions" });
  const before = await footer.boundingBox();
  await page
    .getByRole("region", { name: "Agent chat body" })
    .evaluate((node) => {
      node.scrollTop = 0;
    });
  expect((await footer.boundingBox()).y).toBe(before.y);
  await page.getByRole("button", { name: "Send message" }).click();
  await expect(
    page.getByRole("textbox", { name: "Local message" }),
  ).toContainText("Second line");
  await expect(draft).toBeFocused();
  await expect(
    page.getByRole("button", { name: "Send message" }),
  ).toBeDisabled();
  await expect(draft).toBeInViewport();
}
