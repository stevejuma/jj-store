<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, waitFor } from "storybook/test";
  import CommunityMarkdownContent from "../../../web/src/lib/ui/community-markdown-content/CommunityMarkdownContent.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/community-markdown-content/CommunityMarkdownContent.svelte",
      },
    },
    title: "JJ Store/Conversation Markdown",
    component: CommunityMarkdownContent,
    tags: ["visual-pending", "component"],
    args: { content: "A brief inline comment.", fullWidth: true },
  });
</script>

<Story
  name="Embedded content remains compact after late reader styles"
  play={async ({ canvasElement }) => {
    // The live application can load the same Mira base styles after this owned
    // component. Reproduce that order without relying on Storybook's imports.
    const lateStyles = document.createElement("style");
    lateStyles.textContent =
      ".markdown-preview-surface .markdown-view__document { padding-bottom: var(--mira-preview-bottom-padding); }";
    document.head.append(lateStyles);
    try {
      await waitFor(() => {
        const content = canvasElement.querySelector<HTMLElement>(
          ".markdown-view__document",
        )!;
        expect(content).not.toBeNull();
        expect(content.textContent).toContain("A brief inline comment.");
        expect(getComputedStyle(content).paddingBottom).toBe("0px");
        expect(content.getBoundingClientRect().height).toBeLessThan(60);
      });
    } finally {
      lateStyles.remove();
    }
  }}
/>
