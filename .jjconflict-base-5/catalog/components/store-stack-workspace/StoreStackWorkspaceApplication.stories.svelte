<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceApplicationFixture from "../../fixtures/StackWorkspaceApplicationFixture.svelte";
  import {
    applyPreview,
    messagePreview,
    assertCompactColumnToggle,
  } from "../../fixtures/stack-workspace/story-actions.js";
  import { assertCanvasInsets } from "../../fixtures/stack-workspace/canvas-layout.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Application",
    component: StackWorkspaceApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Workspace in app shell"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await assertCompactColumnToggle(canvasElement);
    const views = canvas.getByRole("tablist", {
      name: "Local repository views",
    });
    await expect(
      within(views).queryByRole("combobox", { name: "Active stack workspace" }),
    ).toBeNull();
    const picker = canvas.getByRole("combobox", {
      name: "Active stack workspace",
    });
    const navigation = canvas.getByRole("navigation", {
      name: "Stack workspace navigation",
    });
    expect(navigation.contains(picker)).toBe(false);
    const selection = canvas.getByRole("group", {
      name: "Stack workspace selection",
    });
    expect(selection.contains(picker)).toBe(true);
    const header = canvas.getByRole("navigation", {
      name: "Application navigation",
    });
    expect(header).not.toBeNull();
    expect(header.getBoundingClientRect().bottom).toBeLessThanOrEqual(
      navigation.getBoundingClientRect().top,
    );
    const body = canvas.getByRole("region", { name: "Local stack workspace" });
    const bounds = body.getBoundingClientRect();
    expect(
      Math.abs(
        selection.getBoundingClientRect().left +
          selection.getBoundingClientRect().width / 2 -
          (bounds.left + bounds.width / 2),
      ),
    ).toBeLessThanOrEqual(2);
    expect(
      canvas.queryByRole("banner", { name: "Stack workspace controls" }),
    ).toBeNull();
    expect(canvas.queryByText("Git + JJ", { exact: true })).toBeNull();

    const columns = body.querySelector<HTMLElement>(
      ".stack-workspace__canvas",
    )!;
    expect(columns.getBoundingClientRect().top).toBeCloseTo(
      navigation.getBoundingClientRect().bottom,
      1,
    );
    await assertCanvasInsets(canvasElement);
    const spacer = columns.querySelector<HTMLElement>(
      '[data-ui-part="trailing-spacer"]',
    )!;
    await expect(spacer.getBoundingClientRect().width).toBe(390);
    const footer = body.querySelector<HTMLElement>(".stack-workspace__footer")!;
    expect(footer.textContent).not.toMatch(/Folder:|Base:/);
    expect(
      within(footer).getByText("Daily work", { exact: true }),
    ).toBeVisible();
    expect(within(footer).getByText("main", { exact: true })).toBeVisible();
  }}
/>
<Story
  name="Branches in app shell"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByRole("region", { name: "Branches" });
    await assertCanvasInsets(canvasElement);
  }}
/>
<Story
  name="Select branch and apply to workspace"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await canvas.findByRole("button", { name: "Stacks" });
    await expect(
      canvas.queryByRole("tablist", { name: "Repository sections" }),
    ).toBeNull();
    await expect(
      canvas.queryByRole("region", { name: "Stack feature/search" }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("option", { name: "feature/search" }),
    );
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/search",
    });
    await expect(
      within(stack).getByRole("button", { name: "Add search preference" }),
    ).toBeVisible();
    const apply = within(stack).getByRole("button", {
      name: "Apply feature/search",
    });
    await expect(apply).toHaveTextContent("Apply to workspace");
    await userEvent.click(apply);
    await applyPreview(canvasElement);
    await userEvent.click(
      within(
        canvas.getByRole("tablist", { name: "Local repository views" }),
      ).getByRole("tab", { name: "Workspace" }),
    );
    await expect(
      canvas.getByRole("region", { name: "Stack feature/search" }),
    ).toBeVisible();
    const working = canvas.getByRole("region", {
      name: "Unstaged files",
    });
    const api = canvas.getByRole("region", { name: "Stack feature/api" });
    await expect(working.getBoundingClientRect().left).toBeLessThan(
      api.getBoundingClientRect().left,
    );
    await expect(
      within(api).getAllByText("feature/api", { exact: true }),
    ).toHaveLength(2);
    await expect(within(api).getByText("Start a commit…")).toBeVisible();
  }}
/>
<Story
  name="Existing branches stays separate"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await canvas.findByRole("button", { name: "Stacks" });
    await userEvent.click(canvas.getByRole("button", { name: "jj-store" }));
    await expect(
      await canvas.findByRole("tablist", { name: "Repository sections" }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("region", { name: "Local stack workspace" }),
    ).toBeNull();
    await expect(canvas.getByRole("tab", { name: "Branches" })).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Stacks" }));
    await expect(
      await canvas.findByRole("region", { name: "Local stack workspace" }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("tablist", { name: "Repository sections" }),
    ).toBeNull();
  }}
/>
<Story
  name="Unlinked repository has no stacks entry"
  args={{ linked: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await canvas.findByText("Local checkout unavailable");
    await expect(canvas.queryByRole("button", { name: "Stacks" })).toBeNull();
    await expect(
      canvas.queryByRole("region", { name: "Stack feature/api" }),
    ).toBeNull();
  }}
/>

<Story
  name="Multiple adjacent diffs"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const api = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const ui = canvas.getByRole("region", { name: "Stack feature/workspace" });
    await userEvent.click(
      within(api).getByRole("button", { name: "Expose repository API" }),
    );
    await userEvent.click(
      within(api).getByRole("button", { name: "src/api.ts" }),
    );
    const apiDiff = await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    await userEvent.click(
      within(apiDiff).getByRole("checkbox", { name: "query endpoint" }),
    );
    await userEvent.click(
      within(apiDiff).getByRole("radio", { name: "Show split diff" }),
    );
    await userEvent.click(
      within(ui).getByRole("button", { name: "Add workspace toolbar" }),
    );
    await userEvent.click(
      within(ui).getByRole("button", { name: "src/toolbar.ts" }),
    );
    const uiDiff = await canvas.findByRole("region", {
      name: "Selected changes from stack-ui",
    });
    const working = canvas.getByRole("region", {
      name: "Unstaged files",
    });
    await userEvent.click(
      within(working).getByRole("button", { name: "README.md" }),
    );
    await canvas.findByRole("region", {
      name: "Selected changes from working",
    });
    const columnIds = () =>
      [
        ...canvasElement.querySelectorAll<HTMLElement>(
          '[data-ui-part="column"]',
        ),
      ].map((element) => element.dataset.columnId);
    await waitFor(() =>
      expect(columnIds()).toEqual([
        "working",
        "diff-working",
        "stack-api",
        "diff-stack-api",
        "stack-ui",
        "diff-stack-ui",
      ]),
    );
    await expect(apiDiff.getBoundingClientRect().left).toBeGreaterThan(
      api.getBoundingClientRect().left,
    );
    await expect(apiDiff.getBoundingClientRect().right).toBeLessThanOrEqual(
      ui.getBoundingClientRect().left,
    );
    await expect(
      within(apiDiff).getByRole("checkbox", { name: "query endpoint" }),
    ).not.toBeChecked();
    await expect(
      within(apiDiff).getByRole("radio", { name: "Show split diff" }),
    ).toBeChecked();
    await expect(
      within(uiDiff).getByRole("checkbox", { name: "workspace picker" }),
    ).toBeChecked();
    await userEvent.click(
      within(apiDiff).getByRole("button", { name: "Split selected changes" }),
    );
    const portal = within(canvasElement.ownerDocument.body);
    await expect(await portal.findByLabelText("Commit message")).toBeVisible();
    await messagePreview(canvasElement, "Separate mutation endpoint");
    await expect(
      await portal.findByText(/Only the selected changes in src\/api.ts/),
    ).toHaveTextContent("1 selected hunk(s).");
    await userEvent.click(
      portal.getByRole("button", { name: "Cancel preview" }),
    );
    await userEvent.click(
      within(uiDiff).getByRole("button", { name: "Close Diff column" }),
    );
    await waitFor(() =>
      expect(
        canvas.queryByRole("region", {
          name: "Selected changes from stack-ui",
        }),
      ).toBeNull(),
    );
    await expect(apiDiff).toBeInTheDocument();
    await userEvent.click(
      within(ui).getByRole("button", { name: "Add workspace toolbar" }),
    );
    await userEvent.click(
      within(ui).getByRole("button", { name: "src/toolbar.ts" }),
    );
    await canvas.findByRole("region", {
      name: "Selected changes from stack-ui",
    });
    await waitFor(() =>
      expect(columnIds()).toEqual([
        "working",
        "diff-working",
        "stack-api",
        "diff-stack-api",
        "stack-ui",
        "diff-stack-ui",
      ]),
    );
  }}
/>

<Story
  name="Stack header and row alignment"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const header = stack.querySelector<HTMLElement>(
      ".stack-workspace__stack-header",
    )!;
    const before = getComputedStyle(header).backgroundColor;
    within(header)
      .getByRole("button", { name: "Show all changes in stack feature/api" })
      .focus();
    await waitFor(() =>
      expect(getComputedStyle(header).backgroundColor).not.toBe(before),
    );
    const avatars = [
      ...stack.querySelectorAll<HTMLElement>(
        '[data-ui-component="community-member-avatar"]',
      ),
    ];
    for (const avatar of avatars) {
      await expect(avatar.dataset.size).toBe("xsmall");
      await expect(avatar.getBoundingClientRect().width).toBe(24);
      await expect(
        Math.abs(
          avatar.getBoundingClientRect().left -
            avatars[0]!.getBoundingClientRect().left,
        ),
      ).toBeLessThanOrEqual(1);
    }
    for (const row of stack.querySelectorAll<HTMLElement>(
      "[data-revision-id]",
    )) {
      const button = within(row).getByRole("button", { name: /^Actions for / });
      const menu = button.getBoundingClientRect(),
        bounds = row.getBoundingClientRect();
      await expect(bounds.right - menu.right).toBeLessThanOrEqual(9);
      await expect(
        Math.abs(
          (menu.top + menu.bottom) / 2 - (bounds.top + bounds.bottom) / 2,
        ),
      ).toBeLessThanOrEqual(1);
    }
  }}
/>

<Story
  name="Whole stack files and continuous graph"
  args={{ continuousStack: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Show all changes in stack feature/api",
      }),
    );
    await expect(
      stack.querySelector(".stack-workspace__bookmark-header"),
    ).toHaveAttribute("data-selected");
    const files = within(stack).getByRole("region", {
      name: "Changed files in branch feature/api",
    });
    await expect(
      within(files).getByRole("button", { name: "src/model.ts" }),
    ).toBeVisible();
    await userEvent.click(
      within(files).getByRole("button", { name: "src/api.ts" }),
    );
    await expect(
      await canvas.findByRole("group", {
        name: "Draggable hunk query endpoint",
      }),
    ).toHaveAttribute("draggable", "true");
    await expect(
      canvas.getByRole("button", { name: "Assign hunk query endpoint" }),
    ).toBeEnabled();
    await userEvent.click(
      within(stack).getByRole("button", { name: "Expose repository API" }),
    );
    await expect(
      stack.querySelector('[aria-current="true"] [data-revision-id]'),
    ).toHaveAttribute("data-revision-id", "api2");
    await waitFor(() => {
      for (const row of stack.querySelectorAll<HTMLElement>(
        "[data-revision-id]",
      )) {
        const svg = row.querySelector<SVGElement>(
          ".community-revision-graph__svg",
        )!;
        const stem = svg.querySelector("line")!;
        expect(stem.getAttribute("y1")).toBe("0");
        expect(Number(stem.getAttribute("y2"))).toBeCloseTo(
          row.parentElement!.getBoundingClientRect().height,
          0,
        );
        expect(svg.getBoundingClientRect().height).toBeCloseTo(
          row.parentElement!.getBoundingClientRect().height,
          0,
        );
      }
      const rows = [
        ...stack.querySelectorAll<HTMLElement>("[data-revision-id]"),
      ];
      expect(
        rows[0]!.querySelector("svg")!.getBoundingClientRect().bottom,
      ).toBeCloseTo(rows[1]!.getBoundingClientRect().top, 0);
    });
  }}
/>
