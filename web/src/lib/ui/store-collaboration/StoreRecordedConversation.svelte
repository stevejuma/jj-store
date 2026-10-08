<script lang="ts">
  import { useLinkedConversation } from "../../conversations/linked-conversation.svelte.js";
  import { mergeConversationProjections } from "../../conversations/merge-projections.js";
  import StoreLinkedConversationActivity from "../store-agent-chat/StoreLinkedConversationActivity.svelte";
  import StoreConversationAgentSupplement from "../store-agent-chat/StoreConversationAgentSupplement.svelte";
  import {
    useConversationReactions,
    recordedReactionTargets,
  } from "../../conversations/reactions.svelte.js";
  import { submitRecordedConversation } from "../../conversations/recorded-send.js";

  import StoreConversationRecovery from "../store-agent-chat/StoreConversationRecovery.svelte";
  import {
    consumeConversationDestination,
    openConversationSource,
  } from "../../conversations/navigation.js";
  import { markdownBlocks } from "../store-agent-chat-body/markdown-blocks.js";
  import { untrack } from "svelte";
  import { sha256 } from "@noble/hashes/sha2.js";
  import { bytesToHex } from "@noble/hashes/utils.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import type { CommunityMemberModel } from "../../community/workspace-model.js";
  import { agentMember } from "../community-agents/model.js";

  import { recordedConversationProjection } from "../../conversations/recorded-adapter.js";
  import { useConversationInspection } from "../../conversations/inspection.svelte.js";
  import { useAppearance } from "../../appearance/store.svelte.js";
  import StoreAgentChatBody from "../store-agent-chat-body/StoreAgentChatBody.svelte";
  import ChatMessageFooter from "../store-agent-chat-body/ChatMessageFooter.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import StoreMessageReactions from "../store-threads/StoreMessageReactions.svelte";
  import StoreAgentPromptContent from "../store-agent-execution/StoreAgentPromptContent.svelte";
  import { agentSentContext } from "../../collaboration/agent-message-content.js";
  import type { ChatPromptSubmission } from "../store-agent-chat-body/model.js";
  let {
    controller,
    activity,
    sessionId,
    integration,
    agent,
    member,
    interactive = true,
    legacy = [],
  }: {
    controller: StoreController;
    activity: readonly Model<"AgentActivity">[];
    sessionId: string;
    integration: Model<"IntegrationId">;
    agent: CommunityMemberModel;
    member: CommunityMemberModel;
    interactive?: boolean;
    legacy?: readonly Model<"AgentTranscriptChunk">[];
  } = $props();
  const threads = $derived(controller.threads!);
  const appearance = useAppearance(),
    inspection = useConversationInspection();
  const identity = (suffix: string) =>
    bytesToHex(
      sha256(
        new TextEncoder().encode(
          `recorded-conversation/${controller.state.repository.id}/${sessionId}/${suffix}`,
        ),
      ),
    ).slice(0, 32);
  const parentId = $derived(identity("thread"));
  const key = $derived(`${threads.conversationScope}:${parentId}`);
  const conversation = $derived(threads.conversations.get(key));
  const read = $derived(threads.conversationReads[key]);
  const user = $derived({
    ...member,
    id: controller.state.repository.principal ?? member.id,
  });
  const captured = $derived(
    controller.state.agentSession?.session.session_id === sessionId
      ? controller.state.agentSession
      : undefined,
  );
  const linked = useLinkedConversation(() => ({
    controller,
    scope: `recorded:${controller.state.repository.id}:${sessionId}`,
    home: controller.state.repository.id,
    user,
  }));
  const projection = $derived.by(() => {
    const next = recordedConversationProjection(
      activity,
      sessionId,
      integration,
      agent,
      user,
      read,
      captured?.conversation_feedback ?? undefined,
    );
    if (!activity.length)
      next.items.unshift(
        ...legacy.map((chunk) => {
          const id = `${sessionId}:legacy:${chunk.sequence}`;
          return {
            id,
            kind: "message" as const,
            role: "assistant" as const,
            author: agent,
            label: "Assistant message",
            blocks: markdownBlocks(
              id,
              String(chunk.sequence),
              "Assistant message",
              chunk.content,
            ).map((block) => ({ ...block, commentable: false })),
            turn: {
              id,
              author: agent.name,
              prompt: "Recorded output",
              summary: chunk.content.slice(0, 240),
            },
          };
        }),
      );
    return mergeConversationProjections(next, linked.projection);
  });
  const disabled = $derived(
    !interactive ||
      !!captured?.conversation_unavailable ||
      !!captured?.conversation_feedback?.write_unavailable ||
      !["write", "admin"].includes(
        controller.state.repository.permission ?? "",
      ) ||
      threads.busy ||
      !!threads.pending,
  );
  const feedback = useConversationReactions(() => ({
    threads,
    targets: recordedReactionTargets(activity, sessionId, projection),
    projection,
    rows: linked.rows,
    manager: controller.execution,
  }));
  $effect(() => {
    const id = parentId;
    void threads.loadConversation(id).catch(() => {});
    const timer = setInterval(() => {
      void threads.loadConversation(id).catch(() => {});
      void controller.actions.refreshAgentConversation?.();
    }, 15000);
    return () => clearInterval(timer);
  });
  $effect(() => {
    const next = projection;
    untrack(() => conversation.adopt(next));
  });
  $effect(() => {
    conversation.host = {
      canComment: !disabled,
      author: user,
      communityEmojis: threads.emojis,
      members: [
        user,
        agent,
        ...(controller.execution?.value?.setups ?? []).map(agentMember),
      ],
      onSend: disabled ? undefined : send,
      onNavigateSource: controller
        ? (thread) =>
            thread.source
              ? openConversationSource(controller, thread.source)
              : undefined
        : undefined,
      onNavigateReply: controller
        ? async (id) => {
            if (conversation.projection.items.some((i) => i.id === id)) return;
            const answer = conversation.projection.answers?.get(id);
            if (answer)
              await openConversationSource(controller, answer.answer, id);
          }
        : undefined,
      onInspectTool: inspection
        ? (tool, opener) => inspection.open(tool, conversation.comments, opener)
        : undefined,
      onReact: disabled
        ? undefined
        : (id, emoji, custom) => {
            void feedback.toggle(id, emoji, custom).catch(() => {});
          },
    };
  });
  $effect(() => {
    const committed = threads.committedRequestId;
    untrack(() => conversation.comments.confirmPublication(committed, true));
  });
  $effect(() => {
    threads.conversations.destination;
    projection;
    untrack(
      () => void consumeConversationDestination(controller, conversation),
    );
  });
  const send = (submission: ChatPromptSubmission) =>
    submitRecordedConversation(
      { controller, threads, conversation, sessionId, parentId, identity },
      submission,
    );
</script>

<StoreAgentChatBody
  items={conversation.projection.items}
  commentState={conversation.comments}
  view={appearance?.live.conversationView ?? "document"}
  reactions={feedback.models}
  onReact={conversation.host.onReact}
  onInspectTool={conversation.host.onInspectTool}
>
  {#snippet beforeItems()}{#if legacy.length}<p role="status">
        This legacy transcript has no exact message source. Selection comments
        are unavailable.
      </p>
      {#if controller.state.canLoadMoreTranscript}<Button
          variant="ghost"
          onclick={() => controller.actions.loadMore("transcript")}
          >Load older messages</Button
        >{/if}{/if}{#if read?.error || captured?.conversation_unavailable}<p
        role="status"
      >
        {read?.error || captured?.conversation_unavailable}
      </p>{/if}{/snippet}
  {#snippet afterItems()}{#if controller.execution}<StoreLinkedConversationActivity
        {controller}
        sessions={linked.sessions}
      />{/if}{/snippet}
  {#snippet supplement(item)}
    {#if controller.execution}<StoreConversationAgentSupplement
        manager={controller.execution}
        rows={linked.rows}
        projection={linked.projection}
        {item}
        {user}
      />{/if}
    {@const entry = activity.find(
      (e) => e.type === "message" && `${sessionId}:${e.message_id}` === item.id,
    )}
    {#if entry?.type === "message" && entry.role === "user"}<StoreAgentPromptContent
        content=""
        sentContext={agentSentContext(entry.content, integration)}
      />{/if}
  {/snippet}
  {#snippet metadataSupplement(item)}{@const entry = feedback.entry(
      item.id,
    )}{#if entry}<StoreMessageReactions
        threads={entry.threads}
        target={entry.target}
        {disabled}
        showControls={false}
      />{/if}{/snippet}
  {#snippet composer()}
    {#if controller?.execution}<StoreConversationRecovery
        {conversation}
        manager={controller.execution}
      />{/if}
    <CommunityChatComposer
      bind:value={conversation.comments.message}
      members={[
        user,
        agent,
        ...(controller.execution?.value?.setups ?? []).map(agentMember),
      ]}
      communityEmojis={threads.emojis}
      audienceKey={key}
      {disabled}
      ariaLabel="Message about recorded conversation"
      placeholder=""
      enterInsertsNewline
      showSubmitButton={false}
      allowEmpty={conversation.comments.pending.length > 0}
      clearOnSubmit={false}
      onSubmit={async () => {
        if (!(await conversation.comments.send()))
          throw new Error(conversation.comments.error);
      }}
    />
    {#if threads.pending}<Button
        variant="outline"
        disabled={threads.busy}
        onclick={() => threads.checkPending()}>Check action status</Button
      ><Button
        variant="ghost"
        disabled={threads.busy}
        onclick={() => threads.retryPending().catch(() => {})}
        >Retry same action</Button
      >{/if}
  {/snippet}
  {#snippet footer()}<ChatMessageFooter
      canSend={!disabled && conversation.comments.canSend}
      onSend={() => void conversation.comments.send()}
      error={conversation.comments.error}
    />{/snippet}
</StoreAgentChatBody>
