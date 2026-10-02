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
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import { Button } from "@lapismd/design-core/shadcn/button";

  let {
    initialPage = "branches",
    initiallyCollapsed = true,
    displayMode = "auto",
    large = false,
    workingCopies,
    loading = false,
    connectionLoading = false,
    detailLoading = false,
    collaborationState = "ready",
    reviewOutcome,
    deferredBranchSelection = false,
  }: {
    initialPage?: StorePage;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
    large?: boolean;
    workingCopies?: readonly import("../../web/src/lib/projects/repository-log.js").RepositoryWorkingCopy[];
    loading?: boolean;
    connectionLoading?: boolean;
    detailLoading?: boolean;
    collaborationState?: CatalogCollaborationState;
    reviewOutcome?: "committed" | "unknown";
    deferredBranchSelection?: boolean;
  } = $props();
  let graphPending = $state(false);

  const controller = untrack(() => {
    const fixture = createInMemoryStoreController(initialPage, {
      large,
      workingCopies,
      reviewOutcome,
    });
    if (deferredBranchSelection) {
      const delayed: StoreController = {
        ...fixture,
        get state() {
          const state = fixture.state;
          return graphPending
            ? {
                ...state,
                graph: { revisions: [], devices: [] },
                status: { ...state.status, graph: "loading" as const },
              }
            : state;
        },
        actions: {
          ...fixture.actions,
          selectBookmark(name) {
            fixture.actions.selectBookmark(name);
            graphPending = true;
          },
          selectWorkingCopy(commit, workspace) {
            fixture.actions.selectWorkingCopy(commit, workspace);
            graphPending = true;
          },
        },
      };
      return delayed;
    }
    if (collaborationState !== "ready")
      return createCollaborationStateStoreController(
        fixture,
        collaborationState,
      );
    if (connectionLoading) {
      if (detailLoading && initialPage === "reviews")
        fixture.actions.selectReviewThread(
          fixture.state.reviewThreads[0]!.thread_id,
        );
      return createLoadingStoreController(fixture, true);
    }
    if (loading) {
      if (detailLoading && initialPage === "reviews")
        fixture.actions.selectReviewThread(
          fixture.state.reviewThreads[0]!.thread_id,
        );
      return createLoadingStoreController(fixture);
    }
    if (!detailLoading) return fixture;
    if (initialPage === "files") fixture.actions.selectFile("README.md");
    else if (initialPage === "reviews")
      fixture.actions.selectReviewThread(
        fixture.state.reviewThreads[0]!.thread_id,
      );
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
{#if deferredBranchSelection}<Button onclick={() => (graphPending = false)}
    >Finish branch load</Button
  >{/if}
