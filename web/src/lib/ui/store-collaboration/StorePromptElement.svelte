<script lang="ts">
  import type { Snippet } from "svelte";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StorePromptTag from "./StorePromptTag.svelte";
  let {
    label,
    closing,
    compact = false,
    children,
    ref = $bindable(null),
  }: {
    label: string;
    closing?: string;
    compact?: boolean;
    children?: Snippet;
    ref?: HTMLDivElement | null;
  } = $props();
  let open = $state(false);
</script>

{#if compact}
  <div
    bind:this={ref}
    class="store-prompt-element store-prompt-element__compact"
    data-prompt-element={label}
  >
    <StorePromptTag {label} />{#if closing}<span
        class="store-prompt-element__inline">{@render children?.()}</span
      ><StorePromptTag label={closing} />{/if}
  </div>
{:else}
  <Collapsible.Root
    bind:ref
    bind:open
    class="store-prompt-element"
    data-prompt-element={label}
  >
    <Collapsible.Trigger>
      {#snippet child({ props })}
        <Button
          {...props}
          variant="ghost"
          aria-label={label}
          class="store-prompt-element__trigger"
        >
          <ChevronRightIcon data-icon="inline-start" aria-hidden="true" />
          <StorePromptTag {label} />
        </Button>
      {/snippet}
    </Collapsible.Trigger>
    <Collapsible.Content class="store-prompt-element__body">
      {@render children?.()}
    </Collapsible.Content>
  </Collapsible.Root>
{/if}

<style>
  :global(.store-prompt-element) {
    min-width: 0;
    margin-block: 0.125rem;
  }
  :global(.store-prompt-element__trigger) {
    width: 100%;
    height: auto;
    justify-content: flex-start;
    white-space: normal;
    text-align: start;
    overflow-wrap: anywhere;
    padding-block: 0.25rem;
    padding-inline: 0.625rem;
    gap: 0.375rem;
    border: 0;
  }
  :global(.store-prompt-element__trigger svg) {
    width: 1rem;
    height: 1rem;
    flex-shrink: 0;
  }
  :global(.store-prompt-element__trigger[data-state="open"] svg) {
    transform: rotate(90deg);
  }
  :global(.store-prompt-element__body) {
    min-width: 0;
    padding-block: 0.125rem 0.375rem;
    padding-inline: 1rem 0.625rem;
  }
  .store-prompt-element__compact {
    padding: 0.25rem 0.625rem;
    /* Reserve the same icon/gap column as disclosure labels. */
    padding-inline-start: 2rem;
    overflow-wrap: anywhere;
  }
  .store-prompt-element__inline {
    margin-inline: 0.35rem;
  }
</style>
