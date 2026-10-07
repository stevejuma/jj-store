<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityAgentComposerActivity from "../community-agent-composer-activity/CommunityAgentComposerActivity.svelte";
  import { agentMember } from "../community-agents/model.js";
  import type { AgentSession } from "../../controller/agent-execution-types.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let {
    manager,
    session,
  }: { manager: AgentExecution; session?: AgentSession } = $props();
  const pending = $derived(manager.pending);
  const pendingSession = $derived(
    manager.value?.sessions.find((s) => s.id === pending?.body.session_id),
  );
  const member = $derived(
    pendingSession ??
      session ??
      manager.value?.setups.find((s) => s.id === pending?.body.setup_id),
  );
  const state = $derived(
    pending ? (manager.outcome?.state ?? "checking") : "running",
  );
  const working = $derived(session?.connected && session.state === "running");
  const label = $derived(
    state === "outcome_unknown"
      ? "Outcome unknown"
      : state === "not_known"
        ? "Request not found"
        : state === "dispatching"
          ? "Starting"
          : state === "running"
            ? "Working"
            : "Checking request outcome",
  );
</script>

{#if pending || working}<CommunityAgentComposerActivity
    member={member && {
      ...agentMember({
        id: "setup_id" in member ? (member.setup_id ?? member.id) : member.id,
        name: "title" in member ? member.title : member.name,
        avatar: member.avatar,
      }),
      acpAgentId:
        "setup_id" in member ? (member.setup_id ?? undefined) : member.id,
    }}
    {label}
    {state}
    streaming={state === "running" ||
      state === "dispatching" ||
      state === "checking"}
  />{/if}
{#if manager.pending && manager.outcome?.state === "not_known"}<Button
    variant="outline"
    disabled={manager.busy}
    onclick={() => manager.retryRetainedRequest()}
    >Submit retained request</Button
  >{/if}
{#if manager.pending && manager.outcome?.state === "outcome_unknown"}<Button
    variant="outline"
    onclick={() => manager.retainAsUnresolved()}
    >Keep unresolved request and review workspace</Button
  >{/if}
