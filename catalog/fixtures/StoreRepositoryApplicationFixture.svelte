<script lang="ts">
  import { untrack } from "svelte";
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";

  let {
    initialPage = "branches",
    initiallyCollapsed = true,
    displayMode = "auto",
  }: {
    initialPage?: StorePage;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
  } = $props();

  const controller = untrack(() => createInMemoryStoreController(initialPage));
</script>

<StoreRepositoryApplication {controller} {initiallyCollapsed} {displayMode} />
