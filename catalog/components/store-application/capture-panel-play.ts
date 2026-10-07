import { expect, userEvent, within } from "storybook/test";

export function inCapturePanel(
  play: (context: { canvasElement: HTMLElement }) => Promise<void>,
) {
  return async (context: { canvasElement: HTMLElement }) => {
    const c = within(context.canvasElement);
    await expect(c.getByRole("tab", { name: "Runtimes" })).toHaveAttribute(
      "data-state",
      "active",
    );
    await userEvent.click(c.getByRole("tab", { name: "Capture" }));
    await play(context);
  };
}

export async function expandCodex(c: ReturnType<typeof within>) {
  const toggle = await c.findByRole("button", { name: "Codex configuration" });
  if (toggle.getAttribute("aria-expanded") === "false")
    await userEvent.click(toggle);
}
