import { expect, userEvent, within, waitFor } from "storybook/test";

export async function codexConfigurationGroup({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
  const group = await c.findByRole("button", { name: "Codex configuration" });
  for (const name of ["Codex", "Cursor", "OpenCode"])
    await expect(
      c.getByRole("button", { name: `${name} configuration` }),
    ).toHaveAttribute("aria-expanded", "false");
  await expect(
    c.queryByRole("textbox", { name: "Codex CLI folders" }),
  ).toBeNull();
  await userEvent.click(group);
  await expect(group).toHaveAttribute("aria-expanded", "true");
  await expect(
    group.querySelector('img[src="/agent-icons/codex.svg"]'),
  ).not.toBeNull();
  const roots = c.getByRole("textbox", { name: "Codex CLI folders" });
  await userEvent.type(roots, "/tmp/grouped/sessions");
  await userEvent.click(group);
  await expect(group).toHaveAttribute("aria-expanded", "false");
  await waitFor(() =>
    expect(c.queryByRole("textbox", { name: "Codex CLI folders" })).toBeNull(),
  );
  await expect(
    c.getByRole("spinbutton", { name: "Concurrent readers" }),
  ).toBeVisible();
  await expect(
    c.getByRole("switch", { name: "Enable automatic capture" }),
  ).toBeVisible();
  group.focus();
  await userEvent.keyboard("{Enter}");
  await expect(
    c.getByRole("textbox", { name: "Codex CLI folders" }),
  ).toHaveValue("/tmp/grouped/sessions");
  await userEvent.click(c.getByRole("button", { name: "Cancel changes" }));
  await expect(
    c.getByRole("textbox", { name: "Codex CLI folders" }),
  ).toHaveValue("");
}

export async function filteredCaptureSources({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  const page = within(canvasElement.ownerDocument.body);
  await userEvent.click(c.getByRole("tab", { name: "Sources" }));
  const table = await c.findByRole("table", { name: "Capture sources" });
  await expect(table.querySelectorAll("tbody > tr")).toHaveLength(3);
  await expect(c.getByText("3 loaded")).toBeVisible();
  await expect(c.getByText("5 observed")).toBeVisible();
  const filter = c.getByRole("button", { name: "Filter by harness" });
  await waitFor(() =>
    expect(getComputedStyle(filter).pointerEvents).not.toBe("none"),
  );
  await userEvent.click(filter);
  await userEvent.click(page.getByRole("option", { name: "Codex" }));
  await expect(table.querySelectorAll("tbody > tr")).toHaveLength(2);
  await expect(c.getByText("Codex Desktop source 3")).toBeVisible();
  await expect(c.queryByText("Cursor Desktop source 2")).toBeNull();
  await expect(c.getByText("2 shown")).toBeVisible();
  await waitFor(() =>
    expect(getComputedStyle(filter).pointerEvents).not.toBe("none"),
  );
  await userEvent.click(filter);
  await userEvent.click(page.getByRole("option", { name: "OpenCode" }));
  await expect(c.getByText("No sources match these filters")).toBeVisible();
  await expect(c.getByText("5 observed")).toBeVisible();
  await waitFor(() =>
    expect(getComputedStyle(filter).pointerEvents).not.toBe("none"),
  );
  await userEvent.click(filter);
  await userEvent.click(page.getByRole("option", { name: "Cursor" }));
  await expect(
    c.getByRole("button", {
      name: "Import history for Cursor Desktop source 2",
    }),
  ).toBeDisabled();
  await waitFor(() =>
    expect(getComputedStyle(filter).pointerEvents).not.toBe("none"),
  );
  await userEvent.click(filter);
  await userEvent.click(page.getByRole("option", { name: "Codex" }));
  const history = c.getByRole("button", {
    name: "Import history for Codex Desktop source 3",
  });
  await waitFor(() =>
    expect(getComputedStyle(history).pointerEvents).not.toBe("none"),
  );
  await userEvent.click(history);
  await expect(c.getByLabelText("Accepted source selector")).toHaveTextContent(
    "ab".repeat(16) + ":0000000000000003",
  );
}

export const captureSettingsLayout = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement);
  await userEvent.click(c.getByRole("tab", { name: "Sources" }));
  await expect(await c.findByText("Codex CLI source 1")).toBeVisible();
  await expect(
    c.getByRole("heading", { name: "Agents", level: 1 }),
  ).toBeVisible();
  const refresh = c.getByRole("button", { name: "Refresh capture" });
  await expect(
    c.getByRole("heading", { name: "Agents", level: 1 }).parentElement,
  ).toContainElement(refresh);
  await userEvent.click(c.getByRole("tab", { name: "Capture" }));
  const pause = c.getByRole("button", { name: "Pause capture" });
  const rescan = c.getByRole("button", { name: "Rescan capture" });
  const actions = pause.closest(".store-settings__actions")!;
  await expect(actions.parentElement).toHaveClass("store-settings-footer");
  await expect(getComputedStyle(actions).flexDirection).toBe("row");
  await expect(getComputedStyle(actions).flexWrap).toBe("wrap");
  await expect(pause.closest(".store-settings-card")).toBe(
    rescan.closest(".store-settings-card"),
  );
  await userEvent.click(c.getByRole("tab", { name: "Sources" }));
  const sourceAction = c.getByRole("button", { name: /Import history for/ });
  await expect(
    sourceAction.closest(".store-capture-sources__table"),
  ).toHaveClass("store-capture-sources__table");
  await userEvent.click(c.getByRole("tab", { name: "Imports" }));
  const request = c.getByRole("textbox", {
    name: "Previous import request ID",
  });
  await expect(request.closest(".store-settings__rows")).not.toBeNull();
};
