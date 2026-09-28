<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import { onDestroy, untrack } from "svelte";
  import type { FileListingFile } from "@lapismd/design-core/diff";
  import type {
    RepositoryLogGraph,
    RepositoryWorkingCopy,
  } from "../../projects/repository-log.js";
  import type { Bookmark } from "../../projects/presentation.js";
  import type { StorePage } from "../community-navigation/model.js";
  import { createRepositoryLogCanvas } from "../../projects/repository-log-canvas.js";
  import CommunityProjectsRevisionGraph from "./CommunityProjectsRevisionGraph.svelte";
  import CommunityProjectsColumnAlert from "./CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsRepositoryLogHeader from "./CommunityProjectsRepositoryLogHeader.svelte";
  import CommunityProjectsRepositoryLogEmpty from "./CommunityProjectsRepositoryLogEmpty.svelte";
  import CommunityProjectsRepositoryLogDiff from "./CommunityProjectsRepositoryLogDiff.svelte";
  import CommunityProjectsRepositoryLogFiles from "./CommunityProjectsRepositoryLogFiles.svelte";
  import CommunityProjectsRepositorySkeleton from "./CommunityProjectsRepositorySkeleton.svelte";
  import CommunityRepositoryBranchCommand from "../community-repository-branch-command/CommunityRepositoryBranchCommand.svelte";
  import { resolveLayoutStorage } from "../layout-storage.js";
  let {
    page = "log",
    graph,
    bookmarks = [],
    workingCopies = [],
    selectedBranch = "",
    selectedId = null,
    selectedPath = null,
    files = [],
    patch,
    hasMoreBookmarks = false,
    hasMoreGraph = false,
    hasMoreFiles = false,
    unavailableComparisonCount = 0,
    loading = false,
    detailLoading = false,
    filesLoading = false,
    error = "",
    now,
    layoutStorage,
    onSelectBranch,
    onSelectWorkingCopy = () => {},
    onSelectRevision,
    onSelectPath,
    onLoadMoreBookmarks = () => {},
    onLoadMoreGraph = () => {},
    onLoadMoreFiles = () => {},
  }: {
    page?: StorePage;
    graph: RepositoryLogGraph;
    bookmarks?: readonly Bookmark[];
    workingCopies?: readonly RepositoryWorkingCopy[];
    selectedBranch?: string;
    selectedId?: string | null;
    selectedPath?: string | null;
    files?: FileListingFile[];
    patch?: string;
    hasMoreBookmarks?: boolean;
    hasMoreGraph?: boolean;
    hasMoreFiles?: boolean;
    unavailableComparisonCount?: number;
    loading?: boolean;
    detailLoading?: boolean;
    filesLoading?: boolean;
    error?: string;
    now: number;
    layoutStorage?: Storage;
    onSelectBranch: (name: string) => void;
    onSelectWorkingCopy?: (commit: string) => void;
    onSelectRevision: (id: string | null) => void;
    onSelectPath: (path: string | null) => void;
    onLoadMoreBookmarks?: () => void | Promise<void>;
    onLoadMoreGraph?: () => void | Promise<void>;
    onLoadMoreFiles?: () => void | Promise<void>;
  } = $props();
  const storage = untrack(() => resolveLayoutStorage(layoutStorage));
  const canvas = createRepositoryLogCanvas(
    untrack(() => page),
    storage,
  );
  onDestroy(() => void canvas.dispose());
  const revisionDepth = $derived(page === "branches" ? 1 : 0);
  const selectedOperation = $derived(
    page === "oplog"
      ? graph.revisions.find((r) => r.eventId === selectedId)
      : undefined,
  );
  const changedFilesTitle = $derived(
    page === "oplog" ? "Changed files in operation" : "Changed files",
  );
  const title = $derived(page === "oplog" ? "Op Log" : "JJ Log");
  const selectedBookmark = $derived(
    bookmarks.find((b) => b.name === selectedBranch),
  );
  const showInitialSkeleton = $derived(loading && graph.revisions.length === 0);
  $effect(() => {
    canvas.layoutReady;
    const branch = selectedBranch,
      revision = selectedId,
      path = selectedPath,
      depth = revisionDepth;
    untrack(() => {
      canvas.clear();
      if (depth === 1 && branch) canvas.select(0, branch);
      if (revision) {
        canvas.select(depth, revision);
        if (page === "oplog") canvas.open("diff");
      }
      if (path) {
        canvas.select(depth + 1, path);
        canvas.open("diff");
      }
    });
  });
  function selectRevision(id: string) {
    onSelectRevision(id === selectedId ? null : id);
  }
  function selectFile(path: string) {
    canvas.open("diff");
    onSelectPath(path);
  }
</script>

<section
  class="community-projects-repository-log"
  aria-label="Repository log"
  aria-busy={loading}
>
  {#if error}<CommunityProjectsColumnAlert
      title="History unavailable"
      description={error}
      tone="destructive"
    />{/if}
  <ColumnCanvas.Root controller={canvas}>
    {#if page === "branches"}<ColumnCanvas.Column
        id="branches"
        title="Branches"
        count={bookmarks.length}
      >
        <ColumnCanvas.Body>
          {#if showInitialSkeleton}<CommunityProjectsRepositorySkeleton
              layout="bookmarks"
              label="Loading branches"
            />{:else}<CommunityRepositoryBranchCommand
              {bookmarks}
              {workingCopies}
              {now}
              selected={selectedBranch}
              selectedCommit={selectedBranch ? null : selectedId}
              hasMore={hasMoreBookmarks}
              {loading}
              onSelect={onSelectBranch}
              {onSelectWorkingCopy}
              onLoadMore={() => void onLoadMoreBookmarks()}
            />{/if}
        </ColumnCanvas.Body>
      </ColumnCanvas.Column>{/if}
    <ColumnCanvas.Column id="revisions">
      <CommunityProjectsRepositoryLogHeader
        {title}
        count={graph.revisions.length}
      />
      <ColumnCanvas.Body>
        {#if showInitialSkeleton}<CommunityProjectsRepositorySkeleton
            layout="graph"
            label={page === "oplog"
              ? "Loading operation log"
              : "Loading JJ log"}
            showStatus={page === "oplog"}
          />{:else}
          {#if selectedBookmark && selectedBookmark.targets.length > 1}<CommunityProjectsColumnAlert
              title="Conflicted bookmark"
              description={`All ${selectedBookmark.targets.length} targets are shown. No target is implicitly selected.`}
            />{/if}
          {#if page === "oplog"}<p class="store-operation-heads" role="status">
              {graph.devices.length} operation heads · selecting one does not reconcile
              history
            </p>{/if}
          {#if unavailableComparisonCount > 0}<CommunityProjectsColumnAlert
              title="Comparison needs a target"
              description={`${unavailableComparisonCount === 1 ? "One affected change has" : `${unavailableComparisonCount} affected changes have`} ambiguous lineage and cannot be compared until an explicit target is selected.`}
            />{/if}
          {#if graph.revisions.length === 0}<CommunityProjectsRepositoryLogEmpty
              pane={page === "oplog" ? "oplog" : "log"}
              branch={selectedBranch}
            />
          {:else}<CommunityProjectsRevisionGraph
              revisions={graph.revisions}
              devices={graph.devices}
              {selectedId}
              {now}
              graphLabel={page === "oplog"
                ? "Operation graph"
                : "Revision graph"}
              commitLabel={page === "oplog" ? "operation ID" : "commit ID"}
              onSelect={(revision) => selectRevision(revision.eventId)}
              hasMore={hasMoreGraph}
              {loading}
              onLoadMore={() => void onLoadMoreGraph()}
            >
              {#snippet selectedFiles()}
                <CommunityProjectsRepositoryLogFiles
                  compact
                  title={changedFilesTitle}
                  {files}
                  {selectedPath}
                  hasMore={hasMoreFiles}
                  initialLoading={detailLoading}
                  loading={filesLoading}
                  loadingLabel={page === "oplog"
                    ? "Loading changed files in operation"
                    : "Loading changed files"}
                  onSelectPath={selectFile}
                  onLoadMore={() => void onLoadMoreFiles()}
                />
              {/snippet}
            </CommunityProjectsRevisionGraph>{/if}
        {/if}
      </ColumnCanvas.Body>
    </ColumnCanvas.Column>
    <CommunityProjectsRepositoryLogDiff
      path={selectedPath}
      {patch}
      {files}
      operation={selectedOperation}
      loading={detailLoading}
      {now}
      onSelectOperation={onSelectRevision}
    />
  </ColumnCanvas.Root>
</section>

<style>
  .store-operation-heads {
    margin: 0;
    padding: 0.5rem;
    background: var(--jj-store-info-row-background);
    border-bottom: 1px solid var(--border);
    font-size: var(--community-text-secondary);
    color: var(--muted-foreground);
  }

  .community-projects-repository-log {
    display: flex;
    min-height: 0;
    height: 100%;
    flex: 1 1 auto;
    flex-direction: column;
    width: 100%;
    overflow: hidden;
  }

  .community-projects-repository-log
    :global([data-ui-component="column-canvas"][data-ui-part="root"]) {
    min-height: 0;
    height: 100%;
    flex: 1 1 auto;
  }
</style>
