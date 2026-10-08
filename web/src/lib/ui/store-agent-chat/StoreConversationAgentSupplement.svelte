<script lang="ts">
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type {
    AgentConversationRow,
    ConversationProjection,
  } from "../../conversations/agent-adapter.js";
  import type { CommunityMemberModel } from "../../community/workspace-model.js";
  import type { ChatBodyItem } from "../store-agent-chat-body/model.js";
  import StoreAgentChatRows from "../store-agent-execution/StoreAgentChatRows.svelte";
  import StoreAgentPromptContent from "../store-agent-execution/StoreAgentPromptContent.svelte";
  import { queuedRequests } from "../store-agent-execution/presentation.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    manager,
    rows,
    projection,
    item,
    user,
  }: {
    manager: AgentExecution;
    rows: readonly AgentConversationRow[];
    projection: ConversationProjection;
    item: ChatBodyItem;
    user: CommunityMemberModel;
  } = $props();
  const target = $derived(projection.targets.get(item.id));
  const row = $derived(
    rows.find(
      (r) =>
        r.key === item.id ||
        (r.session.id === target?.sessionId &&
          (r.sourceRowId ?? r.id) === target.rowId),
    ),
  );
  const queued = $derived(
    row &&
      queuedRequests(
        manager.chat.events[row.session.id] ??
          (manager.selected === row.session.id ? manager.events : []),
      ).has(row.request),
  );
</script>

{#if row && item.external}
  <StoreAgentChatRows {manager} rows={[row]} {user} showDayDividers={false} />
{:else if row?.kind === "user"}
  {#if row.value.sentContext}<StoreAgentPromptContent
      content=""
      sentContext={row.value.sentContext}
    />{/if}
  {#if row.session.connected && queued}<Button
      variant="ghost"
      size="sm"
      disabled={manager.busy || !!manager.pending}
      onclick={() =>
        manager.commandSession(row!.session.id, {
          action: "cancel",
          target: row!.request,
        })}>Cancel queued prompt</Button
    >{/if}
{/if}
