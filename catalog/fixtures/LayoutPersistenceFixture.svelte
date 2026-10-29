<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { tick, untrack } from "svelte";
  import StoreRepositoryApplication from "./ApplicationStory.svelte";
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
  <FixtureControls
    ><div data-story-controls>
      <button
        type="button"
        class="layout-persistence-fixture__remount"
        onclick={remount}>Remount application</button
      >
    </div></FixtureControls
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
</style>
