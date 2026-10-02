<script lang="ts">
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
    rootId,
    members,
    currentMemberId,
    disabled,
  }: {
    threads: ThreadsController;
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
  {members}
  {currentMemberId}
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
    : (parent, text) => threads.send(text, parent)}
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
