<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import {
    expectColumnCanvasFillsBody,
    expectRepositoryBreadcrumbLayout,
  } from "./column-canvas-story-assertions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Review refinements",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { initiallyCollapsed: false },
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Shell and graph alignment"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvasElement.querySelector(".lc-community-navigation__header"),
    ).toBeNull();
    const details = canvas.getByRole("button", { name: "Toggle details" });
    await expect(
      details.querySelector(
        '[data-ui-part="sidebar-toggle-icon"][data-side="right"]',
      ),
    ).not.toBeNull();
    const group = canvasElement.querySelector(
      '[data-ui-part="command-group"]',
    )!;
    const option = canvas.getByRole("option", { name: "main" });
    await expect(option.getBoundingClientRect().left).toBeCloseTo(
      group.getBoundingClientRect().left,
      0,
    );
    await expect(option.getBoundingClientRect().right).toBeCloseTo(
      group.getBoundingClientRect().right,
      0,
    );
    const heading = canvasElement.querySelector(
      '[data-ui-part="command-group-heading"]',
    )!;
    await expect(getComputedStyle(heading).position).toBe("sticky");
    await expect(getComputedStyle(heading).borderBottomWidth).toBe("1px");
    await expect(getComputedStyle(heading).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    const viewport = heading.closest<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    const top = heading.getBoundingClientRect().top;
    viewport.scrollTop = 120;
    await waitFor(() => expect(viewport.scrollTop).toBeGreaterThan(0));
    await expect(heading.getBoundingClientRect().top).toBeCloseTo(top, 0);
    viewport.scrollTop = 0;
    await expect(
      getComputedStyle(canvas.getByRole("option", { name: "retired" }))
        .borderBottomWidth,
    ).toBe("1px");
    await expect(
      getComputedStyle(canvas.getByRole("option", { name: "retired" }))
        .borderBottomLeftRadius,
    ).toBe("0px");
    const branchHeader = canvasElement.querySelector(
      '[data-ui-part="column-header"]',
    )!;
    await expect(getComputedStyle(branchHeader).backgroundColor).toBe(
      getComputedStyle(heading).backgroundColor,
    );
    await userEvent.click(details);
    const rail = canvas.getByRole("complementary", {
      name: "Repository details",
    });
    await expect(rail).toHaveAttribute("data-variant", "rail");
    await expect(getComputedStyle(rail).backgroundColor).toBe(
      "rgba(0, 0, 0, 0)",
    );
    await expect(getComputedStyle(rail).borderLeftColor).toBe(
      "rgba(0, 0, 0, 0)",
    );
    await expect(
      getComputedStyle(canvas.getByRole("option", { name: "main" }))
        .borderBottomWidth,
    ).toBe("1px");
    await userEvent.click(details);
    await userEvent.click(canvas.getByRole("option", { name: "main" }));
    await waitFor(() => {
      const avatars = Array.from(
        canvasElement.querySelectorAll(
          '.community-revision-graph-copy [data-ui-part="git-author"]',
        ),
      );
      expect(avatars.length).toBeGreaterThan(2);
      const xs = avatars.map((node) => node.getBoundingClientRect().left);
      expect(Math.max(...xs) - Math.min(...xs)).toBeLessThan(1);
    });
    await userEvent.click(
      canvas.getByRole("button", {
        name: "Merge native storage and publication",
      }),
    );
    await userEvent.click(
      canvas.getByRole("radio", { name: "Show files as list" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "src/lib.rs" }));
    const headers = canvasElement.querySelectorAll(
      '[data-ui-part="column-header"]',
    );
    await expect(headers).toHaveLength(3);
    for (const header of headers) {
      await expect(getComputedStyle(header).backgroundColor).toBe(
        getComputedStyle(heading).backgroundColor,
      );
      await expect(getComputedStyle(header).borderBottomWidth).toBe("1px");
    }
    await userEvent.click(canvas.getByRole("button", { name: "Op Log" }));
    const status = canvas.getByRole("status");
    await expect(getComputedStyle(status).borderBottomWidth).toBe("1px");
    await expect(getComputedStyle(status).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    const opHeader = canvasElement.querySelector(
      '[data-ui-part="column-header"]',
    )!;
    await expect(getComputedStyle(status).backgroundColor).not.toBe(
      getComputedStyle(opHeader).backgroundColor,
    );
    const graph = canvas.getByRole("list", { name: "Operation graph" });
    await expect(graph.getBoundingClientRect().top).toBeCloseTo(
      status.getBoundingClientRect().bottom,
      0,
    );
    const opRowsBeforeSelection = canvasElement.querySelectorAll<HTMLElement>(
      ".community-revision-graph__row",
    );
    await expect(opRowsBeforeSelection.length).toBeGreaterThan(1);
    await expect(
      getComputedStyle(opRowsBeforeSelection[0]!).borderBottomWidth,
    ).toBe("1px");
    await expect(
      getComputedStyle(opRowsBeforeSelection[opRowsBeforeSelection.length - 1]!)
        .borderBottomWidth,
    ).toBe("1px");
    await expect(
      getComputedStyle(opRowsBeforeSelection[opRowsBeforeSelection.length - 1]!)
        .borderBottomLeftRadius,
    ).toBe("0px");
    await userEvent.click(
      canvas.getByRole("button", {
        name: "describe: native object history",
      }),
    );
    const operationFiles = await canvas.findByRole("region", {
      name: "Changed files in operation",
    });
    await expect(operationFiles).toBeVisible();
    const operationSummary = canvas.getByRole("region", {
      name: "Selected operation",
    });
    const diffColumn = operationSummary.closest<HTMLElement>(
      '[data-ui-part="column"]',
    )!;
    const closeButton = within(diffColumn).getByRole("button", {
      name: "Close Diff column",
    });
    await expect(closeButton).toBeVisible();
    await expect(getComputedStyle(closeButton).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    await expect(
      canvas.queryByText("Operation details"),
    ).not.toBeInTheDocument();
    const metadata = canvas.getByRole("region", {
      name: "Selected operation metadata",
    });
    await expect(getComputedStyle(metadata).borderBottomWidth).toBe("0px");
    const copyButton = within(operationSummary).getByRole("button", {
      name: /Copy full operation ID/,
    });
    const parentButton = within(operationSummary).getByRole("button", {
      name: /Parent/,
    });
    await expect(getComputedStyle(copyButton).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    await expect(getComputedStyle(parentButton).backgroundColor).toBe(
      getComputedStyle(copyButton).backgroundColor,
    );
    await expect(
      within(diffColumn).getByText("No file selected"),
    ).toBeVisible();
    await expect(within(diffColumn).getByRole("status")).toHaveTextContent(
      "Select a changed file to preview the operation diff.",
    );
    await expect(
      diffColumn.querySelector('[data-ui-part="empty-media"] svg'),
    ).not.toBeNull();
    await expect(
      within(diffColumn).getByRole("button", {
        name: "describe: native object history",
      }),
    ).toHaveAttribute("aria-expanded", "true");
    await userEvent.click(
      within(operationFiles).getByRole("button", { name: "assets/logo.bin" }),
    );
    await expect(
      within(diffColumn).getByText("Binary diff unavailable"),
    ).toBeVisible();
    await expect(within(diffColumn).getByRole("status")).toHaveTextContent(
      "Binary or unavailable text diff for assets/logo.bin.",
    );
    await userEvent.click(
      within(operationFiles).getByRole("button", { name: "src/lib.rs" }),
    );
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-ui-component="file-diff"]'),
      ).not.toBeNull(),
    );
    await expect(
      canvasElement.querySelector('[data-ui-component="file-diff"]'),
    ).toHaveTextContent("publish_native_objects()?");
    const selectedFileDiff = canvas.getByRole("region", {
      name: "Selected file diff",
    });
    await expect(operationSummary.getBoundingClientRect().left).toBeCloseTo(
      selectedFileDiff.getBoundingClientRect().left,
      0,
    );
    await expect(operationSummary.getBoundingClientRect().right).toBeCloseTo(
      selectedFileDiff.getBoundingClientRect().right,
      0,
    );
    const operationHeaders = canvasElement.querySelectorAll(
      '[data-ui-part="column-header"]',
    );
    await expect(operationHeaders).toHaveLength(1);
    await expect(getComputedStyle(operationHeaders[0]!).backgroundColor).toBe(
      getComputedStyle(opHeader).backgroundColor,
    );
    await expect(getComputedStyle(operationHeaders[0]!).borderBottomWidth).toBe(
      "1px",
    );
    const breadcrumb = canvas.getByRole("list", {
      name: "Repository breadcrumb",
    });
    await expectRepositoryBreadcrumbLayout(breadcrumb);
    await expect(
      getComputedStyle(
        breadcrumb.querySelector('[data-ui-part="breadcrumb-page"]')!,
      ).color,
    ).toBe(getComputedStyle(status).color);
  }}
/>

<Story
  name="Repository directory navigation"
  args={{ initialPage: "files" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const body = within(canvasElement.ownerDocument.body);
    const filesToolbar = canvasElement.querySelector<HTMLElement>(
      ".store-application__files-toolbar",
    )!;
    const branchSelector = canvas.getByRole("combobox", {
      name: "Repository bookmark or change",
    });
    await expect(filesToolbar).not.toContainElement(branchSelector);
    const pageNav = filesToolbar.closest<HTMLElement>(
      '[data-ui-part="toolbar"]',
    )!;
    await expect(getComputedStyle(pageNav).borderBottomWidth).toBe("1px");
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
    const pageFilesLabel = filesToolbar.querySelector<HTMLElement>(
      '[data-slot="breadcrumb-page"]',
    )!;
    const fileTreeTitle = fileTreeActions.querySelector<HTMLElement>(
      ".community-projects__file-tree-title",
    )!;
    await expect(fileTreeTitle.getBoundingClientRect().left).toBeCloseTo(
      pageFilesLabel.getBoundingClientRect().left,
      0,
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
    await expect(getComputedStyle(pageNav).borderBottomColor).toBe(
      getComputedStyle(fileTree).borderRightColor,
    );
    const fileTreeInner = fileTree.querySelector<HTMLElement>(
      ".community-projects__file-tree",
    )!;
    await expect(
      Number.parseFloat(getComputedStyle(fileTreeInner).paddingLeft),
    ).toBeGreaterThan(0);
    const filesBody = canvasElement.querySelector(
      '[data-ui-part="body"][data-layout="regions"]',
    )!;
    await expect(fileTree.getBoundingClientRect().height).toBeCloseTo(
      filesBody.getBoundingClientRect().height,
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
    const path = canvas.getByRole("list", { name: "Files breadcrumb" });
    await expect(
      within(path).getByRole("link", { name: "Files" }),
    ).toBeVisible();
    await expect(within(path).getByText("src")).toBeVisible();
    await expect(
      canvas.queryByRole("region", { name: "Repository document preview" }),
    ).toBeNull();
    await userEvent.click(branchSelector);
    await expect(
      await body.findByRole("option", { name: "feature/native-objects" }),
    ).toBeVisible();
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
  }}
/>

<Story
  name="Column titles replace body header"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    for (const label of ["Branches", "JJ Log", "Op Log"]) {
      await userEvent.click(canvas.getByRole("button", { name: label }));
      await expectColumnCanvasFillsBody(canvasElement);
      await expect(
        canvasElement.querySelector('[data-ui-part="toolbar"]'),
      ).toBeNull();
      await expect(
        canvasElement.querySelector('[data-ui-part="column-header"]'),
      ).toHaveTextContent(label);
      await userEvent.click(
        canvas.getByRole("button", { name: `Collapse ${label} column` }),
      );
      const collapsed = canvas.getByRole("button", {
        name:
          label === "Branches"
            ? "Expand Branches column"
            : "Expand revisions column",
      }).parentElement!;
      const canvasRoot = collapsed.closest(
        '[data-ui-component="column-canvas"][data-ui-part="root"]',
      )!;
      const rootStyle = getComputedStyle(canvasRoot);
      const rootPaddingTop = Number.parseFloat(rootStyle.paddingTop);
      const rootPaddingBottom = Number.parseFloat(rootStyle.paddingBottom);
      await expect(rootPaddingTop).toBeGreaterThan(0);
      await expect(collapsed.getBoundingClientRect().top).toBeCloseTo(
        canvasRoot.getBoundingClientRect().top + rootPaddingTop,
        0,
      );
      await expect(collapsed.getBoundingClientRect().height).toBeCloseTo(
        canvasRoot.getBoundingClientRect().height -
          rootPaddingTop -
          rootPaddingBottom,
        1,
      );
      await userEvent.click(
        canvas.getByRole("button", {
          name:
            label === "Branches"
              ? "Expand Branches column"
              : "Expand revisions column",
        }),
      );
    }
  }}
/>
