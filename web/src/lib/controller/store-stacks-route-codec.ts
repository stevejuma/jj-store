import type { StoreRoute } from "./store-route-types.js";
import type { StorePage } from "../ui/community-navigation/model.js";
import { one } from "./store-route-fields.js";
type Fields = Pick<StoreRoute, "context" | "stackTab" | "stackHistory">;

export function writeStacksRouteFields(
  route: StoreRoute,
  query: URLSearchParams,
): boolean {
  if (
    (route.context &&
      (route.context !== "stacks" || !["log", "oplog"].includes(route.page))) ||
    (route.stackHistory &&
      (route.context !== "stacks" ||
        !["live", "pinned"].includes(route.stackHistory))) ||
    (route.stackTab &&
      (route.page !== "stacks" ||
        !["branches", "workspace"].includes(route.stackTab)))
  )
    return false;
  if (route.context) query.set("context", route.context);
  if (route.stackTab) query.set("stack_tab", route.stackTab);
  if (route.stackHistory) query.set("stack_history", route.stackHistory);
  return true;
}

export function readStacksRouteFields(
  query: URLSearchParams,
  page: StorePage,
): Fields | null {
  const context = one(query, "context"),
    stackTab = one(query, "stack_tab"),
    stackHistory = one(query, "stack_history");
  if (
    context === undefined ||
    stackTab === undefined ||
    stackHistory === undefined ||
    (stackHistory !== null &&
      (context !== "stacks" || !["live", "pinned"].includes(stackHistory))) ||
    (context !== null &&
      (context !== "stacks" || !["log", "oplog"].includes(page))) ||
    (stackTab !== null &&
      (page !== "stacks" || !["branches", "workspace"].includes(stackTab)))
  )
    return null;
  return {
    ...(context ? { context: "stacks" as const } : {}),
    ...(stackTab ? { stackTab: stackTab as "branches" | "workspace" } : {}),
    ...(stackHistory
      ? { stackHistory: stackHistory as "live" | "pinned" }
      : {}),
  };
}
