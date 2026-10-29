<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import { untrack, type Snippet } from "svelte";
  import { repositoryDetailsPanelId } from "../layout-storage.js";
  let {
    title,
    open,
    children,
    header,
    footer,
    scrollable = true,
    onClose,
    returnFocus,
  }: {
    title: string;
    open: boolean;
    children: Snippet;
    header?: Snippet;
    footer?: Snippet;
    scrollable?: boolean;
    onClose(): void;
    returnFocus?: HTMLElement | null;
  } = $props();
  let capturedReturnFocus = $state(untrack(() => returnFocus));
  $effect(() => {
    if (returnFocus) capturedReturnFocus = returnFocus;
  });
</script>

<AppShell.SurfaceLayer
  {open}
  label={title}
  coverPanelIds={["right", repositoryDetailsPanelId]}
  mobilePlacement="root"
  returnFocus={capturedReturnFocus}
  onDismiss={onClose}
  dismissLabel={`Close ${title}`}
  style="--ui-shell-surface-layer-main-start-allowance: 7rem; --ui-shell-surface-layer-inline-end-inset: 0px; --ui-shell-surface-layer-mobile-main-start-allowance: 0.5rem; --ui-shell-surface-layer-mobile-inline-end-inset: 0.5rem;"
>
  <AppShell.SurfaceLayer.Header>
    {#if header}{@render header()}{:else}<h1>{title}</h1>{/if}
    <AppShell.SurfaceLayer.Close />
  </AppShell.SurfaceLayer.Header>
  <AppShell.SurfaceLayer.Body {scrollable} class="store-discussion-sheet__body">
    {@render children()}
  </AppShell.SurfaceLayer.Body>
  {#if footer}<AppShell.SurfaceLayer.Footer>
      {@render footer()}
    </AppShell.SurfaceLayer.Footer>{/if}
</AppShell.SurfaceLayer>

<style>
  h1 {
    flex: 1;
    min-width: 0;
    margin: 0;
    font-size: var(--community-text-body);
    font-weight: 600;
  }
  :global(.store-discussion-sheet__body) {
    display: flex;
    min-height: 0;
    flex-direction: column;
  }
</style>
