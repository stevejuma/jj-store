<script lang="ts">
  // CommunityMessageActions (Apache-2.0), native permission and tombstone actions.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import EllipsisIcon from "@lucide/svelte/icons/ellipsis";
  import MessageSquareReplyIcon from "@lucide/svelte/icons/message-square-reply";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import Trash2Icon from "@lucide/svelte/icons/trash-2";
  let {
    disabled,
    removed,
    authorName,
    onReply,
    onEdit,
    onRemove,
  }: {
    disabled: boolean;
    removed: boolean;
    authorName: string;
    onReply: () => void;
    onEdit: () => void;
    onRemove: () => void;
  } = $props();
</script>

<div
  class="community-message__actions"
  role="group"
  aria-label={`Actions for ${authorName}'s message`}
  data-ui-part="message-actions"
>
  <Button
    type="button"
    variant="ghost"
    size="icon-sm"
    {disabled}
    aria-label="Reply"
    title="Reply"
    onclick={onReply}><MessageSquareReplyIcon aria-hidden="true" /></Button
  >
  {#if !removed}<DropdownMenu.Root>
      <DropdownMenu.Trigger
        >{#snippet child({ props })}<Button
            {...props}
            type="button"
            variant="ghost"
            size="icon-sm"
            {disabled}
            aria-label="More message actions"
            title="More message actions"
            ><EllipsisIcon aria-hidden="true" /></Button
          >{/snippet}</DropdownMenu.Trigger
      >
      <DropdownMenu.Content
        class="community-message__menu-content"
        style="min-width: 13.5rem;"
        preventScroll={false}
        align="end"
        sideOffset={4}
      >
        <DropdownMenu.Group
          ><DropdownMenu.Item onSelect={onEdit}
            ><PencilIcon data-icon="inline-start" aria-hidden="true" />Edit
            comment</DropdownMenu.Item
          ></DropdownMenu.Group
        >
        <DropdownMenu.Separator />
        <DropdownMenu.Group
          ><DropdownMenu.Item variant="destructive" onSelect={onRemove}
            ><Trash2Icon data-icon="inline-start" aria-hidden="true" />Remove
            comment</DropdownMenu.Item
          ></DropdownMenu.Group
        >
      </DropdownMenu.Content>
    </DropdownMenu.Root>{/if}
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
