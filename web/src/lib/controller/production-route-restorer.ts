import type { StoreRoute } from "./contracts.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

interface RouteReads {
  loadReferences(): Promise<void>;
  loadGraph(): Promise<void>;
  loadEvolution(more?: boolean): Promise<void>;
  loadFiles(
    commit: string,
    options?: { path?: string; recursive?: boolean; more?: boolean },
  ): Promise<boolean>;
  loadBlob(path: string): Promise<void>;
  loadCommit(id: string): Promise<void>;
  loadOperation(id: string, parent?: string): Promise<void>;
  loadPatch(path: string): Promise<void>;
  loadAgents(): Promise<void>;
  loadAgent(id: string): Promise<void>;
  loadReviews(): Promise<void>;
  loadReview(id: string): Promise<void>;
}

function unavailable(
  data: ProductionStoreState,
  area: "files" | "detail",
  message: string,
) {
  data.status[area] = "unavailable";
  data.status.error = message;
}

async function restoreFiles(
  desired: StoreRoute,
  data: ProductionStoreState,
  history: ReturnType<typeof createStoreRouteHistory>,
  reads: RouteReads,
) {
  const workingCopy = data.workingCopies.find(
    (copy) => copy.workspace === desired.workspace,
  );
  const choice = desired.bookmark
    ? data.choices.find(
        (candidate) =>
          candidate.kind === "bookmark" && candidate.label === desired.bookmark,
      )
    : desired.changeId
      ? data.choices.find(
          (candidate) => candidate.value === `change:${desired.changeId}`,
        )
      : undefined;
  if ((desired.bookmark || desired.changeId) && !choice) {
    unavailable(
      data,
      "files",
      "The requested repository reference is unavailable.",
    );
    return;
  }
  const fallback = desired.workspace
    ? undefined
    : data.choices.find((candidate) => candidate.targets.length === 1);
  const selected = choice ?? (desired.commitId ? undefined : fallback);
  data.targets = selected
    ? [...selected.targets]
    : desired.commitId
      ? [desired.commitId]
      : [];
  if (
    desired.commitId &&
    selected &&
    !selected.targets.includes(desired.commitId)
  ) {
    unavailable(
      data,
      "files",
      "The requested commit is not a target of this repository reference.",
    );
    return;
  }
  const commit =
    desired.commitId ??
    (!choice ? workingCopy?.commitId : null) ??
    (selected?.targets.length === 1 ? selected.targets[0]! : null);
  if (!commit) {
    data.status.files = data.targets.length > 1 ? "unavailable" : "empty";
    return;
  }
  data.targets = [commit];
  const next = {
    ...desired,
    bookmark: selected?.kind === "bookmark" ? selected.label : desired.bookmark,
    changeId:
      selected?.kind === "change"
        ? selected.value.slice("change:".length)
        : desired.changeId,
    commitId: desired.workspace && !choice && !desired.commitId ? null : commit,
  };
  history.replace(next);
  await reads.loadFiles(commit);
  if (desired.directory)
    await reads.loadFiles(commit, {
      path: desired.directory,
      recursive: false,
    });
  if (!desired.path) return;
  if (!data.snapshot?.files.some((file) => file.path === desired.path)) {
    unavailable(data, "detail", "The requested file is unavailable.");
    return;
  }
  await reads.loadBlob(desired.path);
}

export async function restoreProductionStoreRoute(
  desired: StoreRoute,
  data: ProductionStoreState,
  history: ReturnType<typeof createStoreRouteHistory>,
  reads: RouteReads,
) {
  data.status.error = "";
  data.status.navigation = "loading";
  await reads.loadReferences();
  if (history.route !== desired) return;
  if (data.status.bookmarks === "failed") {
    data.status.navigation = "failed";
    return;
  }
  if (
    desired.workspace &&
    !data.workingCopies.some((copy) => copy.workspace === desired.workspace)
  ) {
    unavailable(
      data,
      "detail",
      "The requested workspace is unavailable at this operation snapshot.",
    );
    data.status.navigation = "failed";
    return;
  }
  if (desired.page === "agents") {
    await Promise.all([reads.loadAgents(), reads.loadReviews()]);
    if (desired.agentSessionId) await reads.loadAgent(desired.agentSessionId);
    if (desired.reviewThreadId) await reads.loadReview(desired.reviewThreadId);
  } else if (desired.page === "reviews") {
    await reads.loadReviews();
    if (desired.reviewThreadId) await reads.loadReview(desired.reviewThreadId);
  } else if (desired.page === "files")
    await restoreFiles(desired, data, history, reads);
  else {
    if (desired.bookmark) {
      const bookmark = data.bookmarks.find(
        (candidate) => candidate.name === desired.bookmark,
      );
      if (!bookmark) {
        unavailable(data, "detail", "The requested bookmark is unavailable.");
        data.status.navigation = "failed";
        return;
      }
      data.targets = [...bookmark.targets];
    } else if (desired.page === "branches" && desired.commitId) {
      const workingCopy = data.workingCopies.find(
        (candidate) => candidate.commitId === desired.commitId,
      );
      if (workingCopy) data.targets = [workingCopy.commitId];
    }
    await reads.loadGraph();
    if (desired.evolutionCommitId) await reads.loadEvolution();
    const selected =
      desired.page === "oplog"
        ? desired.inspectedOperationId
        : desired.commitId;
    if (selected && desired.page === "oplog")
      await reads.loadOperation(
        selected,
        desired.comparisonParentId ?? undefined,
      );
    else if (selected) await reads.loadCommit(selected);
    if (desired.path && selected) await reads.loadPatch(desired.path);
    if (desired.agentSessionId) await reads.loadAgent(desired.agentSessionId);
    if (desired.reviewThreadId) await reads.loadReview(desired.reviewThreadId);
  }
  data.status.navigation = data.status.error ? "failed" : "ready";
}
