import { expect } from "@playwright/test";
export async function readerInputAcceptance(page, source) {
  const composer = () =>
    page.locator('[data-location="inline"][data-editing="true"]');
  const draft = () => composer().getByRole("textbox");
  const quote = () =>
    page
      .getByRole("group", { name: "Pending comments", exact: true })
      .locator("blockquote")
      .last();
  const capturedText = () =>
    source.locator(":scope > .cm-line").allTextContents();
  await expect(source).toBeVisible({ timeout: 60000 });
  await page.evaluate(() => document.fonts.ready);
  const original = await capturedText();
  const mac = process.platform === "darwin";
  const start = mac ? "Meta+ArrowUp" : "Control+Home";
  const lineStart = mac ? "Meta+ArrowLeft" : "Home";
  const selectLine = mac ? "Meta+Shift+ArrowRight" : "Shift+End";

  // Actual pointer selection, including selection across styled inline text.
  const strong = source.locator(".chat-strong");
  const box = await strong.boundingBox();
  await page.mouse.move(box.x + 1, box.y + box.height / 2);
  await page.mouse.down();
  await page.mouse.move(box.x + box.width + 0.25, box.y + box.height / 2, {
    steps: 10,
  });
  await page.mouse.up();
  await expect
    .poll(() => page.evaluate(() => window.getSelection().toString()))
    .toBe("stable identities");
  await page.keyboard.press("Control+Alt+m");
  await page.keyboard.type("Pointer draft");
  await expect(quote()).toHaveText("stable identities");
  await expect(draft()).toHaveText("Pointer draft");
  await page.keyboard.press("Escape");
  await expect(composer()).toHaveCount(0);
  await expect(source).toBeFocused();
  await page
    .getByRole("button", {
      name: "Comment on Assistant explanation",
      exact: true,
    })
    .click();
  await expect(draft()).toHaveText("Pointer draft");
  await page
    .locator('[data-placement="inline"]')
    .getByRole("button", { name: "Remove pending comment" })
    .click();
  await expect(source).toBeFocused();

  // Native navigation and selection, deletion and undo cannot change captured text.
  await page.keyboard.press(start);
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press(lineStart);
  await page.keyboard.press(selectLine);
  const selected = await page.evaluate(() => window.getSelection().toString());
  expect(selected).toContain("stable identities");
  await page.keyboard.press("Backspace");
  await page.keyboard.press(mac ? "Meta+z" : "Control+z");
  await expect.poll(capturedText).toEqual(original);
  await page.keyboard.press(start);
  await page.keyboard.press(selectLine);
  await page.keyboard.press("Meta+Alt+m");
  await page.keyboard.type("Keyboard draft");
  await expect(quote()).toHaveText("A small change");
  await expect(quote()).toHaveAttribute("data-chat-quote", "## A small change");
  await page.keyboard.press("Control+Enter");
  await expect(
    page
      .locator('[data-placement="inline"]')
      .getByRole("article", { name: "Comment by You" })
      .filter({ hasText: "Keyboard draft" }),
  ).toHaveText(/Keyboard draft/);
  await expect.poll(capturedText).toEqual(original);

  // First native character transfers once and leaves the next character after it.
  await source.focus();
  await page.keyboard.type("First input");
  await expect(draft()).toHaveText("First input");
  await page.keyboard.press("Enter");
  await page.keyboard.type("Second line");
  await expect(draft().locator(":scope > .cm-line")).toHaveText([
    "First input",
    "Second line",
  ]);
  await page.keyboard.press("Escape");
  await expect(source).toBeFocused();

  // Custom rendered views use a whole-section target and return focus.
  const custom = page.getByRole("group", {
    name: "Decision table",
    exact: true,
  });
  await custom.focus();
  await custom.getByRole("button", { name: "Inspect decision" }).click();
  await expect(
    custom.getByRole("button", { name: "Inspect decision" }),
  ).toHaveAttribute("aria-pressed", "true");
  await expect(composer()).toHaveCount(0);
  await custom.focus();
  await page.keyboard.type("Whole view");
  await expect(quote()).toHaveText(/\| Topic \| Choice \|/);
  await expect(draft()).toHaveText("Whole view");
  await page.keyboard.press("Escape");
  await expect(custom).toBeFocused();
}
