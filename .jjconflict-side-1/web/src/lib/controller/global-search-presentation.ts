import type { Model } from "../api/client.js";
import type { StoreRoute } from "./contracts.js";
import type { GlobalSearchResult } from "./global-search-types.js";
export function emptyRoute(repository: string): StoreRoute {
  return {
    page: "threads",
    repository,
    bookmark: "",
    workspace: null,
    snapshotOperationId: null,
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
export function contentResults(
  page: Model<"ContentPage">,
  name: string,
): GlobalSearchResult[] {
  return page.results.map((hit) => ({
    id: `${page.snapshot.repo_id}:${hit.kind}:${hit.id}`,
    kind: hit.kind,
    title: hit.title,
    description: hit.excerpt,
    timestamp: Number(hit.at_millis) / 1000,
    source: name,
    revision: hit.revision,
    route: {
      ...emptyRoute(page.snapshot.repo_id),
      ...(hit.thread
        ? {
            page: "threads" as const,
            threadId: hit.thread,
            replyId: hit.message,
          }
        : {
            page: "projects" as const,
            projectsHub: true,
            projectId: hit.project,
            taskId: hit.task,
            commentId: hit.comment,
            workTab: hit.task ? ("tasks" as const) : undefined,
          }),
    },
  }));
}
