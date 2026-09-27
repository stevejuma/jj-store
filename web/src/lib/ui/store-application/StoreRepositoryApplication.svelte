<script lang="ts">
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreApplication from "./StoreApplication.svelte";
  import CommunityProjectsRepositoryLog from "../community-projects/CommunityProjectsRepositoryLog.svelte";
  import CommunityProjectsRepositoryFiles from "../community-projects/CommunityProjectsRepositoryFiles.svelte";
  let {
    controller,
    initiallyCollapsed = true,
    displayMode = "auto",
  }: {
    controller: StoreController;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
  } = $props();
  const state = $derived(controller.state);
  const selectedId = $derived(
    state.route.page === "oplog"
      ? state.route.inspectedOperationId
      : state.route.commitId,
  );
</script>

<StoreApplication {controller} {initiallyCollapsed} {displayMode}>
  {#snippet children(page)}
    {#if page === "files"}<CommunityProjectsRepositoryFiles
        {controller}
        now={state.now}
      />
    {:else}<CommunityProjectsRepositoryLog
        {page}
        graph={state.graph}
        bookmarks={[...state.bookmarks]}
        selectedBranch={state.route.bookmark}
        {selectedId}
        selectedPath={state.route.path}
        files={state.graph.revisions.find(
          (revision) => revision.eventId === selectedId,
        )?.empty
          ? []
          : (state.detail?.files ?? [])}
        patch={state.route.path
          ? state.detail?.patches[state.route.path]
          : undefined}
        now={state.now}
        onSelectBranch={controller.actions.selectBookmark}
        onSelectRevision={controller.actions.selectRevision}
        onSelectPath={controller.actions.selectFile}
      />{/if}
  {/snippet}
</StoreApplication>
