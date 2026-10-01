<script lang="ts">
  import { MessageMetadata } from "@lapismd/design-core/ai/chat";
  import { targetKey } from "../../collaboration/thread-presentation.js";
  import CommunityReactions from "../community-reactions/CommunityReactions.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import RotateCwIcon from "@lucide/svelte/icons/rotate-cw";
  import ListCheckIcon from "@lucide/svelte/icons/list-check";
  import CircleXIcon from "@lucide/svelte/icons/circle-x";
  import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
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
    showFeedback = true,
  }: {
    threads: ThreadsController;
    target: ReactionTarget;
    disabled?: boolean;
    showControls?: boolean;
    showFeedback?: boolean;
  } = $props();
  let error = $state("");
  const actionError = $derived(threads.reactionError(target));
  const items = $derived(threads.getReaction(target));
  const failure = $derived(error || actionError);
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
    <Tooltip.Provider delayDuration={0}>
      <span class="store-reaction-feedback">
        {#if pending}
          {#if !threads.busy}
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label="Check action status"
              title="Check action status"
              onclick={threads.checkPending}
              ><ListCheckIcon aria-hidden="true" /></Button
            >
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label="Retry same action"
              title="Retry same action"
              onclick={() =>
                threads.retryPending().catch((e) => (error = String(e)))}
              ><RotateCwIcon aria-hidden="true" /></Button
            >
          {/if}
          <span role="status">
            <Spinner aria-hidden="true" role="presentation" />
            <span class="sr-only">Awaiting reaction confirmation.</span>
          </span>
        {:else if failure}
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="Retry reactions"
            title="Retry reactions"
            onclick={() => {
              error = "";
              void threads
                .loadReactions(target, true)
                .catch((e) => (error = String(e)));
            }}><RotateCwIcon aria-hidden="true" /></Button
          >
          <span data-ui-part="separator" aria-hidden="true">·</span>
          <Tooltip.Root>
            <Tooltip.Trigger aria-label="Reaction failed">
              <span data-ui-part="status" data-status="error" role="alert"
                ><CircleXIcon aria-hidden="true" />Failed</span
              >
            </Tooltip.Trigger>
            <Tooltip.Content>Reaction failed: {failure}</Tooltip.Content>
          </Tooltip.Root>
        {:else if items?.conflict_count}
          <Tooltip.Root>
            <Tooltip.Trigger aria-label="Reaction conflict">
              <span data-ui-part="status" role="status">
                <CircleAlertIcon aria-hidden="true" />Conflict
              </span>
            </Tooltip.Trigger>
            <Tooltip.Content
              >Reaction conflict. Refresh and resolve before changing it.</Tooltip.Content
            >
          </Tooltip.Root>
        {/if}
      </span>
    </Tooltip.Provider>
  {/snippet}
  {#if showFeedback && (pending || items?.conflict_count || error || actionError)}<MessageMetadata
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
  .store-reaction-feedback {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
  }
</style>
