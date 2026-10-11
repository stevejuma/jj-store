import type { StoreRoute } from "./store-route-types.js";
import { one } from "./store-route-fields.js";
import {
  validRepositorySettings,
  type RepositorySettingsSection,
  type RepositorySettingsTab,
} from "./repository-settings-navigation.js";

export function readRepositorySettings(
  repository: string,
  query: URLSearchParams,
): StoreRoute | null {
  if (
    [...query.keys()].some((key) => !["section", "tab", "at_op"].includes(key))
  )
    return null;
  const section = one(query, "section"),
    tab = one(query, "tab"),
    operation = one(query, "at_op");
  if (
    section === undefined ||
    tab === undefined ||
    operation === undefined ||
    section === "" ||
    tab === "" ||
    !validRepositorySettings(section ?? "general", tab) ||
    (operation !== null && !/^[0-9a-f]{128}$/.test(operation))
  )
    return null;
  return {
    page: "settings",
    repository,
    repositorySettingsSection: (section ??
      "general") as RepositorySettingsSection,
    ...(tab ? { repositorySettingsTab: tab as RepositorySettingsTab } : {}),
    snapshotOperationId: operation,
    bookmark: "",
    workspace: null,
    inspectedOperationId: null,
    commitId: null,
    evolutionCommitId: null,
    changeId: null,
    comparisonParentId: null,
    comparisonGroupId: null,
    path: null,
    directory: "",
    agentSessionId: null,
    reviewThreadId: null,
  };
}
export function writeRepositorySettings(route: StoreRoute): string | null {
  const section = route.repositorySettingsSection ?? "general";
  if (
    !validRepositorySettings(section, route.repositorySettingsTab) ||
    route.context ||
    route.stackTab ||
    route.stackHistory ||
    route.historyQuery ||
    route.historyTime !== undefined ||
    route.workspace ||
    route.bookmark ||
    route.commitId ||
    route.evolutionCommitId ||
    route.changeId ||
    route.inspectedOperationId ||
    route.comparisonParentId ||
    route.comparisonGroupId ||
    route.path ||
    route.directory ||
    route.agentSessionId ||
    route.reviewThreadId ||
    route.threadId ||
    route.replyId ||
    route.projectId ||
    route.taskId ||
    route.commentId ||
    (route.snapshotOperationId &&
      !/^[0-9a-f]{128}$/.test(route.snapshotOperationId))
  )
    return null;
  const query = new URLSearchParams({ section });
  if (route.repositorySettingsTab)
    query.set("tab", route.repositorySettingsTab);
  if (route.snapshotOperationId) query.set("at_op", route.snapshotOperationId);
  return `#/repos/${route.repository}/settings?${query}`;
}
