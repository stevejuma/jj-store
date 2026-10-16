import { expect, userEvent, waitFor, within } from "storybook/test";

export async function expectRepositoryDirectoryLayout(
  canvasElement: HTMLElement,
) {
  const canvas = within(canvasElement);
  await canvas.findByRole(
    "region",
    { name: "Repository files" },
    { timeout: 10000 },
  );
  const body = within(canvasElement.ownerDocument.body);
  const bodyBreadcrumb = canvasElement.querySelector<HTMLElement>(
    ".store-application__body-breadcrumb",
  )!;
  const branchSelector = canvas.getByRole("combobox", {
    name: "Repository bookmark or change",
  });
  await expect(bodyBreadcrumb).not.toContainElement(branchSelector);
  await expect(getComputedStyle(bodyBreadcrumb).borderBottomWidth).toBe("0px");
  await userEvent.click(
    canvas.getByRole("button", { name: "Show repository file tree" }),
  );
  const fileTree = canvas.getByRole("complementary", {
    name: "Repository file tree",
  });
  await expect(
    within(fileTree).queryByRole("combobox", {
      name: "Repository bookmark or change",
    }),
  ).toBeNull();
  const fileTreeActions = fileTree.querySelector<HTMLElement>(
    ".community-projects__file-tree-actions",
  )!;
  await expect(within(fileTreeActions).getByText("Files")).toBeVisible();
  const repositoryLabel = bodyBreadcrumb.querySelector<HTMLElement>(
    '[data-slot="breadcrumb-link"]',
  )!;
  const fileTreeTitle = fileTreeActions.querySelector<HTMLElement>(
    ".community-projects__file-tree-title",
  )!;
  await expect(fileTreeTitle.getBoundingClientRect().left).toBeCloseTo(
    fileTreeActions.getBoundingClientRect().left +
      Number.parseFloat(getComputedStyle(fileTreeActions).paddingLeft),
    0,
  );
  await expect(repositoryLabel.getBoundingClientRect().left).toBeGreaterThan(
    fileTreeTitle.getBoundingClientRect().left,
  );
  await expect(
    within(fileTreeActions).getByRole("radio", {
      name: "Show files as list",
    }),
  ).toBeVisible();
  await expect(
    within(fileTreeActions).getByRole("radio", {
      name: "Show files as packages",
    }),
  ).toBeVisible();
  await userEvent.click(
    within(fileTreeActions).getByRole("radio", {
      name: "Show files as packages",
    }),
  );
  await expect(
    within(fileTreeActions).getByRole("radio", {
      name: "Show files as packages",
    }),
  ).toBeChecked();
  await expect(getComputedStyle(fileTree).borderRightWidth).toBe("1px");
  const fileTreeInner = fileTree.querySelector<HTMLElement>(
    ".community-projects__file-tree",
  )!;
  await expect(
    Number.parseFloat(getComputedStyle(fileTreeInner).paddingLeft),
  ).toBeGreaterThan(0);
  const filesBody = canvasElement.querySelector(
    ".store-application__body-regions",
  )!;
  const filesBodyStyle = getComputedStyle(filesBody);
  await expect(fileTree.getBoundingClientRect().height).toBeCloseTo(
    filesBody.getBoundingClientRect().height -
      Number.parseFloat(filesBodyStyle.paddingTop) -
      Number.parseFloat(filesBodyStyle.paddingBottom) -
      Number.parseFloat(filesBodyStyle.borderTopWidth),
    0,
  );
  const fileBrowser = canvasElement.querySelector<HTMLElement>(
    ".community-projects__file-browser",
  )!;
  await expect(
    Number.parseFloat(getComputedStyle(fileBrowser).paddingLeft),
  ).toBeGreaterThan(0);
  await expect(getComputedStyle(fileBrowser).borderTopWidth).toBe("0px");
  await expect(
    canvas.queryByRole("navigation", { name: "Repository directory path" }),
  ).toBeNull();
  const tableCard = canvasElement.querySelector<HTMLElement>(
    ".community-projects__file-browser-card",
  )!;
  await expect(getComputedStyle(tableCard).borderTopWidth).toBe("1px");
  await expect(
    Number.parseFloat(getComputedStyle(tableCard).borderTopLeftRadius),
  ).toBeGreaterThan(0);
  const summaryRow = canvasElement.querySelector<HTMLElement>(
    ".community-projects__file-browser-summary",
  )!;
  await expect(summaryRow).toContainElement(branchSelector);
  await expect(summaryRow.getBoundingClientRect().height).toBeGreaterThan(34);
  await expect(summaryRow.getBoundingClientRect().height).toBeLessThan(42);
  await expect(getComputedStyle(summaryRow).borderBottomWidth).toBe("1px");
  await expect(getComputedStyle(summaryRow).backgroundColor).not.toBe(
    "rgba(0, 0, 0, 0)",
  );
  await expect(
    Number.parseFloat(getComputedStyle(fileTreeActions).paddingTop),
  ).toBeLessThan(7);
  await expect(
    Number.parseFloat(getComputedStyle(summaryRow).paddingTop),
  ).toBeLessThan(7);
  await expect(summaryRow.querySelector("button")).toBe(
    canvas.getByRole("button", { name: "Hide repository file tree" }),
  );
  await expect(
    summaryRow.querySelector('[data-ui-part="git-author"]'),
  ).not.toBeNull();
  const tableHead = canvas.getByRole("columnheader", { name: "Name" });
  await expect(tableHead.getBoundingClientRect().height).toBeLessThan(34);
  await expect(
    Number.parseFloat(getComputedStyle(tableHead).paddingTop),
  ).toBeLessThan(6);
  await expect(getComputedStyle(tableHead).backgroundColor).not.toBe(
    getComputedStyle(summaryRow).backgroundColor,
  );
  const landingPreview = canvas.getByRole("region", {
    name: "Repository document preview",
  });
  const readmeTab = within(landingPreview).getByRole("tab", {
    name: "README",
  });
  await expect(readmeTab).toBeVisible();
  await expect(readmeTab.querySelector("svg")).not.toBeNull();
  await expect(getComputedStyle(readmeTab).justifyContent).not.toBe("center");
  await expect(
    within(landingPreview).getByRole("heading", {
      name: "jj-store",
      level: 1,
    }),
  ).toBeVisible();
  const resize = canvas.getByRole("button", {
    name: "Resize repository file tree",
  });
  const beforeResize = Number(resize.getAttribute("data-current-width"));
  resize.focus();
  await userEvent.keyboard("{ArrowRight}");
  await waitFor(() =>
    expect(Number(resize.getAttribute("data-current-width"))).toBeGreaterThan(
      beforeResize,
    ),
  );
  await expect(
    canvas.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await userEvent.click(
    canvas.getByRole("button", { name: "Open directory src" }),
  );
  const path = canvas.getByRole("list", { name: "Repository breadcrumb" });
  await expect(within(path).getByRole("link", { name: "Files" })).toBeVisible();
  await expect(within(path).getByText("src")).toBeVisible();
  await expect(
    canvas.queryByRole("region", { name: "Repository document preview" }),
  ).toBeNull();
  await userEvent.click(branchSelector);
  await waitFor(async () => {
    await expect(
      await body.findByRole("option", { name: "feature/native-objects" }),
    ).toBeVisible();
  });
  await userEvent.keyboard("{Escape}");
  await expect(
    canvas.getByRole("table", { name: "Files in src" }),
  ).toBeVisible();
  await userEvent.click(
    canvas.getByRole("button", { name: "Open file lib.rs" }),
  );
  const sourceBlock = canvasElement.querySelector<HTMLElement>(
    '[data-ui-component="code-block"]',
  )!;
  await expect(sourceBlock).toHaveTextContent("preserve_publication_history");
  await expect(sourceBlock).toHaveAttribute("data-language", "rust");
  await expect(
    sourceBlock.querySelector(".ui-code-token-keyword"),
  ).toHaveTextContent(/pub|fn/);
  await expect(
    canvas.getByRole("region", { name: "Selected file metadata" }),
  ).toBeVisible();
  await expect(
    canvas.getByRole("status", { name: "File statistics" }),
  ).toHaveTextContent(/\d+ lines \(\d+ loc\) · \d+ Bytes/);
  await userEvent.click(canvas.getByRole("button", { name: "Back" }));
  await expect(
    canvas.getByRole("table", { name: "Files in src" }),
  ).toBeVisible();
  await userEvent.click(
    canvas.getByRole("button", { name: "Go to parent directory" }),
  );
  await expect(
    canvas.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
}
