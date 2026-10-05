<script lang="ts">
  import { tick, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import { repositoryShellStorageKey } from "../../web/src/lib/ui/layout-storage.js";

  const controller = untrack(() => createInMemoryStoreController("branches"));
  const layoutStorage = untrack(createMemoryStoryStorage);
  let mountVersion = $state(0);
  let mounted = $state(true);
  let savedShellLayout = $state<string | null>(null);

  async function remount(): Promise<void> {
    mounted = false;
    await tick();
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
    savedShellLayout = layoutStorage.getItem(repositoryShellStorageKey);
    mountVersion += 1;
    mounted = true;
  }
</script>

<div
  class="layout-persistence-fixture"
  data-mount-version={mountVersion}
  data-saved-shell-layout={savedShellLayout}
>
  <button
    type="button"
    class="layout-persistence-fixture__remount"
    onclick={remount}>Remount application</button
  >
  {#if mounted}<StoreRepositoryApplication
      {controller}
      {layoutStorage}
      initiallyCollapsed={false}
    />{/if}
</div>

<style>
  .layout-persistence-fixture {
    height: 100vh;
  }
  .layout-persistence-fixture__remount {
    position: fixed;
    z-index: 100;
    inset-block-end: 0.5rem;
    inset-inline-end: 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 0.35rem 0.6rem;
    background: var(--background);
    color: var(--foreground);
    font: inherit;
  }
</style>
