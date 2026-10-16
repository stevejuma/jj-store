import type { createProductionStoreReader } from "./production-store-reader.svelte.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

type Reader = ReturnType<typeof createProductionStoreReader>;
type History = ReturnType<typeof createStoreRouteHistory>;

function resetBranchGraph(data: ProductionStoreState) {
  // Selection replaces route restoration; graph/detail retain their own busy state.
  data.status.navigation = "ready";
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
      const previous = history.route;
      const next = {
        ...previous,
        bookmark: name,
        commitId: null,
        evolutionCommitId: null,
        changeId: null,
        path: null,
      };
      history.push(next);
      if (history.route === previous) return;
      context.invalidate();
      data.targets = [...bookmark.targets];
      resetBranchGraph(data);
      void reader.loadReferencePickers();
      await reader.loadGraph();
    },
    async selectWorkingCopy(commit: string, workspace?: string) {
      const workingCopy = data.workingCopies.find((candidate) =>
        workspace
          ? candidate.workspace === workspace
          : candidate.commitId === commit,
      );
      if (!workingCopy || history.route.page !== "branches") return;
      const previous = history.route;
      const next = {
        ...previous,
        bookmark: "",
        workspace: workingCopy.workspace,
        commitId: workingCopy.commitId,
        evolutionCommitId: null,
        changeId: workingCopy.changeId,
        path: null,
      };
      history.push(next);
      if (history.route === previous) return;
      context.invalidate();
      data.targets = [workingCopy.commitId];
      resetBranchGraph(data);
      void reader.loadReferencePickers();
      await Promise.all([
        reader.loadGraph(),
        reader.loadCommit(workingCopy.commitId),
      ]);
    },
  };
}
