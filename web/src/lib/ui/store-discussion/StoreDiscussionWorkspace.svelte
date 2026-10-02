<script lang="ts">
  import { untrack, type Snippet } from "svelte";
  import { useDiscussionFocus } from "./discussion-focus.svelte.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { useAppearance } from "../../appearance/store.svelte.js";
  let {
    open,
    title,
    context,
    discussion,
    onClose,
    focusKey,
  }: {
    open: boolean;
    focusKey: object;
    title: string;
    context: Snippet;
    discussion: Snippet;
    onClose: () => void;
  } = $props();
  const appearance = useAppearance();
  const owner = useDiscussionFocus();
  $effect(() => {
    if (!owner) return;
    const showing = open,
      key = focusKey;
    untrack(() =>
      owner.observe(key, showing, {
        title: () => title,
        content: () => discussion,
        close: onClose,
      }),
    );
    return () => untrack(() => owner.unregister(key));
  });
  const focus = $derived(!!owner && appearance?.live.threads === "focus");
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
