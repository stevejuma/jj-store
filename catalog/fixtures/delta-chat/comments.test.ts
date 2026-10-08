import { expect, test } from "vitest";
import { ChatBodyComments } from "../../../web/src/lib/ui/store-agent-chat-body/comments.svelte.js";
import type {
  ChatBodyItem,
  ChatPromptSubmission,
} from "../../../web/src/lib/ui/store-agent-chat-body/model.js";
import { commentSources } from "../../../web/src/lib/ui/store-agent-chat-body/comment-navigation.js";
import {
  DemoNextMessage,
  demoSender,
  demoAgent,
} from "./next-message.svelte.js";
const block = {
  id: "prose",
  revision: "1",
  label: "Explanation",
  kind: "markdown" as const,
  text: "First café 👩🏽‍💻.\n\nNext paragraph.",
};
const items: ChatBodyItem[] = [
  { id: "m", kind: "message", label: "Message", blocks: [block] },
];
const caret = { from: 0, to: 0 };
test("one author's exact anchor resumes a draft; later messages retain sent history", async () => {
  const submissions: ChatPromptSubmission[] = [];
  const state = new ChatBodyComments(() => ({
    items,
    author: demoSender,
    onSend: (s) => {
      submissions.push(s);
    },
  }));
  state.open(block, caret, "First");
  const id = state.active!.commentId,
    threadId = state.active!.threadId;
  state.active = undefined;
  state.open(block, caret, " continued");
  expect(state.pending).toHaveLength(1);
  expect(state.active!.commentId).toBe(id);
  expect(state.draft).toBe("First continued");
  state.message = "Follow up";
  await state.send();
  expect(submissions[0]).toMatchObject({
    text: "Follow up",
    comments: [
      {
        commentId: id,
        text: "First continued",
        anchor: { quote: "First café 👩🏽‍💻." },
      },
    ],
  });
  expect(state.findComment(id)).toMatchObject({
    status: "sent",
    batchId: submissions[0].id,
  });
  state.open(block, caret, "Second");
  expect(state.threads).toHaveLength(1);
  expect(state.active!.threadId).toBe(threadId);
  expect(state.active!.commentId).not.toBe(id);
  expect(state.findComment(id)?.text).toBe("First continued");
});
test("comment-only sends freeze snapshots, prevent double submission and retain failures", async () => {
  let resolve!: () => void, reject!: (e: Error) => void;
  let calls = 0;
  const state = new ChatBodyComments(() => ({
    items,
    onSend: () => {
      calls++;
      return new Promise<void>((a, b) => {
        resolve = a;
        reject = b;
      });
    },
  }));
  state.open(block, caret, "Exact\ncomment");
  const id = state.active!.commentId;
  const sending = state.send();
  expect(state.pending[0].comment.status).toBe("sending");
  state.edit(id, "Cannot change an in-flight snapshot");
  expect(state.findComment(id)?.text).toBe("Exact\ncomment");
  expect(await state.send()).toBe(false);
  expect(calls).toBe(1);
  reject(new Error("Offline"));
  expect(await sending).toBe(false);
  expect(state.error).toBe("Offline");
  expect(state.findComment(id)?.status).toBe("draft");
  const retry = state.send();
  resolve();
  expect(await retry).toBe(true);
  expect(state.pending).toHaveLength(0);
  expect(state.message).toBe("");
});
test("stable author identity, exact anchors, removal and unavailable revisions", () => {
  let author = demoSender;
  let current = items;
  const state = new ChatBodyComments(() => ({ items: current, author }));
  state.open(block, caret, "First");
  author = { ...demoSender, id: "another", name: "You" };
  state.open(block, caret, "Another author");
  expect(state.threads).toHaveLength(1);
  expect(state.pending).toHaveLength(2);
  state.open(block, { from: 25, to: 25 }, "Other paragraph");
  expect(state.threads).toHaveLength(2);
  state.active = undefined;
  state.remove(state.pending[2].comment.id);
  expect(state.threads).toHaveLength(1);
  current = [{ ...items[0], blocks: [{ ...block, revision: "2" }] }];
  expect(state.isAvailable(state.threads[0])).toBe(false);
  expect(state.pending).toHaveLength(2);
});
test("only a linked reply changes sent to replied; repeated delivery is idempotent", async () => {
  const state = new ChatBodyComments(() => ({ items, onSend: () => {} }));
  state.open(block, caret, "Question");
  const { thread, comment } = state.pending[0];
  const reply = {
    id: "r",
    author: demoAgent,
    text: "Answer",
    status: "sent" as const,
  };
  state.recordReply(thread.id, comment.id, reply, "reply-item");
  expect(thread.comments).toHaveLength(1);
  await state.send();
  state.recordReply(thread.id, comment.id, reply, "reply-item");
  state.recordReply(thread.id, comment.id, reply, "reply-item");
  expect(comment.status).toBe("replied");
  expect(comment.replyItemId).toBe("reply-item");
  expect(thread.comments).toHaveLength(2);
});
test("nested tool/source paths explicitly include group, file and raw alternatives", () => {
  const tool = {
    id: "tool",
    title: "Read",
    revision: "1",
    status: "complete" as const,
    blocks: [block],
    raw: { a: 1 },
    detailPlacement: "sidebar" as const,
  };
  const sources = commentSources([
    { id: "group", kind: "tools", label: "Tools", blocks: [], tools: [tool] },
  ]);
  expect(sources[0]).toMatchObject({
    disclosures: ["group", "tool", "file:prose"],
    raw: false,
    tool,
  });
  expect(sources[1].raw).toBe(true);
});
test("explicit new comments share an anchor while ordinary editing resumes the latest draft", () => {
  const state = new ChatBodyComments(() => ({ items }));
  state.open(block, caret, "First comment");
  const thread = state.threads[0];
  state.begin(thread, "inline", "Second comment", true);
  expect(thread.comments.map((c) => c.text)).toEqual([
    "First comment",
    "Second comment",
  ]);
  state.open(block, caret);
  expect(state.active?.commentId).toBe(thread.comments[1].id);
});

test("one submitted batch groups same-anchor comments into linked sections within one turn", async () => {
  const demo = new DemoNextMessage();
  const state = new ChatBodyComments(() => ({
    items: [...items, ...demo.messages],
    onSend: (submission) => demo.accept(submission),
  }));
  state.open(block, caret, "First question");
  state.begin(state.threads[0], "inline", "Second question", true);
  state.open(block, { from: 25, to: 25 }, "Another anchor");
  await state.send();
  demo.completeReplies(state);
  const replies = demo.messages.filter((item) => item.replyContext);
  expect(replies).toHaveLength(2);
  expect(new Set(demo.messages.map((item) => item.turn?.id)).size).toBe(1);
  expect(replies.map((item) => item.replyContext?.commentIds.length)).toEqual([
    2, 1,
  ]);
  for (const reply of replies) {
    for (const id of reply.replyContext!.commentIds)
      expect(state.findComment(id)).toMatchObject({
        status: "replied",
        replyItemId: reply.id,
      });
  }
  expect(state.threads[0].comments.filter((c) => c.replyTo)).toHaveLength(1);
  demo.completeReplies(state);
  expect(demo.messages).toHaveLength(3);
  state.open(block, caret, "Later follow up");
  await state.send();
  demo.completeReplies(state);
  expect(demo.messages.filter((item) => item.replyContext)).toHaveLength(3);
  expect(new Set(demo.messages.map((item) => item.turn?.id)).size).toBe(2);
  expect(state.threads[0].comments[0].replyItemId).toBe(replies[0].id);
  expect(demo.messages.at(-1)?.replyContext?.commentIds).toHaveLength(1);
});
