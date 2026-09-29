<script lang="ts">
  import * as Chat from "@lapismd/design-core/ai/chat";
  import { CommunityMessage } from "@lapismd/lapis-community/components/community-message";
  import type { CommunityMemberModel } from "@lapismd/lapis-community/components/community-message";
  import type { CommunityMessageModel } from "@lapismd/lapis-community/components/community-message";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreAgentToolReview from "./StoreAgentToolReview.svelte";

  let {
    activity,
    agent,
    member,
    patches = {},
    controller,
  }: {
    activity: readonly Model<"AgentActivity">[];
    agent: CommunityMemberModel;
    member: CommunityMemberModel;
    patches?: Readonly<Record<string, string>>;
    controller: StoreController;
  } = $props();

  const members = $derived([agent, member]);

  function message(
    entry: Extract<Model<"AgentActivity">, { type: "message" }>,
  ): CommunityMessageModel {
    let content = entry.content.content;
    if (entry.content.media_type === "json") {
      try {
        content = `\`\`\`json\n${JSON.stringify(JSON.parse(content), null, 2)}\n\`\`\``;
      } catch {
        content = `\`\`\`text\n${content}\n\`\`\``;
      }
    }
    const author =
      entry.role === "assistant" || entry.role === "tool" ? agent : member;
    return {
      id: entry.message_id,
      authorId: author.id,
      content,
      timestamp: `#${Number(entry.sequence) + 1}`,
    };
  }
</script>

<Chat.MessageList
  density="compact"
  isEmpty={activity.length === 0}
  aria-label="Agent conversation activity"
>
  {#each activity as entry (`${entry.type}:${entry.sequence}`)}
    {#if entry.type === "message"}
      {@const author =
        entry.role === "assistant" || entry.role === "tool" ? agent : member}
      <CommunityMessage
        message={message(entry)}
        {author}
        {members}
        highlighted={entry.role === "user"}
      />
    {:else if entry.type === "tool"}
      <StoreAgentToolReview tool={entry.tool} {patches} {controller} />
    {:else if entry.type === "capture_gap"}
      <Chat.SystemMessage>
        Capture gap from {entry.missing_from} to {entry.missing_to}
      </Chat.SystemMessage>
    {:else if entry.type === "permission" && entry.state !== "not_observed"}
      <Chat.SystemMessage>Tool permission {entry.state}</Chat.SystemMessage>
    {/if}
  {/each}
</Chat.MessageList>

<style>
  :global([data-ui-component="ai-chat-message-list"]) {
    min-height: 0;
    padding: 0.75rem;
  }

  :global([data-ui-component="ai-chat-tool-calls"]) {
    box-sizing: border-box;
    width: calc(100% - 2.85rem);
    max-width: 42rem;
    margin-inline: 2.6rem 0.25rem;
  }
</style>
