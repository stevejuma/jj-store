import {
  IDS,
  bookmarks,
  fixtureGraph,
  operations,
  revisions,
} from "../fixtures/repository.js";
import {
  choices,
  repositories,
  repositorySnapshot,
} from "../fixtures/snapshots.js";
import type { StoreController, StoreRoute } from "./contracts.js";
import type { StorePage } from "../components/community-navigation/model.js";
/** Each story owns this instance. It has no persistence or transport effects. */
export function createInMemoryStoreController(
  initialPage: StorePage = "branches",
): StoreController {
  let history = $state<StoreRoute[]>([
    {
      page: initialPage,
      repository: "jj-store",
      bookmark: initialPage === "files" ? "main" : "",
      revisionId: null,
      path: null,
      directory: "",
    },
  ]);
  let index = $state(0);
  let authenticated = $state(true);
  const route = $derived(history[index]!);
  const targets = $derived(
    route.revisionId
      ? [route.revisionId]
      : (bookmarks.find((bookmark) => bookmark.name === route.bookmark)
          ?.targets ?? (route.page === "files" ? [IDS.current] : [])),
  );
  const snapshot = $derived(
    targets.length === 1 &&
      revisions.some((revision) => revision.eventId === targets[0])
      ? repositorySnapshot(route.repository, targets[0]!)
      : undefined,
  );
  function push(next: StoreRoute) {
    if (JSON.stringify(next) === JSON.stringify(route)) return;
    history = [...history.slice(0, index + 1), next];
    index++;
  }
  const actions: StoreController["actions"] = {
    openRepository(id) {
      if (!repositories.some((repo) => repo.id === id))
        throw new Error("Unknown fixture repository");
      push({
        page: "files",
        repository: id,
        bookmark: "main",
        revisionId: null,
        path: null,
        directory: "",
      });
    },
    selectPage(page) {
      push({
        ...route,
        page,
        bookmark: page === "files" ? "main" : "",
        revisionId: null,
        path: null,
        directory: "",
      });
    },
    navigate(delta) {
      index = Math.max(0, Math.min(history.length - 1, index + delta));
    },
    selectBookmark(name) {
      if (!bookmarks.some((bookmark) => bookmark.name === name))
        throw new Error("Unknown fixture bookmark");
      push({
        ...route,
        bookmark: name,
        revisionId: null,
        path: null,
        directory: "",
      });
    },
    selectRevision(id) {
      if (
        id &&
        ![...revisions, ...operations.revisions].some(
          (revision) => revision.eventId === id,
        )
      )
        throw new Error("Unknown fixture revision");
      push({ ...route, revisionId: id, path: null, directory: "" });
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
      const choice = choices.find((choice) => choice.value === value);
      if (!choice) throw new Error("Unknown fixture reference");
      if (choice.kind === "bookmark") actions.selectBookmark(choice.label);
      else
        push({
          ...route,
          bookmark: "",
          revisionId: choice.targets[0]!,
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
    get state() {
      return {
        route,
        canNavigateBack: index > 0,
        canNavigateForward: index < history.length - 1,
        repositories,
        repository: repositories.find((repo) => repo.id === route.repository)!,
        bookmarks,
        choices,
        targets,
        snapshot,
        blob: snapshot?.files.find((file) => file.path === route.path),
        graph:
          route.page === "oplog"
            ? operations
            : fixtureGraph(
                route.page === "branches" && route.bookmark
                  ? route.bookmark
                  : undefined,
              ),
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
      };
    },
    actions,
  };
}
