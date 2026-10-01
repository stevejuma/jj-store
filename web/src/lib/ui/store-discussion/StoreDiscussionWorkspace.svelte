<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { useAppearance } from "../../appearance/store.svelte.js";
  let {
    open,
    title,
    context,
    discussion,
    onClose,
  }: {
    open: boolean;
    title: string;
    context: Snippet;
    discussion: Snippet;
    onClose: () => void;
  } = $props();
  const appearance = useAppearance();
  const focus = $derived(appearance?.live.threads === "focus");
</script>

<div
  class="store-discussion-workspace"
  data-layout={focus ? "focus" : "split"}
  data-open={open}
>
  <div class="store-discussion-workspace__context">{@render context()}</div>
  {#if open && !focus}<aside
      class="store-discussion-workspace__panel"
      aria-label={title}
    >
      <header>
        <strong>{title}</strong><Button
          variant="ghost"
          size="icon-sm"
          aria-label={`Close ${title}`}
          title={`Close ${title}`}
          onclick={onClose}>×</Button
        >
      </header>
      <div class="store-discussion-workspace__content">
        {@render discussion()}
      </div>
    </aside>{/if}
</div>
{#if open && focus}<Dialog.Root
    open
    onOpenChange={(value) => {
      if (!value) onClose();
    }}
    ><Dialog.Content class="store-discussion-focus"
      ><Dialog.Header
        ><Dialog.Title>{title}</Dialog.Title><Dialog.Description
          >Discussion for the selected item.</Dialog.Description
        ></Dialog.Header
      >
      <div class="store-discussion-focus__content">
        {@render discussion()}
      </div></Dialog.Content
    ></Dialog.Root
  >{/if}

<style>
  .store-discussion-workspace {
    display: flex;
    width: 100%;
    min-width: 0;
    min-height: 0;
    height: 100%;
    gap: 0.75rem;
  }
  .store-discussion-workspace__context {
    flex: 1 1 0;
    min-width: 0;
    min-height: 0;
    overflow: auto;
  }
  .store-discussion-workspace__panel {
    display: flex;
    flex-direction: column;
    flex: 1 1 0;
    min-width: 0;
    min-height: 0;
    border-left: 1px solid var(--border);
    background: var(--background);
  }
  .store-discussion-workspace__panel > header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem 1rem;
    font-size: var(--community-text-body);
    border-bottom: 1px solid var(--border);
  }
  .store-discussion-workspace__content {
    flex: 1;
    min-height: 0;
    min-width: 0;
    overflow: auto;
    padding: 0.75rem;
  }
  :global(.store-discussion-focus[data-ui-part="dialog-content"]) {
    display: flex;
    flex-direction: column;
    width: min(58rem, calc(100vw - 2rem));
    max-width: none;
    height: min(48rem, calc(100dvh - 2rem));
    min-height: 0;
  }
  :global(.store-discussion-focus__content) {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    overflow: auto;
  }
  @media (max-width: 48rem) {
    .store-discussion-workspace {
      flex-direction: column;
    }
    .store-discussion-workspace__panel {
      border-left: 0;
      border-top: 1px solid var(--border);
    }
  }
</style>
