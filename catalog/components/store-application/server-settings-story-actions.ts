import { expect, userEvent, within } from "storybook/test";
export async function joinedTable({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  const save = await c.findByRole("button", { name: "Save settings" });
  if (c.queryByRole("button", { name: "Add upstream" }))
    await userEvent.click(c.getByRole("button", { name: "Add upstream" }));
  const form = save.closest("form")!;
  await expect(save.closest(".store-settings-footer")).not.toBeNull();
  await expect(save.parentElement!.lastElementChild).toBe(save);
  await expect(form.querySelectorAll('button[type="submit"]')).toHaveLength(1);
  await expect(canvasElement.querySelector("dl button")).toBeNull();
  const status =
    c.queryByRole("region", { name: "Configuration status" }) ??
    c.getByRole("region", { name: "Current status" });
  await expect(status).toBeVisible();
  const config =
    c.queryByRole("region", { name: "Resource limits" }) ??
    c.queryByRole("region", { name: "Protection rules" }) ??
    c.getByRole("region", { name: "Upstream configuration" });
  await expect(config).toBeVisible();
  await expect(status).not.toContainElement(save);
  const cancel = c.getByRole("button", { name: "Cancel changes" });
  await expect(cancel.getBoundingClientRect().right).toBeLessThanOrEqual(
    save.getBoundingClientRect().left,
  );
}
