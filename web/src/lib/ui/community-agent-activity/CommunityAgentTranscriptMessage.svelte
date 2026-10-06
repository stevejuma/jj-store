<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Chat from "@lapismd/design-core/ai/chat";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  let {
    role,
    text,
    author,
    children,
  }: {
    role: "user" | "assistant";
    text: string;
    author: CommunityMemberModel;
    children?: Snippet;
  } = $props();
</script>

<Chat.Message sender={role} aria-label={`Message from ${author.name}`}>
  {#snippet avatar()}<CommunityMemberAvatar
      member={author}
      size="small"
      shape={role === "assistant" ? "rounded" : "circle"}
      decorative
    />{/snippet}
  {#snippet name()}<span>{author.name}</span>{/snippet}
  <Chat.MessageBubble
    ><CommunityMarkdownContent
      content={text}
    />{@render children?.()}</Chat.MessageBubble
  >
</Chat.Message>
