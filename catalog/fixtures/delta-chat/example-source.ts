export const chatBodyExample = `<script lang="ts">
  import StoreAgentChatBody from './StoreAgentChatBody.svelte';
  import ChatNextMessage from './ChatNextMessage.svelte';
  import ChatMessageFooter from './ChatMessageFooter.svelte';
  import { ChatBodyComments } from './comments.svelte';
  import type { ChatBodyItem, ChatPromptSubmission } from './model';
  let items = $state<ChatBodyItem[]>([]);
  const author = { id: 'me', name: 'You', handle: '', initials: 'YO' };
  async function send(submission: ChatPromptSubmission) {
    // Host accepts this exact text/comments snapshot, or throws to retain drafts.
  }
  const comments = new ChatBodyComments(() => ({ items, author, onSend: send }));
</script>
{#snippet composer()}<ChatNextMessage bind:value={comments.message} member={author} onSubmit={() => comments.send()} />{/snippet}
{#snippet footer()}<ChatMessageFooter canSend={comments.canSend} onSend={() => comments.send()} error={comments.error} />{/snippet}
<StoreAgentChatBody {items} commentState={comments} {composer} {footer} />`;
