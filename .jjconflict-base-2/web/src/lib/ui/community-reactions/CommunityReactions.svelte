<script lang="ts">
  import { ReactionBar } from "@lapismd/design-core/ai/experimental";
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

{#if mode !== "picker"}<ReactionBar
    {...barProps}
    reactions={reactions.map((reaction) => ({
      ...reaction,
      label: `${reaction.count} ${reaction.count === 1 ? "reaction" : "reactions"} with ${reaction.label ?? reaction.emoji}${reaction.actors?.length ? `: ${reaction.actors.map((actor) => actor.name).join(", ")}` : ""}`,
    }))}
    {label}
  />{/if}
{#if mode !== "bar" && onSelect}<CommunityEmojiPicker
    label={pickerLabel}
    {disabled}
    {presets}
    {presetsOnly}
    {communityEmojis}
    {onSelect}
  />{/if}
