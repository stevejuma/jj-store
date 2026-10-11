<script lang="ts">
  import type { Component } from "svelte";
  let {
    Icon,
    size = "row",
    tone,
  }: {
    Icon: Component;
    size?: "row" | "section";
    tone?: "ready" | "warning" | "error";
  } = $props();
</script>

<span
  class="store-settings-icon"
  data-size={size}
  data-tone={tone}
  aria-hidden="true"><Icon aria-hidden="true" /></span
>

<style>
  .store-settings-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex: none;
    width: 1.8rem;
    height: 1.8rem;
    border-radius: 50%;
    color: var(--foreground);
    background: color-mix(in srgb, var(--muted) 60%, transparent);
    vertical-align: middle;
  }
  .store-settings-icon[data-size="section"] {
    width: 2.25rem;
    height: 2.25rem;
  }
  .store-settings-icon :global(svg) {
    width: 1rem;
    height: 1rem;
  }
  .store-settings-icon[data-size="section"] :global(svg) {
    width: 1.2rem;
    height: 1.2rem;
  }
  .store-settings-icon[data-tone="ready"] {
    color: var(--success-foreground, var(--foreground));
    background: color-mix(
      in srgb,
      var(--success, var(--muted)) 15%,
      var(--card)
    );
  }
</style>
