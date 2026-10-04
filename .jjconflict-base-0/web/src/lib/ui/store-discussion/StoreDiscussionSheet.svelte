<script lang="ts">
  import { AppShell, useAppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import XIcon from "@lucide/svelte/icons/x";
  import { untrack, type Snippet } from "svelte";
  import { repositoryDetailsPanelId } from "../layout-storage.js";
  let {
    title,
    open,
    children,
    header,
    footer,
    onClose,
    returnFocus,
  }: {
    title: string;
    open: boolean;
    children: Snippet;
    header?: Snippet;
    footer?: Snippet;
    onClose(): void;
    returnFocus?: HTMLElement | null;
  } = $props();
  const shell = useAppShell();
  let capturedReturnFocus = $state(untrack(() => returnFocus));
  $effect(() => {
    if (returnFocus) capturedReturnFocus = returnFocus;
  });
  const insets = $derived(
    shell.mobile.resolvedMode === "mobile"
      ? "--ui-shell-surface-layer-main-start-allowance: 0.5rem; --ui-shell-surface-layer-inline-end-inset: 0.5rem"
      : "--ui-shell-surface-layer-main-start-allowance: 7rem; --ui-shell-surface-layer-inline-end-inset: 0px",
  );
</script>

<AppShell.SurfaceLayer
  {open}
  label={title}
  coverPanelIds={["right", repositoryDetailsPanelId]}
  mobilePlacement="root"
  returnFocus={capturedReturnFocus}
  onDismiss={onClose}
  style={insets}
>
  <AppShell.SurfaceLayer.Header>
    {#if header}{@render header()}{:else}<h1>{title}</h1>{/if}
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label={`Close ${title}`}
      onclick={onClose}
    >
      <XIcon aria-hidden="true" />
    </Button>
  </AppShell.SurfaceLayer.Header>
  <AppShell.SurfaceLayer.Body class="store-discussion-sheet__body">
    {@render children()}
  </AppShell.SurfaceLayer.Body>
  {#if footer}<div class="store-discussion-sheet__footer">
      {@render footer()}
    </div>{/if}
</AppShell.SurfaceLayer>

<style>
  .store-discussion-sheet__footer {
    flex: 0 0 auto;
    min-width: 0;
  }
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
