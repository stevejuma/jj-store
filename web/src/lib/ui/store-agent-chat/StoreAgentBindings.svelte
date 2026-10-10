<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  import type {
    AgentSession,
    AgentEvent,
  } from "../../controller/agent-execution-types.js";
  import { object } from "../../controller/agent-execution-types.js";
  import { completedAgentChanges } from "../../controller/agent-workspace-changes.js";
  import StoreAgentChangesPanel from "./StoreAgentChangesPanel.svelte";
  let {
    controller,
    session,
    events,
  }: {
    controller: StoreController;
    session: AgentSession;
    events: readonly AgentEvent[];
  } = $props();
  const changes = $derived(completedAgentChanges(session, events));
</script>

{#each session.repository_bindings as binding (binding.repository)}
  {@const snapshot = [...events]
    .reverse()
    .find(
      (e) =>
        object(e.payload).type === "repository_snapshot" &&
        object(e.payload).repository === binding.repository,
    )}
  {@const unknown =
    snapshot && object(snapshot.payload).state === "outcome_unknown"}
  {@const selection = changes.find((c) => c.repository === binding.repository)}
  {#if selection}
    {#key `${selection.operation}:${selection.commit}`}
      <StoreAgentChangesPanel
        {selection}
        repositoryName={controller.state.repositories.find(
          (r) => r.id === binding.repository,
        )?.name ?? binding.repository.slice(0, 8)}
        fetcher={controller.execution!.client.fetcher}
        cache={controller.execution!.changes}
      />
    {/key}
  {:else}
    <div class="agent-binding">
      <span
        >{controller.state.repositories.find((r) => r.id === binding.repository)
          ?.name ?? binding.repository.slice(0, 8)} · {unknown
          ? "Snapshot unknown"
          : snapshot
            ? "Snapshot saved"
            : "Workspace ready"}</span
      >
      <Button
        size="sm"
        variant="ghost"
        disabled={!!unknown}
        onclick={() =>
          void controller.actions.openRoute?.({
            ...controller.state.route,
            agentsHub: false,
            projectsHub: false,
            page: "log",
            repository: binding.repository,
            commitId: binding.commit,
            snapshotOperationId: binding.operation,
            agentSessionId: null,
            executionSessionId: null,
          })}>Review changes</Button
      >
    </div>
  {/if}
{/each}

<style>
  .agent-binding {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.25rem 0.75rem;
    font-size: 0.8rem;
  }
</style>
