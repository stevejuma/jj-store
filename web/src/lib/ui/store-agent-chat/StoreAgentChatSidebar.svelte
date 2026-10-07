<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MessageIcon from "@lucide/svelte/icons/messages-square";
  import StoreAgentChatRepositories from "./StoreAgentChatRepositories.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import CommunityAgentActivityFeed from "../community-agent-activity/CommunityAgentActivityFeed.svelte";
  import StoreAgentChatRows from "../store-agent-execution/StoreAgentChatRows.svelte";
  import { activityRows } from "../store-agent-execution/presentation.js";
  import { agentMember } from "../community-agents/model.js";
  import StoreAgentBindings from "./StoreAgentBindings.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import { mentionedAgentIds } from "../../community/agent-mentions.js";
  import {
    agentLiveState,
    agentLiveLabels,
  } from "../../controller/agent-live-status.js";
  import { object, text } from "../../controller/agent-execution-types.js";
  let {
    controller,
    onClose,
  }: { controller: StoreController; onClose: () => void } = $props();
  const manager = $derived(controller.execution!);
  const chat = $derived(manager.chat);
  const members = $derived((manager.value?.setups ?? []).map(agentMember));
  const headingIds = $derived(mentionedAgentIds(chat.draft?.text ?? ""));
  const sessionMembers = $derived([
    ...new Map(
      chat.sessions.map((s) => [
        s.setup_id ?? s.id,
        {
          ...agentMember({
            id: s.setup_id ?? s.id,
            name: s.title,
            avatar: s.avatar,
          }),
          acpAgentId: s.setup_id ?? undefined,
        },
      ]),
    ).values(),
  ]);
  const headingAgents = $derived(
    headingIds.length
      ? headingIds.flatMap((id) => {
          const member =
            members.find((m) => m.id === id) ??
            sessionMembers.find((m) => m.id === id);
          return member ? [member] : [];
        })
      : sessionMembers,
  );
  const rows = $derived(
    chat.sessions
      .flatMap((session) =>
        activityRows(chat.events[session.id] ?? []).map((row) => ({
          ...row,
          session,
          key: `${session.id}:${row.id}`,
        })),
      )
      .sort(
        (a, b) =>
          (a.observedAtMillis ?? 0) - (b.observedAtMillis ?? 0) ||
          a.sequence - b.sequence,
      ),
  );
  const working = $derived(chat.sessions.some((s) => s.state === "running"));
  const actorKey = $derived(controller.state.member.id);
  function stopTurn(id: string) {
    const events = chat.events[id] ?? [];
    const started = [...events]
      .reverse()
      .find((e) => object(e.payload).type === "started");
    if (started)
      void manager.commandSession(id, {
        action: "cancel",
        target: text(object(started.payload).requestId),
      });
  }
</script>

<AppShell.Sidebar.Header class="agent-chat-heading">
  {#each headingAgents as agent (agent.id)}<CommunityMemberAvatar
      member={agent}
      size="small"
    />{/each}
  <strong>{headingAgents.length === 1 ? headingAgents[0]!.name : "Chat"}</strong
  >
  <AppShell.Sidebar.Close label="Close" onclick={onClose} />
</AppShell.Sidebar.Header>
<AppShell.Sidebar.Body scrollable={false}>
  {#if chat.draft}
    <div class="agent-chat-body" data-ui-component="agent-chat">
      <CommunityAgentActivityFeed
        conversationKey={chat.selected}
        latestMessageId={rows.at(-1)?.key}
        {working}
        empty={!rows.length}
      >
        {#if !rows.length}
          <Empty.Root class="agent-draft-empty">
            <Empty.Header>
              <Empty.Media variant="icon"
                ><MessageIcon aria-hidden="true" /></Empty.Media
              >
              <Empty.Title>Start a chat with an agent</Empty.Title>
              <Empty.Description
                >Use @ to choose an agent and send a message. Choose a
                repository when you want to work on code.</Empty.Description
              >
            </Empty.Header>
            <Empty.Content
              ><StoreAgentChatRepositories {controller} /></Empty.Content
            >
          </Empty.Root>
        {:else}
          <StoreAgentChatRepositories {controller} />
        {/if}
        <StoreAgentChatRows {manager} {rows} user={controller.state.member} />
        {#snippet messageComposer()}
          {#each chat.sessions as session (session.id)}<div
              class="agent-chat-participant"
            >
              <span
                >{session.title} · {agentLiveLabels[
                  agentLiveState(
                    manager.value,
                    session.setup_id ?? "",
                    !!manager.connectionError,
                  )
                ]}</span
              >{#if session.state === "running"}<Button
                  size="sm"
                  variant="ghost"
                  disabled={manager.busy || !!manager.pending}
                  onclick={() => stopTurn(session.id)}>Stop turn</Button
                >{/if}{#if session.connected}<Button
                  size="sm"
                  variant="ghost"
                  disabled={manager.busy || !!manager.pending}
                  onclick={() =>
                    void manager.commandSession(session.id, { action: "end" })}
                  >End participation</Button
                >{/if}
            </div>
            <StoreAgentBindings
              {controller}
              {session}
              events={chat.events[session.id] ?? []}
            />{/each}
          {#key chat.selected}<CommunityChatComposer
              bind:value={chat.draft.text}
              {members}
              audienceKey={JSON.stringify([actorKey, chat.audienceKey])}
              initialAgentIds={chat.sessions.flatMap((s) =>
                s.setup_id ? [s.setup_id] : [],
              )}
              ariaLabel="Message to agents"
              placeholder="Message an @agent…"
              disabled={chat.busy || !!chat.pending || !manager.available()}
              composerStatus={chat.error
                ? { type: "error", message: chat.error }
                : chat.streamError
                  ? { type: "warning", message: chat.streamError }
                  : undefined}
              onSubmit={(submission) =>
                chat.send(submission.content, submission.agentIds ?? [])}
            />{/key}
          {#if chat.pending}<div class="agent-chat-recovery">
              <Button
                variant="outline"
                size="sm"
                disabled={chat.busy}
                onclick={() => void chat.retry().catch(() => {})}
                >Check retained request</Button
              ><Button
                variant="ghost"
                size="sm"
                disabled={chat.busy}
                onclick={() => chat.retainUnknown()}
                >Keep unresolved in history</Button
              >
            </div>{/if}
        {/snippet}
      </CommunityAgentActivityFeed>
    </div>
  {/if}
</AppShell.Sidebar.Body>

<style>
  :global(.agent-chat-heading[data-ui-part="sidebar-header"]) {
    border-bottom: 0;
  }
  :global(.agent-chat-heading[data-ui-part="sidebar-header"]),
  .agent-chat-participant,
  .agent-chat-recovery {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  :global(.agent-chat-heading strong),
  .agent-chat-participant span {
    flex: 1;
  }
  .agent-chat-body {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }
  .agent-chat-participant {
    padding: 0.25rem 0.75rem;
    font-size: 0.8rem;
  }
</style>
