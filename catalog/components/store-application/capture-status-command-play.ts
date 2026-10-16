import { expect, userEvent, within, waitFor } from "storybook/test";

export async function sourceStatusCommand({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement),
    page = within(canvasElement.ownerDocument.body);
  await userEvent.click(c.getByRole("tab", { name: "Sources" }));
  const table = await c.findByRole("table", { name: "Capture sources" });
  const status = c.getByRole("button", { name: "Filter by source status" });
  await userEvent.click(status);
  const search = await page.findByRole("combobox", {
    name: "Search source statuses",
  });
  await userEvent.type(search, "degraded");
  const degraded = page.getByRole("option", { name: "Degraded" });
  await userEvent.keyboard("{ArrowDown}{Enter}");
  await expect(degraded).toHaveAttribute("data-checked", "true");
  await expect(degraded).toHaveAccessibleDescription("Selected");
  await expect(search).toBeVisible();
  await waitFor(() =>
    expect(
      within(
        c
          .getByRole("list", { name: "Capture source counts" })
          .querySelector<HTMLElement>('[data-ui-part="row"]')!,
      ).getByText("1 shown"),
    ).toBeVisible(),
  );
  await userEvent.clear(search);
  await userEvent.click(
    page.getByRole("option", { name: "Awaiting activity" }),
  );
  await expect(table.querySelectorAll("tbody > tr")).toHaveLength(2);
  await expect(status).toHaveTextContent("2 statuses");
  await expect(c.getByText("Codex CLI source 1")).toBeVisible();
  await expect(c.getByText("Codex Desktop source 3")).toBeVisible();
  await expect(c.queryByText("Cursor Desktop source 2")).toBeNull();
  await waitFor(() =>
    expect(
      within(
        c
          .getByRole("list", { name: "Capture source counts" })
          .querySelector<HTMLElement>('[data-ui-part="row"]')!,
      ).getByText("3 loaded"),
    ).toBeVisible(),
  );
  await waitFor(() =>
    expect(
      within(
        c
          .getByRole("list", { name: "Capture source counts" })
          .querySelector<HTMLElement>('[data-ui-part="row"]')!,
      ).getByText("5 observed"),
    ).toBeVisible(),
  );
  const awaiting = page.getByRole("option", {
    name: "Awaiting activity",
  });
  await userEvent.click(awaiting);
  await expect(awaiting).toHaveAttribute("data-checked", "false");
  await waitFor(() =>
    expect(
      within(
        c
          .getByRole("list", { name: "Capture source counts" })
          .querySelector<HTMLElement>('[data-ui-part="row"]')!,
      ).getByText("1 shown"),
    ).toBeVisible(),
  );
  await userEvent.click(awaiting);
  await waitFor(() =>
    expect(
      within(
        c
          .getByRole("list", { name: "Capture source counts" })
          .querySelector<HTMLElement>('[data-ui-part="row"]')!,
      ).getByText("2 shown"),
    ).toBeVisible(),
  );
  await userEvent.type(search, "no-matching-status");
  await expect(page.getByText("No matching statuses")).toBeVisible();
  await userEvent.keyboard("{Escape}");
  await waitFor(() => expect(status).toHaveFocus());
  await userEvent.click(c.getByRole("button", { name: "Filter by harness" }));
  await userEvent.click(page.getByRole("option", { name: "Cursor" }));
  await expect(c.getByText("No sources match these filters")).toBeVisible();
  await waitFor(() =>
    expect(getComputedStyle(status).pointerEvents).not.toBe("none"),
  );
  await userEvent.click(status);
  const reopened = await page.findByRole("combobox", {
    name: "Search source statuses",
  });
  await expect(reopened).toHaveValue("");
  await expect(page.getByRole("option", { name: "Degraded" })).toHaveAttribute(
    "data-checked",
    "true",
  );
  await userEvent.click(page.getByRole("option", { name: "All statuses" }));
  await expect(status).toHaveTextContent("All statuses");
  await expect(c.getByText("Cursor Desktop source 2")).toBeVisible();
  await waitFor(() =>
    expect(
      within(
        c
          .getByRole("list", { name: "Capture source counts" })
          .querySelector<HTMLElement>('[data-ui-part="row"]')!,
      ).getByText("1 shown"),
    ).toBeVisible(),
  );
  await userEvent.keyboard("{Escape}");
}
