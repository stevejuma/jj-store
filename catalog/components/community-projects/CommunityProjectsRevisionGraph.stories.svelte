<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import CommunityProjectsRevisionGraph from "../../../web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte";
  import { fixtureGraph, NOW, operations } from "../../fixtures/repository.js";
  const gitCommit = "87e1cd5db3c0a551cd4a4feedde962ca63368fb9";
  const describedCommit = "20d6c008366360bd7c1c97ac99be03a8348cef7c";
  const gitOperations = {
    ...operations,
    revisions: operations.revisions.map((revision, index) => ({
      ...revision,
      message:
        index === 0
          ? `create bookmark main pointing to commit ${gitCommit}`
          : index === 1
            ? `describe commit ${describedCommit}`
            : revision.message,
    })),
  };
  const longBookmark =
    "jj-store/workspaces/" + "workspace-identity/".repeat(10) + "wip";
  const longLabels = (operation: boolean) => ({
    ...(operation ? operations : fixtureGraph()),
    revisions: [
      {
        ...(operation
          ? operations.revisions[0]!
          : fixtureGraph().revisions[0]!),
        bookmarks: [
          {
            name: longBookmark,
            added: true,
            removed: false,
            conflicted: false,
          },
        ],
        workspaces: ["workspace-".repeat(30)],
      },
    ],
    ...(operation
      ? { commitLabel: "operation ID", graphLabel: "Operation graph" }
      : {}),
  });
  async function boundedChips(element: HTMLElement) {
    await waitFor(() =>
      expect(
        element.querySelector(".community-revision-graph-copy__chips"),
      ).not.toBeNull(),
    );
    const chips = element.querySelector<HTMLElement>(
      ".community-revision-graph-copy__chips",
    )!;
    const bounds = chips.getBoundingClientRect();
    for (const chip of chips.querySelectorAll<HTMLElement>(
      '[data-ui-part="community-chip"]',
    )) {
      const rect = chip.getBoundingClientRect();
      expect(rect.width).toBeLessThanOrEqual(256);
      expect(rect.right).toBeLessThanOrEqual(bounds.right + 1);
    }
    const bookmark = within(chips).getByText(longBookmark);
    expect(bookmark.scrollWidth).toBeGreaterThan(bookmark.clientWidth);
    expect(getComputedStyle(bookmark).textOverflow).toBe("ellipsis");
    expect(
      bookmark
        .closest('[data-ui-part="community-chip"]')
        ?.getAttribute("title"),
    ).toContain(longBookmark);
  }
  const linear = Array.from({ length: 160 }, (_, index) => ({
    ...fixtureGraph().revisions[0]!,
    eventId: (index + 1).toString(16).padStart(128, "0"),
    changeId: (index + 1).toString(16).padStart(32, "0"),
    parents: index ? [index.toString(16).padStart(128, "0")] : [],
    message: `Native revision ${index + 1}`,
    createdAt: NOW - index,
  })).reverse();
  const wideBase = { ...linear.at(-1)!, message: "Shared base" };
  const wide = [
    ...Array.from({ length: 32 }, (_, index) => ({
      ...wideBase,
      eventId: (index + 500).toString(16).padStart(128, "0"),
      changeId: (index + 500).toString(16).padStart(32, "0"),
      parents: [wideBase.eventId],
      message: `Independent branch ${index + 1}`,
    })),
    wideBase,
  ];
  const { Story } = defineMeta({
    title: "JJ Store/Components/Revision graph",
    component: CommunityProjectsRevisionGraph,
    tags: ["visual-pending"],
    args: { ...fixtureGraph(), now: NOW },
    parameters: {
      docs: {
        source: {
          code: '<script>\n  import { CommunityProjectsRevisionGraph } from "./catalog/index.js";\n<\/script>\n<CommunityProjectsRevisionGraph {revisions} {devices} {selectedId} {now} {onSelect} />',
        },
      },
    },
  });
</script>

<Story
  name="JJ Log long chips stay within the row"
  args={longLabels(false)}
  play={async ({ canvasElement }) => boundedChips(canvasElement)}
>
  {#snippet template(args)}<div class="wide-graph-scroll">
      <CommunityProjectsRevisionGraph {...args} />
    </div>{/snippet}
</Story>
<Story
  name="Op Log long chips stay within the row"
  args={longLabels(true)}
  play={async ({ canvasElement }) => boundedChips(canvasElement)}
>
  {#snippet template(args)}<div class="wide-graph-scroll">
      <CommunityProjectsRevisionGraph {...args} />
    </div>{/snippet}
</Story>

<Story
  name="Paged loading uses a skeleton"
  args={{ loading: true }}
  play={async ({ canvasElement }) => {
    const edge = canvasElement.querySelector<HTMLElement>(
      ".community-projects-virtual-rows__edge",
    )!;
    await expect(edge.querySelector('[data-slot="skeleton"]')).toBeVisible();
    await expect(within(edge).getByText("Loading more history")).toHaveClass(
      "sr-only",
    );
    await expect(edge).toHaveAttribute("aria-busy", "true");
  }}
/>

<Story
  name="Fork merge and convergence"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const nativeRow = canvas
      .getByText("Stream immutable native objects")
      .closest<HTMLElement>(".community-revision-graph__row")!;
    for (const [label, tone] of [
      ["Divergent", "warning"],
      ["Conflicted", "danger"],
      ["feature/native-objects", "accent"],
      ["feature/native-objects@origin", "neutral"],
    ] as const) {
      await expect(
        within(nativeRow)
          .getByText(label)
          .closest('[data-ui-part="community-chip"]'),
      ).toHaveAttribute("data-chip-tone", tone);
    }
    const workingCopyRow = canvas
      .getAllByText("Working copy")
      .find((element) =>
        element.classList.contains("community-revision-graph-message__text"),
      )!
      .closest<HTMLElement>(".community-revision-graph__row")!;
    await expect(
      within(workingCopyRow)
        .getAllByText("Working copy")
        .find((element) => element.closest('[data-ui-part="community-chip"]'))
        ?.closest('[data-ui-part="community-chip"]'),
    ).toHaveAttribute("data-chip-tone", "success");
  }}
/>
<Story
  name="Operation siblings"
  args={{
    ...operations,
    commitLabel: "operation ID",
    graphLabel: "Operation graph",
  }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByText("head 1").closest('[data-ui-part="community-chip"]'),
    ).toHaveAttribute("data-chip-tone", "accent");
    await expect(
      canvas
        .getByText("Working copy")
        .closest('[data-ui-part="community-chip"]'),
    ).toHaveAttribute("data-chip-tone", "success");
  }}
/>
<Story
  name="Virtualized selection"
  args={{ revisions: linear, devices: [], selectedId: linear[120]!.eventId }}
  play={async ({ canvasElement }) => {
    await waitFor(() =>
      expect(
        canvasElement.querySelector(
          `[data-revision-id="${linear[120]!.eventId}"]`,
        ),
      ).not.toBeNull(),
    );
    await expect(
      canvasElement.querySelectorAll('[data-ui-part="virtual-row"]').length,
    ).toBeLessThan(160);
    await expect(
      canvasElement.querySelector('[aria-current="true"]'),
    ).toBeVisible();
  }}
>
  {#snippet template(args)}<div class="graph-scroll">
      <CommunityProjectsRevisionGraph {...args} />
    </div>{/snippet}
</Story>

<Story
  name="Wide graph keeps row content readable"
  args={{ revisions: wide, devices: [], onSelect: fn() }}
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement);
    const viewport =
      canvasElement.querySelector<HTMLElement>(".wide-graph-scroll")!;
    await waitFor(() => {
      const avatars = [
        ...canvasElement.querySelectorAll('[data-ui-part="git-author"]'),
      ];
      expect(avatars.length).toBeGreaterThan(2);
      const xs = avatars.map((avatar) => avatar.getBoundingClientRect().left);
      expect(Math.max(...xs) - Math.min(...xs)).toBeGreaterThan(10);
    });
    await expect(viewport.scrollWidth).toBeGreaterThan(viewport.clientWidth);
    const first = canvas.getByRole("button", {
      name: "Independent branch 1",
    });
    await expect(
      first.closest(".community-revision-graph-message")!.clientWidth,
    ).toBeGreaterThan(200);
    await expect(first.scrollWidth).toBeLessThanOrEqual(first.clientWidth);
    first.focus();
    await userEvent.keyboard("{Enter}");
    await expect(args.onSelect).toHaveBeenLastCalledWith(wide[0]);
    viewport.scrollLeft = 120;
    await waitFor(() => expect(viewport.scrollLeft).toBeGreaterThan(0));
    await expect(viewport.scrollTop).toBe(0);
    viewport.scrollTop = 600;
    await waitFor(() => expect(viewport.scrollTop).toBeGreaterThan(0));
    await expect(
      canvasElement.querySelectorAll('[data-ui-part="virtual-row"]').length,
    ).toBeLessThan(wide.length);
  }}
>
  {#snippet template(args)}<div class="graph-scroll wide-graph-scroll">
      <CommunityProjectsRevisionGraph {...args} />
    </div>{/snippet}
</Story>

<Story
  name="Git-backed operation message copies"
  args={{
    ...gitOperations,
    commitLabel: "operation ID",
    graphLabel: "Operation graph",
    onSelect: fn(),
  }}
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement);
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    const write = fn(async (_value: string) => {});
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      for (const commit of [gitCommit, describedCommit]) {
        const button = canvas.getByRole("button", {
          name: `Copy full commit ID: ${commit}`,
        });
        await expect(
          within(button).getByText(`${commit.slice(0, 8)}…${commit.slice(-4)}`),
        ).toBeVisible();
        await userEvent.click(button);
        await expect(write).toHaveBeenLastCalledWith(commit);
        await expect(button).toHaveAttribute("data-copied", "true");
        await expect(args.onSelect).not.toHaveBeenCalled();
      }
      const button = canvas.getByRole("button", {
        name: `Copy full commit ID: ${gitCommit}`,
      });
      button.focus();
      await userEvent.keyboard("{Enter}");
      await expect(write).toHaveBeenLastCalledWith(gitCommit);
      await expect(write).toHaveBeenCalledTimes(3);
      await expect(args.onSelect).not.toHaveBeenCalled();
      await userEvent.click(
        canvas.getByRole("button", {
          name: "create bookmark main pointing to commit",
        }),
      );
      await expect(args.onSelect).toHaveBeenCalledTimes(1);
      await expect(args.onSelect).toHaveBeenLastCalledWith(
        gitOperations.revisions[0],
      );
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
  }}
/>

<style>
  .graph-scroll {
    height: 24rem;
    overflow: auto;
  }

  .wide-graph-scroll {
    width: 24rem;
    max-width: 100%;
  }
</style>
