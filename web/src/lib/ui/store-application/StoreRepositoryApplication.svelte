<script lang="ts">
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import StoreRepositoryProjects from "../store-work/StoreRepositoryProjects.svelte";
  import StoreProjects from "../store-work/StoreProjects.svelte";
  import StoreWork from "../store-work/StoreWork.svelte";
  import StoreThreads from "../store-threads/StoreThreads.svelte";
  import StoreApplication from "./StoreApplication.svelte";
  import CommunityProjectsRepositoryLog from "../community-projects/CommunityProjectsRepositoryLog.svelte";
  import CommunityProjectsRepositoryFiles from "../community-projects/CommunityProjectsRepositoryFiles.svelte";
  import StoreAgentSessions from "../store-collaboration/StoreAgentSessions.svelte";
  import StoreReviewThreads from "../store-collaboration/StoreReviewThreads.svelte";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    controller,
    initiallyCollapsed = true,
    displayMode = "auto",
    layoutStorage,
  }: {
    controller: StoreController;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
    layoutStorage?: Storage;
  } = $props();
  const state = $derived(controller.state);
  const selectedId = $derived(
    state.route.page === "oplog"
      ? state.route.inspectedOperationId
      : state.route.commitId,
  );
  const needsOperation = $derived(
    state.status.connection === "connected" &&
      !state.route.snapshotOperationId &&
      state.operationHeads.length > 1,
  );
</script>

<StoreApplication
  {controller}
  {initiallyCollapsed}
  {displayMode}
  {layoutStorage}
>
  {#snippet children(page)}
    {#if isRepositoryProjects(state.route) && controller.projects}<StoreRepositoryProjects
        {controller}
      />
    {:else if page === "projects" && controller.projects}<StoreProjects
        {controller}
      />
    {:else if page === "projects" && controller.work}<StoreWork
        work={controller.work}
        principal={state.repository.principal}
        repositories={state.repositories}
        onOpenRepository={controller.actions.openRepository}
        writable={state.accessMode === "local_trusted" ||
          state.repository.permission === "write" ||
          state.repository.permission === "admin"}
      />
    {:else if page === "threads"}<StoreThreads {controller} />
    {:else if needsOperation}<section
        class="store-repository-application__state"
      >
        <Alert.Root>
          <Alert.Title>Multiple operation heads</Alert.Title>
          <Alert.Description
            >Choose the native operation snapshot to browse. Opening this view
            never reconciles history.</Alert.Description
          >
          <div class="store-repository-application__choices">
            {#each state.operationHeads as operation}<Button
                variant="outline"
                onclick={() => controller.actions.selectOperation(operation)}
                >{operation.slice(0, 12)}</Button
              >{/each}
          </div>
        </Alert.Root>
      </section>
    {:else if state.route.workspace && state.status.navigation === "failed"}
      <section class="store-repository-application__state">
        <Alert.Root variant="destructive">
          <Alert.Title>Repository view unavailable</Alert.Title>
          <Alert.Description>{state.status.error}</Alert.Description>
          <Button
            variant="outline"
            onclick={() => controller.actions.selectWorkspace(null)}
            >Browse repository</Button
          >
        </Alert.Root>
      </section>
    {:else if page === "agents"}<StoreAgentSessions
        {controller}
        {layoutStorage}
      />
    {:else if page === "reviews"}<StoreReviewThreads
        {controller}
        {layoutStorage}
      />
    {:else if page === "files"}<CommunityProjectsRepositoryFiles
        {controller}
        now={state.now}
        {layoutStorage}
      />
    {:else}<CommunityProjectsRepositoryLog
        {controller}
        {page}
        graph={state.graph}
        bookmarks={[...state.bookmarks]}
        workingCopies={[...state.workingCopies]}
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
        hasMoreBookmarks={state.canLoadMoreBookmarks}
        hasMoreGraph={state.canLoadMoreGraph}
        hasMoreFiles={state.canLoadMoreDetail}
        unavailableComparisonCount={state.unavailableComparisonCount}
        loading={state.status.graph === "loading" ||
          state.status.graph === "refreshing" ||
          state.status.bookmarks === "loading" ||
          state.status.bookmarks === "refreshing"}
        detailLoading={state.status.detail === "loading"}
        filesLoading={state.status.detail === "refreshing"}
        now={state.now}
        {layoutStorage}
        onSelectBranch={controller.actions.selectBookmark}
        onSelectWorkingCopy={controller.actions.selectWorkingCopy}
        onSelectRevision={controller.actions.selectRevision}
        onSelectPath={controller.actions.selectFile}
        onLoadMoreBookmarks={() => controller.actions.loadMore("bookmarks")}
        onLoadMoreGraph={() => controller.actions.loadMore("graph")}
        onLoadMoreFiles={() => controller.actions.loadMore("detail")}
      />{/if}
  {/snippet}
</StoreApplication>

<style>
  .store-repository-application__state {
    padding: 1rem;
  }
  .store-repository-application__choices {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding-block-start: 0.75rem;
  }
</style>
