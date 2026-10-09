import { ApplicationFreshness } from "../../src/lib/controller/application-freshness.js";
import { test, expect, vi } from "vitest";
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

test("list and file caches survive close, reopen and tab changes until invalidated", async () => {
  const { browser: b } = browser();
  const reads: (string | null)[] = [];
  b.read = async (request) => {
    reads.push(request.path);
    return {
      ...value,
      file: request.path ? { ...value.file!, path: request.path } : null,
    };
  };
  b.railOpen = true;
  await b.load();
  b.listing.scroll = 120;
  b.toggleDirectory("src");
  await b.select("src/a.ts");
  const block = b.blocks.after!;
  b.conversation.comments.open(block, { from: 0, to: 4 }, "Keep draft");
  b.fileOpen = false;
  await b.select("src/b.ts");
  await b.select("src/a.ts");
  await b.load();
  expect(reads).toEqual([null, "src/a.ts", "src/b.ts"]);
  await b.selectTab("changes");
  await b.selectTab("files");
  expect(reads).toHaveLength(4);
  expect(b.listing.scroll).toBe(120);
  expect(b.listing.collapsed.has("src")).toBe(true);
  await b.refresh();
  expect(reads.slice(4)).toEqual([null, "src/a.ts"]);
  expect(b.conversation.comments.pending[0].comment.text).toBe("Keep draft");
  b.dispose();
});

test("file cache evicts old entries and never caches failed reads", async () => {
  const { browser: b } = browser();
  let reads = 0;
  b.read = async (request) => {
    reads++;
    return { ...value, file: { ...value.file!, path: request.path! } };
  };
  for (let i = 0; i < 25; i++) await b.select(`src/${i}.ts`);
  await b.select("src/24.ts");
  expect(reads).toBe(25);
  await b.select("src/0.ts");
  expect(reads).toBe(26);
  b.read = async () => {
    throw new Error("unavailable");
  };
  await b.select("src/failure.ts");
  expect(b.fileError).toBe("unavailable");
  b.read = async (request) => {
    reads++;
    return { ...value, file: { ...value.file!, path: request.path! } };
  };
  await b.select("src/failure.ts");
  expect(reads).toBe(27);
});

test("invalidation discards in-flight content without changing retained source navigation", async () => {
  const { browser: b, conversation } = browser();
  await b.select("src/a.ts");
  const source = conversation.workspaceSources.get(b.blocks.after!.id)!;
  b.invalidate();
  let resolve!: (value: WorkspaceRead) => void;
  b.read = async () => new Promise((r) => (resolve = r));
  const read = b.select("src/a.ts");
  b.invalidate();
  resolve({ ...value, file: { ...value.file!, contents: "obsolete" } });
  await read;
  expect(b.file?.file?.contents).toBe(value.file!.contents);
  source.reveal();
  b.railOpen = false;
  await b.refresh();
  expect(b.file?.file?.contents).toBe(value.file!.contents);
  expect(b.fileLoading).toBe(false);
});

test("workspace feed invalidates hidden caches, refreshes visible files, and never polls healthy idle", async () => {
  vi.useFakeTimers();
  let stream!: ReadableStreamDefaultController<Uint8Array>;
  const freshness = new ApplicationFreshness(
    async () =>
      new Response(
        new ReadableStream({
          start(c) {
            stream = c;
          },
        }),
        { headers: { "content-type": "text/event-stream" } },
      ),
  );
  let sequence = 0;
  const frame = (event: unknown, type = "changed") =>
    stream.enqueue(
      new TextEncoder().encode(
        `id: jse1:${"a".repeat(32)}:${"b".repeat(32)}:${sequence++}\nevent: ${type}\ndata: ${JSON.stringify(event)}\n\n`,
      ),
    );
  const ready = freshness.configure(true, null);
  frame({ version: 1, type: "ready", witnesses: [] }, "ready");
  await ready;
  const { browser: b } = browser();
  let active = true;
  let reads = 0;
  b.read = async (request) => {
    reads++;
    return { ...value, file: request.path ? value.file : null };
  };
  b.railOpen = true;
  const stop = b.watch(freshness, () => active && (b.railOpen || b.fileOpen));
  const notice = (domain: string, repo = repository) =>
    frame({
      version: 1,
      type: "changed",
      change: { repository: repo, domain, witness: String(sequence) },
    });
  try {
    await b.load();
    await b.select("src/a.ts");
    for (let i = 0; i < 4; i++) {
      stream.enqueue(new TextEncoder().encode(": keep-alive\n\n"));
      await vi.advanceTimersByTimeAsync(15_000);
    }
    expect(reads).toBe(2);
    notice("threads");
    notice("code", "f".repeat(32));
    await vi.advanceTimersByTimeAsync(1000);
    expect(reads).toBe(2);
    notice("workspaces");
    notice("code");
    await vi.advanceTimersByTimeAsync(1000);
    expect(reads).toBe(4);
    active = false;
    notice("code");
    await vi.advanceTimersByTimeAsync(10_000);
    expect(reads).toBe(4);
    active = true;
    await b.load();
    await b.select("src/a.ts");
    expect(reads).toBe(6);
    await vi.advanceTimersByTimeAsync(10_000);
    expect(reads).toBe(6);
    frame(
      { version: 1, type: "reset", reason: "replay_window", witnesses: [] },
      "reset",
    );
    await vi.advanceTimersByTimeAsync(1000);
    expect(reads).toBe(8);
    for (let i = 0; i < 4; i++) {
      stream.enqueue(new TextEncoder().encode(": keep-alive\n\n"));
      await vi.advanceTimersByTimeAsync(15_000);
    }
    expect(reads).toBe(8);
  } finally {
    stop();
    freshness.destroy();
    vi.useRealTimers();
  }
});
