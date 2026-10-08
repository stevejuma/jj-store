<script lang="ts">
  import ClockIcon from "@lucide/svelte/icons/clock";
  import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
  import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
  import BanIcon from "@lucide/svelte/icons/ban";
  import { SystemMessage } from "@lapismd/design-core/ai/chat";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreAgentPromptContent from "./StoreAgentPromptContent.svelte";
  import StoreAgentInboxMessage from "./StoreAgentInboxMessage.svelte";
  import StoreAgentPermission from "./StoreAgentPermission.svelte";
  import CommunityAgentTranscriptTools from "../community-agent-activity/CommunityAgentTranscriptTools.svelte";
  import CommunityAgentTranscriptPlan from "../community-agent-activity/CommunityAgentTranscriptPlan.svelte";
  import { agentMember } from "../community-agents/model.js";
  import { agentMessageTimestamp } from "../../collaboration/presentation.js";
  import { activityDayDividers, type ActivityRow } from "./presentation.js";
  import { groupActivityRows } from "./activity-row-groups.js";
  import {
    object,
    text,
    type AgentSession,
  } from "../../controller/agent-execution-types.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  let {
    manager,
    rows,
    user = { id: "user", name: "You", handle: "", initials: "YO" },
    restoredReply = "",
    queued = new Set<string>(),
    showDayDividers = true,
  }: {
    manager: AgentExecution;
    rows: readonly (ActivityRow & { session: AgentSession; key: string })[];
    user?: CommunityMemberModel;
    restoredReply?: string;
    queued?: ReadonlySet<string>;
    showDayDividers?: boolean;
  } = $props();
  const days = $derived(
    activityDayDividers(rows.map((row) => ({ ...row, id: row.key }))),
  );
  const groups = $derived(groupActivityRows(rows, days));
</script>

{#each groups as entry (entry.key)}
  {@const row = entry.row}
  {#if showDayDividers && days.has(row.key)}<SystemMessage variant="divider"
      >{days.get(row.key)}</SystemMessage
    >{/if}
  {@const timing = agentMessageTimestamp(row.observedAtMillis)}
  {@const author =
    row.kind === "user"
      ? user
      : {
          ...agentMember({
            id: row.session.setup_id ?? row.session.id,
            name: row.session.title,
            avatar: row.session.avatar,
          }),
          acpAgentId: row.session.setup_id ?? undefined,
        }}
  {#if entry.kind === "tools"}
    <CommunityAgentTranscriptTools
      items={entry.items}
      openCallId={entry.items.find((item) => item.key === restoredReply)?.id}
    >
      {#snippet callWrapper(item, children)}
        <StoreAgentInboxMessage
          {manager}
          sessionId={row.session.id}
          row={{ ...item, kind: "tool" }}
          {author}
          openReply={restoredReply === `${row.session.id}:${item.id}`}
        >
          {#snippet content()}{@render children()}{/snippet}
        </StoreAgentInboxMessage>
      {/snippet}
    </CommunityAgentTranscriptTools>
  {:else if row.kind === "user" || row.kind === "assistant"}
    <StoreAgentInboxMessage
      {manager}
      sessionId={row.session.id}
      row={{ ...row, kind: row.kind }}
      {author}
      openReply={restoredReply === row.key}
    >
      {#snippet content()}
        <StoreAgentPromptContent
          content={row.text}
          context={row.kind === "user" && typeof row.value.context === "string"
            ? row.value.context
            : ""}
          sentContext={row.kind === "user" ? row.value.sentContext : undefined}
          members={[author]}
        />
      {/snippet}
      {#if row.kind === "user" && queued.has(row.request) && row.session.connected}<Button
          type="button"
          variant="outline"
          size="sm"
          disabled={manager.busy || !!manager.pending}
          onclick={() =>
            manager.commandSession(row.session.id, {
              action: "cancel",
              target: row.request,
            })}>Cancel queued prompt</Button
        >{/if}
    </StoreAgentInboxMessage>
  {:else if row.kind === "permission" && row.session.state === "running"}<StoreAgentPermission
      {manager}
      sessionId={row.session.id}
      value={row.value}
      title={row.text}
    />
  {:else if row.kind === "plan"}<CommunityAgentTranscriptPlan
      entries={(Array.isArray(row.value.entries) ? row.value.entries : []).map(
        (v) => ({
          content: text(object(v).content),
          status: text(object(v).status),
        }),
      )}
    />
  {:else if row.kind === "record" || row.kind === "session"}
    <SystemMessage variant="divider">
      {@const label =
        row.kind === "session"
          ? "Session started"
          : `Turn ${row.text.replaceAll("_", " ")}`}
      {@const Icon =
        row.kind === "session"
          ? ClockIcon
          : row.text === "completed"
            ? CircleCheckIcon
            : row.text === "cancelled"
              ? BanIcon
              : CircleAlertIcon}
      <span
        class="agent-chat-lifecycle"
        role="group"
        aria-label={label}
        title={label}
      >
        <Icon size={14} aria-hidden="true" />
        {#if timing.createdAt}<time
            datetime={timing.createdAt}
            title={new Date(timing.createdAt).toLocaleString()}
          >
            · {timing.timestamp}</time
          >{/if}</span
      >
    </SystemMessage>
  {/if}
{/each}

<style>
  .agent-chat-lifecycle {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    white-space: nowrap;
  }
</style>
