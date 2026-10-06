import type { StorePage } from "../ui/community-navigation/model.js";

export interface StoreRoute {
  fileDisplay?: "source" | "preview" | "blame";
  context?: "stacks";
  stackTab?: "branches" | "workspace";
  stackHistory?: "live" | "pinned";
  historyQuery?: string;
  historyTime?: number;
  commentId?: string | null;
  threadId?: string | null;
  replyId?: string | null;
  projectsHub?: boolean;
  workTab?: import("./projects-hub-types.js").ProjectsTab;
  workQuery?: string;
  workActivity?: import("./native-activity-filter.js").NativeActivityFilter;
  workState?: "active" | "all" | "open" | "in_progress" | "blocked" | "done";
  workRepository?: string;
  workHome?: string;
  workAgentRepository?: string | null;
  page: StorePage;
  repository: string;
  bookmark: string;
  workspace: string | null;
  snapshotOperationId: string | null;
  inspectedOperationId: string | null;
  commitId: string | null;
  evolutionCommitId: string | null;
  changeId: string | null;
  comparisonParentId: string | null;
  comparisonGroupId: string | null;
  path: string | null;
  directory: string;
  agentSessionId: string | null;
  reviewThreadId: string | null;
  projectId?: string | null;
  taskId?: string | null;
}
