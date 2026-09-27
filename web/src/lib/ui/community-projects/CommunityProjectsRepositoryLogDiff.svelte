<script lang="ts">
  import CommunityCommitDiff from "../community-commit-diff/CommunityCommitDiff.svelte";
  import {
    FileChangeStats,
    type FileDiffComposerFileContext,
    type FileDiffViewMode,
    type FileListingFile,
  } from "@lapismd/design-core/diff";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import FileSearchIcon from "@lucide/svelte/icons/file-search";
  import FileWarningIcon from "@lucide/svelte/icons/file-warning";
  import CommunityDiffControls from "../community-diff-controls/CommunityDiffControls.svelte";

  import type { RepositoryLogRevision } from "../../projects/repository-log.js";
  import { repositoryLogFileStats } from "../../projects/repository-log-files.js";
  import CommunityProjectsRepositoryLogRail from "./CommunityProjectsRepositoryLogRail.svelte";

  let {
    path,
    patch,
    files,
    operation,
    now,
    onSelectOperation,
  }: {
    path: string | null;
    patch: string | undefined;
    files: readonly FileListingFile[];
    operation?: RepositoryLogRevision | undefined;
    now?: number | undefined;
    onSelectOperation?: ((id: string) => void) | undefined;
  } = $props();

  let viewMode = $state<FileDiffViewMode>("unified");
  let wrap = $state(false);
  let operationExpanded = $state(true);
  const stats = $derived(repositoryLogFileStats(files, path));
  const operationPanelId = $derived(
    operation ? `operation-details-${operation.eventId.slice(0, 16)}` : "",
  );
</script>

<ColumnCanvas.Column
  id="diff"
  class={operation
    ? "community-projects-repository-log-diff__column community-projects-repository-log-diff__column--stacked"
    : "community-projects-repository-log-diff__column"}
>
  {#if operation}
    <ColumnCanvas.Body>
      <div class="community-projects-repository-log-diff__stack">
        <section
          class="community-projects-repository-log-diff__operation-card"
          aria-label="Selected operation"
        >
          <header class="community-projects-repository-log-diff__card-header">
            <button
              class="community-projects-repository-log-diff__operation-toggle"
              type="button"
              aria-expanded={operationExpanded}
              aria-controls={operationPanelId}
              onclick={() => (operationExpanded = !operationExpanded)}
            >
              <ChevronDownIcon aria-hidden="true" />
              <span>{operation.message}</span>
            </button>
            <div class="community-projects-repository-log-diff__card-actions">
              <ColumnCanvas.Close
                aria-label="Close Diff column"
                title="Close Diff"
              />
            </div>
          </header>
          {#if operationExpanded}
            <div id={operationPanelId}>
              <CommunityProjectsRepositoryLogRail
                {operation}
                {now}
                onSelect={onSelectOperation ?? (() => undefined)}
              />
            </div>
          {/if}
        </section>
        {#if path && patch}
          <section
            class="community-projects-repository-log-diff__file-card"
            aria-label="Selected file diff"
          >
            <CommunityCommitDiff files={[{ path, patch }]} {viewMode} {wrap}>
              {#snippet fileHeaderTrailing(
                _context: FileDiffComposerFileContext,
              )}
                <FileChangeStats
                  additions={stats.additions}
                  deletions={stats.deletions}
                  showZero
                />
                <CommunityDiffControls bind:viewMode bind:wrap />
              {/snippet}
            </CommunityCommitDiff>
          </section>
        {:else if path}
          <section
            class="community-projects-repository-log-diff__file-card"
            aria-label="Selected file diff"
          >
            <Empty.Root
              class="community-projects-repository-log-diff__empty"
              role="status"
            >
              <Empty.Header>
                <Empty.Media variant="icon">
                  <FileWarningIcon aria-hidden="true" />
                </Empty.Media>
                <Empty.Title>Binary diff unavailable</Empty.Title>
                <Empty.Description
                  >Binary or unavailable text diff for {path}.</Empty.Description
                >
              </Empty.Header>
            </Empty.Root>
          </section>
        {:else}
          <section
            class="community-projects-repository-log-diff__file-card"
            aria-label="Selected file diff"
          >
            <Empty.Root
              class="community-projects-repository-log-diff__empty"
              role="status"
            >
              <Empty.Header>
                <Empty.Media variant="icon">
                  <FileSearchIcon aria-hidden="true" />
                </Empty.Media>
                <Empty.Title>No file selected</Empty.Title>
                <Empty.Description
                  >Select a changed file to preview the operation diff.</Empty.Description
                >
              </Empty.Header>
            </Empty.Root>
          </section>
        {/if}
      </div>
    </ColumnCanvas.Body>
  {:else}
    <ColumnCanvas.Header>
      <div data-ui-component="column-canvas" data-ui-part="column-header-main">
        <ColumnCanvas.Title>{path ?? "Diff"}</ColumnCanvas.Title>
      </div>
      <ColumnCanvas.HeaderActions>
        <FileChangeStats
          additions={stats.additions}
          deletions={stats.deletions}
          showZero
        />
        <CommunityDiffControls bind:viewMode bind:wrap />
        <ColumnCanvas.Close aria-label="Close Diff column" title="Close Diff" />
      </ColumnCanvas.HeaderActions>
    </ColumnCanvas.Header>
    <ColumnCanvas.Body>
      {#if path && patch}
        <CommunityCommitDiff
          single
          files={[{ path, patch }]}
          {viewMode}
          {wrap}
        />
      {:else if path}<Empty.Root
          class="community-projects-repository-log-diff__empty"
          role="status"
        >
          <Empty.Header>
            <Empty.Media variant="icon">
              <FileWarningIcon aria-hidden="true" />
            </Empty.Media>
            <Empty.Title>Binary diff unavailable</Empty.Title>
            <Empty.Description
              >Binary or unavailable text diff for {path}.</Empty.Description
            >
          </Empty.Header>
        </Empty.Root>{/if}
    </ColumnCanvas.Body>
  {/if}
</ColumnCanvas.Column>

<style>
  :global(.community-projects-repository-log-diff__column--stacked) {
    border: 0 !important;
    border-radius: 0 !important;
    background: transparent !important;
  }

  :global(.community-projects-repository-log-diff__column--stacked)
    :global([data-ui-part="column-body"]) {
    overflow: hidden;
  }

  .community-projects-repository-log-diff__stack {
    display: flex;
    box-sizing: border-box;
    min-height: 0;
    width: 100%;
    flex: 1 1 auto;
    flex-direction: column;
    gap: 0.75rem;
    overflow: auto;
  }

  .community-projects-repository-log-diff__operation-card,
  .community-projects-repository-log-diff__file-card {
    box-sizing: border-box;
    overflow: hidden;
    width: 100%;
    flex: 0 0 auto;
    border: 1px solid var(--border);
    border-radius: var(--ui-column-canvas-radius, var(--radius));
    background: var(--background);
  }

  .community-projects-repository-log-diff__file-card {
    min-height: 0;
  }

  .community-projects-repository-log-diff__card-header {
    display: flex;
    min-width: 0;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    min-height: var(--ui-column-canvas-header-height, 2.5rem);
    padding-inline: 0.75rem;
    border-bottom: 1px solid var(--border);
    background: var(--jj-store-row-background);
  }

  .community-projects-repository-log-diff__operation-toggle {
    display: flex;
    min-width: 0;
    flex: 1 1 auto;
    align-items: center;
    gap: 0.5rem;
    border: 0;
    color: inherit;
    background: transparent;
    font: inherit;
    font-size: var(--text-sm, 0.875rem);
    font-weight: var(--font-weight-semibold, 600);
    text-align: left;
    cursor: pointer;
  }

  .community-projects-repository-log-diff__operation-toggle :global(svg) {
    width: 1rem;
    height: 1rem;
    flex: 0 0 auto;
    color: var(--muted-foreground);
    transition: transform 120ms ease;
  }

  .community-projects-repository-log-diff__operation-toggle[aria-expanded="false"]
    :global(svg) {
    transform: rotate(-90deg);
  }

  .community-projects-repository-log-diff__operation-toggle span {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .community-projects-repository-log-diff__card-actions {
    display: inline-flex;
    flex: 0 0 auto;
    align-items: center;
    gap: 0.35rem;
  }

  :global(
    .community-projects-repository-log-diff__card-actions
      [data-ui-part="column-close"]
  ) {
    color: var(--muted-foreground);
    background: color-mix(in srgb, var(--foreground) 4%, transparent);
  }

  :global(
    .community-projects-repository-log-diff__card-actions
      [data-ui-part="column-close"]:hover
  ) {
    background: color-mix(
      in srgb,
      var(--foreground) 18%,
      var(--background)
    ) !important;
    color: var(--ui-column-canvas-header-action-hover-foreground, inherit);
  }

  :global(
    .community-projects-repository-log-diff__card-actions
      [data-ui-part="column-close"]:focus-visible
  ) {
    background: color-mix(
      in srgb,
      var(--foreground) 18%,
      var(--background)
    ) !important;
    color: var(--ui-column-canvas-header-action-hover-foreground, inherit);
  }

  .community-projects-repository-log-diff__file-card
    :global(.ui-diff-file-diff-composer__file) {
    border: 0;
    border-radius: 0;
    background: transparent;
  }

  .community-projects-repository-log-diff__file-card
    :global(.ui-diff-file-diff-composer__header) {
    border-bottom: 1px solid var(--border);
    background: var(--jj-store-row-background);
  }

  .community-projects-repository-log-diff__file-card
    :global(.ui-diff-file-diff) {
    min-height: 0;
    height: auto;
  }

  :global(.community-projects-repository-log-diff__empty) {
    min-height: 12rem;
    padding: 1.5rem;
    color: var(--muted-foreground);
  }

  :global(
    .community-projects-repository-log-diff__empty [data-ui-part="empty-title"]
  ) {
    color: var(--foreground);
    font-size: var(--community-text-body);
  }

  :global(
    .community-projects-repository-log-diff__empty
      [data-ui-part="empty-description"]
  ) {
    max-width: 28rem;
    font-size: var(--community-text-secondary);
  }
</style>
