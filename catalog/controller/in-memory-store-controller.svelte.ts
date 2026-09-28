import type { FileListingViewMode } from "@lapismd/design-core/diff";
import * as fixture from "../fixtures/repository.js";
import {
  choices,
  repositories,
  repositorySnapshot,
} from "../fixtures/snapshots.js";
import type {
  StoreController,
  StoreControllerState,
  StoreRoute,
} from "../../web/src/lib/controller/contracts.js";
import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
import {
  largeBookmarks,
  largeChoices,
  largeDetail,
  largeGraph,
  largeRepositorySnapshot,
  largeRevisions,
} from "../fixtures/large-repository.js";
import { createInMemoryStoreSearch } from "./in-memory-store-search.svelte.js";

/** Each story owns this instance. It has no persistence or transport effects. */
export function createInMemoryStoreController(
  initialPage: StorePage = "branches",
  options: { large?: boolean } = {},
): StoreController {
  const fixtureBookmarks = options.large ? largeBookmarks : fixture.bookmarks;
  const fixtureChoices = options.large ? largeChoices : choices;
  const fixtureRevisions = options.large ? largeRevisions : fixture.revisions;
  const revisionGraph = options.large ? largeGraph : undefined;
  let history = $state<StoreRoute[]>([
    {
      page: initialPage,
      repository: "jj-store",
      bookmark: initialPage === "files" ? "main" : "",
      snapshotOperationId: fixture.IDS.opLeft,
      inspectedOperationId: null,
      commitId: null,
      changeId: null,
      comparisonParentId: null,
      comparisonGroupId: null,
      path: null,
      directory: "",
    },
  ]);
  let index = $state(0);
  let authenticated = $state(true);
  let fileViewMode = $state<FileListingViewMode>("tree");
  let collapsedDirectoryPaths = $state(new Set<string>());
  const route = $derived(history[index]!);
  const targets = $derived(
    route.commitId
      ? [route.commitId]
      : (fixtureBookmarks.find((bookmark) => bookmark.name === route.bookmark)
          ?.targets ?? (route.page === "files" ? [fixture.IDS.current] : [])),
  );
  const snapshot = $derived(
    targets.length === 1 &&
      fixtureRevisions.some((revision) => revision.eventId === targets[0])
      ? options.large
        ? largeRepositorySnapshot(route.repository, targets[0]!)
        : repositorySnapshot(route.repository, targets[0]!)
      : undefined,
  );
  function push(next: StoreRoute) {
    if (JSON.stringify(next) === JSON.stringify(route)) return;
    history = [...history.slice(0, index + 1), next];
    index++;
  }
  const search = createInMemoryStoreSearch(() => route, push);
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
      });
    },
    navigate(delta) {
      index = Math.max(0, Math.min(history.length - 1, index + delta));
    },
    selectBookmark(name) {
      if (!fixtureBookmarks.some((bookmark) => bookmark.name === name))
        throw new Error("Unknown fixture bookmark");
      push({
        ...route,
        bookmark: name,
        inspectedOperationId: null,
        commitId: null,
        changeId: null,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
      });
    },
    selectRevision(id) {
      if (
        id &&
        ![...fixtureRevisions, ...fixture.operations.revisions].some(
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
            : (fixtureRevisions.find((revision) => revision.eventId === id)
                ?.changeId ?? null),
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
    selectRef(value) {
      const choice = fixtureChoices.find((choice) => choice.value === value);
      if (!choice) throw new Error("Unknown fixture reference");
      if (choice.kind === "bookmark") actions.selectBookmark(choice.label);
      else
        push({
          ...route,
          bookmark: "",
          inspectedOperationId: null,
          commitId: choice.targets[0]!,
          changeId:
            fixtureRevisions.find(
              (revision) => revision.eventId === choice.targets[0],
            )?.changeId ?? null,
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
        route,
        canNavigateBack: index > 0,
        canNavigateForward: index < history.length - 1,
        repositories,
        repository: repositories.find((repo) => repo.id === route.repository)!,
        operationHeads: [fixture.IDS.opLeft, fixture.IDS.opRight],
        comparisonParents: [],
        conflictCount: null,
        unavailableComparisonCount: 0,
        canLoadMoreBookmarks: false,
        canLoadMoreGraph: false,
        canLoadMoreFiles: false,
        canLoadMoreDetail: false,
        fileViewMode,
        collapsedDirectoryPaths,
        bookmarks: fixtureBookmarks,
        choices: fixtureChoices,
        targets,
        snapshot,
        blob: snapshot?.files.find((file) => file.path === route.path),
        search: search.state,
        graph:
          route.page === "oplog"
            ? fixture.operations
            : (revisionGraph ??
              fixture.fixtureGraph(
                route.page === "branches" && route.bookmark
                  ? route.bookmark
                  : undefined,
              )),
        now: fixture.NOW,
        detail:
          (route.commitId || route.inspectedOperationId) &&
          ![...fixtureRevisions, ...fixture.operations.revisions].find(
            (revision) =>
              revision.eventId ===
              (route.page === "oplog"
                ? route.inspectedOperationId
                : route.commitId),
          )?.empty
            ? options.large
              ? largeDetail
              : fixture.sampleDetail
            : undefined,
        authenticated,
        member: authenticated
          ? {
              id: "maya",
              name: "Maya Chen",
              handle: "maya",
              initials: "MC",
              presence: "online" as const,
              status: "Local fixture account",
            }
          : {
              id: "guest",
              name: "Guest",
              handle: "guest",
              initials: "G",
              presence: "offline" as const,
            },
        status: {
          connection: authenticated ? "connected" : "disconnected",
          navigation: "ready",
          repositories: "ready",
          bookmarks: fixtureBookmarks.length ? "ready" : "empty",
          graph: "ready",
          files: snapshot?.files.length ? "ready" : "empty",
          detail:
            route.commitId || route.inspectedOperationId ? "ready" : "idle",
          search: search.state.status,
          live: authenticated ? "live" : "disconnected",
          refreshAvailable: false,
          error: "",
        },
      };
    },
    actions,
  };
}
