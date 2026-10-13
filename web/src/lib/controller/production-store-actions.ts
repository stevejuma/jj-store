import { agentsRoute } from "./agents-route-codec.js";
import { repositorySettingsActions } from "./repository-settings-actions.js";
import { isStacksRoute } from "./store-stacks-navigation.js";
import { workspaceSelectedRoute } from "./store-workspaces.js";
import type { StoreController } from "./contracts.js";
import type { createProductionBranchActions } from "./production-branch-actions.js";
import type { createProductionStoreConflicts } from "./production-store-conflicts.js";
import type { createProductionStoreSettings } from "./production-store-index.js";
import type { createProductionStoreMutations } from "./production-store-mutations.js";
import type { createProductionStoreReader } from "./production-store-reader.svelte.js";
import type { createProductionStoreSearch } from "./production-store-search-controller.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

type History = ReturnType<typeof createStoreRouteHistory>;
type Reader = ReturnType<typeof createProductionStoreReader>;

export function createProductionStoreActions(options: {
  data: ProductionStoreState;
  source(): import("./native-data-source.js").StoreDataSource;
  history: History;
  reader: Reader;
  search: ReturnType<typeof createProductionStoreSearch>;
  mutations: ReturnType<typeof createProductionStoreMutations>;
  conflicts: ReturnType<typeof createProductionStoreConflicts>;
  branches: ReturnType<typeof createProductionBranchActions>;
  selectRevision: StoreController["actions"]["selectRevision"];
  settings: ReturnType<typeof createProductionStoreSettings>;
  connect: StoreController["actions"]["connect"];
  connectRepository: (
    repository: string,
    token: string | null,
  ) => Promise<void>;
  disconnect: () => void;
  destroy: () => void;
  refresh: () => Promise<void>;
  selectOperation: (id: string) => Promise<void>;
  restoreRoute: () => Promise<void>;
  ensureStacks: () => Promise<void>;
  activeRepository: () => string | undefined;
  invalidate: () => void;
}): StoreController["actions"] {
  return {
    async openWorkingFile(path) {
      const source = options.source();
      if (!source.workingCopy)
        throw new Error("Working copy editing is unavailable.");
      const file = await source.workingCopy.read(path);
      if (source !== options.source())
        throw new DOMException("Session changed", "AbortError");
      return file;
    },
    async saveWorkingFile(selection, contents) {
      const source = options.source();
      if (!source.workingCopy)
        throw new Error("Working copy editing is unavailable.");
      const file = await source.workingCopy.save(selection, contents);
      if (source !== options.source())
        throw new DOMException("Session changed", "AbortError");
      return file;
    },
    connect: options.connect,
    disconnect: options.disconnect,
    destroy: options.destroy,
    refresh: options.refresh,
    selectOperation: options.selectOperation,
    async selectComparisonParent(id) {
      if (
        !options.data.comparisonParents.includes(id) ||
        !options.history.route.inspectedOperationId
      )
        return;
      options.data.comparisonParents = [];
      await options.reader.loadOperation(
        options.history.route.inspectedOperationId,
        id,
      );
    },
    async selectWorkspace(workspace) {
      if (
        ["previewing", "prepared", "applying", "unknown"].includes(
          options.data.mutation.status,
        )
      )
        return;
      if (
        workspace &&
        !options.data.workingCopies.some((copy) => copy.workspace === workspace)
      )
        return;
      options.mutations.reset();
      options.history.push(
        workspaceSelectedRoute(options.history.route, workspace),
      );
      await options.restoreRoute();
    },
    refreshSettings: options.settings.refresh,
    refreshWorkspaceLinks: options.settings.refreshWorkspaceLinks,
    setWorkspacePath: options.settings.setWorkspacePath,
    linkWorkspace: options.settings.linkWorkspace,
    pickWorkspaceFolder: options.settings.pickWorkspaceFolder,
    waitForSync: options.settings.waitForSync,
    stopSyncWait: options.settings.stopSyncWait,
    refreshSyncSettings: options.settings.refreshSyncSettings,
    setSearchDraft: options.settings.setSearchDraft,
    cancelSearchDraft: options.settings.cancelSearchDraft,
    refreshSearchSettings: options.settings.refreshSearchSettings,
    saveSearchSettings: options.settings.saveSearchSettings,
    controlSearch: options.settings.controlSearch,
    refreshStorage: options.settings.refreshStorage,
    startStorage: options.settings.startStorage,
    retryStorage: options.settings.retryStorage,
    controlSync: options.settings.controlSync,
    prepareKindIndex: options.settings.prepareKindIndex,
    prepareIndex: options.settings.prepareIndex,
    cancelIndex: options.settings.cancelIndex,
    async loadMore(collection) {
      if (collection === "workspaces")
        await options.reader.loadWorkingCopies(
          Boolean(options.data.workingCopiesCursor),
          true,
        );
      if (collection === "bookmarks" && options.data.refsCursor)
        await options.reader.loadReferences(true);
      if (collection === "graph" && options.data.graphCursor)
        await options.reader.loadGraph(true);
      if (collection === "evolution" && options.data.evolution.canLoadMore)
        await options.reader.loadEvolution(true);
      if (collection === "files") await options.reader.loadMoreFiles();
      if (collection === "detail") await options.reader.loadMoreDetail();
      if (collection === "search" && options.data.searchCursor)
        await options.search.loadMore();
      if (collection === "agents" && options.data.agentCursor)
        await options.reader.loadAgents(true);
      if (collection === "transcript" && options.data.transcriptCursor)
        await options.reader.loadTranscript(true);
      if (collection === "reviews" && options.data.reviewCursor)
        await options.reader.loadReviews(true);
    },
    search: options.search.run,
    selectSearchResult: options.search.select,
    ...options.mutations.actions,
    uploadConflict: options.conflicts.upload,
    setFileViewMode: options.reader.setFileViewMode,
    toggleDirectory: options.reader.toggleDirectory,
    async openReview(repository, thread) {
      if (!options.data.repositories.some((repo) => repo.id === repository))
        return;
      const switched = repository !== options.activeRepository();
      if (switched) {
        await options.connectRepository(repository, null);
        if (
          options.history.route.repository !== repository ||
          options.history.route.page === "projects"
        )
          return;
      }
      const next = {
        ...options.history.route,
        repository,
        snapshotOperationId:
          (options.history.route.repository === repository
            ? options.history.route.snapshotOperationId
            : null) ??
          (options.data.heads.length === 1 ? options.data.heads[0]! : null),
        page: "reviews" as const,
        reviewThreadId: thread,
        projectsHub: false,
        projectId: null,
        taskId: null,
      };
      switched ? options.history.replace(next) : options.history.push(next);
      await options.restoreRoute();
    },
    async openRepository(id) {
      if (id !== options.activeRepository()) {
        if (
          options.data.repositories.some((repository) => repository.id === id)
        ) {
          await options.connectRepository(id, null);
          if (
            options.history.route.repository !== id ||
            options.history.route.page === "projects"
          )
            return;
          if (options.history.route.page !== "files") {
            options.history.page("files");
            await options.restoreRoute();
          }
        }
        return;
      }
      if (
        options.history.route.page === "projects" ||
        options.history.route.agentsHub
      ) {
        options.history.connect(
          id,
          options.data.heads.length === 1 ? options.data.heads[0]! : null,
        );
      } else options.history.page("files");
      await options.restoreRoute();
    },
    async selectPage(page) {
      if (page === "workflows") { options.history.push({...agentsRoute(),page:"workflows",agentsHub:false});return; }
      options.history.page(page);
      await options.restoreRoute();
    },
    async selectStacksView(view, historyMode) {
      const retainedCanvas =
        isStacksRoute(options.history.route) &&
        (view === "branches" || view === "workspace");
      options.history.stacks.select(view, historyMode);
      if (retainedCanvas) {
        const desired = options.history.route;
        options.data.status.error = "";
        options.data.status.navigation = "loading";
        await options.ensureStacks();
        if (options.history.route === desired)
          options.data.status.navigation = options.data.status.error
            ? "failed"
            : "ready";
      } else await options.restoreRoute();
    },
    ...repositorySettingsActions(options.history, options.restoreRoute),
    async navigate(delta) {
      if (options.history.navigate(delta)) await options.restoreRoute();
    },
    ...options.branches,
    selectRevision: options.selectRevision,
    async selectEvolution(id) {
      const route = options.history.route;
      if (id && (!route.commitId || !["log", "branches"].includes(route.page)))
        return;
      if (
        id &&
        id !== route.commitId &&
        !options.data.evolution.graph.revisions.some(
          (revision) => revision.eventId === id,
        )
      )
        return;
      options.history.push({ ...route, evolutionCommitId: id });
      if (id) await options.reader.loadEvolution();
    },
    setFileDisplay(fileDisplay) {
      const route = options.history.route;
      if (
        route.page === "files" &&
        route.path &&
        route.commitId &&
        route.snapshotOperationId
      )
        options.history.push({ ...route, fileDisplay });
    },
    async selectFile(path) {
      options.invalidate();
      options.history.push({
        ...options.history.route,
        path,
        fileDisplay: undefined,
      });
      options.data.blob = undefined;
      if (!path) return;
      if (options.history.route.page === "files")
        await options.reader.loadBlob(path);
      else {
        await options.reader.loadPatch(path);
        const commit = options.history.route.commitId;
        if (commit) await options.reader.loadPathReview(commit, path);
        else if (options.history.route.inspectedOperationId)
          await options.reader.loadOperationPathReview(
            options.history.route.inspectedOperationId,
            path,
          );
      }
    },
    selectDirectory: options.reader.selectDirectory,
    refreshAgentConversation: options.reader.refreshAgentConversation,
    async selectAgentSession(id) {
      options.invalidate();
      await options.reader.selectAgentSession(id);
    },
    async selectReviewThread(id) {
      options.invalidate();
      await options.reader.selectReviewThread(id);
    },
    createRepositoryReview: options.reader.createRepositoryReview,
    replyToReview: options.reader.reply,
    editReviewComment: options.reader.edit,
    commentOnDiffLine: options.reader.commentOnDiffLine,
    commentOnAgentTool: options.reader.commentOnAgentTool,
    readAgentToolReview: options.reader.readAgentToolReview,
    readAgentSessionReview: options.reader.readAgentSessionReview,
    commentOnAgentSession: options.reader.commentOnAgentSession,
    setReviewResolved: options.reader.setResolved,
    checkReviewAction: options.reader.checkAction,
    selectRef: (value) => options.reader.selectRef(value),
    signIn() {},
    signOut: options.disconnect,
  };
}
