<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import TriangleAlert from "@lucide/svelte/icons/triangle-alert";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import {
    FileChangeStats,
    FileListing,
    FileListingViewModeToggle,
  } from "@lapismd/design-core/diff";
  import FileDiffIcon from "@lucide/svelte/icons/file-diff";
  import UndoIcon from "@lucide/svelte/icons/undo-2";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import ChevronUp from "@lucide/svelte/icons/chevron-up";
  import CommunityRevisionIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import { encodeJjChangeId } from "../../projects/repository-log-graph.js";
  import { useConversationInspection } from "../../conversations/inspection.svelte.js";
  import type { ConversationSession } from "../../conversations/session.svelte.js";
  import type { LocalClient } from "../../api/local-client.js";
  import type { ApplicationFreshness } from "../../controller/application-freshness.js";
  import {
    readAgentWorkspaceChanges,
    type AgentTurnSelection,
    type AgentChangeSummary,
  } from "../../controller/agent-workspace-changes.js";
  import type { AgentChangesCache } from "../../controller/agent-changes-cache.js";
  let {
    selection,
    conversation,
    client,
    freshness,
    repositoryName,
    fetcher,
    cache,
  }: {
    selection: AgentTurnSelection;
    conversation: ConversationSession;
    client: LocalClient;
    freshness?: ApplicationFreshness;
    repositoryName: string;
    fetcher: typeof fetch;
    cache?: AgentChangesCache;
  } = $props();
  let summary = $state<AgentChangeSummary | null>(null);
  let error = $state("");
  let attempt = $state(0);
  let expanded = $state(false);
  const inspection = useConversationInspection();
  const browser = untrack(() =>
    inspection?.workspaceBrowser(client, conversation, freshness),
  );
  let root = $state<HTMLElement>();
  let visible = $state(false);
  let mode =
    $state<import("@lapismd/design-core/diff").FileListingViewMode>("tree");
  const selectedTurn = $derived(
    browser?.savedTurn?.selection.requestId === selection.requestId &&
      browser.savedTurn.selection.repository === selection.repository,
  );
  const listingMode = $derived(selectedTurn ? browser!.mode : mode);
  $effect(() => {
    if (!root) return;
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries.some((e) => e.isIntersecting)) visible = true;
      },
      { rootMargin: "200px" },
    );
    observer.observe(root);
    return () => observer.disconnect();
  });
  function openChanges(path?: string) {
    if (!browser || !inspection || !summary) return;
    browser.mode = listingMode;
    browser.showSavedTurn(selection, summary);
    inspection.showSavedTurn(browser, !!path);
    if (path) {
      browser.presentation = "diff";
      void browser.select(path).then(() => {
        browser.presentation = "diff";
      });
    }
  }
  const selectionKey = $derived(
    JSON.stringify([
      selection.repository,
      selection.profile,
      selection.base_commit,
      selection.operation,
      selection.commit,
    ]),
  );
  const totals = $derived(
    summary &&
      summary.complete &&
      summary.files.every((f) => f.additions !== null && f.deletions !== null)
      ? summary.files.reduce(
          (s, f) => ({
            additions: s.additions + f.additions!,
            deletions: s.deletions + f.deletions!,
          }),
          { additions: 0, deletions: 0 },
        )
      : null,
  );
  $effect(() => {
    if (!visible) return;
    selectionKey;
    const captured = untrack(() => ({ ...selection }));
    const transport = fetcher;
    attempt;
    const abort = new AbortController();
    summary = null;
    error = "";
    expanded = false;

    const read = untrack(() =>
      cache
        ? cache.read(captured, abort.signal)
        : readAgentWorkspaceChanges(captured, transport, abort.signal),
    );
    void read
      .then((value) => {
        if (!abort.signal.aborted) summary = value;
      })
      .catch(() => {
        if (!abort.signal.aborted)
          error = "Saved file changes are unavailable. Your draft is retained.";
      });
    return () => abort.abort();
  });
</script>

<div bind:this={root} data-saved-turn={selection.requestId}>
  {#if error}
    <Alert.Root data-severity="warning">
      <TriangleAlert />
      <Alert.Title>File changes unavailable</Alert.Title>
      <Alert.Description
        >{error}
        <Button size="sm" variant="secondary" onclick={() => attempt++}>
          <RefreshIcon data-icon="inline-start" />Retry file changes
        </Button>
      </Alert.Description>
    </Alert.Root>
  {:else if !summary}
    <p class="agent-changes-feedback" role="status">Loading saved changes…</p>
  {:else if !summary.complete && !summary.files.length}
    <p class="agent-changes-feedback" role="status">
      The saved comparison is incomplete. Changed-file totals are unavailable.
    </p>
  {:else if summary.files.length}
    <section
      class="agent-changes-panel"
      aria-label={`Changes in ${repositoryName} · turn ${selection.requestId.slice(0, 8)}`}
    >
      <header>
        <span class="agent-changes-icon"
          ><FileDiffIcon aria-hidden="true" /></span
        >
        <div class="agent-changes-heading">
          <strong
            >{summary.complete ? "Edited" : "At least"}
            {summary.files.length}
            {summary.files.length === 1 ? "file" : "files"}</strong
          >
          <CommunityRevisionIds
            placement="inline"
            commitId={selection.commit}
            changeId={encodeJjChangeId(summary.changeId ?? "")}
          />
          <span class="agent-changes-repository">{repositoryName}</span>
          {#if totals}<FileChangeStats
              showZero
              additions={totals.additions}
              deletions={totals.deletions}
            />
          {:else}<span class="agent-changes-unavailable"
              >Full line counts unavailable</span
            >{/if}
        </div>
        <span
          title="Session-specific Undo is not available for saved agent changes."
          ><Button
            size="sm"
            variant="ghost"
            disabled
            aria-label="Undo unavailable"
            >Undo <UndoIcon aria-hidden="true" /></Button
          ></span
        >
        <div class="agent-changes-actions">
          <Button
            size="sm"
            variant="outline"
            disabled={!inspection?.workspaceEnabled}
            onclick={() => openChanges()}>View changes</Button
          >
          <FileListingViewModeToggle
            mode={listingMode}
            onModeChange={(next) => {
              mode = next;
              if (selectedTurn && browser) browser.mode = next;
            }}
          />
        </div>
      </header>
      <div class="agent-changes-files">
        <FileListing
          files={(expanded ? summary.files : summary.files.slice(0, 3)).map(
            (file) => ({
              ...file,
              additions: file.additions ?? undefined,
              deletions: file.deletions ?? undefined,
            }),
          )}
          mode={listingMode}
          selectedPath={selectedTurn ? browser!.listing.selected : null}
          onSelectPath={(path) => openChanges(path)}
        />
      </div>
      {#if summary.files.length > 3}<Button
          size="sm"
          variant="ghost"
          class="agent-changes-disclosure"
          aria-expanded={expanded}
          onclick={() => (expanded = !expanded)}
          >{expanded
            ? "Show fewer files"
            : `Show ${summary.files.length - 3} more files`}{#if expanded}<ChevronUp
              aria-hidden="true"
            />{:else}<ChevronDown aria-hidden="true" />{/if}</Button
        >{/if}
      {#if !summary.complete}<p class="agent-changes-feedback">
          Showing {summary.files.length} changed files. Additional files are outside
          this bounded preview.
        </p>{/if}
    </section>
  {/if}
</div>

<style>
  .agent-changes-panel {
    border: 1px solid var(--border);
    border-radius: var(--radius);
    margin: 0.25rem 0 0.5rem;
    min-width: 0;
    background: var(--background);
  }
  .agent-changes-panel header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    border-bottom: 1px solid var(--border);
    flex-wrap: wrap;
  }
  .agent-changes-icon {
    display: grid;
    place-items: center;
    width: 2rem;
    height: 2rem;
    flex-shrink: 0;
    border-radius: var(--radius);
    background: var(--muted);
  }
  .agent-changes-heading {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 5rem;
    font-size: 0.875rem;
  }
  .agent-changes-repository,
  .agent-changes-unavailable {
    font-size: 0.75rem;
    color: var(--muted-foreground);
  }
  .agent-changes-actions {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.5rem;
    margin-inline-start: auto;
  }
  .agent-changes-files {
    padding: 0.375rem;
    max-height: 12rem;
    overflow: auto;
  }
  :global(.agent-changes-disclosure) {
    margin: 0 0.375rem 0.375rem;
  }
  .agent-changes-feedback {
    font-size: 0.8rem;
    padding: 0.25rem 0.75rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>
