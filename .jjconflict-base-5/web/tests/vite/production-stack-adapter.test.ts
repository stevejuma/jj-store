import { expect, test } from "vitest";
import { ProductionStackWorkspaceAdapter } from "../../src/lib/controller/stack-workspace/production-adapter.js";
import { workspaceBranchSource } from "../../src/lib/controller/stack-workspace/files.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import type {
  LocalReadSnapshot,
  LocalHead,
} from "../../src/lib/controller/repository-profile.js";
import type { Model } from "../../src/lib/api/client.js";

const repository = "b".repeat(32),
  operation = "a".repeat(128);
const tip = "1".repeat(40),
  base = "2".repeat(40),
  parent = "3".repeat(40);
const signature = {
  name: "Alex",
  email: "alex@example.test",
  timestamp: { millis: 123000, offset_minutes: 0 },
};
const target = (commit: string, parents = [parent]) => ({
  commit,
  change: "c".repeat(32),
  parents,
  description: "Real history",
  author: signature,
  committer: signature,
  conflicted: false,
  workspaces: [],
});
function fixture() {
  const pin: LocalReadSnapshot = {
    version: 1,
    profile: "git_sha1_jj045",
    repository,
    operation,
    generation: 2,
    heads: [operation],
  };
  const captured = {
    version: 1,
    profile: "git_sha1_jj045",
    repository,
    generation: 2,
    heads: [operation],
    metadata_commit: null,
    actor: null,
  } as LocalHead;
  const binding: Model<"LocalBinding"> = {
    repository,
    path: "/test/registered",
    name: "Repo",
    generation: 2,
    availability: "ready",
  };
  const calls: unknown[] = [];
  const reference = (name: string, commit: string) => ({
    kind: "bookmark" as const,
    name,
    terms: [commit],
    conflicted: false,
    workspaces: [],
    targets: [target(commit)],
  });
  const references = [reference("main", base), reference("topic", tip)];
  const source = {
    repository,
    profile: "git_sha1_jj045",
    snapshot: async () => captured,
    references: async (op: string, cursor?: string) => {
      calls.push(["references", op, cursor]);
      return {
        snapshot: pin,
        next_cursor: null,
        previous_cursor: null,
        data: { version: 1, operation, references, previous: null, next: null },
      };
    },
    stackHistory: async (query: {
      head: string;
      base?: string;
      cursor?: string;
      limit?: number;
      signal?: AbortSignal;
    }) => {
      calls.push(["history", query]);
      return {
        snapshot: pin,
        next_cursor: null,
        data: {
          version: 1,
          operation,
          head: query.head,
          base: query.base ?? null,
          next: null,
          commits: [
            {
              commit: {
                id: query.head,
                change_id: "c".repeat(32),
                parents: [parent],
                description: "Real history",
                author: signature,
                committer: signature,
                tree_terms: [],
                conflicted: false,
                divergent: false,
                visible: true,
                bookmarks: [],
                workspaces: [],
              },
              edges: [{ target: parent, kind: "direct" }],
            },
          ],
        },
      };
    },
    diffPaths: async (op: string, from: string, to: string) => {
      calls.push(["files", op, from, to]);
      return {
        snapshot: pin,
        next_cursor: null,
        data: { version: 1, from, to, changes: [] },
      };
    },
    close: () => calls.push("shared-close"),
  } as unknown as StoreDataSource;
  return { source, binding, captured, references, calls, pin };
}

test("production metadata is bounded and never initializes target, membership, files or commands", async () => {
  const { source, binding, calls } = fixture();
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  const snapshot = await adapter.read();
  expect(snapshot.folder).toBe(binding.path);
  expect(snapshot.target).toBeNull();
  expect(snapshot.active).toBeNull();
  expect(snapshot.workspaces).toEqual([]);
  expect(snapshot.capabilities).toEqual([]);
  expect(snapshot.targetOptions?.map((target) => target.name)).toEqual([
    "main",
    "topic",
  ]);
  expect(
    snapshot.stacks.every((stack) =>
      stack.commits.every((commit) => commit.files.length === 0),
    ),
  ).toBe(true);
  expect(calls).toEqual([["references", operation, undefined]]);
  await expect(
    adapter.preview({
      id: "request",
      repository,
      revision: snapshot.revision,
      command: { kind: "create_workspace", name: "work" },
    }),
  ).rejects.toThrow(/executor/i);
  adapter.close();
  expect(calls).not.toContain("shared-close");
});

test("binding generation must match the physical provider before its path is exposed", async () => {
  const { source, binding } = fixture();
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    { ...binding, generation: 3 },
    operation,
  );
  await expect(adapter.read()).rejects.toThrow();
  adapter.close();
});

test("exact target choice stays in the current read scope and branch net files compare real trees", async () => {
  const { source, binding, calls } = fixture();
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  let snapshot = await adapter.read();
  const chosen = snapshot.targetOptions!.find(
    (target) => target.name === "main",
  )!;
  snapshot = await adapter.chooseTarget({
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    target: chosen,
  });
  expect(snapshot.active).toBeNull();
  const stack = snapshot.stacks.find((stack) => stack.name === "topic")!;
  await adapter.readSourceFiles({
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    source: workspaceBranchSource(stack.id, "topic"),
  });
  expect(calls).toContainEqual(["files", operation, base, tip]);
  await expect(
    adapter.chooseTarget({
      repository,
      revision: snapshot.revision,
      readKey: "stale",
      target: chosen,
    }),
  ).rejects.toThrow();
  await expect(
    adapter.chooseTarget({
      repository,
      revision: snapshot.revision,
      readKey: snapshot.readKey,
      target: { ...chosen, commit: parent },
    }),
  ).rejects.toThrow();
  adapter.close();
});

test("target history pins its exact head and teardown aborts owned reads", async () => {
  const { source, binding, calls } = fixture();
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  const first = await adapter.read();
  const snapshot = await adapter.chooseTarget({
    repository,
    revision: first.revision,
    readKey: first.readKey,
    target: first.targetOptions![0]!,
  });
  const page = await adapter.readTargetHistory({
    repository,
    revision: snapshot.revision,
    branch: "main",
    readKey: snapshot.readKey,
    head: base,
    limit: 40,
  });
  expect(page.commits[0]!.graph?.parents).toEqual([parent]);
  const query = (
    calls.find((call) => Array.isArray(call) && call[0] === "history") as [
      string,
      { signal: AbortSignal },
    ]
  )[1];
  adapter.close();
  expect(query.signal.aborted).toBe(true);
  await expect(adapter.read()).rejects.toThrow();
  expect(calls).not.toContain("shared-close");
});

test("stack history uses the exact target range and commit files use provider parent identities", async () => {
  const { source, binding, calls } = fixture();
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  let snapshot = await adapter.read();
  snapshot = await adapter.chooseTarget({
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    target: snapshot.targetOptions![0]!,
  });
  const stack = snapshot.stacks[0]!;
  const page = await adapter.readStackHistory({
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    stack: stack.id,
    branch: "topic",
    head: tip,
    base,
    limit: 40,
  });
  expect(page.commits[0]!.id).toBe(tip);
  const query = calls.find(
    (call) => Array.isArray(call) && call[0] === "history",
  ) as [string, { head: string; base: string; limit: number }];
  expect(query[1]).toMatchObject({ head: tip, base, limit: 40 });
  await adapter.readSourceFiles({
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    source: tip,
  });
  expect(calls).toContainEqual(["files", operation, parent, tip]);
  await expect(
    adapter.readSourceFiles({
      repository,
      revision: snapshot.revision,
      readKey: snapshot.readKey,
      source: "9".repeat(40),
    }),
  ).rejects.toThrow(/loaded read scope/);
  adapter.close();
});

test("branch continuation reaches later target choices and rejects replayed cursors", async () => {
  const { source, binding, pin, references } = fixture();
  source.references = async (_op, cursor) =>
    ({
      snapshot: pin,
      next_cursor: cursor ? null : "next-branches",
      previous_cursor: null,
      data: {
        version: 1,
        operation,
        references: cursor
          ? [{ ...references[1]!, name: "later" }]
          : references,
        previous: null,
        next: null,
      },
    }) as never;
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  const first = await adapter.read();
  const next = await adapter.readBranches({
    repository,
    revision: first.revision,
    readKey: first.readKey,
    cursor: first.branchesNextCursor!,
  });
  expect(next.targetOptions?.map((item) => item.name)).toEqual([
    "main",
    "topic",
    "later",
  ]);
  expect(next.branchesNextCursor).toBeUndefined();
  await expect(
    adapter.readBranches({
      repository,
      revision: next.revision,
      readKey: next.readKey,
      cursor: "next-branches",
    }),
  ).rejects.toThrow();
  const refreshed = await adapter.read();
  expect(refreshed.targetOptions?.some((item) => item.name === "later")).toBe(
    true,
  );
  adapter.close();
});

test("refresh retains verified loaded ancestors at an unchanged head and base", async () => {
  const { source, binding, pin } = fixture();
  source.stackHistory = async () =>
    ({
      snapshot: pin,
      next_cursor: null,
      data: {
        version: 1,
        operation,
        head: tip,
        base,
        next: null,
        commits: [tip, parent].map((id) => ({
          commit: {
            id,
            change_id: "c".repeat(32),
            parents: [base],
            description: "Ancestor",
            author: signature,
            committer: signature,
            tree_terms: [],
            conflicted: false,
            divergent: false,
            visible: true,
            bookmarks: [],
            workspaces: [],
          },
          edges: [{ target: base, kind: "direct" }],
        })),
      },
    }) as never;
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  let snapshot = await adapter.read();
  snapshot = await adapter.chooseTarget({
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    target: snapshot.targetOptions![0]!,
  });
  const stack = snapshot.stacks[0]!;
  await adapter.readStackHistory({
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    stack: stack.id,
    branch: stack.name,
    head: tip,
    base,
    limit: 40,
  });
  const refreshed = await adapter.read();
  expect(refreshed.stacks[0]!.commits.map((commit) => commit.id)).toContain(
    parent,
  );
  expect(refreshed.stacks[0]!.segments?.[0]?.baseCommit).toBe(base);
  adapter.close();
});

test("operation refresh revalidates loaded target ancestry and rejects the old read scope", async () => {
  const { source, binding, captured, pin, references } = fixture();
  source.references = async () =>
    ({
      snapshot: pin,
      next_cursor: null,
      previous_cursor: null,
      data: {
        version: 1,
        operation: pin.operation,
        references,
        next: null,
        previous: null,
      },
    }) as never;
  source.stackHistory = async (query) =>
    ({
      snapshot: pin,
      next_cursor: null,
      data: {
        version: 1,
        operation: pin.operation,
        head: query.head,
        base: query.base ?? null,
        next: null,
        commits: [query.head, parent].map((id) => ({
          commit: {
            id,
            change_id: "c".repeat(32),
            parents: [tip],
            description:
              pin.operation === operation
                ? "Old annotation"
                : "Refreshed annotation",
            author: signature,
            committer: signature,
            tree_terms: [],
            conflicted: false,
            divergent: false,
            visible: true,
            bookmarks: [],
            workspaces: [],
          },
          edges: [{ target: tip, kind: "direct" }],
        })),
      },
    }) as never;
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  const first = await adapter.read();
  const snapshot = await adapter.chooseTarget({
    repository,
    revision: first.revision,
    readKey: first.readKey,
    target: first.targetOptions![0]!,
  });
  await adapter.readTargetHistory({
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    branch: "main",
    head: base,
    limit: 40,
  });
  const nextOperation = "f".repeat(128);
  pin.operation = nextOperation;
  captured.heads = [nextOperation];
  adapter.setOperation(nextOperation);
  const refreshed = await adapter.read();
  expect(
    refreshed.loadedTargetHistory?.commits.map((commit) => commit.id),
  ).toEqual([base, parent]);
  expect(refreshed.loadedTargetHistory?.commits[1]?.message).toBe(
    "Refreshed annotation",
  );
  await expect(
    adapter.readTargetHistory({
      repository,
      revision: snapshot.revision,
      readKey: snapshot.readKey,
      branch: "main",
      head: base,
      limit: 40,
    }),
  ).rejects.toThrow(/scope/);
  adapter.close();
});
