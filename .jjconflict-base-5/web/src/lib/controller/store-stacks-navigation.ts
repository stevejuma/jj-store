import type { StoreRoute } from "./store-route-types.js";
import type { StorePage } from "../ui/community-navigation/model.js";

export type StoreStacksView = "branches" | "workspace" | "log" | "oplog";

export function isStacksRoute(route: StoreRoute): boolean {
  return (
    route.page === "stacks" ||
    (route.context === "stacks" && ["log", "oplog"].includes(route.page))
  );
}

export function stacksRouteView(route: StoreRoute): StoreStacksView {
  return route.page === "stacks"
    ? (route.stackTab ?? "workspace")
    : route.page === "oplog"
      ? "oplog"
      : "log";
}

/** Ordinary page changes clear selections while retaining the native workspace. */
export function storePageRoute(route: StoreRoute, page: StorePage): StoreRoute {
  return {
    ...route,
    page,
    context: undefined,
    stackTab: undefined,
    historyQuery: undefined,
    historyTime: undefined,
    projectsHub: false,
    workTab: undefined,
    workQuery: undefined,
    workState: undefined,
    workRepository: undefined,
    workHome: undefined,
    workActivity: undefined,
    bookmark: "",
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
    projectId: null,
    taskId: null,
    threadId: null,
    replyId: null,
    commentId: null,
  };
}

/** Session-only presentation state; named workspace configuration belongs to JJ. */
export function createStoreStacksNavigation(options: {
  route(): StoreRoute;
  push(route: StoreRoute): void;
}) {
  let returnRoute: StoreRoute | undefined;
  const retained = new Map<string, StoreRoute>();
  const canvas = new Map<string, "branches" | "workspace">();
  function select(view: StoreStacksView) {
    const current = options.route();
    if (isStacksRoute(current)) {
      retained.set(
        `${current.repository}:${stacksRouteView(current)}`,
        current,
      );
      if (current.page === "stacks")
        canvas.set(current.repository, current.stackTab ?? "workspace");
    } else returnRoute = { ...current };
    const page = view === "log" || view === "oplog" ? view : "stacks";
    const next = retained.get(`${current.repository}:${view}`) ?? {
      ...storePageRoute(current, page),
      workspace: null,
      ...(page === "stacks"
        ? { stackTab: view as "branches" | "workspace" }
        : { context: "stacks" as const }),
    };
    if (view === "branches" || view === "workspace")
      canvas.set(current.repository, view);
    options.push(next);
  }
  return {
    select,
    enter() {
      select(canvas.get(options.route().repository) ?? "workspace");
    },
    leave() {
      const current = options.route();
      if (!isStacksRoute(current)) return;
      retained.set(
        `${current.repository}:${stacksRouteView(current)}`,
        current,
      );
      options.push(
        returnRoute?.repository === current.repository
          ? returnRoute
          : {
              ...storePageRoute(current, "files"),
              workspace: null,
            },
      );
    },
  };
}
