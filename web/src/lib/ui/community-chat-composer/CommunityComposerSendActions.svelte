<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ArrowUpIcon from "@lucide/svelte/icons/arrow-up";
  let {
    show,
    disabled,
    busy,
    label,
    editing = false,
    onCancel,
    onSend,
  }: {
    show: boolean;
    disabled: boolean;
    busy: boolean;
    label?: string;
    editing?: boolean;
    onCancel?: () => void;
    onSend: () => void;
  } = $props();
</script>

{#if show}
  {#if onCancel}<Button
      type="button"
      variant="ghost"
      size="sm"
      disabled={busy}
      onclick={onCancel}>Cancel</Button
    >{/if}
  <Button
    type="button"
    size="icon-sm"
    class="community-chat-composer__send"
    data-ui-part="send-button"
    aria-label={label ?? (editing ? "Save message" : "Send message")}
    {disabled}
    onclick={onSend}><ArrowUpIcon aria-hidden="true" /></Button
  >
{/if}
