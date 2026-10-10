<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import SendIcon from "@lucide/svelte/icons/send";
  import StopIcon from "@lucide/svelte/icons/square";
  import type { Snippet } from "svelte";
  let {
    canSend,
    onSend,
    onStop,
    stopping = false,
    error = "",
    controls,
    workspace,
    actions,
  }: {
    canSend: boolean;
    onSend: () => void;
    onStop?: () => void;
    stopping?: boolean;
    error?: string;
    controls?: Snippet;
    workspace?: Snippet;
    actions?: Snippet;
  } = $props();
</script>

<div class="chat-message-footer" role="group" aria-label="Message actions">
  {#if error}<p role="alert">{error}</p>{/if}
  {#if actions}<div class="chat-message-footer__actions">
      {@render actions()}
    </div>{/if}
  {@render workspace?.()}
  {@render controls?.()}
  <Button
    size="icon-sm"
    variant={onStop ? "destructive" : "outline"}
    disabled={onStop ? stopping : !canSend}
    onclick={onStop ?? onSend}
    aria-label={onStop
      ? stopping
        ? "Stopping agent"
        : "Stop agent"
      : "Send message"}
    title={onStop ? "Stop active work" : "Send message (⌘ / Ctrl + Enter)"}
    >{#if onStop}<StopIcon />{:else}<SendIcon />{/if}</Button
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
