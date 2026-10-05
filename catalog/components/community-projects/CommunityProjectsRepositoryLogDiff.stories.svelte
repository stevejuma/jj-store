<script module lang="ts">
  import { SOURCE_TEXT, operations } from "../../fixtures/repository.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import CommunityProjectsRepositoryLogDiff from "../../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte";
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

<Story
  name="Binary file"
  play={async ({ canvasElement }) => {
    await expect(within(canvasElement).getByRole("status")).toHaveTextContent(
      "Binary diff unavailable",
    );
  }}
  >{#snippet template()}<DiffFixture
      path="assets/logo.bin"
      unavailableReason="binary"
    />{/snippet}</Story
>

<Story
  name="Text work limit"
  play={async ({ canvasElement }) => {
    const status = within(canvasElement).getByRole("status");
    await expect(status).toHaveTextContent("Text diff unavailable");
    await expect(status).toHaveTextContent("diff work limit");
    await expect(status).not.toHaveTextContent("Binary");
  }}
  >{#snippet template()}<DiffFixture
      path="long.c"
      unavailableReason="complexity"
    />{/snippet}</Story
>

<Story
  name="Missing comparison"
  play={async ({ canvasElement }) => {
    const status = within(canvasElement).getByRole("status");
    await expect(status).toHaveTextContent("No text diff is available");
    await expect(status).not.toHaveTextContent("Binary");
  }}>{#snippet template()}<DiffFixture path="missing.c" />{/snippet}</Story
>

<Story
  name="Pending comparison"
  play={async ({ canvasElement }) => {
    const status = within(canvasElement).getByRole("status", {
      name: "Loading selected comparison",
    });
    await expect(status).toHaveAttribute("aria-busy", "true");
    await expect(
      within(canvasElement).queryByText("Text diff unavailable"),
    ).toBeNull();
  }}>{#snippet template()}<DiffFixture path="long.c" loading />{/snippet}</Story
>

<Story
  name="No file selected"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const empty = canvas.getByRole("status");
    await expect(empty).toHaveTextContent("No file selected");
    await expect(
      empty.querySelector('[data-ui-part="empty-media"] svg'),
    ).not.toBeNull();
  }}>{#snippet template()}<DiffFixture path={null} />{/snippet}</Story
>

<Story
  name="Operation text work limit"
  play={async ({ canvasElement }) => {
    const status = within(canvasElement).getByRole("status");
    await expect(status).toHaveTextContent("Text diff unavailable");
    await expect(status).toHaveTextContent("diff work limit");
    await expect(status).not.toHaveTextContent("Binary");
  }}
  >{#snippet template()}<DiffFixture
      path="long.c"
      unavailableReason="complexity"
      operation={operations.revisions[0]}
    />{/snippet}</Story
>
