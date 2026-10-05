import assert from "node:assert/strict";
import { test } from "vitest";
import { StackWorkspaceReads } from "../../src/lib/controller/stack-workspace/reads.js";
import { stackRevisions } from "../../src/lib/controller/stack-workspace/presentation.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import type { LocalReadSnapshot } from "../../src/lib/controller/repository-profile.js";
const id = (part: string) => part.repeat(40);
const operation = "a".repeat(128),
  repository = "b".repeat(32);
const head = id("1"),
  base = id("2"),
  other = id("3");
const pin: LocalReadSnapshot = {
  version: 1,
  profile: "git_sha1_jj045",
  repository,
  generation: 1,
  operation,
  heads: [operation],
};
const signature = {
  name: "Alex",
  email: "alex@example.test",
  timestamp: { millis: 123000, offset_minutes: 0 },
};
const node = {
  commit: {
    id: head,
    change_id: "c".repeat(32),
    parents: [base, other],
    description: "Merge topic",
    author: signature,
    committer: signature,
    tree_terms: [id("4")],
    conflicted: true,
    divergent: true,
    visible: true,
    bookmarks: [
      {
        name: "topic",
        remote: null,
        added: true,
        removed: false,
        conflicted: false,
      },
    ],
    workspaces: ["default"],
  },
  edges: [
    { target: base, kind: "direct" },
    { target: other, kind: "missing" },
  ],
};
const fileValue = { type: "file", id: id("5"), executable: false };
const textChange = {
  path: "src/café.ts",
  before: [fileValue],
  after: [fileValue],
  sources: [],
  content: {
    status: "text",
    hunks: [
      {
        kind: "matching",
        before_start: 0,
        after_start: 0,
        before: "café\n",
        after: "café\n",
      },
      {
        kind: "different",
        before_start: 6,
        after_start: 6,
        before: "old\n",
        after: "new\n",
      },
      {
        kind: "matching",
        before_start: 10,
        after_start: 10,
        before: "tail\n",
        after: "tail\n",
      },
    ],
  },
};
function fixture() {
  const calls: Array<[string, ...unknown[]]> = [];
  const envelope = (data: unknown, next: string | null = null) => ({
    snapshot: pin,
    data,
    next_cursor: next,
    previous_cursor: null,
  });
  const source = {
    repository,
    profile: "git_sha1_jj045",
    stackHistory: async (query: any) => {
      calls.push(["history", query]);
      return envelope(
        { version: 1, operation, head, base, commits: [node], next: other },
        "opaque-next",
      );
    },
    diffPaths: async (...args: unknown[]) => {
      calls.push(["files", ...args]);
      return envelope({
        version: 1,
        from: base,
        to: head,
        changes: [
          {
            ...textChange,
            content: { status: "unavailable", reason: "not_requested" },
          },
        ],
        next: null,
        previous: null,
      });
    },
    diff: async (...args: unknown[]) => {
      calls.push(["diff", ...args]);
      return envelope({
        version: 1,
        from: base,
        to: head,
        path: textChange.path,
        changes: [textChange],
      });
    },
    close: () => calls.push(["close-source"]),
  } as unknown as StoreDataSource;
  return {
    source,
    calls,
    envelope,
    reads: new StackWorkspaceReads(source, pin),
  };
}
test("stack reads preserve real graph metadata and load no file details with history", async () => {
  const { reads, calls } = fixture();
  const page = await reads.history({
    head,
    base,
    limit: 2,
    cursor: "opaque-original",
  });
  assert.equal(page.nextCursor, "opaque-next");
  assert.deepEqual(page.commits[0]?.files, []);
  const commit = page.commits[0]!;
  const revision = stackRevisions({
    id: "stack",
    name: "topic",
    base,
    heads: [{ name: "topic", commit: head }],
    commits: [commit],
    conflicts: true,
  })[0]!;
  assert.deepEqual(revision.parents, [base, other]);
  assert.equal(revision.conflicted, true);
  assert.equal(revision.divergent, true);
  assert.equal(revision.bookmarks[0]?.name, "topic");
  assert.deepEqual(revision.workspaces, ["default"]);
  assert.equal(calls.length, 1);
  const query = calls[0]![1] as any;
  assert.deepEqual(
    { ...query, signal: undefined },
    {
      operation,
      head,
      base,
      limit: 2,
      cursor: "opaque-original",
      signal: undefined,
    },
  );
  reads.close();
});
test("net files use exact boundary trees and preserve unloaded state instead of unioning commit hunks", async () => {
  const { reads, source, calls, envelope } = fixture();
  const page = await reads.files({ from: base, to: head }, "opaque-file-page");
  assert.deepEqual(
    page.files.map((file) => file.path),
    [textChange.path],
  );
  assert.deepEqual(page.files[0]?.hunks, []);
  assert.equal(page.files[0]?.content?.status, "unloaded");
  assert.deepEqual(calls[0]?.slice(0, 5), [
    "files",
    operation,
    base,
    head,
    "opaque-file-page",
  ]);
  source.diffPaths = async () =>
    envelope({ version: 1, from: base, to: head, changes: [] }) as any;
  assert.deepEqual((await reads.files({ from: base, to: head })).files, []);
  reads.close();
});
test("diff presentation keeps byte offsets out of line numbers and skips matching-only sections", async () => {
  const { reads, calls } = fixture();
  const file = await reads.diff({ from: base, to: head }, textChange.path);
  assert.equal(file.content?.status, "ready");
  assert.match(file.patch!, /^ café$/mu);
  assert.match(file.patch!, /^ tail$/mu);
  assert.equal(file.hunks.length, 1);
  assert.match(file.hunks[0]!.patch, /@@ -2,1 \+2,1 @@\n-old\n\+new\n/u);
  assert.doesNotMatch(file.hunks[0]!.patch, /^ (café|tail)$/mu);
  assert.match(file.hunks[0]!.id, /^presentation:/u);
  assert.equal(calls.length, 1);
  reads.close();
});
test("unavailable diffs retain ordered conflict terms and rename sources without fabricated text", async () => {
  const { reads, source, envelope } = fixture();
  const change = {
    ...textChange,
    before: [fileValue, null, fileValue],
    sources: [{ kind: "rename", path: "old.ts", terms: [fileValue] }],
    content: { status: "unavailable", reason: "conflict" },
  };
  source.diff = async () =>
    envelope({
      version: 1,
      from: base,
      to: head,
      path: change.path,
      changes: [change],
    }) as any;
  const file = await reads.diff({ from: base, to: head }, change.path);
  assert.deepEqual(file.hunks, []);
  assert.deepEqual(file.content, { status: "unavailable", reason: "conflict" });
  assert.deepEqual(file.change?.before, change.before);
  assert.deepEqual(file.change?.sources, change.sources);
  reads.close();
});
test("reads reject foreign snapshots, wrong tree ranges and nonadvancing pages", async () => {
  const { reads, source, envelope } = fixture();
  source.diffPaths = async () =>
    ({
      ...envelope({ version: 1, from: base, to: head, changes: [] }),
      snapshot: { ...pin, generation: 2 },
    }) as any;
  await assert.rejects(reads.files({ from: base, to: head }), {
    code: "snapshot_mismatch",
  });
  source.diffPaths = async () =>
    envelope({ version: 1, from: other, to: head, changes: [] }) as any;
  await assert.rejects(reads.files({ from: base, to: head }), {
    code: "snapshot_mismatch",
  });
  source.diffPaths = async () =>
    envelope({ version: 1, from: base, to: head, changes: [] }, "same") as any;
  await assert.rejects(reads.files({ from: base, to: head }, "same"), {
    code: "invalid_stack_page",
  });
  reads.close();
});
test("canceling or closing rejects late reads without closing the repository source", async () => {
  const { reads, source, calls, envelope } = fixture();
  let finish: (value: any) => void = () => {};
  let signal: AbortSignal | undefined;
  source.diffPaths = async (_op, _from, _to, _cursor, inputSignal) => {
    signal = inputSignal;
    return new Promise((resolve) => {
      finish = resolve;
    });
  };
  const canceled = new AbortController();
  const result = reads.files(
    { from: base, to: head },
    undefined,
    canceled.signal,
  );
  canceled.abort();
  assert.equal(signal?.aborted, true);
  finish(envelope({ from: base, to: head, changes: [] }));
  await assert.rejects(result, { name: "AbortError" });
  const late = reads.files({ from: base, to: head });
  reads.close();
  assert.equal(signal?.aborted, true);
  finish(envelope({ from: base, to: head, changes: [] }));
  await assert.rejects(late, { name: "AbortError" });
  await assert.rejects(reads.files({ from: base, to: head }), {
    name: "AbortError",
  });
  assert.deepEqual(calls, []);
});

test("stack reads validate profile, page limits and advertised history capability before transport", async () => {
  const { reads, source, calls } = fixture();
  assert.throws(
    () => new StackWorkspaceReads({ ...source, repository: "different" }, pin),
    { code: "snapshot_mismatch" },
  );
  assert.throws(
    () => new StackWorkspaceReads({ ...source, profile: "native" }, pin),
    { code: "snapshot_mismatch" },
  );
  for (const limit of [0, 1.5, 501])
    await assert.rejects(reads.history({ head, base, limit }), {
      code: "invalid_stack_page",
    });
  source.stackHistory = undefined;
  await assert.rejects(reads.history({ head, base }), {
    code: "stack_history_unavailable",
  });
  assert.deepEqual(calls, []);
  reads.close();
});
