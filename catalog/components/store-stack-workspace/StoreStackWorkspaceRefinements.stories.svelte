<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import { workspaceCanvas } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Refinements",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  function header(element: HTMLElement, name: string) {
    return workspaceCanvas(element)
      .getByRole("button", { name: `Show all changes in ${name}` })
      .closest<HTMLElement>('[data-ui-part="card-header"]')!;
  }
  function row(element: HTMLElement, name: string) {
    return workspaceCanvas(element)
      .getByRole("button", { name })
      .closest<HTMLElement>("[data-revision-id]")!;
  }
  async function verifyKebabMenus({
    canvasElement,
  }: {
    canvasElement: HTMLElement;
  }) {
    const canvas = workspaceCanvas(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const branch = within(stack).getByRole("button", {
      name: "Branch actions for feature/api",
    });
    const commit = within(stack).getByRole("button", {
      name: "Actions for Expose repository API",
    });
    expect(
      Math.abs(
        branch.getBoundingClientRect().right -
          commit.getBoundingClientRect().right,
      ),
    ).toBeLessThan(1);
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    const sample = document.createElement("canvas");
    sample.width = sample.height = 1;
    const context = sample.getContext("2d")!;
    function color(element: HTMLElement, background: string) {
      context.clearRect(0, 0, 1, 1);
      context.fillStyle = background;
      context.fillRect(0, 0, 1, 1);
      context.fillStyle = getComputedStyle(element).backgroundColor;
      context.fillRect(0, 0, 1, 1);
      return [...context.getImageData(0, 0, 1, 1).data].slice(0, 3);
    }
    for (const button of [
      branch,
      commit,
      within(stack).getByRole("button", { name: "Actions for feature/api" }),
    ]) {
      await pointer.hover(button);
      await waitFor(() => expect(button.matches(":hover")).toBe(true));
      const parent = button.closest<HTMLElement>(
        '[data-ui-part="column-header"], .stack-workspace__bookmark-actions, [data-revision-id]',
      )!;
      await waitFor(() => {
        const row = color(parent, getComputedStyle(stack).backgroundColor);
        const hover = color(button, `rgb(${row.join(",")})`);
        expect(
          Math.max(
            ...hover.map((value, index) => Math.abs(value - row[index]!)),
          ),
        ).toBeGreaterThanOrEqual(12);
      });
    }
    await pointer.hover(canvasElement, { position: { x: 0, y: 0 } });
  }
</script>

<Story
  name="Canvas starts without top padding"
  play={async ({ canvasElement }) => {
    await workspaceCanvas(canvasElement).findByRole("region", {
      name: "Stack feature/api",
    });
    const canvas = canvasElement.querySelector<HTMLElement>(
      ".stack-workspace__canvas",
    )!;
    expect(getComputedStyle(canvas).paddingTop).toBe("0px");
  }}
/>

<Story
  name="Branch actions differ from adjacent commit hover"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    const commit = row(canvasElement, "Expose repository API");
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    await pointer.hover(commit);
    await waitFor(() => expect(commit.matches(":hover")).toBe(true));
    const actions = canvas.getByRole("group", {
      name: "Branch actions for feature/api",
    });
    const sample = document.createElement("canvas");
    sample.width = sample.height = 1;
    const context = sample.getContext("2d")!;
    function color(element: HTMLElement) {
      context.clearRect(0, 0, 1, 1);
      context.fillStyle = getComputedStyle(element).backgroundColor;
      context.fillRect(0, 0, 1, 1);
      return [...context.getImageData(0, 0, 1, 1).data];
    }
    const bar = color(actions),
      hover = color(commit);
    expect(
      Math.max(
        ...bar
          .slice(0, 3)
          .map((value, index) => Math.abs(value - hover[index]!)),
      ),
    ).toBeGreaterThanOrEqual(3);
    expect(bar[3]).toBe(255);
    expect(hover[3]).toBe(255);
    await pointer.hover(canvasElement, { position: { x: 0, y: 0 } });
  }}
/>

<Story
  name="Branch chips share the available wrapping row"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    const branch = header(canvasElement, "feature/api");
    await userEvent.click(branch);
    const bookmark = within(branch).getByText("Local bookmark", {
      exact: true,
    });
    const group = bookmark.parentElement!;
    expect(
      Math.abs(
        bookmark.getBoundingClientRect().left -
          within(branch)
            .getByRole("button", { name: "Show all changes in feature/api" })
            .getBoundingClientRect().left,
      ),
    ).toBeLessThan(1);
    expect(getComputedStyle(group).flexWrap).toBe("wrap");
    expect(
      Math.abs(
        group.getBoundingClientRect().width -
          group.parentElement!.getBoundingClientRect().width +
          parseFloat(getComputedStyle(group.parentElement!).paddingRight),
      ),
    ).toBeLessThan(1);
  }}
/>

<Story
  name="Expanded branch and commit files use a short heading"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    await userEvent.click(header(canvasElement, "feature/api"));
    const branchFiles = canvas.getByRole("region", {
      name: "Changed files in branch feature/api",
    });
    expect(
      within(branchFiles).getByText("Changed files", { exact: true }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Expose repository API" }),
    );
    const commitFiles = canvas.getByRole("region", {
      name: "Changed files in feature/api",
    });
    expect(
      within(commitFiles).getByText("Changed files", { exact: true }),
    ).toBeVisible();
  }}
/>

<Story
  name="Left markers replace selected chips"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    const branch = header(canvasElement, "feature/api");
    await userEvent.click(branch);
    expect(within(branch).queryByText("Selected", { exact: true })).toBeNull();
    expect(getComputedStyle(branch, "::before").width).toBe("4px");
    await userEvent.click(
      canvas.getByRole("button", { name: "Expose repository API" }),
    );
    const commit = row(canvasElement, "Expose repository API");
    expect(within(commit).queryByText("Selected", { exact: true })).toBeNull();
    expect(getComputedStyle(commit, "::before").width).toBe("4px");
  }}
/>

<Story
  name="Content padding separates components from edge scrollbars"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await userEvent.click(header(canvasElement, "feature/api"));
    await userEvent.click(canvas.getByRole("button", { name: "README.md" }));
    await canvas.findByRole("region", {
      name: "Selected changes from working",
    });
    await userEvent.click(
      within(stack).getByRole("button", { name: "src/api.ts" }),
    );
    await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    function verify() {
      const bodies = canvasElement.querySelectorAll<HTMLElement>(
        '.stack-workspace [data-ui-part="column-body"]',
      );
      expect(bodies.length).toBeGreaterThanOrEqual(2);
      for (const body of bodies) {
        const area = body.querySelector<HTMLElement>(
          ':scope > [data-ui-part="scroll-area"]',
        )!;
        expect(
          body.getBoundingClientRect().right -
            area.getBoundingClientRect().right,
        ).toBeLessThan(1);
        const viewport = area.querySelector<HTMLElement>(
          ':scope > [data-ui-part="scroll-area-viewport"]',
        )!;
        expect(
          viewport.getBoundingClientRect().right -
            viewport.firstElementChild!.getBoundingClientRect().right,
        ).toBeGreaterThanOrEqual(8);
      }
    }
    verify();
    await waitFor(() => {
      const scrollbar = stack.querySelector<HTMLElement>(
        '[data-ui-part="column-body"] [data-ui-part="scroll-area-scrollbar"][data-orientation="vertical"]',
      )!;
      expect(scrollbar).not.toBeNull();
      expect(scrollbar).toBeVisible();
      expect(
        stack.getBoundingClientRect().right -
          scrollbar.getBoundingClientRect().right,
      ).toBeLessThanOrEqual(1);
      const viewport = scrollbar.parentElement!.querySelector<HTMLElement>(
        ':scope > [data-ui-part="scroll-area-viewport"]',
      )!;
      for (const card of stack.querySelectorAll<HTMLElement>(
        ".stack-workspace__bookmark-card, .stack-workspace__stage-card",
      )) {
        const left =
          card.getBoundingClientRect().left -
          viewport.getBoundingClientRect().left;
        const right =
          scrollbar.getBoundingClientRect().left -
          card.getBoundingClientRect().right;
        expect(left).toBeGreaterThanOrEqual(8);
        expect(right).toBeGreaterThanOrEqual(8);
        expect(Math.abs(left - right)).toBeLessThan(1);
      }
    });
    await userEvent.click(canvas.getByRole("tab", { name: /^Branches$/ }));
    await waitFor(() =>
      expect(
        canvasElement.querySelector(
          '[data-column-id="branches"] [data-ui-part="column-body"]',
        ),
      ).not.toBeNull(),
    );
    const branchBodies = canvasElement.querySelectorAll<HTMLElement>(
      '.stack-workspace [data-ui-part="column-body"]',
    );
    expect(branchBodies.length).toBeGreaterThanOrEqual(3);
    for (const body of branchBodies) {
      const viewport = body.querySelector<HTMLElement>(
        ':scope > [data-ui-part="scroll-area"] > [data-ui-part="scroll-area-viewport"]',
      )!;
      expect(getComputedStyle(viewport).paddingInlineEnd).toBe(
        body.closest<HTMLElement>('[data-ui-part="column"]')!.dataset
          .columnId === "branches"
          ? "0px"
          : "8px",
      );
    }
  }}
/>

<Story
  name="Stack columns omit redundant application and base metadata"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    expect(within(stack).queryByText("Applied", { exact: true })).toBeNull();
    expect(within(stack).queryByText(/^Base:/)).toBeNull();
    expect(stack.querySelector(".stack-workspace__lane-meta")).toBeNull();
    await userEvent.click(canvas.getByRole("tab", { name: /^Branches$/ }));
    await userEvent.click(
      canvas.getByRole("option", { name: "feature/search" }),
    );
    const conflictStack = await canvas.findByRole("region", {
      name: "Stack feature/search",
    });
    const columnHeader = conflictStack.querySelector<HTMLElement>(
      '[data-ui-part="column-header"]',
    )!;
    expect(within(columnHeader).getByText("Conflicts")).toBeVisible();
    expect(
      within(conflictStack).queryByText("Not applied", { exact: true }),
    ).toBeNull();
    expect(within(conflictStack).queryByText(/^Base:/)).toBeNull();
  }}
/>

<Story
  name="Kebab menus have distinct hover and aligned branch controls"
  play={verifyKebabMenus}
/>
<Story
  name="Dark kebab menus have distinct hover and aligned branch controls"
  globals={{ colorMode: "dark" }}
  play={verifyKebabMenus}
/>

<Story
  name="Branch and commit chips remain distinct on hover"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    const sample = document.createElement("canvas");
    sample.width = sample.height = 1;
    const context = sample.getContext("2d")!;
    function color(element: HTMLElement) {
      context.clearRect(0, 0, 1, 1);
      context.fillStyle = getComputedStyle(element).backgroundColor;
      context.fillRect(0, 0, 1, 1);
      return [...context.getImageData(0, 0, 1, 1).data];
    }
    for (const target of [
      header(canvasElement, "feature/api"),
      row(canvasElement, "Expose repository API"),
    ]) {
      await pointer.hover(target);
      await waitFor(() => expect(target.matches(":hover")).toBe(true));
      const hover = color(target);
      const chips = target.querySelectorAll<HTMLElement>(
        '[data-ui-part="community-chip"][data-chip-tone="neutral"]',
      );
      expect(chips.length).toBeGreaterThan(0);
      for (const chip of chips) {
        const background = color(chip);
        expect(background[3]).toBe(255);
        expect(
          Math.max(
            ...background
              .slice(0, 3)
              .map((value, index) => Math.abs(value - hover[index]!)),
          ),
        ).toBeGreaterThanOrEqual(12);
      }
    }
    await pointer.hover(canvasElement, { position: { x: 0, y: 0 } });
  }}
/>
