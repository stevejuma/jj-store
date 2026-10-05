<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Documents from "../../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryDocuments.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Repository document previews",
    component: Documents,
    tags: ["visual-pending"],
    args: {
      currentPath: "",
      previewDocuments: [
        "README.md",
        "CODE_OF_CONDUCT.md",
        "CONTRIBUTING.md",
        "LICENSE",
        "SECURITY.md",
      ].map((path) => ({
        path,
        contents:
          "# " +
          path +
          "\n\nUse `pnpm dev:restart` with **native** [storage](https://example.test/storage).\n\n```sh\npnpm dev:restart\n```",
      })),
    },
  });
</script>

<Story
  name="All document tabs use the shared Markdown preview"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    for (const name of [
      "README",
      "Code of conduct",
      "Contributing",
      "License",
      "Security",
    ]) {
      await userEvent.click(c.getByRole("tab", { name }));
      const preview = c.getByRole("region", {
        name: "Repository document preview",
      });
      await expect(
        preview.querySelector(".mira-markdown-preview"),
      ).not.toBeNull();
      await expect(preview.querySelector("p code")).toHaveTextContent(
        "pnpm dev:restart",
      );
      await expect(
        within(preview).getByRole("link", { name: "storage" }),
      ).toHaveAttribute("href", "https://example.test/storage");
      await expect(preview.querySelector("strong")).toHaveTextContent("native");
      const tab = c.getByRole("tab", { name });
      const indicator = tab.querySelector<HTMLElement>(
        ".community-projects__repository-document-tab-indicator",
      )!;
      await expect(
        indicator.getBoundingClientRect().height,
      ).toBeGreaterThanOrEqual(3);
    }
  }}
/>
