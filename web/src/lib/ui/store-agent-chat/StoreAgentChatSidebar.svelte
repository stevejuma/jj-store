<script lang="ts">
  import { submitManagedConversation } from "../../conversations/managed-send.js";

  import StoreConversationRecovery from "../store-agent-chat/StoreConversationRecovery.svelte";
  import { untrack } from "svelte";
  import StoreConversationRuntime from "./StoreConversationRuntime.svelte";
  import StoreLiveConversation from "./StoreLiveConversation.svelte";
  import ChatMessageFooter from "../store-agent-chat-body/ChatMessageFooter.svelte";
  import type { ChatPromptSubmission } from "../store-agent-chat-body/model.js";
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MessageIcon from "@lucide/svelte/icons/messages-square";
  import StoreAgentChatRepositories from "./StoreAgentChatRepositories.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";

  import { activityRows } from "../store-agent-execution/presentation.js";
  import { agentMember } from "../community-agents/model.js";
  import StoreAgentBindings from "./StoreAgentBindings.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import {
    mentionedAgentIds,
    agentMentionToken,
  } from "../../community/agent-mentions.js";
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
  const conversation = $derived(
    manager.conversations.get(`chat:${chat.selected}`),
  );
  $effect(() => {
    const current = conversation;
    untrack(() => {
      if (!current.initialized) {
        current.comments.message = chat.draft?.text ?? "";
        current.initialized = true;
      }
    });
  });
  $effect(() => {
    const message = conversation.comments.message;
    if (chat.draft) chat.draft.text = message;
  });

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
  const selectedAgent = $derived(headingAgents[0]?.id);
  const selectedSession = $derived(
    chat.sessions.find((s) => s.setup_id === selectedAgent),
  );
  const savedConversation = $derived(
    manager.value?.conversations.find((c) => c.id === chat.selected),
  );
  const author = $derived({
    ...controller.state.member,
    id: savedConversation
      ? manager.discussions.actor(savedConversation.home)
      : controller.state.member.id,
  });
  const actorKey = $derived(author.id);
  const send = (submission: ChatPromptSubmission) =>
    submitManagedConversation(manager, conversation, submission);

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
      <StoreLiveConversation
        {controller}
        {manager}
        {rows}
        {conversation}
        user={author}
        onSend={send}
      >
        {#snippet beforeItems()}
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
        {/snippet}
        {#snippet composer()}
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
              bind:value={conversation.comments.message}
              {members}
              audienceKey={JSON.stringify([actorKey, chat.audienceKey])}
              initialAgentIds={chat.sessions.flatMap((s) =>
                s.setup_id ? [s.setup_id] : [],
              )}
              ariaLabel="Message to agents"
              placeholder=""
              enterInsertsNewline
              clearOnSubmit={false}
              showSubmitButton={false}
              allowEmpty={conversation.comments.pending.length > 0}
              disabled={chat.busy || !!chat.pending || !manager.available()}
              composerStatus={chat.error
                ? { type: "error", message: chat.error }
                : chat.streamError
                  ? { type: "warning", message: chat.streamError }
                  : undefined}
              onSubmit={async () => {
                if (!(await conversation.comments.send()))
                  throw new Error(
                    conversation.comments.error ||
                      "The submission is awaiting confirmation.",
                  );
              }}
            />{/key}
          <StoreConversationRecovery {conversation} {manager} />
        {/snippet}
        {#snippet footer()}
          <ChatMessageFooter
            canSend={conversation.comments.canSend &&
              !chat.busy &&
              !chat.pending &&
              manager.available()}
            onSend={() => void conversation.comments.send()}
            error={conversation.comments.error}
          >
            {#snippet controls()}<StoreConversationRuntime
                {manager}
                session={selectedSession}
                agentId={selectedAgent}
                disabled={chat.busy || !!chat.pending}
                onAgentChange={(id) => {
                  const member = members.find((m) => m.id === id);
                  if (member)
                    conversation.comments.message += `${conversation.comments.message.endsWith(" ") || !conversation.comments.message ? "" : " "}${agentMentionToken(id)} `;
                }}
              />{/snippet}
          </ChatMessageFooter>
        {/snippet}
      </StoreLiveConversation>
    </div>
  {/if}
</AppShell.Sidebar.Body>

<style>
  :global(.agent-chat-heading[data-ui-part="sidebar-header"]) {
    border-bottom: 0;
  }
  :global(.agent-chat-heading[data-ui-part="sidebar-header"]),
  .agent-chat-participant {
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
