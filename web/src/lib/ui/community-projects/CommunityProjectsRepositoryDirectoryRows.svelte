<script lang="ts">
  import DirectoryAttribution from "./CommunityProjectsRepositoryDirectoryAttribution.svelte";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import { createVirtualizer } from "@tanstack/svelte-virtual";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Table from "@lapismd/design-core/shadcn/table";
  import FileIcon from "@lucide/svelte/icons/file";
  import FolderIcon from "@lucide/svelte/icons/folder";
  import { get } from "svelte/store";
  import { onDestroy, tick, untrack } from "svelte";
  import type { RepositoryCommitSummary } from "../../projects/repository-log.js";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import type { RepositoryBrowserEntry } from "./repository-browser.js";
  import { repositoryParentPath } from "./repository-browser.js";

  type DirectoryRow =
    | { kind: "parent"; path: string }
    | { kind: "entry"; entry: RepositoryBrowserEntry };

  let {
    currentPath,
    entries,
    pathCommits,
    canLoadPathCommit = false,
    now,
    hasMore = false,
    loading = false,
    onOpenDirectory,
    onOpenFile,
    onSelectCommit,
    onLoadMore,
  }: {
    currentPath: string;
    entries: readonly RepositoryBrowserEntry[];
    pathCommits: Readonly<Record<string, RepositoryCommitSummary | undefined>>;
    canLoadPathCommit?: boolean;
    now: number;
    hasMore?: boolean;
    loading?: boolean;
    onOpenDirectory: (path: string) => void;
    onOpenFile: (path: string) => void;
    onSelectCommit?: (commit: RepositoryCommitSummary) => void;
    onLoadMore?: () => void;
  } = $props();

  const rows = $derived<DirectoryRow[]>([
    ...(currentPath
      ? [{ kind: "parent" as const, path: repositoryParentPath(currentPath) }]
      : []),
    ...entries.map((entry) => ({ kind: "entry" as const, entry })),
  ]);
  let host = $state<HTMLTableSectionElement | null>(null);
  let scrollElement = $state<HTMLElement | null | undefined>(undefined);
  let alive = true;
  let lastLoadMoreKey = "";

  const virtualizer = createVirtualizer<HTMLElement, HTMLTableRowElement>({
    count: 0,
    getScrollElement: () => scrollElement ?? null,
    getItemKey: (index) => rowKey(rows[index], index),
    estimateSize: () => 38,
    enabled: false,
    overscan: 8,
    useAnimationFrameWithResizeObserver: true,
  });

  onDestroy(() => {
    alive = false;
  });

  function rowKey(row: DirectoryRow | undefined, index: number): string {
    return row?.kind === "entry" ? row.entry.path : `parent:${index}`;
  }
  function nearestScrollElement(node: HTMLElement): HTMLElement | null {
    let current = node.parentElement;
    while (current) {
      const style = getComputedStyle(current);
      if (
        current.dataset.uiPart === "scroll-area-viewport" ||
        /(auto|scroll|overlay)/u.test(`${style.overflowY} ${style.overflow}`)
      )
        return current;
      current = current.parentElement;
    }
    return null;
  }
  function measure(node: HTMLTableRowElement) {
    untrack(() => get(virtualizer).measureElement(node));
    return {
      destroy() {
        untrack(() => {
          if (alive) get(virtualizer).measureElement(null);
        });
      },
    };
  }
  async function focusRow(index: number): Promise<void> {
    get(virtualizer).scrollToIndex(index, { align: "auto" });
    await tick();
    requestAnimationFrame(() =>
      host
        ?.querySelector<HTMLElement>(`[data-row-index="${index}"] button`)
        ?.focus(),
    );
  }
  function handleKeydown(event: KeyboardEvent): void {
    if (!["ArrowDown", "ArrowUp", "Home", "End"].includes(event.key)) return;
    const row = (event.target as HTMLElement).closest<HTMLElement>(
      "[data-row-index]",
    );
    const current = Number(row?.dataset.rowIndex ?? 0);
    const next =
      event.key === "Home"
        ? 0
        : event.key === "End"
          ? rows.length - 1
          : event.key === "ArrowUp"
            ? Math.max(0, current - 1)
            : Math.min(rows.length - 1, current + 1);
    event.preventDefault();
    void focusRow(next);
  }

  $effect(() => {
    scrollElement = host ? nearestScrollElement(host) : undefined;
  });
  $effect(() => {
    const nextRows = rows,
      nextScroll = scrollElement,
      nextHost = host;
    if (!nextHost) return;
    untrack(() =>
      get(virtualizer).setOptions({
        enabled: nextScroll != null,
        count: nextRows.length,
        getScrollElement: () => nextScroll ?? null,
        getItemKey: (index) => rowKey(nextRows[index], index),
        estimateSize: () => 38,
        overscan: 8,
        scrollMargin: nextHost.offsetTop,
      }),
    );
  });
  $effect(() => {
    const last = $virtualizer.getVirtualItems().at(-1);
    if (!last || !hasMore || loading || !onLoadMore || rows.length === 0)
      return;
    if (last.index < Math.max(0, rows.length - 8)) return;
    const key = `${rows.length}:${rowKey(rows.at(-1), rows.length - 1)}`;
    if (key === lastLoadMoreKey) return;
    lastLoadMoreKey = key;
    onLoadMore();
  });
</script>

{#snippet directoryCells(row: DirectoryRow, index: number)}
  {@const entry = row.kind === "entry" ? row.entry : undefined}
  {@const entryCommit = entry ? pathCommits[entry.path] : undefined}
  {@const commitLoaded = entry
    ? Object.prototype.hasOwnProperty.call(pathCommits, entry.path)
    : true}
  <Table.Cell>
    <Button
      type="button"
      variant="ghost"
      size="sm"
      class="community-projects__file-entry"
      aria-label={row.kind === "parent"
        ? "Go to parent directory"
        : `${entry?.type === "tree" ? "Open directory" : "Open file"} ${entry?.name}`}
      onclick={() =>
        row.kind === "parent"
          ? onOpenDirectory(row.path)
          : entry?.type === "tree"
            ? onOpenDirectory(entry.path)
            : entry && onOpenFile(entry.path)}
      onkeydown={handleKeydown}
    >
      {#if row.kind === "parent" || entry?.type === "tree"}
        <FolderIcon data-icon="inline-start" aria-hidden="true" />
      {:else}
        <FileIcon data-icon="inline-start" aria-hidden="true" />
      {/if}
      <strong>{row.kind === "parent" ? ".." : entry?.name}</strong>
    </Button>
  </Table.Cell>
  <Table.Cell>
    {#if canLoadPathCommit && !commitLoaded}
      <div
        role="status"
        aria-label={`Loading metadata for ${entry?.path}`}
        aria-busy="true"
      >
        <Skeleton style="height: 1rem; width: 100%" />
      </div>
    {:else}
      <DirectoryAttribution
        commit={entryCommit}
        path={entry?.path}
        {onSelectCommit}
      />
    {/if}
  </Table.Cell>
  <Table.Cell>
    {#if entryCommit?.createdAt}
      <CommunityRelativeTime timestamp={entryCommit.createdAt} {now} />
    {:else if canLoadPathCommit && !commitLoaded}<Skeleton
        style="height: 1rem; width: 4rem"
      />
    {:else}
      <span aria-hidden="true">—</span>
    {/if}
  </Table.Cell>
{/snippet}

<Table.Body
  bind:ref={host}
  class="community-projects__virtual-directory-body"
  aria-busy={loading}
  style={scrollElement
    ? `height: ${$virtualizer.getTotalSize() + (loading ? 38 : 0)}px`
    : undefined}
>
  {#if rows.length === 0 && !loading}
    <Table.Row>
      <Table.Cell colspan={3}>
        <p class="community-projects__status">
          This directory does not contain any files.
        </p>
      </Table.Cell>
    </Table.Row>
  {:else if scrollElement}
    {#each $virtualizer.getVirtualItems() as virtualRow (virtualRow.key)}
      {@const row = rows[virtualRow.index]}
      {#if row}
        <tr
          data-ui-component="table"
          data-ui-part="table-row"
          data-index={virtualRow.index}
          data-row-index={virtualRow.index}
          class="community-projects__virtual-directory-row"
          style:transform={`translateY(${virtualRow.start - $virtualizer.options.scrollMargin}px)`}
          use:measure
        >
          {@render directoryCells(row, virtualRow.index)}
        </tr>
      {/if}
    {/each}
  {:else if scrollElement === null}
    {#each rows as row, index (rowKey(row, index))}
      <Table.Row data-row-index={index}>
        {@render directoryCells(row, index)}
      </Table.Row>
    {/each}
  {/if}
  {#if loading}
    <tr
      class:community-projects__virtual-directory-row={!!scrollElement}
      style:transform={scrollElement
        ? `translateY(${$virtualizer.getTotalSize()}px)`
        : undefined}
    >
      <Table.Cell colspan={3}>
        <div
          role="status"
          aria-label="Loading more repository files"
          aria-busy="true"
        >
          <Skeleton style="height: 1rem; width: 100%" />
        </div>
      </Table.Cell>
    </tr>
  {/if}
</Table.Body>

<style>
  :global(.community-projects__virtual-directory-body) {
    position: relative;
    display: table-row-group;
  }
  :global(.community-projects__virtual-directory-row) {
    position: absolute;
    inset-block-start: 0;
    inset-inline: 0;
    display: table;
    width: 100%;
    table-layout: fixed;
  }
</style>
