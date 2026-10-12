<script lang="ts">
  import {
    readOnlyTranscriptItems,
    transcriptMessage,
  } from "./readonly-transcript.js";
  import type { Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import StoreRecordedConversation from "./StoreRecordedConversation.svelte";
  import StoreAgentChatBody from "../store-agent-chat-body/StoreAgentChatBody.svelte";
  import { ChatBodyComments } from "../store-agent-chat-body/comments.svelte.js";
  import { useConversationInspection } from "../../conversations/inspection.svelte.js";
  import { useAppearance } from "../../appearance/store.svelte.js";
  const appearance = useAppearance();
  import * as Chat from "@lapismd/design-core/ai/chat";
  import CommunityMessage from "../community-message/CommunityMessage.svelte";
  import type { CommunityMemberModel } from "../../community/workspace-model.js";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreMessageReactions from "../store-threads/StoreMessageReactions.svelte";
  import StoreAgentTools from "./StoreAgentTools.svelte";
  import { agentMessageTimestamp } from "../../collaboration/presentation.js";
  import { agentTimeline } from "../../collaboration/agent-timeline.js";
  import StoreAgentPromptContent from "../store-agent-execution/StoreAgentPromptContent.svelte";
  import {
    agentSentContext,
    agentMessageContent,
  } from "../../collaboration/agent-message-content.js";
  import StoreRecordedMessageMenu from "./StoreRecordedMessageMenu.svelte";
  import StoreMessageActions from "./StoreMessageActions.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";

  let {
    activity,
    agent,
    member,
    patches = {},
    controller,
    interactive = true,
    sessionId,
    integration,
    legacy = [],
    afterItems,
  }: {
    activity: readonly Model<"AgentActivity">[];
    agent: CommunityMemberModel;
    member: CommunityMemberModel;
    patches?: Readonly<Record<string, string>>;
    controller: StoreController;
    interactive?: boolean;
    sessionId: string;
    integration: Model<"IntegrationId">;
    legacy?: readonly Model<"AgentTranscriptChunk">[];
    afterItems?: Snippet;
  } = $props();

  const members = $derived([agent, member]);
  const inspection = useConversationInspection();
  const readOnlyItems = $derived(
    readOnlyTranscriptItems(
      activity,
      sessionId,
      integration,
      agent,
      member,
      legacy,
    ),
  );
  const readOnlyComments = new ChatBodyComments(() => ({
    items: readOnlyItems,
    canComment: false,
  }));
  const timeline = $derived(agentTimeline(activity));
  const canReact = $derived(
    interactive &&
      ["write", "admin"].includes(controller.state.repository.permission ?? ""),
  );
  const reactionDisabled = $derived(
    !canReact || !!controller.threads?.busy || !!controller.threads?.pending,
  );
</script>

{#if controller.threads}
  <StoreRecordedConversation
    {controller}
    {activity}
    {sessionId}
    {integration}
    {agent}
    {member}
    {interactive}
    {legacy}
    {afterItems}
  />
{:else if (appearance?.live.conversationView ?? "document") === "document"}
  <StoreAgentChatBody
    items={readOnlyItems}
    commentState={readOnlyComments}
    canComment={false}
    logLabel="Agent conversation activity"
    onInspectTool={inspection
      ? (tool, opener) => inspection.open(tool, readOnlyComments, opener)
      : undefined}
    loading={!!legacy.length &&
      controller.state.status.transcript === "refreshing"}
    {afterItems}
  >
    {#snippet customBlock(block)}<CommunityMarkdownContent
        content={block.text}
        promptElements
        fullWidth
        {members}
      />{/snippet}
    {#snippet beforeItems()}{#if legacy.length && controller.state.canLoadMoreTranscript}<Button
          variant="ghost"
          disabled={controller.state.status.transcript === "refreshing"}
          onclick={() => controller.actions.loadMore("transcript")}
          >Load older messages</Button
        >{/if}{/snippet}
    {#snippet messageActions(id)}<StoreRecordedMessageMenu
        {id}
        {sessionId}
        {activity}
        {legacy}
        transcriptId={controller.state.agentSession?.transcripts[0]
          ?.transcript_id}
      />{/snippet}
    {#snippet supplement(item)}
      {@const entry = activity.find(
        (e) =>
          e.type === "message" && `${sessionId}:${e.message_id}` === item.id,
      )}
      {#if entry?.type === "message" && entry.role === "user"}<StoreAgentPromptContent
          content=""
          sentContext={agentSentContext(entry.content, integration)}
          {members}
        />{/if}
    {/snippet}
  </StoreAgentChatBody>
{:else if !activity.length && legacy.length}
  <Chat.MessageList
    density={appearance?.density ?? "compact"}
    aria-label="Agent conversation activity"
  >
    <CommunityProjectsVirtualRows
      rows={legacy}
      hasMore={controller.state.canLoadMoreTranscript}
      loading={controller.state.status.transcript === "refreshing"}
      loadingLabel="Loading older agent transcript messages"
      onLoadMore={() => controller.actions.loadMore("transcript")}
      getKey={(chunk) => String(chunk.sequence)}
      estimateSize={() => 118}
    >
      {#snippet row(chunk)}<CommunityMessage
          promptElements
          message={{
            id: `${sessionId}:${chunk.sequence}`,
            authorId: sessionId,
            content: agentMessageContent(chunk, integration),
            ...agentMessageTimestamp(chunk.occurred_at_millis),
          }}
          author={agent}
          members={[agent]}
          >{#snippet supplement()}<StoreMessageActions
              authorName={agent.name}
              items={[
                { label: "Copy message", value: chunk.content },
                { label: "Copy session ID", value: sessionId },
                {
                  label: "Copy transcript ID",
                  value:
                    controller.state.agentSession?.transcripts[0]
                      ?.transcript_id,
                },
                { label: "Copy sequence", value: String(chunk.sequence) },
              ]}
            />{/snippet}</CommunityMessage
        >{/snippet}
    </CommunityProjectsVirtualRows>
  </Chat.MessageList>
  {@render afterItems?.()}
{:else}
  <Chat.MessageList
    density={appearance?.density ?? "compact"}
    isEmpty={activity.length === 0}
    aria-label="Agent conversation activity"
  >
    <CommunityProjectsVirtualRows
      rows={timeline}
      getKey={(entry) => `${entry.type}:${entry.sequence}`}
      estimateSize={(entry) => (entry?.type === "tools" ? 42 : 118)}
    >
      {#snippet row(entry)}
        {#if entry.type === "message"}
          {@const author =
            entry.role === "assistant" || entry.role === "tool"
              ? agent
              : member}
          <CommunityMessage
            promptElements
            message={transcriptMessage(
              entry,
              integration,
              agent,
              member,
              controller.threads,
            )}
            {author}
            {members}
            highlighted={entry.role === "user"}
            onToggleReaction={canReact &&
            controller.threads &&
            entry.reaction_target
              ? (_id, emoji, custom) => {
                  if (!reactionDisabled)
                    void controller
                      .threads!.toggle(
                        entry.reaction_target!,
                        emoji,
                        custom,
                        true,
                      )
                      .catch(() => {});
                }
              : undefined}
            >{#snippet content()}<StoreAgentPromptContent
                content={agentMessageContent(entry.content, integration)}
                sentContext={entry.role === "user"
                  ? agentSentContext(entry.content, integration)
                  : undefined}
                {members}
              />{/snippet}
            {#snippet supplement()}<StoreMessageActions
                {reactionDisabled}
                communityEmojis={controller.threads?.emojis ?? []}
                onReact={canReact && controller.threads && entry.reaction_target
                  ? (emoji, custom) => {
                      if (!reactionDisabled)
                        void controller
                          .threads!.toggle(
                            entry.reaction_target!,
                            emoji,
                            custom,
                          )
                          .catch(() => {});
                    }
                  : undefined}
                authorName={author.name}
                items={[
                  { label: "Copy message", value: entry.content.content },
                  { label: "Copy message ID", value: entry.message_id },
                  { label: "Copy turn ID", value: entry.turn_id },
                  { label: "Copy session ID", value: sessionId },
                  { label: "Copy sequence", value: String(entry.sequence) },
                ]}
              />{/snippet}
            {#snippet metadataSupplement()}{#if controller.threads && entry.reaction_target}<StoreMessageReactions
                  threads={controller.threads}
                  target={entry.reaction_target}
                  showControls={false}
                />{/if}{/snippet}</CommunityMessage
          >
        {:else if entry.type === "tools"}
          <div class="store-agent-transcript__tools">
            <StoreAgentTools
              items={entry.items}
              {patches}
              {controller}
              interactive={interactive &&
                controller.state.repository.profile !== "git_sha1_jj045"}
            />
          </div>
        {:else if entry.type === "capture_gap"}
          <Chat.SystemMessage>
            Capture gap from {entry.missing_from} to {entry.missing_to}
          </Chat.SystemMessage>
        {:else if entry.type === "permission" && entry.state !== "not_observed"}
          <Chat.SystemMessage>Tool permission {entry.state}</Chat.SystemMessage>
        {/if}
      {/snippet}
    </CommunityProjectsVirtualRows>
  </Chat.MessageList>
  {@render afterItems?.()}
{/if}

<style>
  .store-agent-transcript__tools {
    width: 100%;
    min-width: 0;
    margin-inline: -0.5rem;
  }

  :global([data-ui-component="ai-chat-message-list"]) {
    min-height: 0;
    padding: 0.75rem;
  }

  .store-agent-transcript__tools :global([data-ai-chat-part="group-trigger"]),
  .store-agent-transcript__tools :global([data-ai-chat-part="call-trigger"]) {
    padding-inline-start: 0.5rem;
  }
</style>
