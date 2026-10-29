<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StackWorkspaceApplicationFixture from "../../fixtures/StackWorkspaceApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Target branch",
    component: StackWorkspaceApplicationFixture,
    args: { page: "branches" },
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
  });
</script>

<Story
  name="Target header uses the stack card layout"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const branches = await canvas.findByRole("region", { name: "Branches" });
    await userEvent.click(
      within(branches).getByRole("option", { name: "main" }),
    );
    const stack = await canvas.findByRole("region", { name: "Stack main" });
    await within(stack).findByRole("button", { name: "Main history 240" });
    const header = stack.querySelector<HTMLElement>(
      ".stack-workspace__bookmark-header",
    )!;
    const icon = header.querySelector<SVGElement>(":scope > svg")!;
    const title = header.querySelector<HTMLElement>(
      '[data-slot="card-title"]',
    )!;
    const description = header.querySelector<HTMLElement>(
      '[data-slot="card-description"]',
    )!;
    const chips = header.querySelector<HTMLElement>(
      ".stack-workspace__branch-chips",
    )!;
    expect(
      getComputedStyle(header).gridTemplateColumns.split(" "),
    ).toHaveLength(2);
    expect(title.getBoundingClientRect().left).toBeGreaterThanOrEqual(
      icon.getBoundingClientRect().right,
    );
    expect(title.getBoundingClientRect().left).toBeCloseTo(
      description.getBoundingClientRect().left,
      1,
    );
    expect(title.getBoundingClientRect().left).toBeCloseTo(
      chips.getBoundingClientRect().left,
      1,
    );
    const box = header.getBoundingClientRect();
    expect(title.getBoundingClientRect().right).toBeLessThanOrEqual(box.right);
    expect(title.getBoundingClientRect().width).toBeGreaterThan(0);
    const iconBox = icon.getBoundingClientRect();
    const border = parseFloat(getComputedStyle(header).borderBottomWidth);
    expect(iconBox.top + iconBox.height / 2).toBeCloseTo(
      box.top + (box.height - border) / 2,
      1,
    );
    const avatar = stack.querySelector<HTMLElement>(
      ".community-revision-graph-copy > [data-ui-part='git-author']",
    )!;
    expect(title.getBoundingClientRect().left).toBeCloseTo(
      avatar.getBoundingClientRect().left,
      1,
    );
  }}
/>

<Story
  name="Workspace target supports keyboard selection above the filters"
  args={{ targetBranch: "origin/trunk" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const branches = await canvas.findByRole("region", { name: "Branches" });
    const target = () =>
      within(branches).getByRole("option", { name: "origin/trunk" });
    const search = within(branches).getByPlaceholderText("Search branches");
    await userEvent.click(
      within(branches).getByRole("tab", { name: "Conflicted" }),
    );
    expect(target()).toBeVisible();
    await userEvent.type(search, "missing-branch");
    expect(target()).toBeVisible();
    await userEvent.keyboard("{ArrowDown}{Enter}");
    const stack = await canvas.findByRole("region", {
      name: "Stack origin/trunk",
    });
    expect(target()).toHaveAttribute("data-current", "true");
    const commit = await within(stack).findByRole("button", {
      name: "origin/trunk history 240",
    });
    commit.focus();
    await userEvent.keyboard("{Enter}");
    expect(
      await within(stack).findByRole("region", {
        name: "Changed files in origin/trunk",
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="Target branch opens its own commit stack"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const column = await canvas.findByRole("region", { name: "Branches" });
    const target = within(column).getByRole("option", { name: "main" });
    expect(target.getBoundingClientRect().bottom).toBeLessThanOrEqual(
      within(column).getByRole("tab", { name: "All" }).getBoundingClientRect()
        .top,
    );
    await userEvent.click(target);
    const stack = await canvas.findByRole("region", { name: "Stack main" });
    expect(canvas.getAllByRole("region", { name: /^Stack / })).toHaveLength(1);
    const commit = await within(stack).findByRole("button", {
      name: "Main history 240",
    });
    await userEvent.click(commit);
    const files = await within(stack).findByRole("region", {
      name: "Changed files in main",
    });
    expect(
      commit.closest(".community-revision-graph__item")!.contains(files),
    ).toBe(true);
    await userEvent.click(within(files).getByText("entry-240.ts"));
    const diff = await canvas.findByRole("region", {
      name: "Selected changes from target-history",
    });
    expect(diff.previousElementSibling).toBe(stack);
    expect(within(diff).getByText("@@ -1,1 +1,1 @@")).toBeVisible();
    expect(
      within(diff).queryByRole("button", { name: "Split selected changes" }),
    ).toBeNull();
    expect(stack.querySelector('[draggable="true"]')).toBeNull();
    await userEvent.click(
      within(diff).getByRole("button", { name: "Close Diff column" }),
    );
    await userEvent.click(
      within(column).getByRole("option", { name: "feature/api" }),
    );
    expect(
      await canvas.findByRole("region", { name: "Stack feature/api" }),
    ).toBeVisible();
    expect(canvas.queryByRole("region", { name: "Stack main" })).toBeNull();
    await userEvent.click(target);
    await waitFor(() =>
      expect(
        within(canvas.getByRole("region", { name: "Stack main" })).getByRole(
          "region",
          { name: "Changed files in main" },
        ),
      ).toBeVisible(),
    );
  }}
/>

<Story
  name="Large target history loads lazily on scroll"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const branches = await canvas.findByRole("region", { name: "Branches" });
    await userEvent.click(
      within(branches).getByRole("option", { name: "main" }),
    );
    const stack = await canvas.findByRole("region", { name: "Stack main" });
    const card = stack.querySelector<HTMLElement>("[data-loaded-commits]")!;
    await waitFor(() =>
      expect(card).toHaveAttribute("data-loaded-commits", "40"),
    );
    const viewport = stack.querySelector<HTMLElement>(
      '[data-ui-part="column-body"] > [data-ui-part="scroll-area"] > [data-ui-part="scroll-area-viewport"]',
    )!;
    expect(viewport.scrollHeight).toBeGreaterThan(viewport.clientHeight);
    for (const count of [80, 120, 160, 200, 240]) {
      viewport.scrollTop = viewport.scrollHeight;
      viewport.dispatchEvent(new Event("scroll"));
      await waitFor(() =>
        expect(card).toHaveAttribute("data-loaded-commits", String(count)),
      );
    }
    expect(stack.querySelectorAll("[data-revision-id]").length).toBeLessThan(
      40,
    );
    viewport.scrollTop = viewport.scrollHeight;
    viewport.dispatchEvent(new Event("scroll"));
    await userEvent.click(
      await within(stack).findByRole("button", { name: "Main history 1" }),
    );
    const files = await within(stack).findByRole("region", {
      name: "Changed files in main",
    });
    expect(within(files).getByText("entry-1.ts")).toBeVisible();
    viewport.scrollTop = 0;
    viewport.dispatchEvent(new Event("scroll"));
    expect(
      await within(stack).findByRole("button", { name: "Main history 240" }),
    ).toBeVisible();
    expect(card).toHaveAttribute("data-loaded-commits", "240");
  }}
/>
