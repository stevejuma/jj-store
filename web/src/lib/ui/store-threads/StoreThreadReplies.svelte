<script lang="ts">
  import {
    CommunityThreadPanel,
    CommunityDetails,
  } from "@lapismd/lapis-community/components";
  import type {
    CommunityMemberModel,
    CommunityMessageModel,
  } from "@lapismd/lapis-community/components";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { untrack } from "svelte";
  import { principalMember } from "../../collaboration/presentation.js";
  import type { ThreadsController } from "../../controller/thread-types.js";
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
  let error = $state(""),
    actorsOpen = $state(false);
  const messages = $derived.by(() => {
    const all = threads.projection.messages,
      ids = new Set([rootId]);
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
  const actors = $derived([
    ...new Map(
      messages
        .flatMap(
          (m) =>
            threads.getReaction(threads.projection.targets[m.id]!)?.actors ??
            [],
        )
        .map((a) => [a.actor, principalMember(a.actor)]),
    ).values(),
  ]);
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
  thread={{ rootMessageId: rootId, messages }}
  {members}
  {currentMemberId}
  {disabled}
  disabledReason="Sending replies is unavailable."
  onSendReply={(parent, text) => threads.send(text, parent)}
  onReply={threads.selectReply}
  communityEmojis={threads.emojis}
  onToggleReaction={(id, emoji, custom) => react(id, emoji, custom, true)}
  onAddReaction={(id, emoji, custom) => react(id, emoji, custom)}
  onSaveMessageEdit={(id, text) => threads.send(text, undefined, id)}
/>
{#if error}<p role="alert">{error}</p>{/if}
{#if actors.length}<Button
    variant="ghost"
    size="sm"
    onclick={() => (actorsOpen = true)}>Who reacted to replies</Button
  >{/if}
<Dialog.Root bind:open={actorsOpen}
  ><Dialog.Content
    ><Dialog.Header
      ><Dialog.Title>Who reacted to replies</Dialog.Title><Dialog.Description
        >Authorized reaction actors in the loaded reply conversation.</Dialog.Description
      ></Dialog.Header
    ><CommunityDetails
      room={{ id: rootId, name: "Replies", kind: "channel" }}
      members={actors}
      details={{
        description: "Visible actors for the loaded reply messages.",
        memberIds: actors.map((m) => m.id),
      }}
    /></Dialog.Content
  ></Dialog.Root
>
