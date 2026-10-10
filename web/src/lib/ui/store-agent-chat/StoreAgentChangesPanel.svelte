<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { FileChangeStats } from "@lapismd/design-core/diff";
  import FileDiffIcon from "@lucide/svelte/icons/file-diff";
  import UndoIcon from "@lucide/svelte/icons/undo-2";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import ChevronUp from "@lucide/svelte/icons/chevron-up";
  import StoreAgentChangesDiff from "./StoreAgentChangesDiff.svelte";
  import {
    readAgentWorkspaceChanges,
    type AgentChangeSelection,
    type AgentChangeSummary,
  } from "../../controller/agent-workspace-changes.js";
  import type { AgentChangesCache } from "../../controller/agent-changes-cache.js";
  let {
    selection,
    repositoryName,
    fetcher,
    cache,
  }: {
    selection: AgentChangeSelection;
    repositoryName: string;
    fetcher: typeof fetch;
    cache?: AgentChangesCache;
  } = $props();
  let summary = $state<AgentChangeSummary | null>(null);
  let error = $state("");
  let attempt = $state(0);
  let expanded = $state(false);
  let reviewing = $state(false);
  let selectedPath = $state("");
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
    selectionKey;
    const captured = untrack(() => ({ ...selection }));
    const transport = fetcher;
    attempt;
    const abort = new AbortController();
    summary = null;
    error = "";
    expanded = false;
    reviewing = false;
    selectedPath = "";
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

{#if error}
  <div class="agent-changes-feedback" role="status">
    <span>{error}</span><Button
      size="sm"
      variant="ghost"
      onclick={() => attempt++}>Retry file changes</Button
    >
  </div>
{:else if !summary}
  <p class="agent-changes-feedback" role="status">Loading saved changes…</p>
{:else if !summary.complete && !summary.files.length}
  <p class="agent-changes-feedback" role="status">
    The saved comparison is incomplete. Changed-file totals are unavailable.
  </p>
{:else if summary.files.length}
  <section
    class="agent-changes-panel"
    aria-label={`Changes in ${repositoryName}`}
  >
    <header>
      <span class="agent-changes-icon"><FileDiffIcon aria-hidden="true" /></span
      >
      <div class="agent-changes-heading">
        <strong
          >{summary.complete ? "Edited" : "At least"}
          {summary.files.length}
          {summary.files.length === 1 ? "file" : "files"}</strong
        >
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
      <Button size="sm" variant="outline" onclick={() => (reviewing = true)}
        >View changes</Button
      >
    </header>
    <ul>
      {#each expanded ? summary.files : summary.files.slice(0, 3) as file (file.path)}
        <li>
          <button
            type="button"
            class="agent-changes-file"
            onclick={() => {
              selectedPath = file.path;
              reviewing = true;
            }}
            title={file.path}
            aria-label={`View changes to ${file.path}`}
          >
            <span>{file.path}</span>
            {#if file.additions !== null && file.deletions !== null}<FileChangeStats
                showZero
                additions={file.additions}
                deletions={file.deletions}
              />
            {:else}<span class="agent-changes-unavailable"
                >Counts unavailable</span
              >{/if}
          </button>
        </li>
      {/each}
    </ul>
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
  <StoreAgentChangesDiff
    files={summary.files}
    {repositoryName}
    bind:open={reviewing}
    bind:selectedPath
  />
{/if}

<style>
  .agent-changes-panel {
    border: 1px solid var(--border);
    border-radius: var(--radius);
    margin: 0.25rem 0.75rem 0.5rem;
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
  .agent-changes-icon :global(svg) {
    width: 1.125rem;
    height: 1.125rem;
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
  .agent-changes-panel ul {
    list-style: none;
    margin: 0;
    padding: 0.375rem;
    max-height: 12rem;
    overflow: auto;
  }
  .agent-changes-file {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.375rem;
    text-align: start;
    width: 100%;
    border: 0;
    border-radius: var(--radius);
    background: transparent;
    color: var(--foreground);
    font: inherit;
    font-size: 0.8125rem;
    cursor: pointer;
  }
  .agent-changes-file:hover {
    background: var(--muted);
  }
  .agent-changes-file:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: 1px;
  }
  .agent-changes-file > span:first-child {
    flex: 1;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    min-width: 0;
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
