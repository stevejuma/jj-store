import { expect, userEvent, waitFor, within } from "storybook/test";
export const retainedDetails = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const canvas = within(canvasElement);
  await userEvent.click(await canvas.findByRole("tab", { name: "Stacks" }));
  await canvas.findByRole("region", { name: "Local stack workspace" });
  const rail = canvasElement.querySelector<HTMLElement>(
    '[data-ui-part="sidebar"][aria-label="Repository details"]',
  )!;
  const assertClosed = () => {
    expect(
      canvas.getByRole("button", { name: "Toggle details" }),
    ).toHaveAttribute("aria-expanded", "false");
    expect(rail).toHaveAttribute("data-state", "closed");
    expect(rail.hidden).toBe(true);
    expect(rail.getBoundingClientRect().width).toBe(0);
  };
  await waitFor(assertClosed);
  await userEvent.click(
    canvas.getByRole("button", { name: "Repository settings" }),
  );
  await canvas.findByRole("navigation", {
    name: "Repository settings navigation",
  });
  await userEvent.click(
    canvas.getByRole("button", { name: "Back to repository" }),
  );
  await canvas.findByRole("region", { name: "Local stack workspace" });
  await waitFor(assertClosed);
  const toggle = canvas.getByRole("button", { name: "Toggle details" });
  await userEvent.click(toggle);
  await canvas.findByRole("navigation", { name: "Stack workspace views" });
  await waitFor(() =>
    expect(rail.getBoundingClientRect().width).toBeGreaterThan(0),
  );
  await userEvent.click(toggle);
  await waitFor(assertClosed);
};
