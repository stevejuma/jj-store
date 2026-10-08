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
