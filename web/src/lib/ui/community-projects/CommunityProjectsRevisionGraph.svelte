<script lang="ts">
  import type { Snippet } from "svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { StoreControllerState } from "../../controller/contracts.js";

  import CommunityProjectsRevisionGraphCopy from "./CommunityProjectsRevisionGraphCopy.svelte";
  import CommunityProjectsRevisionGraphIds from "./CommunityProjectsRevisionGraphIds.svelte";
  import CommunityProjectsVirtualRows from "./CommunityProjectsVirtualRows.svelte";
  import { estimateCommunityProjectRow } from "./virtual-project-row-estimates.js";
  import type {
    RepositoryLogDevice,
    RepositoryLogRevision,
  } from "../../projects/repository-log.js";
  import {
    REPOSITORY_LOG_NODE_RADIUS,
    REPOSITORY_LOG_ROW_HEIGHT,
    REPOSITORY_LOG_STROKE_WIDTH,
    buildRepositoryLogGraph,
    repositoryLogConvergePath,
    repositoryLogLaneStem,
    repositoryLogLaneX,
    repositoryLogMergePath,
    repositoryLogSplitPath,
    repositoryLogSvgWidth,
    type RepositoryLogGraphRow,
  } from "../../projects/repository-log-graph.js";

  let {
    revisions,
    members = [],
    now,
    devices = [],
    selectedId = null,
    graphLabel = "Revision graph",
    commitLabel = "commit ID",
    onSelect,
    selectedFiles,
    revisionHref,
    hasMore = false,
    loading = false,
    onLoadMore,
    collaborationContext,
  }: {
    revisions: RepositoryLogRevision[];
    members?: readonly CommunityMemberModel[];
    now?: number;
    devices?: RepositoryLogDevice[] | undefined;
    selectedId?: string | null | undefined;
    graphLabel?: string | undefined;
    commitLabel?: string | undefined;
    onSelect?: ((revision: RepositoryLogRevision) => void) | undefined;
    selectedFiles?: Snippet | undefined;
    revisionHref?:
      ((revision: RepositoryLogRevision) => string | undefined) | undefined;
    hasMore?: boolean | undefined;
    loading?: boolean | undefined;
    onLoadMore?: (() => void) | undefined;
    collaborationContext?: StoreControllerState | undefined;
  } = $props();

  const rows = $derived(buildRepositoryLogGraph(revisions, devices));
  const svgWidth = $derived(repositoryLogSvgWidth(rows));
  let rowHeights = $state<Record<string, number>>({});
  const radius = REPOSITORY_LOG_NODE_RADIUS;

  // Native bookmark names can wrap beyond Community's fixed device-label row.
  // Keep its graph paths, using the measured content height for their endpoints.
  function measureBookmarkRow(node: HTMLDivElement, id: string) {
    const observer = new ResizeObserver(() => {
      const height = node.getBoundingClientRect().height;
      if (rowHeights[id] !== height) rowHeights[id] = height;
    });
    observer.observe(node);
    return { destroy: () => observer.disconnect() };
  }

  function revisionEstimate(row: RepositoryLogGraphRow | undefined): number {
    return estimateCommunityProjectRow(
      row
        ? {
            kind: "repository-log",
            text: row.revision.message,
            detail: row.revision.eventId,
            selected: selectedId === row.revision.eventId,
          }
        : undefined,
    );
  }

  function laneColor(row: RepositoryLogGraphRow, lane: number): string {
    return row.lanes[lane]?.color ?? "var(--primary)";
  }
</script>

<div class="community-revision-graph" role="list" aria-label={graphLabel}>
  {#snippet revisionRow(row: RepositoryLogGraphRow)}
    {@const rowHeight =
      rowHeights[row.revision.eventId] ?? REPOSITORY_LOG_ROW_HEIGHT}
    {@const commitX = repositoryLogLaneX(row.commitLane)}
    {@const centerY = rowHeight / 2}
    <div
      role="listitem"
      class="community-revision-graph__item"
      class:community-revision-graph__item--selected={selectedId ===
        row.revision.eventId}
      aria-current={selectedId === row.revision.eventId ? "true" : undefined}
    >
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="community-revision-graph__row"
        class:community-revision-graph__row--tip={row.isTip}
        style:padding-left="{svgWidth}px"
        style:min-height="{REPOSITORY_LOG_ROW_HEIGHT}px"
        style:--community-revision-graph-row-height="{rowHeight}px"
        data-revision-id={row.revision.eventId}
        use:measureBookmarkRow={row.revision.eventId}
        onclick={() => onSelect?.(row.revision)}
      >
        <svg
          class="community-revision-graph__svg"
          width={svgWidth}
          height={rowHeight}
          aria-hidden="true"
        >
          {#each row.lanes as lane, index (index)}
            {@const x = repositoryLogLaneX(index)}
            {@const stem = repositoryLogLaneStem(row, index, rowHeight)}
            {#if stem.y1 !== stem.y2}
              <line
                x1={x}
                y1={stem.y1}
                x2={x}
                y2={stem.y2}
                stroke={lane.color}
                stroke-width={REPOSITORY_LOG_STROKE_WIDTH}
              />
            {/if}
          {/each}
          {#if row.splitFromLane != null}
            <path
              class="community-revision-graph__edge"
              d={repositoryLogSplitPath(
                repositoryLogLaneX(row.splitFromLane),
                commitX,
                rowHeight,
              )}
              stroke={laneColor(row, row.splitFromLane)}
              stroke-width={REPOSITORY_LOG_STROKE_WIDTH}
            />
          {/if}
          {#each row.mergeFromLanes as mergeLane (mergeLane)}
            <path
              class="community-revision-graph__edge"
              d={repositoryLogMergePath(
                commitX,
                repositoryLogLaneX(mergeLane),
                rowHeight,
              )}
              stroke={laneColor(row, mergeLane)}
              stroke-width={REPOSITORY_LOG_STROKE_WIDTH}
            />
          {/each}
          {#each row.convergingLanes as convergeLane (convergeLane)}
            <path
              class="community-revision-graph__edge"
              d={repositoryLogConvergePath(
                repositoryLogLaneX(convergeLane),
                commitX,
                rowHeight,
              )}
              stroke={laneColor(row, convergeLane)}
              stroke-width={REPOSITORY_LOG_STROKE_WIDTH}
            />
          {/each}
          <circle
            class="community-revision-graph__node"
            cx={commitX}
            cy={centerY}
            r={row.isTip ? radius + 1 : radius}
            fill={laneColor(row, row.commitLane)}
          />
        </svg>
        <CommunityProjectsRevisionGraphCopy
          {row}
          {members}
          {now}
          href={revisionHref?.(row.revision)}
          onSelect={onSelect ? () => onSelect?.(row.revision) : undefined}
          {collaborationContext}
        />
        <CommunityProjectsRevisionGraphIds
          changeId={row.changeIdEncoded}
          commitId={row.revision.eventId}
          {commitLabel}
          placement="inline"
        />
      </div>
      {#if selectedId === row.revision.eventId && selectedFiles}
        <div class="community-revision-graph__files">
          {@render selectedFiles()}
        </div>
      {/if}
    </div>
  {/snippet}
  <CommunityProjectsVirtualRows
    {rows}
    row={revisionRow}
    getKey={(row) => row.revision.eventId}
    estimateSize={revisionEstimate}
    scrollToKey={selectedId}
    {hasMore}
    {loading}
    loadingLabel="Loading more history"
    loadingAsListItem
    {onLoadMore}
    class="community-revision-graph__virtual"
  />
</div>

<style>
  .community-revision-graph {
    container-type: inline-size;
    min-width: 0;
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .community-revision-graph__item {
    position: relative;
    display: flex;
    min-width: 0;
    flex-direction: column;
  }

  .community-revision-graph__item--selected {
    background: color-mix(in srgb, var(--foreground) 12%, var(--background));
  }

  .community-revision-graph__row {
    position: relative;
    z-index: 1;
    display: flex;
    flex-wrap: wrap;
    box-sizing: border-box;
    min-width: 0;
    align-items: center;
    width: 100%;
    margin: 0;
    gap: 0.5rem;
    padding-right: 0.5rem;
    padding-block: 0.5rem;
    border: 0;
    border-bottom: 1px solid var(--border);
    border-radius: 0;
    color: inherit;
    background: transparent;
    font: inherit;
    text-align: left;
    cursor: pointer;
  }

  .community-revision-graph__row :global(.community-revision-graph-copy) {
    flex-basis: 12rem;
  }

  .community-revision-graph__row:hover,
  .community-revision-graph__row:focus-visible {
    background: color-mix(in srgb, var(--foreground) 7%, var(--background));
    outline: none;
  }

  .community-revision-graph__row--tip
    :global(.community-revision-graph-message) {
    font-weight: 600;
  }

  .community-revision-graph__svg {
    position: absolute;
    z-index: 2;
    top: 0;
    left: 0;
    overflow: visible;
    pointer-events: none;
  }

  .community-revision-graph__files {
    position: relative;
    z-index: 1;
    padding: 0 0.5rem 0.5rem;
  }

  .community-revision-graph__edge {
    fill: none;
  }

  @container (max-width: 440px) {
    .community-revision-graph__row {
      flex-wrap: wrap;
    }

    .community-revision-graph__row :global(.community-revision-graph-copy) {
      flex-basis: 100%;
    }
  }
</style>
