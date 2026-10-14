import type { StoreRoute } from "./contracts.js";
import {
  readWorkflowsHash,
  workflowsRoute,
  workflowsRouteToHash,
} from "./workflows-route-codec.js";
const repositoryId = /^[0-9a-f]{32}$/;
/** Actions shares WorkflowSelection validation, with an immutable repository scope. */
export function readRepositoryActionsHash(hash: string) {
  const path = hash.split("?")[0]!;
  if (!/^#\/repos\/[^/]+\/actions$/.test(path)) return null;
  const repository = path.split("/")[2]!;
  const invalid = () => ({
    kind: "invalid" as const,
    message: "Invalid repository Actions route.",
  });
  const query = new URLSearchParams(
    hash.includes("?") ? hash.slice(hash.indexOf("?") + 1) : "",
  );
  if (!repositoryId.test(repository) || query.has("source") || query.has("tab"))
    return invalid();
  query.set("tab", "history");
  query.set("source", repository);
  const parsed = readWorkflowsHash(`#/workflows?${query}`);
  if (parsed?.kind !== "route") return invalid();
  return {
    kind: "route" as const,
    route: { ...parsed.route, repository, page: "actions" as const },
  };
}
export function repositoryActionsRouteToHash(route: StoreRoute): string | null {
  const selection = route.workflow ?? {};
  if (
    !repositoryId.test(route.repository) ||
    (selection.source && selection.source !== route.repository) ||
    (selection.tab && selection.tab !== "history")
  )
    return null;
  const validated = workflowsRouteToHash(
    workflowsRoute({ ...selection, tab: "history", source: route.repository }),
  );
  if (!validated) return null;
  const query = new URLSearchParams(validated.split("?")[1]);
  query.delete("tab");
  query.delete("source");
  return `#/repos/${route.repository}/actions${query.size ? `?${query}` : ""}`;
}
