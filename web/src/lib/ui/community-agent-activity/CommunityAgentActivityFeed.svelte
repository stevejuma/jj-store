<script lang="ts">
  import {
    Layout,
    LayoutScrollButton,
    MessageList,
  } from "@lapismd/design-core/ai/chat";
  import type { Snippet } from "svelte";
  import "./activity-feed.css";
  import "../community-conversation/conversation.css";
  let {
    conversationKey,
    latestMessageId,
    working = false,
    empty = false,
    children,
    messageComposer,
  }: {
    conversationKey: string;
    latestMessageId?: string;
    working?: boolean;
    empty?: boolean;
    children: Snippet;
    messageComposer: Snippet;
  } = $props();
</script>

<div
  class="agent-activity-feed community-conversation"
  data-composer-placement="row"
>
  <Layout
    density="compact"
    scrollMode="stream"
    {conversationKey}
    aria-label="Agent activity"
    isEmpty={empty}
  >
    {#snippet emptyState()}{@render children()}{/snippet}
    {#snippet scrollButton({
      isVisible,
      hasNewMessages,
      scrollToBottom,
    })}<LayoutScrollButton
        {isVisible}
        {hasNewMessages}
        onClick={scrollToBottom}
      />{/snippet}
    {#snippet composer()}{@render messageComposer()}{/snippet}
    <MessageList {latestMessageId} isStreaming={working}
      >{@render children()}</MessageList
    >
  </Layout>
</div>
