<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import type {
    AgentSession,
    AgentEvent,
  } from "../../controller/agent-execution-types.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { ConversationSession } from "../../conversations/session.svelte.js";
  import { completedAgentTurns } from "../../controller/agent-workspace-changes.js";
  import StoreAgentChangesPanel from "./StoreAgentChangesPanel.svelte";
  let {
    controller,
    manager,
    session,
    events,
    conversation,
    requestId,
  }: {
    controller?: StoreController;
    manager: AgentExecution;
    session: AgentSession;
    events: readonly AgentEvent[];
    conversation: ConversationSession;
    requestId: string;
  } = $props();
  const changes = $derived(
    completedAgentTurns(session, events).filter(
      (turn) => turn.requestId === requestId,
    ),
  );
</script>

{#each changes as selection (`${selection.requestId}:${selection.repository}:${selection.operation}:${selection.commit}`)}
  <StoreAgentChangesPanel
    {selection}
    {conversation}
    client={manager.client}
    freshness={manager.freshness}
    repositoryName={controller?.state.repositories.find(
      (r) => r.id === selection.repository,
    )?.name ?? selection.repository.slice(0, 8)}
    fetcher={manager.client.fetcher}
    cache={manager.changes}
  />
{/each}
