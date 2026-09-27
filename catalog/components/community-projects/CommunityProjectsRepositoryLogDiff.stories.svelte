<script module lang="ts">
  import { SOURCE_TEXT } from "../../fixtures/repository.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import CommunityProjectsRepositoryLogDiff from "./CommunityProjectsRepositoryLogDiff.svelte";
  import DiffFixture from "../../fixtures/DiffFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Components/Diff",
    component: CommunityProjectsRepositoryLogDiff,
    tags: ["visual-pending"],
    parameters: {
      docs: {
        source: {
          code: '<script>\n  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";\n  import { CommunityProjectsRepositoryLogDiff } from "./catalog/index.js";\n<\/script>\n<ColumnCanvas.Root {controller}>\n  <CommunityProjectsRepositoryLogDiff {path} {patch} {files} />\n</ColumnCanvas.Root>',
        },
      },
    },
  });
</script>

<Story
  name="Safe source rendering"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByText(SOURCE_TEXT, { exact: false })).toBeVisible();
    await expect(
      canvasElement.querySelector(".ui-diff-file-diff script"),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("radio", { name: "Show split diff" }),
    );
    await expect(
      canvasElement.querySelector('[data-view-mode="split"]'),
    ).not.toBeNull();
  }}>{#snippet template()}<DiffFixture />{/snippet}</Story
>

<Story name="Binary file"
  >{#snippet template()}<DiffFixture path="assets/logo.bin" />{/snippet}</Story
>
