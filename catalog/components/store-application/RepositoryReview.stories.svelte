<script module lang="ts">
  import { expectRepositoryDirectoryLayout } from "./repository-directory-story-assertions.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import { expectColumnCanvasFillsBody } from "./column-canvas-story-assertions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Review refinements",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { initiallyCollapsed: false },
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Shell and row-local graph spacing"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await canvas.findByRole("option", { name: "main" });
    const navigationHeader = canvasElement.querySelector<HTMLElement>(
      ".lc-community-navigation__header",
    )!;
    await expect(
      within(navigationHeader).getByRole("button", {
        name: "Search repository",
      }),
    ).toBeVisible();
    await expect(within(navigationHeader).queryByText("jj-store")).toBeNull();
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
      expect(Math.max(...xs) - Math.min(...xs)).toBeGreaterThan(1);
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
    await expect(headers).toHaveLength(5);
    for (const header of headers) {
      await expect(getComputedStyle(header).backgroundColor).toBe(
        getComputedStyle(heading).backgroundColor,
      );
      await expect(getComputedStyle(header).borderBottomWidth).toBe("1px");
    }
    await userEvent.click(canvas.getByRole("tab", { name: "Op Log" }));
    const graph = await canvas.findByRole("list", { name: "Operation graph" });
    const opHeader = canvasElement.querySelector(
      '[data-ui-part="column-header"]',
    )!;
    await expect(canvas.queryByRole("status")).toBeNull();
    await expect(
      await within(document.body).findByText(
        /operation heads · selecting one does not reconcile history/,
      ),
    ).toBeVisible();
    await expect(graph.getBoundingClientRect().top).toBeCloseTo(
      opHeader.getBoundingClientRect().bottom,
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
      await within(diffColumn).findByText("Text diff unavailable"),
    ).toBeVisible();
    await expect(
      await within(diffColumn).findByRole("status"),
    ).toHaveTextContent("No text diff is available for this comparison.");
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
      '[data-ui-part="column"][data-column-id="revisions"] [data-ui-part="column-header"]',
    );
    await expect(operationHeaders).toHaveLength(1);
    await expect(getComputedStyle(operationHeaders[0]!).backgroundColor).toBe(
      getComputedStyle(opHeader).backgroundColor,
    );
    await expect(getComputedStyle(operationHeaders[0]!).borderBottomWidth).toBe(
      "1px",
    );
    await expect(
      canvas.queryByRole("list", { name: "Repository breadcrumb" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Repository directory navigation"
  args={{ initialPage: "files" }}
  play={async ({ canvasElement }) =>
    expectRepositoryDirectoryLayout(canvasElement)}
/>

<Story
  name="Column titles replace body header"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    for (const label of ["Branches", "JJ Log", "Op Log"]) {
      await userEvent.click(
        canvas.getByRole("tab", {
          name: label === "JJ Log" ? "Commits" : label,
        }),
      );
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
