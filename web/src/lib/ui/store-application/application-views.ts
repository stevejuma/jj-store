import { lazyView, viewModuleURL } from "./lazy-view.js";

export const views = {
  workflows: lazyView(
    () => import("../store-workflows/StoreWorkflows.svelte"),
    viewModuleURL("../store-workflows/StoreWorkflows.svelte"),
  ),
  projectsRepository: lazyView(
    () => import("../store-work/StoreRepositoryProjects.svelte"),
    viewModuleURL("../store-work/StoreRepositoryProjects.svelte"),
  ),
  projects: lazyView(
    () => import("../store-work/StoreProjects.svelte"),
    viewModuleURL("../store-work/StoreProjects.svelte"),
  ),
  work: lazyView(
    () => import("../store-work/StoreWork.svelte"),
    viewModuleURL("../store-work/StoreWork.svelte"),
  ),
  threads: lazyView(
    () => import("../store-threads/StoreThreads.svelte"),
    viewModuleURL("../store-threads/StoreThreads.svelte"),
  ),
  agentsRecorded: lazyView(
    () => import("../store-collaboration/StoreAgentSessions.svelte"),
    viewModuleURL("../store-collaboration/StoreAgentSessions.svelte"),
  ),
  reviews: lazyView(
    () => import("../store-collaboration/StoreReviewThreads.svelte"),
    viewModuleURL("../store-collaboration/StoreReviewThreads.svelte"),
  ),
  files: lazyView(
    () =>
      import("../community-projects/CommunityProjectsRepositoryFiles.svelte"),
    viewModuleURL(
      "../community-projects/CommunityProjectsRepositoryFiles.svelte",
    ),
  ),
  log: lazyView(
    () => import("../community-projects/CommunityProjectsRepositoryLog.svelte"),
    viewModuleURL(
      "../community-projects/CommunityProjectsRepositoryLog.svelte",
    ),
  ),
  repositorySettings: lazyView(
    () => import("../store-settings/StoreRepositorySettingsView.svelte"),
    viewModuleURL("../store-settings/StoreRepositorySettingsView.svelte"),
  ),
  settings: lazyView(
    () => import("../store-settings/StoreSettings.svelte"),
    viewModuleURL("../store-settings/StoreSettings.svelte"),
  ),
  stacks: lazyView(
    () => import("../store-stack-workspace/StoreStacksApplicationView.svelte"),
    viewModuleURL("../store-stack-workspace/StoreStacksApplicationView.svelte"),
  ),
  agents: lazyView(
    () => import("../store-agent-execution/StoreAgentExecutionHost.svelte"),
    viewModuleURL("../store-agent-execution/StoreAgentExecutionHost.svelte"),
  ),
  agentDetails: lazyView(
    () => import("../store-agent-execution/StoreAgentDetailsSidebar.svelte"),
    viewModuleURL("../store-agent-execution/StoreAgentDetailsSidebar.svelte"),
  ),
  threadSidebar: lazyView(
    () => import("../store-threads/StoreThreadSidebar.svelte"),
    viewModuleURL("../store-threads/StoreThreadSidebar.svelte"),
  ),
  stackSettings: lazyView(
    () => import("../store-stack-workspace/StoreStackWorkspaceSettings.svelte"),
    viewModuleURL(
      "../store-stack-workspace/StoreStackWorkspaceSettings.svelte",
    ),
  ),
  agentSidebar: lazyView(
    () => import("../store-collaboration/StoreAgentSidebar.svelte"),
    viewModuleURL("../store-collaboration/StoreAgentSidebar.svelte"),
  ),
  agentChatSidebar: lazyView(
    () => import("../store-agent-chat/StoreAgentChatSidebar.svelte"),
    viewModuleURL("../store-agent-chat/StoreAgentChatSidebar.svelte"),
  ),
  workspaceInspector: lazyView(
    () => import("../store-agent-workspace/StoreWorkspaceInspector.svelte"),
    viewModuleURL("../store-agent-workspace/StoreWorkspaceInspector.svelte"),
  ),
  toolPanel: lazyView(
    () => import("../store-agent-chat-body/ChatToolPanel.svelte"),
    viewModuleURL("../store-agent-chat-body/ChatToolPanel.svelte"),
  ),
  workspaceFiles: lazyView(
    () => import("../store-agent-workspace/StoreWorkspaceFiles.svelte"),
    viewModuleURL("../store-agent-workspace/StoreWorkspaceFiles.svelte"),
  ),
  projectsRail: lazyView(
    () => import("../store-work/StoreProjectsRail.svelte"),
    viewModuleURL("../store-work/StoreProjectsRail.svelte"),
  ),
  stackDetails: lazyView(
    () => import("../store-stack-workspace/StoreStackWorkspaceDetails.svelte"),
    viewModuleURL("../store-stack-workspace/StoreStackWorkspaceDetails.svelte"),
  ),
  search: lazyView(
    () => import("../store-search/StoreSearchOverlay.svelte"),
    viewModuleURL("../store-search/StoreSearchOverlay.svelte"),
  ),
  conflicts: lazyView(
    () => import("../store-conflicts/StoreConflictEditor.svelte"),
    viewModuleURL("../store-conflicts/StoreConflictEditor.svelte"),
  ),
  stackConflicts: lazyView(
    () => import("../store-stack-workspace/StoreStackWorkspaceConflict.svelte"),
    viewModuleURL(
      "../store-stack-workspace/StoreStackWorkspaceConflict.svelte",
    ),
  ),
};
