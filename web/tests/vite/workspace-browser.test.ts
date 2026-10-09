import { test, expect } from "vitest";
import {
  WorkspaceBrowser,
  type WorkspaceRead,
} from "../../src/lib/conversations/workspace-browser.svelte.js";
import { ConversationSession } from "../../src/lib/conversations/session.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";
import { conversationSubmissionResult } from "../../src/lib/conversations/submission-result.js";
const session = "1".repeat(32),
  repository = "2".repeat(32),
  operation = "3".repeat(128),
  commit = "4".repeat(40);
const value: WorkspaceRead = {
  version: 1,
  session,
  repository,
  workspace: "agent-fixture",
  location: "/fixture/workspace",
  operation,
  commit,
  selection: "selected",
  references: [],
  comparison: null,
  incomplete: null,
  files: [
    {
      path: "src/a.ts",
      kind: "file",
      bytes: "11",
      fingerprint: "observed",
      status: null,
    },
  ],
  next: null,
  file: {
    path: "src/a.ts",
    fingerprint: "5".repeat(64),
    contents: "café 🦀\n",
    before: null,
    unavailable: null,
  },
};
function browser() {
  const conversation = new ConversationSession();
  const client = new LocalClient(
    "",
    async () =>
      new Response(JSON.stringify(value), {
        headers: { "content-type": "application/json" },
      }),
  );
  const browser = new WorkspaceBrowser(client, conversation, () => {});
  browser.bindings = [
    {
      session,
      repository,
      repositoryName: "Example",
      workspace: "agent-fixture",
      agent: "Agent",
    },
  ];
  browser.bindingKey = browser.key(browser.bindings[0]);
  return { conversation, browser };
}
test("file selection joins the existing queue and sends exact UTF-8 ranges without transcript rows", async () => {
  const { conversation, browser: b } = browser();
  await b.select("src/a.ts");
  const block = b.blocks.after!;
  conversation.comments.open(block, { from: 5, to: 7 }, "Review crab");
  expect(conversation.projection.items).toHaveLength(0);
  const { thread, comment } = conversation.comments.pending[0];
  expect(conversation.comments.isAvailable(thread)).toBe(true);
  const sent = conversation.prepare(
    {
      id: "6".repeat(32),
      text: "",
      comments: [
        {
          threadId: thread.id,
          commentId: comment.id,
          anchor: thread.anchor,
          author: comment.author,
          text: comment.text,
        },
      ],
    },
    {
      id: "7".repeat(32),
      revision: { kind: "thread_version", id: "8".repeat(64) },
    },
  );
  expect(sent.comments).toEqual([]);
  expect(sent.workspace_comments).toHaveLength(1);
  expect(sent.workspace_comments![0].source.selection).toMatchObject({
    start_byte: "6",
    end_byte: "10",
  });
  await b.selectTab("changes");
  expect(conversation.comments.pending[0].comment.id).toBe(comment.id);
  expect(b.file?.file?.contents).toBe(value.file!.contents);
});
test("pending file publication never clears drafts just because delivery was queued", () => {
  const result = conversationSubmissionResult({
    request_id: "6".repeat(32),
    conversation_id: session,
    agents: [
      {
        version: 1,
        request_id: "7".repeat(32),
        session_id: session,
        state: "queued",
        error: null,
      },
    ],
    comment_publication: {
      request_id: "8".repeat(32),
      state: "pending",
      batch: null,
    },
  });
  expect(result.publication).toBe("unknown");
  expect(result.recoveryRequired).toBe(true);
});
test("late file responses cannot replace the current inspector selection", async () => {
  const { browser: b } = browser();
  const pending: ((v: WorkspaceRead) => void)[] = [];
  b.read = async () => new Promise<WorkspaceRead>((r) => pending.push(r));
  const first = b.select("src/a.ts"),
    second = b.select("src/b.ts");
  pending[1]({
    ...value,
    file: { ...value.file!, path: "src/b.ts", contents: "second" },
  });
  await second;
  pending[0](value);
  await first;
  expect(b.file?.file?.path).toBe("src/b.ts");
});
