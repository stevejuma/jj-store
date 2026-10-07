<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { MiraMarkdownActionId } from "@lapismd/mira";
  import CommunityComposerMentionButton from "../community-composer-mention-button/CommunityComposerMentionButton.svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import TypeIcon from "@lucide/svelte/icons/type";
  import Bold from "@lucide/svelte/icons/bold";
  import Italic from "@lucide/svelte/icons/italic";
  import Strikethrough from "@lucide/svelte/icons/strikethrough";
  import Code from "@lucide/svelte/icons/code";
  import List from "@lucide/svelte/icons/list";
  import ListOrdered from "@lucide/svelte/icons/list-ordered";
  import Quote from "@lucide/svelte/icons/quote";
  import X from "@lucide/svelte/icons/x";
  import CommunityEmojiPicker from "../community-emoji-picker/CommunityEmojiPicker.svelte";
  import type { CommunityCustomEmoji } from "../../community/contracts.js";
  let {
    formattingOpen = $bindable(false),
    disabled,
    communityEmojis,
    onAction,
    onInsert,
    onEmoji,
    mentionMember,
    onDismissMention,
  }: {
    formattingOpen?: boolean;
    disabled: boolean;
    communityEmojis: readonly CommunityCustomEmoji[];
    onAction: (action: MiraMarkdownActionId) => void;
    onInsert: (text: string) => void;
    onEmoji: (emoji: string, custom?: CommunityCustomEmoji) => void;
    mentionMember?: CommunityMemberModel;
    onDismissMention?: (id: string) => void;
  } = $props();
  const actions = [
    { id: "bold", label: "Bold", icon: Bold },
    { id: "italic", label: "Italic", icon: Italic },
    { id: "strikethrough", label: "Strikethrough", icon: Strikethrough },
    { id: "inlineCode", label: "Inline code", icon: Code },
    { id: "bulletList", label: "Bulleted list", icon: List },
    { id: "numberedList", label: "Numbered list", icon: ListOrdered },
    { id: "quote", label: "Quote", icon: Quote },
  ] as const;
</script>

<div
  class="community-chat-composer__toolbar-main"
  role="toolbar"
  aria-label="Message formatting and actions"
  data-ui-part="composer-toolbar"
>
  {#if formattingOpen}
    <Button
      type="button"
      variant="secondary"
      size="icon-sm"
      aria-label="Formatting options"
      aria-pressed="true"
      onclick={() => (formattingOpen = false)}
      ><TypeIcon aria-hidden="true" /></Button
    >
    <Button
      type="button"
      variant="ghost"
      size="icon-sm"
      aria-label="Hide formatting"
      onclick={() => (formattingOpen = false)}><X aria-hidden="true" /></Button
    >
    <span class="community-chat-composer__separator" aria-hidden="true"></span>
    {#each actions as action}<Button
        type="button"
        variant="ghost"
        size="icon-sm"
        aria-label={action.label}
        {disabled}
        onclick={() => onAction(action.id)}
        ><action.icon aria-hidden="true" /></Button
      >{/each}
  {:else}
    <CommunityComposerMentionButton
      member={mentionMember}
      {disabled}
      onclick={() => onInsert("@")}
      onDismiss={onDismissMention}
    />
    <CommunityEmojiPicker
      label="Add emoji"
      {disabled}
      {communityEmojis}
      onSelect={onEmoji}
    />
    <Button
      type="button"
      variant="ghost"
      size="icon-sm"
      aria-label="Show formatting"
      aria-pressed="false"
      {disabled}
      onclick={() => (formattingOpen = true)}
      ><TypeIcon aria-hidden="true" /></Button
    >
  {/if}
</div>
