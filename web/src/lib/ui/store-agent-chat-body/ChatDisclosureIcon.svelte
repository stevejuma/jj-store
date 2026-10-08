<script lang="ts">
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
</script>

<span class="chat-disclosure-icon" aria-hidden="true">
  <ChevronRightIcon size={14} />
</span>

<style>
  .chat-disclosure-icon {
    --ui-button-icon-size: 14px;
    display: inline-flex;
    flex: none;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    margin-inline-start: -4px;
    border-radius: calc(var(--radius) * 0.5);
  }
  .chat-disclosure-icon:hover,
  :global(button:focus-visible) .chat-disclosure-icon {
    background: var(--muted);
  }
  :global(button[data-state="open"]) .chat-disclosure-icon :global(svg) {
    rotate: 90deg;
  }
</style>
