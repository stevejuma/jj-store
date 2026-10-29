import { expect, waitFor, within } from "storybook/test";

export async function expectMobileApplicationToolbar(
  canvasElement: HTMLElement,
) {
  const canvas = within(canvasElement);
  await waitFor(() => {
    const navigation = canvas.getByRole("navigation", {
      name: "Application navigation",
    });
    const toolbar = within(navigation).getByRole("toolbar", {
      name: "Application controls",
    });
    expect(navigation).toHaveAttribute("data-ui-part", "topbar-target");
    expect(toolbar).toHaveAttribute("data-placement", "body");
    expect(
      canvasElement.querySelector(
        '[data-ui-part="topbar"][data-placement="top"]',
      ),
    ).toBeNull();
    expect(
      canvas.getAllByRole("button", { name: "Application menu" }),
    ).toHaveLength(1);
  });
}
