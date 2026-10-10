<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import MessageCircleIcon from "@lucide/svelte/icons/message-circle";
  import CommunityReactions from "../community-reactions/CommunityReactions.svelte";
  import type { ChatBodyBlock, ChatBodyProps } from "./model.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  let {
    block,
    state,
    onComment,
    reactionTarget,
    messageActions,
  }: {
    block: ChatBodyBlock;
    state: ChatBodyComments;
    onComment: () => void;
    reactionTarget?: string;
    messageActions?: ChatBodyProps["messageActions"];
  } = $props();
</script>

<div
  class="chat-text-actions"
  role="group"
  aria-label={`Actions for ${block.label}`}
>
  {#if reactionTarget && state.props().onReact}
    <CommunityReactions
      mode="picker"
      communityEmojis={state.props().communityEmojis ?? []}
      pickerLabel={`Add reaction to ${block.label}`}
      presets={state.props().reactionOptions ?? []}
      onSelect={(emoji, custom) =>
        state.props().onReact?.(reactionTarget!, emoji, custom)}
    />
  {/if}
  <Button
    variant="ghost"
    size="sm"
    aria-label={`Comment on ${block.label}`}
    disabled={block.commentable === false || state.props().canComment === false}
    title={state.props().canComment === false
      ? (state.props().commentUnavailable ?? "Commenting is unavailable")
      : block.commentable === false
        ? "Waiting for an exact published source"
        : undefined}
    onclick={onComment}><MessageCircleIcon size={14} /> Comment</Button
  >
  {#if reactionTarget}{@render messageActions?.(reactionTarget)}{/if}
</div>
