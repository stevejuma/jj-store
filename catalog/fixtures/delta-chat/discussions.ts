import type {
  ChatBodyItem,
  ChatCommentThread,
  ChatComment,
} from "../../../web/src/lib/ui/store-agent-chat-body/model.js";
import { validAnchor } from "../../../web/src/lib/ui/store-agent-chat-body/anchors.js";
import { utf8ToUtf16 } from "./offsets.js";
import type { DeltaChatFixture } from "./model.js";
import { demoSender, demoAgent } from "./next-message.svelte.js";
export function deltaChatThreads(
  fixture: DeltaChatFixture,
  items: readonly ChatBodyItem[],
): ChatCommentThread[] {
  const blocks = items.flatMap((item) => [
    ...item.blocks,
    ...(item.tools ?? []).flatMap((tool) => tool.blocks),
  ]);
  return (fixture.discussions ?? []).map((discussion) => {
    const source = fixture.records.find(
      (r) => r.id === discussion.target.recordId,
    );
    if (!source) throw new Error("Comment source record unavailable");
    const text =
      source.kind === "ToolUse" ? (source.output ?? "") : (source.text ?? "");
    const prefix = `${source.id}/${source.kind === "ToolUse" ? "output" : "text"}`;
    const from = utf8ToUtf16(text, discussion.target.from),
      to = utf8ToUtf16(text, discussion.target.to);
    const block = blocks.find(
      (b) =>
        b.id.startsWith(prefix) &&
        from >= (b.sourceFrom ?? 0) &&
        to <= (b.sourceFrom ?? 0) + b.text.length,
    );
    if (!block)
      throw new Error("Comment range crosses or misses a rendered block");
    const anchor = {
      blockId: block.id,
      revision: block.revision,
      from: from - (block.sourceFrom ?? 0),
      to: to - (block.sourceFrom ?? 0),
      quote: discussion.target.quote,
    };
    if (!validAnchor(block, anchor))
      throw new Error(
        "Reviewed comment quote does not match the captured source",
      );
    const comments = discussion.comments.map((c) => {
      if (!["draft", "sent", "replied"].includes(c.status))
        throw new Error("Unsupported captured comment status");
      return {
        ...c,
        author: c.author === "user" ? demoSender : demoAgent,
        status: c.status as ChatComment["status"],
      };
    });
    return {
      id: discussion.id,
      label: block.label,
      sourceKind: block.kind,
      anchor,
      comments,
    };
  });
}
