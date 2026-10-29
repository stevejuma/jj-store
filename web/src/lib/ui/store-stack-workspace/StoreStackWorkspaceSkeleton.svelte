<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import type { StoreRoute } from "../../controller/contracts.js";
  import type { StoreStacksView } from "../../controller/store-stacks-navigation.js";
  import StoreStackHistorySkeleton from "./StoreStackHistorySkeleton.svelte";
  import RepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import StoreRepositoryHistorySkeleton from "../store-application/StoreRepositoryHistorySkeleton.svelte";
  import "./stack-workspace.css";
  import "./stack-workspace-layout.css";
  let {
    view = "workspace",
    route,
    layoutStorage,
    navigation = false,
  }: {
    view?: StoreStacksView;
    route?: StoreRoute;
    layoutStorage?: Storage;
    navigation?: boolean;
  } = $props();
  const history = $derived(view === "log" || view === "oplog");
  const canvas = untrack(() =>
    ColumnCanvas.createColumnCanvasController({
      trailingSpacerWidth: view === "workspace" ? 390 : 240,
      columns: {
        working: {
          defaultWidth: 330,
          minWidth: 260,
          maxWidth: 520,
          pathLevel: 0,
        },
        branches: {
          defaultWidth: 320,
          minWidth: 260,
          maxWidth: 520,
          pathLevel: 0,
        },
        stack: {
          defaultWidth: 390,
          minWidth: 310,
          maxWidth: 640,
          pathLevel: 0,
        },
      },
    }),
  );
  onDestroy(() => void canvas.dispose());
  const historyRoute = $derived(
    route ?? {
      page: view === "oplog" ? ("oplog" as const) : ("log" as const),
      bookmark: "",
      inspectedOperationId: null,
      commitId: null,
      path: null,
    },
  );
  let viewport = $state<HTMLElement | null>(null);
</script>

<div
  class="stack-workspace stack-workspace-skeleton"
  data-page={view}
  data-view={view}
  role="status"
  aria-label="Loading stack workspace"
  aria-busy="true"
>
  {#if navigation}<div
      class="stack-workspace-skeleton__navigation"
      aria-hidden="true"
      inert
    >
      <Skeleton class="stack-workspace-skeleton__back" />
      {#each [0, 1, 2, 3] as tab}<Skeleton />{/each}
      <div class="stack-workspace-skeleton__navigation-actions">
        <Skeleton /><Skeleton /><Skeleton />
      </div>
    </div>{/if}
  {#if history}
    <StoreRepositoryHistorySkeleton
      route={historyRoute}
      {layoutStorage}
      decorative
    />
  {:else}
    <div class="stack-workspace__views" aria-hidden="true" inert>
      <div class="stack-workspace__canvas-view">
        <div class="stack-workspace__content">
          <ScrollArea
            orientation="horizontal"
            scrollStrategy="styled"
            fillContent
            bind:viewportRef={viewport}
            class="stack-workspace__canvas-scroll"
          >
            <ColumnCanvas.Root
              controller={canvas}
              displayMode="fixed"
              scrollViewport={viewport}
              class="stack-workspace__canvas"
            >
              <ColumnCanvas.Column
                id={view === "workspace" ? "working" : "branches"}
                fixed
                title={view === "workspace" ? "Unstaged" : "Branches"}
              >
                <ColumnCanvas.Body scrollStrategy="styled">
                  {#if view === "branches"}
                    <div class="stack-workspace-skeleton__target">
                      <RepositorySkeleton
                        layout="bookmarks"
                        label="Loading target branch"
                        count={1}
                      />
                    </div>
                    <div class="stack-workspace-skeleton__filters">
                      <Skeleton /><Skeleton /><Skeleton />
                    </div>
                    <RepositorySkeleton
                      layout="bookmarks"
                      label="Loading branches"
                      count={4}
                    />
                  {:else}<RepositorySkeleton
                      layout="changed-files"
                      label="Loading unstaged files"
                      count={6}
                    />{/if}
                </ColumnCanvas.Body>
              </ColumnCanvas.Column>
              {#if view === "workspace"}<ColumnCanvas.Column id="stack">
                  <ColumnCanvas.Header
                    style="--ui-skeleton-background: color-mix(in srgb, var(--foreground) 12%, var(--background))"
                    ><Skeleton class="stack-workspace-skeleton__column-title" />
                    <div class="stack-workspace-skeleton__header-actions">
                      <Skeleton /><Skeleton />
                    </div></ColumnCanvas.Header
                  >
                  <ColumnCanvas.Body scrollStrategy="styled">
                    <div class="stack-workspace-skeleton__composer">
                      <Skeleton /><Skeleton />
                    </div>
                    <div class="stack-workspace-skeleton__bookmark">
                      <StoreStackHistorySkeleton
                        label="Loading stack commits"
                        decorative
                      />
                    </div>
                  </ColumnCanvas.Body>
                </ColumnCanvas.Column>{/if}
            </ColumnCanvas.Root>
          </ScrollArea>
        </div>
        <footer class="stack-workspace__footer" data-stack-skeleton-footer>
          <div class="stack-workspace__footer-items">
            {#each [0, 1, 2, 3, 4] as item}<Skeleton />{/each}
          </div>
        </footer>
      </div>
    </div>
  {/if}
</div>

<style>
  .stack-workspace-skeleton__navigation {
    display: flex;
    flex: none;
    align-items: center;
    gap: 0.4rem;
    min-height: 3.25rem;
    box-sizing: border-box;
    padding: 0.55rem 1rem 0.55rem 0.5rem;
    overflow: hidden;
  }
  .stack-workspace-skeleton__navigation
    > :global([data-ui-component="skeleton"]) {
    width: 5rem;
    height: 1.8rem;
    border-radius: 999px;
    flex: none;
  }
  .stack-workspace-skeleton__navigation
    > :global(.stack-workspace-skeleton__back) {
    width: 1.8rem;
  }
  .stack-workspace-skeleton__navigation-actions {
    display: flex;
    gap: 0.5rem;
    margin-inline-start: auto;
  }
  .stack-workspace-skeleton__navigation-actions
    :global([data-ui-component="skeleton"]) {
    width: 1.8rem;
    height: 1.8rem;
    flex: none;
  }

  .stack-workspace-skeleton__filters {
    display: flex;
    gap: 0.35rem;
    padding: 0.5rem;
  }
  .stack-workspace-skeleton__filters :global([data-ui-component="skeleton"]) {
    width: 4.5rem;
    height: 1.75rem;
    border-radius: 999px;
  }
  .stack-workspace-skeleton__target {
    padding: 0.25rem 0.5rem;
  }
  :global(.stack-workspace-skeleton__column-title) {
    width: 9rem;
    height: 0.8rem;
  }
  .stack-workspace-skeleton__header-actions {
    display: flex;
    gap: 0.5rem;
    margin-inline-start: auto;
  }
  .stack-workspace-skeleton__header-actions
    :global([data-ui-component="skeleton"]) {
    width: 1.25rem;
    height: 1.25rem;
  }
  .stack-workspace-skeleton__composer {
    display: grid;
    gap: 0.5rem;
    padding: 0.5rem;
    margin: 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }
  .stack-workspace-skeleton__composer
    :global([data-ui-component="skeleton"]:first-child) {
    height: 4rem;
  }
  .stack-workspace-skeleton__composer
    :global([data-ui-component="skeleton"]:last-child) {
    height: 2rem;
  }
  .stack-workspace-skeleton__bookmark {
    margin: 0.75rem 0.5rem 0;
  }
  .stack-workspace-skeleton
    .stack-workspace__footer-items
    :global([data-ui-component="skeleton"]) {
    width: 7rem;
    max-width: 28vw;
    height: 1.4rem;
  }
  .stack-workspace-skeleton
    .stack-workspace__footer-items
    :global([data-ui-component="skeleton"]:first-child) {
    width: 15rem;
  }
  @media (prefers-reduced-motion: reduce) {
    .stack-workspace-skeleton :global([data-ui-component="skeleton"]) {
      animation: none;
    }
  }
</style>
