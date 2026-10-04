import type { WorkController } from "./work-types.js";
import type { FreshnessChange } from "../api/application-events.js";
export function pendingProjectCommands(
  works: ReadonlyMap<string, WorkController>,
) {
  return [...works]
    .filter(([, work]) => work.state.pending)
    .map(([home, work]) => ({ home, work }));
}
export function invalidateProjectViews(
  views: { clear(): void },
  works: ReadonlyMap<string, WorkController>,
  changes: readonly FreshnessChange[],
) {
  views.clear();
  for (const change of changes)
    if (change.repository && ["repository", "work"].includes(change.domain))
      works.get(change.repository)?.invalidate?.();
}

export function projectProjectionAccepted(
  domain: import("./projects-dirtiness.js").ProjectDomain,
  id: string,
  route: import("./contracts.js").StoreRoute,
  works: ReadonlyMap<string, WorkController>,
  projections: Record<
    import("./projects-dirtiness.js").ProjectDomain,
    readonly { repository: { id: string }; error: string; loading: boolean }[]
  >,
) {
  if (domain === "work" && route.projectId) {
    const state = works.get(id)?.state;
    return !!state?.snapshot && !state.error && !state.loading;
  }
  const row = projections[domain].find((row) => row.repository.id === id);
  return !!row && !row.error && !row.loading;
}
