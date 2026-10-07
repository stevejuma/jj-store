<script lang="ts">
  import "./thread-panel.css";
  import { focusNativeMessage } from "../focus-message.js";
  import {
    Layout,
    LayoutScrollButton,
    MessageList,
  } from "@lapismd/design-core/ai/chat";
  import { useAppearance } from "../../appearance/store.svelte.js";
  import { useDrafts } from "../../appearance/drafts.svelte.js";
  const appearance = useAppearance();
  const drafts = useDrafts();
  import { projectThreadTree } from "./thread-tree-layout.js";
  import CommunityThreadTreeRow from "./CommunityThreadTreeRow.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import { memberById } from "../community-workspace/model.js";
  import { isMessageContinuation } from "../community-message/grouping.js";
  import type { ThreadPanelProps } from "./props.js";
  let panel: ThreadPanelProps = $props();
  const tree = $derived(
    projectThreadTree(panel.thread.messages, panel.thread.rootMessageId),
  );
  const draft = $derived(drafts.get(`replies:${panel.thread.rootMessageId}`));
  function startEditing(id: string) {
    draft.editing = id;
    draft.value = panel.thread.messages.find((m) => m.id === id)?.content ?? "";
  }
  const target = $derived(
    panel.thread.messages.find(
      (m) => m.id === (draft.replying ?? panel.thread.rootMessageId),
    ),
  );
  async function submit(content: string) {
    if (draft.editing && panel.onSaveMessageEdit) {
      await panel.onSaveMessageEdit(draft.editing, content);
      draft.editing = undefined;
    } else {
      await panel.onSendReply?.(
        draft.replying ?? panel.thread.rootMessageId,
        content,
      );
      draft.replying = undefined;
    }
  }
  const rowPanel = $derived({
    ...panel,
    onReply:
      panel.disabled || !panel.onSendReply
        ? undefined
        : (id: string) => {
            draft.replying = id;
            draft.editing = undefined;
          },
  });
</script>

<div
  class="community-thread-panel"
  use:focusNativeMessage={panel.focusMessageId}
>
  <Layout
    density={appearance?.density ?? "compact"}
    scrollMode="history"
    conversationKey={panel.thread.rootMessageId}
    aria-label="Message thread"
    isEmpty={tree.messages.length === 0}
  >
    {#snippet scrollButton({
      isVisible,
      hasNewMessages,
      scrollToBottom,
    })}<LayoutScrollButton
        {isVisible}
        {hasNewMessages}
        onClick={scrollToBottom}
      />{/snippet}
    {#snippet composer()}
      <CommunityChatComposer
        bind:value={draft.value}
        members={panel.members}
        audienceKey={draft.editing ? undefined : panel.audienceKey}
        initialAgentIds={panel.initialAgentIds}
        rooms={panel.rooms ?? []}
        communityEmojis={panel.communityEmojis ?? []}
        disabled={panel.disabled}
        disabledReason={panel.disabledReason}
        composerStatus={panel.composerStatus}
        placeholder="Reply…"
        context={draft.editing
          ? { kind: "edit", id: draft.editing, label: "Editing reply" }
          : target
            ? {
                kind: "reply",
                target: {
                  eventId: target.id,
                  roomId: panel.thread.rootMessageId,
                  authorId: target.authorId,
                  kind: "channel",
                  authorName: memberById(panel.members, target.authorId).name,
                  content: target.content,
                },
              }
            : undefined}
        onCancelContext={() => {
          draft.editing = undefined;
          draft.replying = undefined;
          draft.value = "";
        }}
        onSubmit={(s) => submit(s.content)}
      />
    {/snippet}
    <MessageList
      density={appearance?.density ?? "compact"}
      aria-label="Thread replies"
      conversationKey={panel.thread.rootMessageId}
      latestMessageId={tree.messages.at(-1)?.id ?? ""}
      scrollToTopAction={panel.onLoadOlder}
      hasOlder={panel.history?.hasOlder ?? false}
      loading={panel.history?.loading ?? false}
    >
      {#each tree.messages as message, index (message.id)}<CommunityThreadTreeRow
          panel={rowPanel}
          {message}
          isContinuation={isMessageContinuation(tree.messages, index)}
          members={panel.members}
          focused={panel.focusMessageId}
          treeRow={tree.rowsById.get(message.id)}
          transcriptComment={false}
          {startEditing}
        />{/each}
    </MessageList>
  </Layout>
</div>
