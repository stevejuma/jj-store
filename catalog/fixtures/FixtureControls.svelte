<script lang="ts">
  import type { Snippet } from "svelte";
  let {
    children,
    label = "Fixture controls",
    floating = true,
  }: {
    children: Snippet;
    label?: string;
    floating?: boolean;
  } = $props();
</script>

<details
  class="fixture-controls"
  class:floating
  data-fixture-controls
  open={!floating}
>
  <summary>{label}</summary>
  <div class="fixture-controls__body" data-story-controls>
    {@render children()}
  </div>
</details>

<style>
  .fixture-controls {
    box-sizing: border-box;
    padding: 0.75rem;
    border: 2px dotted var(--muted-foreground);
    border-radius: 0.5rem;
    background: var(--background);
    color: var(--foreground);
    font: 0.75rem/1.5 var(--font-sans, sans-serif);
  }
  .floating {
    position: fixed;
    inset-inline-end: 0.75rem;
    inset-block-end: 0.75rem;
    z-index: 100;
    max-width: min(28rem, calc(100vw - 1.5rem));
    max-height: min(40vh, 24rem);
    overflow: auto;
    box-shadow: 0 2px 8px rgb(0 0 0 / 0.12);
  }
  summary {
    cursor: pointer;
    font-weight: 600;
  }
  .fixture-controls[open] > summary {
    margin-block-end: 0.5rem;
  }
  .fixture-controls__body {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }
  .fixture-controls__body :global(output) {
    overflow-wrap: anywhere;
  }
</style>
