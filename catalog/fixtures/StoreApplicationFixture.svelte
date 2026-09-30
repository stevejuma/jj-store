<script lang="ts">
  import { untrack } from "svelte";
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import StoreApplication from "../../web/src/lib/ui/store-application/StoreApplication.svelte";
  import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import type { CatalogSearchSettings } from "../controller/in-memory-store-settings.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";

  let {
    initialPage = "branches",
    initiallyCollapsed = true,
    displayMode = "auto",
    settingsSearch = "fresh",
  }: {
    initialPage?: StorePage;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
    settingsSearch?: CatalogSearchSettings;
  } = $props();

  const controller = untrack(() =>
    createInMemoryStoreController(initialPage, { settingsSearch }),
  );
  const layoutStorage = untrack(createMemoryStoryStorage);
</script>

<StoreApplication
  {controller}
  {initiallyCollapsed}
  {displayMode}
  {layoutStorage}
/>
