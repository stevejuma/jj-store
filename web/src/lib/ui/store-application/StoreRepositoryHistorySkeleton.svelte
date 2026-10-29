<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import type { StoreRoute } from "../../controller/contracts.js";
  import { createRepositoryLogCanvas } from "../../projects/repository-log-canvas.js";
  import { resolveLayoutStorage } from "../layout-storage.js";
  import RepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import "../community-projects/repository-log.css";
  let {
    route,
    label = "Loading repository history",
    layoutStorage,
    decorative = false,
  }: {
    route: Pick<
      StoreRoute,
      "page" | "bookmark" | "inspectedOperationId" | "commitId" | "path"
    >;
    label?: string;
    layoutStorage?: Storage;
    decorative?: boolean;
  } = $props();
  const canvas = untrack(() =>
    createRepositoryLogCanvas(route.page, resolveLayoutStorage(layoutStorage)),
  );
  onDestroy(() => void canvas.dispose());
  $effect(() => {
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

<section
  class="community-projects-repository-log repository-history-skeleton"
  role={decorative ? undefined : "status"}
  aria-label={decorative ? undefined : label}
  aria-busy={decorative ? undefined : "true"}
>
  <div class="repository-history-skeleton__canvas" aria-hidden="true" inert>
    <ColumnCanvas.Root controller={canvas}>
      {#if route.page === "branches"}<ColumnCanvas.Column
          id="branches"
          title="Branches"
        >
          <ColumnCanvas.Body
            ><RepositorySkeleton
              layout="bookmarks"
              label="Loading branches"
            /></ColumnCanvas.Body
          >
        </ColumnCanvas.Column>{/if}
      <ColumnCanvas.Column
        id="revisions"
        title={route.page === "oplog" ? "Op Log" : "JJ Log"}
      >
        <ColumnCanvas.Body
          ><RepositorySkeleton
            layout="graph"
            label={route.page === "oplog"
              ? "Loading operation log"
              : "Loading JJ log"}
            showStatus={route.page === "oplog"}
          /></ColumnCanvas.Body
        >
      </ColumnCanvas.Column>
      {#if route.path || (route.page === "oplog" && route.inspectedOperationId)}<ColumnCanvas.Column
          id="diff"
          title={route.path ?? "Operation details"}
        >
          <ColumnCanvas.Body
            ><RepositorySkeleton
              layout="diff"
              label={route.page === "oplog"
                ? "Loading selected operation comparison"
                : "Loading selected file diff"}
            /></ColumnCanvas.Body
          >
        </ColumnCanvas.Column>{/if}
    </ColumnCanvas.Root>
  </div>
</section>

<style>
  .repository-history-skeleton__canvas {
    display: flex;
    flex: 1;
    min-height: 0;
    min-width: 0;
  }
  @media (prefers-reduced-motion: reduce) {
    .repository-history-skeleton :global([data-ui-component="skeleton"]) {
      animation: none;
    }
  }
</style>
