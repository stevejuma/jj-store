<script lang="ts">
  import { MessageMetadata } from "@lapismd/design-core/ai/chat";
  import { targetKey } from "../../collaboration/thread-presentation.js";
  import CommunityReactions from "../community-reactions/CommunityReactions.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { untrack } from "svelte";
  import type {
    ReactionTarget,
    ThreadsController,
  } from "../../controller/thread-types.js";
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
  let error = $state("");
  const actionError = $derived(threads.reactionError(target));
  const items = $derived(threads.getReaction(target));
  const pending = $derived(
    threads.pending?.kind === "reactions" &&
      (threads.pending.request.command.type === "set" ||
        threads.pending.request.command.type === "clear") &&
      targetKey(threads.pending.request.command.target) === targetKey(target),
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
    } catch {
      /* Targeted controller feedback is rendered below. */
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
  {#snippet feedback()}
    {#if pending}<span role="status">Awaiting reaction confirmation.</span>
      <Button
        variant="outline"
        size="sm"
        disabled={threads.busy}
        onclick={threads.checkPending}>Check action status</Button
      >
      <Button
        variant="ghost"
        size="sm"
        disabled={threads.busy}
        onclick={() => threads.retryPending().catch((e) => (error = String(e)))}
        >Retry same action</Button
      >
    {/if}
    {#if items?.conflict_count}<span role="status"
        >Reaction conflict. Refresh and resolve before changing it.</span
      >{/if}
    {#if !pending && (error || actionError)}<span role="alert"
        >{error || actionError}</span
      ><Button
        variant="ghost"
        size="sm"
        onclick={() => {
          error = "";
          void threads
            .loadReactions(target, true)
            .catch((e) => (error = String(e)));
        }}>Retry reactions</Button
      >{/if}
  {/snippet}
  {#if pending || items?.conflict_count || error || actionError}<MessageMetadata
      footer={feedback}
    />{/if}
</div>

<style>
  .store-message-reactions {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.35rem;
  }
</style>
