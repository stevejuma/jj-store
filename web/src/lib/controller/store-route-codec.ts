import {
  projectsRouteFromHash,
  projectsRouteToHash,
} from "./projects-route-codec.js";
import type { StorePage } from "../ui/community-navigation/model.js";
import type { StoreRoute } from "./contracts.js";

const repositoryId = /^[0-9a-f]{32}$/u;
const commitObjectId = /^(?:[0-9a-f]{40}|[0-9a-f]{128})$/u;
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
  "projects",
]);
const queryKeys = new Set([
  "at_op",
  "bookmark",
  "workspace",
  "commit",
  "evolution",
  "change",
  "operation",
  "parent",
  "group",
  "file",
  "directory",
  "session",
  "thread",
  "review_view",
  "project",
  "task",
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
  if (route.projectsHub && route.page === "projects")
    return projectsRouteToHash(route);
  if (!repositoryId.test(route.repository) || !pages.has(route.page))
    return null;
  const query = new URLSearchParams();
  if (route.snapshotOperationId) query.set("at_op", route.snapshotOperationId);
  if (route.workspace) query.set("workspace", route.workspace);
  if (route.bookmark) query.set("bookmark", route.bookmark);
  if (route.commitId) query.set("commit", route.commitId);
  if (route.evolutionCommitId) query.set("evolution", route.evolutionCommitId);
  if (route.changeId) query.set("change", route.changeId);
  if (route.inspectedOperationId)
    query.set("operation", route.inspectedOperationId);
  if (route.comparisonParentId) query.set("parent", route.comparisonParentId);
  if (route.comparisonGroupId) query.set("group", route.comparisonGroupId);
  if (route.path) query.set("file", route.path);
  if (route.directory) query.set("directory", route.directory);
  if (route.agentSessionId) query.set("session", route.agentSessionId);
  if (route.reviewThreadId) query.set("thread", route.reviewThreadId);
  if (route.projectId) query.set("project", route.projectId);
  if (route.taskId) query.set("task", route.taskId);
  const suffix = query.toString();
  return `#/repos/${route.repository}/${route.page}${suffix ? `?${suffix}` : ""}`;
}

export function storeRouteFromHash(hash: string): StoreRouteHashResult {
  if (hash.startsWith("#/projects")) {
    const route = projectsRouteFromHash(hash);
    return route
      ? { kind: "route", route }
      : { kind: "invalid", message: "Invalid Projects route." };
  }
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
  const workspace = one(query, "workspace");
  const commitId = one(query, "commit");
  const evolutionCommitId = one(query, "evolution");
  const change = one(query, "change");
  const inspectedOperationId = one(query, "operation");
  const comparisonParentId = one(query, "parent");
  const comparisonGroupId = one(query, "group");
  const path = one(query, "file");
  const directory = one(query, "directory");
  const agentSessionId = one(query, "session");
  const reviewThreadId = one(query, "thread");
  const reviewView = one(query, "review_view");
  const projectId = one(query, "project");
  const taskId = one(query, "task");
  const valid =
    (reviewView === null || (page === "reviews" && reviewView === "columns")) &&
    validOptional(projectId, stableId) &&
    validOptional(taskId, stableId) &&
    validOptional(snapshotOperationId, objectId) &&
    validOptional(commitId, commitObjectId) &&
    validOptional(evolutionCommitId, commitObjectId) &&
    validOptional(change, changeId) &&
    validOptional(inspectedOperationId, objectId) &&
    validOptional(comparisonParentId, objectId) &&
    validOptional(comparisonGroupId, changeId) &&
    workspace !== undefined &&
    (workspace === null || safeLabel(workspace, 1024)) &&
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
  if ((projectId || taskId) && page !== "projects")
    return { kind: "invalid", message: "Task selections require Projects." };
  if (taskId && !projectId)
    return { kind: "invalid", message: "Task selections require a project." };
  const hasSnapshotSelection = Boolean(
    workspace ||
    bookmark ||
    commitId ||
    evolutionCommitId ||
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
    evolutionCommitId &&
    (!commitId || (page !== "log" && page !== "branches"))
  )
    return {
      kind: "invalid",
      message: "Evolution requires a selected JJ Log or Branches commit.",
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
  if (
    agentSessionId &&
    page !== "agents" &&
    page !== "log" &&
    page !== "branches"
  )
    return {
      kind: "invalid",
      message: "Agent session selections require the Agents page.",
    };
  if (
    reviewThreadId &&
    page !== "reviews" &&
    page !== "agents" &&
    page !== "log" &&
    page !== "branches"
  )
    return {
      kind: "invalid",
      message: "Review thread selections require the Reviews page.",
    };
  if (
    (page === "agents" || page === "reviews" || page === "projects") &&
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
      workspace,
      snapshotOperationId,
      inspectedOperationId,
      commitId,
      evolutionCommitId,
      changeId: change,
      comparisonParentId,
      comparisonGroupId,
      path,
      directory: directory ?? "",
      agentSessionId,
      reviewThreadId,
      ...(page === "projects" ? { projectId, taskId } : {}),
    },
  };
}
