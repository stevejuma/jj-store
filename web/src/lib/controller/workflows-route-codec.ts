import { agentsRoute } from "./agents-route-codec.js";
import type { StoreRoute } from "./contracts.js";
export const workflowStates = [
  "pending",
  "running",
  "cancel_requested",
  "completed",
  "failed",
  "cancelled",
  "unknown",
] as const;
export type WorkflowSelection = {
  tab?: "workflows" | "history";
  q?: string;
  state?: (typeof workflowStates)[number];
  scope?: string;
  source?: string;
  workflow?: string;
  home?: string;
  run?: string;
  at?: string;
  attempt?: string;
};
const ids = ["scope", "source", "workflow", "home", "run", "attempt"];
const fields = ["tab", "q", "state", ...ids, "at"];
const id = /^[0-9a-f]{32}$/;
export function workflowsRoute(selection: WorkflowSelection = {}): StoreRoute {
  return {
    ...agentsRoute(),
    page: "workflows",
    agentsHub: false,
    workflow: selection,
  };
}
export function readWorkflowsHash(hash: string) {
  if (hash.split("?")[0] !== "#/workflows") return null;
  const query = new URLSearchParams(
    hash.includes("?") ? hash.slice(hash.indexOf("?") + 1) : "",
  );
  const invalid = () => ({
    kind: "invalid" as const,
    message: "Invalid Workflows route.",
  });
  if (
    [...query.keys()].some(
      (key) => !fields.includes(key) || query.getAll(key).length !== 1,
    )
  )
    return invalid();
  if (ids.some((key) => query.has(key) && !id.test(query.get(key)!)))
    return invalid();
  const tab = query.get("tab") ?? "workflows",
    q = query.get("q") ?? "",
    at = query.get("at");
  if (
    !["workflows", "history"].includes(tab) ||
    new TextEncoder().encode(q).length > 512 ||
    /[\u0000-\u001f]/.test(q)
  )
    return invalid();
  if (
    query.has("state") &&
    !workflowStates.includes(
      query.get("state") as (typeof workflowStates)[number],
    )
  )
    return invalid();
  if (query.has("workflow") && !query.has("scope")) return invalid();
  if (
    query.has("run") !== query.has("home") ||
    ((at !== null || query.has("attempt")) && !query.has("run"))
  )
    return invalid();
  if (
    tab !== "history" &&
    ["state", "workflow", "home", "run", "at", "attempt"].some((key) =>
      query.has(key),
    )
  )
    return invalid();
  if (
    at !== null &&
    (!/^(0|[1-9][0-9]{0,19})$/.test(at) || BigInt(at) > 18446744073709551615n)
  )
    return invalid();
  const selection = Object.fromEntries(query.entries()) as WorkflowSelection;
  return { kind: "route" as const, route: workflowsRoute(selection) };
}
export function workflowsRouteToHash(route: StoreRoute): string | null {
  const query = new URLSearchParams();
  for (const [key, value] of Object.entries(route.workflow ?? {})) {
    if (value && !(key === "tab" && value === "workflows"))
      query.set(key, value);
  }
  const hash = `#/workflows${query.size ? `?${query}` : ""}`;
  return readWorkflowsHash(hash)?.kind === "route" ? hash : null;
}
