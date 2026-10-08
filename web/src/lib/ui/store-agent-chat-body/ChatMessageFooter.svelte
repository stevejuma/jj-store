<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ArrowUpIcon from "@lucide/svelte/icons/arrow-up";
  let {
    canSend,
    onSend,
    error = "",
  }: { canSend: boolean; onSend: () => void; error?: string } = $props();
</script>

<div class="chat-message-footer" role="group" aria-label="Message actions">
  {#if error}<p role="alert">{error}</p>{/if}
  <span>⌘ / Ctrl + Enter</span>
  <Button
    size="sm"
    disabled={!canSend}
    onclick={onSend}
    aria-label="Send message"><ArrowUpIcon size={16} />Send</Button
  >
</div>

<style>
  .chat-message-footer {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0.75rem;
  }
  span {
    font-size: 0.75rem;
    color: var(--muted-foreground);
  }
</style>
