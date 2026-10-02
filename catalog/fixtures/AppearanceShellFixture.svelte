<script lang="ts">
  import { untrack } from "svelte";
  import { ownAppearance } from "../../web/src/lib/appearance/runtime.svelte.js";
  import { createAppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  import {
    appearanceStorageKey,
    defaultAppearance,
  } from "../../web/src/lib/appearance/preferences.js";
  import {
    lightThemes,
    type ThemeName,
  } from "../../web/src/lib/appearance/theme-catalog.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import ProjectsHubFixture from "./ProjectsHubFixture.svelte";
  import RepositoryFixture from "./StoreRepositoryApplicationFixture.svelte";
  let {
    theme = "buzz",
    glass = false,
    repository = false,
  }: { theme?: ThemeName; glass?: boolean; repository?: boolean } = $props();
  const storage = untrack(() => {
    const memory = createMemoryStoryStorage();
    memory.setItem(
      appearanceStorageKey,
      JSON.stringify({
        ...defaultAppearance,
        theme,
        mode: lightThemes.has(theme) ? "light" : "dark",
        glass,
      }),
    );
    return memory;
  });
  ownAppearance(createAppearanceStore(storage));
</script>

<div class="appearance-shell-fixture">
  {#if repository}
    <RepositoryFixture initialPage="files" layoutStorage={storage} />
  {:else}
    <ProjectsHubFixture layoutStorage={storage} />
  {/if}
</div>

<style>
  .appearance-shell-fixture {
    height: 100%;
    min-height: 0;
    background: var(--background);
    color: var(--foreground);
  }
</style>
