<script lang="ts">
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import StoreAgentSentContext from "./StoreAgentSentContext.svelte";
  let {
    content,
    context = "",
    sentContext,
    members = [],
  }: {
    content: string;
    context?: string;
    sentContext?: unknown;
    members?: readonly CommunityMemberModel[];
  } = $props();
</script>

<StoreAgentSentContext value={sentContext} fallback={context} />
<CommunityMarkdownContent
  class="community-message__content"
  {content}
  promptElements
  {members}
/>
