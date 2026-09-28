import type { createProductionStoreReader } from "./production-store-reader.svelte.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

type Reader = ReturnType<typeof createProductionStoreReader>;
type History = ReturnType<typeof createStoreRouteHistory>;

function resetBranchGraph(data: ProductionStoreState) {
  Object.assign(data, {
    graph: { revisions: [], devices: [] },
    graphCursor: null,
    graphAtTimeMillis: null,
    graphPageSnapshot: null,
    detail: undefined,
    comparisons: new Map(),
    inspectionContinuation: null,
  });
}

export function createProductionBranchActions(context: {
  data: ProductionStoreState;
  history: History;
  reader: Reader;
  invalidate(): void;
}) {
  const { data, history, reader } = context;
  return {
    async selectBookmark(name: string) {
      const bookmark = data.bookmarks.find((item) => item.name === name);
      if (!bookmark) return;
      context.invalidate();
      data.targets = [...bookmark.targets];
      history.push({
        ...history.route,
        bookmark: name,
        commitId: null,
        changeId: null,
        path: null,
      });
      resetBranchGraph(data);
      await reader.loadGraph();
    },
    async selectWorkingCopy(commit: string) {
      const workingCopy = data.workingCopies.find(
        (candidate) => candidate.commitId === commit,
      );
      if (!workingCopy || history.route.page !== "branches") return;
      context.invalidate();
      data.targets = [workingCopy.commitId];
      history.push({
        ...history.route,
        bookmark: "",
        commitId: workingCopy.commitId,
        changeId: workingCopy.changeId,
        path: null,
      });
      resetBranchGraph(data);
      await Promise.all([
        reader.loadGraph(),
        reader.loadCommit(workingCopy.commitId),
      ]);
    },
  };
}
