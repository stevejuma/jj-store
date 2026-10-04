<script lang="ts">
  import { untrack } from "svelte";
  import { createAppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  import { ownAppearance } from "../../web/src/lib/appearance/runtime.svelte.js";
  import {
    appearanceStorageKey,
    defaultAppearance,
    type AppearancePreferences,
  } from "../../web/src/lib/appearance/preferences.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import StoreSettingsPage from "../../web/src/lib/ui/store-settings/StoreSettingsPage.svelte";
  import StoreAppearance from "../../web/src/lib/ui/store-appearance/StoreAppearance.svelte";
  import CommunityMarkdownContent from "../../web/src/lib/ui/community-markdown-content/CommunityMarkdownContent.svelte";
  let { preferences = {} }: { preferences?: Partial<AppearancePreferences> } =
    $props();
  const storage = createMemoryStoryStorage();
  storage.setItem(
    appearanceStorageKey,
    JSON.stringify({ ...defaultAppearance, ...untrack(() => preferences) }),
  );
  const appearance = ownAppearance(createAppearanceStore(storage));
  let mounted = $state(true);
</script>

<div class="appearance-fixture" data-saved={JSON.stringify(appearance.saved)}>
  <StoreSettingsPage section="appearance"><StoreAppearance /></StoreSettingsPage
  >
  <button type="button" onclick={() => (mounted = !mounted)}
    >Toggle message mount</button
  >
  {#if mounted}<div aria-label="Sample conversation">
      <CommunityMarkdownContent
        content="[Existing product updates](https://example.com/updates) and `https://example.com/code`"
      />
    </div>{/if}
</div>

<style>
  .appearance-fixture {
    padding: 1rem;
    margin: auto;
    max-width: 64rem;
    font-family: var(--font-sans);
  }
</style>
