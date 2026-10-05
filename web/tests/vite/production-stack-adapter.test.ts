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

test("local state enables only core commands and projects named WIP into Unstaged", async () => {
  const { source, binding, calls } = fixture();
  const state: Model<"LocalStacksState"> = {
    version: 1,
    operation,
    revision: 1,
    active: "work",
    warning: null,
    workspaces: [
      {
        id: "work",
        name: "Work",
        target: "main",
        target_commit: base,
        applied: ["topic"],
        merge_commit: parent,
        conflicted: false,
        wip_commit: tip,
      },
    ],
  };
  const commands: unknown[] = [];
  Object.assign(source, {
    stacks: {
      execute: async (request: unknown) => {
        commands.push(request);
        return state;
      },
    },
  });
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  const snapshot = await adapter.read();
  expect(snapshot.active).toBe("work");
  expect(snapshot.capabilities).toEqual([
    "create_workspace",
    "switch_workspace",
    "update_workspace",
    "delete_workspace",
    "apply_stack",
    "unapply_stack",
  ]);
  expect(snapshot.workspaces[0]?.applied).toEqual([snapshot.stacks[0]?.id]);
  expect(adapter.observeWorkingTree).toBeUndefined();
  await adapter.readSourceFiles({
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    source: "unassigned",
  });
  expect(calls).toContainEqual(["files", operation, parent, tip]);
  const request = {
    id: "apply",
    readKey: snapshot.readKey,
    repository,
    revision: snapshot.revision,
    command: { kind: "unapply_stack" as const, stack: snapshot.stacks[0]!.id },
  };
  await adapter.preview(request);
  expect(commands).toHaveLength(1);
  await adapter.apply(request);
  expect(commands).toHaveLength(2);
  expect(commands[1]).toMatchObject({
    command: { kind: "unapply", branch: "topic" },
    operation,
    revision: 1,
  });
  const updated = await adapter.read();
  const settingsRequest = {
    ...request,
    id: "settings",
    revision: updated.revision,
    readKey: updated.readKey,
    command: {
      kind: "update_workspace" as const,
      workspace: "work",
      name: "Renamed",
      targetBranch: { name: "main", commit: base },
    },
  };
  const preview = await adapter.preview(settingsRequest);
  expect(preview.summary).toContain("Renamed");
  await adapter.apply(settingsRequest);
  expect(commands.at(-1)).toMatchObject({
    command: {
      kind: "update",
      workspace: "work",
      name: "Renamed",
      target: "main",
    },
    operation,
    revision: 1,
  });
  const deleting = await adapter.read();
  const deleteRequest = {
    ...request,
    id: "delete",
    revision: deleting.revision,
    readKey: deleting.readKey,
    command: { kind: "delete_workspace" as const, workspace: "work" },
  };
  expect((await adapter.preview(deleteRequest)).summary).toContain(
    "unfinished JJ work are retained",
  );
  await adapter.apply(deleteRequest);
  expect(commands.at(-1)).toMatchObject({
    command: { kind: "delete", workspace: "work" },
    operation,
    revision: 1,
  });
  adapter.close();
});
test("unknown local outcomes refresh state without executing or claiming command completion", async () => {
  const { source, binding } = fixture();
  const requests: unknown[] = [];
  Object.assign(source, {
    stacks: {
      execute: async (request: unknown) => {
        requests.push(request);
        return {
          version: 1,
          operation,
          revision: 0,
          active: null,
          workspaces: [],
          warning: null,
        };
      },
    },
  });
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  expect(adapter.retryPending).toBe(false);
  const result = await adapter.status("lost-response");
  expect(result.status).toBe("refreshed");
  expect(requests).toEqual([
    {
      version: 1,
      binding_generation: 2,
      operation: null,
      revision: null,
      command: { kind: "refresh" },
    },
  ]);
  adapter.close();
});

test("confirmed local membership settles before the new operation graph index is prepared", async () => {
  const { source, binding, pin, captured } = fixture();
  const state: Model<"LocalStacksState"> = {
    version: 1,
    operation,
    revision: 1,
    active: "work",
    warning: null,
    workspaces: [
      {
        id: "work",
        name: "Work",
        target: "main",
        target_commit: base,
        applied: [],
        merge_commit: parent,
        conflicted: false,
        wip_commit: tip,
      },
    ],
  };
  const references = source.references;
  source.references = (async (...args: Parameters<typeof references>) => {
    const page = await references(...args);
    return { ...page, data: { ...page.data, operation: pin.operation } };
  }) as typeof references;
  Object.assign(source, {
    stacks: {
      execute: async (request: { command: { kind: string } }) => {
        if (request.command.kind === "apply") {
          state.operation = "f".repeat(128);
          state.revision++;
          state.workspaces[0]!.applied = ["topic"];
          pin.operation = state.operation;
          captured.heads = [state.operation];
          source.stackHistory = async () => {
            throw new Error("index rebuild required");
          };
        }
        return state;
      },
    },
  });
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  const snapshot = await adapter.read();
  await adapter.readStackHistory({
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    stack: snapshot.stacks[0]!.id,
    branch: "topic",
    head: tip,
    base,
    limit: 40,
  });
  const result = await adapter.apply({
    id: "apply",
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    command: { kind: "apply_stack", stack: snapshot.stacks[0]!.id },
  });
  expect(result.status).toBe("committed");
  if (result.status !== "unknown") {
    expect(result.snapshot.workspaces[0]!.applied).toEqual([
      snapshot.stacks[0]!.id,
    ]);
    expect(result.snapshot.stacks[0]!.segments).toBeUndefined();
  }
  adapter.close();
});

test("local selected-file previews bind WIP and do not execute before apply", async () => {
  const { source, binding } = fixture();
  const requests: unknown[] = [],
    previews: unknown[] = [];
  const state: Model<"LocalStacksState"> = {
    version: 1,
    operation,
    revision: 1,
    active: "work",
    warning: null,
    workspaces: [
      {
        id: "work",
        name: "Work",
        target: "main",
        target_commit: base,
        applied: ["topic"],
        merge_commit: parent,
        wip_commit: tip,
        conflicted: false,
      },
    ],
  };
  source.stacks = {
    execute: async (request) => {
      requests.push(request);
      return structuredClone(state);
    },
    preview: async (request) => {
      previews.push(request);
      return {
        title: "Create commit",
        summary: "Selected file only",
        paths: ["README.md"],
      };
    },
  };
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  const snapshot = await adapter.read();
  expect(snapshot.capabilities).toContain("create_commit");
  const request = {
    id: "selected-file",
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    command: {
      kind: "create_commit" as const,
      stack: snapshot.stacks[0]!.id,
      message: "Selected file",
      selections: [{ source: "unassigned", path: "README.md" }],
    },
  };
  await adapter.preview(request);
  expect(requests).toHaveLength(1);
  expect(previews).toEqual([
    {
      version: 1,
      binding_generation: 2,
      operation,
      revision: 1,
      command: {
        kind: "commit",
        branch: "topic",
        message: "Selected file",
        selections: [{ source: tip, path: "README.md" }],
      },
    },
  ]);
  await adapter.apply(request);
  expect(requests).toHaveLength(2);
  expect(requests[1]).toEqual(previews[0]);
  await expect(
    adapter.preview({
      ...request,
      command: {
        ...request.command,
        selections: [
          { source: "unassigned", path: "README.md", hunks: ["display-only"] },
        ],
      },
    }),
  ).rejects.toThrow("whole files");
  adapter.close();
});

test("local basic mutations use server previews and inspect only displayed commits", async () => {
  const { source, binding, calls } = fixture();
  const previews: unknown[] = [],
    inspections: unknown[] = [];
  source.stacks = {
    execute: async () => ({
      version: 1,
      operation,
      revision: 1,
      active: "work",
      warning: null,
      workspaces: [
        {
          id: "work",
          name: "Work",
          target: "main",
          target_commit: base,
          applied: ["topic"],
          merge_commit: parent,
          wip_commit: tip,
          conflicted: false,
        },
      ],
    }),
    preview: async (request) => {
      previews.push(request.command);
      return { title: "Native preview", summary: "JJ mutation", paths: [] };
    },
    inspect: async (request) => {
      inspections.push(request);
      return {
        commits: request.commits.map((commit) => ({
          commit,
          mutable: commit === tip,
          reason: commit === tip ? null : "Protected target",
        })),
      };
    },
  };
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  const snapshot = await adapter.read(),
    stack = snapshot.stacks[0]!;
  expect(stack.commits[0]!.immutable).toBe(false);
  expect(snapshot.capabilities).toContain("reword");
  expect(
    calls.some((call) => Array.isArray(call) && call[0] === "history"),
  ).toBe(false);
  const request = {
    id: "basic",
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
  };
  await adapter.preview({
    ...request,
    command: { kind: "create_stack", name: "dependent", parent: stack.id },
  });
  await adapter.preview({
    ...request,
    command: {
      kind: "create_branch",
      stack: stack.id,
      commit: tip,
      name: "lower",
      placement: "below",
    },
  });
  await adapter.preview({
    ...request,
    command: {
      kind: "rename_branch",
      stack: stack.id,
      branch: "topic",
      name: "renamed",
    },
  });
  await adapter.preview({
    ...request,
    command: { kind: "reword", commit: tip, message: "Changed" },
  });
  await adapter.preview({
    ...request,
    command: {
      kind: "add_empty_commit",
      commit: tip,
      message: "",
      placement: "above",
    },
  });
  expect(previews).toEqual([
    { kind: "create_stack", name: "dependent", parent: "topic" },
    { kind: "create_branch", commit: tip, name: "lower", placement: "below" },
    { kind: "rename_branch", branch: "topic", name: "renamed" },
    { kind: "reword", commit: tip, message: "Changed" },
    { kind: "empty", commit: tip, message: "", placement: "above" },
  ]);
  const page = await adapter.readStackHistory({
    ...request,
    stack: stack.id,
    branch: "topic",
    head: tip,
    base,
    limit: 40,
  });
  expect(page.commits[0]!.immutable).toBe(false);
  expect(inspections).toHaveLength(2);
  expect(inspections[0]).toMatchObject({
    operation,
    revision: 1,
    commits: [tip],
  });
  await expect(
    adapter.preview({
      ...request,
      command: { kind: "reword", commit: "9".repeat(40), message: "Unknown" },
    }),
  ).rejects.toThrow("outside the displayed");
  adapter.close();
});

test("lazy local diff reads replace display hunks with native byte-bound selections", async () => {
  const { source, binding, pin } = fixture();
  const calls: unknown[] = [];
  source.stacks = {
    execute: async () => ({
      version: 1,
      operation,
      revision: 1,
      active: "work",
      warning: null,
      workspaces: [
        {
          id: "work",
          name: "Work",
          target: "main",
          target_commit: base,
          applied: ["topic"],
          merge_commit: parent,
          wip_commit: tip,
          conflicted: false,
        },
      ],
    }),
    preview: async () => ({
      title: "Native preview",
      summary: "JJ mutation",
      paths: [],
    }),
    inspect: async (request) => ({
      commits: request.commits.map((commit) => ({
        commit,
        mutable: true,
        reason: null,
      })),
    }),
    hunks: async (request) => {
      calls.push(request);
      return {
        source: request.source,
        path: request.path,
        reason: null,
        hunks: [
          {
            id: "native:" + "a".repeat(64),
            label: "@@ -1 +1 @@",
            patch: "-old\n+new\n",
          },
        ],
      };
    },
  };
  source.diff = async (_op, from, to, path) =>
    ({
      snapshot: pin,
      next_cursor: null,
      data: {
        version: 1,
        from,
        to,
        path,
        changes: [
          {
            path,
            before: { kind: "absent" },
            after: { kind: "absent" },
            sources: [],
            content: { status: "unavailable", reason: "not_requested" },
          },
        ],
      },
    }) as never;
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  const snapshot = await adapter.read();
  const request = {
    repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    source: "unassigned",
    path: "draft.txt",
  };
  const diff = await adapter.readFileDiff(request);
  expect(diff.hunks[0]?.id).toBe("native:" + "a".repeat(64));
  expect(calls).toEqual([
    {
      version: 1,
      binding_generation: 2,
      operation,
      revision: 1,
      source: tip,
      path: "draft.txt",
    },
  ]);
  await adapter.readFileDiff({
    ...request,
    source: workspaceBranchSource(snapshot.stacks[0]!.id, "topic"),
  });
  expect(calls).toHaveLength(1);
  adapter.close();
});

test("dependent native ranges retain top cursors and issue per-segment tree comparisons", async () => {
  const { source, binding, pin, references, calls } = fixture();
  references.push({
    ...references[1]!,
    name: "foundation",
    terms: [parent],
    targets: [target(parent, [base])],
  });
  const original = source.stackHistory!;
  source.stackHistory = async (query) => {
    const result = await original(query);
    const upper = result.data.commits[0]!;
    const lower = structuredClone(upper);
    lower.commit.id = parent;
    lower.commit.parents = [base];
    lower.edges = [{ target: base, kind: "direct" }];
    lower.commit.bookmarks = [
      {
        name: "foundation",
        remote: null,
        added: true,
        removed: false,
        conflicted: false,
      },
    ];
    return { ...result, data: { ...result.data, commits: [upper, lower] } };
  };
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  const initial = await adapter.read();
  const selected = await adapter.chooseTarget({
    repository,
    revision: initial.revision,
    readKey: initial.readKey,
    target: { name: "main", commit: base },
  });
  const topic = selected.stacks.find((stack) => stack.name === "topic")!;
  await adapter.readStackHistory({
    repository,
    revision: selected.revision,
    readKey: selected.readKey,
    stack: topic.id,
    branch: "topic",
    head: tip,
    base,
    limit: 40,
  });
  await adapter.readSourceFiles({
    repository,
    revision: selected.revision,
    readKey: selected.readKey,
    source: workspaceBranchSource(topic.id, "foundation"),
  });
  expect(calls).toContainEqual(["files", operation, base, parent]);
  const refreshed = await adapter.read();
  const projected = refreshed.stacks.find((stack) => stack.name === "topic")!;
  expect(projected.segments?.map((segment) => segment.head.name)).toEqual([
    "topic",
    "foundation",
  ]);
  const request = {
    repository,
    revision: refreshed.revision,
    readKey: refreshed.readKey,
  };
  await adapter.readSourceFiles({
    ...request,
    source: workspaceBranchSource(topic.id, "topic"),
  });
  await adapter.readSourceFiles({
    ...request,
    source: workspaceBranchSource(topic.id, "foundation"),
  });
  expect(calls).toContainEqual(["files", operation, parent, tip]);
  expect(calls).toContainEqual(["files", operation, base, parent]);
  await expect(
    adapter.readStackHistory({
      ...request,
      stack: topic.id,
      branch: "foundation",
      head: parent,
      base,
      limit: 40,
    }),
  ).rejects.toThrow("stack head");
  expect(pin.operation).toBe(operation);
  adapter.close();
});

test("newly loaded ancestor destinations are usable without a refresh", async () => {
  const { source, binding } = fixture();
  const previews: unknown[] = [];
  source.stacks = {
    execute: async () => ({
      version: 1,
      operation,
      revision: 1,
      active: "work",
      warning: null,
      workspaces: [
        {
          id: "work",
          name: "Work",
          target: "main",
          target_commit: base,
          applied: ["topic"],
          merge_commit: "4".repeat(40),
          wip_commit: "5".repeat(40),
          conflicted: false,
        },
      ],
    }),
    preview: async (request) => {
      previews.push(request.command);
      return {
        title: "Transfer",
        summary: "Native squash",
        paths: ["draft.txt"],
      };
    },
    inspect: async (request) => ({
      commits: request.commits.map((commit) => ({
        commit,
        mutable: true,
        reason: null,
      })),
    }),
  };
  const history = source.stackHistory!;
  source.stackHistory = async (query) => {
    const page = await history(query);
    const lower = structuredClone(page.data.commits[0]!);
    lower.commit.id = parent;
    lower.commit.parents = [base];
    lower.edges = [{ target: base, kind: "direct" }];
    return {
      ...page,
      data: { ...page.data, commits: [...page.data.commits, lower] },
    };
  };
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  try {
    const snapshot = await adapter.read();
    const stack = snapshot.stacks.find((stack) => stack.name === "topic")!;
    await adapter.readStackHistory({
      repository,
      revision: snapshot.revision,
      readKey: snapshot.readKey,
      stack: stack.id,
      branch: "topic",
      head: tip,
      base,
      limit: 40,
    });
    await adapter.preview({
      id: "ancestor-drop",
      repository,
      revision: snapshot.revision,
      readKey: snapshot.readKey,
      command: {
        kind: "transfer",
        target: parent,
        selection: {
          source: "unassigned",
          path: "draft.txt",
          hunks: ["native:" + "a".repeat(64)],
        },
      },
    });
    expect(previews).toEqual([
      {
        kind: "transfer",
        target: parent,
        selections: [
          {
            source: "5".repeat(40),
            path: "draft.txt",
            hunks: ["native:" + "a".repeat(64)],
          },
        ],
      },
    ]);
  } finally {
    adapter.close();
  }
});

function enableLocalState(source: StoreDataSource) {
  source.stacks = {
    execute: async () => ({
      version: 1,
      operation,
      revision: 1,
      active: "work",
      warning: null,
      conflicts: ["context.ts"],
      workspaces: [
        {
          id: "work",
          name: "Work",
          target: "main",
          target_commit: base,
          applied: ["topic"],
          merge_commit: parent,
          wip_commit: tip,
          conflicted: true,
        },
      ],
    }),
    preview: async () => ({ title: "Native", summary: "Native", paths: [] }),
    inspect: async (request) => ({
      commits: request.commits.map((commit) => ({
        commit,
        mutable: true,
        reason: null,
      })),
    }),
  };
}
test("native hunk replacement retains the complete operation-pinned patch for context expansion", async () => {
  const { source, binding, pin } = fixture();
  enableLocalState(source);
  const hunks = [
    {
      id: "native:" + "a".repeat(64),
      label: "@@ -2,1 +2,1 @@",
      patch:
        "--- a/context.ts\n+++ b/context.ts\n@@ -2,1 +2,1 @@\n-old\n+new\n",
    },
  ];
  source.stacks!.hunks = async () => ({
    source: tip,
    path: "context.ts",
    hunks,
    reason: null,
  });
  source.diff = async () =>
    ({
      snapshot: pin,
      next_cursor: null,
      previous_cursor: null,
      data: {
        version: 1,
        from: parent,
        to: tip,
        path: "context.ts",
        changes: [
          {
            path: "context.ts",
            before: [null],
            after: [null],
            sources: [],
            content: {
              status: "text",
              hunks: [
                {
                  kind: "matching",
                  before_start: 0,
                  after_start: 0,
                  before: "prefix\n",
                  after: "prefix\n",
                },
                {
                  kind: "different",
                  before_start: 7,
                  after_start: 7,
                  before: "old\n",
                  after: "new\n",
                },
                {
                  kind: "matching",
                  before_start: 11,
                  after_start: 11,
                  before: "suffix\n",
                  after: "suffix\n",
                },
              ],
            },
          },
        ],
      },
    }) as never;
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  try {
    const snapshot = await adapter.read();
    const file = await adapter.readFileDiff({
      repository,
      revision: snapshot.revision,
      readKey: snapshot.readKey,
      source: "unassigned",
      path: "context.ts",
    });
    expect(file.hunks).toEqual(hunks);
    expect(file.patch).toMatch(/^ prefix$/mu);
    expect(file.patch).toMatch(/^ suffix$/mu);
  } finally {
    adapter.close();
  }
});
test("conflict reads preserve ordered multi-sided and unsupported descriptors and reject stale/foreign scopes", async () => {
  const { source, binding } = fixture();
  enableLocalState(source);
  const terms = ["left\n", "base\n", "right\n", "base\n", null];
  let reason: string | null = null,
    responseSource = tip;
  source.stacks!.conflict = async () => ({
    source: responseSource,
    path: "context.ts",
    terms,
    reason,
  });
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    operation,
  );
  try {
    const snapshot = await adapter.read();
    const request = {
      repository,
      revision: snapshot.revision,
      readKey: snapshot.readKey,
      path: "context.ts",
    };
    expect(await adapter.readConflict(request)).toMatchObject({
      path: "context.ts",
      terms,
    });
    reason = "Conflict versions exceed the bounded text or version limit.";
    expect(await adapter.readConflict(request)).toMatchObject({
      terms,
      reason,
    });
    responseSource = parent;
    await expect(adapter.readConflict(request)).rejects.toThrow();
    await expect(
      adapter.readConflict({ ...request, readKey: "stale" }),
    ).rejects.toThrow();
    adapter.close();
    await expect(adapter.readConflict(request)).rejects.toThrow();
  } finally {
    adapter.close();
  }
});
