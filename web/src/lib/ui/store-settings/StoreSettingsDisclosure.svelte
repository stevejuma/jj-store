<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  let {
    label = "Advanced",
    open = $bindable(false),
    inset = true,
    children,
  }: {
    label?: string;
    open?: boolean;
    inset?: boolean;
    children: Snippet;
  } = $props();
</script>

<Collapsible.Root class="store-settings-disclosure" bind:open>
  <Collapsible.Trigger>
    {#snippet child({ props })}
      <Button
        {...props}
        type="button"
        variant="ghost"
        class="store-settings-disclosure__trigger"
      >
        <span>{label}</span>
        <ChevronRightIcon data-icon="inline-end" aria-hidden="true" />
      </Button>
    {/snippet}
  </Collapsible.Trigger>
  <Collapsible.Content forceMount>
    {#snippet child({ props })}
      <div
        {...props}
        hidden={!open}
        data-inset={inset}
        class="store-settings-disclosure__content"
      >
        {@render children()}
      </div>
    {/snippet}
  </Collapsible.Content>
</Collapsible.Root>

<style>
  :global(.store-settings-disclosure) {
    min-width: 0;
  }
  :global(.store-settings-disclosure__trigger) {
    --ui-button-radius: 0px;
    width: 100%;
    height: auto;
    justify-content: space-between;
    padding: 0.85rem 1rem;
  }
  :global(.store-settings-disclosure__trigger[aria-expanded="true"]) {
    border-end-start-radius: 0;
    border-end-end-radius: 0;
  }
  :global(.store-settings-disclosure__trigger:focus-visible) {
    border-color: transparent;
    box-shadow: none;
    outline: 2px solid var(--ring);
    outline-offset: -3px;
  }
  :global(.store-settings-disclosure__trigger[aria-expanded="true"] > svg) {
    transform: rotate(90deg);
  }
  .store-settings-disclosure__content:not([hidden]) {
    display: grid;
    gap: 0.8rem;
    padding: 1rem;
    border-block-start: 1px solid var(--border);
  }
  .store-settings-disclosure__content[data-inset="false"]:not([hidden]) {
    gap: 0;
    padding: 0;
  }
</style>
