<script lang="ts">
  const shortcut =
    typeof navigator !== "undefined" && navigator.platform.includes("Mac")
      ? "⌘ ↓"
      : "Ctrl ↓";
</script>

<div class="chat-end-hint" role="tooltip">
  <span>Jump to End</span><kbd>{shortcut}</kbd>
</div>

<style>
  .chat-end-hint {
    position: absolute;
    right: calc(100% - 2px);
    bottom: -0.75rem;
    display: flex;
    align-items: center;
    gap: 1.25rem;
    white-space: nowrap;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--muted);
    color: var(--foreground);
    box-shadow: 0 3px 10px #0002;
    font-size: 0.875rem;
  }
  kbd {
    font: inherit;
    color: var(--muted-foreground);
  }
</style>
