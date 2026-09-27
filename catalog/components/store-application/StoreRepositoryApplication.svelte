<script lang="ts">
  import { untrack } from "svelte";
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import { createInMemoryStoreController } from "../../controller/in-memory-store-controller.svelte.js";
  import StoreApplication from "./StoreApplication.svelte";
  import CommunityProjectsRepositoryLog from "../community-projects/CommunityProjectsRepositoryLog.svelte";
  import CommunityProjectsRepositoryFiles from "../community-projects/CommunityProjectsRepositoryFiles.svelte";
  import type { StorePage } from "../community-navigation/model.js";
  import { NOW, sampleDetail } from "../../fixtures/repository.js";
  let {
    initialPage = "branches",
    controller = untrack(() => createInMemoryStoreController(initialPage)),
    initiallyCollapsed = true,
    displayMode = "auto",
  }: {
    initialPage?: StorePage;
    controller?: StoreController;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
  } = $props();
  const state = $derived(controller.state);
</script>

<StoreApplication {controller} {initiallyCollapsed} {displayMode}>
  {#snippet children(page)}
    {#if page === "files"}<CommunityProjectsRepositoryFiles {controller} />
    {:else}<CommunityProjectsRepositoryLog
        {page}
        graph={state.graph}
        bookmarks={[...state.bookmarks]}
        selectedBranch={state.route.bookmark}
        selectedId={state.route.revisionId}
        selectedPath={state.route.path}
        files={state.graph.revisions.find(
          (revision) => revision.eventId === state.route.revisionId,
        )?.empty
          ? []
          : sampleDetail.files.filter((file) =>
              state.snapshot?.files.some((entry) => entry.path === file.path),
            )}
        patch={state.route.path
          ? sampleDetail.patches[state.route.path]
          : undefined}
        now={NOW}
        onSelectBranch={controller.actions.selectBookmark}
        onSelectRevision={controller.actions.selectRevision}
        onSelectPath={controller.actions.selectFile}
      />{/if}
  {/snippet}
</StoreApplication>
