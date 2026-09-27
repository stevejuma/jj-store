<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within } from "storybook/test";
  import CommunityChip from "./CommunityChip.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Components/Chip",
    component: CommunityChip,
    tags: ["visual-pending"],
    parameters: {
      docs: {
        source: {
          code: '<script>\n  import { CommunityChip } from "./catalog/index.js";\n<\/script>\n<CommunityChip tone="accent">main</CommunityChip>',
        },
      },
    },
  });
</script>

<Story
  name="Bookmark"
  play={async ({ canvasElement }) => {
    await expect(within(canvasElement).getByText("main")).toBeVisible();
  }}
>
  {#snippet template()}<CommunityChip tone="accent">main</CommunityChip
    >{/snippet}
</Story>
