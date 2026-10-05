<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import SearchIcon from "@lucide/svelte/icons/search";
  let {
    expanded = $bindable(false),
    controls,
    label,
  }: { expanded?: boolean; controls: string; label: string } = $props();
</script>

<Button
  variant={expanded ? "default" : "ghost"}
  size="icon"
  class="store-history-query-toggle"
  aria-label={label}
  title={label}
  aria-pressed={expanded}
  aria-expanded={expanded}
  aria-controls={controls}
  onclick={() => (expanded = !expanded)}
  ><SearchIcon aria-hidden="true" /></Button
>

<style>
  :global(.store-history-query-toggle) {
    --ui-button-radius: 999px;
    width: 1.75rem;
    height: 1.75rem;
  }
  :global(.store-history-query-toggle[aria-pressed="true"]) {
    --ui-button-background: color-mix(
      in srgb,
      var(--jj-store-row-background, var(--muted)) 78%,
      var(--foreground)
    );
    --ui-button-foreground: var(--primary);
  }
</style>
