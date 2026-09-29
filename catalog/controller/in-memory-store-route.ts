import type { StoreRoute } from "../../web/src/lib/controller/contracts.js";
import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
import { IDS } from "../fixtures/repository.js";
import type { InMemoryMutationRouteChange } from "./in-memory-store-repository.svelte.js";

export function initialStoreRoute(page: StorePage): StoreRoute {
  return {
    page,
    repository: "jj-store",
    bookmark: page === "files" ? "main" : "",
    snapshotOperationId: IDS.opLeft,
    inspectedOperationId: null,
    commitId: null,
    changeId: null,
    comparisonParentId: null,
    comparisonGroupId: null,
    path: null,
    directory: "",
    agentSessionId: null,
    reviewThreadId: null,
  };
}

export function mutationCommittedRoute(
  route: StoreRoute,
  change: InMemoryMutationRouteChange,
): StoreRoute | null {
  if (change.replacementCommit)
    return { ...route, commitId: change.replacementCommit, path: null };
  if (change.createdCommit)
    return { ...route, commitId: change.createdCommit, path: null };
  if (change.deletedBookmark === route.bookmark)
    return { ...route, bookmark: "", path: null };
  return null;
}

export function revisionSelectedRoute(
  route: StoreRoute,
  commitId: string | null,
  changeId: string | null,
  context: Pick<StoreRoute, "agentSessionId" | "reviewThreadId">,
): StoreRoute {
  const operation = route.page === "oplog";
  return {
    ...route,
    inspectedOperationId: operation ? commitId : null,
    commitId: operation ? null : commitId,
    changeId: operation ? null : changeId,
    comparisonParentId: null,
    comparisonGroupId: null,
    path: null,
    directory: "",
    ...context,
  };
}
