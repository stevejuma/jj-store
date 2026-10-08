import { expect, userEvent, waitFor, within } from "storybook/test";
export async function nextMessagePlay({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement),
    draft = await c.findByRole("textbox", { name: "Next message" });
  const send = c.getByRole("button", { name: "Send message" });
  await expect(send).toBeDisabled();
  await userEvent.click(c.getByRole("button", { name: "Jump to end" }));
  await expect(draft).toHaveFocus();
  await userEvent.keyboard("First line{Enter}Second line");
  await waitFor(() =>
    expect(draft.querySelectorAll(".cm-line")).toHaveLength(2),
  );
  await expect(c.queryByRole("textbox", { name: "Local message" })).toBeNull();
  await expect(send).toBeEnabled();
  const footer = c
    .getByRole("group", { name: "Message actions" })
    .getBoundingClientRect().top;
  const body = c.getByRole("region", { name: "Agent chat body" });
  body.scrollTop = 0;
  expect(
    c.getByRole("group", { name: "Message actions" }).getBoundingClientRect()
      .top,
  ).toBe(footer);
  await userEvent.click(send);
  await expect(
    await c.findByRole("textbox", { name: "Local message" }),
  ).toHaveTextContent("Second line");
  await waitFor(() => expect(send).toBeDisabled());
  await expect(draft).toHaveFocus();
  await userEvent.keyboard("Another message");
  await userEvent.keyboard(
    navigator.platform.includes("Mac")
      ? "{Meta>}{Enter}{/Meta}"
      : "{Control>}{Enter}{/Control}",
  );
  await waitFor(() =>
    expect(c.getAllByRole("textbox", { name: "Local message" })).toHaveLength(
      2,
    ),
  );
}
