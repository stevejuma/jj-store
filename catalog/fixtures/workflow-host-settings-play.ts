import { userEvent, within } from "storybook/test";

export async function admission(canvasElement: HTMLElement) {
  const canvas = within(canvasElement);
  await userEvent.click(
    await canvas.findByRole("button", {
      name: "Execution access",
    }),
  );
  const input = await canvas.findByRole("combobox", {
    name: "Admitted execution principals",
  });
  return { canvas, input };
}
export async function selectPrincipal(input: HTMLElement) {
  await userEvent.click(input);
  await userEvent.click(
    await within(document.body).findByRole("option", { name: /Steve/ }),
  );
  await userEvent.keyboard("{Escape}");
}
