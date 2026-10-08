import { expect, userEvent, waitFor, within } from "storybook/test";
export async function toolSpacing({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await userEvent.click(
    c.getByRole("button", { name: "Expand Read a file, ran a command" }),
  );
  const group = canvasElement.querySelector(".chat-tool-group-content")!;
  const calls = group.querySelectorAll(":scope > .chat-tool-item");
  const gap = (before: Element, after: Element) =>
    after.getBoundingClientRect().top - before.getBoundingClientRect().bottom;
  expect(gap(calls[0], calls[1])).toBe(8);
  await userEvent.click(
    c.getByRole("button", { name: "Show details for Run tests" }),
  );
  const output = c
    .getByRole("textbox", { name: "Captured test output" })
    .closest(".chat-text-block")!;
  const trigger = c.getByRole("button", { name: "Hide details for Run tests" });
  expect(gap(trigger.closest(".chat-tool-heading")!, output)).toBe(8);
  expect(gap(calls[0], calls[1])).toBe(8);
  const first = c
    .getByRole("button", { name: "Hide captured file example.ts" })
    .closest(".chat-tool-file")!;
  const next = c
    .getByRole("button", { name: "Show captured file example.test.ts" })
    .closest(".chat-tool-file")!;
  expect(gap(first, next)).toBe(8);
  await userEvent.click(
    c.getByRole("button", { name: "Hide captured file example.ts" }),
  );
  await waitFor(() => expect(gap(first, next)).toBe(8));
  const toolItem = trigger.closest("[data-chat-item]")!;
  expect(gap(toolItem, toolItem.nextElementSibling!)).toBe(8);
  const message = canvasElement.querySelector(
    '[data-kind="message"][data-chat-item]',
  )!;
  expect(gap(message, message.nextElementSibling!)).toBe(16);
  await userEvent.click(
    c.getByRole("button", { name: "Comment on Assistant explanation" }),
  );
  await userEvent.keyboard("Balanced spacing");
  await waitFor(() => {
    const cards = canvasElement.querySelectorAll(
      '.chat-comment-card[data-status="draft"]',
    );
    expect(cards).toHaveLength(2);
    for (const card of cards) {
      const bounds = card.getBoundingClientRect();
      const content = card
        .querySelector(".cm-content")!
        .getBoundingClientRect();
      expect(content.top - bounds.top).toBeCloseTo(
        bounds.bottom - content.bottom,
        0,
      );
    }
    const quote = canvasElement.querySelector(".chat-comment-quote")!;
    const text = quote.querySelector(".markdown-view__document")!;
    expect(text.getBoundingClientRect().left).toBe(
      quote.getBoundingClientRect().left,
    );
  });
}
