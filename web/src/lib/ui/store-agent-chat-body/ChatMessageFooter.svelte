<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import SendIcon from "@lucide/svelte/icons/send-horizontal";
  import QueueIcon from "@lucide/svelte/icons/arrow-up";
  import StopIcon from "@lucide/svelte/icons/square";
  import type { Snippet } from "svelte";
  let {
    canSend,
    hasContent = canSend,
    sendPending = false,
    onSend,
    onStop,
    stopping = false,
    error = "",
    controls,
    workspace,
    actions,
  }: {
    canSend: boolean;
    hasContent?: boolean;
    sendPending?: boolean;
    onSend: () => void;
    onStop?: () => void;
    stopping?: boolean;
    error?: string;
    controls?: Snippet;
    workspace?: Snippet;
    actions?: Snippet;
  } = $props();
  const stopSelected = $derived(!!onStop && !hasContent);
  const sendLabel = $derived(onStop ? "Queue message" : "Send message");
</script>

<div class="chat-message-footer" role="group" aria-label="Message actions">
  {#if error}<p role="alert">{error}</p>{/if}
  {#if actions}<div class="chat-message-footer__actions">
      {@render actions()}
    </div>{/if}
  {@render workspace?.()}
  {@render controls?.()}
  {#if onStop && hasContent && sendPending}
    <Button
      size="icon-sm"
      variant="destructive"
      disabled={stopping}
      onclick={onStop}
      aria-label={stopping ? "Stopping agent" : "Stop agent"}
      title="Stop active work"><StopIcon aria-hidden="true" /></Button
    >
  {/if}
  <Button
    size="icon-sm"
    class={stopSelected ? undefined : "chat-message-footer__send"}
    variant={stopSelected ? "destructive" : "default"}
    disabled={stopSelected ? stopping : !canSend}
    onclick={stopSelected ? onStop : onSend}
    aria-label={stopSelected
      ? stopping
        ? "Stopping agent"
        : "Stop agent"
      : sendLabel}
    title={stopSelected
      ? "Stop active work"
      : `${sendLabel} (⌘ / Ctrl + Enter)`}
    >{#if stopSelected}<StopIcon
        aria-hidden="true"
      />{:else if onStop}<QueueIcon aria-hidden="true" />{:else}<SendIcon
        aria-hidden="true"
      />{/if}</Button
  >
</div>

<style>
  .chat-message-footer {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0.75rem;
    container: chat-footer / inline-size;
  }
  .chat-message-footer__actions {
    flex: 1 0 auto;
    min-width: 0;
    max-width: 100%;
  }
  .chat-message-footer :global(.chat-message-footer__send) {
    border-radius: 50%;
  }
  @media (hover: hover) {
    .chat-message-footer :global(.conversation-context-trigger:hover) {
      background-color: color-mix(
        in oklab,
        var(--secondary) 90%,
        var(--secondary-foreground) 10%
      );
    }
  }
  .chat-message-footer__actions:has(
    :global([data-ui-part="composer-toolbar"][data-formatting-open])
  ) {
    flex-basis: 100%;
  }
</style>
