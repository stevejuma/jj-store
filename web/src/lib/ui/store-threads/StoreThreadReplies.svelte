<script lang="ts">
  import { agentMember } from "../community-agents/model.js";
  import { originAgentIds } from "../../controller/agent-chat-context.js";
  import { sendDiscussionMessage } from "../../controller/agent-discussion-dispatch.js";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityThreadPanel from "../community-thread-panel/CommunityThreadPanel.svelte";
  import type {
    CommunityMemberModel,
    CommunityMessageModel,
  } from "../../community/workspace-model.js";
  import StoreMessageReactions from "./StoreMessageReactions.svelte";
  import type { ThreadsController } from "../../controller/thread-types.js";
  import { replyRoot, threadDescendants } from "./thread-view.js";
  let {
    threads,
    controller,
    rootId,
    members,
    currentMemberId,
    disabled,
  }: {
    threads: ThreadsController;
    controller?: StoreController;
    rootId: string;
    members: CommunityMemberModel[];
    currentMemberId?: string;
    disabled: boolean;
  } = $props();
  const conversationRoot = $derived(
    replyRoot(threads.projection.messages, rootId),
  );
  const messages = $derived.by(() => {
    return threadDescendants(threads.projection.messages, conversationRoot).map(
      (m): CommunityMessageModel => ({
        ...m,
        reactions: threads.reactionModels(threads.projection.targets[m.id]!),
      }),
    );
  });
</script>

<CommunityThreadPanel
  thread={{ rootMessageId: conversationRoot, messages }}
  members={[
    ...members,
    ...(controller?.execution?.value?.setups ?? []).map(agentMember),
  ]}
  audienceKey={controller
    ? JSON.stringify([
        controller.state.member.id,
        controller.state.route.repository,
        controller.state.route.threadId,
        conversationRoot,
      ])
    : undefined}
  {currentMemberId}
  initialAgentIds={controller
    ? originAgentIds(
        controller.execution,
        `thread:${controller.state.route.repository}:${controller.state.route.threadId}`,
        controller.state.route.repository,
      )
    : undefined}
  {disabled}
  focusMessageId={rootId}
  history={{
    key: conversationRoot,
    hasNewer: false,
    complete: !threads.hasOlder,
    hasOlder: threads.hasOlder,
    loading: threads.status === "refreshing",
  }}
  onLoadOlder={threads.older}
  disabledReason="Sending replies is unavailable."
  onSendReply={disabled
    ? undefined
    : (parent, text) =>
        controller
          ? sendDiscussionMessage(controller, text, parent)
          : threads.send(text, parent)}
  onReply={disabled ? undefined : threads.selectReply}
  communityEmojis={threads.emojis}
  onToggleReaction={disabled
    ? undefined
    : (id, emoji, custom) =>
        threads
          .toggle(threads.projection.targets[id]!, emoji, custom, true)
          .catch(() => {})}
  onAddReaction={disabled
    ? undefined
    : (id, emoji, custom) =>
        threads
          .toggle(threads.projection.targets[id]!, emoji, custom)
          .catch(() => {})}
  onSaveMessageEdit={disabled
    ? undefined
    : (id, text) => threads.send(text, undefined, id)}
>
  {#snippet messageMetadata(message)}
    {#if threads.projection.targets[message.id]}<StoreMessageReactions
        {threads}
        target={threads.projection.targets[message.id]!}
        {disabled}
        showControls={false}
      />{/if}
  {/snippet}
</CommunityThreadPanel>
