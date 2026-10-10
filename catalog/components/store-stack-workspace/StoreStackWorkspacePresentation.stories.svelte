<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import {
    applyPreview,
    chooseWorkspace,
    messagePreview,
    workspaceCanvas,
    workspacePortal,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Presentation",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Whole stack headers and selected commits"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const header = stack.querySelector<HTMLElement>(
      '[data-ui-part="column-header"]',
    )!;
    await userEvent.click(header);
    expect(
      stack.querySelector(".stack-workspace__bookmark-header"),
    ).toHaveAttribute("data-selected");
    expect(
      within(stack).getByRole("region", {
        name: "Changed files in branch feature/api",
      }),
    ).toBeVisible();
    await userEvent.click(
      within(stack).getByRole("button", { name: "Expose repository API" }),
    );
    const selected = stack.querySelector<HTMLElement>(
      '[aria-current="true"] .community-revision-graph__row',
    )!;
    expect(
      within(selected).queryByText("Selected", { exact: true }),
    ).toBeNull();
    expect(getComputedStyle(selected, "::before").width).toBe("4px");
    const bookmarkHeader = stack.querySelector<HTMLElement>(
      ".stack-workspace__bookmark-header",
    )!;
    await userEvent.click(bookmarkHeader);
    expect(
      stack.querySelector(".stack-workspace__bookmark-header"),
    ).toHaveAttribute("data-selected");
    expect(
      within(stack).queryByText("Selected", {
        selector: ".community-revision-graph__row *",
      }),
    ).toBeNull();
    for (const list of canvas.getAllByRole("list", { name: /^Commits in / }))
      expect(within(list).getAllByRole("listitem").length).toBeGreaterThan(1);
    // Header actions keep their own semantics instead of selecting the stack.
    await userEvent.click(
      within(stack).getByRole("button", { name: "Expose repository API" }),
    );
    await userEvent.click(
      within(header).getByRole("button", { name: "Actions for feature/api" }),
    );
    expect(bookmarkHeader).not.toHaveAttribute("data-selected");
    await userEvent.keyboard("{Escape}");
  }}
/>

<Story
  name="Diff header and wide action menus"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    for (const name of [
      "Workspace actions",
      "Actions for feature/api",
      "Branch actions for feature/api",
      "Actions for Expose repository API",
    ]) {
      await userEvent.click(canvas.getByRole("button", { name }));
      const menu = await portal.findByRole("menu");
      await waitFor(() =>
        expect(menu.getBoundingClientRect().width).toBeGreaterThanOrEqual(288),
      );
      for (const item of within(menu).getAllByRole("menuitem")) {
        for (const node of item.childNodes) {
          if (node.nodeType !== Node.TEXT_NODE || !node.textContent?.trim())
            continue;
          const range = item.ownerDocument.createRange();
          range.selectNode(node);
          expect(range.getClientRects()).toHaveLength(1);
        }
      }
      const unapply = within(menu).queryByRole("menuitem", {
        name: "Unapply stack",
      });
      if (unapply)
        expect(unapply.querySelector(".lucide-eject")).not.toBeNull();
      await userEvent.keyboard("{Escape}");
      await waitFor(() => expect(portal.queryByRole("menu")).toBeNull());
    }
    await userEvent.click(canvas.getByRole("button", { name: "README.md" }));
    const diff = await canvas.findByRole("region", {
      name: "Selected changes from working",
    });
    const header = diff.querySelector<HTMLElement>(
      '[data-ui-part="column-header"]',
    )!;
    const body = diff.querySelector<HTMLElement>(
      '[data-ui-part="column-body"]',
    )!;
    expect(
      within(header).getByRole("radio", { name: "Show split diff" }),
    ).toBeVisible();
    expect(
      within(header).getByRole("button", { name: "Wrap text" }),
    ).toBeVisible();
    expect(
      within(body).queryByRole("radio", { name: "Show split diff" }),
    ).toBeNull();
    expect(
      canvas.queryByRole("button", { name: "Commit selected changes" }),
    ).toBeNull();
    expect(canvas.queryByRole("button", { name: "Amend commit" })).toBeNull();
    {
      const commit = canvas.getByRole("button", {
        name: "Expose repository API",
      });
      if (!commit.closest('[aria-current="true"]'))
        await userEvent.click(commit);
    }
    await userEvent.click(
      await canvas.findByRole("button", { name: "src/api.ts" }),
    );
    const stackDiff = await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    expect(
      within(
        stackDiff.querySelector<HTMLElement>('[data-ui-part="column-header"]')!,
      ).getByRole("button", {
        name: "Split selected changes",
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="Icon toolbar and workspace command picker"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    const toolbar = within(
      await canvas.findByRole("group", { name: "Stack workspace controls" }),
    );
    for (const name of [
      "Workspace actions",
      "Create stack",
      "Undo",
      "Refresh",
    ]) {
      const action = toolbar.getByRole("button", { name });
      expect(action.textContent?.trim()).toBe("");
      expect(action.querySelector("svg")).not.toBeNull();
    }
    const action = toolbar.getByRole("button", { name: "Create stack" });
    await userEvent.hover(action);
    await expect(action).toHaveAttribute("aria-describedby");
    await expect(
      await portal.findByText("Create stack", { exact: true }),
    ).toBeVisible();
    await userEvent.unhover(action);
    const picker = canvas.getByRole("combobox", {
      name: "Active stack workspace",
    });
    await userEvent.click(picker);
    const search = await portal.findByLabelText("Search workspaces");
    await userEvent.type(search, "unmatched");
    await expect(portal.getByText("No matching workspaces.")).toBeVisible();
    await userEvent.clear(search);
    await userEvent.type(search, "Exper");
    await expect(
      portal.getByRole("option", { name: /Experiment/ }),
    ).toBeVisible();
    expect(portal.queryByRole("option", { name: /Daily work/ })).toBeNull();
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await portal.findByRole("button", { name: "Apply command" });
    expect(picker).toHaveTextContent("Daily work");
    await userEvent.click(
      portal.getByRole("button", { name: "Cancel preview" }),
    );
    expect(picker).toHaveTextContent("Daily work");
    await chooseWorkspace(canvasElement, "Experiment");
    await expect(picker).toHaveTextContent("Experiment");
  }}
/>

<Story
  name="Linked bookmarks and optional actions"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const connector = stack.querySelector<HTMLElement>(
      '[data-ui-part="stack-link"]',
    )!;
    expect(connector).not.toBeNull();
    for (const card of stack.querySelectorAll(
      ".stack-workspace__bookmark-card",
    )) {
      const title = card.querySelector<HTMLElement>(
        '[data-ui-part="column-title"]',
      )!;
      const columnTitle = stack.querySelector<HTMLElement>(
        '[data-ui-part="column-header"] [data-ui-part="column-title"]',
      )!;
      for (const property of ["fontFamily", "fontSize", "fontWeight"] as const)
        expect(getComputedStyle(title)[property]).toBe(
          getComputedStyle(columnTitle)[property],
        );
      const rows = card.querySelectorAll<HTMLElement>(
        ".community-revision-graph__row",
      );
      expect(getComputedStyle(rows[rows.length - 1]!).borderBottomWidth).toBe(
        "0px",
      );
    }
    expect(connector.getBoundingClientRect().height).toBeGreaterThan(0);
    const connectorLine = connector.querySelector("line")!;
    const aligned = () => {
      for (const stem of stack.querySelectorAll<SVGLineElement>(
        '.community-revision-graph__svg line[x1="14"]',
      )) {
        expect(
          Math.abs(
            stem.getBoundingClientRect().left -
              connectorLine.getBoundingClientRect().left,
          ),
        ).toBeLessThan(0.1);
        expect(stem.getAttribute("stroke-width")).toBe(
          connectorLine.getAttribute("stroke-width"),
        );
      }
    };
    await waitFor(aligned);
    await userEvent.click(
      within(stack).getByRole("button", { name: "Expose repository API" }),
    );
    await waitFor(aligned);
    const resize = within(stack).getByRole("separator");
    resize.focus({ preventScroll: true });
    await userEvent.keyboard("{ArrowRight}");
    await waitFor(aligned);
    expect(
      within(stack).queryByRole("button", { name: "Unapply feature/api" }),
    ).toBeNull();
    const group = within(stack).getByRole("group", {
      name: "Branch actions for feature/api",
    });
    expect(
      within(group).getByRole("button", { name: "Push feature/api" }),
    ).toHaveAttribute("aria-disabled", "true");
    await userEvent.click(
      within(group).getByRole("button", {
        name: "Add empty commit to feature/api",
      }),
    );
    await messagePreview(canvasElement, "Empty WIP commit");
    await expect(portal.getByText(/Create an empty commit/)).toBeVisible();
    await applyPreview(canvasElement);
    await expect(
      within(stack).getByRole("button", { name: "Empty WIP commit" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Bookmark action bars hidden"
  args={{ showStackActions: false }}
  play={async ({ canvasElement }) => {
    const stack = await workspaceCanvas(canvasElement).findByRole("region", {
      name: "Stack feature/api",
    });
    expect(stack.querySelector('[data-ui-part="stack-link"]')).not.toBeNull();
    expect(
      within(stack).queryByRole("group", {
        name: "Branch actions for feature/api",
      }),
    ).toBeNull();
    expect(
      within(stack).getByRole("button", { name: "Expose repository API" }),
    ).toBeVisible();
  }}
/>
