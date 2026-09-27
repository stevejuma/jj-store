<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, waitFor } from "storybook/test";
  import CommunityProjectsRevisionGraph from "../../../web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte";
  import { fixtureGraph, NOW, operations } from "../../fixtures/repository.js";
  const linear = Array.from({ length: 160 }, (_, index) => ({
    ...fixtureGraph().revisions[0]!,
    eventId: (index + 1).toString(16).padStart(128, "0"),
    changeId: (index + 1).toString(16).padStart(32, "0"),
    parents: index ? [index.toString(16).padStart(128, "0")] : [],
    message: `Native revision ${index + 1}`,
    createdAt: NOW - index,
  })).reverse();
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

<Story name="Fork merge and convergence" />
<Story
  name="Operation siblings"
  args={{
    ...operations,
    commitLabel: "operation ID",
    graphLabel: "Operation graph",
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

<style>
  .graph-scroll {
    height: 24rem;
    overflow: auto;
  }
</style>
