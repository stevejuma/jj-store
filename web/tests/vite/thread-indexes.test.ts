import { expect, test } from "vitest";
import { channelMessages } from "../../src/lib/collaboration/thread-presentation.js";
import { threadDescendants } from "../../src/lib/ui/store-threads/thread-view.js";
import {
  createThreadTransport,
  THREAD_IDS as ids,
} from "../../../catalog/fixtures/thread-transport.js";
import type { CommunityMessageModel } from "../../src/lib/community/workspace-model.js";

test("large loaded reply trees retain order and participants in indexed passes", () => {
  const f = createThreadTransport();
  const root = f.history.get(ids.thread)![0]!.value.event;
  if (root.type !== "message") throw Error("root revision");
  for (let n = 0; n < 1200; n++)
    f.addMessage(
      ids.thread,
      n.toString(16).padStart(32, "0"),
      `Reply ${n}`,
      root.revision,
    );
  const projection = channelMessages(f.history.get(ids.thread)!);
  const parent = projection.messages.find((m) => m.id === ids.message)!;
  expect(parent.replyCount).toBe(1200);
  expect(parent.replyParticipantIds).toEqual([ids.actor]);
  const reversed = [...projection.messages].reverse();
  const descendants = threadDescendants(reversed, ids.message);
  expect(descendants.map((m) => m.id)).toEqual(reversed.map((m) => m.id));
});

test("deep replies, unloaded ancestors and cycles have bounded traversal", () => {
  const messages: CommunityMessageModel[] = Array.from(
    { length: 5000 },
    (_, n) => ({
      id: String(n),
      authorId: "actor",
      content: "body",
      createdAt: "2026-10-02",
      parentMessageId: n ? String(n - 1) : "unloaded:original",
    }),
  ).reverse();
  messages.push({
    id: "other",
    authorId: "actor",
    content: "Other",
    createdAt: "2026-10-02",
  });
  const descendants = threadDescendants(messages, "0");
  expect(descendants).toHaveLength(5000);
  expect(descendants[0]!.id).toBe("4999");
  messages.find((m) => m.id === "0")!.parentMessageId = "4999";
  expect(threadDescendants(messages, "0")).toHaveLength(5000);
});
