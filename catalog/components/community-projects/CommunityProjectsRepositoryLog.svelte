<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import { untrack } from "svelte";
  import type { FileListingFile } from "@lapismd/design-core/diff";
  import type { RepositoryLogGraph } from "../../projects/repository-log.js";
  import type { Bookmark } from "../../projects/presentation.js";
  import type { StorePage } from "../community-navigation/model.js";
  import { createRepositoryLogCanvas } from "../../projects/repository-log-canvas.js";
  import CommunityProjectsRevisionGraph from "./CommunityProjectsRevisionGraph.svelte";
  import CommunityProjectsRepositoryLogHeader from "./CommunityProjectsRepositoryLogHeader.svelte";
  import CommunityProjectsRepositoryLogEmpty from "./CommunityProjectsRepositoryLogEmpty.svelte";
  import CommunityProjectsRepositoryLogDiff from "./CommunityProjectsRepositoryLogDiff.svelte";
  import CommunityProjectsRepositoryLogFiles from "./CommunityProjectsRepositoryLogFiles.svelte";
  import CommunityProjectsRepositoryLogRail from "./CommunityProjectsRepositoryLogRail.svelte";
  import CommunityRepositoryBranchCommand from "../community-repository-branch-command/CommunityRepositoryBranchCommand.svelte";
  let {
    page = "log",
    graph,
    bookmarks = [],
    selectedBranch = "",
    selectedId = null,
    selectedPath = null,
    files = [],
    patch,
    loading = false,
    error = "",
    now,
    onSelectBranch,
    onSelectRevision,
    onSelectPath,
  }: {
    page?: StorePage;
    graph: RepositoryLogGraph;
    bookmarks?: readonly Bookmark[];
    selectedBranch?: string;
    selectedId?: string | null;
    selectedPath?: string | null;
    files?: FileListingFile[];
    patch?: string;
    loading?: boolean;
    error?: string;
    now: number;
    onSelectBranch: (name: string) => void;
    onSelectRevision: (id: string | null) => void;
    onSelectPath: (path: string | null) => void;
  } = $props();
  const canvas = createRepositoryLogCanvas(untrack(() => page === "branches"));
  const revisionDepth = $derived(page === "branches" ? 1 : 0);
  const selectedOperation = $derived(
    page === "oplog"
      ? graph.revisions.find((r) => r.eventId === selectedId)
      : undefined,
  );
  const title = $derived(page === "oplog" ? "Op Log" : "JJ Log");
  const selectedBookmark = $derived(
    bookmarks.find((b) => b.name === selectedBranch),
  );
  $effect(() => {
    const branch = selectedBranch,
      revision = selectedId,
      path = selectedPath,
      depth = revisionDepth;
    untrack(() => {
      canvas.clear();
      if (depth === 1 && branch) canvas.select(0, branch);
      if (revision) {
        canvas.select(depth, revision);
        canvas.open("operation");
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
  {#if error}<Alert.Root variant="destructive"
      ><Alert.Title>History unavailable</Alert.Title><Alert.Description
        >{error}</Alert.Description
      ></Alert.Root
    >{/if}
  {#if loading && graph.revisions.length === 0}
    <div class="community-projects-repository-log__skeleton" aria-busy="true">
      <span class="sr-only">Loading repository log</span><Skeleton /><Skeleton
      /><Skeleton />
    </div>
  {:else}
    <ColumnCanvas.Root controller={canvas}>
      {#if page === "branches"}<ColumnCanvas.Column
          id="branches"
          title="Branches"
          count={bookmarks.length}
        >
          <ColumnCanvas.Body
            ><CommunityRepositoryBranchCommand
              {bookmarks}
              {now}
              selected={selectedBranch}
              onSelect={onSelectBranch}
            /></ColumnCanvas.Body
          >
        </ColumnCanvas.Column>{/if}
      <ColumnCanvas.Column id="revisions">
        <CommunityProjectsRepositoryLogHeader
          {title}
          count={graph.revisions.length}
        />
        <ColumnCanvas.Body>
          {#if selectedBookmark && selectedBookmark.targets.length > 1}<Alert.Root
              ><Alert.Title>Conflicted bookmark</Alert.Title><Alert.Description
                >All {selectedBookmark.targets.length} targets are shown. No target
                is implicitly selected.</Alert.Description
              ></Alert.Root
            >{/if}
          {#if page === "oplog"}<p class="store-operation-heads" role="status">
              {graph.devices.length} operation heads · selecting one does not reconcile
              history
            </p>{/if}
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
            >
              {#snippet selectedFiles()}{#if page !== "oplog"}<CommunityProjectsRepositoryLogFiles
                    compact
                    {files}
                    {selectedPath}
                    onSelectPath={selectFile}
                  />{/if}{/snippet}
            </CommunityProjectsRevisionGraph>{/if}
        </ColumnCanvas.Body>
      </ColumnCanvas.Column>
      {#if selectedOperation}<ColumnCanvas.Column
          id="operation"
          title="Operation details"
          ><ColumnCanvas.Body
            ><CommunityProjectsRepositoryLogRail
              operation={selectedOperation}
              {now}
              onSelect={onSelectRevision}
            /></ColumnCanvas.Body
          ></ColumnCanvas.Column
        >{/if}
      {#if page !== "oplog"}<CommunityProjectsRepositoryLogDiff
          path={selectedPath}
          {patch}
          {files}
        />{/if}
    </ColumnCanvas.Root>
  {/if}
</section>

<style>
  .store-operation-heads {
    padding: 0.5rem;
    font-size: var(--community-text-secondary);
    color: var(--muted-foreground);
  }

  .community-projects-repository-log {
    display: flex;
    min-height: 0;
    flex: 1 1 auto;
    flex-direction: column;
    width: 100%;
    overflow: hidden;
  }

  .community-projects-repository-log
    :global([data-ui-component="column-canvas"][data-ui-part="root"]) {
    min-height: 0;
    flex: 1 1 auto;
  }

  .community-projects-repository-log__skeleton {
    display: grid;
    gap: 0.75rem;
    padding: 1rem;
  }
</style>
