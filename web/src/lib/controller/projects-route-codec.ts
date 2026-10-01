import type { StoreRoute } from "./contracts.js";
const id = /^[0-9a-f]{32}$/u;
const tabs = ["projects", "tasks", "reviews", "repositories", "activity"];
const states = ["active", "all", "open", "in_progress", "blocked", "done"];
const fields = [
  "comment",
  "home",
  "project",
  "task",
  "tab",
  "q",
  "state",
  "repository",
  "scope",
  "at_op",
  "activity",
];
export function projectsRouteToHash(route: StoreRoute): string | null {
  const query = new URLSearchParams();
  const pairs = {
    comment: route.commentId,
    home: route.projectId ? route.repository : undefined,
    project: route.projectId,
    task: route.taskId,
    tab: route.workTab,
    q: route.workQuery,
    state: route.workState,
    repository: route.workRepository,
    scope: route.workHome,
    activity: route.workActivity === "all" ? undefined : route.workActivity,
    at_op: route.projectId ? route.snapshotOperationId : undefined,
  };
  for (const [key, value] of Object.entries(pairs))
    if (value) query.set(key, value);
  return `#/projects${query.size ? `?${query}` : ""}`;
}
export function projectsRouteFromHash(hash: string): StoreRoute | null {
  if (hash.split("?")[0] !== "#/projects") return null;
  const query = new URLSearchParams(
    hash.includes("?") ? hash.slice(hash.indexOf("?") + 1) : "",
  );
  if (
    [...query.keys()].some(
      (key) => !fields.includes(key) || query.getAll(key).length !== 1,
    )
  )
    return null;
  for (const key of [
    "home",
    "project",
    "task",
    "comment",
    "repository",
    "scope",
  ])
    if (query.has(key) && !id.test(query.get(key)!)) return null;
  if (
    query.has("at_op") &&
    (!query.has("home") || !/^[0-9a-f]{128}$/.test(query.get("at_op")!))
  )
    return null;
  const activity = query.get("activity");
  const tab = query.get("tab"),
    state = query.get("state"),
    q = query.get("q") ?? "";
  if (
    (activity !== null && !["all", "agent", "unassisted"].includes(activity)) ||
    (tab && !tabs.includes(tab)) ||
    (query.has("project") && tab === "projects") ||
    (query.has("task") && tab && tab !== "tasks") ||
    (state && !states.includes(state)) ||
    new TextEncoder().encode(q).length > 512 ||
    /[\u0000-\u001f]/u.test(q)
  )
    return null;
  if (
    query.has("project") !== query.has("home") ||
    (query.has("task") && !query.has("project")) ||
    (query.has("comment") && !query.has("task"))
  )
    return null;
  return {
    page: "projects",
    projectsHub: true,
    repository: query.get("home") ?? "",
    projectId: query.get("project"),
    taskId: query.get("task"),
    ...(query.get("comment") ? { commentId: query.get("comment") } : {}),
    workTab: (tab ??
      (query.has("project") ? "tasks" : "projects")) as StoreRoute["workTab"],
    workState: (state ?? "active") as StoreRoute["workState"],
    workQuery: q,
    workActivity: (activity ?? "all") as StoreRoute["workActivity"],
    workRepository: query.get("repository") ?? "",
    workHome: query.get("scope") ?? "",
    bookmark: "",
    workspace: null,
    snapshotOperationId: query.get("at_op"),
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
