<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, within } from "storybook/test";
  import CommunityProjectsRevisionGraphIds from "../../../web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import { IDS } from "../../fixtures/repository.js";
  const demoCommitId = `c1346134${"a".repeat(56)}`;
  const demoChangeId = `yrlvqtlmolyz${"b".repeat(20)}`;
  const { Story } = defineMeta({
    title: "JJ Store/Components/Native IDs",
    component: CommunityProjectsRevisionGraphIds,
    tags: ["visual-pending"],
    args: {
      commitId: demoCommitId,
      changeId: demoChangeId,
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
    const body = within(canvasElement.ownerDocument.body);
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    const writeText = fn(async (_value: string) => {});
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText },
    });
    try {
      const commit = canvas.getByRole("button", {
        name: `Copy full commit ID: ${demoCommitId}`,
      });
      const split = canvas.getByRole("group", {
        name: "Native revision identifiers",
      });
      await expect(split).toBeVisible();
      await expect(commit).toHaveTextContent("c1346134");
      await expect(
        canvas.getByRole("button", {
          name: `Copy full change ID: ${demoChangeId}`,
        }),
      ).toHaveTextContent("yrlvqtlmolyz");
      await expect(split.querySelectorAll("svg")).toHaveLength(2);
      await userEvent.hover(commit);
      await expect(await body.findByText(demoCommitId)).toBeVisible();
      await userEvent.unhover(commit);
      await userEvent.click(commit);
      await expect(writeText).toHaveBeenLastCalledWith(demoCommitId);
      await expect(commit).toHaveAttribute("data-copied", "true");
      const change = canvas.getByRole("button", {
        name: `Copy full change ID: ${demoChangeId}`,
      });
      await userEvent.hover(change);
      await expect(await body.findByText(demoChangeId)).toBeVisible();
      await userEvent.click(change);
      await expect(writeText).toHaveBeenLastCalledWith(demoChangeId);
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
