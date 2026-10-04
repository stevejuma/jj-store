import { expect, userEvent, within, waitFor } from "storybook/test";

export async function sourceRepositoryCommand({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement),
    page = within(canvasElement.ownerDocument.body);
  await userEvent.click(c.getByRole("tab", { name: "Sources" }));
  const table = await c.findByRole("table", { name: "Capture sources" });
  const filter = c.getByRole("button", { name: "Filter by source repository" });
  await userEvent.click(filter);
  const search = await page.findByRole("combobox", {
    name: "Search source repositories",
  });
  await userEvent.type(search, "capture");
  await userEvent.keyboard("{ArrowDown}{Enter}");
  await expect(
    page.getByRole("option", { name: "Capture repository" }),
  ).toHaveAttribute("data-checked", "true");
  await expect(c.getByText("1 shown")).toBeVisible();
  await expect(c.getByText("Codex CLI source 1")).toBeVisible();
  await userEvent.clear(search);
  await userEvent.click(page.getByRole("option", { name: "Other repository" }));
  await expect(filter).toHaveTextContent("2 repositories");
  await expect(c.getByText("2 shown")).toBeVisible();
  await userEvent.click(
    page.getByRole("option", { name: "No recorded repository binding" }),
  );
  await expect(c.getByText("3 shown")).toBeVisible();
  await userEvent.keyboard("{Escape}");
  await waitFor(() => expect(filter).toHaveFocus());
  await userEvent.click(
    c.getByRole("button", { name: "Filter by source status" }),
  );
  await userEvent.click(page.getByRole("option", { name: "Degraded" }));
  await expect(c.getByText("1 shown")).toBeVisible();
  await expect(c.getByText("Codex Desktop source 3")).toBeVisible();
  await expect(c.getByText("3 loaded")).toBeVisible();
  await expect(c.getByText("5 observed")).toBeVisible();
  await userEvent.keyboard("{Escape}");
  await userEvent.click(filter);
  const reopened = await page.findByRole("combobox", {
    name: "Search source repositories",
  });
  await expect(reopened).toHaveValue("");
  await userEvent.click(
    page.getByRole("option", { name: "No recorded repository binding" }),
  );
  await expect(c.getByText("No sources match these filters")).toBeVisible();
  await userEvent.click(page.getByRole("option", { name: "All repositories" }));
  await expect(filter).toHaveTextContent("All repositories");
  await expect(c.getByText("Codex Desktop source 3")).toBeVisible();
  await userEvent.keyboard("{Escape}");
}
