import { readGlobalSettingsHash } from "./global-settings-route-codec.js";
import { readThreadsHash } from "./threads-route-codec.js";
import { readAgentsHash } from "./agents-route-codec.js";
import {
  safeLabel,
  safePath,
  one,
  validOptional,
  repositoryPages,
  repositoryQueryKeys,
} from "./store-route-fields.js";
import { historyQueryFromParams } from "./history-query-route.js";
import { projectsRouteFromHash } from "./projects-route-codec.js";
import type { StorePage } from "../ui/community-navigation/model.js";
import type { StoreRoute } from "./contracts.js";
import { readStacksRouteFields } from "./store-stacks-route-codec.js";

import { readRepositorySettings } from "./repository-settings-route-codec.js";

const repositoryId = /^[0-9a-f]{32}$/u;
const commitObjectId = /^(?:[0-9a-f]{40}|[0-9a-f]{128})$/u;
const objectId = /^[0-9a-f]{128}$/u;
const changeId = /^[0-9a-f]{32}$/u;
const stableId = /^[0-9a-f]{32}$/u;

export type StoreRouteHashResult =
  | { kind: "none" }
  | { kind: "invalid"; message: string }
  | { kind: "route"; route: StoreRoute };

export { storeRouteToHash } from "./store-route-writer.js";

export function storeRouteFromHash(hash: string): StoreRouteHashResult {
  const settings = readGlobalSettingsHash(hash);
  if (settings) return settings;
  const threads = readThreadsHash(hash);
  if (threads) return threads;
  const agents = readAgentsHash(hash);
  if (agents) return agents;
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
  if (!repositoryId.test(repository) || !repositoryPages.has(page))
    return { kind: "invalid", message: "Invalid repository or page." };
  const query = new URLSearchParams(
    separator === -1 ? "" : hash.slice(separator + 1),
  );
  if ([...query.keys()].some((key) => !repositoryQueryKeys.has(key)))
    return { kind: "invalid", message: "Unknown repository route field." };

  if (page === "settings") {
    const route = readRepositorySettings(repository, query);
    return route
      ? { kind: "route", route }
      : { kind: "invalid", message: "Invalid repository settings route." };
  }
  if (query.has("section") || query.has("tab"))
    return {
      kind: "invalid",
      message: "Settings fields require a settings page.",
    };
  const stacksFields = readStacksRouteFields(query, page);
  if (!stacksFields)
    return { kind: "invalid", message: "Invalid Stacks navigation context." };

  const historyFields = historyQueryFromParams(query, page);
  if (!historyFields)
    return { kind: "invalid", message: "Invalid history query context." };
  const { historyQuery, historyTime } = historyFields;
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
  const fileDisplay = one(query, "file_view");
  if (
    fileDisplay !== null &&
    (page !== "files" ||
      !path ||
      !commitId ||
      !["source", "preview", "blame"].includes(fileDisplay ?? ""))
  )
    return {
      kind: "invalid",
      message: "File display requires a selected recorded file.",
    };
  const directory = one(query, "directory");
  const agentSessionId = one(query, "session");
  const primaryThread = one(query, "thread");
  const legacyThread = one(query, "channel");
  if (page === "threads" && primaryThread !== null && legacyThread !== null)
    return {
      kind: "invalid",
      message: "Thread route has duplicate selections.",
    };
  const reviewThreadId = page === "threads" ? null : primaryThread;
  const threadId =
    page === "threads" ? (primaryThread ?? legacyThread) : legacyThread;
  const replyId = one(query, "reply");
  const reviewView = one(query, "review_view");
  const projectId = one(query, "project");
  const taskId = one(query, "task");
  const commentId = one(query, "comment");
  const valid =
    (reviewView === null || (page === "reviews" && reviewView === "columns")) &&
    validOptional(projectId, stableId) &&
    validOptional(commentId, stableId) &&
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
    validOptional(reviewThreadId, stableId) &&
    validOptional(threadId, stableId) &&
    validOptional(replyId, stableId);
  if (!valid)
    return { kind: "invalid", message: "Invalid repository route value." };
  if (((threadId || replyId) && page !== "threads") || (replyId && !threadId))
    return { kind: "invalid", message: "Replies require a selected Thread." };
  if ((projectId || taskId) && page !== "projects")
    return { kind: "invalid", message: "Task selections require Projects." };
  if (commentId && (!taskId || page !== "projects"))
    return { kind: "invalid", message: "Comment selections require a task." };
  if (taskId && !projectId)
    return { kind: "invalid", message: "Task selections require a project." };
  const hasSnapshotSelection = Boolean(
    historyQuery ||
    historyTime !== null ||
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
    (page === "agents" ||
      page === "reviews" ||
      page === "projects" ||
      page === "threads") &&
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
      ...stacksFields,
      ...(fileDisplay
        ? { fileDisplay: fileDisplay as "source" | "preview" | "blame" }
        : {}),
      ...(historyQuery ? { historyQuery } : {}),
      ...(historyTime !== null ? { historyTime: Number(historyTime) } : {}),
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
      ...(threadId ? { threadId } : {}),
      ...(replyId ? { replyId } : {}),
      ...(page === "projects"
        ? { projectId, taskId, ...(commentId ? { commentId } : {}) }
        : {}),
    },
  };
}
