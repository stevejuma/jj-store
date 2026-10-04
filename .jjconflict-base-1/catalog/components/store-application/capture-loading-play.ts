import { expect, userEvent, waitFor, within } from "storybook/test";

export async function loadingCaptureSettings({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  const pause = c.getByRole("button", { name: "Pause capture" });
  await expect(pause).toBeVisible();
  await expect(pause).toBeDisabled();
  await expect(
    c.getByRole("button", { name: "Rescan capture" }),
  ).toBeDisabled();
  await expect(
    c.getByRole("status", { name: "Loading capture status" }),
  ).toBeVisible();
  await expect(
    canvasElement.querySelector('[data-ui-component="skeleton"]'),
  ).not.toBeNull();
  await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
  await expect(
    c.getByRole("status", { name: "Loading capture configuration" }),
  ).toBeVisible();
  await userEvent.click(c.getByRole("tab", { name: "Sources" }));
  await expect(
    c.getByRole("status", { name: "Loading capture sources" }),
  ).toBeVisible();
  await expect(
    c.queryByText("Refresh capture to select current source coverage."),
  ).toBeNull();
  await userEvent.click(c.getByRole("button", { name: "Finish capture load" }));
  await expect(await c.findByText("Codex CLI source 1")).toBeVisible();
  await userEvent.click(c.getByRole("tab", { name: "Capture" }));
  await expect(c.getByRole("button", { name: "Pause capture" })).toBeEnabled();
  await userEvent.click(
    c.getByRole("button", { name: "Delay capture refresh" }),
  );
  await expect(c.getByText("/Users/capture/.codex/sessions")).toBeVisible();
  await expect(c.getByRole("button", { name: "Pause capture" })).toBeDisabled();
  await expect(
    canvasElement.querySelector('[data-ui-component="skeleton"]'),
  ).toBeNull();
  await userEvent.click(c.getByRole("button", { name: "Finish capture load" }));
  await waitFor(() =>
    expect(c.getByRole("button", { name: "Pause capture" })).toBeEnabled(),
  );
}

export async function busyCaptureSettings({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await expect(
    await c.findByRole("status", { name: "Loading capture status" }),
  ).toBeVisible();
  await expect(c.getByRole("button", { name: "Pause capture" })).toBeDisabled();
  await expect(
    within(canvasElement.ownerDocument.body).queryByRole("button", {
      name: "Dismiss Capture request unavailable",
    }),
  ).toBeNull();
  await userEvent.click(
    c.getByRole("button", { name: "Allow capture observations" }),
  );
  await waitFor(() =>
    expect(c.getByRole("button", { name: "Pause capture" })).toBeEnabled(),
  );
}
