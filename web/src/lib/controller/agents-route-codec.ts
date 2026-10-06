import type { StoreRoute } from "./contracts.js";
import { safeLabel } from "./store-route-fields.js";
export function agentsRoute(
  view: StoreRoute["agentView"] = "directory",
  id?: string,
  relatedId?: string,
): StoreRoute {
  return {
    page: "agents",
    agentsHub: true,
    agentView: view,
    setupId:
      view === "session"
        ? (relatedId ?? null)
        : view !== "directory"
          ? (id ?? null)
          : null,
    executionSessionId:
      view === "session"
        ? (id ?? null)
        : view === "activity"
          ? (relatedId ?? null)
          : null,
    repository: "",
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
export function agentsRouteToHash(route: StoreRoute): string | null {
  const view = route.agentView ?? "directory";
  if (view === "directory") return "#/agents";
  if (route.setupId && !safeLabel(route.setupId, 64)) return null;
  const query = new URLSearchParams();
  if (route.setupId) query.set("agent", route.setupId);
  if (view === "activity" && route.executionSessionId) {
    if (!/^[0-9a-f]{32}$/.test(route.executionSessionId)) return null;
    query.set("session", route.executionSessionId);
  }
  if (
    view === "session" &&
    !/^[0-9a-f]{32}$/.test(route.executionSessionId ?? "")
  )
    return null;
  const path =
    view === "session" ? `sessions/${route.executionSessionId}` : view;
  return `#/agents/${path}${query.size ? `?${query}` : ""}`;
}
export function agentsRouteFromHash(hash: string): StoreRoute | null {
  const [path, queryString = ""] = hash.split("?");
  const query = new URLSearchParams(queryString);
  if (path === "#/agents" && query.size === 0) return agentsRoute();
  if (
    query.getAll("agent").length > 1 ||
    (query.has("agent") && !safeLabel(query.get("agent")!, 64))
  )
    return null;
  const agent = query.get("agent") ?? undefined;
  if (
    ["#/agents/new", "#/agents/sessions", "#/agents/activity"].includes(
      path ?? "",
    )
  ) {
    const view = path!.slice("#/agents/".length) as
      "new" | "sessions" | "activity";
    if (
      ![...query.keys()].every(
        (key) => key === "agent" || (view === "activity" && key === "session"),
      )
    )
      return null;
    if (
      query.getAll("session").length > 1 ||
      (query.has("session") && !/^[0-9a-f]{32}$/.test(query.get("session")!))
    )
      return null;
    return agentsRoute(view, agent, query.get("session") ?? undefined);
  }
  const match = /^#\/agents\/sessions\/([0-9a-f]{32})$/.exec(path ?? "");
  return match && [...query.keys()].every((key) => key === "agent")
    ? agentsRoute("session", match[1], agent)
    : null;
}

export function readAgentsHash(
  hash: string,
): import("./store-route-codec.js").StoreRouteHashResult | undefined {
  if (!(
    hash === "#/agents" ||
    hash.startsWith("#/agents/") ||
    hash.startsWith("#/agents?")
  ))
    return;
  const route = agentsRouteFromHash(hash);
  return route
    ? { kind: "route", route }
    : { kind: "invalid", message: "Invalid Agents route." };
}
