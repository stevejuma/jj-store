import { expect, userEvent, waitFor, within } from "storybook/test";
export async function reader(
  canvasElement: HTMLElement,
  label = "Assistant explanation",
) {
  return within(canvasElement).findByRole("textbox", { name: label });
}
export async function draft(
  canvasElement: HTMLElement,
  label = "Assistant explanation",
) {
  let editor: HTMLElement | undefined;
  await waitFor(() => {
    editor = Array.from(
      canvasElement.querySelectorAll<HTMLElement>(
        '[data-location="inline"] [role="textbox"]',
      ),
    ).find((node) => node.getAttribute("aria-label") === `Comment on ${label}`);
    expect(editor).toBeDefined();
    expect(editor).toHaveFocus();
  });
  return editor!;
}
export async function addComment(canvasElement: HTMLElement) {
  await userEvent.click(
    within(canvasElement).getByRole("button", { name: "Send message" }),
  );
  await waitFor(() =>
    expect(canvasElement.querySelector("[data-comment-editor]")).toBeNull(),
  );
}
export function inlineComments(canvasElement: HTMLElement) {
  return within(canvasElement.querySelector(".chat-document")! as HTMLElement);
}
export function expectAdjacentChevron(header: HTMLElement) {
  const label = header.querySelector("span:not(.sr-only)")!;
  const chevron = header.querySelector("svg:last-child")!;
  const gap =
    chevron.getBoundingClientRect().left - label.getBoundingClientRect().right;
  expect(gap).toBeGreaterThanOrEqual(7);
  expect(gap).toBeLessThanOrEqual(9);
}
