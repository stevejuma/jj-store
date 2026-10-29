<script lang="ts">
  import { untrack, onMount } from "svelte";
  import StoreApplicationRoot from "../../web/src/lib/ui/store-application/StoreApplicationRoot.svelte";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import type { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import { applicationController } from "./application-controller.js";
  import { applicationConnections } from "./application-connections.svelte.js";
  import {
    createAppearanceStore,
    useAppearance,
  } from "../../web/src/lib/appearance/store.svelte.js";
  import type { AppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let {
    controller,
    layoutStorage,
    stackWorkspace,
    initiallyCollapsed = true,
    desktopChrome,
    displayMode = "auto",
    appearance,
  }: {
    controller: StoreController;
    layoutStorage?: Storage;
    stackWorkspace?: StackWorkspaceController;
    initiallyCollapsed?: boolean;
    desktopChrome?: boolean;
    displayMode?: AppShellDisplayMode;
    appearance?: AppearanceStore;
  } = $props();
  const adapted = untrack(() =>
    applicationController(controller, () => stackWorkspace),
  );
  const connections = untrack(() =>
    applicationConnections(adapted, layoutStorage),
  );
  const preferences = untrack(
    () =>
      appearance ??
      useAppearance() ??
      createAppearanceStore(
        layoutStorage ?? createMemoryStoryStorage(),
        globalThis.matchMedia?.("(prefers-color-scheme: dark)").matches,
      ),
  );
  function applyStoryAppearance() {
    const mode = document.documentElement.dataset.storyColorMode;
    if (mode === "light" || mode === "dark" || mode === "system")
      preferences.mode(mode);
  }
  untrack(() => {
    if (typeof document !== "undefined") applyStoryAppearance();
  });
  onMount(() => {
    window.addEventListener("jj-store:story-appearance", applyStoryAppearance);
    return () =>
      window.removeEventListener(
        "jj-store:story-appearance",
        applyStoryAppearance,
      );
  });
</script>

<StoreApplicationRoot
  {connections}
  appearance={preferences}
  {initiallyCollapsed}
  {displayMode}
  {desktopChrome}
/>
