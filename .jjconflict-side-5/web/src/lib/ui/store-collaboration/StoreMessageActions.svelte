<script lang="ts">
  // @community-copy CommunityMessageActions (Apache-2.0), native comment callbacks.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import MessageSquareReplyIcon from "@lucide/svelte/icons/message-square-reply";
  import CommunityEmojiPicker from "../community-emoji-picker/CommunityEmojiPicker.svelte";
  import type { CommunityCustomEmoji } from "../../community/contracts.js";
  import StoreRecordMenu from "./StoreRecordMenu.svelte";
  let {
    disabled = false,
    authorName,
    items,
    onReact,
    communityEmojis = [],
    reactionDisabled = disabled,
    onReply,
    onEdit,
    onRemove,
  }: {
    disabled?: boolean;
    authorName: string;
    items: readonly { label: string; value: string | null | undefined }[];
    onReact?:
      ((emoji: string, custom?: CommunityCustomEmoji) => void) | undefined;
    communityEmojis?: readonly CommunityCustomEmoji[];
    reactionDisabled?: boolean;
    onReply?: (() => void) | undefined;
    onEdit?: (() => void) | undefined;
    onRemove?: (() => void) | undefined;
  } = $props();
  let menuOpen = $state(false);
  let reactionOpen = $state(false);
</script>

<div
  class="community-message__actions"
  role="group"
  aria-label={`Actions for ${authorName}'s message`}
  data-ui-part="message-actions"
  data-menu-open={menuOpen || reactionOpen || undefined}
>
  {#if onReact}<CommunityEmojiPicker
      bind:open={reactionOpen}
      label="Add reaction"
      disabled={reactionDisabled}
      {communityEmojis}
      onSelect={onReact}
    />{/if}
  {#if onReply}<Button
      type="button"
      variant="ghost"
      size="icon-sm"
      {disabled}
      aria-label="Reply"
      title="Reply"
      onclick={onReply}><MessageSquareReplyIcon aria-hidden="true" /></Button
    >{/if}
  <StoreRecordMenu
    {items}
    {disabled}
    {onEdit}
    {onRemove}
    onOpenChange={(open) => (menuOpen = open)}
  />
</div>

<style>
  .community-message__actions {
    position: absolute;
    z-index: 3;
    inset-block-start: 0;
    inset-inline-end: 0.5rem;
    display: flex;
    align-items: center;
    gap: 0.125rem;
    padding: 0.125rem;
    border: 1px solid var(--border);
    border-radius: 999px;
    opacity: 0;
    background: var(--popover, var(--background));
    box-shadow: var(--shadow-sm);
    pointer-events: none;
    transition:
      opacity 120ms ease,
      transform 120ms ease;
    transform: translateY(calc(-100% + 0.15rem));
  }
  .community-message__actions[data-menu-open],
  :global(.community-message[data-actions-visible])
    .community-message__actions {
    opacity: 1;
    pointer-events: auto;
    transform: translateY(calc(-100% + 0.3rem));
  }
  .community-message__actions :global([data-ui-component="button"]) {
    border-radius: inherit;
  }
  .community-message__actions :global([data-ui-component="button"]:hover),
  .community-message__actions
    :global([data-ui-component="button"]:focus-visible) {
    background: var(--muted);
    box-shadow: inset 0 0 0 1px
      color-mix(in srgb, var(--foreground) 10%, transparent);
  }
  @media (hover: none) {
    .community-message__actions {
      position: relative;
      inset: auto;
      width: fit-content;
      margin-inline-start: auto;
      opacity: 1;
      pointer-events: auto;
      transform: none;
    }
  }
</style>
