import { createInMemoryStoreNavigation } from "./in-memory-store-navigation.js";
import type { FileListingViewMode } from "@lapismd/design-core/diff";
import * as fixture from "../fixtures/repository.js";
import { repositories } from "../fixtures/snapshots.js";
import type * as StoreContract from "../../web/src/lib/controller/contracts.js";
import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
import * as large from "../fixtures/large-repository.js";
import { createInMemoryStoreSearch } from "./in-memory-store-search.svelte.js";
import { createInMemoryStoreMutations } from "./in-memory-store-mutations.svelte.js";
import * as storeRoute from "./in-memory-store-route.js";
import { createInMemoryStoreRepository } from "./in-memory-store-repository.svelte.js";
import { inMemoryStoreMember } from "./in-memory-store-member.js";
import { createInMemoryStoreConflicts } from "./in-memory-store-conflicts.js";
import { createInMemoryStoreBranchActions } from "./in-memory-store-branch-actions.js";
import { createInMemoryStoreCollaboration } from "./in-memory-store-collaboration.svelte.js";
import { inMemoryStoreStatus } from "./in-memory-store-status.js";
import { inMemoryStoreDetail } from "./in-memory-store-history.js";
import type { InMemoryStoreControllerOptions } from "./in-memory-store-options.js";
import { createInMemoryStoreSettings } from "./in-memory-store-settings.svelte.js";
import { createInMemoryStoreSession } from "./in-memory-store-session.svelte.js";
import { inMemoryStoreEvolution } from "./in-memory-store-evolution.js";

/** Each story owns this instance. It has no persistence or transport effects. */
export function createInMemoryStoreController(
  initialPage: StorePage = "branches",
  options: InMemoryStoreControllerOptions = {},
): StoreContract.StoreController {
  const workingCopies = options.workingCopies ?? fixture.workingCopies;
  const repositoryData = createInMemoryStoreRepository(options.large);
  let history = $state<StoreContract.StoreRoute[]>([
    storeRoute.initialStoreRoute(initialPage),
  ]);
  let index = $state(0);
  let fileViewMode = $state<FileListingViewMode>("tree");
  let collapsedDirectoryPaths = $state(new Set<string>());
  const route = $derived(history[index]!);
  const targets = $derived(
    route.commitId
      ? [route.commitId]
      : (repositoryData.bookmarks.find(
          (bookmark) => bookmark.name === route.bookmark,
        )?.targets ??
          (route.workspace
            ? [
                workingCopies.find(
                  (copy) => copy.workspace === route.workspace,
                )!.commitId,
              ]
            : route.page === "files"
              ? [fixture.IDS.current]
              : [])),
  );
  const snapshot = $derived(
    targets.length === 1 &&
      repositoryData.revisions.some(
        (revision) => revision.eventId === targets[0],
      )
      ? options.large
        ? large.largeRepositorySnapshot(route.repository, targets[0]!)
        : repositoryData.snapshot(route.repository, targets[0]!)
      : undefined,
  );
  const conflictController = createInMemoryStoreConflicts(() => route);
  function push(next: StoreContract.StoreRoute) {
    if (JSON.stringify(next) === JSON.stringify(route)) return;
    history = [...history.slice(0, index + 1), next];
    index++;
  }
  const search = createInMemoryStoreSearch(() => route, push);
  const mutations = createInMemoryStoreMutations({
    onCommitted(command, result) {
      const change = repositoryData.commit(
        command,
        result,
        route.repository,
        route.commitId,
      );
      const next = storeRoute.mutationCommittedRoute(route, change);
      if (next) push(next);
    },
    firstApply: options.mutationOutcome,
    previewFailure: options.mutationFailure,
  });
  const branchActions = createInMemoryStoreBranchActions({
    bookmarks: () => repositoryData.bookmarks,
    workingCopies,
    route: () => route,
    push,
  });
  const collaboration = createInMemoryStoreCollaboration({
    route: () => route,
    push,
    now: fixture.NOW,
    outcome: options.reviewOutcome,
  });
  const session = createInMemoryStoreSession();
  const settings = createInMemoryStoreSettings({
    repository: () => route.repository,
    heads: () => repositoryData.operationHeads,
    operation: () => route.snapshotOperationId,
    mode: options.settingsSearch,
  });
  const actions: StoreContract.StoreController["actions"] = {
    ...session.actions,
    destroy() {},
    async refresh() {},
    async selectOperation() {},
    async selectComparisonParent() {},
    refreshSettings: settings.refresh,
    async refreshWorkspaceLinks() {},
    setWorkspacePath() {},
    async linkWorkspace() {},
    async pickWorkspaceFolder() {},
    prepareIndex: settings.prepareIndex,
    async loadMore() {},
    search: search.search,
    selectSearchResult: search.select,
    ...mutations.actions,
    ...conflictController.actions,
    setFileViewMode(mode) {
      fileViewMode = mode;
    },
    toggleDirectory(path) {
      const next = new Set(collapsedDirectoryPaths);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      collapsedDirectoryPaths = next;
    },
    ...createInMemoryStoreNavigation({
      route: () => route,
      push,
      workingCopies,
    }),
    navigate(delta) {
      index = Math.max(0, Math.min(history.length - 1, index + delta));
    },
    ...branchActions,
    selectEvolution(id) {
      if (id && (!route.commitId || !["log", "branches"].includes(route.page)))
        return;
      push({ ...route, evolutionCommitId: id });
    },
    selectRevision(id) {
      const revision = [
        ...repositoryData.revisions,
        ...fixture.operations.revisions,
      ].find((revision) => revision.eventId === id);
      if (id && !revision) throw new Error("Unknown fixture revision");
      const context = id
        ? route.page === "oplog"
          ? collaboration.operationContext(id)
          : collaboration.revisionContext(id)
        : { agentSessionId: null, reviewThreadId: null };
      const changeId =
        route.page === "oplog" ? null : (revision?.changeId ?? null);
      push(storeRoute.revisionSelectedRoute(route, id, changeId, context));
    },
    selectFile(path) {
      if (
        path &&
        route.page !== "oplog" &&
        !snapshot?.files.some((file) => file.path === path)
      )
        throw new Error("File absent from selected snapshot");
      push({
        ...route,
        path,
        directory: path
          ? path.split("/").slice(0, -1).join("/")
          : route.directory,
        reviewThreadId:
          path && route.commitId
            ? collaboration.pathReview(route.commitId, path)
            : path && route.inspectedOperationId
              ? collaboration.operationPathReview(
                  route.inspectedOperationId,
                  path,
                )
              : null,
      });
    },
    selectDirectory(directory) {
      if (
        directory &&
        !snapshot?.files.some((file) => file.path.startsWith(`${directory}/`))
      )
        throw new Error("Directory absent from selected snapshot");
      push({ ...route, directory, path: null });
    },
    ...collaboration.actions,
    selectRef(value) {
      const choice = repositoryData.choices.find(
        (choice) => choice.value === value,
      );
      if (!choice) throw new Error("Unknown fixture reference");
      push({
        ...route,
        bookmark: choice.kind === "bookmark" ? choice.label : "",
        inspectedOperationId: null,
        commitId:
          choice.targets.length === 1 ? (choice.targets[0] ?? null) : null,
        evolutionCommitId: null,
        changeId:
          choice.kind === "change"
            ? (repositoryData.revisions.find(
                (revision) => revision.eventId === choice.targets[0],
              )?.changeId ?? null)
            : null,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
      });
    },
  };
  return {
    get state(): StoreContract.StoreControllerState {
      return {
        accessMode: "authenticated",
        storageReady: true,
        backups: [],
        route,
        canNavigateBack: index > 0,
        canNavigateForward: index < history.length - 1,
        repositories,
        repository: repositories.find((repo) => repo.id === route.repository)!,
        operationHeads: repositoryData.operationHeads,
        comparisonParents: [],
        ...conflictController.view,
        unavailableComparisonCount: 0,
        canLoadMoreBookmarks: false,
        canLoadMoreGraph: false,
        canLoadMoreFiles: false,
        canLoadMoreDetail: false,
        canLoadMoreAgents: false,
        canLoadMoreTranscript: false,
        canLoadMoreReviews: false,
        fileViewMode,
        evolution: inMemoryStoreEvolution(route, repositoryData.revisions),
        collapsedDirectoryPaths,
        bookmarks: repositoryData.bookmarks,
        choices: repositoryData.choices,
        workingCopies,
        targets,
        snapshot,
        blob: snapshot?.files.find((file) => file.path === route.path),
        search: search.state,
        settings: settings.state,
        mutation: mutations.state,
        ...collaboration.view,
        graph:
          route.page === "oplog"
            ? fixture.operations
            : options.large
              ? large.largeGraph
              : fixture.fixtureGraph(
                  route.page === "branches" && route.bookmark
                    ? route.bookmark
                    : undefined,
                  repositoryData.bookmarks,
                  repositoryData.revisions,
                  route.commitId
                    ? [route.commitId]
                    : route.workspace
                      ? targets
                      : undefined,
                ),
        now: fixture.NOW,
        detail: inMemoryStoreDetail(
          route,
          [...repositoryData.revisions, ...fixture.operations.revisions],
          options.large ? large.largeDetail : fixture.sampleDetail,
        ),
        fileRevision: snapshot
          ? repositoryData.revisions.find(
              (revision) => revision.eventId === snapshot.id,
            )
          : undefined,
        authenticated: session.authenticated,
        member: inMemoryStoreMember(session.authenticated),
        status: inMemoryStoreStatus({
          authenticated: session.authenticated,
          hasBookmarks: repositoryData.bookmarks.length > 0,
          hasFiles: Boolean(snapshot?.files.length),
          hasDetail: Boolean(route.commitId || route.inspectedOperationId),
          hasAgentSession: Boolean(route.agentSessionId),
          hasAgents: collaboration.view.agentSessions.length > 0,
          hasReviews: collaboration.view.reviewThreads.length > 0,
          reviewAction: collaboration.reviewActionStatus,
          search: search.state.status,
        }),
      };
    },
    actions,
  };
}
