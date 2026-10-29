import { expect, userEvent, within } from "storybook/test";

export async function agentToolDurationPlay({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await userEvent.click(
    await c.findByRole("button", { name: /Open Codex ended/ }),
  );

  await userEvent.click(
    await c.findByRole("button", {
      name: /^Expand Read(?: .+)? 3 tool calls$/,
    }),
  );
  for (const [name, duration, status] of [
    ["apply_patch", "1.25s", "unknown"],
    ["Bash", "250ms", "error"],
    ["Read", "0ms", "complete"],
  ]) {
    const trigger = c.getByRole("button", {
      name: new RegExp(`^Show details for ${name}(?: |$)`),
    });
    const timing = within(trigger).getByText(duration!, { exact: true });
    const chevron = trigger.querySelector('[data-ui-part="detail-chevron"]')!;
    await expect(
      timing.nextElementSibling?.getAttribute("data-ui-part") ??
        timing.parentElement?.nextElementSibling?.getAttribute("data-ui-part"),
    ).toBe("detail-chevron");
    await expect(timing.getBoundingClientRect().right).toBeLessThanOrEqual(
      chevron.getBoundingClientRect().left,
    );
    await expect(
      trigger.querySelector('[data-ui-part="call-status"]'),
    ).toHaveAttribute("data-status", status!);
  }
  await expect(
    c
      .getByRole("button", { name: /^Show details for Search(?: |$)/ })
      .querySelector('[data-ui-part="call-stats"]'),
  ).toBeNull();
}
