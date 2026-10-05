import { workspaceFileDrag } from "../../src/lib/controller/stack-workspace/file-drag.js";
import { expect, test, vi } from "vitest";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import {
  cloneWorkspace,
  type WorkspaceSourceRequest,
  type WorkspaceFilePage,
  type WorkspaceFile,
} from "../../src/lib/controller/stack-workspace/types.js";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import { workspaceBranchSource } from "../../src/lib/controller/stack-workspace/files.js";
import { workspaceDropFeedback } from "../../src/lib/controller/stack-workspace/drag.js";
function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((r) => {
    resolve = r;
  });
  return { promise, resolve };
}
async function setup() {
  const original = workspaceFixture();
  const base = new MockStackWorkspaceAdapter(cloneWorkspace(original));
  for (const stack of base.snapshot.stacks)
    for (const commit of stack.commits) commit.files = [];
  base.snapshot.capabilities = [];
  const calls: WorkspaceSourceRequest[] = [];
  const patches: Array<{ source: string; path: string; signal?: AbortSignal }> =
    [];
  const adapter = Object.assign(base, {
    readSourceFiles: async (
      request: WorkspaceSourceRequest,
    ): Promise<WorkspaceFilePage> => {
      calls.push(request);
      const commit = original.stacks
        .flatMap((s) => s.commits)
        .find((c) => c.id === request.source);
      return {
        files: (commit?.files ?? []).map((file) => ({
          ...cloneWorkspace(file),
          hunks: [],
          content: { status: "unloaded" },
        })),
      };
    },
    readFileDiff: async (
      request: WorkspaceSourceRequest & { path: string },
    ): Promise<WorkspaceFile> => {
      patches.push(request);
      const commit = original.stacks
        .flatMap((s) => s.commits)
        .find((c) => c.id === request.source);
      return cloneWorkspace(
        commit!.files.find((f) => f.path === request.path)!,
      );
    },
  });
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  return { controller, adapter, calls, patches, original };
}
test("lazy file reads are authoritative, selected independently, and do not globally lock browsing", async () => {
  const { controller, calls, original, adapter } = await setup();
  // A mock commit union must never substitute for a not-yet-loaded real net diff.
  adapter.snapshot.stacks[0]!.commits[0]!.files = cloneWorkspace(
    original.stacks[0]!.commits[0]!.files,
  );
  expect(
    controller.filesFor(workspaceBranchSource("api", "feature/api")),
  ).toEqual([]);
  controller.selectStack("api");
  controller.selectCommit("api2");
  await vi.waitFor(() => expect(controller.filesFor("api2")).toHaveLength(1));
  expect(calls).toHaveLength(1);
  expect(controller.locked).toBe(false);
  expect(controller.filesFor("api2")[0]!.hunks).toEqual([]);
  controller.selectCommit("api2");
  await Promise.resolve();
  expect(calls).toHaveLength(1);
  controller.close();
});
test("opening lazy diffs keeps each stack's adjacent diff and only loads selected patches", async () => {
  const { controller, patches } = await setup();
  controller.selectCommit("api2");
  await vi.waitFor(() => expect(controller.filesFor("api2")).toHaveLength(1));
  controller.selectFile("api2", "src/api.ts");
  await vi.waitFor(() =>
    expect(controller.filesFor("api2")[0]!.hunks.length).toBeGreaterThan(0),
  );
  controller.selectCommit("ui1");
  await vi.waitFor(() => expect(controller.filesFor("ui1")).toHaveLength(1));
  controller.selectFile("ui1", "src/toolbar.ts");
  await vi.waitFor(() => expect(patches).toHaveLength(2));
  expect(controller.diffs.map((diff) => diff.origin)).toEqual([
    "stack-api",
    "stack-ui",
  ]);
  controller.close();
});
test("branch net files load inline and never fall back to the mock's reverted file union", async () => {
  const { controller, adapter } = await setup();
  const gate = deferred<WorkspaceFilePage>();
  adapter.readSourceFiles = async () => gate.promise;
  controller.selectStackFiles("api", "feature/api");
  const source = workspaceBranchSource("api", "feature/api");
  expect(controller.details.fileState(source).loading).toBe(true);
  expect(controller.filesFor(source)).toEqual([]);
  gate.resolve({ files: [] });
  await vi.waitFor(() =>
    expect(controller.details.fileState(source).loaded).toBe(true),
  );
  expect(controller.filesFor(source)).toEqual([]);
  controller.close();
});
test("refresh and teardown cancel reads and reject late completions", async () => {
  const { controller, adapter } = await setup();
  const gate = deferred<WorkspaceFilePage>();
  let signal: AbortSignal | undefined;
  adapter.readSourceFiles = async (request) => {
    signal = request.signal;
    return gate.promise;
  };
  controller.selectCommit("api2");
  expect(signal?.aborted).toBe(false);
  adapter.snapshot.revision++;
  await controller.refresh();
  expect(signal?.aborted).toBe(true);
  controller.close();
  gate.resolve({ files: [{ path: "late.ts", hunks: [] }] });
  await Promise.resolve();
  expect(controller.filesFor("api2")).toEqual([]);
});
test("closing one diff cancels its patch while the other stack remains open", async () => {
  const { controller, adapter } = await setup();
  for (const id of ["api2", "ui1"]) {
    controller.selectCommit(id);
    await vi.waitFor(() => expect(controller.filesFor(id)).toHaveLength(1));
  }
  const gate = deferred<WorkspaceFile>();
  const signals = new Map<string, AbortSignal>();
  adapter.readFileDiff = async (request) => {
    signals.set(request.source, request.signal!);
    return gate.promise;
  };
  controller.selectFile("api2", "src/api.ts");
  controller.selectFile("ui1", "src/toolbar.ts");
  controller.closeDiff("stack-api");
  expect(signals.get("api2")?.aborted).toBe(true);
  expect(signals.get("ui1")?.aborted).toBe(false);
  expect(controller.diffs.map((diff) => diff.origin)).toEqual(["stack-ui"]);
  controller.close();
  gate.resolve({ path: "src/api.ts", hunks: [] });
  await Promise.resolve();
});
test("read-only capabilities allow browsing and reject preview without invoking the executor", async () => {
  const { controller, adapter } = await setup();
  controller.selectStackFiles("api", "feature/api");
  expect(controller.selectedStack).toBe("api");
  expect(controller.can("apply_stack")).toBe(false);
  expect(controller.eligibility("apply_stack").reason).toBeTruthy();
  await controller.prepare({ kind: "apply_stack", stack: "api" });
  expect(controller.validation).toContain("available");
  expect(adapter.calls).toEqual([]);
  expect(controller.locked).toBe(false);
  controller.close();
});

test("file page retry retains the exact cursor and rejects duplicate and cyclic continuations", async () => {
  const { controller, adapter, calls } = await setup();
  adapter.readSourceFiles = async (request) => {
    calls.push(request);
    if (!request.cursor)
      return { files: [{ path: "first.ts", hunks: [] }], nextCursor: "next" };
    if (calls.length === 2) throw new Error("Retry this page");
    if (calls.length === 3) return { files: [{ path: "first.ts", hunks: [] }] };
    return { files: [{ path: "second.ts", hunks: [] }], nextCursor: "next" };
  };
  await controller.details.loadFiles("api2");
  for (let i = 0; i < 3; i++) await controller.details.loadFiles("api2", true);
  expect(calls.map((call) => call.cursor)).toEqual([
    undefined,
    "next",
    "next",
    "next",
  ]);
  expect(controller.filesFor("api2").map((file) => file.path)).toEqual([
    "first.ts",
  ]);
  expect(controller.details.fileState("api2").error).toContain("bound");
  controller.close();
});
test("opaque read-key changes invalidate loaded details even when the UI revision is unchanged", async () => {
  const { controller, adapter } = await setup();
  controller.selectCommit("api2");
  await vi.waitFor(() =>
    expect(controller.details.fileState("api2").loaded).toBe(true),
  );
  adapter.snapshot.readKey = "another-operation-and-binding";
  await controller.refresh();
  expect(controller.details.fileState("api2").loaded).toBe(false);
  expect(controller.filesFor("api2")).toEqual([]);
  await controller.details.loadFiles("api2");
  expect(controller.filesFor("api2")).toHaveLength(1);
  controller.close();
});
test("target-history selection loads only its files and listed patch with caller cancellation", async () => {
  const { controller, adapter, calls, patches } = await setup();
  adapter.readTargetHistory = async () => ({
    commits: [
      {
        ...cloneWorkspace(controller.snapshot!.stacks[0]!.commits[0]!),
        id: "target-commit",
        files: [],
      },
    ],
  });
  adapter.readSourceFiles = async (request) => {
    calls.push(request);
    return {
      files: [
        { path: "target.ts", hunks: [], content: { status: "unloaded" } },
      ],
    };
  };
  const gate = deferred<WorkspaceFile>();
  adapter.readFileDiff = async (request) => {
    patches.push(request);
    return gate.promise;
  };
  await controller.selectTargetBranch();
  controller.targetHistory.selectCommit("target-commit");
  await vi.waitFor(() =>
    expect(controller.filesFor("target-commit")).toHaveLength(1),
  );
  expect(patches).toEqual([]);
  controller.targetHistory.selectFile("target-commit", "target.ts");
  expect(patches).toHaveLength(1);
  controller.closeDiff("target-history");
  expect(patches[0]!.signal!.aborted).toBe(true);
  gate.resolve({ path: "target.ts", hunks: [] });
  await Promise.resolve();
  expect(controller.targetHistory.diff).toBeNull();
  controller.close();
});
test("loaded adjacent diffs retain their paths across refresh and remove missing files only after a complete summary", async () => {
  const { controller, adapter } = await setup();
  for (const [source, path] of [
    ["api2", "src/api.ts"],
    ["ui1", "src/toolbar.ts"],
  ]) {
    controller.selectCommit(source!);
    await vi.waitFor(() =>
      expect(controller.details.fileState(source!).loaded).toBe(true),
    );
    controller.selectFile(source!, path!);
  }
  adapter.snapshot.revision++;
  await controller.refresh();
  expect(controller.diffs.map((diff) => diff.path)).toEqual([
    "src/api.ts",
    "src/toolbar.ts",
  ]);
  adapter.readSourceFiles = async () => ({ files: [], nextCursor: undefined });
  await controller.details.loadFiles("api2");
  expect(controller.diffs.map((diff) => diff.path)).toEqual(["src/toolbar.ts"]);
  controller.close();
});

test("oversized file pages are rejected without installing partial files", async () => {
  const { controller, adapter } = await setup();
  adapter.readSourceFiles = async () => ({
    files: Array.from({ length: 51 }, (_, index) => ({
      path: `file-${index}`,
      hunks: [],
    })),
  });
  await controller.details.loadFiles("api2");
  expect(controller.details.fileState("api2").error).toContain("bound");
  expect(controller.filesFor("api2")).toEqual([]);
  controller.close();
});
test("target-history reads abort when only the opaque read key changes", async () => {
  const { controller, adapter } = await setup();
  const gate = deferred<{ commits: never[] }>();
  let signal: AbortSignal | undefined;
  adapter.readTargetHistory = async (_request, callerSignal?: AbortSignal) => {
    signal = callerSignal;
    return gate.promise;
  };
  const loading = controller.selectTargetBranch();
  expect(signal?.aborted).toBe(false);
  const oldSignal = signal;
  adapter.snapshot.readKey = "changed-provider-pin";
  await controller.refresh();
  expect(oldSignal?.aborted).toBe(true);
  expect(signal).not.toBe(oldSignal);
  // The active history immediately started a new read.
  expect(controller.targetHistory.loading).toBe(true);
  gate.resolve({ commits: [] });
  await loading;
  controller.close();
  expect(signal?.aborted).toBe(true);
});

test("loaded Unstaged files become draft selections without eager patches", async () => {
  const { controller, adapter } = await setup();
  adapter.readSourceFiles = async () => ({
    files: [{ path: "draft.txt", hunks: [], content: { status: "unloaded" } }],
  });
  await controller.details.loadFiles("unassigned");
  expect(
    controller.snapshot!.workspaces.find(
      (w) => w.id === controller.snapshot!.active,
    )!.unassigned,
  ).toHaveLength(1);
  const stack = controller.snapshot!.workspaces.find(
    (w) => w.id === controller.snapshot!.active,
  )!.applied[0]!;
  expect(
    controller.commitDrafts.open(controller.snapshot!, stack, {
      source: "unassigned",
      path: "draft.txt",
    }),
  ).toBe(true);
  expect(
    controller.commitDrafts.get(controller.snapshot!.active!, stack)!
      .selections,
  ).toEqual([{ source: "unassigned", path: "draft.txt" }]);
  controller.close();
});

test("same-scope refresh and history retain loaded file selections", async () => {
  const { controller, adapter, original } = await setup();
  adapter.snapshot.capabilities = ["create_commit", "transfer"];
  const stack = adapter.snapshot.stacks[0]!,
    commit = stack.commits.at(-1)!;
  commit.immutable = false;
  await controller.refresh();
  const readFiles = adapter.readSourceFiles;
  adapter.readSourceFiles = async (request) =>
    request.source === "unassigned"
      ? { files: [{ path: "draft.txt", hunks: [] }] }
      : readFiles(request);
  await controller.details.loadFiles("unassigned");
  await controller.details.loadFiles(commit.id);
  const assertSelections = () => {
    for (const selection of [
      { source: "unassigned", path: "draft.txt" },
      { source: commit.id, path: controller.filesFor(commit.id)[0]!.path },
    ]) {
      expect(
        workspaceDropFeedback(
          controller.snapshot,
          JSON.stringify({ kind: "selection", selection }),
          { kind: "stage", id: stack.id },
        ).state,
      ).toBe("valid");
      expect(
        controller.commitDrafts.open(controller.snapshot!, stack.id, selection),
      ).toBe(true);
    }
  };
  assertSelections();
  await controller.refresh();
  assertSelections();
  Object.assign(adapter, {
    readStackHistory: async () => ({
      commits: cloneWorkspace(original.stacks[0]!.commits)
        .reverse()
        .map((item) => ({ ...item, files: [], immutable: false })),
    }),
  });
  await controller.histories.load(stack.id);
  assertSelections();
  controller.close();
});

test("loaded commit files and patches become reusable draft selections", async () => {
  const { controller, patches } = await setup();
  const snapshot = controller.snapshot!,
    stack = snapshot.stacks[0]!,
    commit = stack.commits.at(-1)!;
  await controller.details.loadFiles(commit.id);
  expect(commit.files.length).toBeGreaterThan(0);
  expect(patches).toHaveLength(0);
  const path = commit.files[0]!.path;
  expect(
    controller.commitDrafts.open(snapshot, stack.id, {
      source: commit.id,
      path,
    }),
  ).toBe(true);
  await controller.details.loadDiff(commit.id, path);
  expect(commit.files[0]!.hunks.length).toBeGreaterThan(0);
  expect(patches).toHaveLength(1);
  controller.close();
});
test("accepted lazy native file rows can drag whole files before patches are loaded", async () => {
  const { controller, original } = await setup();
  const commit = original.stacks[0]!.commits[0]!;
  controller.snapshot!.capabilities = ["transfer", "create_commit"];
  controller.snapshot!.stacks[0]!.commits[0]!.immutable = false;
  await controller.details.loadFiles(commit.id);
  const bindings = workspaceFileDrag(controller, () => commit.id);
  const file = controller.filesFor(commit.id)[0]!;
  expect(bindings.fileDraggable?.({ path: file.path } as never)).toBe(true);
  const data = { setData: vi.fn(), effectAllowed: "" };
  bindings.onFileDragStart?.(
    {
      dataTransfer: data,
      preventDefault: vi.fn(),
      stopPropagation: vi.fn(),
    } as never,
    { path: file.path } as never,
  );
  expect(data.setData).toHaveBeenCalled();
  const net = workspaceFileDrag(controller, () =>
    workspaceBranchSource(
      controller.snapshot!.stacks[0]!.id,
      controller.snapshot!.stacks[0]!.name,
    ),
  );
  expect(net.fileDraggable?.({ path: file.path } as never)).toBe(false);
  controller.close();
});
