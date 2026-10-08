<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import MessageCircleIcon from "@lucide/svelte/icons/message-circle";
  import CommunityReactions from "../community-reactions/CommunityReactions.svelte";
  import type { ChatBodyBlock } from "./model.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  let {
    block,
    state,
    onComment,
    reactionTarget,
  }: {
    block: ChatBodyBlock;
    state: ChatBodyComments;
    onComment: () => void;
    reactionTarget?: string;
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
    onclick={onComment}><MessageCircleIcon size={14} /> Comment</Button
  >
</div>
