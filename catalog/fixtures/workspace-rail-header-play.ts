import { expect, userEvent, within, waitFor } from "storybook/test";
export const verifyWorkspaceRailHeader = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement);

  await waitFor(() =>
    expect(
      c.getByRole("button", { name: "Toggle workspace files" }),
    ).toBeEnabled(),
  );
  await userEvent.click(
    c.getByRole("button", { name: "Toggle workspace files" }),
  );
  const panel = await c.findByRole("complementary", {
    name: "Workspace files",
  });
  await within(panel).findByRole("button", { name: "README.md" });
  const header = panel.querySelector<HTMLElement>(
    '[data-ui-part="sidebar-header"]',
  )!;
  const toolbar = panel.querySelector<HTMLElement>(
    ".community-projects__file-tree-actions",
  )!;
  await expect(
    {
      headerBorder: getComputedStyle(header).borderBottomWidth,
      startsAtPanelEdge:
        Math.abs(
          toolbar.getBoundingClientRect().left -
            panel.getBoundingClientRect().left,
        ) <= 1,
      endsAtPanelEdge:
        Math.abs(
          toolbar.getBoundingClientRect().right -
            panel.getBoundingClientRect().right,
        ) <= 1,
    },
    JSON.stringify({
      state: panel.getAttribute("data-state"),
      variant: panel.getAttribute("data-variant"),
      shell: !!header.closest(".community-app-shell__surface"),
      headerClass: header.className,
    }),
  ).toEqual({
    headerBorder: "1px",
    startsAtPanelEdge: true,
    endsAtPanelEdge: true,
  });
  await expect(getComputedStyle(toolbar).borderBottomWidth).toBe("1px");
};
