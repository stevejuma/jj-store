import { ApiError, type Model } from "../api/client.js";
import type { StoreRoute, StoreSearchResult } from "./contracts.js";

const seconds = (millis: number | bigint) => Number(BigInt(millis) / 1000n);
const short = (value: string | null) => value?.slice(0, 12) ?? "unavailable";

export function presentSearchResults(
  documents: readonly Model<"SearchDocument">[],
): StoreSearchResult[] {
  return documents.map((document) => {
    const kind = document.key.kind;
    const path = kind === "file" ? document.key.path : null;
    const scope = kind === "file" ? document.key.scope : null;
    const nativeId = kind === "file" ? document.file : document.key.id;
    const title = document.title.trim() || path || `Untitled ${kind}`;
    const description =
      kind === "file"
        ? `${scope} · commit ${short(document.commit)}`
        : kind === "commit"
          ? `${document.author || "Unknown author"} · ${short(document.change)}`
          : `${document.actor || document.author || "Unknown actor"} · operation`;
    return {
      id: `${kind}:${nativeId ?? `${scope}:${path}`}`,
      kind,
      title,
      description,
      author: document.author,
      timestamp: seconds(document.timestamp_millis),
      operationId: kind === "operation" ? document.key.id : document.operation,
      commitId: kind === "commit" ? document.key.id : document.commit,
      changeId: document.change,
      scope,
      path,
      hasConflict: document.has_conflict,
    };
  });
}

export function assertSearchPage(
  page: Model<"SearchPage">,
  repository: string,
  operation: string,
): void {
  if (
    page.repository !== repository ||
    page.selected_view?.repo_id !== repository ||
    page.selected_view.operation_id !== operation
  )
    throw new ApiError("snapshot_mismatch");
}

export function routeForSearchResult(
  current: StoreRoute,
  result: StoreSearchResult,
): StoreRoute {
  const base = {
    ...current,
    bookmark: "",
    inspectedOperationId: null,
    commitId: null,
    changeId: null,
    comparisonParentId: null,
    comparisonGroupId: null,
    path: null,
    directory: "",
  };
  if (result.kind === "operation")
    return {
      ...base,
      page: "oplog",
      inspectedOperationId: result.operationId,
    };
  if (result.kind === "commit")
    return {
      ...base,
      page: "log",
      commitId: result.commitId,
      changeId: result.changeId,
    };
  return {
    ...base,
    page: "files",
    bookmark: result.scope ?? "",
    commitId: result.commitId,
    changeId: result.changeId,
    path: result.path,
    directory: result.path?.split("/").slice(0, -1).join("/") ?? "",
  };
}
