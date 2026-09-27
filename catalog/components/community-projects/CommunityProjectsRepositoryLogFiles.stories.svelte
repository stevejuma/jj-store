<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, within } from "storybook/test";
  import CommunityProjectsRepositoryLogFiles from "../../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte";
  import FilesFixture from "../../fixtures/FilesFixture.svelte";
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
  name="Error"
  args={{
    files: [],
    error: "Fixture file inventory is unavailable.",
    onSelectPath: fn(),
  }}
/>
