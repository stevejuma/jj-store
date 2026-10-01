<script lang="ts">
  import CommunityThreadPanel from "../community-thread-panel/CommunityThreadPanel.svelte";
  import type {
    CommunityMemberModel,
    CommunityMessageModel,
  } from "../../community/workspace-model.js";
  import { untrack } from "svelte";
  import type { ThreadsController } from "../../controller/thread-types.js";
  import { replyRoot } from "./thread-view.js";
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
  let error = $state("");
  const conversationRoot = $derived(
    replyRoot(threads.projection.messages, rootId),
  );
  const messages = $derived.by(() => {
    const all = threads.projection.messages,
      ids = new Set([conversationRoot]);
    for (let i = 0; i < all.length; i++)
      for (const m of all)
        if (m.parentMessageId && ids.has(m.parentMessageId)) ids.add(m.id);
    return all
      .filter((m) => ids.has(m.id))
      .map((m): CommunityMessageModel => ({
        ...m,
        reactions: threads.reactionModels(threads.projection.targets[m.id]!),
      }));
  });
  $effect(() => {
    const targets = messages.map((m) => threads.projection.targets[m.id]!);
    threads.reactionRevision;
    let live = true;
    untrack(() => {
      for (const target of targets)
        void threads.loadReactions(target).catch((e) => {
          if (live) error = String(e);
        });
    });
    return () => {
      live = false;
    };
  });
  async function react(
    id: string,
    emoji: string,
    custom?: { shortcode: string; url: string },
    fromBar = false,
  ) {
    if (disabled || threads.busy || threads.pending) return;
    try {
      await threads.toggle(
        threads.projection.targets[id]!,
        emoji,
        custom,
        fromBar,
      );
      error = "";
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }
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
    : (id, emoji, custom) => react(id, emoji, custom, true)}
  onAddReaction={disabled
    ? undefined
    : (id, emoji, custom) => react(id, emoji, custom)}
  onSaveMessageEdit={disabled
    ? undefined
    : (id, text) => threads.send(text, undefined, id)}
/>
{#if error}<p role="alert">{error}</p>{/if}
