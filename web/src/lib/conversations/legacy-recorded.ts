import type { Model } from "../api/client.js";
import type { CommunityMemberModel } from "../community/workspace-model.js";
import type { ChatBodyItem } from "../ui/store-agent-chat-body/model.js";
import { markdownBlocks } from "../ui/store-agent-chat-body/markdown-blocks.js";

export function legacyRecordedItems(
  sessionId: string,
  legacy: readonly Model<"AgentTranscriptChunk">[],
  agent: CommunityMemberModel,
): ChatBodyItem[] {
  return legacy.map((chunk) => {
    const id = `${sessionId}:legacy:${chunk.sequence}`;
    return {
      id,
      kind: "message",
      role: "assistant",
      author: agent,
      label: "Assistant message",
      blocks: markdownBlocks(
        id,
        String(chunk.sequence),
        "Assistant message",
        chunk.content,
      ).map((block) => ({ ...block, commentable: false })),
      turn: {
        id,
        author: agent.name,
        prompt: "Recorded output",
        summary: chunk.content.slice(0, 240),
      },
    };
  });
}
