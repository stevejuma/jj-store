import { describe, expect, it, vi } from "vitest";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import { TARGET_HISTORY_PAGE_SIZE } from "../../src/lib/controller/stack-workspace/target-history.svelte.js";
import type {
  WorkspaceHistoryPage,
  WorkspaceHistoryRequest,
} from "../../src/lib/controller/stack-workspace/types.js";

async function fixture() {
  const adapter = new MockStackWorkspaceAdapter();
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  return { adapter, controller, history: controller.targetHistory };
}
function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((done) => {
    resolve = done;
  });
  return { promise, resolve };
}

describe("workspace target branch history", () => {
  it("loads only when selected, pages the pinned target to exhaustion and never changes applied stacks", async () => {
    const { adapter, controller, history } = await fixture();
    expect(adapter.historyCalls).toHaveLength(0);
    const before = JSON.stringify(controller.snapshot);
    controller.selectFile("unassigned", "README.md");
    await controller.selectTargetBranch();
    expect(controller.selection).toBeNull();
    expect(history.commits).toHaveLength(TARGET_HISTORY_PAGE_SIZE);
    expect(adapter.historyCalls[0]).toEqual({
      repository: "local-demo",
      revision: 1,
      branch: "main",
      head: "main-history-240",
      limit: 40,
    });
    await controller.selectTargetBranch();
    expect(adapter.historyCalls).toHaveLength(1);
    while (history.hasMore) await history.loadMore();
    await history.loadMore();
    expect(adapter.historyCalls.map((call) => call.cursor)).toEqual([
      undefined,
      "40",
      "80",
      "120",
      "160",
      "200",
    ]);
    expect(history.commits.map((commit) => commit.id)).toEqual(
      adapter.targetCommits.map((commit) => commit.id),
    );
    expect(history.total).toBe(240);
    expect(history.commits.every((commit) => commit.immutable)).toBe(true);
    expect(JSON.stringify(controller.snapshot)).toBe(before);
    expect(adapter.calls).toHaveLength(0);
    controller.close();
  });
  it("keeps target commit/files/diff separate from ordinary stack selections and caches when revisited", async () => {
    const { adapter, controller, history } = await fixture();
    controller.selectStack("api");
    controller.selectCommit("api2");
    controller.selectFile("api2", "src/api.ts");
    await controller.selectTargetBranch();
    const commit = history.commits[0]!;
    history.selectCommit(commit.id);
    history.selectFile(commit.id, commit.files[0]!.path);
    expect(history.selectedId).toBe(commit.id);
    expect(history.diff).toEqual({
      origin: "target-history",
      source: commit.id,
      path: commit.files[0]!.path,
    });
    expect(controller.diffs[0]?.origin).toBe("stack-api");
    expect(controller.selection).toBeNull();
    history.selectCommit("missing");
    history.selectFile(commit.id, "missing");
    expect(history.selectedId).toBe(commit.id);
    controller.selectStack("api", "feature/model");
    expect(history.active).toBe(false);
    await controller.selectTargetBranch();
    expect(history.selectedId).toBe(commit.id);
    expect(adapter.historyCalls).toHaveLength(1);
    controller.close();
  });
  it("deduplicates concurrent page requests and repeated commit identities", async () => {
    const { adapter, controller, history } = await fixture();
    await controller.selectTargetBranch();
    const read = adapter.readTargetHistory.bind(adapter);
    const page = deferred<WorkspaceHistoryPage>();
    adapter.readTargetHistory = () => page.promise;
    const loading = history.loadMore();
    await history.loadMore();
    expect(history.loading).toBe(true);
    const result = await read({ ...adapter.historyCalls[0]!, cursor: "40" });
    page.resolve({
      ...result,
      commits: [history.commits[0]!, ...result.commits.slice(1)],
    });
    await loading;
    expect(history.commits).toHaveLength(79);
    expect(new Set(history.commits.map((commit) => commit.id)).size).toBe(79);
    controller.close();
  });
  it("invalidates late pages after refresh to another target head and after teardown", async () => {
    const { adapter, controller, history } = await fixture();
    const old = deferred<WorkspaceHistoryPage>();
    const read = adapter.readTargetHistory.bind(adapter);
    adapter.readTargetHistory = (request: WorkspaceHistoryRequest) =>
      request.revision === 1 ? old.promise : read(request);
    const selecting = controller.selectTargetBranch();
    adapter.snapshot.revision++;
    adapter.snapshot.target.commit = "main-history-239";
    adapter.targetCommits.shift();
    await controller.refresh();
    await vi.waitFor(() => expect(history.commits).toHaveLength(40));
    old.resolve({ commits: [adapter.targetCommits.at(-1)!] });
    await selecting;
    expect(history.branch?.commit).toBe("main-history-239");
    expect(history.commits[0]?.id).toBe("main-history-239");
    expect(history.commits).toHaveLength(40);
    const late = deferred<WorkspaceHistoryPage>();
    adapter.readTargetHistory = () => late.promise;
    const more = history.loadMore();
    controller.close();
    late.resolve({ commits: [adapter.targetCommits.at(-1)!] });
    await more;
    expect(history.commits).toHaveLength(40);
  });
  it("retains loaded history and retries the exact cursor after a failed read", async () => {
    const { adapter, controller, history } = await fixture();
    await controller.selectTargetBranch();
    const read = adapter.readTargetHistory.bind(adapter);
    adapter.readTargetHistory = async () => {
      throw new Error("Temporary read failure");
    };
    await history.loadMore();
    expect(history.error).toBe("Temporary read failure");
    expect(history.commits).toHaveLength(40);
    adapter.readTargetHistory = read;
    await history.loadMore();
    expect(adapter.historyCalls.at(-1)?.cursor).toBe("40");
    expect(history.error).toBe("");
    expect(history.commits).toHaveLength(80);
    controller.close();
  });
  it("rejects oversized pages and non-advancing cursors without accepting data", async () => {
    const { adapter, controller, history } = await fixture();
    adapter.readTargetHistory = async () => ({
      commits: adapter.targetCommits.slice(0, 41),
      nextCursor: "41",
    });
    await controller.selectTargetBranch();
    expect(history.error).toContain("bound");
    expect(history.commits).toHaveLength(0);
    adapter.readTargetHistory = async () => ({
      commits: adapter.targetCommits.slice(0, 40),
      nextCursor: "40",
    });
    await history.loadMore();
    adapter.readTargetHistory = async () => ({
      commits: adapter.targetCommits.slice(40, 80),
      nextCursor: "40",
    });
    await history.loadMore();
    expect(history.error).toContain("advance");
    expect(history.commits).toHaveLength(40);
    controller.close();
  });
  it("does not activate while a command preview is retained", async () => {
    const { adapter, controller, history } = await fixture();
    await controller.prepare({ kind: "apply_stack", stack: "search" });
    await controller.selectTargetBranch();
    expect(history.active).toBe(false);
    expect(adapter.historyCalls).toHaveLength(0);
    controller.close();
  });
});
