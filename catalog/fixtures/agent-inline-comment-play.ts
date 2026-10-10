import { expect, userEvent, waitFor, within } from "storybook/test";
import type { InlineAgentCommand } from "./agent-inline-comment-transport.js";

export async function openInlineConversation(canvas: HTMLElement) {
  const c = within(canvas);
  await userEvent.click(
    c.getByRole("button", { name: "Open fixture conversation" }),
  );
  await c.findAllByRole("textbox", { name: "Assistant message" });
  return c;
}
export function inlineEditors(
  canvas: HTMLElement,
  label = "Assistant message",
) {
  return [
    ...canvas.querySelectorAll<HTMLElement>(
      '[data-location="inline"] [role="textbox"]',
    ),
  ].filter((node) => node.getAttribute("aria-label") === `Comment on ${label}`);
}
export async function commentEditor(
  canvas: HTMLElement,
  label = "Assistant message",
) {
  let editor: HTMLElement;
  await waitFor(() => {
    editor = inlineEditors(canvas, label).find(
      (node) => node === document.activeElement,
    )!;
    expect(editor).toBeDefined();
  });
  return editor!;
}
export async function startInlineComment(canvas: HTMLElement) {
  const c = within(canvas);
  const source = c.getAllByRole("textbox", { name: "Assistant message" })[0];
  await userEvent.click(source);
  await userEvent.click(
    c.getAllByRole("button", { name: "Comment on Assistant message" })[0],
  );
  return commentEditor(canvas);
}
export function commentCommands(canvas: HTMLElement) {
  return JSON.parse(
    canvas.querySelector('[data-testid="comment-commands"]')!.textContent!,
  ) as InlineAgentCommand[];
}
export function promptCommands(canvas: HTMLElement) {
  return commentCommands(canvas).filter(
    (command) => command.action.action === "prompt",
  );
}
export async function sendInlineComments(canvas: HTMLElement) {
  const c = within(canvas);
  await userEvent.click(c.getByRole("button", { name: "Send message" }));
  await waitFor(() =>
    expect(canvas.querySelector("[data-comment-editor]")).toBeNull(),
  );
  await userEvent.click(
    c.getByRole("button", { name: "Refresh captured feedback" }),
  );
  await waitFor(() =>
    expect(
      c.getAllByRole("button", { name: "Edit comment" }).length,
    ).toBeGreaterThan(0),
  );
}
