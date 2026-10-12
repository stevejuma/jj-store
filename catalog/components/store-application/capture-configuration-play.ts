import { expandCodex } from "./capture-panel-play.js";
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
  await expect(
    c.getByText("Enabled · Configuration reload required"),
  ).toBeVisible();
}

export async function captureOptionSliders({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
  const fields = [
    ["Concurrent readers", 4, 2],
    ["Discovery interval (seconds)", 30, 2],
    ["Tracked source limit", 4096, 2],
    ["Pending byte budget", 67108864, 1048576],
  ] as const;
  await expect(
    await c.findByRole("slider", { name: "Concurrent readers slider" }),
  ).toBeVisible();
  const save = c.getByRole("button", { name: "Save configuration" });
  await expect(save).toBeDisabled();
  for (const [label, max, next] of fields) {
    const slider = c.getByRole("slider", { name: `${label} slider` });
    await expect(slider).toHaveAttribute("aria-valuemin", "1");
    await expect(slider).toHaveAttribute("aria-valuemax", String(max));
    slider.focus();
    await userEvent.keyboard("{Home}{ArrowRight}");
    await expect(c.getByRole("spinbutton", { name: label })).toHaveValue(next);
    await expect(slider).toHaveAttribute("aria-valuenow", String(next));
  }
  await expect(save).toBeEnabled();
  await userEvent.click(c.getByRole("tab", { name: "Diagnostics" }));
  await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
  for (const [label, , next] of fields)
    await expect(c.getByRole("spinbutton", { name: label })).toHaveValue(next);
  await userEvent.click(c.getByRole("button", { name: "Save configuration" }));
  await expect(
    c.getByRole("button", { name: "Save configuration" }),
  ).toBeDisabled();
  await expect(
    c.getByText("Enabled · Configuration reload required"),
  ).toBeVisible();
}

export async function disabledCaptureConfiguration({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
  const enabled = await c.findByRole("switch", {
    name: "Enable automatic capture",
  });
  await userEvent.click(enabled);
  await expandCodex(c);
  const roots = c.getByRole("textbox", { name: "Codex CLI folders" });
  await userEvent.type(roots, "/tmp/agent-settings/sessions");
  await userEvent.click(c.getByRole("tab", { name: "Diagnostics" }));
  await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
  await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
  await expandCodex(c);
  await expect(
    c.getByRole("textbox", { name: "Codex CLI folders" }),
  ).toHaveValue("/tmp/agent-settings/sessions");
  await userEvent.click(c.getByRole("button", { name: "Save configuration" }));
  await expect(
    c.getByText("Disabled · Configuration reload required"),
  ).toBeVisible();
  await expect(
    c.getByRole("button", { name: "Save configuration" }),
  ).toBeDisabled();
  const actions = c
    .getByRole("button", { name: "Cancel changes" })
    .closest(".store-settings__actions")!;
  await expect(getComputedStyle(actions).flexDirection).toBe("row");
}
