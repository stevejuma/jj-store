import type { StoreRoute } from "./contracts.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { RepositoryWorkingCopy } from "../projects/repository-log.js";
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

/** References are paged by changes, so a workspace can occur on a later page. */
export async function readWorkingCopies(
  source: StoreDataSource,
  operation: string,
  firstPage?: Awaited<ReturnType<typeof readReferences>>,
) {
  const copies = new Map<string, RepositoryWorkingCopy>();
  const seen = new Set<string>();
  let page = firstPage ?? (await readReferences(source, operation));
  const snapshot = page.snapshot;
  while (true) {
    for (const copy of page.workingCopies) copies.set(copy.workspace, copy);
    if (!page.cursor) break;
    if (seen.has(page.cursor))
      throw new Error("Repeated workspace discovery cursor.");
    seen.add(page.cursor);
    page = await readReferences(source, operation, page.cursor, snapshot);
  }
  return [...copies.values()].sort((a, b) =>
    a.workspace.localeCompare(b.workspace),
  );
}
