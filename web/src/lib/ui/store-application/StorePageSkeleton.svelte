<script lang="ts">
  import { untrack, onDestroy } from "svelte";
  import { AppShell } from "@lapismd/design-core/shell";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import WorkflowPageSkeleton from "../store-workflows/WorkflowPageSkeleton.svelte";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import type { StoreRoute } from "../../controller/contracts.js";
  import { createRepositoryLogCanvas } from "../../projects/repository-log-canvas.js";
  import {
    loadRepositoryFileTreeLayout,
    resolveLayoutStorage,
  } from "../layout-storage.js";
  import CommunityInboxSkeleton from "../community-inbox/CommunityInboxSkeleton.svelte";
  import RepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import StoreReviewDetailSkeleton from "../store-collaboration/StoreReviewDetailSkeleton.svelte";
  import StoreCollaborationSkeleton from "../store-collaboration/StoreCollaborationSkeleton.svelte";
  import StoreProjectsLoading from "../store-work/StoreProjectsLoading.svelte";
  import StoreWorkRowsSkeleton from "../store-work/StoreWorkRowsSkeleton.svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import "../community-projects/repository-files.css";
  import "../community-projects/repository-log.css";
  import "../store-work/store-projects.css";
  import "../community-agents/agents.css";
  let {
    route,
    label,
    surface = "repository",
    layoutStorage,
    writable = false,
  }: {
    route: StoreRoute;
    label: string;
    surface?: "repository" | "projects" | "agents" | "threads";
    layoutStorage?: Storage;
    writable?: boolean;
  } = $props();
  const storage = untrack(() => resolveLayoutStorage(layoutStorage));
  const tree = untrack(() => loadRepositoryFileTreeLayout(storage));
  const history = $derived(
    surface === "repository" &&
      ["branches", "log", "oplog"].includes(route.page),
  );
  const canvas = untrack(() =>
    history ? createRepositoryLogCanvas(route.page, storage) : undefined,
  );
  onDestroy(() => void canvas?.dispose());
  $effect(() => {
    if (!canvas) return;
    canvas.layoutReady;
    const current = route;
    untrack(() => {
      canvas.clear();
      const depth = current.page === "branches" ? 1 : 0;
      if (depth && current.bookmark) canvas.select(0, current.bookmark);
      const revision =
        current.page === "oplog"
          ? current.inspectedOperationId
          : current.commitId;
      if (revision) canvas.select(depth, revision);
      if (current.path) canvas.select(depth + 1, current.path);
      if (current.path || (current.page === "oplog" && revision))
        canvas.open("diff");
    });
  });
</script>

<div
  class="store-page-skeleton"
  role="status"
  aria-label={`Loading ${label}`}
  aria-busy="true"
  data-surface={surface}
>
  {#if route.page === "actions"}<WorkflowPageSkeleton
      view={route.workflow?.run ? "detail" : "history"}
      sourceRepository={route.repository}
    />
  {:else if surface === "threads" || (surface === "agents" && route.agentView && route.agentView !== "directory")}
    <CommunityInboxSkeleton
      kind={surface === "threads" ? "thread" : "session"}
      selected={surface === "threads"
        ? !!route.threadId
        : !!route.executionSessionId || route.agentView === "new"}
      {layoutStorage}
    />
  {:else if surface === "agents"}
    <AppShell.Body layout="regions" label="Agents workspace"
      ><AppShell.Body.Content label="Agents directory">
        <div class="agents" aria-hidden="true">
          <header>
            <div class="store-page-skeleton__gallery-heading">
              <Skeleton class="store-page-skeleton__title" /><Skeleton
                class="store-page-skeleton__subtitle"
              />
            </div>
            <Skeleton class="store-page-skeleton__action" />
          </header>
          <CommunitySkeleton
            layout="gallery"
            galleryVariant="agents"
            count={6}
            label="Loading your agents"
          />
        </div>
      </AppShell.Body.Content></AppShell.Body
    >
  {:else if surface === "projects"}
    <div class="store-projects community-projects">
      {#if !route.taskId}
        {#if route.projectsHub || route.projectId}<div
            class="store-page-skeleton__tabs"
            aria-hidden="true"
          >
            {#each [0, 1, 2, 3] as tab}<Skeleton />{/each}
          </div>{/if}
        {@render heading()}
      {/if}
      <StoreProjectsLoading {route} />
    </div>
  {:else if route.page === "files"}
    {#if tree.open}<AppShell.Body.Sidebar
        panelId="repository-file-tree"
        side="left"
        label="Repository file tree"
        open
        class="community-projects__file-tree-region"
        style={`--ui-shell-body-sidebar-width: ${tree.width}px`}
      >
        <RepositorySkeleton
          layout="file-tree"
          label="Loading repository file tree"
        />
      </AppShell.Body.Sidebar>{/if}
    <AppShell.Body.Content
      label="Repository file browser"
      class="community-projects__files-content"
    >
      <div class="community-projects__files-surface">
        <div class="store-page-skeleton__file-heading" aria-hidden="true">
          <Skeleton class="store-page-skeleton__title" />
        </div>
        <RepositorySkeleton
          layout={route.path ? "source" : "directory"}
          label="Loading repository files"
        />
      </div>
    </AppShell.Body.Content>
  {:else if canvas}
    <section class="community-projects-repository-log">
      <ColumnCanvas.Root controller={canvas}>
        {#if route.page === "branches"}<ColumnCanvas.Column
            id="branches"
            title="Branches"
            ><ColumnCanvas.Body
              ><RepositorySkeleton
                layout="bookmarks"
                label="Loading branches"
              /></ColumnCanvas.Body
            ></ColumnCanvas.Column
          >{/if}
        <ColumnCanvas.Column
          id="revisions"
          title={route.page === "oplog" ? "Op Log" : "JJ Log"}
          ><ColumnCanvas.Body
            ><RepositorySkeleton
              layout="graph"
              label={route.page === "oplog"
                ? "Loading operation log"
                : "Loading JJ log"}
              showStatus={route.page === "oplog"}
            /></ColumnCanvas.Body
          ></ColumnCanvas.Column
        >
        {#if route.path || (route.page === "oplog" && route.inspectedOperationId)}<ColumnCanvas.Column
            id="diff"
            title={route.path ?? "Operation details"}
            ><ColumnCanvas.Body
              ><RepositorySkeleton
                layout="diff"
                label={route.page === "oplog"
                  ? "Loading selected operation comparison"
                  : "Loading selected file diff"}
              /></ColumnCanvas.Body
            ></ColumnCanvas.Column
          >{/if}
      </ColumnCanvas.Root>
    </section>
  {:else if route.page === "reviews" && route.reviewThreadId}
    <StoreReviewDetailSkeleton {writable} />
  {:else}
    <div class="store-projects community-projects">
      {@render heading()}
      <div class="store-projects__loading">
        {#if route.page === "agents"}<StoreCollaborationSkeleton
            label="Loading agent sessions"
            agents
          />{:else}<StoreWorkRowsSkeleton
            label="Loading review threads"
            kind="reviews"
          />{/if}
      </div>
    </div>
  {/if}
</div>

{#snippet heading()}<header
    class="community-projects__surface-header"
    aria-hidden="true"
  >
    <div>
      <Skeleton class="store-page-skeleton__action" /><Skeleton
        class="store-page-skeleton__title"
      />
    </div>
    <div>
      {#each [0, 1, 2] as action}<Skeleton
          class="store-page-skeleton__action"
        />{/each}
    </div>
  </header>{/snippet}

<style>
  .store-page-skeleton {
    display: flex;
    flex: 1;
    width: 100%;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
  }
  .store-page-skeleton[data-surface="projects"] {
    flex-direction: column;
  }
  .store-page-skeleton .community-projects__surface-header > div {
    display: flex;
    align-items: center;
    gap: 0.65rem;
  }
  .store-page-skeleton .community-projects__surface-header > div:first-child {
    flex: 1;
  }
  .store-page-skeleton__tabs {
    display: flex;
    align-items: center;
    min-height: 3.25rem;
    padding-inline: 0.5rem 1rem;
    gap: 0.5rem;
    overflow: hidden;
  }
  .store-page-skeleton__tabs :global([data-ui-component="skeleton"]) {
    width: 5rem;
    height: 1.8rem;
    flex: none;
    border-radius: 999px;
  }
  :global(.store-page-skeleton__title) {
    width: min(45%, 12rem);
    height: 1rem;
  }
  :global(.store-page-skeleton__subtitle) {
    width: 15rem;
    max-width: 100%;
    height: 0.85rem;
  }
  :global(.store-page-skeleton__action) {
    width: 1.8rem;
    height: 1.8rem;
    flex: none;
  }
  .store-page-skeleton__gallery-heading {
    display: grid;
    gap: 0.5rem;
    flex: 1;
  }
  .store-page-skeleton__file-heading {
    display: flex;
    align-items: center;
    min-height: 2.75rem;
    padding-inline: 0.5rem;
    border-bottom: 1px solid var(--border);
  }
</style>
