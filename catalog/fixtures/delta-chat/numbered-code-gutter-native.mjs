import { expect } from "@playwright/test";

export async function numberedCodeGutterAcceptance(page) {
  const expand = page.getByRole("button", {
    name: "Expand Listed a directory, ran a command, read 7 files, ran a search, thought",
  });
  if (await expand.isVisible()) await expand.click();
  await page
    .getByRole("button", { name: "Show details for Read README.md" })
    .click();
  const pane = page.getByRole("complementary", { name: "Read tool details" });
  const reader = pane.getByRole("textbox", { name: "README.md", exact: true });
  await expect(reader).toBeVisible();
  const editor = pane.locator(".cm-editor").first();
  await expect(editor.locator('[data-captured-line="1"]')).toHaveText("1");
  await expect(reader.locator(".cm-line").first()).toHaveText(
    "# Bedtime for macOS",
  );
  const toolbar = page.locator('[data-ui-part="toolbar"]');
  const heading = pane.locator('[data-ui-part="sidebar-header"]');
  await expect(toolbar).toHaveCSS("border-bottom-width", "1px");
  const mainHeader = await toolbar.boundingBox(),
    sideHeader = await heading.boundingBox();
  expect(mainHeader.y + mainHeader.height).toBe(
    sideHeader.y + sideHeader.height,
  );
  const handle = page.getByRole("slider", { name: "Resize read tool details" });
  await handle.focus();
  await page.keyboard.press("Home");
  await expect(pane).toHaveCSS("width", "280px");
  const positions = await reader
    .locator(".cm-line")
    .nth(2)
    .evaluate((node) => {
      const range = document.createRange();
      range.selectNodeContents(node);
      return Array.from(range.getClientRects())
        .filter((rect) => rect.width > 0)
        .map((rect) => rect.left);
    });
  expect(positions.length).toBeGreaterThan(1);
  for (const x of positions) expect(x).toBeCloseTo(positions[0], 0);
  await expect(editor.locator('[data-captured-line="30"]')).toHaveCount(0);
  await pane
    .getByRole("button", { name: "Show 55 more lines in README.md" })
    .click();
  await expect(editor.locator('[data-captured-line="30"]')).toBeAttached();
  await handle.focus();
  await page.keyboard.press("End");
  await expect(pane).toHaveCSS("width", "720px");
  await page
    .getByRole("button", {
      name: "Show details for Read Sources/BedtimeCore/Schedule.swift",
    })
    .click();
  await expect(pane).toHaveCSS("width", "720px");
  await expect(pane.locator('[data-captured-line="1"]')).toHaveText("1");
  console.log(
    "PASS gutter labels, wrapped alignment, middle reveal, aligned header dividers and width retention across files",
  );
}
