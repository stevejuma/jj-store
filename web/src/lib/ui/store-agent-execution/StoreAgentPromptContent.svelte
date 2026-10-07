<script lang="ts">
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { ActivityRow } from "./presentation.js";
  import { text } from "../../controller/agent-execution-types.js";
  let {
    row,
    members = [],
  }: { row: ActivityRow; members?: readonly CommunityMemberModel[] } = $props();
  const context = $derived(row.kind === "user" ? text(row.value.context) : "");
</script>

{#if context}<CommunityMarkdownContent content={context} promptElements />{/if}
<CommunityMarkdownContent
  class="community-message__content"
  content={row.text}
  promptElements
  {members}
/>
