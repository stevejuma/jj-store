import type { Model } from "../../api/client.js";
import type {
  CommunityMemberModel,
  CommunityMessageModel,
} from "../../community/workspace-model.js";
import { promptTags } from "./prompt-element-ranges.js";
import { markdownBlocks } from "../store-agent-chat-body/markdown-blocks.js";
import { recordedConversationProjection } from "../../conversations/recorded-adapter.js";
import { agentMessageTimestamp } from "../../collaboration/presentation.js";
import { agentMessageContent } from "../../collaboration/agent-message-content.js";

export function readOnlyTranscriptItems(
  activity: readonly Model<"AgentActivity">[],
  sessionId: string,
  integration: Model<"IntegrationId">,
  agent: CommunityMemberModel,
  member: CommunityMemberModel,
  legacy: readonly Model<"AgentTranscriptChunk">[],
) {
  return activity.length
    ? recordedConversationProjection(
        activity,
        sessionId,
        integration,
        agent,
        member,
      ).items.map((item) => {
        const entry = activity.find(
          (e) =>
            e.type === "message" && `${sessionId}:${e.message_id}` === item.id,
        );
        if (
          entry?.type === "message" &&
          entry.content.media_type === "text" &&
          promptTags(entry.content.content).length
        )
          return {
            ...item,
            blocks: readOnlyBlocks(
              item.id,
              item.blocks[0]?.revision ?? "captured",
              item.label,
              agentMessageContent(entry.content, integration),
            ),
          };
        return item;
      })
    : legacy.map((chunk) => {
        const id = `${sessionId}:legacy:${chunk.sequence}`;
        return {
          id,
          kind: "message" as const,
          role: "assistant" as const,
          author: agent,
          label: "Assistant message",
          timestamp: agentMessageTimestamp(chunk.occurred_at_millis),
          blocks: readOnlyBlocks(
            id,
            String(chunk.sequence),
            "Assistant message",
            agentMessageContent(chunk, integration),
          ),
        };
      });
}
function readOnlyBlocks(
  id: string,
  revision: string,
  label: string,
  text: string,
) {
  // The established inert prompt renderer needs the complete source to retain
  // nested elements and document-wide Markdown references. No writable source
  // is manufactured for this capability-limited reader.
  return promptTags(text).length
    ? [
        {
          id: `${id}/context`,
          revision,
          label,
          text,
          kind: "context" as const,
          commentable: false,
        },
      ]
    : markdownBlocks(id, revision, label, text).map((block) => ({
        ...block,
        commentable: false,
      }));
}

export function transcriptMessage(
  entry: Extract<Model<"AgentActivity">, { type: "message" }>,
  integration: Model<"IntegrationId">,
  agent: CommunityMemberModel,
  member: CommunityMemberModel,
  threads:
    import("../../controller/thread-types.js").ThreadsController | undefined,
): CommunityMessageModel {
  const author =
    entry.role === "assistant" || entry.role === "tool" ? agent : member;
  return {
    id: entry.message_id,
    authorId: author.id,
    content: agentMessageContent(entry.content, integration),
    ...agentMessageTimestamp(entry.occurred_at_millis),
    reactions: entry.reaction_target
      ? threads?.reactionModels(entry.reaction_target)
      : [],
  };
}
