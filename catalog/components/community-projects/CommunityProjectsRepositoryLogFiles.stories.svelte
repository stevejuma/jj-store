<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import CommunityProjectsRepositoryLogFiles from "../../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte";
  import FilesFixture from "../../fixtures/FilesFixture.svelte";
  import { sampleDetail } from "../../fixtures/repository.js";
  const { Story } = defineMeta({
    title: "JJ Store/Components/Changed files",
    component: CommunityProjectsRepositoryLogFiles,
    tags: ["visual-pending"],
    parameters: {
      docs: {
        source: {
          code: '<script>\n  import { CommunityProjectsRepositoryLogFiles } from "./catalog/index.js";\n<\/script>\n<CommunityProjectsRepositoryLogFiles {files} {selectedPath} {onSelectPath} />',
        },
      },
    },
  });
</script>

<Story
  name="Tree and list"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("radio", { name: "Show files as list" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "src/lib.rs" }));
    await expect(
      canvas.getByRole("button", { name: "src/lib.rs" }),
    ).toHaveAttribute("aria-current", "true");
    await userEvent.click(
      canvas.getByRole("radio", { name: "Show files as folders" }),
    );
    await expect(canvas.getByRole("button", { name: "src" })).toBeVisible();
  }}>{#snippet template()}<FilesFixture />{/snippet}</Story
>

<Story name="Empty" args={{ files: [], onSelectPath: fn() }} />
<Story
  name="Paged"
  args={{
    files: sampleDetail.files,
    compact: true,
    hasMore: true,
    onSelectPath: fn(),
    onLoadMore: fn(),
  }}
  play={async ({ canvasElement, args }) => {
    const body = canvasElement.querySelector<HTMLElement>(
      ".community-revision-graph-files__body",
    );
    if (!body) throw new Error("Expected the changed-file scroll body");
    body.scrollTop = body.scrollHeight;
    body.dispatchEvent(new Event("scroll"));
    await waitFor(() => expect(args.onLoadMore).toHaveBeenCalledTimes(1));
  }}
/>
<Story
  name="Error"
  args={{
    files: [],
    error: "Fixture file inventory is unavailable.",
    onSelectPath: fn(),
  }}
/>
