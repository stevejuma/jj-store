import { expect, test, vi } from "vitest";
import { createProductionBranchActions } from "../../src/lib/controller/production-branch-actions.js";
import { createProductionRevisionSelector } from "../../src/lib/controller/production-store-lifecycle.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";

const repository = "ab".repeat(16),
  operation = "cd".repeat(64),
  commit = "12".repeat(64);

function fixture() {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory(undefined);
  history.connect(repository, operation);
  history.replace({ ...history.route, page: "branches" });
  // Browser reactivity wraps stored routes; raw pushed object identity is not
  // an acceptance signal. Model that boundary in the server-transformed unit.
  const push = history.push;
  history.push = (next) => push(new Proxy(next, {}));
  data.status.navigation = "loading";
  return { data, history };
}

test.each(["bookmark", "working copy"])(
  "%s selection retires interrupted restoration while its own reads stay busy",
  async (kind) => {
    const { data, history } = fixture();
    data.bookmarks = [
      { name: "main", targets: [commit] },
    ] as typeof data.bookmarks;
    data.workingCopies = [
      { workspace: "demo", commitId: commit },
    ] as typeof data.workingCopies;
    let finish!: () => void;
    const loadGraph = vi.fn(() => {
      data.status.graph = "loading";
      return new Promise<void>((resolve) => {
        finish = resolve;
      });
    });
    const actions = createProductionBranchActions({
      data,
      history,
      invalidate() {},
      reader: {
        loadGraph,
        loadCommit: vi.fn(),
        loadReferencePickers: vi.fn(),
      } as never,
    });
    const pending =
      kind === "bookmark"
        ? actions.selectBookmark("main")
        : actions.selectWorkingCopy(commit, "demo");
    expect(data.status.navigation).toBe("ready");
    expect(data.status.graph).toBe("loading");
    history.page("projects");
    data.status.navigation = "loading";
    finish();
    await pending;
    expect(data.status.navigation).toBe("loading");
    history.destroy();
  },
);

test("revision selection retires interrupted restoration without clearing a later navigation", async () => {
  const { data, history } = fixture();
  let finish!: () => void;
  const loadRevisionContext = vi.fn();
  const select = createProductionRevisionSelector(data, history, {
    loadCommit() {
      data.status.detail = "loading";
      return new Promise<void>((resolve) => {
        finish = resolve;
      });
    },
    loadRevisionContext,
  } as never);
  const pending = select(commit);
  expect(data.status.navigation).toBe("ready");
  expect(data.status.detail).toBe("loading");
  expect(loadRevisionContext).toHaveBeenCalledWith(commit);
  history.page("projects");
  data.status.navigation = "loading";
  finish();
  await pending;
  expect(data.status.navigation).toBe("loading");
  history.destroy();
});

test("guarded selections retain the current loading owner and do not start reads", async () => {
  const { data, history } = fixture();
  const retained = history.route;
  history.setNavigationGuard("pending", () => "Resolve the original action");
  data.bookmarks = [
    { name: "main", targets: [commit] },
  ] as typeof data.bookmarks;
  data.workingCopies = [
    { workspace: "demo", commitId: commit },
  ] as typeof data.workingCopies;
  const loadGraph = vi.fn(),
    loadCommit = vi.fn(),
    invalidate = vi.fn(),
    loadReferencePickers = vi.fn();
  const reader = {
    loadGraph,
    loadCommit,
    loadRevisionContext: vi.fn(),
    loadReferencePickers,
  } as never;
  const actions = createProductionBranchActions({
    data,
    history,
    reader,
    invalidate,
  });
  await actions.selectBookmark("main");
  await actions.selectWorkingCopy(commit, "demo");
  await createProductionRevisionSelector(data, history, reader)(commit);
  expect(history.route).toBe(retained);
  expect(data.status.navigation).toBe("loading");
  expect(data.targets).toEqual([]);
  expect(loadGraph).not.toHaveBeenCalled();
  expect(loadCommit).not.toHaveBeenCalled();
  expect(invalidate).not.toHaveBeenCalled();
  expect(loadReferencePickers).not.toHaveBeenCalled();
  history.destroy();
});
