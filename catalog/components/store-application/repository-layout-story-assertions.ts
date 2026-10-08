import { expect, userEvent, waitFor, within } from "storybook/test";

export async function expectPersistentRepositoryLayout(
  canvasElement: HTMLElement,
) {
  const canvas = within(canvasElement),
    body = within(canvasElement.ownerDocument.body);
  const shellResize = canvas.getByRole("slider", {
    name: "Resize repository navigation sidebar",
  });
  await new Promise<void>((resolve) => setTimeout(resolve, 0));
  const shellWidth = Number(shellResize.getAttribute("aria-valuenow"));
  shellResize.focus();
  await userEvent.keyboard("{ArrowRight}{ArrowRight}");
  await waitFor(() =>
    expect(Number(shellResize.getAttribute("aria-valuenow"))).toBeGreaterThan(
      shellWidth,
    ),
  );
  const persistedShellWidth = Number(shellResize.getAttribute("aria-valuenow"));

  await userEvent.click(
    canvas.getByRole("button", { name: "Collapse Branches column" }),
  );
  for (const name of ["Branches", "Commits", "Op Log", "Agents", "Reviews"]) {
    await userEvent.click(canvas.getByRole("tab", { name }));
    await expect(
      canvas.queryByRole("list", { name: "Repository breadcrumb" }),
    ).not.toBeInTheDocument();
  }
  await userEvent.click(canvas.getByRole("tab", { name: "Overview" }));
  const breadcrumb = canvas.getByRole("list", {
    name: "Repository breadcrumb",
  });
  await expect(breadcrumb).toBeVisible();
  await expect(
    getComputedStyle(breadcrumb.closest("nav")!).borderBottomWidth,
  ).toBe("0px");
  await expect(
    canvas.queryByRole("complementary", { name: "Repository file tree" }),
  ).toBeNull();
  await userEvent.click(
    canvas.getByRole("button", { name: "Show repository file tree" }),
  );
  const regions = canvasElement.querySelector<HTMLElement>(
    '.store-application__body-regions[data-page="files"]',
  )!;
  const styles = getComputedStyle(regions);
  for (const side of [
    styles.paddingTop,
    styles.paddingRight,
    styles.paddingBottom,
    styles.paddingLeft,
  ])
    await expect(side).toBe("0px");
  await expect(styles.borderTopWidth).toBe("1px");
  await expect(styles.marginTop).toBe("8px");
  const treeResize = canvas.getByRole("button", {
    name: "Resize repository file tree",
  });
  const treeWidth = Number(treeResize.getAttribute("data-current-width"));
  treeResize.focus();
  await userEvent.keyboard("{ArrowRight}{ArrowRight}");
  const persistedTreeWidth = Number(
    treeResize.getAttribute("data-current-width"),
  );
  await expect(persistedTreeWidth).toBeGreaterThan(treeWidth);
  await userEvent.click(
    canvas.getByRole("button", { name: "Hide repository file tree" }),
  );
  await userEvent.click(
    canvas.getByRole("button", { name: "Remount application" }),
  );
  await waitFor(() =>
    expect(
      canvasElement.querySelector(".layout-persistence-fixture"),
    ).toHaveAttribute("data-mount-version", "1"),
  );
  await expect(
    canvasElement.querySelector(".layout-persistence-fixture"),
  ).toHaveAttribute(
    "data-saved-shell-layout",
    expect.stringContaining(`\"width\":${persistedShellWidth}`),
  );
  await waitFor(() =>
    expect(
      canvas.getByRole("button", { name: "Show repository file tree" }),
    ).toBeVisible(),
  );
  const restoredShellResize = canvas.getByRole("slider", {
    name: "Resize repository navigation sidebar",
  });
  await waitFor(() =>
    expect(Number(restoredShellResize.getAttribute("aria-valuenow"))).toBe(
      persistedShellWidth,
    ),
  );
  await userEvent.click(
    canvas.getByRole("button", { name: "Show repository file tree" }),
  );
  await expect(
    Number(
      canvas
        .getByRole("button", { name: "Resize repository file tree" })
        .getAttribute("data-current-width"),
    ),
  ).toBe(persistedTreeWidth);

  await userEvent.click(canvas.getByRole("tab", { name: "Branches" }));
  await expect(
    canvas.getByRole("button", { name: "Expand Branches column" }),
  ).toBeVisible();
  await userEvent.click(canvas.getByRole("tab", { name: "Commits" }));
  await expect(
    canvas.getByRole("button", { name: "Collapse JJ Log column" }),
  ).toBeVisible();
  await userEvent.click(canvas.getByRole("tab", { name: "Branches" }));
  await expect(
    canvas.getByRole("button", { name: "Expand Branches column" }),
  ).toBeVisible();
}
