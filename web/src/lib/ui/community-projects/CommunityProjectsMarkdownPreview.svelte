<script lang="ts">
  import { CommunityMarkdownContent } from "@lapismd/lapis-community/components";
  let {
    content,
    label = "Markdown preview",
  }: { content: string; label?: string } = $props();
</script>

<section
  class="community-projects__markdown-preview"
  data-ui-component="community-markdown-preview"
  aria-label={label}
>
  {#if content.trim()}<CommunityMarkdownContent {content} fullWidth />{:else}<p
      class="community-projects__status"
    >
      This file is empty.
    </p>{/if}
</section>
