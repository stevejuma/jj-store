<script lang="ts">
  import ChatCommentArea from "./ChatCommentArea.svelte";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import type { ChatBodyBlock } from "./model.js";
  let {
    state,
    blocks,
  }: { state: ChatBodyComments; blocks: readonly ChatBodyBlock[] } = $props();
  const threads = $derived(state.forBlocks(blocks));
</script>

{#if threads.length}
  <div class="chat-collapsed-comments">
    {#each threads as thread (thread.id)}<ChatCommentArea
        {state}
        threadId={thread.id}
        placement="summary"
      />{/each}
  </div>
{/if}

<style>
  .chat-collapsed-comments {
    display: grid;
    gap: 0.5rem;
    padding-block-start: 0.5rem;
  }
</style>
