import { expect } from "@playwright/test";

export async function footerRuntimeAcceptance(page, base) {
  await page.setViewportSize({ width: 1100, height: 760 });
  await page.goto(
    `${base}/iframe.html?id=jj-store-commentable-agent-chat-body--interaction-playground&viewMode=story&embed=true`,
  );
  await page.getByRole("button", { name: "Jump to end" }).click();
  await page.keyboard.type("Keep my runtime draft");
  const footer = page.getByRole("group", { name: "Message actions" });
  const input = page.getByRole("textbox", {
    name: "Next message",
    exact: true,
  });
  await footer.getByRole("button", { name: "Thinking: Medium" }).focus();
  await page.keyboard.press("Enter");
  await menuFits(14);
  await page.keyboard.press("End");
  await page.keyboard.press("Enter");
  await expect(
    footer.getByRole("button", { name: "Thinking: High" }),
  ).toBeFocused();
  await footer.getByRole("button", { name: "Model: 6.1 Sol" }).click();
  await menuFits(18);
  await page
    .getByRole("menuitemradio", { name: "6 Astra", exact: true })
    .click();
  await footer.getByRole("button", { name: "Agent: Codex" }).click();
  await menuFits(18);
  await page
    .getByRole("menuitemradio", { name: "OpenCode", exact: true })
    .click();
  await expect(input).toHaveText("Keep my runtime draft");
  await expect
    .poll(() =>
      footer
        .locator('img[src="/agent-icons/opencode.svg"]')
        .evaluate((img) => img.complete && img.naturalWidth > 0),
    )
    .toBe(true);
  await page.setViewportSize({ width: 390, height: 700 });
  await expect(
    footer.getByRole("button", { name: "Agent: OpenCode" }),
  ).toBeVisible();
  await expect(
    footer.getByRole("button", { name: "Model: 6 Astra" }),
  ).toBeVisible();
  expect(
    await footer.evaluate((node) => node.scrollWidth <= node.clientWidth),
  ).toBe(true);
  expect(await footer.locator(".chat-runtime-label").first().isVisible()).toBe(
    false,
  );
  for (const [label, rem] of [
    ["Agent: OpenCode", 18],
    ["Thinking: High", 14],
    ["Model: 6 Astra", 18],
  ]) {
    const trigger = footer.getByRole("button", { name: label });
    await trigger.click();
    await menuFits(rem);
    await page.keyboard.press("Escape");
    await expect(trigger).toBeFocused();
  }
  await expect(input).toHaveText("Keep my runtime draft");
  const send = footer.getByRole("button", { name: "Send message" });
  await expect(send).toHaveText("");
  await send.click();
  await expect(
    page.getByRole("textbox", { name: "Local message", exact: true }),
  ).toHaveText("Keep my runtime draft");
  await expect(send).toBeDisabled();
  console.log(
    "PASS footer keyboard menus, wide viewport-bounded popovers, harness asset, retained draft, compact layout and icon-only send",
  );

  async function menuFits(rem) {
    await expect
      .poll(() =>
        page
          .locator('[role="menu"][data-state="open"]')
          .evaluate((node, rem) => {
            const rootSize = parseFloat(
              getComputedStyle(document.documentElement).fontSize,
            );
            const viewport = document.documentElement.clientWidth;
            const rect = node.getBoundingClientRect();
            return (
              rect.width >=
                Math.min(rem * rootSize, viewport - 2 * rootSize) - 1 &&
              rect.left >= 0 &&
              rect.right <= viewport &&
              node.scrollWidth <= node.clientWidth
            );
          }, rem),
      )
      .toBe(true);
  }
}
