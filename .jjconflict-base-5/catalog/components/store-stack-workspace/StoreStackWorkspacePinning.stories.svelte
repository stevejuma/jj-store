<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import {
    dropWorkspaceChange,
    workspaceCanvas,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Presentation",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Unstaged header and styled canvas"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const column = await canvas.findByRole("region", {
      name: "Unstaged files",
    });
    expect(within(column).getByText("Unstaged")).toBeVisible();
    const header = column.querySelector<HTMLElement>(
      '[data-ui-part="column-header"]',
    )!;
    const body = column.querySelector<HTMLElement>(
      '[data-ui-part="column-body"]',
    )!;
    const center =
      header.getBoundingClientRect().top +
      header.getBoundingClientRect().height / 2;
    for (const child of header.children) {
      const rect = child.getBoundingClientRect();
      expect(Math.abs(rect.top + rect.height / 2 - center)).toBeLessThan(1);
    }
    expect(
      within(header).getByRole("group", {
        name: "File listing view mode",
      }),
    ).toBeVisible();
    expect(
      within(header).getByRole("button", { name: "Browse stacks to apply" }),
    ).toBeVisible();
    expect(
      body.querySelector('[data-ui-component="file-listing-view-mode-toggle"]'),
    ).toBeNull();
    expect(
      within(body).getByRole("button", { name: "README.md" }),
    ).toBeVisible();
    expect(body.querySelector("p")).toBeNull();
    expect(
      within(header).queryByRole("group", { name: "Working file scope" }),
    ).toBeNull();
    expect(column).toHaveAttribute("data-fixed", "true");
    await userEvent.click(
      within(body).getByRole("button", { name: "README.md" }),
    );
    await canvas.findByRole("region", {
      name: "Selected changes from working",
    });
    for (const col of canvasElement.querySelectorAll<HTMLElement>(
      '[data-ui-part="column"]',
    )) {
      const handle = within(col).getByRole("separator");
      const before = col.getBoundingClientRect().width;
      handle.focus({ preventScroll: true });
      await userEvent.keyboard("{ArrowRight}");
      await waitFor(() =>
        expect(col.getBoundingClientRect().width).toBe(before + 8),
      );
      await userEvent.keyboard("{ArrowLeft}");
      await waitFor(() =>
        expect(col.getBoundingClientRect().width).toBe(before),
      );
    }
    const area = canvasElement.querySelector<HTMLElement>(
      ".stack-workspace__canvas-scroll",
    )!;
    expect(area.dataset.scrollStrategy).toBe("styled");
    const viewport = area.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    await waitFor(() =>
      expect(viewport.scrollWidth).toBeGreaterThan(viewport.clientWidth),
    );
    const width = column.getBoundingClientRect().width;
    const assertPinned = async (element: HTMLElement) => {
      await waitFor(() =>
        expect(element).toHaveAttribute("data-fixed-state", "stuck"),
      );
      expect(
        Math.abs(
          element.getBoundingClientRect().left -
            viewport.getBoundingClientRect().left,
        ),
      ).toBeLessThanOrEqual(1);
    };
    viewport.scrollTo({ left: viewport.scrollWidth, behavior: "auto" });
    await assertPinned(column);
    expect(column.getBoundingClientRect().width).toBe(width);
    expect(
      canvasElement.querySelector('[data-sticky-for="working"]'),
    ).toBeNull();
    const source = within(column).getByRole("button", { name: "README.md" });
    expect(source).toBeVisible();
    // Files stay clickable while the later stacks are in view and their diff
    // remains immediately after Unstaged in the original canvas sequence.
    await userEvent.click(source);
    const diff = await canvas.findByRole("region", {
      name: "Selected changes from working",
    });
    expect(column.nextElementSibling).toBe(diff);
    const stack = canvas.getByRole("region", {
      name: "Stack feature/workspace",
    });
    await dropWorkspaceChange(
      source,
      within(stack).getByRole("button", {
        name: "Assign to feature/workspace",
      }),
    );
    const form = await within(stack).findByRole("form", {
      name: "Create commit in feature/workspace",
    });
    expect(
      within(form).getByRole("button", { name: "README.md" }),
    ).toBeVisible();
    await userEvent.click(within(form).getByRole("button", { name: "Cancel" }));
    await userEvent.click(
      within(column).getByRole("button", { name: "Collapse Unstaged" }),
    );
    const expand = await canvas.findByRole("button", {
      name: "Expand Unstaged column",
    });
    const rail = expand.closest<HTMLElement>(
      '[data-ui-part="collapsed-column"]',
    )!;
    viewport.scrollTo({ left: viewport.scrollWidth, behavior: "auto" });
    await assertPinned(rail);
    const railWidth = rail.getBoundingClientRect().width;
    expect(railWidth).toBeLessThan(width);
    await userEvent.click(expand);
    const expanded = await canvas.findByRole("region", {
      name: "Unstaged files",
    });
    viewport.scrollTo({ left: viewport.scrollWidth, behavior: "auto" });
    await assertPinned(expanded);
    expect(expanded.getBoundingClientRect().width).toBe(width);
    expect(
      within(expanded).getByRole("button", { name: "README.md" }),
    ).toBeVisible();
    for (const area of canvasElement.querySelectorAll<HTMLElement>(
      ".stack-workspace__canvas-scroll [data-scroll-strategy]",
    ))
      expect(area.dataset.scrollStrategy).toBe("styled");
    expect(
      expanded
        .querySelector<HTMLElement>('[data-ui-part="column-body"]')!
        .getBoundingClientRect().height,
    ).toBeGreaterThan(100);
  }}
/>
