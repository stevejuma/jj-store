import type { StorePage } from "../ui/community-navigation/model.js";
import type { StoreRoute } from "./contracts.js";

const repositoryId = /^[0-9a-f]{32}$/u;
const objectId = /^[0-9a-f]{128}$/u;
const changeId = /^[0-9a-f]{32}$/u;
const stableId = /^[0-9a-f]{32}$/u;
const pages = new Set<StorePage>([
  "branches",
  "log",
  "oplog",
  "files",
  "agents",
  "reviews",
]);
const queryKeys = new Set([
  "at_op",
  "bookmark",
  "commit",
  "change",
  "operation",
  "parent",
  "group",
  "file",
  "directory",
  "session",
  "thread",
]);

export type StoreRouteHashResult =
  | { kind: "none" }
  | { kind: "invalid"; message: string }
  | { kind: "route"; route: StoreRoute };

function safeLabel(value: string, maximum: number): boolean {
  return (
    value.length > 0 &&
    value.length <= maximum &&
    !/[\u0000-\u001f]/u.test(value)
  );
}

function safePath(value: string): boolean {
  if (!safeLabel(value, 4096) || value.startsWith("/") || value.includes("\\"))
    return false;
  return value
    .split("/")
    .every((part) => part && part !== "." && part !== "..");
}

function one(query: URLSearchParams, key: string): string | null | undefined {
  const values = query.getAll(key);
  return values.length > 1 ? undefined : (values[0] ?? null);
}

function validOptional(
  value: string | null | undefined,
  pattern: RegExp,
): value is string | null {
  return value !== undefined && (value === null || pattern.test(value));
}

export function storeRouteToHash(route: StoreRoute): string | null {
  if (!repositoryId.test(route.repository) || !pages.has(route.page))
    return null;
  const query = new URLSearchParams();
  if (route.snapshotOperationId) query.set("at_op", route.snapshotOperationId);
  if (route.bookmark) query.set("bookmark", route.bookmark);
  if (route.commitId) query.set("commit", route.commitId);
  if (route.changeId) query.set("change", route.changeId);
  if (route.inspectedOperationId)
    query.set("operation", route.inspectedOperationId);
  if (route.comparisonParentId) query.set("parent", route.comparisonParentId);
  if (route.comparisonGroupId) query.set("group", route.comparisonGroupId);
  if (route.path) query.set("file", route.path);
  if (route.directory) query.set("directory", route.directory);
  if (route.agentSessionId) query.set("session", route.agentSessionId);
  if (route.reviewThreadId) query.set("thread", route.reviewThreadId);
  const suffix = query.toString();
  return `#/repos/${route.repository}/${route.page}${suffix ? `?${suffix}` : ""}`;
}

export function storeRouteFromHash(hash: string): StoreRouteHashResult {
  if (!hash.startsWith("#/repos/")) return { kind: "none" };
  const separator = hash.indexOf("?");
  const pathname = separator === -1 ? hash : hash.slice(0, separator);
  const match = /^#\/repos\/([^/]+)\/([^/]+)$/u.exec(pathname);
  if (!match) return { kind: "invalid", message: "Invalid repository route." };
  let repository: string;
  try {
    repository = decodeURIComponent(match[1]!);
  } catch {
    return { kind: "invalid", message: "Invalid repository route encoding." };
  }
  const page = match[2] as StorePage;
  if (!repositoryId.test(repository) || !pages.has(page))
    return { kind: "invalid", message: "Invalid repository or page." };
  const query = new URLSearchParams(
    separator === -1 ? "" : hash.slice(separator + 1),
  );
  if ([...query.keys()].some((key) => !queryKeys.has(key)))
    return { kind: "invalid", message: "Unknown repository route field." };

  const snapshotOperationId = one(query, "at_op");
  const bookmark = one(query, "bookmark");
  const commitId = one(query, "commit");
  const change = one(query, "change");
  const inspectedOperationId = one(query, "operation");
  const comparisonParentId = one(query, "parent");
  const comparisonGroupId = one(query, "group");
  const path = one(query, "file");
  const directory = one(query, "directory");
  const agentSessionId = one(query, "session");
  const reviewThreadId = one(query, "thread");
  const valid =
    validOptional(snapshotOperationId, objectId) &&
    validOptional(commitId, objectId) &&
    validOptional(change, changeId) &&
    validOptional(inspectedOperationId, objectId) &&
    validOptional(comparisonParentId, objectId) &&
    validOptional(comparisonGroupId, changeId) &&
    bookmark !== undefined &&
    (bookmark === null || safeLabel(bookmark, 1024)) &&
    path !== undefined &&
    (path === null || safePath(path)) &&
    directory !== undefined &&
    (directory === null || safePath(directory)) &&
    validOptional(agentSessionId, stableId) &&
    validOptional(reviewThreadId, stableId);
  if (!valid)
    return { kind: "invalid", message: "Invalid repository route value." };
  const hasSnapshotSelection = Boolean(
    bookmark ||
    commitId ||
    change ||
    inspectedOperationId ||
    comparisonParentId ||
    comparisonGroupId ||
    path ||
    directory ||
    agentSessionId ||
    reviewThreadId,
  );
  if (hasSnapshotSelection && !snapshotOperationId)
    return {
      kind: "invalid",
      message: "Repository selections require a snapshot operation.",
    };
  if (page === "oplog" && commitId)
    return {
      kind: "invalid",
      message: "Commit selections are unavailable on the operation log.",
    };
  if (
    page !== "oplog" &&
    (inspectedOperationId || comparisonParentId || comparisonGroupId)
  )
    return {
      kind: "invalid",
      message: "Operation comparisons require the operation log.",
    };
  if (
    path &&
    page === "files" &&
    directory &&
    !path.startsWith(`${directory}/`)
  )
    return { kind: "invalid", message: "File is outside its directory route." };
  if (page !== "agents" && agentSessionId)
    return {
      kind: "invalid",
      message: "Agent session selections require the Agents page.",
    };
  if (page !== "reviews" && reviewThreadId)
    return {
      kind: "invalid",
      message: "Review thread selections require the Reviews page.",
    };
  if (
    (page === "agents" || page === "reviews") &&
    (bookmark ||
      commitId ||
      change ||
      inspectedOperationId ||
      comparisonParentId ||
      comparisonGroupId ||
      path ||
      directory)
  )
    return {
      kind: "invalid",
      message: "Collaboration routes contain an incompatible code selection.",
    };
  return {
    kind: "route",
    route: {
      page,
      repository,
      bookmark: bookmark ?? "",
      snapshotOperationId,
      inspectedOperationId,
      commitId,
      changeId: change,
      comparisonParentId,
      comparisonGroupId,
      path,
      directory: directory ?? "",
      agentSessionId,
      reviewThreadId,
    },
  };
}
