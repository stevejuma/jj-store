import { test, expect } from "vitest";
import { ConversationSessions } from "../../src/lib/conversations/session.svelte.js";
import { exactSource } from "../../src/lib/conversations/source-map.js";
import type { ConversationProjection } from "../../src/lib/conversations/agent-adapter.js";
const user = { id: "1".repeat(32), name: "You", initials: "YO", handle: "" };
const target = {
  type: "human_message" as const,
  thread_id: "2".repeat(32),
  message_id: "3".repeat(32),
  revision: { kind: "human_message_revision" as const, id: "4".repeat(64) },
};
const block = {
  id: "m",
  revision: target.revision.id,
  label: "Message",
  kind: "markdown" as const,
  text: "Hello 🦀",
};
const source = {
  target,
  part: null,
  text: block.text,
  from: 0,
  revision: block.revision,
};
const projection = (): ConversationProjection => ({
  items: [
    {
      id: "m",
      kind: "message",
      role: "user",
      label: "Message",
      blocks: [block],
    },
  ],
  sources: new Map([[block.id, source]]),
  threads: [],
  targets: new Map(),
});
test("session drafts survive remounts and exact publication reconciles without duplicates", async () => {
  const sessions = new ConversationSessions(),
    session = sessions.get("repo:actor:thread");
  session.adopt(projection());
  session.host = {
    author: user,
    onSend: async (submission) => {
      const incoming = projection();
      const draft = submission.comments[0];
      incoming.threads = [
        {
          id: draft.threadId,
          label: "Message",
          anchor: draft.anchor,
          source: exactSource(block, draft.anchor, source),
          comments: [
            {
              id: `${draft.commentId}:revision`,
              messageId: draft.commentId,
              author: user,
              text: draft.text,
              revision: { kind: "human_message_revision", id: "5".repeat(64) },
              status: "sent",
            },
          ],
        },
      ];
      session.adopt(incoming);
      return { publication: "published", batchId: "receipt" };
    },
  };
  session.comments.open(block, { from: 0, to: 5 }, "Question");
  const id = session.comments.active!.commentId;
  expect(sessions.get("repo:actor:thread")).toBe(session);
  expect(sessions.get("other:actor:thread").comments.pending).toHaveLength(0);
  expect(await session.comments.send()).toBe(true);
  expect(session.comments.threads).toHaveLength(1);
  expect(session.comments.threads[0].comments).toHaveLength(1);
  expect(session.comments.findComment(id)).toMatchObject({
    status: "sent",
    batchId: "receipt",
    messageId: id,
  });
});
test("unknown publication freezes the original snapshot and rejects duplicate sends until receipt recovery", async () => {
  const session = new ConversationSessions().get("a");
  session.adopt(projection());
  let count = 0;
  session.host = {
    author: user,
    onSend: () => {
      count++;
      return { publication: "unknown" };
    },
  };
  session.comments.open(block, { from: 0, to: 5 }, "Retained");
  session.comments.message = "Prompt";
  expect(await session.comments.send()).toBe(false);
  expect(await session.comments.send()).toBe(false);
  expect(count).toBe(1);
  const retained = session.comments.retainedSubmission!;
  session.comments.confirmPublication("unrelated", true);
  expect(session.comments.retainedSubmission).toBeDefined();
  session.comments.confirmPublication(retained.id, true);
  expect(session.comments.retainedSubmission).toBeUndefined();
  expect(session.comments.message).toBe("");
  expect(session.comments.pending).toHaveLength(0);
});
test("a deferred review queue acknowledgement frees the composer without claiming publication", async () => {
  const session = new ConversationSessions().get("queued-reviews");
  session.adopt(projection());
  session.host = { author: user, onSend: () => ({ publication: "unknown" }) };
  session.comments.open(block, { from: 0, to: 5 }, "Review next");
  session.comments.message = "With this review";
  expect(await session.comments.send()).toBe(false);
  const retained = session.comments.retainedSubmission!;
  session.comments.recover(retained.id, {
    publication: "queued",
    recoveryRequired: false,
  });
  expect(session.comments.retainedSubmission).toBeUndefined();
  expect(session.comments.message).toBe("");
  expect(session.comments.pending).toHaveLength(0);
  expect(
    session.comments.findComment(retained.comments[0].commentId),
  ).toMatchObject({ status: "queued" });
  expect(
    session.comments.findComment(retained.comments[0].commentId)?.revision,
  ).toBeUndefined();
  session.comments.message = "Another message";
  expect(session.comments.canSend).toBe(true);
});
test("a proven queue refusal retains editable text and comments rather than freezing an unknown request", async () => {
  const session = new ConversationSessions().get("full-queue");
  session.adopt(projection());
  session.host = {
    author: user,
    onSend: () => ({
      publication: "refused",
      recoveryRequired: true,
      error: "queue_full",
    }),
  };
  session.comments.open(block, { from: 0, to: 5 }, "Review next");
  session.comments.message = "Keep this message";
  expect(await session.comments.send()).toBe(false);
  expect(session.comments.message).toBe("Keep this message");
  expect(session.comments.pending[0].comment.status).toBe("draft");
  expect(session.comments.retainedSubmission).toBeUndefined();
  expect(session.comments.canSend).toBe(true);
});
test("editing a sent comment retains its old answered revision and prepares a checked replacement", () => {
  const session = new ConversationSessions().get("a");
  session.host = { author: user };
  const p = projection();
  const anchor = {
      blockId: block.id,
      revision: block.revision,
      from: 0,
      to: 5,
      quote: "Hello",
    },
    revision = { kind: "human_message_revision" as const, id: "5".repeat(64) };
  p.threads = [
    {
      id: "6".repeat(32),
      label: "Message",
      anchor,
      source: exactSource(block, anchor, source),
      revision: { kind: "thread_version", id: "7".repeat(64) },
      comments: [
        {
          id: "old",
          messageId: "8".repeat(32),
          author: user,
          text: "Old",
          revision,
          status: "replied",
          replyItemId: "answer",
          batchId: "batch",
        },
      ],
    },
  ];
  session.adopt(p);
  const thread = session.comments.threads[0];
  session.comments.editSent(thread, thread.comments[0]);
  session.comments.draft = "Edited";
  const draft = session.comments.pending[0].comment;
  const batch = session.prepare(
    {
      id: "9".repeat(32),
      text: "",
      comments: [
        {
          threadId: thread.id,
          commentId: draft.id,
          anchor,
          author: user,
          text: draft.text,
        },
      ],
    },
    {
      id: target.thread_id,
      revision: { kind: "thread_version", id: "a".repeat(64) },
    },
  );
  expect(batch.comments[0]).toMatchObject({
    message_id: "8".repeat(32),
    expected_revision: revision,
    body: "Edited",
  });
  expect(thread.comments[0]).toMatchObject({
    text: "Old",
    status: "replied",
    replyItemId: "answer",
  });
  expect(draft.replyItemId).toBeUndefined();
  const next = projection();
  next.items[0].blocks = [{ ...block, revision: "changed" }];
  session.adopt(next);
  expect(() =>
    session.prepare(
      {
        id: "x",
        text: "",
        comments: [
          {
            threadId: thread.id,
            commentId: draft.id,
            anchor,
            author: user,
            text: draft.text,
          },
        ],
      },
      {
        id: target.thread_id,
        revision: { kind: "thread_version", id: "a".repeat(64) },
      },
    ),
  ).toThrow(/unavailable/);
});
test("published mixed submission with failed delivery retains one request without replaying delivered recipients", async () => {
  const session = new ConversationSessions().get("a");
  session.adopt(projection());
  let calls = 0;
  session.host = {
    author: user,
    onSend: () => {
      calls++;
      return {
        publication: "published",
        batchId: "published-batch",
        recoveryRequired: true,
        delivery: [
          { id: "a", label: "One", state: "completed" },
          { id: "b", label: "Two", state: "failed" },
        ],
        error: "Second agent failed",
      };
    },
  };
  session.comments.open(block, { from: 0, to: 5 }, "Question");
  session.comments.message = "Do work";
  expect(await session.comments.send()).toBe(false);
  expect(await session.comments.send()).toBe(false);
  expect(calls).toBe(1);
  expect(session.comments.threads[0].comments[0]).toMatchObject({
    status: "sent",
    batchId: "published-batch",
  });
  const retained = session.comments.retainedSubmission!;
  session.comments.keepUnresolved();
  expect(session.comments.unresolvedHistory[0].submission.id).toBe(retained.id);
  expect(session.comments.message).toBe("");
  expect(session.comments.pending).toHaveLength(0);
  session.comments.message = "A new request";
  expect(session.comments.canSend).toBe(true);
});
test("successful receipt clears only the original text and retains the canonical publication ID", async () => {
  const session = new ConversationSessions().get("a");
  session.adopt(projection());
  session.host = { author: user, onSend: () => ({ publication: "unknown" }) };
  session.comments.open(block, { from: 0, to: 5 }, "Question");
  session.comments.message = "Original";
  await session.comments.send();
  const id = session.comments.retainedSubmission!.id;
  session.comments.message = "New text";
  session.comments.recover(id, {
    publication: "published",
    batchId: "receipt-id",
  });
  expect(session.comments.message).toBe("New text");
  expect(session.comments.threads[0].comments[0].batchId).toBe("receipt-id");
  expect(session.comments.retainedSubmission).toBeUndefined();
});
test("authoritative tombstones remove published revisions while unavailable reads and local drafts retain them", () => {
  const session = new ConversationSessions().get("a"),
    p = projection();
  session.host = { author: user };
  p.threads = [
    {
      id: "d",
      label: "Message",
      anchor: {
        blockId: block.id,
        revision: block.revision,
        from: 0,
        to: 5,
        quote: "Hello",
      },
      comments: [
        {
          id: "old",
          messageId: "m",
          revision: { kind: "human_message_revision", id: "a".repeat(64) },
          author: user,
          text: "Published",
          status: "sent",
        },
      ],
    },
  ];
  session.adopt(p);
  session.comments.begin(session.comments.threads[0], "inline", "Draft");
  session.adopt(projection());
  expect(session.comments.threads[0].comments).toHaveLength(2);
  const removed = projection();
  removed.authoritativeDiscussions = new Set(["d"]);
  session.adopt(removed);
  expect(session.comments.threads[0].comments.map((c) => c.text)).toEqual([
    "Draft",
  ]);
});
