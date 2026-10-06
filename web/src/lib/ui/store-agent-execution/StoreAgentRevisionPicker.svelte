<script lang="ts">
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import { agentRevisions } from "../../controller/agent-revisions.js";
  import {
    needsAgentRevisionIndex,
    rebuildAgentRevisionIndex,
  } from "../../controller/agent-revision-index.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import IndexIcon from "@lucide/svelte/icons/network";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentTarget } from "../../controller/agent-execution-types.js";
  let { manager, target }: { manager: AgentExecution; target?: AgentTarget } =
    $props();
  let query = $state(""),
    loading = $state(false),
    error = $state(""),
    next = $state<string | null>(null);
  let rows = $state<{ id: string; name: string; description: string }[]>([]);
  let active: AbortController | undefined;
  let preparation: AbortController | undefined;
  let rebuilding = $state(false),
    needsIndex = $state(false);
  const scope = $derived(
    target ? `${target.id}:${target.repository}:${target.operation}` : "",
  );
  $effect(() => {
    scope;
    preparation?.abort();
    rebuilding = false;
    needsIndex = false;
    return () => preparation?.abort();
  });
  $effect(() => {
    scope;
    const search = query;
    const abort = new AbortController();
    active?.abort();
    active = abort;
    rows = [];
    next = null;
    error = "";
    const timer = setTimeout(() => void load(false, search, abort), 150);
    return () => {
      clearTimeout(timer);
      abort.abort();
    };
  });
  async function load(more: boolean, search = query, abort = active) {
    if (!target || !abort || abort.signal.aborted) return;
    loading = true;
    try {
      const result = await agentRevisions(
        target,
        manager.client.fetcher,
        search,
        abort.signal,
        more ? (next ?? undefined) : undefined,
      );
      if (abort.signal.aborted) return;
      rows = [
        ...new Map(
          [...(more ? rows : []), ...result.items].map((r) => [r.id, r]),
        ).values(),
      ];
      next = result.next;
      error = "";
      needsIndex = false;
    } catch (cause) {
      if (!abort.signal.aborted) {
        needsIndex = needsAgentRevisionIndex(cause, target);
        error = needsIndex
          ? "The repository revision index needs to be rebuilt. Your selected revision is retained."
          : `Revisions unavailable: ${cause instanceof Error ? cause.message : "request failed"}. The selected revision is retained.`;
      }
    } finally {
      if (!abort.signal.aborted) loading = false;
    }
  }
  async function rebuild() {
    if (!target || rebuilding) return;
    const selected = target,
      abort = new AbortController();
    preparation = abort;
    rebuilding = true;
    try {
      await rebuildAgentRevisionIndex(
        selected,
        manager.client.fetcher,
        abort.signal,
      );
      if (abort.signal.aborted) return;
      await load(false);
    } catch (cause) {
      if (!abort.signal.aborted)
        error = `Index rebuild could not be confirmed: ${cause instanceof Error ? cause.message : "request failed"}. Your selected revision is retained.`;
    } finally {
      if (!abort.signal.aborted) rebuilding = false;
    }
  }
  const choices = $derived(
    [
      {
        id: manager.launchDraft.commit,
        name:
          manager.launchDraft.commit === target?.commit
            ? "Current workspace"
            : "Selected revision",
        description: manager.launchDraft.commit,
      },
      ...rows.filter((r) => r.id !== manager.launchDraft.commit),
    ].filter((r) => r.id),
  );
</script>

<StoreRepositoryPicker
  label="Starting revision"
  variant="ghost"
  controlled
  value={manager.launchDraft.commit}
  repositories={choices}
  searchLabel="Search revisions"
  groupLabel="Repository revisions"
  emptyResult="No matching revisions."
  notice={rebuilding
    ? "Rebuilding the repository revision index…"
    : error || (loading ? "Loading revisions…" : "")}
  disabled={!target || manager.busy || !!manager.pending}
  hasMore={!!next}
  {loading}
  onLoadMore={() => void load(true)}
  onSearch={(value) => (query = value)}
  onChange={(value) => (manager.launchDraft.commit = value)}
  >{#snippet noticeAction()}{#if needsIndex || rebuilding}<Button
        variant="outline"
        size="sm"
        disabled={rebuilding || !target || manager.busy || !!manager.pending}
        onclick={() => void rebuild()}
        ><IndexIcon data-icon="inline-start" aria-hidden="true" />{rebuilding
          ? "Rebuilding index…"
          : "Rebuild index"}</Button
      >{/if}{/snippet}
  {#snippet triggerIcon()}<GitBranchIcon
      aria-hidden="true"
    />{/snippet}</StoreRepositoryPicker
>
