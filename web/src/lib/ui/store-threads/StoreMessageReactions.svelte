<script lang="ts">
  import CommunityReactions from "../community-reactions/CommunityReactions.svelte";
  import CommunityDetails from "../community-details/CommunityDetails.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { untrack } from "svelte";
  import type {
    ReactionTarget,
    ThreadsController,
  } from "../../controller/thread-types.js";
  import { principalMember } from "../../collaboration/presentation.js";
  let {
    threads,
    target,
    disabled = false,
    showControls = true,
  }: {
    threads: ThreadsController;
    target: ReactionTarget;
    disabled?: boolean;
    showControls?: boolean;
  } = $props();
  let error = $state(""),
    actorsOpen = $state(false);
  const items = $derived(threads.getReaction(target));
  const members = $derived(
    items?.actors.map((actor) => principalMember(actor.actor)) ?? [],
  );
  $effect(() => {
    const captured = target;
    threads.reactionRevision;
    let live = true;
    untrack(() => {
      void threads.loadReactions(captured).catch((e) => {
        if (live) error = e instanceof Error ? e.message : String(e);
      });
      void threads.loadEmojis().catch((e) => {
        if (live) error = e instanceof Error ? e.message : String(e);
      });
    });
    return () => {
      live = false;
    };
  });
  async function select(
    emoji: string,
    custom?: { shortcode: string; url: string },
    fromBar = false,
  ) {
    if (disabled || threads.busy || threads.pending) return;
    error = "";
    try {
      await threads.toggle(target, emoji, custom, fromBar);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }
</script>

<div
  class="store-message-reactions"
  role="group"
  aria-label="Message reactions"
>
  {#if showControls}<CommunityReactions
      reactions={threads.reactionModels(target)}
      communityEmojis={threads.emojis}
      disabled={disabled || threads.busy || !!threads.pending}
      onToggle={(emoji) => select(emoji, undefined, true)}
      onSelect={(emoji, custom) => select(emoji, custom)}
    />{/if}
  {#if members.length}<Button
      variant="ghost"
      size="sm"
      onclick={() => (actorsOpen = true)}>Who reacted</Button
    >{/if}
  {#if threads.pending}<span role="status"
      >An action is awaiting confirmation.</span
    ><Button
      variant="outline"
      size="sm"
      disabled={threads.busy}
      onclick={threads.checkPending}>Check action status</Button
    ><Button
      variant="ghost"
      size="sm"
      disabled={threads.busy}
      onclick={() => threads.retryPending().catch((e) => (error = String(e)))}
      >Retry same action</Button
    >{/if}
  {#if items?.conflict_count}<span role="status"
      >Reaction conflict. Refresh and resolve before changing it.</span
    >{/if}
  {#if error}<span role="alert">{error}</span><Button
      variant="ghost"
      size="sm"
      onclick={() => {
        error = "";
        void threads
          .loadReactions(target, true)
          .catch((e) => (error = String(e)));
      }}>Retry reactions</Button
    >{/if}
</div>
<Dialog.Root bind:open={actorsOpen}>
  <Dialog.Content preventScroll={false}
    ><Dialog.Header
      ><Dialog.Title>Who reacted</Dialog.Title><Dialog.Description
        >Authorized reactions to this exact message revision.</Dialog.Description
      ></Dialog.Header
    >
    <CommunityDetails
      room={{ id: "reactions", name: "this message", kind: "channel" }}
      details={{
        description:
          "People with visible reactions. Conflicted reactions are excluded from the counts.",
        memberIds: members.map((m) => m.id),
      }}
      {members}
    />
  </Dialog.Content>
</Dialog.Root>

<style>
  .store-message-reactions {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.35rem;
  }
</style>
