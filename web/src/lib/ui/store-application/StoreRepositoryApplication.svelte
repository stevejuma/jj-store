<script lang="ts">
  import StoreLazyView from "./StoreLazyView.svelte";
  import { views } from "./application-views.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import StoreApplication from "./StoreApplication.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    controller,
    stackWorkspace,
    initiallyCollapsed = true,
    displayMode = "auto",
    layoutStorage,
  }: {
    controller: StoreController;
    stackWorkspace?: StackWorkspaceController;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
    layoutStorage?: Storage;
  } = $props();
  const state = $derived(controller.state);
  const preparing = $derived(
    state.status.connection === "connecting" ||
      state.status.navigation === "loading" ||
      state.settings.indexPreparation === "loading" ||
      !!controller.historyQuery?.pending,
  );
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
  {stackWorkspace}
  {initiallyCollapsed}
  {displayMode}
  {layoutStorage}
>
  {#snippet children(page)}
    {#if isRepositoryProjects(state.route) && controller.projects}<StoreLazyView
        load={views.projectsRepository}
        label="Repository projects"
        >{#snippet content(View)}<View {controller} />{/snippet}</StoreLazyView
      >
    {:else if page === "projects" && controller.projects}<StoreLazyView
        load={views.projects}
        label="Projects"
        >{#snippet content(View)}<View {controller} />{/snippet}</StoreLazyView
      >
    {:else if page === "projects" && controller.work}<StoreLazyView
        load={views.work}
        label="Work"
        >{#snippet content(View)}<View
            work={controller.work!}
            principal={state.repository.principal}
            repositories={state.repositories}
            onOpenRepository={controller.actions.openRepository}
            writable={state.accessMode === "local_trusted" ||
              state.repository.permission === "write" ||
              state.repository.permission === "admin"}
          />{/snippet}</StoreLazyView
      >
    {:else if page === "threads"}<StoreLazyView
        load={views.threads}
        label="Threads"
        >{#snippet content(View)}<View {controller} />{/snippet}</StoreLazyView
      >
    {:else if needsOperation}<section
        class="store-repository-application__state"
      >
        <StoreNotice
          noticeKey="operation-head-selection"
          severity="warning"
          title="Multiple operation heads"
          message="Choose the operation snapshot to browse. Opening this view never reconciles history."
        />
        <div
          class="store-repository-application__choices"
          role="group"
          aria-label="Choose operation snapshot"
        >
          {#each state.operationHeads as operation}<Button
              variant="outline"
              onclick={() => controller.actions.selectOperation(operation)}
              >{operation.slice(0, 12)}</Button
            >{/each}
        </div>
      </section>
    {:else if state.route.workspace && state.status.navigation === "failed"}
      <section class="store-repository-application__state">
        <Button
          variant="outline"
          onclick={() => controller.actions.selectWorkspace(null)}
          >Browse repository</Button
        >
      </section>
    {:else if page === "agents"}<StoreLazyView
        load={views.agentsRecorded}
        label="Agent sessions"
        >{#snippet content(View)}<View {controller} />{/snippet}</StoreLazyView
      >
    {:else if page === "reviews"}<StoreLazyView
        load={views.reviews}
        label="Reviews"
        >{#snippet content(View)}<View
            {controller}
            {layoutStorage}
          />{/snippet}</StoreLazyView
      >
    {:else if page === "files"}<StoreLazyView load={views.files} label="Files"
        >{#snippet content(View)}<View
            {controller}
            now={state.now}
            {layoutStorage}
          />{/snippet}</StoreLazyView
      >
    {:else}<StoreLazyView load={views.log} label="History"
        >{#snippet content(View)}<View
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
              preparing}
            referencesLoading={state.status.bookmarks === "loading" ||
              state.status.bookmarks === "refreshing" ||
              preparing}
            detailLoading={state.status.detail === "loading" ||
              (preparing && !!selectedId && !state.detail)}
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
          />{/snippet}</StoreLazyView
      >{/if}
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
