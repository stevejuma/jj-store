<script lang="ts">
  import { onDestroy, untrack, type Snippet } from "svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { WorkspaceFloatingLayer } from "@lapismd/design-core/workspace/floating-layer";
  import {
    WorkspaceShellController,
    createWorkspaceViewRegistry,
    createWorkspaceTab,
  } from "@lapismd/design-core/workspace/core";
  import "./detachable-panel.css";
  let {
    title,
    floating = $bindable(false),
    children,
  }: {
    title: string;
    floating?: boolean;
    children: Snippet<[{ floating: boolean; toggle: () => void }]>;
  } = $props();
  let anchor = $state<HTMLDivElement>();
  let content = $state<HTMLDivElement>();
  let returnFocus: HTMLElement | null = null;
  let disposed = false;
  const registry = createWorkspaceViewRegistry();
  const controller = untrack(() => new WorkspaceShellController({ registry }));
  controller.showTabTitleBar = false;
  const unregister = registry.register({
    kind: "imperative",
    type: "detachable-panel",
    showHeader: false,
    mount(target) {
      if (!content) return;
      const focused = content.ownerDocument.activeElement;
      target.appendChild(content);
      queueMicrotask(() => {
        if (disposed || !content?.isConnected) return;
        if (focused instanceof HTMLElement && content.contains(focused))
          focused.focus();
        else content.closest<HTMLElement>('[role="dialog"]')?.focus();
      });
      return () => {
        if (anchor && content) anchor.appendChild(content);
      };
    },
  });
  const unsubscribe = controller.onChange((layout) => {
    if (!layout.windows.length) floating = false;
  });
  function dock() {
    if (anchor && content) anchor.appendChild(content);
    for (const entry of [...controller.layout.windows])
      controller.closeWindow(entry.id);
    const active = controller.activeTab;
    if (active?.view.type === "detachable-panel")
      controller.closeTab(active.id);
    queueMicrotask(() => {
      if (!disposed) returnFocus?.focus();
    });
  }
  $effect(() => {
    const detached = floating;
    if (!anchor || !content) return;
    untrack(() => {
      if (!detached) {
        dock();
        return;
      }
      if (controller.layout.windows.length) return;
      const active = content!.ownerDocument.activeElement;
      returnFocus = active instanceof HTMLElement ? active : null;
      const viewport = content!.ownerDocument.defaultView!;
      const width = Math.min(600, viewport.innerWidth - 24);
      const height = Math.min(600, viewport.innerHeight - 48);
      controller.openWindow(
        createWorkspaceTab({ title, view: { type: "detachable-panel" } }),
        "floating",
        {
          x: Math.max(12, (viewport.innerWidth - width) / 2),
          y: 24,
          width,
          height,
        },
      );
    });
  });
  function resize() {
    if (!floating || !content) return;
    const viewport = content.ownerDocument.defaultView!;
    for (const entry of controller.layout.windows) {
      const width = Math.min(
        entry.bounds.width,
        Math.max(280, viewport.innerWidth - 24),
      );
      const height = Math.min(
        entry.bounds.height,
        Math.max(180, viewport.innerHeight - 24),
      );
      controller.setWindowBounds(
        entry.id,
        {
          width,
          height,
          x: Math.max(0, Math.min(entry.bounds.x, viewport.innerWidth - width)),
          y: Math.max(
            0,
            Math.min(entry.bounds.y, viewport.innerHeight - height),
          ),
        },
        false,
      );
    }
  }
  function escape(event: KeyboardEvent) {
    if (
      event.key !== "Escape" ||
      event.defaultPrevented ||
      !floating ||
      !content
    )
      return;
    const window = content.closest('[role="dialog"]');
    if (window?.contains(content.ownerDocument.activeElement)) {
      event.preventDefault();
      floating = false;
    }
  }
  onDestroy(() => {
    disposed = true;
    if (anchor && content) anchor.appendChild(content);
    unsubscribe();
    unregister();
    controller.destroy();
  });
</script>

<svelte:window onresize={resize} onkeydown={escape} />
<div
  bind:this={anchor}
  class="store-detachable-panel"
  data-detached={floating || undefined}
>
  <div bind:this={content} class="store-detachable-panel__content">
    {@render children({ floating, toggle: () => (floating = !floating) })}
  </div>
</div>
<Dialog.Portal><WorkspaceFloatingLayer {controller} /></Dialog.Portal>
