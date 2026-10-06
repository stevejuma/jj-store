import { expect, userEvent, waitFor, within } from "storybook/test";
export async function rail(element: HTMLElement) {
  const canvas = within(element);
  await canvas.findByRole("region", { name: "Unstaged files" });
  await userEvent.click(canvas.getByRole("button", { name: "Toggle details" }));
  return within(
    canvas.getByRole("complementary", { name: "Repository details" }),
  );
}
export async function closeEditor(element: HTMLElement) {
  const portal = within(element.ownerDocument.body);
  const dialog = await portal.findByRole("region", {
    name: "Resolve workspace conflicts",
  });
  expect(dialog).toHaveTextContent("src/settings.ts");
  await userEvent.click(
    within(dialog).getByRole("button", { name: "Close conflict editor" }),
  );
  await waitFor(() => expect(dialog).not.toBeInTheDocument());
}

export async function loadingConflict({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const canvas = within(canvasElement);
  const page = within(canvasElement.ownerDocument.body);
  await canvas.findByRole("region", { name: "Unstaged files" });
  await userEvent.click(
    canvas.getByRole("button", { name: "Resolve conflicts" }),
  );
  const surface = await page.findByRole("region", {
    name: "Resolve workspace conflicts",
  });
  const view = within(surface);
  const skeleton = view.getByRole("status", {
    name: "Loading conflict file",
  });
  expect(skeleton).toHaveAttribute("aria-busy", "true");
  expect(
    skeleton.querySelectorAll('[data-ui-component="skeleton"]').length,
  ).toBeGreaterThan(12);
  expect(view.queryByText("Loading conflict…")).toBeNull();
  expect(view.queryByRole("textbox", { name: "Edit Resolved" })).toBeNull();
  expect(
    view.getByRole("button", { name: "Stage merged file" }),
  ).toBeDisabled();
  await userEvent.click(
    canvas.getByRole("button", { name: "Complete conflict read" }),
  );
  await view.findByRole("textbox", { name: "Edit Resolved" });
  expect(
    view.queryByRole("status", { name: "Loading conflict file" }),
  ).toBeNull();
  expect(view.getByRole("button", { name: "Stage merged file" })).toBeEnabled();
}
