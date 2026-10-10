<script lang="ts">
  import CommunityAgentComposerActivity from "../community-agent-composer-activity/CommunityAgentComposerActivity.svelte";
  import { agentMember } from "../community-agents/model.js";
  import type { AgentSession } from "../../controller/agent-execution-types.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import { sessionActivity } from "../../controller/agent-session-activity.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    manager,
    session,
  }: { manager: AgentExecution; session?: AgentSession } = $props();
  const pending = $derived(
    session && manager.pending?.body.session_id !== session.id
      ? null
      : manager.pending,
  );
  const pendingSession = $derived(
    manager.value?.sessions.find((s) => s.id === pending?.body.session_id),
  );
  const member = $derived(
    pendingSession ??
      session ??
      manager.value?.setups.find((s) => s.id === pending?.body.setup_id),
  );
  const activity = $derived(
    manager.connectionError
      ? { state: "unknown", label: "Unknown", streaming: false }
      : sessionActivity(session, pending, manager.outcome),
  );
  const stop = $derived(session && manager.stops.requests[session.id]);
  const stopError = $derived(
    stop?.error || (session && manager.stops.errors[session.id]),
  );
  const stopping = $derived(
    stop &&
      (stop.busy ||
        ["dispatching", "queued", "started"].includes(
          stop.outcome?.state ?? "",
        )),
  );
</script>

{#if activity}<CommunityAgentComposerActivity
    member={member && {
      ...agentMember({
        id: "setup_id" in member ? (member.setup_id ?? member.id) : member.id,
        name: "title" in member ? member.title : member.name,
        avatar: member.avatar,
      }),
      acpAgentId:
        "setup_id" in member ? (member.setup_id ?? undefined) : member.id,
    }}
    label={stopping ? "Stopping" : activity.label}
    state={stopping ? "stopping" : activity.state}
    streaming={stopping || activity.streaming}
  />{/if}
{#if stopError}<div role="status">
    {stopError}{#if stop}<Button
        variant="ghost"
        size="sm"
        disabled={stop.busy}
        onclick={() => session && void manager.stops.check(session.id)}
        >Check Stop outcome</Button
      >{/if}
  </div>{/if}
