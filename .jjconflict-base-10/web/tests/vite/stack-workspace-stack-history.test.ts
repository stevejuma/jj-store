import { expect, test, vi } from "vitest";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import { stackGroups } from "../../src/lib/controller/stack-workspace/presentation.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import {
  cloneWorkspace,
  type StackWorkspaceAdapter,
  type WorkspaceHistoryPage,
} from "../../src/lib/controller/stack-workspace/types.js";

function fixture() {
  const snapshot = workspaceFixture();
  const stack = snapshot.stacks[0]!,
    commits = [...stack.commits].reverse();
  snapshot.capabilities = [];
  snapshot.readKey = "operation-one";
  stack.heads = [stack.heads[0]!];
  stack.commits = [];
  const adapter: StackWorkspaceAdapter = {
    read: async () => cloneWorkspace(snapshot),
    readTargetHistory: async () => ({ commits: [] }),
    readStackHistory: async () => ({ commits }),
    preview: async () => {
      throw new Error("No executor");
    },
    apply: async () => ({ status: "unknown" }),
    status: async () => ({ status: "unknown" }),
  };
  return { snapshot, stack, commits, adapter };
}
test("selected stack history installs explicit bounded segments without mock slicing", async () => {
  const { snapshot, stack, commits, adapter } = fixture();
  const read = vi.fn(adapter.readStackHistory!);
  adapter.readStackHistory = read;
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  expect(read).not.toHaveBeenCalled();
  controller.selectStack(stack.id);
  await vi.waitFor(() =>
    expect(controller.histories.state(stack.id).loaded).toBe(true),
  );
  expect(read.mock.calls[0]![0]).toMatchObject({
    repository: snapshot.repository,
    revision: snapshot.revision,
    readKey: snapshot.readKey,
    branch: stack.heads[0]!.name,
    head: stack.heads[0]!.commit,
    base: snapshot.target.commit,
    limit: 40,
  });
  const loaded = controller.snapshot!.stacks[0]!;
  expect(loaded.segments?.[0]?.baseCommit).toBe(snapshot.target.commit);
  expect(stackGroups(loaded)[0]!.commits.map((commit) => commit.id)).toEqual(
    commits.map((commit) => commit.id),
  );
  controller.close();
});
test("stack continuation is bounded and retry retains its original cursor", async () => {
  const { stack, commits, adapter } = fixture();
  const read = vi
    .fn()
    .mockResolvedValueOnce({ commits: [commits[0]], nextCursor: "page-two" })
    .mockRejectedValueOnce(new Error("temporary"))
    .mockResolvedValueOnce({ commits: commits.slice(1) });
  adapter.readStackHistory = read;
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  controller.selectStack(stack.id);
  await vi.waitFor(() =>
    expect(controller.histories.state(stack.id).loaded).toBe(true),
  );
  await controller.histories.load(stack.id, true);
  expect(controller.histories.state(stack.id).error).toBe("temporary");
  await controller.histories.load(stack.id, true);
  expect(read.mock.calls.slice(1).map((call) => call[0].cursor)).toEqual([
    "page-two",
    "page-two",
  ]);
  expect(controller.histories.state(stack.id).hasMore).toBe(false);
  controller.close();
});
test.each(["refresh", "close"] as const)(
  "%s cancels history and rejects late rows",
  async (action) => {
    const { snapshot, stack, commits, adapter } = fixture();
    let finish!: (page: WorkspaceHistoryPage) => void, signal!: AbortSignal;
    adapter.readStackHistory = (_request, input) => {
      signal = input!;
      return new Promise((resolve) => {
        finish = resolve;
      });
    };
    const controller = new StackWorkspaceController(adapter);
    await controller.refresh();
    controller.selectStack(stack.id);
    if (action === "refresh") {
      snapshot.readKey = "operation-two";
      await controller.refresh();
    } else controller.close();
    expect(signal.aborted).toBe(true);
    finish({ commits });
    await Promise.resolve();
    await Promise.resolve();
    expect(controller.snapshot!.stacks[0]!.commits).toEqual([]);
    controller.close();
  },
);
