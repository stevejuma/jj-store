<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, within } from "storybook/test";
  import CommunityProjectsRevisionGraphIds from "./CommunityProjectsRevisionGraphIds.svelte";
  import { IDS } from "../../fixtures/repository.js";
  const { Story } = defineMeta({
    title: "JJ Store/Components/Native IDs",
    component: CommunityProjectsRevisionGraphIds,
    tags: ["visual-pending"],
    args: {
      commitId: IDS.merge,
      changeId: "ab".repeat(16),
      placement: "inline",
    },
    parameters: {
      docs: {
        source: {
          code: '<script>\n  import CommunityProjectsRevisionGraphIds from "./catalog/components/community-projects/CommunityProjectsRevisionGraphIds.svelte";\n<\/script>\n<CommunityProjectsRevisionGraphIds {commitId} {changeId} placement="inline" />',
        },
      },
    },
  });
</script>

<Story
  name="Copy complete identity"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    const writeText = fn(async (_value: string) => {});
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText },
    });
    try {
      const commit = canvas.getByRole("button", {
        name: `Copy full commit ID: ${IDS.merge}`,
      });
      await expect(commit).toHaveTextContent(
        `${IDS.merge.slice(0, 8)}…${IDS.merge.slice(-4)}`,
      );
      await userEvent.click(commit);
      await expect(writeText).toHaveBeenLastCalledWith(IDS.merge);
      await expect(commit).toHaveAttribute("data-copied", "true");
      await userEvent.click(
        canvas.getByRole("button", {
          name: `Copy full change ID: ${"ab".repeat(16)}`,
        }),
      );
      await expect(writeText).toHaveBeenLastCalledWith("ab".repeat(16));
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
  }}
/>
<Story
  name="Operation identity"
  args={{ changeId: "", commitId: IDS.opLeft, commitLabel: "operation ID" }}
/>
