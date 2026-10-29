<script lang="ts">
  import "../community-overlays.css";
  import "./right-sidebar.css";
  import "./header-hover.css";
  import "./desktop-chrome.css";

  import { Button } from "@lapismd/design-core/shadcn/button";
  import {
    AppShell,
    type AppShellController,
    type AppShellDisplayMode,
  } from "@lapismd/design-core/shell";
  import ChevronLeftIcon from "@lucide/svelte/icons/chevron-left";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";

  import { untrack, type Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  let {
    controller,
    displayMode = "auto",
    mobileBreakpoint = 1024,
    desktopMinMainWidth,
    desktopChrome = false,
    mainElement,
    mobileTarget,
    mobileSidebarName = "navigation",
    canNavigateBack = false,
    canNavigateForward = false,
    onNavigateBack,
    onNavigateForward,
    headerCenter,
    headerEnd,
    class: className,
    children,
    ...restProps
  }: HTMLAttributes<HTMLDivElement> & {
    controller: AppShellController;
    displayMode?: AppShellDisplayMode;
    mobileBreakpoint?: number;
    desktopMinMainWidth?: number;
    desktopChrome?: boolean;
    mainElement?: HTMLElement | null;
    mobileTarget?: HTMLElement | null;
    mobileSidebarName?: string;
    canNavigateBack?: boolean;
    canNavigateForward?: boolean;
    onNavigateBack?: () => void;
    onNavigateForward?: () => void;
    headerCenter?: Snippet;
    headerEnd?: Snippet;
    children?: Snippet;
  } = $props();

  let shellSurface = $state<HTMLDivElement | null>(null);
  let measuredMain = $state<HTMLElement | null>(null);
  const mobileMode = $derived(controller.mobile.resolvedMode === "mobile");
  const inBody = $derived(mobileMode && !desktopChrome && !!mobileTarget);
  $effect(() => {
    measuredMain =
      mainElement ??
      shellSurface?.querySelector<HTMLElement>('[data-ui-part="main"]') ??
      null;
  });

  $effect(() => {
    const rightSidebar = controller.right;
    const preventCollapsedRightSidebar = () => {
      if (!rightSidebar.collapsed) return;
      const wasClosed = rightSidebar.closed;
      rightSidebar.expand();
      if (wasClosed) rightSidebar.close();
    };

    untrack(preventCollapsedRightSidebar);
    return rightSidebar.onLayoutChange(preventCollapsedRightSidebar);
  });

  $effect(() => {
    const stage = controller.mobile.stage;
    if (!mobileMode || !shellSurface) return;
    queueMicrotask(() => {
      if (
        controller.mobile.resolvedMode !== "mobile" ||
        controller.mobile.stage !== stage
      ) {
        return;
      }
      const mobileStage = shellSurface?.querySelector<HTMLElement>(
        '[data-ui-part="mobile-stage"]',
      );
      if (mobileStage) mobileStage.scrollLeft = 0;
    });
  });
</script>

<div
  {...restProps}
  class={["community-app-shell", className].filter(Boolean).join(" ")}
  data-ui-component="community-app-shell"
  data-display-mode={controller.mobile.resolvedMode}
  data-desktop-chrome={desktopChrome ? "macos" : undefined}
>
  <AppShell.Topbar
    {controller}
    mainElement={measuredMain}
    mobilePlacement={desktopChrome ? "top" : "body"}
    {mobileTarget}
    class="community-app-shell__header"
    role={inBody ? "toolbar" : "navigation"}
    aria-label={inBody ? "Application controls" : "Application navigation"}
    data-tauri-drag-region={desktopChrome ? "deep" : undefined}
  >
    {#snippet leading()}
      <div class="community-app-shell__header-start">
        <AppShell.Sidebar.Toggle
          shellController={controller}
          side="left"
          sidebarName={mobileSidebarName}
          label={!mobileMode
            ? controller.left.collapsed
              ? "Expand left sidebar"
              : "Collapse left sidebar"
            : undefined}
        />
        <div
          class="community-app-shell__history"
          role="group"
          aria-label="Navigation history"
        >
          <Button
            type="button"
            variant="ghost"
            size="icon-sm"
            aria-label="Back"
            title="Back"
            disabled={!canNavigateBack}
            onclick={() => onNavigateBack?.()}
          >
            <ChevronLeftIcon aria-hidden="true" />
          </Button>
          <Button
            type="button"
            variant="ghost"
            size="icon-sm"
            aria-label="Forward"
            title="Forward"
            disabled={!canNavigateForward}
            onclick={() => onNavigateForward?.()}
          >
            <ChevronRightIcon aria-hidden="true" />
          </Button>
        </div>
      </div>
    {/snippet}
    {#snippet center()}
      <div class="community-app-shell__header-center">
        {@render headerCenter?.()}
      </div>
    {/snippet}
    {#snippet trailing()}
      <div class="community-app-shell__header-end">{@render headerEnd?.()}</div>
    {/snippet}
  </AppShell.Topbar>

  <div class="community-app-shell__stage">
    <AppShell.Root
      bind:ref={shellSurface}
      {controller}
      {displayMode}
      {mobileBreakpoint}
      {...desktopMinMainWidth === undefined ? {} : { desktopMinMainWidth }}
      class="community-app-shell__surface"
    >
      {@render children?.()}
    </AppShell.Root>
  </div>
</div>

<style>
  .community-app-shell {
    box-sizing: border-box;
    --ui-shell-topbar-height: 2.5rem;
    --ui-shell-topbar-padding: 0;
    --ui-shell-topbar-background: var(
      --store-theme-chrome,
      var(--sidebar, var(--muted))
    );
    display: flex;
    flex-direction: column;
    width: 100%;
    min-width: 0;
    height: 100%;
    min-height: 0;
    overflow: hidden;
    color: var(--foreground);
    background: var(--muted);
  }

  /* Native controls retain fixed geometry when application text scales. */
  .community-app-shell[data-desktop-chrome="macos"] {
    --ui-shell-topbar-height: 40px;
  }

  .community-app-shell__header-start {
    position: relative;
    z-index: 1;
    display: flex;
    box-sizing: border-box;
    align-items: center;
    gap: 0.125rem;
    padding-inline-start: 0.5rem;
  }

  .community-app-shell__history {
    display: flex;
    align-items: center;
    gap: 0.125rem;
  }

  .community-app-shell__header-center {
    display: flex;
    min-width: 0;
    align-items: center;
    justify-content: center;
  }

  .community-app-shell__header-end {
    position: relative;
    z-index: 1;
    display: flex;
    align-items: center;
    gap: 0.25rem;
    margin-inline-start: auto;
    padding-inline-end: 0.75rem;
  }

  .community-app-shell__stage {
    flex: 1;
    position: relative;
    display: flex;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
  }

  :global(.community-app-shell__surface) {
    --ui-shell-height: 100%;
    --ui-shell-right-sidebar-gap: 0;

    width: auto;
    flex: 1;
    border: 0;
    border-radius: 0;
    box-shadow: none;
  }

  :global(.community-app-shell__surface [data-ui-part="toolbar"]) {
    padding-inline: 1rem;
    padding-inline-start: 0.5rem;
    border-block-end: 0;
  }

  :global(
    .community-app-shell__surface
      [data-ui-part="sidebar"][data-side="right"]
      > [data-ui-part="sidebar-header"]
      > strong
  ) {
    min-width: 0;
    flex: 1;
  }

  :global(
    .community-app-shell__surface [data-ui-part="sidebar-header"] > strong
  ) {
    font-size: var(--community-text-body);
  }
</style>
