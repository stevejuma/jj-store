import type { ChatBodyItem } from "../../../web/src/lib/ui/store-agent-chat-body/model.js";
import type { DeltaChatFixture } from "./model.js";
export function assignDeltaTurns(
  items: ChatBodyItem[],
  fixture: DeltaChatFixture,
) {
  const users = items.filter((item) => item.role === "user");
  for (let index = 0; index < users.length; index++) {
    const user = users[index];
    const start = items.indexOf(user);
    const end = users[index + 1]
      ? items.indexOf(users[index + 1])
      : items.length;
    const members = items.slice(start, end);
    const record = fixture.records.find((r) => r.id === user.id)!;
    const nextRecord = fixture.records.find(
      (r) => r.id === users[index + 1]?.id,
    );
    const turn = fixture.records.find(
      (r) =>
        r.kind === "AssistantTurn" &&
        r.start >= record.start &&
        r.start < (nextRecord?.start ?? Infinity),
    );
    const lastMessage = members
      .filter((item) => item.role === "assistant")
      .at(-1);
    const summary = (
      lastMessage?.blocks[0]?.text.split(/\n\s*\n/)[0] ?? fixture.source.title
    )
      .replace(/[*`_]/g, "")
      .replace(/\s+/g, " ")
      .slice(0, 280);
    const commentCount =
      fixture.discussions
        ?.flatMap((d) => d.comments)
        .filter((c) => c.batchId === user.commentBatchId).length ?? 0;
    for (const item of members)
      item.turn = {
        id: turn?.id ?? user.id,
        author: "You",
        agentName: "Codex",
        summary,
        prompt:
          user.blocks.map((b) => b.text).join("\n\n") ||
          `${commentCount} ${commentCount === 1 ? "comment" : "comments"}`,
        createdAt: record.createdAt,
      };
  }
  return items;
}
