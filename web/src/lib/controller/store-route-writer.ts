import { globalSettingsHash } from "./global-settings-route-codec.js";
import { threadsRouteToHash } from "./threads-route-codec.js";
import { agentsRouteToHash } from "./agents-route-codec.js";
import { projectsRouteToHash } from "./projects-route-codec.js";
import { repositoryActionsRouteToHash } from "./repository-actions-route-codec.js";
import { workflowsRouteToHash } from "./workflows-route-codec.js";
import { repositoryPages } from "./store-route-fields.js";
import { historyQueryToParams } from "./history-query-route.js";
import { writeStacksRouteFields } from "./store-stacks-route-codec.js";
import { writeRepositorySettings } from "./repository-settings-route-codec.js";
import type { StoreRoute } from "./contracts.js";
const repositoryId = /^[0-9a-f]{32}$/u;

export function storeRouteToHash(route: StoreRoute): string | null {
  if (route.globalSettings) return globalSettingsHash(route.globalSettings);
  if (route.page === "actions") return repositoryActionsRouteToHash(route);
  if (route.page === "workflows") return workflowsRouteToHash(route);
  if (route.threadsHub && route.page === "threads")
    return threadsRouteToHash(route);
  if (route.agentsHub && route.page === "agents")
    return agentsRouteToHash(route);
  if (route.projectsHub && route.page === "projects")
    return projectsRouteToHash(route);
  if (!repositoryId.test(route.repository) || !repositoryPages.has(route.page))
    return null;
  if (route.page === "settings") return writeRepositorySettings(route);
  if (route.repositorySettingsSection || route.repositorySettingsTab)
    return null;
  const query = new URLSearchParams();
  if (!writeStacksRouteFields(route, query)) return null;
  historyQueryToParams(query, route);
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
  if (
    route.fileDisplay &&
    route.page === "files" &&
    route.path &&
    route.commitId
  )
    query.set("file_view", route.fileDisplay);
  if (route.directory) query.set("directory", route.directory);
  if (route.agentSessionId) query.set("session", route.agentSessionId);
  if (route.threadId) query.set("thread", route.threadId);
  if (route.commentId) query.set("comment", route.commentId);
  if (route.replyId) query.set("reply", route.replyId);
  if (route.reviewThreadId) query.set("thread", route.reviewThreadId);
  if (route.projectId) query.set("project", route.projectId);
  if (route.taskId) query.set("task", route.taskId);
  const suffix = query.toString();
  return `#/repos/${route.repository}/${route.page}${suffix ? `?${suffix}` : ""}`;
}
