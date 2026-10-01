<script lang="ts">
  import { useAppearance } from "../../appearance/store.svelte.js";
  const appearance = useAppearance();
  import * as Chat from "@lapismd/design-core/ai/chat";
  import CommunityMessage from "../community-message/CommunityMessage.svelte";
  import type { CommunityMemberModel } from "../../community/workspace-model.js";
  import type { CommunityMessageModel } from "../../community/workspace-model.js";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreMessageReactions from "../store-threads/StoreMessageReactions.svelte";
  import StoreAgentToolReview from "./StoreAgentToolReview.svelte";
  import StoreMessageActions from "./StoreMessageActions.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";

  let {
    activity,
    agent,
    member,
    patches = {},
    controller,
    interactive = true,
    sessionId,
  }: {
    activity: readonly Model<"AgentActivity">[];
    agent: CommunityMemberModel;
    member: CommunityMemberModel;
    patches?: Readonly<Record<string, string>>;
    controller: StoreController;
    interactive?: boolean;
    sessionId: string;
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
  density={appearance?.density ?? "compact"}
  isEmpty={activity.length === 0}
  aria-label="Agent conversation activity"
>
  <CommunityProjectsVirtualRows
    rows={activity}
    getKey={(entry) => `${entry.type}:${entry.sequence}`}
    estimateSize={(entry) => (entry?.type === "tool" ? 180 : 118)}
  >
    {#snippet row(entry)}
      {#if entry.type === "message"}
        {@const author =
          entry.role === "assistant" || entry.role === "tool" ? agent : member}
        <CommunityMessage
          message={message(entry)}
          {author}
          {members}
          highlighted={entry.role === "user"}
          >{#snippet supplement()}{#if controller.threads && entry.reaction_target}<StoreMessageReactions
                threads={controller.threads}
                target={entry.reaction_target}
                showFeedback={false}
                disabled={!interactive ||
                  !["write", "admin"].includes(
                    controller.state.repository.permission ?? "",
                  )}
              />{/if}<StoreMessageActions
              authorName={author.name}
              items={[
                { label: "Copy message", value: entry.content.content },
                { label: "Copy message ID", value: entry.message_id },
                { label: "Copy turn ID", value: entry.turn_id },
                { label: "Copy session ID", value: sessionId },
                { label: "Copy sequence", value: String(entry.sequence) },
              ]}
            />{/snippet}
          {#snippet metadataSupplement()}{#if controller.threads && entry.reaction_target}<StoreMessageReactions
                threads={controller.threads}
                target={entry.reaction_target}
                showControls={false}
              />{/if}{/snippet}</CommunityMessage
        >
      {:else if entry.type === "tool"}
        <StoreAgentToolReview
          tool={entry.tool}
          {patches}
          {controller}
          interactive={interactive &&
            controller.state.repository.profile !== "git_sha1_jj045"}
        />
      {:else if entry.type === "capture_gap"}
        <Chat.SystemMessage>
          Capture gap from {entry.missing_from} to {entry.missing_to}
        </Chat.SystemMessage>
      {:else if entry.type === "permission" && entry.state !== "not_observed"}
        <Chat.SystemMessage>Tool permission {entry.state}</Chat.SystemMessage>
      {/if}
    {/snippet}
  </CommunityProjectsVirtualRows>
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
