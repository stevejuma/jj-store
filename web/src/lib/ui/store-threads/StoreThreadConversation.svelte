<script lang="ts">
  import StoreThreadAgentRepositories from "./StoreThreadAgentRepositories.svelte";
  import StoreThreadCodebase from "./StoreThreadCodebase.svelte";
  import StoreWorkspaceButton from "../store-agent-workspace/StoreWorkspaceButton.svelte";
  import StoreConversationRuntime from "../store-agent-chat/StoreConversationRuntime.svelte";
  import {
    agentMentionToken,
    mentionedAgentIds,
  } from "../../community/agent-mentions.js";
  import type { Snippet } from "svelte";
  let inputActions = $state<Snippet>();
  import StoreConversationMessageActions from "./StoreConversationMessageActions.svelte";
  import { submitThreadConversation } from "../../conversations/thread-send.js";

  import StoreConversationRecovery from "../store-agent-chat/StoreConversationRecovery.svelte";
  import {
    consumeConversationDestination,
    conversationNavigation,
  } from "../../conversations/navigation.js";
  import { tick, untrack } from "svelte";
  import { mergeConversationProjections } from "../../conversations/merge-projections.js";
  import { useConversationInspection } from "../../conversations/inspection.svelte.js";
  import StoreMessageReactions from "./StoreMessageReactions.svelte";

  import { useLinkedConversation } from "../../conversations/linked-conversation.svelte.js";
  import StoreLinkedConversationActivity from "../store-agent-chat/StoreLinkedConversationActivity.svelte";
  import StoreConversationAgentSupplement from "../store-agent-chat/StoreConversationAgentSupplement.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreAgentChatBody from "../store-agent-chat-body/StoreAgentChatBody.svelte";
  import ChatMessageFooter from "../store-agent-chat-body/ChatMessageFooter.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import { threadConversationProjection } from "../../conversations/thread-adapter.js";
  import { useAppearance } from "../../appearance/store.svelte.js";

  import type { StoreController } from "../../controller/contracts.js";
  import type { ThreadsController } from "../../controller/thread-types.js";
  import type {
    CommunityMemberModel,
    CommunityMessageModel,
  } from "../../community/workspace-model.js";
  import type { ChatPromptSubmission } from "../store-agent-chat-body/model.js";
  let {
    threads,
    controller,
    messages,
    members,
    user,
    disabled = false,
    parent,
    onReply,
  }: {
    threads: ThreadsController;
    controller?: StoreController;
    messages: readonly CommunityMessageModel[];
    members: readonly CommunityMemberModel[];
    user: CommunityMemberModel;
    disabled?: boolean;
    parent?: string;
    onReply?: (id: string) => void;
  } = $props();
  const appearance = useAppearance();
  const id = $derived(threads.detail?.thread.value.thread_id ?? "");
  const conversation = $derived(
    threads.conversations.get(
      `${threads.conversationScope}:${id}:${parent ?? "main"}`,
    ),
  );
  const humanProjection = $derived(
    threadConversationProjection(
      messages,
      threads.projection.targets,
      threads.discussions,
      threads.events,
      members,
      user,
      parent,
    ),
  );
  const inspection = useConversationInspection();
  const linked = useLinkedConversation(() => ({
    controller,
    scope: `thread:${controller?.state.route.repository}:${id}`,
    home: controller?.state.route.repository ?? null,
    parent,
    user,
  }));
  const linkedSessions = $derived(linked.sessions);
  const linkedRows = $derived(linked.rows);
  const projection = $derived(
    mergeConversationProjections(humanProjection, linked.projection),
  );
  $effect(() => {
    const next = projection;
    untrack(() => conversation.adopt(next));
  });
  $effect(() => {
    conversation.host = {
      canComment: !disabled,
      author: user,
      communityEmojis: threads.emojis,
      members,
      onSend: disabled ? undefined : send,

      ...conversationNavigation(controller, conversation),
      onInspectTool: inspection
        ? (tool, opener) => inspection.open(tool, conversation.comments, opener)
        : undefined,
      onReact: disabled
        ? undefined
        : (id, emoji, custom) => {
            const target = threads.projection.targets[id];
            if (target)
              void threads.toggle(target, emoji, custom).catch(() => {});
            else {
              const linked = projection.targets.get(id),
                session = linkedSessions.find(
                  (s) => s.id === linked?.sessionId,
                );
              const feedback =
                linked &&
                controller?.execution?.discussions.message(
                  linked.sessionId,
                  linked.rowId,
                );
              if (session && feedback)
                void controller!
                  .execution!.discussions.toggle(
                    session.repository,
                    feedback.target,
                    emoji,
                    custom,
                  )
                  .catch(() => {});
            }
          },
    };
  });
  $effect(() => {
    const committed = threads.committedRequestId;
    untrack(() => conversation.comments.confirmPublication(committed, true));
  });
  const selectedAgent = $derived(
    mentionedAgentIds(conversation.comments.message)[0] ??
      linkedSessions.find((s) => s.setup_id)?.setup_id,
  );
  const selectedSession = $derived(
    linkedSessions.filter((s) => s.setup_id === selectedAgent).at(-1),
  );
  const initialAgentIds = $derived([
    ...new Set(linkedSessions.flatMap((s) => (s.setup_id ? [s.setup_id] : []))),
  ]);
  let element: HTMLDivElement;
  $effect(() => {
    const focus = parent;
    if (focus)
      void tick().then(() =>
        element
          ?.querySelector<HTMLElement>(
            `[data-chat-item="${CSS.escape(focus)}"]`,
          )
          ?.scrollIntoView({ block: "nearest" }),
      );
  });
  const reactions = $derived(
    Object.fromEntries(
      projection.items.map((item) => {
        const target = threads.projection.targets[item.id];
        if (target) return [item.id, threads.reactionModels(target)];
        const linked = projection.targets.get(item.id),
          session = linkedSessions.find((s) => s.id === linked?.sessionId);
        const feedback =
          linked &&
          controller?.execution?.discussions.message(
            linked.sessionId,
            linked.rowId,
          );
        return [
          item.id,
          session && feedback
            ? controller!
                .execution!.discussions.reader(session.repository)
                .reactionModels(feedback.target)
            : [],
        ];
      }),
    ),
  );
  $effect(() => {
    threads.conversations.destination;
    projection;
    untrack(
      () => void consumeConversationDestination(controller, conversation),
    );
  });
  const send = (submission: ChatPromptSubmission) =>
    submitThreadConversation(
      { threads, controller, conversation, disabled, id, parent },
      submission,
    );
</script>

{#snippet messageActions(messageId: string)}
  {@const item = projection.items.find((item) => item.id === messageId)}
  {#if item}<StoreConversationMessageActions
      menu
      {item}
      {messages}
      {threads}
      {conversation}
      {controller}
      {disabled}
      {user}
      {onReply}
      {id}
    />{/if}
{/snippet}

<div class="thread-conversation" bind:this={element}>
  <StoreAgentChatBody
    {messageActions}
    items={conversation.projection.items}
    commentState={conversation.comments}
    view={appearance?.live.conversationView ?? "document"}
    label={parent ? "Thread reply conversation" : "Thread conversation"}
    logLabel={parent
      ? "Thread replies"
      : `${threads.detail?.thread.value.title ?? "Thread"} messages`}
    onOpenThread={onReply}
    {reactions}
    onReact={conversation.host.onReact}
    onInspectTool={conversation.host.onInspectTool}
  >
    {#snippet beforeItems()}
      {#if threads.hasOlder}<Button
          variant="ghost"
          disabled={threads.status === "refreshing"}
          onclick={threads.older}>Load older messages</Button
        >{/if}
      {#if threads.discussionError}<p role="status">
          {threads.discussionError}
        </p>{/if}
    {/snippet}
    {#snippet afterItems()}{#if controller?.execution}<StoreLinkedConversationActivity
          {controller}
          sessions={linkedSessions}
        />{/if}{/snippet}
    {#snippet supplement(item)}
      {#if controller?.execution}<StoreConversationAgentSupplement
          manager={controller.execution}
          rows={linkedRows}
          {projection}
          {item}
          {user}
        />{/if}
      {#if messages.some((m) => m.id === item.id)}<StoreConversationMessageActions
          {item}
          {messages}
          {threads}
          {conversation}
          {controller}
          {disabled}
          {user}
          {onReply}
          {id}
        />{/if}
    {/snippet}
    {#snippet metadataSupplement(item)}
      {@const target = threads.projection.targets[item.id]}
      {@const linked = projection.targets.get(item.id)}
      {@const captured =
        linked &&
        controller?.execution?.discussions.message(
          linked.sessionId,
          linked.rowId,
        )}
      {#if target}<StoreMessageReactions
          {threads}
          {target}
          {disabled}
          showControls={false}
        />{:else if captured && controller?.execution}<StoreMessageReactions
          threads={controller.execution.discussions.reader(
            linkedSessions.find((s) => s.id === linked?.sessionId)!.repository,
          )}
          target={captured.target}
          {disabled}
          showControls={false}
        />{/if}
    {/snippet}
    {#snippet composer(presentation)}
      {#if controller?.execution}<StoreConversationRecovery
          {conversation}
          manager={controller.execution}
        />{/if}
      <CommunityChatComposer
        {presentation}
        dockActions
        bind:documentActions={inputActions}
        member={user}
        hideAvatar={conversation.comments.pending.length > 0}
        bind:value={conversation.comments.message}
        {members}
        communityEmojis={threads.emojis}
        audienceKey={JSON.stringify([
          threads.conversationScope,
          id,
          parent ?? "main",
        ])}
        {initialAgentIds}
        {disabled}
        placeholder=""
        enterInsertsNewline
        allowEmpty={conversation.comments.pending.length > 0}
        clearOnSubmit={false}
        showSubmitButton={false}
        onSubmit={async () => {
          if (!(await conversation.comments.send()))
            throw new Error(conversation.comments.error);
        }}
      />
    {/snippet}
    {#snippet footer()}<ChatMessageFooter
        actions={inputActions}
        canSend={!disabled && conversation.comments.canSend}
        onSend={() => void conversation.comments.send()}
        error={conversation.comments.error}
      >
        {#snippet workspace()}{#if controller?.execution && !parent}<StoreThreadCodebase
              {controller}
            /><StoreWorkspaceButton
              manager={controller.execution}
              sessions={linkedSessions}
              {conversation}
              {controller}
              repositories={threads.detail?.thread.value.codebase ??
                (threads.detail ? [controller.state.route.repository] : [])}
            />{/if}{/snippet}
        {#snippet controls()}{#if controller?.execution}<div
              class="thread-runtime-controls"
            >
              {#if !parent}<StoreThreadAgentRepositories {controller} />{/if}
              <StoreConversationRuntime
                manager={controller.execution}
                agentId={selectedAgent}
                session={selectedSession}
                {disabled}
                onAgentChange={(id) => {
                  conversation.comments.message = `${agentMentionToken(id)} ${conversation.comments.message}`;
                }}
              />
            </div>{/if}{/snippet}
      </ChatMessageFooter>{/snippet}
  </StoreAgentChatBody>
</div>

<style>
  .thread-runtime-controls {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    min-width: 0;
  }
  .thread-conversation {
    display: flex;
    flex-direction: column;
    min-height: 0;
    flex: 1;
  }
</style>
