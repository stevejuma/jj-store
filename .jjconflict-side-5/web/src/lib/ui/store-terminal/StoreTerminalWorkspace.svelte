<script lang="ts">
  import {
    onDestroy,
    onMount,
    untrack,
    tick,
    type ComponentProps,
    type Snippet,
  } from "svelte";
  import * as Resizable from "@lapismd/design-core/shadcn/resizable";
  import { terminalControls } from "../../terminal/controls.svelte.js";
  const controls = terminalControls();
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import { TerminalController } from "../../terminal/controller.svelte.js";
  import {
    nativeTerminalBridge,
    type TerminalBridge,
  } from "../../terminal/bridge.js";
  import StoreTerminalDock from "./StoreTerminalDock.svelte";
  import "./terminal.css";
  let {
    repository,
    profile = "native",
    enabled = true,
    onLink,
    onRefresh = () => {},
    onMaximize = () => {},
    bridge = nativeTerminalBridge(),
    children,
  }: {
    repository: string;
    profile?: string;
    enabled?: boolean;
    onLink(): void;
    onRefresh?: () => void;
    onMaximize?: () => void;
    bridge?: TerminalBridge;
    children: Snippet;
  } = $props();
  const notifications = ownNotifications();
  const controller = untrack(() =>
    bridge
      ? new TerminalController(bridge, (error) =>
          notifications.context.service.publish({
            scope: "desktop-terminal",
            key: "operation",
            title: "Terminal unavailable",
            message: error instanceof Error ? error.message : String(error),
            lifetime: "dismiss",
          }),
        )
      : undefined,
  );
  let pane = $state<ComponentProps<typeof Resizable.Pane>["pane"]>(null);
  let dockSize = 40;
  let shown = $derived(Boolean(controller?.visible && enabled && repository));
  function hide() {
    if (controller) controller.visible = false;
    onRefresh();
    void tick().then(() => controls?.trigger?.focus());
  }
  async function toggleDock() {
    if (!controller || !enabled || !repository) return;
    if (shown) hide();
    else {
      await controller.context(repository, profile);
      await controller.show();
    }
  }
  $effect(() => {
    if (controls) {
      controls.available = Boolean(controller && enabled && repository);
      controls.open = shown;
      controls.toggle = () => {
        void toggleDock();
      };
    }
  });
  $effect(() => {
    const repo = repository;
    const kind = profile;
    if (controller) untrack(() => void controller.context(repo, kind));
  });
  $effect(() => {
    const visible = shown;
    const maximized = controller?.maximized;
    if (pane)
      untrack(() => pane?.resize(visible ? (maximized ? 100 : dockSize) : 0));
  });
  onMount(() => {
    try {
      const stored = Number(
        localStorage.getItem("jj-store.terminal-layout.v1"),
      );
      if (stored >= 20 && stored <= 70) dockSize = stored;
    } catch {
      /* Device storage is optional. */
    }
    if (controller) void controller.initialize();
    const key = (event: KeyboardEvent) => {
      if (
        event.metaKey &&
        event.code === "KeyJ" &&
        !event.shiftKey &&
        !event.altKey &&
        !event.ctrlKey &&
        !event.isComposing &&
        !event.repeat &&
        enabled &&
        repository &&
        controller
      ) {
        event.preventDefault();
        event.stopImmediatePropagation();
        void toggleDock();
      }
    };
    window.addEventListener("keydown", key, true);
    return () => window.removeEventListener("keydown", key, true);
  });
  onDestroy(() => controller?.dispose());
</script>

{#if controller}
  <div class="store-terminal-workspace">
    <Resizable.PaneGroup
      direction="vertical"
      onLayoutChange={(sizes) => {
        if (
          shown &&
          !controller.maximized &&
          sizes[1] >= 20 &&
          sizes[1] <= 70
        ) {
          dockSize = sizes[1];
          try {
            localStorage.setItem(
              "jj-store.terminal-layout.v1",
              String(dockSize),
            );
          } catch {
            /* Optional preference. */
          }
        }
      }}
    >
      <Resizable.Pane
        defaultSize={100}
        minSize={0}
        collapsible
        collapsedSize={0}
        ><div
          class="store-terminal-content"
          inert={shown && controller.maximized}
        >
          {@render children()}
        </div></Resizable.Pane
      >
      <Resizable.Handle
        onDraggingChange={(dragging) => {
          if (!dragging && shown && pane?.isCollapsed()) hide();
        }}
        onkeyup={() => {
          void tick().then(() => {
            if (shown && pane?.isCollapsed()) hide();
          });
        }}
        disabled={!shown || controller.maximized}
        aria-label="Resize terminal"
        class={shown && !controller.maximized
          ? "store-terminal-handle"
          : "store-terminal-handle-hidden"}
      />
      <Resizable.Pane
        bind:pane
        defaultSize={0}
        minSize={20}
        maxSize={controller.maximized ? 100 : 70}
        collapsible
        collapsedSize={0}
      >
        {#if shown}<StoreTerminalDock
            {controller}
            onHide={hide}
            {onMaximize}
            {onLink}
          />{/if}
      </Resizable.Pane>
    </Resizable.PaneGroup>
  </div>
{:else}{@render children()}{/if}
