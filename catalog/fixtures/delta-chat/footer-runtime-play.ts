import { expect, userEvent, waitFor, within } from "storybook/test";
import { reader } from "./acceptance-helpers.js";

export async function footerRuntime({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement),
    page = within(canvasElement.ownerDocument.body);
  await reader(canvasElement);
  const model = c.getByRole("button", { name: "Model: 6.1 Sol" });
  const thinking = c.getByRole("button", { name: "Thinking: Medium" });
  const agent = c.getByRole("button", { name: "Agent: Codex" });
  const send = c.getByRole("button", { name: "Send message" });
  expect(send.textContent?.trim()).toBe("");
  expect(
    canvasElement
      .querySelector(".chat-message-avatar img")
      ?.getAttribute("src"),
  ).toBe("/agent-icons/codex.svg");
  await userEvent.click(c.getByRole("button", { name: "Jump to end" }));
  const input = c.getByRole("textbox", { name: "Next message" });
  await userEvent.keyboard("Preserve this draft");
  await userEvent.click(thinking);
  await menuFits(14);
  await userEvent.click(page.getByRole("menuitemradio", { name: "High" }));
  expect(c.getByRole("button", { name: "Thinking: High" })).toHaveFocus();
  await userEvent.click(model);
  await menuFits(18);
  await userEvent.click(page.getByRole("menuitemradio", { name: "6 Astra" }));
  expect(c.getByRole("button", { name: "Model: 6 Astra" })).toHaveFocus();
  await userEvent.click(agent);
  await menuFits(18);
  await userEvent.click(page.getByRole("menuitemradio", { name: "OpenCode" }));
  expect(c.getByRole("button", { name: "Agent: OpenCode" })).toHaveFocus();
  expect(
    canvasElement
      .querySelector(".chat-message-avatar img")
      ?.getAttribute("src"),
  ).toBe("/agent-icons/codex.svg");
  expect(input).toHaveTextContent("Preserve this draft");
  await userEvent.click(send);
  expect(await reader(canvasElement, "Local message")).toHaveTextContent(
    "Preserve this draft",
  );
  await expect(send).toBeDisabled();

  async function menuFits(rem: number) {
    await waitFor(() => {
      const doc = canvasElement.ownerDocument;
      const rootSize = parseFloat(
        getComputedStyle(doc.documentElement).fontSize,
      );
      const viewport = doc.documentElement.clientWidth;
      const menu = page.getByRole("menu");
      const rect = menu.getBoundingClientRect();
      expect(rect.width).toBeGreaterThanOrEqual(
        Math.min(rem * rootSize, viewport - 2 * rootSize) - 1,
      );
      expect(rect.left).toBeGreaterThanOrEqual(0);
      expect(rect.right).toBeLessThanOrEqual(viewport);
      expect(menu.scrollWidth).toBeLessThanOrEqual(menu.clientWidth);
    });
  }
}
