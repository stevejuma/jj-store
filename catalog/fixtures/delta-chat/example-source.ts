export const chatBodyExample = `<script lang="ts">
  import StoreAgentChatBody from './StoreAgentChatBody.svelte';
  import type { ChatBodyItem, ChatCommentAnchor, ChatCommentThread } from './model';
  let items: ChatBodyItem[] = [];
  let threads = $state<ChatCommentThread[]>([]);
  function add(anchor: ChatCommentAnchor, text: string) {
    const id = crypto.randomUUID();
    threads = [...threads, { id, anchor, comments: [{ id, author: 'You', text }] }];
  }
  function reply(id: string, text: string) {
    threads = threads.map(t => t.id === id ? { ...t, comments: [...t.comments, { id: crypto.randomUUID(), author: 'You', text }] } : t);
  }
</script>
<StoreAgentChatBody {items} {threads} onAddComment={add} onReply={reply} />`;
