<script lang="ts">
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreApplication from "./StoreApplication.svelte";
  import CommunityProjectsRepositoryLog from "../community-projects/CommunityProjectsRepositoryLog.svelte";
  import CommunityProjectsRepositoryFiles from "../community-projects/CommunityProjectsRepositoryFiles.svelte";
  import StoreAgentSessions from "../store-collaboration/StoreAgentSessions.svelte";
  import StoreReviewThreads from "../store-collaboration/StoreReviewThreads.svelte";
  import StoreRepositoryDetails from "./StoreRepositoryDetails.svelte";
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
    {#if needsOperation}<section class="store-repository-application__state">
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
  {#snippet details(page)}
    <StoreRepositoryDetails
      {controller}
      title={page === "files"
        ? "Files"
        : page === "log"
          ? "JJ Log"
          : page === "oplog"
            ? "Op Log"
            : page[0]!.toUpperCase() + page.slice(1)}
    />
    {#if !selectedId && (state.backups.length || state.conflictCount !== null)}
      <dl class="store-repository-application__details">
        <dt>Snapshot</dt>
        <dd>
          {state.route.snapshotOperationId?.slice(0, 12) ?? "Choose a head"}
        </dd>
        {#each state.backups as backup (backup.target)}
          <dt>Backup · {backup.target}</dt>
          <dd>
            Transfer: {backup.transfer.state} · {backup.transfer
              .completed_objects}
            objects<br />
            Last verified restore point:
            {backup.last_verified_restore_point
              ? `${backup.last_verified_restore_point.operation.slice(0, 12)} at position ${backup.last_verified_restore_point.position}`
              : "none"}
          </dd>
        {/each}
        {#if state.conflictCount !== null}<dt>Conflicts</dt>
          <dd>{state.conflictCount}</dd>{/if}
      </dl>
    {/if}
    {#if state.comparisonParents.length}<Alert.Root>
        <Alert.Title>Choose comparison parent</Alert.Title>
        <Alert.Description
          >This merge operation has multiple recorded predecessors.</Alert.Description
        >
        {#each state.comparisonParents as parent}<Button
            variant="outline"
            onclick={() => controller.actions.selectComparisonParent(parent)}
            >{parent.slice(0, 12)}</Button
          >{/each}
      </Alert.Root>{/if}
    {#if state.status.error}<Alert.Root variant="destructive">
        <Alert.Title>Repository request failed</Alert.Title>
        <Alert.Description>{state.status.error}</Alert.Description>
        {#if state.status.error.includes("index rebuild required")}<Button
            variant="outline"
            onclick={() => controller.actions.prepareIndex()}
            >Prepare index</Button
          >{/if}
      </Alert.Root>{/if}
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
  .store-repository-application__details {
    padding: 0.75rem;
    font-size: var(--community-text-body);
  }
  .store-repository-application__details dd {
    margin: 0 0 1rem;
    color: var(--muted-foreground);
    overflow-wrap: anywhere;
  }
</style>
