import type { FileListingViewMode } from "@lapismd/design-core/diff";
import * as fixture from "../fixtures/repository.js";
import { repositories, repositorySnapshot } from "../fixtures/snapshots.js";
import type {
  StoreController,
  StoreControllerState,
  StoreRoute,
} from "../../web/src/lib/controller/contracts.js";
import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
import {
  largeDetail,
  largeGraph,
  largeRepositorySnapshot,
} from "../fixtures/large-repository.js";
import { createInMemoryStoreSearch } from "./in-memory-store-search.svelte.js";
import { createInMemoryStoreMutations } from "./in-memory-store-mutations.svelte.js";
import {
  initialStoreRoute,
  mutationCommittedRoute,
} from "./in-memory-store-route.js";
import { createInMemoryStoreRepository } from "./in-memory-store-repository.svelte.js";
import { inMemoryStoreMember } from "./in-memory-store-member.js";
import { createInMemoryStoreConflicts } from "./in-memory-store-conflicts.js";
import { createInMemoryStoreBranchActions } from "./in-memory-store-branch-actions.js";
import { createInMemoryStoreCollaboration } from "./in-memory-store-collaboration.svelte.js";
import { inMemoryStoreStatus } from "./in-memory-store-status.js";
import { inMemoryStoreDetail } from "./in-memory-store-history.js";
import type { InMemoryStoreControllerOptions } from "./in-memory-store-options.js";

/** Each story owns this instance. It has no persistence or transport effects. */
export function createInMemoryStoreController(
  initialPage: StorePage = "branches",
  options: InMemoryStoreControllerOptions = {},
): StoreController {
  const repositoryData = createInMemoryStoreRepository(options.large);
  const revisionGraph = options.large ? largeGraph : undefined;
  let history = $state<StoreRoute[]>([initialStoreRoute(initialPage)]);
  let index = $state(0);
  let authenticated = $state(true);
  let fileViewMode = $state<FileListingViewMode>("tree");
  let collapsedDirectoryPaths = $state(new Set<string>());
  const route = $derived(history[index]!);
  const targets = $derived(
    route.commitId
      ? [route.commitId]
      : (repositoryData.bookmarks.find(
          (bookmark) => bookmark.name === route.bookmark,
        )?.targets ?? (route.page === "files" ? [fixture.IDS.current] : [])),
  );
  const snapshot = $derived(
    targets.length === 1 &&
      repositoryData.revisions.some(
        (revision) => revision.eventId === targets[0],
      )
      ? options.large
        ? largeRepositorySnapshot(route.repository, targets[0]!)
        : repositorySnapshot(route.repository, targets[0]!)
      : undefined,
  );
  const conflictController = createInMemoryStoreConflicts(() => route);
  function push(next: StoreRoute) {
    if (JSON.stringify(next) === JSON.stringify(route)) return;
    history = [...history.slice(0, index + 1), next];
    index++;
  }
  const search = createInMemoryStoreSearch(() => route, push);
  const mutations = createInMemoryStoreMutations({
    onCommitted(command, result) {
      const change = repositoryData.commit(command, result, route.commitId);
      const next = mutationCommittedRoute(route, change);
      if (next) push(next);
    },
    firstApply: options.mutationOutcome,
    previewFailure: options.mutationFailure,
  });
  const branchActions = createInMemoryStoreBranchActions({
    bookmarks: () => repositoryData.bookmarks,
    workingCopies: fixture.workingCopies,
    route: () => route,
    push,
  });
  const collaboration = createInMemoryStoreCollaboration({
    route: () => route,
    push,
    now: fixture.NOW,
    outcome: options.reviewOutcome,
  });
  const actions: StoreController["actions"] = {
    async connect() {
      authenticated = true;
    },
    disconnect() {
      authenticated = false;
    },
    destroy() {},
    async refresh() {},
    async selectOperation() {},
    async selectComparisonParent() {},
    async prepareIndex() {},
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
    openRepository(id) {
      if (!repositories.some((repo) => repo.id === id))
        throw new Error("Unknown fixture repository");
      push({
        page: "files",
        repository: id,
        bookmark: "main",
        snapshotOperationId: fixture.IDS.opLeft,
        inspectedOperationId: null,
        commitId: null,
        changeId: null,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
        agentSessionId: null,
        reviewThreadId: null,
      });
    },
    selectPage(page) {
      push({
        ...route,
        page,
        bookmark: page === "files" ? "main" : "",
        inspectedOperationId: null,
        commitId: null,
        changeId: null,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
        agentSessionId: null,
        reviewThreadId: null,
      });
    },
    navigate(delta) {
      index = Math.max(0, Math.min(history.length - 1, index + delta));
    },
    ...branchActions,
    selectRevision(id) {
      if (
        id &&
        ![...repositoryData.revisions, ...fixture.operations.revisions].some(
          (revision) => revision.eventId === id,
        )
      )
        throw new Error("Unknown fixture revision");
      push({
        ...route,
        inspectedOperationId: route.page === "oplog" ? id : null,
        commitId: route.page === "oplog" ? null : id,
        changeId:
          route.page === "oplog"
            ? null
            : (repositoryData.revisions.find(
                (revision) => revision.eventId === id,
              )?.changeId ?? null),
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
      });
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
    signIn() {
      authenticated = true;
    },
    signOut() {
      authenticated = false;
    },
  };
  return {
    get state(): StoreControllerState {
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
        collapsedDirectoryPaths,
        bookmarks: repositoryData.bookmarks,
        choices: repositoryData.choices,
        workingCopies: fixture.workingCopies,
        targets,
        snapshot,
        blob: snapshot?.files.find((file) => file.path === route.path),
        search: search.state,
        mutation: mutations.state,
        ...collaboration.view,
        graph:
          route.page === "oplog"
            ? fixture.operations
            : (revisionGraph ??
              fixture.fixtureGraph(
                route.page === "branches" && route.bookmark
                  ? route.bookmark
                  : undefined,
                repositoryData.bookmarks,
                repositoryData.revisions,
                route.commitId ? [route.commitId] : undefined,
              )),
        now: fixture.NOW,
        detail: inMemoryStoreDetail(
          route,
          [...repositoryData.revisions, ...fixture.operations.revisions],
          options.large ? largeDetail : fixture.sampleDetail,
        ),
        authenticated,
        member: inMemoryStoreMember(authenticated),
        status: inMemoryStoreStatus({
          authenticated,
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
