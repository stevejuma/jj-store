<script lang="ts">
  import { Reasoning } from "@lapismd/design-core/ai/experimental";
  import StoreAgentChatRows from "./StoreAgentChatRows.svelte";
  import StoreAgentReplyDraftRecovery from "./StoreAgentReplyDraftRecovery.svelte";
  import {
    activityRows,
    openingReplayRowIds,
    queuedRequests,
  } from "./presentation.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let { manager }: { manager: AgentExecution } = $props();
  const session = $derived(manager.session);
  const rows = $derived(activityRows(manager.events));
  const queued = $derived(queuedRequests(manager.events));
  const retainedReplayDrafts = $derived(
    openingReplayRowIds(manager.events).filter(
      (row) => !!manager.discussions.drafts[`${session?.id}:${row}`],
    ),
  );
  let restoredReply = $state("");
  $effect(() => {
    const id = session?.id,
      repository = session?.repository;
    if (id && repository) return manager.discussions.watch(id, repository);
  });
</script>

{#if session}
  <StoreAgentChatRows
    {manager}
    rows={rows.map((row) => ({
      ...row,
      session,
      key: `${session.id}:${row.id}`,
    }))}
    {restoredReply}
    {queued}
  />
  {#each retainedReplayDrafts as source (`${session.id}:${source}`)}
    <StoreAgentReplyDraftRecovery
      {manager}
      {source}
      {rows}
      onRestore={(row) => (restoredReply = `${session.id}:${row}`)}
    />
  {/each}
  {#if session.connected && session.state === "running"}<Reasoning
      label="Working"
      streaming
    />{/if}
{/if}
