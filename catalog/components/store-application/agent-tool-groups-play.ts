import { userEvent, within } from "storybook/test";
export const openSession = async (canvasElement: HTMLElement) => {
  const c = within(canvasElement);
  await userEvent.click(
    await c.findByRole("button", { name: /Open Codex ended/ }),
  );
  await c.findByRole(
    "button",
    { name: /^Expand Read(?: .+)? 3 tool calls$/ },
    { timeout: 10_000 },
  );
  return c;
};
