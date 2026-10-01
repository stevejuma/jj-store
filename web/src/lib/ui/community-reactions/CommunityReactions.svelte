<script lang="ts">
  import { ReactionBar } from "@lapismd/design-core/ai/experimental";
  import CommunityReactionActors from "./CommunityReactionActors.svelte";
  import type { CommunityReactionModel } from "../../community/workspace-model.js";
  import type { ComponentProps } from "svelte";
  import type { CommunityCustomEmoji } from "../../community/contracts.js";
  import CommunityEmojiPicker from "../community-emoji-picker/CommunityEmojiPicker.svelte";
  let {
    mode = "all",
    reactions = [],
    label = "Reactions",
    pickerLabel = "Add reaction",
    disabled = false,
    presets = [],
    presetsOnly = false,
    communityEmojis = [],
    onSelect,
    ...barProps
  }: Omit<ComponentProps<typeof ReactionBar>, "reactions"> & {
    reactions?: CommunityReactionModel[];
    mode?: "all" | "bar" | "picker";
    pickerLabel?: string;
    disabled?: boolean;
    presets?: readonly string[];
    presetsOnly?: boolean;
    communityEmojis?: readonly CommunityCustomEmoji[];
    onSelect?: (emoji: string, custom?: CommunityCustomEmoji) => void;
  } = $props();
</script>

{#if mode !== "picker"}
  {#if reactions.some((r) => r.actors?.length)}
    <div class="community-reactions" role="group" aria-label={label}>
      {#each reactions as reaction (reaction.emoji)}
        {#if reaction.actors?.length}<CommunityReactionActors
            {reaction}
            onToggle={barProps.onToggle}
          />
        {:else}<ReactionBar {...barProps} reactions={[reaction]} {label} />{/if}
      {/each}
    </div>
  {:else}<ReactionBar {...barProps} {reactions} {label} />{/if}
{/if}
{#if mode !== "bar" && onSelect}<CommunityEmojiPicker
    label={pickerLabel}
    {disabled}
    {presets}
    {presetsOnly}
    {communityEmojis}
    {onSelect}
  />{/if}

<style>
  .community-reactions {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.375rem;
  }
</style>
