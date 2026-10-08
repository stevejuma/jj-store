<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CodeXmlIcon from "@lucide/svelte/icons/code-xml";
  let {
    raw,
    label,
    onToggle,
  }: { raw: boolean; label: string; onToggle: (opener: HTMLElement) => void } =
    $props();
</script>

<span class="chat-raw-toggle">
  <Button
    variant="ghost"
    size="icon-sm"
    aria-label={label}
    title={label}
    aria-pressed={raw}
    onclick={(event) => onToggle(event.currentTarget)}
  >
    <CodeXmlIcon size={15} />
  </Button>
</span>

<style>
  .chat-raw-toggle {
    flex: none;
    opacity: 0;
  }
  :global(.chat-tool-heading:hover) .chat-raw-toggle,
  :global(.chat-tool-heading:focus-within) .chat-raw-toggle,
  :global(.chat-tool-panel header:hover) .chat-raw-toggle,
  :global(.chat-tool-panel header:focus-within) .chat-raw-toggle,
  .chat-raw-toggle:focus-within {
    opacity: 1;
  }
  @media (hover: none) {
    .chat-raw-toggle {
      opacity: 1;
    }
  }
</style>
