<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Chat from "@lapismd/design-core/ai/chat";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityMessageMeta from "../community-message/CommunityMessageMeta.svelte";
  import { agentMessageTimestamp } from "../../collaboration/presentation.js";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  let {
    role,
    text,
    author,
    children,
    observedAtMillis,
    messageId,
  }: {
    role: "user" | "assistant";
    text: string;
    author: CommunityMemberModel;
    children?: Snippet;
    observedAtMillis?: number;
    messageId: string;
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
  {#snippet metadata()}<CommunityMessageMeta
      message={{
        id: messageId,
        authorId: author.id,
        content: text,
        ...agentMessageTimestamp(observedAtMillis),
      }}
    />{/snippet}
</Chat.Message>
