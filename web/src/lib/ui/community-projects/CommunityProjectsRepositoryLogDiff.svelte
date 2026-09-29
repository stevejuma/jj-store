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
  import CommunityProjectsRepositorySkeleton from "./CommunityProjectsRepositorySkeleton.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import { anchorPath } from "../../collaboration/presentation.js";
  import StoreReviewConversation from "../store-collaboration/StoreReviewConversation.svelte";
  import "./repository-log-diff.css";

  let {
    path,
    patch,
    files,
    operation,
    loading = false,
    now,
    onSelectOperation,
    controller,
  }: {
    path: string | null;
    patch: string | undefined;
    files: readonly FileListingFile[];
    operation?: RepositoryLogRevision | undefined;
    loading?: boolean | undefined;
    now?: number | undefined;
    onSelectOperation?: ((id: string) => void) | undefined;
    controller?: StoreController | undefined;
  } = $props();

  let viewMode = $state<FileDiffViewMode>("unified");
  let wrap = $state(false);
  let operationExpanded = $state(true);
  const stats = $derived(repositoryLogFileStats(files, path));
  const operationPanelId = $derived(
    operation ? `operation-details-${operation.eventId.slice(0, 16)}` : "",
  );
  const inlineReview = $derived(
    Boolean(
      path &&
      controller?.state.reviewThread &&
      anchorPath(controller.state.reviewThread.anchor.value) === path,
    ),
  );
</script>

{#snippet inlineDiscussion()}
  {#if inlineReview && controller}<StoreReviewConversation
      {controller}
      inline
    />{/if}
{/snippet}

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
        {#if loading}
          <section
            class="community-projects-repository-log-diff__file-card"
            aria-label="Loading selected operation comparison"
          >
            <CommunityProjectsRepositorySkeleton
              layout="diff"
              label="Loading selected operation comparison"
            />
          </section>
        {:else if path && patch}
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
            {@render inlineDiscussion()}
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
            {@render inlineDiscussion()}
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
      {#if loading}<CommunityProjectsRepositorySkeleton
          layout="diff"
          label="Loading selected comparison"
        />{:else if path && patch}
        <CommunityCommitDiff
          single
          files={[{ path, patch }]}
          {viewMode}
          {wrap}
        />
        {@render inlineDiscussion()}
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
        </Empty.Root>
        {@render inlineDiscussion()}
      {:else}<Empty.Root
          class="community-projects-repository-log-diff__empty"
          role="status"
        >
          <Empty.Header>
            <Empty.Media variant="icon">
              <FileSearchIcon aria-hidden="true" />
            </Empty.Media>
            <Empty.Title>No file selected</Empty.Title>
            <Empty.Description
              >Select a changed file to preview the diff.</Empty.Description
            >
          </Empty.Header>
        </Empty.Root>{/if}
    </ColumnCanvas.Body>
  {/if}
</ColumnCanvas.Column>
