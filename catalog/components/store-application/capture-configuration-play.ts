import { expect, userEvent, within } from "storybook/test";
export async function unknownConfigurationSave({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
  await userEvent.click(
    await c.findByRole("switch", { name: "Enable automatic capture" }),
  );
  await userEvent.click(c.getByRole("button", { name: "Save configuration" }));
  await expect(c.getByText(/Save outcome unknown\. Refresh/)).toBeVisible();
  await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
  await expect(c.queryByText(/Save outcome unknown\. Refresh/)).toBeNull();
  await expect(c.getByText("Enabled · Restart required")).toBeVisible();
}
