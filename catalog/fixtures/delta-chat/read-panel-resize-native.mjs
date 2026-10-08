import { expect } from "@playwright/test";

export async function readPanelResizeAcceptance(page, base) {
  await page.setViewportSize({ width: 1100, height: 760 });
  await page.goto(
    `${base}/iframe.html?id=jj-store-commentable-agent-chat-body--interaction-playground&viewMode=story&embed=true`,
  );
  await page
    .getByRole("button", { name: "Expand Read a file, ran a command" })
    .click();
  const opener = page.getByRole("button", {
    name: "Show details for Read example.ts",
  });
  await opener.click();
  const pane = page.getByRole("complementary", { name: "Read tool details" });
  const handle = page.getByRole("slider", {
    name: "Resize read tool details",
  });
  const width = async () => (await pane.boundingBox()).width;
  await expect(handle).toBeVisible();
  await expect(pane.getByText("Captured read", { exact: true })).toHaveCount(0);
  await expect(page.locator(".chat-body-fixture-header")).toHaveCSS(
    "border-bottom-width",
    "0px",
  );
  await pane
    .getByRole("button", { name: "Comment on Captured example.ts" })
    .click();
  const draft = pane.getByRole("textbox", {
    name: "Comment on Captured example.ts",
  });
  await expect(draft).toBeFocused();
  await page.keyboard.type("Retain my read comment");
  const editor = await draft.elementHandle();
  const initial = await width();
  const mainBounds = await page.locator('[data-ui-part="main"]').boundingBox();
  const paneBounds = await pane.boundingBox();
  expect(paneBounds.y).toBe(mainBounds.y);
  expect(paneBounds.height).toBe(mainBounds.height);
  expect(paneBounds.x).toBe(mainBounds.x + mainBounds.width);
  const footer = page.getByRole("group", { name: "Message actions" });
  const footerBefore = await footer.boundingBox();
  async function dragBy(distance) {
    const rect = await handle.boundingBox();
    const x = rect.x + rect.width / 2,
      y = rect.y + rect.height / 2;
    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x + distance, y, { steps: 6 });
    await page.mouse.up();
    await expect(pane).not.toHaveAttribute("data-resizing");
  }
  await dragBy(-120);
  await expect.poll(width).toBe(initial + 120);
  await expect(draft).toBeFocused();
  expect(await editor.evaluate((node) => node.isConnected)).toBe(true);
  await expect(draft).toHaveText("Retain my read comment");
  expect((await footer.boundingBox()).y).toBe(footerBefore.y);
  expect((await footer.boundingBox()).height).toBe(footerBefore.height);
  await dragBy(800);
  await expect.poll(width).toBe(280);
  await dragBy(-1000);
  await expect.poll(width).toBe(720);
  await handle.focus();
  await page.keyboard.press("Home");
  await expect.poll(width).toBe(280);
  await dragBy(-80);
  expect(await width()).toBe(360);
  await page.setViewportSize({ width: 700, height: 760 });
  // A constrained desktop uses the same replace-main presentation as replies.
  await expect(pane).toHaveAttribute("data-presentation", "replace-main");
  await expect(handle).toHaveCount(0);
  await page.setViewportSize({ width: 1100, height: 760 });
  await expect(handle).toBeVisible();
  await handle.focus();
  await page.keyboard.press("Home");
  await page.keyboard.press("ArrowLeft");
  await expect.poll(width).toBe(296);
  await page.setViewportSize({ width: 390, height: 700 });
  await expect(handle).toHaveCount(0);
  await opener.click();
  await expect(pane).toBeVisible();
  expect(await width()).toBeLessThanOrEqual(390);
  expect(await width()).toBeGreaterThan(300);
  await expect(draft).toHaveText("Retain my read comment");
  await page.setViewportSize({ width: 1100, height: 760 });
  await expect(handle).toBeVisible();
  await expect.poll(width).toBe(296);
  await pane.getByRole("button", { name: "Close read tool details" }).click();
  await expect(opener).toBeFocused();
  await opener.click();
  await expect.poll(width).toBe(296);
  await expect(draft).toHaveText("Retain my read comment");
  await pane
    .getByRole("button", {
      name: "Show raw output in read sidebar for Read example.ts",
    })
    .click();
  await expect.poll(width).toBe(296);
  await pane
    .getByRole("button", {
      name: "Show rich output in read sidebar for Read example.ts",
    })
    .click();
  await expect(draft).toHaveText("Retain my read comment");
  expect(await page.evaluate(() => document.scrollingElement.scrollWidth)).toBe(
    1100,
  );
  expect(
    await page.evaluate(() => document.scrollingElement.scrollHeight),
  ).toBe(760);
  console.log(
    "PASS full-height read sidebar drag, bounds, editor identity/focus, draft retention, fixed footer, mobile and reopen",
  );
}
