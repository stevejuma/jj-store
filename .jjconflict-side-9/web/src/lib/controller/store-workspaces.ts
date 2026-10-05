import type { StoreRoute } from "./contracts.js";
import type { StoreDataSource } from "./native-data-source.js";
import { readReferences } from "./production-store-loaders.js";

export function workspaceSelectedRoute(
  route: StoreRoute,
  workspace: string | null,
): StoreRoute {
  return {
    ...route,
    workspace,
    bookmark: "",
    commitId: null,
    evolutionCommitId: null,
    changeId: null,
    inspectedOperationId: null,
    comparisonParentId: null,
    comparisonGroupId: null,
    path: null,
    directory: "",
    agentSessionId: null,
    reviewThreadId: null,
  };
}

/** Workspaces have their own bounded, operation-pinned windows. */
export async function readWorkingCopies(
  source: StoreDataSource,
  operation: string,
  cursor?: string,
  expected?: import("./repository-profile.js").CodeSnapshot,
  signal?: AbortSignal,
) {
  return readReferences(source, operation, cursor, expected, {
    kind: "workspaces",
    ...(signal ? { signal } : {}),
  });
}
