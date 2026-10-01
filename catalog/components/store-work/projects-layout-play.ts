import { expect, userEvent, waitFor, within } from "storybook/test";

export async function playOwnershipLayouts({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const canvas = within(canvasElement);
  const view = within(
    canvas.getByRole("region", { name: "Projects and tasks" }),
  );
  await waitFor(() =>
    expect(view.getByRole("button", { name: "Mine" })).toBeVisible(),
  );
  await userEvent.click(
    view.getByRole("button", { name: "Filters for projects" }),
  );
  const filters = within(
    within(document.body).getByRole("dialog", { name: "Projects filters" }),
  );
  const search = filters.getByLabelText("Search projects");
  await userEvent.click(search);
  const group = search.closest('[data-ui-part="input-group"]')!;
  await expect(getComputedStyle(group).borderTopWidth).toBe("0px");
  await expect(getComputedStyle(group).boxShadow).toBe("none");
  await expect(getComputedStyle(search).boxShadow).toBe("none");
  await expect(
    filters.getByRole("button", { name: "Search" }).querySelector("svg"),
  ).not.toBeNull();
  await expect(view.queryByLabelText("Search projects")).toBeNull();
  await userEvent.click(filters.getByRole("button", { name: "Close filters" }));
  await userEvent.click(view.getByRole("radio", { name: "Grid layout" }));
  await expect(view.getByRole("list", { name: "Projects" })).toHaveAttribute(
    "data-layout",
    "grid",
  );
  await userEvent.click(view.getByRole("button", { name: "Mine" }));
  await waitFor(() =>
    expect(
      view.queryByRole("button", { name: "Open project Native release" }),
    ).toBeNull(),
  );
  await userEvent.click(view.getByRole("button", { name: /^Mine/ }));
  await userEvent.click(view.getByRole("tab", { name: "Tasks" }));
  await userEvent.click(
    view.getByRole("button", { name: "Filters for tasks" }),
  );
  const stateFilter = within(document.body).getByRole("button", {
    name: "Task state",
  });
  await userEvent.click(stateFilter);
  const options = within(document.body).getAllByRole("menuitemradio");
  await expect(options).toHaveLength(6);
  for (const option of options)
    await expect(option.querySelector("svg")).not.toBeNull();
  await userEvent.click(
    within(document.body).getByRole("menuitemradio", { name: "In progress" }),
  );
  await expect(stateFilter).toHaveTextContent("In progress");
  await expect(stateFilter).toHaveFocus();
  await userEvent.click(
    within(document.body).getByRole("button", { name: "Close filters" }),
  );
  await expect(
    view.getByRole("button", { name: "Filters for tasks" }),
  ).toHaveAttribute("aria-pressed", "true");
  await waitFor(() =>
    expect(
      view.getAllByRole("list", { name: "Project tasks" })[0]!,
    ).toHaveAttribute("data-layout", "grid"),
  );
  await userEvent.click(view.getByRole("tab", { name: "Repositories" }));
  await expect(
    view.getByRole("list", { name: "My repositories" }),
  ).toHaveTextContent("Parser repository");
  await expect(
    view.getByRole("list", { name: "Other repositories" }),
  ).toHaveTextContent("UI repository");
  await expect(
    view.getByRole("list", { name: "My repositories" }),
  ).toHaveAttribute("data-layout", "grid");
  await userEvent.click(view.getByRole("radio", { name: "List layout" }));
  await expect(
    view.getByRole("list", { name: "My repositories" }),
  ).toHaveAttribute("data-layout", "list");
  const repositoryRow = view
    .getByRole("button", { name: "Open repository Parser repository" })
    .closest("article")!;
  const access = within(repositoryRow).getByText("admin access");
  const action = within(repositoryRow).getByRole("button", {
    name: "Projects and tasks",
  });
  await expect(access.getBoundingClientRect().right).toBeLessThan(
    action.getBoundingClientRect().left - 8,
  );
  await expect(action.getBoundingClientRect().right).toBeLessThanOrEqual(
    repositoryRow.getBoundingClientRect().right,
  );
}
