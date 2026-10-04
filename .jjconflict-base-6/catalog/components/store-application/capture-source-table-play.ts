import { expect, userEvent, within, waitFor } from "storybook/test";

export async function sourceTableIdentification({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement),
    page = within(canvasElement.ownerDocument.body);
  await userEvent.click(c.getByRole("tab", { name: "Sources" }));
  const table = await c.findByRole("table", { name: "Capture sources" });
  const filter = c.getByRole("button", { name: "Filter by harness" });
  const status = c.getByRole("button", { name: "Filter by source status" });
  await expect(filter.closest(".store-capture-sources__table")).toBeNull();
  await expect(status.closest(".store-capture-sources__table")).toBeNull();
  await expect(
    within(table).queryByRole("columnheader", { name: "Repository" }),
  ).toBeNull();
  const rows = table.querySelectorAll("tbody > tr");
  await expect(rows).toHaveLength(3);
  await expect(
    rows[0].querySelector('td:first-child img[src="/agent-icons/codex.svg"]'),
  ).not.toBeNull();
  await expect(
    rows[1].querySelector('td:first-child img[src="/agent-icons/cursor.svg"]'),
  ).not.toBeNull();
  const repository = within(rows[0] as HTMLElement).getByRole("link", {
    name: "Capture repository",
  });
  await expect(repository).toHaveAttribute(
    "href",
    `#/repos/${"ef".repeat(16)}/files`,
  );
  await expect(repository.closest("td")).toBe(
    rows[0].querySelector("td:first-child"),
  );
  await expect(repository.querySelector("svg")).not.toBeNull();
  await userEvent.click(repository);
  await expect(c.getByLabelText("Opened capture repository")).toHaveTextContent(
    "ef".repeat(16),
  );
  const history = c.getByRole("button", {
    name: "Import history for Codex CLI source 1",
  });
  await expect(history.textContent?.trim()).toBe("");
  await expect(history.querySelector("svg")).not.toBeNull();
  const footer = table.querySelector("tfoot td");
  await expect(footer).toHaveAttribute("colspan", "3");
  await expect(footer?.querySelector("time[datetime]")).not.toBeNull();
  await expect(
    within(footer as HTMLElement).getByRole("status", {
      name: "Source coverage status",
    }),
  ).toBeVisible();
  await userEvent.click(filter);
  await userEvent.click(page.getByRole("option", { name: "Codex" }));
  await waitFor(() =>
    expect(getComputedStyle(status).pointerEvents).not.toBe("none"),
  );
  await userEvent.click(status);
  await userEvent.click(page.getByRole("option", { name: "Degraded" }));
  await expect(table.querySelectorAll("tbody > tr")).toHaveLength(1);
  await expect(c.getByText("Codex Desktop source 3")).toBeVisible();
  await expect(c.queryByText("Codex CLI source 1")).toBeNull();
  await expect(c.getByText("1 shown")).toBeVisible();
  await expect(c.getByText("5 observed")).toBeVisible();
  await userEvent.keyboard("{Escape}");
}

export async function sourceRefreshSpinner({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await userEvent.click(c.getByRole("button", { name: "Finish capture load" }));
  await userEvent.click(c.getByRole("tab", { name: "Sources" }));
  const table = await c.findByRole("table", { name: "Capture sources" });
  await userEvent.click(
    c.getByRole("button", { name: "Delay capture refresh" }),
  );
  const footer = table.querySelector("tfoot")!;
  await expect(
    footer.querySelector('[data-ui-component="spinner"]'),
  ).not.toBeNull();
  await expect(table.querySelectorAll("tbody > tr")).toHaveLength(1);
  await expect(
    within(footer as HTMLElement).getByText("Refreshing sources…"),
  ).toBeVisible();
  await userEvent.click(c.getByRole("button", { name: "Finish capture load" }));
  await waitFor(() =>
    expect(footer.querySelector('[data-ui-component="spinner"]')).toBeNull(),
  );
}

export async function sourceRepositoryDraftGuard({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
  const roots = await c.findByRole("textbox", { name: "Codex CLI folders" });
  await userEvent.type(roots, "/tmp/retained-capture-draft");
  await userEvent.click(c.getByRole("tab", { name: "Sources" }));
  const link = c.getByRole("link", { name: "Capture repository" });
  await expect(link).toHaveAttribute("aria-disabled", "true");
  await expect(link).not.toHaveAttribute("href");
  await userEvent.click(link);
  await expect(c.queryByLabelText("Opened capture repository")).toBeNull();
  await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
  await expect(
    c.getByRole("textbox", { name: "Codex CLI folders" }),
  ).toHaveValue("/tmp/retained-capture-draft");
}
