<script lang="ts">
  import type { Snippet } from "svelte";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    label,
    children,
    ref = $bindable(null),
  }: {
    label: string;
    children?: Snippet;
    ref?: HTMLDivElement | null;
  } = $props();
  let open = $state(false);
</script>

<Collapsible.Root
  bind:ref
  bind:open
  class="store-prompt-element"
  data-prompt-element={label}
>
  {#if label.trimEnd().endsWith("/>")}
    <code class="store-prompt-element__leaf">{label}</code>
  {:else}
    <Collapsible.Trigger>
      {#snippet child({ props })}
        <Button
          {...props}
          variant="ghost"
          class="store-prompt-element__trigger w-full justify-start h-auto whitespace-normal"
        >
          <ChevronRightIcon data-icon="inline-start" aria-hidden="true" />
          <span>{label}</span>
        </Button>
      {/snippet}
    </Collapsible.Trigger>
    <Collapsible.Content class="store-prompt-element__body">
      {@render children?.()}
    </Collapsible.Content>
  {/if}
</Collapsible.Root>

<style>
  :global(.store-prompt-element) {
    min-width: 0;
    margin-block: 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }
  :global(.store-prompt-element__trigger) {
    text-align: start;
    overflow-wrap: anywhere;
  }
  :global(.store-prompt-element__trigger span),
  .store-prompt-element__leaf {
    min-width: 0;
    font-family: var(--font-mono);
    overflow-wrap: anywhere;
  }
  :global(.store-prompt-element__trigger[data-state="open"] svg) {
    transform: rotate(90deg);
  }
  :global(.store-prompt-element__body) {
    min-width: 0;
    padding: 0.25rem 0.75rem 0.75rem;
  }
  .store-prompt-element__leaf {
    display: block;
    padding: 0.5rem 0.75rem;
  }
</style>
