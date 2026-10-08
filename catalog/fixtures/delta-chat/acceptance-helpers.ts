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
  const composer = await within(canvasElement).findByRole("group", {
    name: `Comment on ${label}`,
  });
  const editor = await within(composer).findByRole("textbox", {
    name: "Markdown editor",
  });
  await waitFor(() => expect(editor).toHaveFocus());
  return editor;
}
export async function addComment(canvasElement: HTMLElement) {
  const c = within(canvasElement);
  await userEvent.click(c.getByRole("button", { name: "Add comment" }));
  await waitFor(() =>
    expect(
      c.queryByRole("form", { name: "Inline comment composer" }),
    ).toBeNull(),
  );
}
export function expectAdjacentChevron(header: HTMLElement) {
  const label = header.querySelector("span:not(.sr-only)")!;
  const chevron = header.querySelector("svg:last-child")!;
  const gap =
    chevron.getBoundingClientRect().left - label.getBoundingClientRect().right;
  expect(gap).toBeGreaterThanOrEqual(7);
  expect(gap).toBeLessThanOrEqual(9);
}
