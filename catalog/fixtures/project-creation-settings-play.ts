import { expect, within } from "storybook/test";

export async function expectProjectCreationSettings(
  dialogElement: HTMLElement,
) {
  const dialog = within(dialogElement),
    title = dialog.getByLabelText("Project title"),
    card = title.closest<HTMLElement>(".store-settings-card")!;
  await expect(card).not.toBeNull();
  await expect(
    within(card).getByRole("heading", { name: "Project details" }),
  ).toBeVisible();
  await expect(card.querySelector(".store-settings-icon svg")).not.toBeNull();
  const label = title
    .closest('[data-slot="field"]')!
    .querySelector('[data-ui-part="field-label"]')!;
  const mobile = matchMedia("(max-width: 40rem)").matches;
  if (mobile)
    await expect(label.getBoundingClientRect().bottom).toBeLessThan(
      title.getBoundingClientRect().top,
    );
  else
    await expect(label.getBoundingClientRect().right).toBeLessThan(
      title.getBoundingClientRect().left,
    );
  const create = dialog.getByRole("button", { name: "Create project" });
  await expect(
    create.closest('[data-ui-part="dialog-frame-footer"]'),
  ).not.toBeNull();
  await expect(create).toBeDisabled();
  const description = dialog.getByRole("group", {
    name: "Project description",
  });
  if (!mobile)
    await expect(description.getBoundingClientRect().width).toBeGreaterThan(
      title.getBoundingClientRect().width,
    );
  await expect(dialogElement.scrollWidth).toBeLessThanOrEqual(
    dialogElement.clientWidth + 1,
  );
}
