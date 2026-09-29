<script lang="ts">
  import { untrack } from "svelte";
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import {
    createDetailLoadingStoreController,
    createLoadingStoreController,
  } from "../controller/loading-store-controller.js";
  import {
    createCollaborationStateStoreController,
    type CatalogCollaborationState,
  } from "../controller/collaboration-store-controller.js";
  import { IDS } from "./repository.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";

  let {
    initialPage = "branches",
    initiallyCollapsed = true,
    displayMode = "auto",
    large = false,
    loading = false,
    detailLoading = false,
    collaborationState = "ready",
    reviewOutcome,
  }: {
    initialPage?: StorePage;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
    large?: boolean;
    loading?: boolean;
    detailLoading?: boolean;
    collaborationState?: CatalogCollaborationState;
    reviewOutcome?: "committed" | "unknown";
  } = $props();

  const controller = untrack(() => {
    const fixture = createInMemoryStoreController(initialPage, {
      large,
      reviewOutcome,
    });
    if (collaborationState !== "ready")
      return createCollaborationStateStoreController(
        fixture,
        collaborationState,
      );
    if (loading) return createLoadingStoreController(fixture);
    if (!detailLoading) return fixture;
    if (initialPage === "files") fixture.actions.selectFile("README.md");
    else
      fixture.actions.selectRevision(
        initialPage === "oplog" ? IDS.opLeft : IDS.current,
      );
    return createDetailLoadingStoreController(fixture);
  });
  const layoutStorage = untrack(createMemoryStoryStorage);
</script>

<StoreRepositoryApplication
  {controller}
  {initiallyCollapsed}
  {displayMode}
  {layoutStorage}
/>
