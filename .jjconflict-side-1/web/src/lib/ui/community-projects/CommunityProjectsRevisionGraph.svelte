<script lang="ts">
  import "./repository-row-tokens.css";
  import type { Snippet } from "svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { StoreControllerState } from "../../controller/contracts.js";
  import CommunityProjectsRevisionGraphSvg from "./CommunityProjectsRevisionGraphSvg.svelte";
  import CommunityProjectsRevisionGraphCopy from "./CommunityProjectsRevisionGraphCopy.svelte";
  import CommunityProjectsRevisionGraphIds from "./CommunityProjectsRevisionGraphIds.svelte";
  import CommunityProjectsVirtualRows from "./CommunityProjectsVirtualRows.svelte";
  import { estimateCommunityProjectRow } from "./virtual-project-row-estimates.js";
  import type {
    RepositoryLogDevice,
    RepositoryLogRevision,
  } from "../../projects/repository-log.js";
  import {
    REPOSITORY_LOG_ROW_HEIGHT,
    buildRepositoryLogGraph,
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
    rowActions,
    rowDropState,
    rowDropPlacement,
    avatarSize = "small",
    showSelectedChip = false,
    graphInset = 0,
    continuous = false,
    endBorder = true,
    draggable,
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
    rowActions?: Snippet<[RepositoryLogRevision]>;
    rowDropState?: (
      revision: RepositoryLogRevision,
    ) => "idle" | "valid" | "invalid";
    rowDropPlacement?: (
      revision: RepositoryLogRevision,
    ) => "above" | "below" | undefined;
    avatarSize?: "small" | "xsmall";
    showSelectedChip?: boolean;
    graphInset?: number;
    continuous?: boolean;
    endBorder?: boolean;
    draggable?: (revision: RepositoryLogRevision) => boolean;
    revisionHref?:
      ((revision: RepositoryLogRevision) => string | undefined) | undefined;
    hasMore?: boolean | undefined;
    loading?: boolean | undefined;
    onLoadMore?: (() => void) | undefined;
    collaborationContext?: StoreControllerState | undefined;
  } = $props();
  const rows = $derived(buildRepositoryLogGraph(revisions, devices));
  const svgWidth = $derived(Math.max(graphInset, repositoryLogSvgWidth(rows)));
  let itemHeights = $state<Record<string, number>>({});
  let rowHeights = $state<Record<string, number>>({});
  // Native bookmark names can wrap beyond Community's fixed device-label row.
  // Keep its graph paths, using the measured content height for their endpoints.
  function measureBookmarkRow(node: HTMLDivElement, id: string) {
    let frame: number | null = null;
    const observer = new ResizeObserver(() => {
      if (frame !== null) return;
      frame = requestAnimationFrame(() => {
        frame = null;
        const height = node.getBoundingClientRect().height;
        if (rowHeights[id] !== height) rowHeights[id] = height;
        const itemHeight =
          node.parentElement?.getBoundingClientRect().height ?? height;
        if (itemHeights[id] !== itemHeight) itemHeights[id] = itemHeight;
      });
    });
    observer.observe(node);
    if (node.parentElement) observer.observe(node.parentElement);
    return {
      destroy() {
        observer.disconnect();
        if (frame !== null) cancelAnimationFrame(frame);
      },
    };
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
</script>

<div class="community-revision-graph" role="list" aria-label={graphLabel}>
  {#snippet revisionRow(row: RepositoryLogGraphRow)}
    {@const rowHeight =
      rowHeights[row.revision.eventId] ?? REPOSITORY_LOG_ROW_HEIGHT}
    {@const graphHeight = continuous
      ? (itemHeights[row.revision.eventId] ?? rowHeight)
      : rowHeight}
    <div
      role="listitem"
      class="community-revision-graph__item"
      class:community-revision-graph__item--continuous={continuous}
      style:--community-revision-graph-inset={`${svgWidth}px`}
      class:community-revision-graph__item--selected={selectedId ===
        row.revision.eventId}
      aria-current={selectedId === row.revision.eventId ? "true" : undefined}
    >
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="community-revision-graph__row"
        class:community-revision-graph__row--actions={Boolean(rowActions)}
        class:community-revision-graph__row--tip={row.isTip}
        class:community-revision-graph__row--borderless={!endBorder &&
          row.revision.eventId === rows.at(-1)?.revision.eventId}
        style:padding-left="{svgWidth}px"
        style:min-height="{REPOSITORY_LOG_ROW_HEIGHT}px"
        style:--community-revision-graph-row-height="{rowHeight}px"
        data-revision-id={row.revision.eventId}
        data-drop-target-state={rowDropState?.(row.revision)}
        data-drop-placement={rowDropPlacement?.(row.revision)}
        class:stack-workspace__drop-target={Boolean(rowDropState)}
        draggable={draggable?.(row.revision)}
        use:measureBookmarkRow={row.revision.eventId}
        onclick={() => onSelect?.(row.revision)}
      >
        <CommunityProjectsRevisionGraphSvg
          {row}
          width={svgWidth}
          {rowHeight}
          {graphHeight}
          {continuous}
        />
        {#snippet revisionContent()}
          <CommunityProjectsRevisionGraphCopy
            {row}
            {members}
            {avatarSize}
            selected={showSelectedChip && selectedId === row.revision.eventId}
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
        {/snippet}
        {#if rowActions}
          <div class="community-revision-graph__content">
            {@render revisionContent()}
          </div>
        {:else}{@render revisionContent()}{/if}
        {#if rowActions}<div class="community-revision-graph__actions">
            {@render rowActions(row.revision)}
          </div>{/if}
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
    background: var(--jj-store-repository-row-selected-background);
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

  .community-revision-graph__row--borderless {
    border-bottom: 0;
  }

  .community-revision-graph__row--actions {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
  }
  .community-revision-graph__content {
    display: flex;
    min-width: 0;
    flex-direction: column;
    gap: 0.4rem;
  }
  .community-revision-graph__actions {
    display: flex;
    align-items: center;
    justify-content: flex-end;
  }
  .community-revision-graph__row:hover,
  .community-revision-graph__row:focus-visible {
    background: var(--jj-store-repository-row-hover-background);
    outline: none;
  }

  .community-revision-graph__row--tip
    :global(.community-revision-graph-message) {
    font-weight: 600;
  }

  .community-revision-graph__files {
    position: relative;
    z-index: 1;
    padding: 0 0.5rem 0.5rem;
  }

  .community-revision-graph__item--continuous .community-revision-graph__files {
    padding-left: var(--community-revision-graph-inset);
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
