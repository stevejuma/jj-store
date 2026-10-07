<script lang="ts">
  import { SystemMessage } from "@lapismd/design-core/ai/chat";
  import { Reasoning } from "@lapismd/design-core/ai/experimental";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { agentMember } from "../community-agents/model.js";
  import StoreAgentInboxMessage from "./StoreAgentInboxMessage.svelte";
  import StoreAgentReplyDraftRecovery from "./StoreAgentReplyDraftRecovery.svelte";
  import CommunityAgentTranscriptPlan from "../community-agent-activity/CommunityAgentTranscriptPlan.svelte";
  import CommunityAgentTranscriptTools from "../community-agent-activity/CommunityAgentTranscriptTools.svelte";
  import StoreAgentPermission from "./StoreAgentPermission.svelte";
  import {
    activityRows,
    openingReplayRowIds,
    queuedRequests,
  } from "./presentation.js";
  import { object, text } from "../../controller/agent-execution-types.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let { manager }: { manager: AgentExecution } = $props();
  const session = $derived(manager.session);
  const rows = $derived(activityRows(manager.events));
  const queued = $derived(queuedRequests(manager.events));
  const idle = $derived(session?.connected === true);
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
  {#each rows as row (row.id)}
    {#if row.kind === "user" || row.kind === "assistant"}<StoreAgentInboxMessage
        {manager}
        openReply={restoredReply === `${session.id}:${row.id}`}
        row={{ ...row, kind: row.kind }}
        author={row.kind === "assistant"
          ? agentMember({
              id: session.id,
              name: session.title,
              avatar: session.avatar,
            })
          : { id: "user", name: "You", handle: "", initials: "YO" }}
      >
        {#if row.kind === "user" && queued.has(row.request) && idle}<Button
            type="button"
            variant="outline"
            size="sm"
            disabled={manager.busy || !!manager.pending}
            onclick={() =>
              manager.command({ action: "cancel", target: row.request })}
            >Cancel queued prompt</Button
          >{/if}
      </StoreAgentInboxMessage>
    {:else if row.kind === "permission" && session.state === "running"}<StoreAgentPermission
        {manager}
        value={row.value}
        title={row.text}
      />
    {:else if row.kind === "tool"}<CommunityAgentTranscriptTools item={row} />
    {:else if row.kind === "plan"}<CommunityAgentTranscriptPlan
        entries={(Array.isArray(row.value.entries)
          ? row.value.entries
          : []
        ).map((v) => ({
          content: text(object(v).content),
          status: text(object(v).status),
        }))}
      />
    {:else if row.kind === "record"}<SystemMessage>
        Turn {row.text.replaceAll("_", " ")}
      </SystemMessage>{/if}
  {/each}
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
