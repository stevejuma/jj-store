<script lang="ts">
  import { provideHostEnvironment } from "../../web/src/lib/host/context.js";
  import {
    browserHost,
    type HostEnvironment,
  } from "../../web/src/lib/host/contracts.js";
  import { untrack } from "svelte";
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import StoreApplication from "./ApplicationStory.svelte";
  import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import type { CatalogSearchSettings } from "../controller/in-memory-store-settings.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";

  let {
    initialPage = "branches",
    initiallyCollapsed = true,
    displayMode = "auto",
    settingsSearch = "fresh",
    desktopChrome,
    host = browserHost,
    accessMode = "authenticated",
  }: {
    initialPage?: StorePage;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
    settingsSearch?: CatalogSearchSettings;
    desktopChrome?: boolean;
    host?: HostEnvironment;
    accessMode?: "authenticated" | "local_trusted";
  } = $props();

  provideHostEnvironment(untrack(() => host));
  const controller = untrack(() =>
    createInMemoryStoreController(initialPage, { settingsSearch, accessMode }),
  );
  const layoutStorage = untrack(createMemoryStoryStorage);
</script>

<StoreApplication
  {controller}
  {initiallyCollapsed}
  {displayMode}
  {layoutStorage}
  {desktopChrome}
/>
