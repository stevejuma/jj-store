<script lang="ts">
  import ChatCommentArea from "./ChatCommentArea.svelte";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import type { ChatBodyItem } from "./model.js";
  let { item, state }: { item: ChatBodyItem; state: ChatBodyComments } =
    $props();
</script>

{#if item.replyContext}
  <ChatCommentArea
    {state}
    threadId={item.replyContext.threadId}
    commentIds={item.replyContext.commentIds}
    placement="reply"
  />
{/if}
